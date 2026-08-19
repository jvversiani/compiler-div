"""Acquisition: compile / run / trace measurement and the sweep orchestration.

``build.py`` is exercised against a *fake compiler* -- a tiny Python script that
behaves like ``cc -o out src`` -- so the tests need no real toolchain. The
sweep, which additionally needs strace, runs with ``compile_one`` / ``run_binary``
/ ``trace_once`` / ``elf_facts`` stubbed in the sweep module namespace, so the
whole orchestration (bench pass, ELF pass, trace pass, controls, resume) is
covered deterministically. Preflight is driven with a stubbed strace.
"""

from __future__ import annotations

import dataclasses
import shutil
import sys
from pathlib import Path

import pytest

import importlib

from compilerdiv.acquire import build as build_mod
from compilerdiv.acquire import sweep as sweep_mod

# The `acquire` package re-exports a function named `preflight`, which shadows
# the submodule attribute; fetch the real module from sys.modules instead.
preflight_mod = importlib.import_module("compilerdiv.acquire.preflight")
from compilerdiv.acquire.build import (
    CompileResult,
    ElfFacts,
    ExecResult,
    compile_one,
    elf_facts,
    run_binary,
)
from compilerdiv.acquire.trace import TraceResult
from compilerdiv.config import (
    BuildConfig,
    CompilerSpec,
    ControlSettings,
    PathSettings,
    Settings,
)
from compilerdiv.corpus import Program
from compilerdiv.store.raw import COUNTS, ELF, RawStore

pytestmark = pytest.mark.acquire


# A stand-in compiler. Reads the source, honours a couple of directives, and
# writes an executable shell script as its "binary".
FAKE_CC = r"""
import os, sys
from pathlib import Path
args = sys.argv[1:]
out = args[args.index("-o") + 1] if "-o" in args else None
srcs = [a for a in args if Path(a).is_file()]
text = Path(srcs[-1]).read_text() if srcs else ""
if "COMPILEFAIL" in text:
    sys.stderr.write("fake-cc: error: deliberate compile failure\n")
    sys.exit(1)
line = "ok"
for l in text.splitlines():
    if l.strip().startswith("PRINT:"):
        line = l.split("PRINT:", 1)[1].strip()
        break
exitcode = 1 if "RUNFAIL" in text else 0
o = Path(out)
o.write_text("#!/bin/sh\necho %s\nexit %d\n" % (line, exitcode))
os.chmod(o, 0o755)
"""


@pytest.fixture
def fake_cc(tmp_path):
    p = tmp_path / "fake_cc.py"
    p.write_text(FAKE_CC)
    return CompilerSpec(key="A", label="fakecc", cmd=(sys.executable, str(p)))


@pytest.fixture
def build_settings(tmp_path, fake_cc):
    return Settings(
        baseline="A",
        compilers=(fake_cc,),
        configs=(BuildConfig(name="basic"),),
        paths=PathSettings(project_root=str(tmp_path)),
    )


def _program(tmp_path, body: str, expected=("ok",)) -> Program:
    src = tmp_path / "prog.c"
    src.write_text(body)
    return Program(
        stem="prog", path=src, expected=expected, has_block=True, sentinel=False, sloc=1
    )


class TestCompileOne:
    def test_success(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "PRINT: ok\n")
        out = tmp_path / "bin"
        r = compile_one(build_settings, fake_cc, "basic", prog, out)
        assert r.ok
        assert r.size_b > 0
        assert out.is_file()

    def test_failure_reports_stderr_tail(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "COMPILEFAIL\n")
        out = tmp_path / "bin"
        r = compile_one(build_settings, fake_cc, "basic", prog, out)
        assert not r.ok
        assert "deliberate compile failure" in r.error

    def test_removes_stale_output_first(self, tmp_path, build_settings, fake_cc):
        out = tmp_path / "bin"
        out.write_text("stale")
        prog = _program(tmp_path, "COMPILEFAIL\n")
        r = compile_one(build_settings, fake_cc, "basic", prog, out)
        # A failed compile must not leave the previous binary masquerading as new.
        assert not r.ok


