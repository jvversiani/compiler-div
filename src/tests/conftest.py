"""Shared fixtures and helpers for the compilerdiv test suite.

The suite is organised as OOP: related assertions live together in ``Test*``
classes, and every class carries a pytest marker (``@pytest.mark.<area>``) so a
single area can be run from the command line, e.g.::

    pytest -m languages          # only the multi-language tests
    pytest -m "corpus or config" # two areas
    pytest -k Detector           # by class/name
    pytest src/tests/test_store.py::TestRoundTrip

The fixtures here are deliberately *factories* (``make_settings``,
``write_config``, ``make_corpus``) so a test can ask for exactly the shape it
needs without a web of narrow fixtures.
"""

from __future__ import annotations

from pathlib import Path
from typing import Callable

import matplotlib
import pandas as pd
import pytest

# The plotting tests render real figures; force a headless backend so they work
# in CI and on machines with no display.
matplotlib.use("Agg")

from compilerdiv.config import (
    BuildConfig,
    CompilerSpec,
    ControlSettings,
    DetectSettings,
    Settings,
    TraceSettings,
    BenchSettings,
)
from compilerdiv.corpus import LanguageSpec

# ---------------------------------------------------------------------------
# Settings factories
# ---------------------------------------------------------------------------


@pytest.fixture
def make_settings() -> Callable[..., Settings]:
    """Return a factory that builds a :class:`Settings` with A/B defaults.

    Any field can be overridden by keyword; the two-compiler ``A`` (baseline) /
    ``B`` shape is what most detector and noise tests want.
    """

    def _factory(**overrides) -> Settings:
        base = dict(
            baseline="A",
            compilers=(
                CompilerSpec(key="A", label="reference", cmd=("cc",)),
                CompilerSpec(key="B", label="variant", cmd=("varcc",)),
            ),
            configs=(BuildConfig(name="basic"),),
        )
        base.update(overrides)
        return Settings(**base)

    return _factory


@pytest.fixture
def settings(make_settings) -> Settings:
    """The common A/B settings object, for tests that need no customisation."""
    return make_settings(detect=DetectSettings(auto_noisy_min_files=3))


# ---------------------------------------------------------------------------
# YAML config on disk
# ---------------------------------------------------------------------------


@pytest.fixture
def write_config(tmp_path) -> Callable[[str], Path]:
    """Write a YAML config to a temp file and return its path."""

    def _write(text: str, name: str = "c.yaml") -> Path:
        p = tmp_path / name
        p.write_text(text)
        return p

    return _write


MINIMAL_CONFIG = (
    "baseline: A\n"
    "compilers:\n"
    "  - {key: A, label: reference, cmd: [cc]}\n"
    "configs:\n"
    "  - {name: basic}\n"
)


# ---------------------------------------------------------------------------
# Corpus on disk
# ---------------------------------------------------------------------------


def make_header(expected: list[str] | None, line_comment: str = "//") -> str:
    """Build an embedded expected-output header for a given comment marker.

    ``expected=None`` emits the explicit "no expected output" sentinel.
    """
    c = line_comment
    lines = [
        f"{c} Rosetta Code task: Probe",
        f"{c} Source: example.org/probe",
        f"{c} =======================",
        f"{c} Expected output:",
    ]
    if expected is None:
        lines.append(f"{c} (no expected output provided on Rosetta Code)")
    else:
        lines += [f"{c} {line}" for line in expected]
    lines.append(f"{c} =======================")
    return "\n".join(lines) + "\n"


@pytest.fixture
def make_corpus(tmp_path) -> Callable[..., Path]:
    """Return a factory that materialises a corpus directory on disk.

    ``files`` maps a filename to its full source text. Returns the directory.
    """

    def _factory(files: dict[str, str], subdir: str = "corpus") -> Path:
        d = tmp_path / subdir
        d.mkdir(parents=True, exist_ok=True)
        for name, text in files.items():
            (d / name).write_text(text)
        return d

    return _factory


# ---------------------------------------------------------------------------
# Reusable language specs
# ---------------------------------------------------------------------------


@pytest.fixture
def py_like_language() -> LanguageSpec:
    """A ``#``-comment, no-block language, for exercising non-C-family paths."""
    return LanguageSpec(
        name="pylike",
        extension=".pyl",
        line_comment="#",
        block_comment=None,
        hello_world='print("ok")\n',
    )


# ---------------------------------------------------------------------------
# A fully fabricated raw store, for driving the whole analysis pipeline
# ---------------------------------------------------------------------------


class AnalysisEnv:
    """Bundle returned by the ``analysis_env`` fixture."""

    def __init__(self, settings, store, corpus_dir):
        self.settings = settings
        self.store = store
        self.corpus_dir = corpus_dir


class FakeFrames:
    """A minimal stand-in for ``Frames`` for the noise tests.

    ``derive_noise`` only reads ``counts`` and ``ngrams``; building a real store
    to exercise a threshold would obscure what the test is about.
    """

    def __init__(self, counts, ngrams=None):
        self.counts = counts
        self.ngrams = (
            ngrams
            if ngrams is not None
            else pd.DataFrame(
                columns=[
                    "config",
                    "compiler",
                    "file",
                    "n",
                    "ngram",
                    "stable",
                    "n_threads",
                ]
            )
        )
        self.elf = pd.DataFrame()
        self.bench = pd.DataFrame()


def _counts_frame(rows):
    """``(compiler, file, syscall, mean)`` tuples -> a counts frame."""
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "mean_count": m,
                "std_count": 0.0,
                "n_reps": 3,
            }
            for c, f, s, m in rows
        ]
    )


