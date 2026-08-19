"""Evidence attached to departure rows after the detectors have run.

The point of this layer is that a departure row should be readable on its own.
Three things were previously impossible to see from one: what values were
passed, how solid the numbers are, and whether the baseline does the same thing
to itself on that file.
"""

from __future__ import annotations

from dataclasses import dataclass

import pandas as pd
import pytest

from compilerdiv.stats.detectors import KIND_ARG, KIND_COUNT, KIND_SET
from compilerdiv.stats.enrich import (
    ARG_CAPTURED,
    ARG_NONE_SEEN,
    ARG_NOT_TRACED,
    ENRICHED_COLUMNS,
    enrich_departures,
)
from compilerdiv.stats.noise import NoiseFloor


@dataclass
class _Frames:
    counts: pd.DataFrame
    args: pd.DataFrame


def _counts(rows):
    return pd.DataFrame(
        rows,
        columns=["config", "compiler", "file", "syscall", "mean_count", "std_count"],
    )


def _args(rows):
    return pd.DataFrame(
        rows, columns=["config", "compiler", "file", "syscall", "arg", "stable"]
    )


def _departure(kind, syscall, **kw):
    row = {
        "config": "basic",
        "pair": "BvA",
        "variant": "B",
        "file": "prog",
        "kind": kind,
        "syscall": syscall,
        "detail": "",
        "magnitude": 1.0,
        "baseline_value": "",
        "variant_value": "",
    }
    row.update(kw)
    return row


@pytest.fixture
def frames():
    return _Frames(
        counts=_counts(
            [
                ("basic", "A", "prog", "connect", 5.0, 0.0),
                ("basic", "B", "prog", "connect", 1.6, 1.2),
                ("basic", "A", "prog", "read", 5.0, 0.0),
                ("basic", "B", "prog", "read", 6.0, 0.0),
                ("basic", "B", "prog", "gettid", 2.0, 0.0),
            ]
        ),
        args=_args(
            [
                ("basic", "A", "prog", "connect", "/var/run/nscd/socket", True),
                ("basic", "A", "prog", "connect", "10.0.0.1", True),
                ("basic", "B", "prog", "connect", "/var/run/nscd/socket", True),
                ("basic", "A", "prog", "connect", "flickers", False),
            ]
        ),
    )


@pytest.fixture
def floor():
    return NoiseFloor(rebuild_delta={("prog", "read"): 1.0})


@pytest.mark.stats
class TestEnrichDepartures:
    def test_adds_every_column_for_every_row(self, frames, floor, settings):
        dep = pd.DataFrame([_departure(KIND_COUNT, "connect", magnitude=-3.4)])
        out = enrich_departures(dep, frames, floor, settings, "basic", "B")
        for col in ENRICHED_COLUMNS:
            assert col in out.columns
            assert out[col].notna().all(), f"{col} left undefined"

    def test_fractional_mean_comes_with_its_dispersion(self, frames, floor, settings):
        """A mean of 1.60 is not 1.6 syscalls; without the sd beside it the
        fraction reads as a bug rather than as an unstable program."""
        dep = pd.DataFrame([_departure(KIND_COUNT, "connect", magnitude=-3.4)])
        out = enrich_departures(dep, frames, floor, settings, "basic", "B").iloc[0]
        assert out["baseline_mean"] == 5.0 and out["baseline_sd"] == 0.0
        assert out["variant_mean"] == pytest.approx(1.6)
        assert out["variant_sd"] == pytest.approx(1.2)

    def test_set_rows_also_get_means(self, frames, floor, settings):
        """A set row that says only "gettid is new" throws away how much of it
        there is. 0 -> 2 is strictly more informative."""
        dep = pd.DataFrame([_departure(KIND_SET, "gettid")])
        out = enrich_departures(dep, frames, floor, settings, "basic", "B").iloc[0]
        assert out["variant_mean"] == 2.0

    def test_stable_arguments_are_attached_unstable_ones_are_not(
        self, frames, floor, settings
    ):
        dep = pd.DataFrame([_departure(KIND_COUNT, "connect", magnitude=-3.4)])
        out = enrich_departures(dep, frames, floor, settings, "basic", "B").iloc[0]
        assert "10.0.0.1" in out["baseline_arg"]
        assert "/var/run/nscd/socket" in out["baseline_arg"]
        assert "flickers" not in out["baseline_arg"]
        assert out["variant_arg"] == "/var/run/nscd/socket"
        assert out["arg_status"] == ARG_CAPTURED

    def test_argument_rows_keep_the_single_value_that_is_the_finding(
        self, frames, floor, settings
    ):
        """Re-deriving the arg set here would widen variant_arg to everything
        the variant passes and lose which value tripped the detector."""
        dep = pd.DataFrame(
            [
                _departure(
                    KIND_ARG,
                    "connect",
                    baseline_value="10.0.0.1",
                    variant_value="/etc/alternate.conf",
                )
            ]
        )
        out = enrich_departures(dep, frames, floor, settings, "basic", "B").iloc[0]
        assert out["variant_arg"] == "/etc/alternate.conf"

    def test_arg_status_explains_an_empty_cell(self, frames, floor, settings):
        """Three reasons a cell is blank, and they are not interchangeable."""
        dep = pd.DataFrame(
            [
                _departure(KIND_COUNT, "connect"),  # traced, values present
                _departure(KIND_SET, "gettid"),  # not in arg_syscalls
                _departure(KIND_COUNT, "openat"),  # traced, nothing observed
            ]
        )
        out = enrich_departures(dep, frames, floor, settings, "basic", "B")
        assert list(out["arg_status"]) == [ARG_CAPTURED, ARG_NOT_TRACED, ARG_NONE_SEEN]

    def test_selfdiff_reports_the_baseline_disagreeing_with_itself(
        self, frames, floor, settings
    ):
        dep = pd.DataFrame(
            [
                _departure(KIND_COUNT, "read", magnitude=1.0),
                _departure(KIND_COUNT, "connect", magnitude=-3.4),
            ]
        )
        out = enrich_departures(dep, frames, floor, settings, "basic", "B")
        assert list(out["baseline_selfdiff"]) == [1.0, 0.0]

    def test_empty_departures_keep_the_schema(self, frames, floor, settings):
        out = enrich_departures(pd.DataFrame(), frames, floor, settings, "basic", "B")
        assert all(c in out.columns for c in ENRICHED_COLUMNS)
