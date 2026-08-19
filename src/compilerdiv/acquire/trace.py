"""Tracing: invoke strace and parse a full log into three views.

``strace -f -c`` emits only an aggregate count summary, which discards two
things the paper needs:

* **arguments** -- ``openat("/etc/passwd")`` and ``openat("/lib/x.so")`` are
  indistinguishable under counting;
* **order** -- a payload that reorders existing calls is invisible.

So we run a *full* trace (``strace -f -tt``) and derive all three views from one
log. Counts are recovered by tallying, so nothing is lost relative to ``-c``.

Per-thread grouping matters. Global syscall order is scheduler-dependent in any
threaded program and is therefore not a usable signal; *within-thread* order is
deterministic in the absence of the scheduler and is where the real resolution
lives. We key sequences by TID and let the noise floor decide what survives.
"""

from __future__ import annotations

import re
import subprocess
from collections import Counter, defaultdict
from dataclasses import dataclass, field
from pathlib import Path

from ..config import Settings

# strace line prefix. Two things vary and BOTH must be tolerated.
#
# 1. The PID is OPTIONAL: `-f` only prefixes a line once strace is following
#    more than one process, and it drops back to unprefixed once the extra
#    tracees are gone. A single-threaded program is unprefixed throughout.
# 2. The PID has TWO spellings. Contemporary strace (>= 4.x, verified on 7.0)
#    writes a bracketed prefix; the bare-numeric form appears in other
#    versions and in `-o`-split output:
#
#   bracketed  : [pid 12345] 10:11:12.123456 openat(...) = 3
#   bare       : 12345 10:11:12.123456 openat(...) = 3
#   unprefixed :             10:11:12.123456 openat(...) = 3
#
# Accepting only one spelling is silent, not loud: unmatched lines are simply
# dropped, so a threaded program loses every syscall its spawned threads make
# -- and, because the prefix is decided per line by the interleaving, loses a
# *random subset* of the main thread's calls too. That reads downstream as
# run-to-run instability in the baseline, which zeroes out its stable sets and
# makes every variant look like it departed.
_PREFIX = (
    r"^(?:\[pid\s+(?P<pid_bracketed>\d+)\]\s+|(?P<pid_bare>\d+)\s+)?"
    r"(?P<ts>\d{1,2}:\d{2}:\d{2}\.\d+)\s+"
)

# A completed call:  openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY) = 3
#
# `args` is GREEDY. Traced string data routinely contains ") = " -- an ELF
# header read, a program printing punctuation -- and a lazy match would stop at
# the first one inside the quotes and mis-parse the call. Anchoring on the LAST
# ") = " on the line is correct, because the return value is always last.
_LINE_RE = re.compile(
    _PREFIX + r"(?P<call>[a-zA-Z_][a-zA-Z0-9_]*)\("
    r"(?P<args>.*)"
    r"\)\s+=\s+(?P<ret>.+)$"
)

# futex(0x7f.., FUTEX_WAIT, 0, NULL <unfinished ...>
_UNFINISHED_RE = re.compile(
    _PREFIX + r"(?P<call>[a-zA-Z_][a-zA-Z0-9_]*)\((?P<args>.*)\s*<unfinished \.\.\.>$"
)

# <... futex resumed>) = 0
_RESUMED_RE = re.compile(
    _PREFIX + r"<\.\.\.\s+(?P<call>[a-zA-Z_][a-zA-Z0-9_]*)\s+resumed>"
)

# Signal delivery and exit notices, e.g.
#   --- SIGCHLD {si_signo=SIGCHLD, ...} ---
#   +++ exited with 0 +++
_SKIP_RE = re.compile(_PREFIX + r"(?:---|\+\+\+)")

# Strace's own diagnostics, which carry no timestamp and must not be parsed as
# syscalls (e.g. "strace: Process 123 attached").
_STRACE_NOTE_RE = re.compile(r"^strace:\s")

# Trailing return value, used to learn a clone's child TID off a resumed line
# (`_RESUMED_RE` deliberately stops at the call name).
_RET_RE = re.compile(r"=\s*(?P<ret>-?\d+)\s*$")

#: Calls whose return value is a newly created TID.
_CLONE_CALLS = frozenset({"clone", "clone2", "clone3", "fork", "vfork"})

#: TID used when strace omits the PID prefix (single-process trace). The value
#: is arbitrary; only the grouping matters, and there is exactly one group.
_SOLO_TID = "0"


def _pid(m: re.Match[str]) -> str | None:
    """The TID from whichever prefix spelling matched, or None if unprefixed.

    ``re`` forbids reusing a group name across an alternation, so the two
    spellings need two groups and one accessor.
    """
    return m.group("pid_bracketed") or m.group("pid_bare")


