"""The nested control grid: ``builds`` x ``passes``.

Three things are worth pinning down. The tag algebra, because every other layer
trusts it to say which group a row belongs to. The variance decomposition,
because separating run noise from build noise is the entire reason the passes
are nested inside the builds rather than sitting beside them. And the pooled
baseline, because without it the extra groups would be acquired and then
ignored.
"""

from __future__ import annotations

import pandas as pd
import pytest

from compilerdiv.config import (
    ConfigError,
    ControlSettings,
    control_tag,
    load_settings,
    parse_control_tag,
)
from compilerdiv.stats.noise import derive_noise
from compilerdiv.store.aggregate import observed_tags, pool_counts, pool_sets

from conftest import FakeFrames, _counts_frame


@pytest.mark.config
class TestTagAlgebra:
    def test_grid_generates_every_group(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        assert s.baseline_tags() == [
            "A",
            "A@0.1",
            "A@0.2",
            "A@1.0",
            "A@1.1",
            "A@1.2",
            "A@2.0",
            "A@2.1",
            "A@2.2",
        ]
        assert len(s.control_tags()) == 8

    def test_baseline_keeps_its_own_key(self, make_settings):
        """(0, 0) must stay `A`: every detector, figure and workbook column
        that names the baseline would otherwise have to be rewritten."""
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        assert s.tag(0, 0) == s.baseline
        assert s.baseline not in s.control_tags()

    def test_coordinates_round_trip(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        for b in range(3):
            for p in range(3):
                tag = s.tag(b, p)
                assert s.tag_build(tag) == b
                assert s.tag_pass(tag) == p

    def test_pass_and_build_groupings_are_orthogonal(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        # All passes of one build: same binary, so differences are run noise.
        assert s.pass_tags(1) == ["A@1.0", "A@1.1", "A@1.2"]
        # One per build: different binaries, so differences include build noise.
        assert s.build_tags() == ["A", "A@1.0", "A@2.0"]
        assert s.rerun_tags(0) == ["A@0.1", "A@0.2"]

    def test_one_build_disables_the_rebuild_control(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=1, passes=3))
        assert not s.controls.rebuild_enabled
        assert s.build_tags() == ["A"]
        assert all(s.tag_build(t) == 0 for t in s.control_tags())

    def test_one_pass_disables_the_rerun_control(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=1))
        assert not s.controls.rerun_enabled
        assert s.rerun_tags(0) == []

    def test_a_flat_grid_has_no_controls_at_all(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=1, passes=1))
        assert s.control_tags() == []
        assert s.baseline_tags() == ["A"]

    def test_zero_is_rejected(self):
        with pytest.raises(ConfigError):
            ControlSettings(builds=0, passes=2)

    def test_parse_rejects_a_plain_compiler_key(self):
        assert parse_control_tag("A") is None
        assert parse_control_tag("rustc_gcc") is None
        assert parse_control_tag("A@1.2") == (1, 2)

    def test_control_tag_helper_matches_settings(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=2, passes=2))
        assert control_tag("A", 1, 1) == s.tag(1, 1)

    def test_budget_is_the_product(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        b = s.trace_budget()
        assert b["baseline_groups"] == 9
        assert b["baseline_compiles"] == 3
        assert b["baseline_traces"] == 9 * s.trace.reps
        # One variant in the fixture settings.
        assert b["traces_per_program"] == 9 * s.trace.reps + s.trace.reps


@pytest.mark.config
class TestLegacyControlSpelling:
    def test_booleans_map_onto_counts(self, write_config):
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "configs:\n"
            "  - {name: basic}\n"
            "controls:\n"
            "  rerun: true\n"
            "  rebuild: false\n"
        )
        c = load_settings(cfg).controls
        assert (c.builds, c.passes) == (1, 2)

    def test_explicit_counts_win(self, write_config):
        cfg = write_config(
            "baseline: A\n"
            "compilers:\n"
            "  - {key: A, label: reference, cmd: [cc]}\n"
            "configs:\n"
            "  - {name: basic}\n"
            "controls:\n"
            "  builds: 3\n"
            "  passes: 3\n"
            "  rebuild: false\n"
        )
        c = load_settings(cfg).controls
        assert (c.builds, c.passes) == (3, 3)

    def test_grid_is_fingerprinted(self, make_settings):
        """The store's contents depend on it, so resuming across a change must
        be refused rather than silently mixing grids."""
        a = make_settings(controls=ControlSettings(builds=2, passes=2))
        b = make_settings(controls=ControlSettings(builds=3, passes=2))
        c = make_settings(controls=ControlSettings(builds=2, passes=3))
        assert len({a.fingerprint(), b.fingerprint(), c.fingerprint()}) == 3


