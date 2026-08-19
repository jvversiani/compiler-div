"""End-to-end analysis: fabricated raw store -> analyze() -> reports + figures.

One realistic sweep is simulated in the ``analysis_env`` fixture; a single
``analyze()`` call then drives aggregation, all four detectors, the noise floor,
the taxonomy, the significance stats, every plot, the benchmark summary, and the
workbook writers. The assertions verify the *findings* are what the fixture
seeded, so this is a behavioural test and not merely a coverage vehicle.
"""

from __future__ import annotations

import pytest

from compilerdiv.analyze import analyze
from compilerdiv.store.aggregate import Frames

from conftest import build_analysis_env

pytestmark = pytest.mark.integration


@pytest.fixture(scope="module")
def analyzed(tmp_path_factory):
    """Run the full ``analyze()`` once and share its result across the read-only
    assertions -- rendering every figure per test would be needlessly slow."""
    env = build_analysis_env(tmp_path_factory.mktemp("analyzed"))
    res = analyze(env.settings, env.store, verbose=False)
    return env, res


class TestAnalyzePipeline:
    def test_runs_and_writes_outputs(self, analyzed):
        _, res = analyzed
        assert res.workbooks, "no workbooks written"
        assert all(p.exists() for p in res.workbooks)
        assert res.figures, "no figures written"
        assert all(p.exists() for p in res.figures)

    def test_findings_match_seeded_departures(self, analyzed):
        _, res = analyzed
        assert not res.per_file.empty
        assert not res.departures.empty
        assert not res.taxonomy.empty

        kinds = set(res.departures["kind"])
        # The fixture seeded one of every detector's signature.
        assert {"set", "count", "argument", "sequence"} <= kinds

        # statx is present on every B file -> a uniform set departure.
        gr = res.departures[res.departures["syscall"] == "statx"]
        assert not gr.empty

    def test_taxonomy_has_a_uniform_class(self, analyzed):
        _, res = analyzed
        assert "uniform" in set(res.taxonomy["class"])

    def test_bench_summary_present(self, analyzed):
        _, res = analyzed
        assert not res.bench_geomeans.empty
        assert not res.bench_foldchange.empty

    def test_verbose_path_runs(self, analysis_env, capsys):
        # The verbose branch prints the noise-floor and build-variance report;
        # exercise it and confirm it emits something.
        analyze(analysis_env.settings, analysis_env.store, verbose=True)
        out = capsys.readouterr().out
        assert "noise floor" in out

    def test_empty_store_raises(self, tmp_path, settings):
        from compilerdiv.store.raw import RawStore

        store = RawStore(tmp_path / "empty_raw", settings.fingerprint())
        store.init(settings)
        with pytest.raises(RuntimeError, match="no data"):
            analyze(settings, store, verbose=False)


class TestFabricatedFrames:
    """Sanity-checks on the fabricated store, so a fixture regression is caught
    here rather than as a confusing failure deep in analyze()."""

    def test_all_streams_populated(self, analysis_env):
        f = Frames(analysis_env.store)
        assert not f.counts.empty
        assert not f.ngrams.empty
        assert not f.args.empty
        assert not f.bench.empty
        assert not f.elf.empty

    def test_controls_present(self, analysis_env):
        f = Frames(analysis_env.store)
        compilers = set(f.counts["compiler"])
        assert set(analysis_env.settings.baseline_tags()) | {"B"} <= compilers
