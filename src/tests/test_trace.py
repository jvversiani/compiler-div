"""Trace parsing: counts, per-TID sequences, arguments, unfinished/resumed.

OOP layout, all under the ``trace`` marker (``pytest -m trace``).
"""

from __future__ import annotations

from pathlib import Path

import pytest

from compilerdiv.acquire.trace import (
    ArgNormalizer,
    TraceResult,
    ngram_profile,
    ngrams,
    parse_trace,
)

pytestmark = pytest.mark.trace

NORM = ArgNormalizer(
    (
        (r"/proc/\d+/", "/proc/<PID>/"),
        (r"0x[0-9a-f]{6,}", "<ADDR>"),
    )
)
ARGS = frozenset({"openat", "connect", "execve"})

SIMPLE = """\
1000 10:00:00.000001 execve("/tmp/x/hello", ["hello"], 0x7ffd /* 30 vars */) = 0
1000 10:00:00.000002 brk(NULL)          = 0x5586c1000000
1000 10:00:00.000003 openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
1000 10:00:00.000004 read(3, "\\177ELF", 832) = 832
1000 10:00:00.000005 close(3)           = 0
1000 10:00:00.000006 write(1, "hello\\n", 6) = 6
1000 10:00:00.000007 exit_group(0)      = ?
"""


class TestCounts:
    def test_counts_recovered(self):
        tr = parse_trace(SIMPLE, ARGS, NORM)
        assert tr.ok
        assert tr.counts["openat"] == 1
        assert tr.counts["read"] == 1
        assert tr.counts["write"] == 1
        assert tr.counts["brk"] == 1

    def test_read_count_is_exact(self):
        """read(+1) is the central finding; miscounting read breaks it."""
        text = "\n".join(
            [
                r'10:00:00.1 read(3, "aaa"..., 1024) = 1024',
                r'10:00:00.2 read(3, "bbb"..., 1024) = 1024',
                r'10:00:00.3 read(3, ""..., 1024) = 0',
            ]
        )
        tr = parse_trace(text, ARGS, NORM)
        assert tr.counts["read"] == 3

    def test_empty_trace_is_not_ok(self):
        tr = parse_trace("strace: Operation not permitted\n", ARGS, NORM)
        assert not tr.ok
        assert tr.error


class TestUnprefixedFormat:
    """`strace -f` only prefixes lines with a PID once following >1 process. A
    single-threaded program -- most of the corpus -- produces unprefixed lines,
    and requiring the PID silently dropped every one of them."""

    # Verbatim from `unshare -Ur strace -f -tt -s 512 -qq /bin/true`.
    REAL = r"""12:23:15.821370 execve("/bin/true", ["/bin/true"], 0x7ffe3ff48428 /* 103 vars */) = 0
12:23:15.822188 brk(NULL)               = 0x55746b6c8000
12:23:15.822307 access("/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)
12:23:15.822410 openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
12:23:15.822671 fstat(3, {st_mode=S_IFREG|0644, st_size=319634, ...}) = 0
12:23:15.822862 mmap(NULL, 319634, PROT_READ, MAP_PRIVATE, 3, 0) = 0x7fbc5f0dc000
12:23:15.822977 close(3)                = 0
12:23:15.823124 openat(AT_FDCWD, "/usr/lib64/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
12:23:15.823160 read(3, "\177ELF\2\1\1\3\0\0\0\0\0\0\0\0\3\0>\0\1\0\0\0 t\2\0\0\0\0\0@\0\0\0\0\0\0\0"..., 832) = 832
12:23:15.823204 pread64(3, "\6\0\0\0\4\0\0\0@\0\0\0\0\0\0\0"..., 784, 64) = 784"""

    def test_lines_parse(self):
        tr = parse_trace(self.REAL, ARGS, NORM)
        assert tr.ok
        assert tr.counts["openat"] == 2
        assert tr.counts["read"] == 1
        assert tr.counts["execve"] == 1
        assert tr.n_threads == 1

    def test_sequence_order_preserved(self):
        tr = parse_trace(self.REAL, ARGS, NORM)
        seq = list(tr.sequences.values())[0]
        assert seq[:4] == ["execve", "brk", "access", "openat"]

    def test_arguments_captured(self):
        tr = parse_trace(self.REAL, ARGS, NORM)
        assert set(tr.args["openat"]) == {"/etc/ld.so.cache", "/usr/lib64/libc.so.6"}

    def test_mixed_prefixed_and_unprefixed(self):
        """strace starts unprefixed and adds PIDs once a second process appears."""
        text = (
            '10:00:00.1 execve("/x", ["x"], 0x7f) = 0\n'
            "1000 10:00:00.2 clone3({flags=CLONE_VM}, 88) = 1001\n"
            '1001 10:00:00.3 write(1, "hi\\n", 3) = 3\n'
        )
        tr = parse_trace(text, ARGS, NORM)
        assert tr.counts["execve"] == 1
        assert tr.counts["clone3"] == 1
        assert tr.counts["write"] == 1


