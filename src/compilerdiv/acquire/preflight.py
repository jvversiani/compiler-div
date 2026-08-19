"""Fail fast, before a multi-hour sweep.

Checks that strace actually traces on this machine (ptrace is commonly confined
on clusters and in containers) and that every configured compiler can build a
hello-world. Failures print the compiler's full stderr rather than a summary,
because a broken ``y.sh`` wrapper is usually diagnosable only from it.
"""

from __future__ import annotations

import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path

from ..config import Settings
from ..corpus import LanguageSpec, MixedCorpusError, Program, validate_corpus
from .build import compile_one
from .trace import ArgNormalizer, build_strace_argv, parse_trace

#: Loose "this looks like an strace line" test, deliberately independent of the
#: real parser: any `name(...) = ...`. Used only to tell "strace was denied"
#: apart from "strace worked and our parser failed" -- two failures with
#: completely different remedies.
_TRACE_LINE_HINT = re.compile(r"^.*\b[a-z_][a-z0-9_]*\(.*\)\s+=\s+", re.M)


@dataclass
class PreflightReport:
    ok: bool
    lines: list[str]
    failures: list[tuple[str, str]]

    def render(self) -> str:
        out = list(self.lines)
        for name, err in self.failures:
            out.append(f"\n[FAIL] {name}:\n{err}")
        return "\n".join(out)


def _probe_program(path: Path, lang: LanguageSpec) -> Program:
    return Program(
        stem=path.stem,
        path=path,
        expected=("ok",),
        has_block=True,
        sentinel=False,
        sloc=1,
        lang=lang,
    )