@pytest.mark.noise
class TestVarianceDecomposition:
    @staticmethod
    def _grid_rows(settings, syscall, per_tag):
        rows = []
        for i in range(5):
            f = f"p{i}"
            for tag, val in per_tag.items():
                rows.append((tag, f, syscall, val))
        return _counts_frame(rows)

    def test_within_build_difference_is_run_noise(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        # futex moves between passes of every build, never between builds.
        per_tag = {t: (9.0 if s.tag_pass(t) else 3.0) for t in s.baseline_tags()}
        floor = derive_noise(
            FakeFrames(self._grid_rows(s, "futex", per_tag)), s, "basic"
        )
        assert "futex" in floor.run_noisy
        assert "futex" not in floor.build_noisy

    def test_between_build_difference_is_build_noise(self, make_settings):
        """The claim the rebuild control exists to support: a syscall identical
        across every retrace of a binary but different in another build."""
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        per_tag = {t: 4.0 + s.tag_build(t) for t in s.baseline_tags()}
        floor = derive_noise(
            FakeFrames(self._grid_rows(s, "read", per_tag)), s, "basic"
        )
        assert "read" not in floor.run_noisy
        assert "read" in floor.build_noisy

    def test_rebuild_delta_is_the_worst_pair_not_the_first(self, make_settings):
        """With three builds the floor must reflect the widest disagreement.

        Builds at 4, 5 and 9: comparing only the first rebuild would report a
        floor of 1 and let a delta of 5 through as a departure.
        """
        s = make_settings(controls=ControlSettings(builds=3, passes=1))
        per_tag = {"A": 4.0, "A@1.0": 5.0, "A@2.0": 9.0}
        floor = derive_noise(
            FakeFrames(self._grid_rows(s, "read", per_tag)), s, "basic"
        )
        assert floor.build_floor("p0", "read") == pytest.approx(5.0)

    def test_floor_records_the_grid_it_came_from(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=2))
        floor = derive_noise(FakeFrames(_counts_frame([])), s, "basic")
        assert (floor.n_builds, floor.n_passes) == (3, 2)

    def test_run_noise_still_wins_over_build_noise(self, make_settings):
        """A syscall unstable within a build must not also be blamed on the
        build: the attribution would be unsupported."""
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        per_tag = {
            t: 3.0 + 6.0 * s.tag_pass(t) + 2.0 * s.tag_build(t)
            for t in s.baseline_tags()
        }
        floor = derive_noise(
            FakeFrames(self._grid_rows(s, "futex", per_tag)), s, "basic"
        )
        assert "futex" in floor.run_noisy
        assert "futex" not in floor.build_noisy


@pytest.mark.stats
class TestPooledBaseline:
    def test_counts_pool_to_the_mean_with_between_build_spread(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=1))
        df = _counts_frame(
            [
                ("A", "p0", "read", 4.0),
                ("A@1.0", "p0", "read", 6.0),
                ("A@2.0", "p0", "read", 8.0),
            ]
        )
        out = pool_counts(df, "basic", s.baseline_tags())
        row = out.iloc[0]
        assert row["mean_count"] == pytest.approx(6.0)
        # The groups' own sd is 0; the spread between them is not, and that is
        # what a single group cannot see.
        assert row["std_count"] > 1.0

    def test_a_syscall_missing_from_one_build_widens_the_spread(self, make_settings):
        """ "That build never made the call" is an observation about variance,
        not a row to skip.

        Distinct from a build missing from the store altogether: build 1 is
        present here (it recorded ``read``), it simply never called ``statx``.
        That zero was measured, so it belongs in the spread.
        """
        s = make_settings(controls=ControlSettings(builds=2, passes=1))
        df = _counts_frame(
            [
                ("A", "p0", "statx", 4.0),
                ("A", "p0", "read", 1.0),
                ("A@1.0", "p0", "read", 1.0),
            ]
        )
        out = pool_counts(df, "basic", s.baseline_tags())
        row = out[out["syscall"] == "statx"].iloc[0]
        assert row["mean_count"] == pytest.approx(2.0)
        assert row["std_count"] > 0.0

    def test_groups_absent_from_the_store_are_not_averaged_in(self, make_settings):
        """Regression: a store from a different grid must not deflate the mean.

        Declaring 9 groups and finding 1 used to average the real measurement
        against 8 fabricated zeros, dividing every baseline count by nine and
        turning the whole corpus into departures.
        """
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        df = _counts_frame([("A", "p0", "read", 9.0)])
        out = pool_counts(df, "basic", s.baseline_tags())
        assert out.iloc[0]["mean_count"] == pytest.approx(9.0)

    def test_missing_groups_are_reported_not_guessed(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=3, passes=3))
        df = _counts_frame([("A", "p0", "read", 9.0), ("A@1.0", "p0", "read", 9.0)])
        assert observed_tags(df, "basic", s.baseline_tags()) == ["A", "A@1.0"]

    def test_sets_pool_to_the_union(self, make_settings):
        """A syscall any baseline build makes is not new under a variant."""
        s = make_settings(controls=ControlSettings(builds=2, passes=1))
        df = pd.DataFrame(
            [
                {
                    "config": "basic",
                    "compiler": "A",
                    "file": "p0",
                    "syscall": "read",
                    "stable": True,
                },
                {
                    "config": "basic",
                    "compiler": "A@1.0",
                    "file": "p0",
                    "syscall": "statx",
                    "stable": True,
                },
            ]
        )
        out = pool_sets(df, "basic", s.baseline_tags())
        assert set(out["syscall"]) == {"read", "statx"}
        assert set(out["compiler"]) == {"A"}

    def test_unstable_rows_never_enter_the_pool(self, make_settings):
        s = make_settings(controls=ControlSettings(builds=2, passes=1))
        df = pd.DataFrame(
            [
                {
                    "config": "basic",
                    "compiler": "A",
                    "file": "p0",
                    "syscall": "read",
                    "stable": True,
                },
                {
                    "config": "basic",
                    "compiler": "A@1.0",
                    "file": "p0",
                    "syscall": "flaky",
                    "stable": False,
                },
            ]
        )
        out = pool_sets(df, "basic", s.baseline_tags())
        assert set(out["syscall"]) == {"read"}