#: Real `strace -f -tt -s 512 -qq` output (strace 7.0) for a two-thread Rust
#: program, thinned to the lines that carry the regression. Note what varies:
#: the top-level process (541163) is UNPREFIXED before it clones and again
#: after its threads exit, and BRACKET-PREFIXED in between -- strace decides
#: per line, from the interleaving, not from the program.
BRACKETED = """\
12:21:23.782758 execve("/w/rz", ["rz"], 0x7ffd /* 30 vars */) = 0
12:21:23.782758 clone3({flags=CLONE_VM|CLONE_THREAD}, 88) = 541164
[pid 541163] 12:21:23.783190 clone3({flags=CLONE_VM|CLONE_THREAD} <unfinished ...>
[pid 541164] 12:21:23.782984 rseq({cpu_id_start=0, flags=0}, 32, 0, 0x53053053) = 0
[pid 541163] 12:21:23.783247 <... clone3 resumed> => {parent_tid=[541165]}, 88) = 541165
[pid 541164] 12:21:23.783666 write(1, "(Primary) Old Mother Goose,\\n", 28 <unfinished ...>
[pid 541165] 12:21:23.784083 write(1, "(Primary) Humpty Dumpty sat on a wall.\\n", 39) = 39
[pid 541164] 12:21:23.784174 <... write resumed>) = 28
[pid 541165] 12:21:23.784489 write(1, "      Printer out of ink\\n", 25 <unfinished ...>
[pid 541163] 12:21:23.784600 futex(0x7f9c, FUTEX_WAIT, 0, NULL) = 0
12:21:23.784927 write(1, "Both threads finished.\\n", 23) = 23
12:21:23.785082 exit_group(0)           = ?
"""


class TestBracketedPidPrefix:
    """strace 7.0 writes `[pid N] `, not the bare `N `. Accepting only the bare
    spelling drops every prefixed line *silently* -- and since the prefix is
    decided per line by the interleaving, the top-level thread loses a random
    subset of its own calls too. Downstream that reads as an unstable baseline,
    which empties A's stable sets and reports every variant as departing.
    """

    ARGS = frozenset({"write", "execve"})

    def test_bracketed_lines_are_not_dropped(self):
        tr = parse_trace(BRACKETED, self.ARGS, NORM)
        # 4 writes, not the 1 that survives if bracketed lines are discarded.
        assert tr.counts["write"] == 4
        assert tr.counts["clone3"] == 2
        assert tr.counts["rseq"] == 1
        assert tr.counts["futex"] == 1

    def test_thread_emitted_arguments_reach_the_detector(self):
        """The payloads the *threads* write are the ones that were lost."""
        tr = parse_trace(BRACKETED, self.ARGS, NORM)
        assert set(tr.args["write"]) == {
            r"(Primary) Old Mother Goose,\n",
            r"(Primary) Humpty Dumpty sat on a wall.\n",
            r"Printer out of ink\n",  # leading indent stripped by the normalizer
            r"Both threads finished.\n",
        }

    def test_bare_spelling_still_accepted(self):
        """Other strace versions emit the bare form; both must parse."""
        bare = (
            BRACKETED.replace("[pid 541163] ", "541163 ")
            .replace("[pid 541164] ", "541164 ")
            .replace("[pid 541165] ", "541165 ")
        )
        assert (
            parse_trace(bare, self.ARGS, NORM).counts
            == parse_trace(BRACKETED, self.ARGS, NORM).counts
        )

    def test_padded_pid_accepted(self):
        """strace right-aligns the PID, so the gap after `pid` can widen."""
        tr = parse_trace(BRACKETED.replace("[pid ", "[pid  "), self.ARGS, NORM)
        assert tr.counts["write"] == 4

    @pytest.mark.parametrize("main_prefixed", [False, True])
    def test_main_thread_write_survives_either_spelling(self, main_prefixed):
        """The exact flake behind the Rendezvous_778 false positive.

        Whether the final `write` carries a prefix depends on whether a thread
        happened to print just before it. It must be counted either way, or the
        argument detector sees a baseline that made the call in 4 of 5 reps,
        marks it unstable, and reports the variant's identical write as a value
        `not seen under baseline`.
        """
        text = BRACKETED
        if main_prefixed:
            text = text.replace(
                '12:21:23.784927 write(1, "Both',
                '[pid 541163] 12:21:23.784927 write(1, "Both',
            )
        tr = parse_trace(text, self.ARGS, NORM)
        assert tr.counts["write"] == 4
        assert r"Both threads finished.\n" in tr.args["write"]

    def test_signal_lines_skipped_when_prefixed(self):
        text = BRACKETED + "[pid 541164] 12:21:23.785 +++ exited with 0 +++\n"
        tr = parse_trace(text, self.ARGS, NORM)
        assert "exited" not in tr.counts


