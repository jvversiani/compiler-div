"""Taxonomy tests.

The scenarios here are the real ones from the paper's data:

* a variant's ``gettid(+1)`` on every file        -> uniform
* a variant's ``read(+1)`` on a 59/672 subset     -> conditional
* a hypothetical payload scaling with program    -> program_dependent

OOP layout under the ``taxonomy`` marker (``pytest -m taxonomy``).
"""

from __future__ import annotations

import numpy as np
import pandas as pd
import pytest

from compilerdiv.config import (
    BuildConfig,
    CompilerSpec,
    DetectSettings,
    Settings,
)
from compilerdiv.stats.noise import NoiseFloor
from compilerdiv.stats.taxonomy import (
    CLASS_BUILD_NOISE,
    CLASS_CONDITIONAL,
    CLASS_PROGRAM_DEPENDENT,
    CLASS_RUN_NOISE,
    CLASS_UNIFORM,
    classify,
)

pytestmark = pytest.mark.taxonomy


@pytest.fixture
def settings():
    # Local override of the conftest fixture: the taxonomy needs specific
    # uniform / program-dependent thresholds.
    return Settings(
        baseline="A",
        compilers=(
            CompilerSpec(key="A", label="reference", cmd=("cc",)),
            CompilerSpec(key="B", label="variant", cmd=("varcc",)),
        ),
        configs=(BuildConfig(name="basic"),),
        detect=DetectSettings(uniform_threshold=0.95, program_dependent_rho=0.5),
    )


class FakeFrames:
    def __init__(self, elf, bench):
        self.elf = elf
        self.bench = bench
        self.counts = pd.DataFrame()


def _corpus(n=100, seed=0):
    """n files with varying SLOC and binary size."""
    rng = np.random.default_rng(seed)
    files = [f"prog{i:03d}" for i in range(n)]
    elf = pd.DataFrame(
        {
            "config": "basic",
            "compiler": "A",
            "file": files,
            "size_b": rng.integers(300_000, 900_000, n),
            "n_segments": rng.integers(8, 12, n),
            "n_sections": rng.integers(25, 32, n),
            "interp": "/lib64/ld-linux-x86-64.so.2",
            "n_dynamic_needed": 4,
            "sloc": rng.integers(5, 400, n),
        }
    )
    bench = pd.DataFrame(
        {
            "config": "basic",
            "compiler": "A",
            "file": files,
            "compile_s": 0.5,
            "exec_s": 0.01,
            "size_b": elf["size_b"],
            "n_reps": 3,
        }
    )
    return files, elf, bench


def _departures(files, syscall, magnitude, kind="count"):
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "pair": "BvA",
                "variant": "B",
                "file": f,
                "kind": kind,
                "syscall": syscall,
                "detail": f"{syscall}(+{magnitude})",
                "magnitude": (
                    float(magnitude)
                    if not isinstance(magnitude, dict)
                    else float(magnitude[f])
                ),
                "baseline_value": "0",
                "variant_value": "1",
            }
            for f in files
        ]
    )