def build_analysis_env(tmp_path: Path) -> AnalysisEnv:
    """Build a realistic raw store and matching settings for ``analyze()``.

    The dataset is small but deliberately *interesting*: it contains a genuine
    behavioural departure of every kind, plus run-noise and build-noise seeded
    through the controls, so a single ``analyze()`` run reaches the detector,
    taxonomy, noise, benchmark, plotting and reporting code at once.

        * B gains ``statx`` on every file          -> uniform set departure
        * B does ``write`` +1 on files 0..3        -> conditional count departure
        * B opens a different path on file 0       -> argument departure
        * B reorders an n-gram on file 0           -> sequence departure
        * passes 1+ differ from pass 0 in ``futex``-> run-noise
        * builds 1+ differ from build 0 in ``close``-> build-noise

    The controls are a ``builds x passes`` grid (2x2 here, matching the
    defaults). Run noise is seeded *within* a build and build noise *between*
    builds, so the two classes are separable exactly as the real design intends.

    ``statx`` and ``close`` are chosen deliberately: they are outside the
    default ``departure_ignore`` set, and ``close`` does not appear in the
    reordered n-gram, so seeding build-noise on it does not silence the sequence
    departure (the sequence detector drops any n-gram containing a noisy call).
    """
    from compilerdiv.config import (
        BuildConfig,
        CompilerSpec,
        ControlSettings,
        PathSettings,
        Settings,
    )
    from compilerdiv.store.raw import ARGS, BENCH, COUNTS, ELF, SEQUENCE, RawStore

    n_files = 8
    reps = 3
    files = [f"prog{i:03d}" for i in range(n_files)]
    base_syscalls = {
        "execve": 1,
        "brk": 2,
        "openat": 3,
        "read": 4,
        "write": 2,
        "close": 3,
        "mmap": 5,
        "futex": 2,
    }

    out_dir = tmp_path / "out"
    corpus_dir = tmp_path / "corpus"
    corpus_dir.mkdir()
    for i in range(3):  # a few real sources so the SLOC figure renders
        (corpus_dir / f"{files[i]}.rs").write_text(
            make_header([f"line {i}"], "//") + f"fn main() {{ /* {i} */ }}\n"
        )

    settings = Settings(
        baseline="A",
        compilers=(
            CompilerSpec(key="A", label="reference", cmd=("cc",), color="#4C72B0"),
            CompilerSpec(key="B", label="variant", cmd=("varcc",), color="#55A868"),
        ),
        configs=(BuildConfig(name="basic"),),
        paths=PathSettings(
            project_root=str(tmp_path),
            corpus_dir="corpus",
            out_dir="out",
        ),
        controls=ControlSettings(builds=2, passes=2),
    )

    store = RawStore(tmp_path / "raw", settings.fingerprint())
    store.init(settings)

    baseline_tags = settings.baseline_tags()
    all_tags = ["B", *baseline_tags]

    def counts_for(compiler: str, fi: int) -> dict[str, int]:
        c = dict(base_syscalls)
        if compiler == "B":
            c["statx"] = 1  # uniform new-syscall departure
            if fi < 4:
                c["write"] += 1  # conditional count departure
            return c
        build, pass_ = settings.tag_coord(compiler)
        if pass_ > 0 and fi < 4:
            c["futex"] = 5  # run-noise: differs WITHIN a build
        if build > 0 and fi < 3:
            c["close"] += 1  # build-noise: differs BETWEEN builds
        return c

    def ngrams_for(compiler: str, fi: int) -> list[str]:
        grams = ["execve→brk→openat", "openat→read→write"]
        if compiler == "B" and fi == 0:
            grams = ["execve→brk→openat", "read→openat→write"]  # reorder
        return grams

    def openat_arg(compiler: str, fi: int) -> str:
        if compiler == "B" and fi == 0:
            return "/etc/alternate.conf"  # argument departure
        return "/etc/ld.so.cache"

    for fi, f in enumerate(files):
        for compiler in all_tags:
            cs = counts_for(compiler, fi)
            for rep in range(1, reps + 1):
                for sc, n in cs.items():
                    store.add(COUNTS, ["basic", compiler, f, rep, sc, n])
                for g in ngrams_for(compiler, fi):
                    store.add(SEQUENCE, ["basic", compiler, f, rep, 3, g, 1])
                store.add(
                    ARGS,
                    ["basic", compiler, f, rep, "openat", openat_arg(compiler, fi), 3],
                )
        # bench + elf only for the real compilers
        for compiler in ("A", "B"):
            size = 300_000 + fi * 50_000 + (1000 if compiler == "B" else 0)
            for rep in range(1, reps + 1):
                store.add(
                    BENCH,
                    [
                        "basic",
                        compiler,
                        f,
                        rep,
                        0.5 + fi * 0.01,
                        0.01 + fi * 0.001,
                        size,
                        True,
                        0,
                    ],
                )
            store.add(
                ELF,
                [
                    "basic",
                    compiler,
                    f,
                    size,
                    9,
                    27,
                    "/lib64/ld-linux-x86-64.so.2",
                    4,
                    10 + fi * 5,
                ],
            )

    store.flush()
    return AnalysisEnv(settings, store, corpus_dir)


@pytest.fixture
def analysis_env(tmp_path) -> AnalysisEnv:
    """Function-scoped fabricated environment (fresh store per test)."""
    return build_analysis_env(tmp_path)