class TestSoloTidAttribution:
    """strace omits the prefix only while a single tracee is live, and that
    tracee is always the top-level process. Its unprefixed and prefixed lines
    are therefore one thread, and must land in one sequence bucket -- otherwise
    n_threads is inflated and the top-level thread's n-grams are cut in half at
    every point the prefix flips."""

    def test_unprefixed_lines_merge_into_the_top_level_thread(self):
        tr = parse_trace(BRACKETED, frozenset(), NORM)
        assert tr.n_threads == 3
        assert set(tr.sequences) == {"541163", "541164", "541165"}

    def test_merge_preserves_interleaved_order(self):
        """The top-level thread's calls span three prefix regions; the merged
        sequence must read in trace order, not region order."""
        tr = parse_trace(BRACKETED, frozenset(), NORM)
        assert tr.sequences["541163"] == [
            "execve",
            "clone3",  # unprefixed, before the threads exist
            "clone3",  # bracketed, once strace is following two tracees
            "futex",
            "write",  # unprefixed again, after the threads are gone
            "exit_group",
        ]

    def test_child_tid_is_never_treated_as_the_owner(self):
        """The owner is the TID no clone in this trace produced."""
        tr = parse_trace(BRACKETED, frozenset(), NORM)
        assert "541164" in tr.sequences and "541165" in tr.sequences
        assert tr.sequences["541164"] == ["rseq", "write"]

    def test_single_threaded_trace_keeps_one_bucket(self):
        tr = parse_trace(SIMPLE.replace("1000 ", ""), ARGS, NORM)
        assert tr.n_threads == 1
        assert len(next(iter(tr.sequences.values()))) == 7

    def test_ambiguous_ownership_is_not_guessed(self):
        """Two prefixed TIDs and no clone to explain either: leave them apart
        rather than attribute the unprefixed lines to a coin flip."""
        text = (
            '10:00:00.1 write(1, "a\\n", 2) = 2\n'
            '2000 10:00:00.2 write(1, "b\\n", 2) = 2\n'
            '3000 10:00:00.3 write(1, "c\\n", 2) = 2\n'
        )
        tr = parse_trace(text, ARGS, NORM)
        assert tr.n_threads == 3
        assert set(tr.sequences) == {"0", "2000", "3000"}


class TestTrickyLines:
    def test_return_paren_inside_string_does_not_split(self):
        """Traced strings routinely contain ') = '. A lazy match would stop there."""
        line = r'10:00:00.1 write(1, "result) = 42 done\n", 18) = 18'
        tr = parse_trace(line, ARGS, NORM)
        assert tr.counts == {"write": 1}

    def test_elf_header_read_with_nulls_and_quotes(self):
        line = r'10:00:00.1 read(3, "\177ELF\2\1\1\3\0\0\0\0"..., 832) = 832'
        tr = parse_trace(line, ARGS, NORM)
        assert tr.counts == {"read": 1}

    def test_errno_return_parsed(self):
        line = '10:00:00.1 access("/etc/ld.so.preload", R_OK) = -1 ENOENT (No such file or directory)'
        tr = parse_trace(line, frozenset({"access"}), NORM)
        assert tr.counts == {"access": 1}
        assert tr.args["access"]["/etc/ld.so.preload"] == 1

    def test_strace_own_diagnostics_ignored(self):
        text = (
            "strace: Process 1000 attached\n"
            '10:00:00.1 write(1, "x\\n", 2) = 2\n'
            "strace: Process 1000 detached\n"
        )
        tr = parse_trace(text, ARGS, NORM)
        assert tr.counts == {"write": 1}

    def test_signal_and_exit_lines_ignored(self):
        text = SIMPLE + "1000 10:00:00.000008 +++ exited with 0 +++\n"
        text += "1000 10:00:00.000009 --- SIGCHLD {si_signo=SIGCHLD} ---\n"
        tr = parse_trace(text, ARGS, NORM)
        assert "exited" not in tr.counts
        assert "SIGCHLD" not in tr.counts


class TestArgumentCapture:
    def test_only_for_configured_syscalls(self):
        tr = parse_trace(SIMPLE, ARGS, NORM)
        assert "openat" in tr.args
        assert tr.args["openat"]["/etc/ld.so.cache"] == 1
        assert "read" not in tr.args  # read is not in ARGS

    def test_normalization_applied(self):
        text = '1000 10:00:00.1 openat(AT_FDCWD, "/proc/4242/maps", O_RDONLY) = 3\n'
        tr = parse_trace(text, ARGS, NORM)
        assert tr.args["openat"]["/proc/<PID>/maps"] == 1