def _solo_owner(seen_tids: set[str], child_tids: set[str]) -> str | None:
    """The real TID the unprefixed lines belong to, when it can be pinned down.

    strace drops the prefix exactly while one tracee is live, and that tracee is
    always the top-level process -- so a threaded program's unprefixed lines and
    its top-level process's *prefixed* lines come from the same thread. Left
    unmerged, that thread's sequence is split across two buckets and
    ``n_threads`` is inflated by one, on precisely the threaded programs this
    parser exists to measure.

    The top-level process is the traced TID that no clone in this trace
    produced. When that is ambiguous -- a clone whose return was never
    observed, a trace that starts mid-flight -- the buckets are left alone
    rather than guessed at.
    """
    if _SOLO_TID not in seen_tids:
        return None
    candidates = seen_tids - child_tids - {_SOLO_TID}
    return candidates.pop() if len(candidates) == 1 else None


@dataclass
class TraceResult:
    """One trace run, parsed.

    Attributes
    ----------
    counts:
        ``syscall -> total invocations`` (across all threads). Equivalent to
        what ``strace -c`` would have reported.
    sequences:
        ``tid -> [syscall, ...]`` in per-thread execution order.
    args:
        ``syscall -> Counter(normalized_arg_string -> n)``. Only populated for
        syscalls in :attr:`TraceSettings.arg_syscalls`.
    n_threads:
        Distinct TIDs observed. A file with >1 thread has a structurally noisy
        sequence oracle.
    ok:
        Whether the trace produced any parseable syscall at all.
    error:
        Diagnostic when ``ok`` is False.
    """

    counts: dict[str, int] = field(default_factory=dict)
    sequences: dict[str, list[str]] = field(default_factory=dict)
    args: dict[str, Counter] = field(default_factory=dict)
    n_threads: int = 0
    ok: bool = True
    error: str = ""


class ArgNormalizer:
    """Applies the configured regex substitutions to raw argument strings.

    Run-varying content (pids, ASLR addresses, mkstemp suffixes) would make
    every argument comparison fire. The normalizers are configurable because
    what counts as incidental is corpus-dependent.

    ``binary_path`` handles a systematic case that no regex should have to
    guess at: each compiler necessarily writes its binary to a different
    directory, so ``execve("<work>/A/prog")`` vs ``execve("<work>/B/prog")``
    differs for every file, for every variant, always. That is harness geometry,
    not behavior. When set, the binary's own path (and its parent directory) is
    rewritten to a placeholder before comparison.
    """

    def __init__(
        self,
        patterns: tuple[tuple[str, str], ...],
        binary_path: Path | None = None,
    ):
        self._subs = [(re.compile(p), r) for p, r in patterns]
        self._binary = binary_path

    def with_binary(self, binary: Path) -> ArgNormalizer:
        """Return a normalizer bound to a specific binary under trace."""
        # Same-class construction: reuse the compiled substitutions and rebind
        # the binary. pylint's protected-access does not model "self's own type".
        out = ArgNormalizer.__new__(ArgNormalizer)
        out._subs = self._subs  # pylint: disable=protected-access
        out._binary = binary  # pylint: disable=protected-access
        return out

    def __call__(self, s: str) -> str:
        if self._binary is not None:
            b = str(self._binary)
            if b in s:
                s = s.replace(b, "<BINARY>")
            d = str(self._binary.parent)
            if d in s:
                s = s.replace(d, "<BINDIR>")
        for rx, rep in self._subs:
            s = rx.sub(rep, s)
        return s.strip()


def _first_string_arg(args: str) -> str | None:
    """Extract the first quoted string from an argument list, if any.

    For path-taking syscalls this is the interesting part; keeping the whole
    argument string would fold in flags and fds that churn for benign reasons.
    """
    m = re.search(r'"((?:[^"\\]|\\.)*)"', args)
    return m.group(1) if m else None