class TestRunBinary:
    def _compile(self, tmp_path, settings, spec, prog):
        out = tmp_path / "bin"
        assert compile_one(settings, spec, "basic", prog, out).ok
        return out

    def test_success_verified(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "PRINT: ok\n")
        out = self._compile(tmp_path, build_settings, fake_cc, prog)
        r = run_binary(build_settings, out, prog)
        assert r.ok and r.verified and r.returncode == 0

    def test_output_mismatch_unverified(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "PRINT: nope\n", expected=("ok",))
        out = self._compile(tmp_path, build_settings, fake_cc, prog)
        r = run_binary(build_settings, out, prog)
        assert r.ok and not r.verified
        assert r.detail

    def test_nonzero_exit(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "PRINT: ok\nRUNFAIL\n")
        out = self._compile(tmp_path, build_settings, fake_cc, prog)
        r = run_binary(build_settings, out, prog)
        assert not r.ok
        assert r.returncode != 0

    def test_no_oracle_still_ok(self, tmp_path, build_settings, fake_cc):
        prog = _program(tmp_path, "PRINT: whatever\n", expected=None)
        out = self._compile(tmp_path, build_settings, fake_cc, prog)
        r = run_binary(build_settings, out, prog)
        assert r.ok and r.verified


class TestElfFacts:
    @pytest.mark.skipif(not Path("/bin/true").exists(), reason="no /bin/true")
    def test_real_elf_parsed(self):
        facts = elf_facts(Path("/bin/true"))
        assert facts.size_b > 0
        # readelf may be absent; then we only get the size.
        if shutil.which("readelf"):
            assert facts.probe_ok
            assert facts.n_sections > 0 or facts.interp

    @pytest.mark.skipif(not shutil.which("readelf"), reason="no readelf")
    @pytest.mark.skipif(not Path("/bin/true").exists(), reason="no /bin/true")
    def test_segments_counted(self):
        """Regression: _SEG_RE lacked re.M, so this was silently always 0.

        Every executable ELF has at least one LOAD segment. A zero here means
        the segment regex stopped matching, which the layout probe would report
        as a benign constant column rather than as a failure.
        """
        facts = elf_facts(Path("/bin/true"))
        assert facts.n_segments > 0

    def test_non_elf_degrades_gracefully(self, tmp_path):
        p = tmp_path / "notelf"
        p.write_text("#!/bin/sh\necho hi\n")
        facts = elf_facts(p)
        assert facts.size_b > 0
        assert facts.n_segments == 0

    def test_missing_file_zero_size(self, tmp_path):
        facts = elf_facts(tmp_path / "nope")
        assert facts.size_b == 0

    def test_probe_not_ok_without_readelf(self, monkeypatch, tmp_path):
        """No readelf -> size only, and the row must not read as collected."""
        monkeypatch.setattr(build_mod.shutil, "which", lambda _: None)
        p = tmp_path / "bin"
        p.write_bytes(b"\x7fELF" + b"\x00" * 64)
        facts = elf_facts(p)
        assert facts.size_b > 0
        assert not facts.probe_ok


# ---------------------------------------------------------------------------
# The sweep, with a stubbed toolchain
# ---------------------------------------------------------------------------


@pytest.fixture
def sweep_settings(tmp_path):
    corpus = tmp_path / "corpus"
    corpus.mkdir()
    for i in range(3):
        (corpus / f"p{i}.rs").write_text(
            "// =======================\n// Expected output:\n// ok\n"
            "// =======================\nfn main() {}\n"
        )
    return Settings(
        baseline="A",
        compilers=(
            CompilerSpec(key="A", label="a", cmd=("acc",)),
            CompilerSpec(key="B", label="b", cmd=("bcc",)),
        ),
        configs=(BuildConfig(name="basic"),),
        paths=PathSettings(project_root=str(tmp_path), corpus_dir="corpus"),
    )


@pytest.fixture
def stub_toolchain(monkeypatch):
    """Replace compile/run/trace/elf in the sweep namespace with fakes."""

    def fake_compile(settings, spec, config, prog, out):
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_bytes(b"\x7fELF fake binary")
        return CompileResult(ok=True, wall_s=0.1, size_b=16)

    def fake_run(settings, binary, prog):
        return ExecResult(ok=True, wall_s=0.01, returncode=0, verified=True)

    def fake_trace(settings, binary, norm):
        return TraceResult(
            counts={"read": 4, "write": 2, "openat": 3},
            sequences={"0": ["openat", "read", "write"]},
            args={"openat": {"/etc/x": 3}},
            n_threads=1,
            ok=True,
        )

    def fake_elf(binary):
        return ElfFacts(
            size_b=16,
            n_segments=9,
            n_sections=27,
            interp="/lib/ld",
            n_dynamic_needed=4,
            probe_ok=True,
        )

    monkeypatch.setattr(sweep_mod, "compile_one", fake_compile)
    monkeypatch.setattr(sweep_mod, "run_binary", fake_run)
    monkeypatch.setattr(sweep_mod, "trace_once", fake_trace)
    monkeypatch.setattr(sweep_mod, "elf_facts", fake_elf)