class TestBinaryMasking:
    """Each compiler writes to its own directory, so the binary's own path
    differs for every file under every variant -- harness geometry, not
    behaviour, and it must not reach the argument detector."""

    def test_binary_path_masked(self):
        norm_a = NORM.with_binary(Path("/work/basic/A/prog_0"))
        norm_b = NORM.with_binary(Path("/work/basic/B/prog_0"))
        line_a = (
            '1000 10:00:00.1 execve("/work/basic/A/prog_0", ["prog_0"], 0x7f) = 0\n'
        )
        line_b = (
            '1000 10:00:00.1 execve("/work/basic/B/prog_0", ["prog_0"], 0x7f) = 0\n'
        )
        a = parse_trace(line_a, frozenset({"execve"}), norm_a)
        b = parse_trace(line_b, frozenset({"execve"}), norm_b)
        assert set(a.args["execve"]) == {"<BINARY>"}
        assert set(a.args["execve"]) == set(b.args["execve"])

    def test_binary_dir_masked(self):
        norm = NORM.with_binary(Path("/work/basic/B/prog_0"))
        text = (
            '1000 10:00:00.1 openat(AT_FDCWD, "/work/basic/B/data.txt", O_RDONLY) = 3\n'
        )
        tr = parse_trace(text, frozenset({"openat"}), norm)
        assert set(tr.args["openat"]) == {"<BINDIR>/data.txt"}

    def test_with_binary_does_not_mutate_original(self):
        bound = NORM.with_binary(Path("/work/A/prog"))
        assert NORM("/work/A/prog") == "/work/A/prog"
        assert bound("/work/A/prog") == "<BINARY>"

    def test_unrelated_paths_survive_masking(self):
        norm = NORM.with_binary(Path("/work/basic/B/prog_0"))
        text = '1000 10:00:00.1 openat(AT_FDCWD, "/etc/passwd", O_RDONLY) = 3\n'
        tr = parse_trace(text, frozenset({"openat"}), norm)
        assert set(tr.args["openat"]) == {"/etc/passwd"}


THREADED = """\
1000 10:00:00.000001 clone3({flags=CLONE_VM}, 88) = 1001
1001 10:00:00.000002 mmap(NULL, 8392704, PROT_NONE) = 0x7f9c00000000
1000 10:00:00.000003 write(1, "a\\n", 2) = 2
1001 10:00:00.000004 write(1, "b\\n", 2) = 2
1000 10:00:00.000005 futex(0x7f9c, FUTEX_WAIT, 0, NULL) = 0
"""

UNFINISHED = """\
1000 10:00:00.000001 futex(0x7f9c, FUTEX_WAIT, 0, NULL <unfinished ...>
1001 10:00:00.000002 write(1, "x\\n", 2) = 2
1000 10:00:00.000003 <... futex resumed>) = 0
"""


class TestThreadsAndSequences:
    def test_single_thread_sequence_order(self):
        tr = parse_trace(SIMPLE, ARGS, NORM)
        assert tr.n_threads == 1
        seq = tr.sequences["1000"]
        assert seq[:3] == ["execve", "brk", "openat"]
        assert seq[-1] == "exit_group"

    def test_threads_tracked_separately(self):
        tr = parse_trace(THREADED, ARGS, NORM)
        assert tr.n_threads == 2
        assert tr.sequences["1000"] == ["clone3", "write", "futex"]
        assert tr.sequences["1001"] == ["mmap", "write"]
        assert tr.counts["write"] == 2

    def test_unfinished_resumed_counted_once_at_initiation(self):
        tr = parse_trace(UNFINISHED, ARGS, NORM)
        assert tr.counts["futex"] == 1
        assert tr.counts["write"] == 1
        # order reflects where the call was initiated, not where it returned
        assert tr.sequences["1000"] == ["futex"]


class TestNgrams:
    def test_basic(self):
        assert ngrams(["a", "b", "c"], 3) == {"a→b→c"}
        assert ngrams(["a", "b"], 3) == set()
        assert ngrams(["a", "b", "c", "d"], 2) == {"a→b", "b→c", "c→d"}

    def test_profile_pools_threads_after_extraction(self):
        tr = TraceResult(
            counts={},
            sequences={"1": ["a", "b", "c"], "2": ["x", "y", "z"]},
            args={},
            n_threads=2,
        )
        prof = ngram_profile(tr, (3,))
        assert (3, "a→b→c") in prof
        assert (3, "x→y→z") in prof
        # No cross-thread n-gram is ever formed.
        assert (3, "c→x→y") not in prof