def parse_trace(
    text: str,
    arg_syscalls: frozenset[str],
    normalizer: ArgNormalizer,
) -> TraceResult:
    """Parse a full strace log.

    Handles ``<unfinished ...>`` / ``<... resumed>`` pairs, which appear
    whenever a thread blocks. The syscall is attributed once, at the point it
    was *initiated*, which is the ordering that reflects program structure
    rather than scheduling.
    """
    counts: Counter[str] = Counter()
    args_by_call: dict[str, Counter] = defaultdict(Counter)
    seen_tids: set[str] = set()
    child_tids: set[str] = set()
    # (tid, call) in file order. Flat rather than pre-bucketed so the solo TID
    # can be reattributed at the end without losing the interleaving.
    records: list[tuple[str, str]] = []

    def _record(pid: str | None, call: str, argstr: str) -> None:
        tid = pid or _SOLO_TID
        seen_tids.add(tid)
        counts[call] += 1
        records.append((tid, call))
        if call in arg_syscalls:
            s = _first_string_arg(argstr)
            if s is not None:
                args_by_call[call][normalizer(s)] += 1

    def _note_child(call: str, ret: str | None) -> None:
        if call in _CLONE_CALLS and ret is not None and ret.strip().isdigit():
            child_tids.add(ret.strip())

    for raw in text.splitlines():
        line = raw.rstrip()
        if not line or _STRACE_NOTE_RE.match(line) or _SKIP_RE.match(line):
            continue

        m = _LINE_RE.match(line)
        if m:
            _record(_pid(m), m.group("call"), m.group("args"))
            _note_child(m.group("call"), m.group("ret"))
            continue

        m = _UNFINISHED_RE.match(line)
        if m:
            # Count and order at initiation; the resumed half is ignored.
            _record(_pid(m), m.group("call"), m.group("args"))
            continue

        m = _RESUMED_RE.match(line)
        if m:
            seen_tids.add(_pid(m) or _SOLO_TID)
            # A clone that blocked reports its child's TID only here.
            r = _RET_RE.search(line)
            _note_child(m.group("call"), r.group("ret") if r else None)
            continue

    owner = _solo_owner(seen_tids, child_tids)
    if owner is not None:
        seen_tids.discard(_SOLO_TID)
        records = [(owner if t == _SOLO_TID else t, c) for t, c in records]

    sequences: dict[str, list[str]] = defaultdict(list)
    for tid, call in records:
        sequences[tid].append(call)

    ok = bool(counts)
    return TraceResult(
        counts=dict(counts),
        sequences=dict(sequences),
        args=dict(args_by_call),
        n_threads=len(seen_tids),
        ok=ok,
        error="" if ok else "no syscalls parsed from trace output",
    )


def build_strace_argv(settings: Settings, binary: Path) -> list[str]:
    """Assemble the strace command.

    ``-f`` follows threads, ``-tt`` gives per-line timestamps (used to keep
    unfinished/resumed pairs attributable), ``-s`` widens string truncation so
    paths survive intact, ``-qq`` suppresses exit-status chatter.
    """
    ts = settings.trace
    return [
        *ts.wrapper,
        ts.strace_bin,
        "-f",
        "-tt",
        "-s",
        "512",
        "-qq",
        str(binary),
    ]


def trace_once(
    settings: Settings, binary: Path, normalizer: ArgNormalizer
) -> TraceResult:
    """Run one traced execution of ``binary``.

    The normalizer is bound to this binary so its own path is masked: each
    compiler writes to its own output directory, so an unmasked ``execve``
    argument would differ for every file under every variant.
    """
    argv = build_strace_argv(settings, binary)
    norm = normalizer.with_binary(binary)
    arg_syscalls = (
        frozenset(settings.trace.arg_syscalls)
        if settings.trace.capture_args
        else frozenset()
    )
    try:
        res = subprocess.run(
            argv,
            cwd=str(binary.parent),
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            timeout=settings.trace.timeout_s,
        )
    except subprocess.TimeoutExpired:
        return TraceResult(
            ok=False, error=f"strace timeout after {settings.trace.timeout_s}s"
        )
    except OSError as e:
        return TraceResult(ok=False, error=f"could not run strace: {e}")

    text = res.stderr.decode(errors="replace")
    out = parse_trace(text, arg_syscalls, norm)
    if not out.ok:
        tail = " | ".join(text.strip().splitlines()[-3:]) or "(empty stderr)"
        out.error = f"rc={res.returncode}: {tail}"
    return out


def ngrams(seq: list[str], n: int) -> set[str]:
    """Distinct n-grams of a per-thread syscall sequence, as joined strings."""
    if len(seq) < n:
        return set()
    return {"\u2192".join(seq[i : i + n]) for i in range(len(seq) - n + 1)}


def ngram_profile(result: TraceResult, sizes: tuple[int, ...]) -> set[tuple[int, str]]:
    """All distinct ``(n, ngram)`` pairs across every thread of one run.

    Threads are pooled *after* n-gram extraction: an n-gram is a within-thread
    fact, but which thread produced it is not stable across runs, so thread
    identity is dropped at this point.
    """
    out: set[tuple[int, str]] = set()
    for seq in result.sequences.values():
        for n in sizes:
            for g in ngrams(seq, n):
                out.add((n, g))
    return out
