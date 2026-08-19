"""The workbook's presentation layer.

Two things are worth pinning down here. The first is the glossary: a readme
maintained by hand drifts away from the columns it describes, silently, and a
stale glossary is worse than none because a reader trusts it. The test below
makes drift a test failure.

The second is the departure projection, which exists to undo a specific defect:
``magnitude`` meant four different things depending on ``kind``, and ``detail``
restated it as a *rounded* string (``connect(-3)`` for a delta of ``-3.4``).
The assertions here are about not reintroducing either.
"""

from __future__ import annotations

import pandas as pd
import pytest

from compilerdiv.analyze import analyze
from compilerdiv.report import (
    GLOSSARY,
    SHEETS,
    build_sheets,
    glossary_frame,
    project_departures,
)
from compilerdiv.stats.detectors import KIND_ARG, KIND_COUNT, KIND_SEQ, KIND_SET

from conftest import build_analysis_env


@pytest.fixture(scope="module")
def sheets(tmp_path_factory):
    env = build_analysis_env(tmp_path_factory.mktemp("report"))
    res = analyze(env.settings, env.store, verbose=False)
    return build_sheets(
        res.per_file, res.departures, res.taxonomy, res.significance, res.noise
    )


@pytest.mark.cli
class TestGlossary:
    def test_covers_every_sheet(self, sheets):
        documented = {s for s, _, _ in GLOSSARY}
        written = {name for name in SHEETS if name != "readme"}
        assert documented == written

    def test_covers_every_column_exactly(self, sheets):
        """No undocumented column, and no glossary entry for a column that is
        no longer written. Both directions matter -- the second is how a
        glossary rots after a rename."""
        for name, df in sheets.items():
            if name == "readme" or df.empty:
                continue
            documented = {c for s, c, _ in GLOSSARY if s == name}
            assert documented == set(df.columns), f"glossary mismatch on {name!r}"

    def test_readme_carries_notes_and_priors(self):
        g = glossary_frame()
        assert set(g.columns) == {"sheet", "column", "meaning"}
        assert (g["sheet"] == "(note)").sum() >= 5
        assert set(g[g["sheet"] == "(class)"]["column"]) >= {
            "uniform",
            "conditional",
            "program_dependent",
        }

    def test_no_placeholder_meanings(self):
        """Some entries are legitimately terse ("The syscall."); none may be
        empty or a stub."""
        for _, col, meaning in GLOSSARY:
            assert meaning.strip().endswith("."), col
            assert len(meaning) > 10 and "TODO" not in meaning, col


@pytest.mark.cli
class TestDepartureProjection:
    @staticmethod
    def _row(kind, magnitude, **kw):
        base = {
            "config": "basic",
            "variant": "B",
            "variant_label": "mrustc",
            "file": "prog",
            "kind": kind,
            "syscall": "connect",
            "detail": "ignored",
            "magnitude": magnitude,
            "baseline_value": "5.00",
            "variant_value": "1.60",
            "baseline_mean": 5.0,
            "variant_mean": 1.6,
            "baseline_sd": 0.0,
            "variant_sd": 1.2,
            "baseline_arg": "",
            "variant_arg": "",
            "arg_status": "captured",
            "baseline_selfdiff": 0.0,
        }
        base.update(kw)
        return base

    def test_count_delta_is_exact_not_rounded(self):
        out = project_departures(pd.DataFrame([self._row(KIND_COUNT, -3.4)]))
        assert out.loc[0, "delta"] == pytest.approx(-3.4)
        assert out.loc[0, "direction"] == "fewer"

    def test_delta_is_blank_where_there_was_no_quantity(self):
        """``set`` used magnitude as a ``±1`` direction flag and ``argument``
        as a constant ``1.0``. Neither is a number anyone should average."""
        out = project_departures(
            pd.DataFrame([self._row(KIND_SET, 1.0), self._row(KIND_ARG, 1.0)])
        )
        assert out["delta"].isna().all()
        assert list(out["direction"]) == ["new", "new value"]

    def test_set_direction_distinguishes_new_from_missing(self):
        out = project_departures(
            pd.DataFrame([self._row(KIND_SET, 1.0), self._row(KIND_SET, -1.0)])
        )
        assert list(out["direction"]) == ["new", "missing"]

    def test_sequence_keeps_its_evidence_and_count(self):
        out = project_departures(
            pd.DataFrame(
                [
                    self._row(
                        KIND_SEQ,
                        3.0,
                        detail="3 new stable n-gram(s): a->b",
                        baseline_value="50 stable n-grams",
                        variant_value="56 stable n-grams",
                    )
                ]
            )
        )
        assert out.loc[0, "delta"] == 3.0
        assert "a->b" in out.loc[0, "evidence"]
        assert "50 stable n-grams" in out.loc[0, "evidence"]

    def test_non_sequence_rows_have_no_evidence_text(self):
        out = project_departures(pd.DataFrame([self._row(KIND_COUNT, -3.4)]))
        assert out.loc[0, "evidence"] == ""

    def test_variant_carries_key_and_label(self):
        out = project_departures(pd.DataFrame([self._row(KIND_COUNT, 1.0)]))
        assert out.loc[0, "variant"] == "B (mrustc)"
        assert "variant_label" not in out.columns

    def test_empty_input_still_has_the_schema(self):
        out = project_departures(pd.DataFrame())
        assert not out.empty is True or list(out.columns)  # schema, not rows
        assert "delta" in out.columns and "magnitude" not in out.columns


@pytest.mark.cli
class TestSheetContents:
    def test_one_workbook_with_the_expected_sheets(self, sheets):
        assert list(sheets) == SHEETS

    def test_departures_dropped_the_overloaded_columns(self, sheets):
        cols = set(sheets["departures"].columns)
        assert not cols & {"magnitude", "detail", "pair", "baseline_value"}

    def test_per_file_ranks_by_worst_class_not_by_count(self, sheets):
        pf = sheets["per_file"]
        assert "worst_class" in pf.columns
        assert "detail" not in pf.columns