class TestControlGridSweep:
    """The nested grid must actually be walked, not just declared."""

    @staticmethod
    def _run(tmp_path, settings):
        store = RawStore(tmp_path / "raw", settings.fingerprint())
        store.init(settings)
        sweep_mod.Sweeper(settings, store, verbose=False).run()
        return store

    def test_every_group_of_a_3x3_grid_is_traced(
        self, tmp_path, sweep_settings, stub_toolchain
    ):
        s = dataclasses.replace(
            sweep_settings, controls=ControlSettings(builds=3, passes=3)
        )
        counts = self._run(tmp_path, s).read(COUNTS)
        assert set(counts["compiler"]) == set(s.baseline_tags()) | {"B"}
        # Every group carries a full set of reps -- a half-traced group would
        # silently weaken the floor it feeds.
        per_tag = counts.groupby("compiler")["rep"].nunique()
        assert set(per_tag) == {s.trace.reps}

    def test_layout_facts_are_recorded_once_per_build(
        self, tmp_path, sweep_settings, stub_toolchain
    ):
        """ELF facts belong to the binary, and all passes of a build share one.

        Recording them per pass would triple the rows and imply the layout
        differs between trace sessions, which it cannot.
        """
        s = dataclasses.replace(
            sweep_settings, controls=ControlSettings(builds=3, passes=3)
        )
        elf = self._run(tmp_path, s).read(ELF)
        assert set(elf["compiler"]) == set(s.build_tags()) | {"B"}

    def test_a_flat_grid_records_the_baseline_only(
        self, tmp_path, sweep_settings, stub_toolchain
    ):
        s = dataclasses.replace(
            sweep_settings, controls=ControlSettings(builds=1, passes=1)
        )
        counts = self._run(tmp_path, s).read(COUNTS)
        assert set(counts["compiler"]) == {"A", "B"}

    def test_resume_is_keyed_per_group(self, tmp_path, sweep_settings, stub_toolchain):
        """A second run must add nothing: each (tag, file, rep) is already done."""
        s = dataclasses.replace(
            sweep_settings, controls=ControlSettings(builds=3, passes=2)
        )
        store = self._run(tmp_path, s)
        first = len(store.read(COUNTS))
        sweep_mod.Sweeper(s, store, verbose=False).run()
        assert len(store.read(COUNTS)) == first


class TestSweep:
    def test_full_sweep_populates_store(self, tmp_path, sweep_settings, stub_toolchain):
        store = RawStore(tmp_path / "raw", sweep_settings.fingerprint())
        store.init(sweep_settings)
        sweeper = sweep_mod.Sweeper(sweep_settings, store, verbose=False)
        stats = sweeper.run()

        assert stats.compiled > 0
        assert stats.traced > 0
        assert stats.compile_failed == 0
        counts = store.read(COUNTS)
        assert not counts.empty
        # Every control group was recorded alongside the real compilers.
        assert set(sweep_settings.baseline_tags()) | {"B"} <= set(counts["compiler"])

    def test_resume_skips_completed_work(
        self, tmp_path, sweep_settings, stub_toolchain
    ):
        store = RawStore(tmp_path / "raw", sweep_settings.fingerprint())
        store.init(sweep_settings)
        sweep_mod.Sweeper(sweep_settings, store, verbose=False).run()
        first = len(store.read(COUNTS))

        # A second run has nothing new to do; row count must not grow.
        sweep_mod.Sweeper(sweep_settings, store, verbose=False).run()
        assert len(store.read(COUNTS)) == first

    def test_compile_failure_is_recorded_not_fatal(
        self, tmp_path, sweep_settings, monkeypatch, stub_toolchain
    ):
        calls = {"n": 0}
        good = sweep_mod.compile_one

        def flaky(settings, spec, config, prog, out):
            calls["n"] += 1
            if prog.stem == "p1" and spec.key == "A":
                return CompileResult(ok=False, wall_s=0.0, size_b=0, error="boom")
            return good(settings, spec, config, prog, out)

        monkeypatch.setattr(sweep_mod, "compile_one", flaky)
        store = RawStore(tmp_path / "raw", sweep_settings.fingerprint())
        store.init(sweep_settings)
        stats = sweep_mod.Sweeper(sweep_settings, store, verbose=False).run()
        assert stats.compile_failed >= 1
        assert stats.compiled > 0  # the sweep kept going