class TestClassification:
    def test_uniform_when_every_file_affected(self, settings):
        """A variant's gettid(+1): identical delta, whole corpus."""
        files, elf, bench = _corpus(100)
        dep = _departures(files, "gettid", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        assert len(out) == 1
        r = out.iloc[0]
        assert r["class"] == CLASS_UNIFORM
        assert r["n_files_affected"] == 100
        assert r["frac_affected"] == 1.0
        assert r["magnitude_fixed"]

    def test_conditional_when_fixed_delta_on_a_subset(self, settings):
        """A variant's read(+1): fixed +1, but only 59 of 672 files."""
        files, elf, bench = _corpus(672)
        dep = _departures(files[:59], "read", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["class"] == CLASS_CONDITIONAL
        assert r["n_files_affected"] == 59
        assert r["magnitude_fixed"]
        assert r["frac_affected"] == pytest.approx(59 / 672, abs=1e-4)

    def test_program_dependent_when_delta_scales_with_sloc(self, settings):
        """A payload doing per-item work: delta grows with the program."""
        files, elf, bench = _corpus(80, seed=3)
        sloc = elf.set_index("file")["sloc"].to_dict()
        mags = {f: max(1, int(sloc[f] / 10)) for f in files}
        dep = _departures(files, "write", mags)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["class"] == CLASS_PROGRAM_DEPENDENT
        assert not r["magnitude_fixed"]
        assert abs(r["rho_vs_sloc"]) >= 0.5


class TestNoisePrecedence:
    """A syscall the controls flagged is never promoted to a real class -- but
    only for the set/sequence detectors. The count detector is deliberately
    exempt: its noise floor is per (file, syscall) and already folds in the
    baseline's own build variance, so the coarse corpus-wide verdict must not
    overrule it (see taxonomy._classify_one). These tests therefore exercise the
    precedence on a *sequence* departure, where the global class still stands.
    """

    def test_run_noise_takes_precedence(self, settings):
        files, elf, bench = _corpus(100)
        dep = _departures(files, "futex", 3, kind="sequence")
        floor = NoiseFloor(run_noisy=frozenset({"futex"}))
        out = classify(dep, FakeFrames(elf, bench), floor, settings, "basic", "B")
        assert out.iloc[0]["class"] == CLASS_RUN_NOISE

    def test_build_noise_classified_separately(self, settings):
        files, elf, bench = _corpus(100)
        dep = _departures(files, "read", 1, kind="sequence")
        floor = NoiseFloor(build_noisy=frozenset({"read"}))
        out = classify(dep, FakeFrames(elf, bench), floor, settings, "basic", "B")
        assert out.iloc[0]["class"] == CLASS_BUILD_NOISE

    def test_count_departure_is_exempt_from_global_noise(self, settings):
        """The carve-out itself: a run-noisy syscall as a *count* departure is
        still classified on its own merits, not greyed out."""
        files, elf, bench = _corpus(100)
        dep = _departures(files, "futex", 1, kind="count")
        floor = NoiseFloor(run_noisy=frozenset({"futex"}))
        out = classify(dep, FakeFrames(elf, bench), floor, settings, "basic", "B")
        assert out.iloc[0]["class"] != CLASS_RUN_NOISE


class TestLayoutProbe:
    def test_detects_size_separation(self, settings):
        """When membership tracks binary size, the probe should say so.

        This is the read(+1) layout hypothesis made testable: if the affected
        files are the large ones, membership is a linking artifact.
        """
        files, elf, bench = _corpus(120, seed=7)
        big = list(elf.sort_values("size_b").tail(40)["file"])
        dep = _departures(big, "read", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["class"] == CLASS_CONDITIONAL
        assert r["layout_rho_size"] is not None
        assert abs(r["layout_rho_size"]) > 0.5

    def test_null_when_membership_is_random(self, settings):
        files, elf, bench = _corpus(120, seed=11)
        rng = np.random.default_rng(5)
        picked = list(rng.choice(files, 40, replace=False))
        dep = _departures(picked, "read", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert abs(r["layout_rho_size"] or 0.0) < 0.4
        # A computed-but-weak correlation is real evidence and must say so.
        assert r["layout_status"].startswith("ok")

    def test_status_when_no_elf_data(self, settings):
        """A missing correlation must be distinguishable from a null correlation."""
        files, elf, bench = _corpus(20)
        dep = _departures(files[:5], "read", 1)
        out = classify(
            dep, FakeFrames(pd.DataFrame(), bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["layout_rho_size"] is None
        assert "no ELF data" in r["layout_status"]

    def test_status_when_uniform(self, settings):
        """When every file is affected, layout cannot discriminate -- say so."""
        files, elf, bench = _corpus(50)
        dep = _departures(files, "gettid", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["class"] == CLASS_UNIFORM
        assert "every file affected" in r["layout_status"]

    def test_status_when_variables_constant(self, settings):
        """Constant layout columns are skipped, and the status names them."""
        files, elf, bench = _corpus(30)
        elf = elf.copy()
        elf["size_b"] = 1000
        elf["n_segments"] = 9
        elf["n_sections"] = 27
        elf["sloc"] = 40
        dep = _departures(files[:10], "read", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        r = out.iloc[0]
        assert r["layout_rho_size"] is None
        assert "no usable layout variable" in r["layout_status"]
        assert "size_b" in r["layout_status"]


class TestOutputShape:
    def test_affected_files_list_is_complete(self, settings):
        """manual_review.xlsx must carry the full membership, not a sample."""
        files, elf, bench = _corpus(200)
        affected = files[:37]
        dep = _departures(affected, "read", 1)
        out = classify(
            dep, FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        listed = out.iloc[0]["affected_files"].split("; ")
        assert sorted(listed) == sorted(affected)

    def test_empty_departures_yields_empty_taxonomy(self, settings):
        files, elf, bench = _corpus(10)
        out = classify(
            pd.DataFrame(), FakeFrames(elf, bench), NoiseFloor(), settings, "basic", "B"
        )
        assert out.empty