def preflight(settings: Settings) -> PreflightReport:
    lines: list[str] = ["=== preflight ==="]
    failures: list[tuple[str, str]] = []

    # 0. One corpus, one language. Checked before anything else: it is a
    #    configuration error, it costs nothing to detect, and every check below
    #    (which compiler builds which probe) is meaningless while it holds.
    try:
        validate_corpus(settings.corpus_path, settings.languages)
    except MixedCorpusError as e:
        failures.append(("corpus", str(e)))
        return PreflightReport(False, lines, failures)

    # 1. Every executable we intend to invoke is present. Checking only
    #    wrapper[0] would let a missing strace through whenever a wrapper is
    #    configured.
    needed = list(settings.trace.wrapper[:1]) + [settings.trace.strace_bin]
    for exe in needed:
        if shutil.which(exe) is None and not Path(exe).is_file():
            failures.append(("strace", f"{exe!r} not found in PATH"))
            return PreflightReport(False, lines, failures)

    # 2. strace can actually trace, and we can parse it.
    norm = ArgNormalizer(settings.detect.arg_normalizers)
    argv = build_strace_argv(settings, Path("/bin/true"))
    try:
        res = subprocess.run(argv, capture_output=True, timeout=60)
        text = res.stderr.decode(errors="replace")
        tr = parse_trace(text, frozenset(settings.trace.arg_syscalls), norm)
    except (OSError, subprocess.TimeoutExpired) as e:
        failures.append(("strace", f"could not run {' '.join(argv)}: {e}"))
        return PreflightReport(False, lines, failures)

    if not tr.ok:
        # Distinguish "strace was denied" from "strace worked, we failed to read
        # it". Both used to print the same ptrace advice, which sends you to
        # check ptrace_scope when the fault is actually in the parser.
        looks_like_a_trace = bool(_TRACE_LINE_HINT.search(text))
        if looks_like_a_trace:
            diagnosis = (
                "strace RAN CORRECTLY -- the output above contains syscalls -- but "
                "compilerdiv could not parse it. This is a bug in compilerdiv, not a "
                "problem with your machine, and ptrace is NOT the issue.\n"
                "Most likely an strace version emitting a line format the parser "
                "does not recognise. Please report the first few lines above."
            )
        else:
            diagnosis = (
                "strace produced no syscall output at all. ptrace is often "
                "confined on clusters and in containers. Try\n"
                f"  {' '.join(argv)}\n"
                "by hand, and check /proc/sys/kernel/yama/ptrace_scope.\n"
                "If your server needs a user namespace, set trace.wrapper in the "
                "config (default: ['unshare', '-Ur']); if it does not, set it to "
                "[]."
            )
        failures.append(
            (
                "strace",
                f"ran but produced no parseable syscalls (rc={res.returncode}).\n"
                f"--- stderr (first 1500 chars) ---\n{text.strip()[:1500]}\n"
                f"---------------------------------\n{diagnosis}",
            )
        )
        return PreflightReport(False, lines, failures)

    lines.append(
        f"  [ok] strace traces and parses ({len(tr.counts)} syscall types on /bin/true)"
    )

    # 3. every compiler builds a probe, in each language the corpus actually
    #    uses. The corpus language is *detected* from file extensions rather
    #    than assumed, so a C corpus is probed with a C hello-world.
    probe_langs = settings.corpus_languages()
    if not probe_langs:
        lines.append(
            "  [note] no recognised source files in the corpus dir; skipping the "
            "compile probe. Populate the corpus, or add a `languages:` entry for "
            "its extension, to have preflight build-check the compilers."
        )
    else:
        lines.append(
            f"  [ok] corpus language(s) detected: "
            f"{', '.join(sorted(l.name for l in probe_langs))}"
        )

    with tempfile.TemporaryDirectory() as td:
        for lang in probe_langs:
            if not lang.hello_world:
                lines.append(
                    f"  [note] {lang.name}: no hello_world probe configured; "
                    "compiler build-check skipped for this language."
                )
                continue
            src = Path(td) / f"compilerdiv_probe{lang.extension}"
            src.write_text(lang.hello_world)
            prog = _probe_program(src, lang)
            for cfg in settings.config_names:
                for key in settings.compilers_for(cfg):
                    spec = settings.compiler(key)
                    out = Path(td) / f"probe_{key}_{cfg}_{lang.name}"
                    cr = compile_one(settings, spec, cfg, prog, out)
                    if cr.ok:
                        lines.append(
                            f"  [ok] {spec.label} built the {lang.name} probe ({cfg})"
                        )
                    else:
                        failures.append(
                            (f"{spec.label} / {cfg} / {lang.name}", cr.error)
                        )

    lines.extend(_budget_lines(settings))

    ok = not failures
    if ok:
        lines.append("=== preflight passed ===")
    return PreflightReport(ok, lines, failures)


def _budget_lines(settings) -> list[str]:
    """What the sweep is about to cost, before hours are committed to it.

    The control grid multiplies: ``builds x passes x trace.reps`` baseline
    traces per program. Raising ``builds`` from 2 to 3 and ``passes`` from 2 to
    3 is a 2.25x increase on the baseline half of the sweep, which is easy to
    ask for and expensive to discover afterwards.
    """
    b = settings.trace_budget()
    ctl = settings.controls
    n_configs = len(settings.config_names)
    n_programs = settings.n_corpus_files()

    lines = [
        "",
        "  measurement budget:",
        f"    controls        : {ctl.builds} builds x {ctl.passes} passes "
        f"= {b['baseline_groups']} baseline sample groups",
        f"    per program     : {b['baseline_compiles']} baseline compiles, "
        f"{b['traces_per_program']} traces "
        f"({b['baseline_traces']} baseline + {b['variant_traces']} variant)",
    ]
    if n_programs:
        total = b["traces_per_program"] * n_programs * n_configs
        lines.append(
            f"    whole sweep     : {n_programs} programs x {n_configs} config(s) "
            f"= {total:,} traces"
        )
    if ctl.builds < 3:
        lines.append(
            f"    [note] controls.builds = {ctl.builds} estimates build variance "
            f"from n={ctl.builds}. That detects a difference but cannot say "
            "whether it reproduces; 3+ is what the per-file build floor wants."
        )
    return lines