# ---------------------------------------------------------------------------
# Preflight, with a stubbed strace
# ---------------------------------------------------------------------------


class _FakeProc:
    def __init__(self, stderr: str, returncode: int = 0):
        self.stderr = stderr.encode()
        self.returncode = returncode


VALID_TRACE = '10:00:00.1 openat(AT_FDCWD, "/etc/ld.so.cache", O_RDONLY) = 3\n'


@pytest.fixture
def preflight_settings(tmp_path):
    corpus = tmp_path / "corpus"
    corpus.mkdir()
    (corpus / "hello.rs").write_text("fn main() {}\n")
    from compilerdiv.config import TraceSettings

    return Settings(
        baseline="A",
        compilers=(CompilerSpec(key="A", label="reference", cmd=("cc",)),),
        configs=(BuildConfig(name="basic"),),
        paths=PathSettings(project_root=str(tmp_path), corpus_dir="corpus"),
        trace=TraceSettings(wrapper=(), strace_bin="true"),
    )


def _stub_strace(monkeypatch, stderr: str, returncode: int = 0):
    monkeypatch.setattr(
        preflight_mod.subprocess,
        "run",
        lambda *a, **k: _FakeProc(stderr, returncode),
    )


class TestPreflight:
    def test_all_ok(self, monkeypatch, preflight_settings):
        _stub_strace(monkeypatch, VALID_TRACE)
        monkeypatch.setattr(
            preflight_mod,
            "compile_one",
            lambda *a, **k: CompileResult(ok=True, wall_s=0.1, size_b=10),
        )
        rep = preflight_mod.preflight(preflight_settings)
        assert rep.ok
        assert "preflight passed" in rep.render()
        assert "rust probe" in rep.render()

    def test_compile_failure_makes_report_not_ok(self, monkeypatch, preflight_settings):
        _stub_strace(monkeypatch, VALID_TRACE)
        monkeypatch.setattr(
            preflight_mod,
            "compile_one",
            lambda *a, **k: CompileResult(ok=False, wall_s=0.0, size_b=0, error="nope"),
        )
        rep = preflight_mod.preflight(preflight_settings)
        assert not rep.ok
        assert any("nope" in err for _, err in rep.failures)

    def test_strace_denied_diagnosed(self, monkeypatch, preflight_settings):
        _stub_strace(monkeypatch, "strace: Operation not permitted\n", returncode=1)
        rep = preflight_mod.preflight(preflight_settings)
        assert not rep.ok
        assert "ptrace" in rep.render()

    def test_missing_strace_binary(self, monkeypatch, preflight_settings):
        object.__setattr__(
            preflight_settings.trace, "strace_bin", "definitely-not-here-xyz"
        )
        rep = preflight_mod.preflight(preflight_settings)
        assert not rep.ok
        assert "not found" in rep.render()

    def test_empty_corpus_skips_compile_probe(self, monkeypatch, tmp_path):
        from compilerdiv.config import TraceSettings

        empty = tmp_path / "empty"
        empty.mkdir()
        settings = Settings(
            baseline="A",
            compilers=(CompilerSpec(key="A", label="reference", cmd=("cc",)),),
            configs=(BuildConfig(name="basic"),),
            paths=PathSettings(project_root=str(tmp_path), corpus_dir="empty"),
            trace=TraceSettings(wrapper=(), strace_bin="true"),
        )
        _stub_strace(monkeypatch, VALID_TRACE)
        # compile_one must never be called; make it explode if it is.
        monkeypatch.setattr(
            preflight_mod,
            "compile_one",
            lambda *a, **k: (_ for _ in ()).throw(AssertionError("should not compile")),
        )
        rep = preflight_mod.preflight(settings)
        assert rep.ok
        assert "skipping the compile probe" in rep.render()
