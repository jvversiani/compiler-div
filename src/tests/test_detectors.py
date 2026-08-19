"""Detector tests, one per stated requirement.

These are the tests that matter: each corresponds to a scenario the paper claims
the pipeline can detect. OOP layout under the ``detectors`` marker; the four
``_frame`` builders below construct the aggregated inputs each detector expects.
"""

from __future__ import annotations

import pandas as pd
import pytest

from compilerdiv.stats.detectors import (
    KIND_ARG,
    KIND_COUNT,
    KIND_INSTAB,
    KIND_SEQ,
    KIND_SET,
    detect_args,
    detect_count,
    detect_instability,
    detect_sequence,
    detect_set,
    all_departures,
)

pytestmark = pytest.mark.detectors

NONE = frozenset()


SET_COLUMNS = [
    "config",
    "compiler",
    "file",
    "syscall",
    "n_reps_present",
    "n_reps_total",
    "stable",
]
COUNT_COLUMNS = [
    "config",
    "compiler",
    "file",
    "syscall",
    "mean_count",
    "std_count",
    "n_reps",
]
ARG_COLUMNS = SET_COLUMNS[:4] + [
    "arg",
    "n_reps_present",
    "n_reps_total",
    "stable",
    "mean_count",
]
NGRAM_COLUMNS = [
    "config",
    "compiler",
    "file",
    "n",
    "ngram",
    "n_reps_present",
    "n_reps_total",
    "stable",
    "n_threads",
]


def _sets(rows):
    """rows: (compiler, file, syscall, stable)"""
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "n_reps_present": 3 if st else 1,
                "n_reps_total": 3,
                "stable": st,
            }
            for c, f, s, st in rows
        ],
        columns=SET_COLUMNS,
    )


def _counts(rows):
    """rows: (compiler, file, syscall, mean, std)"""
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "mean_count": m,
                "std_count": sd,
                "n_reps": 3,
            }
            for c, f, s, m, sd in rows
        ],
        columns=COUNT_COLUMNS,
    )


def _args(rows):
    """rows: (compiler, file, syscall, arg, stable)"""
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "arg": a,
                "n_reps_present": 3 if st else 1,
                "n_reps_total": 3,
                "stable": st,
                "mean_count": 1.0,
            }
            for c, f, s, a, st in rows
        ],
        columns=ARG_COLUMNS,
    )


#: What ``counts_agg`` really produces. The three extra columns describe the
#: *shape* of the rep sample rather than its centre, which is what the
#: instability detector reads; ``_counts`` above omits them on purpose so the
#: older tests keep exercising the graceful-degradation path.
REP_COUNT_COLUMNS = [
    "config",
    "compiler",
    "file",
    "syscall",
    "mean_count",
    "std_count",
    "min_count",
    "max_count",
    "n_distinct",
    "n_reps",
]


def _from_reps(rows):
    """rows: ``(compiler, file, syscall, [count per rep])``, ``None`` = did not fire.

    Returns ``(syscall_sets, counts)`` derived exactly as the aggregate layer
    derives them, so a test states the rep sample it means and never a summary
    of it. Writing ``mean``/``std`` by hand is how a fixture ends up describing
    a sample that could not exist.
    """
    totals: dict[tuple[str, str], int] = {}
    for c, f, _s, vals in rows:
        totals[(c, f)] = max(totals.get((c, f), 0), len(vals))

    set_rows, count_rows = [], []
    for c, f, s, vals in rows:
        seen = [v for v in vals if v is not None]
        total = totals[(c, f)]
        set_rows.append(
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "n_reps_present": len(seen),
                "n_reps_total": total,
                "stable": len(seen) == total,
            }
        )
        if not seen:
            continue
        mean = sum(seen) / len(seen)
        var = sum((v - mean) ** 2 for v in seen) / len(seen)
        count_rows.append(
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "syscall": s,
                "mean_count": float(mean),
                "std_count": float(var**0.5),
                "min_count": float(min(seen)),
                "max_count": float(max(seen)),
                "n_distinct": len(set(seen)),
                "n_reps": len(seen),
            }
        )
    return (
        pd.DataFrame(set_rows, columns=SET_COLUMNS),
        pd.DataFrame(count_rows, columns=REP_COUNT_COLUMNS),
    )


def _ngrams(rows):
    """rows: (compiler, file, ngram, stable, n_threads)"""
    return pd.DataFrame(
        [
            {
                "config": "basic",
                "compiler": c,
                "file": f,
                "n": 3,
                "ngram": g,
                "n_reps_present": 3 if st else 1,
                "n_reps_total": 3,
                "stable": st,
                "n_threads": nt,
            }
            for c, f, g, st, nt in rows
        ],
        columns=NGRAM_COLUMNS,
    )


class TestReq1NewSyscallType:
    def test_new_syscall_flagged(self):
        """A has no write; B has write. Must be flagged."""
        df = _sets(
            [
                ("A", "prog", "openat", True),
                ("A", "prog", "read", True),
                ("B", "prog", "openat", True),
                ("B", "prog", "read", True),
                ("B", "prog", "write", True),
            ]
        )
        out = detect_set(df, "basic", "A", "B", NONE)
        assert len(out) == 1
        r = out.iloc[0]
        assert r["kind"] == KIND_SET
        assert r["syscall"] == "write"
        assert r["detail"] == "new syscall type"
        assert r["variant_value"] == "present"

    def test_missing_syscall_also_flagged(self):
        df = _sets(
            [
                ("A", "prog", "openat", True),
                ("A", "prog", "write", True),
                ("B", "prog", "openat", True),
            ]
        )
        out = detect_set(df, "basic", "A", "B", NONE)
        assert len(out) == 1
        assert out.iloc[0]["detail"] == "missing syscall type"

    def test_unstable_syscall_not_flagged(self):
        """A syscall appearing in only some reps of B is noise, not a departure."""
        df = _sets(
            [
                ("A", "prog", "openat", True),
                ("B", "prog", "openat", True),
                ("B", "prog", "write", False),  # flickers
            ]
        )
        out = detect_set(df, "basic", "A", "B", NONE)
        assert out.empty

    def test_ignored_syscall_suppressed(self):
        df = _sets(
            [
                ("A", "prog", "openat", True),
                ("B", "prog", "openat", True),
                ("B", "prog", "write", True),
            ]
        )
        out = detect_set(df, "basic", "A", "B", frozenset({"write"}))
        assert out.empty


class TestReq2DifferentArgument:
    def test_different_argument_flagged_at_identical_counts(self):
        """A opens /etc/passwd, B opens /lib/x.so. Counts identical."""
        df = _args(
            [
                ("A", "prog", "openat", "/etc/passwd", True),
                ("B", "prog", "openat", "/lib/x.so", True),
            ]
        )
        out = detect_args(df, "basic", "A", "B", NONE)
        kinds = set(out["kind"])
        assert kinds == {KIND_ARG}
        details = " ".join(out["detail"])
        assert "/lib/x.so" in details  # new under B -- the reported direction
        # The mirror ("/etc/passwd absent under B") is the same substitution and
        # is deliberately not emitted, so it does not double-count.
        assert "/etc/passwd" not in details
        assert len(out) == 1
        assert out.iloc[0]["magnitude"] == pytest.approx(1.0)

    def test_count_detector_is_blind_to_this(self):
        """Demonstrates why the argument detector exists at all."""
        df = _counts(
            [
                ("A", "prog", "openat", 1.0, 0.0),
                ("B", "prog", "openat", 1.0, 0.0),
            ]
        )
        out = detect_count(df, "basic", "A", "B", NONE, NONE, 0.5)
        assert out.empty

    def test_identical_arguments_not_flagged(self):
        df = _args(
            [
                ("A", "prog", "openat", "/etc/ld.so.cache", True),
                ("B", "prog", "openat", "/etc/ld.so.cache", True),
            ]
        )
        out = detect_args(df, "basic", "A", "B", NONE)
        assert out.empty

    def test_unstable_argument_not_flagged(self):
        df = _args(
            [
                ("A", "prog", "openat", "/etc/x", True),
                ("B", "prog", "openat", "/etc/x", True),
                ("B", "prog", "openat", "/tmp/flaky", False),
            ]
        )
        out = detect_args(df, "basic", "A", "B", NONE)
        assert out.empty


class TestReq3Reordering:
    def test_reorder_flagged_when_stable(self):
        """Same syscalls, same counts, different order."""
        df = _ngrams(
            [
                ("A", "prog", "openat→read→close", True, 1),
                ("B", "prog", "read→openat→close", True, 1),
            ]
        )
        out = detect_sequence(df, "basic", "A", "B", NONE)
        assert len(out) == 1
        assert out.iloc[0]["kind"] == KIND_SEQ
        assert "read→openat→close" in out.iloc[0]["detail"]

    def test_unstable_ngrams_do_not_fire(self):
        """Scheduler-dependent n-grams are dropped by the rep intersection."""
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 4),
                ("B", "prog", "a→b→c", True, 4),
                ("B", "prog", "x→y→z", False, 4),  # only some reps
            ]
        )
        out = detect_sequence(df, "basic", "A", "B", NONE)
        assert out.empty

    def test_threaded_file_is_annotated(self):
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 8),
                ("B", "prog", "q→r→s", True, 8),
            ]
        )
        out = detect_sequence(df, "basic", "A", "B", NONE)
        assert len(out) == 1
        assert "8 threads" in out.iloc[0]["detail"]
        assert "weak" in out.iloc[0]["detail"]

    def test_ngram_containing_ignored_syscall_dropped(self):
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "a→futex→c", True, 1),
            ]
        )
        out = detect_sequence(df, "basic", "A", "B", frozenset({"futex"}))
        assert out.empty


class TestCountDetector:
    def test_delta_above_tolerance_flagged(self):
        df = _counts(
            [
                ("A", "prog", "read", 4.0, 0.0),
                ("B", "prog", "read", 5.0, 0.0),
            ]
        )
        out = detect_count(df, "basic", "A", "B", NONE, NONE, 0.5)
        assert len(out) == 1
        assert out.iloc[0]["kind"] == KIND_COUNT
        assert out.iloc[0]["magnitude"] == pytest.approx(1.0)
        assert out.iloc[0]["detail"] == "read(+1)"

    def test_delta_below_file_noise_floor_not_flagged(self):
        """A syscall that jitters on this program cannot produce a departure."""
        df = _counts(
            [
                ("A", "prog", "read", 4.0, 2.0),  # std 2.0 swamps a +1
                ("B", "prog", "read", 5.0, 0.0),
            ]
        )
        out = detect_count(df, "basic", "A", "B", NONE, NONE, 0.5)
        assert out.empty

    def test_noisy_syscall_excluded(self):
        df = _counts(
            [
                ("A", "prog", "futex", 4.0, 0.0),
                ("B", "prog", "futex", 40.0, 0.0),
            ]
        )
        out = detect_count(df, "basic", "A", "B", NONE, frozenset({"futex"}), 0.5)
        assert out.empty

    def test_negative_delta_direction_recorded(self):
        df = _counts(
            [
                ("A", "prog", "read", 5.0, 0.0),
                ("B", "prog", "read", 3.0, 0.0),
            ]
        )
        out = detect_count(df, "basic", "A", "B", NONE, NONE, 0.5)
        assert out.iloc[0]["detail"] == "read(-2)"
        assert out.iloc[0]["magnitude"] == pytest.approx(-2.0)

    def test_empty_input_returns_empty_frame_not_error(self):
        empty = pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "syscall",
                "mean_count",
                "std_count",
                "n_reps",
            ]
        )
        assert detect_count(empty, "basic", "A", "B", NONE, NONE, 0.5).empty


# ---------------------------------------------------------------------------
# Subsumption
# ---------------------------------------------------------------------------


class _Frames:
    """The four aggregated inputs ``all_departures`` reads, fabricated."""

    def __init__(self, syscall_sets=None, counts=None, args=None, ngrams=None):
        self.syscall_sets = syscall_sets if syscall_sets is not None else _sets([])
        self.counts = counts if counts is not None else _counts([])
        self.args = args if args is not None else _args([])
        self.ngrams = ngrams if ngrams is not None else _ngrams([])


class TestSetSubsumption:
    """A syscall type the baseline never makes trips the count detector (it is
    +N of something), the sequence detector (it sits in n-grams the baseline
    never produced) and, if it carries a string, the argument detector too. All
    four then report the same event. The set row is the sharpest statement of
    it, so it is the only one kept.
    """

    #: gettid under rustc_gcc: new type, +3 count, inside two new n-grams.
    GETTID = dict(
        syscall_sets=_sets(
            [
                ("A", "prog", "openat", True),
                ("B", "prog", "openat", True),
                ("B", "prog", "gettid", True),
            ]
        ),
        counts=_counts(
            [
                ("A", "prog", "openat", 2.0, 0.0),
                ("B", "prog", "openat", 2.0, 0.0),
                ("A", "prog", "gettid", 0.0, 0.0),
                ("B", "prog", "gettid", 3.0, 0.0),
            ]
        ),
        ngrams=_ngrams(
            [
                ("A", "prog", "openat→read→close", True, 1),
                ("B", "prog", "openat→read→close", True, 1),
                ("B", "prog", "gettid→read→close", True, 1),
                ("B", "prog", "read→gettid→close", True, 1),
            ]
        ),
    )

    def test_only_the_set_row_survives(self, settings):
        out = all_departures(_Frames(**self.GETTID), settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_SET]
        assert out.iloc[0]["syscall"] == "gettid"

    def test_without_subsumption_all_three_would_fire(self, settings):
        """Guards the premise: the inputs really do trip the other detectors."""
        f = _Frames(**self.GETTID)
        assert not detect_count(f.counts, "basic", "A", "B", NONE, NONE, 0.5).empty
        assert not detect_sequence(f.ngrams, "basic", "A", "B", NONE).empty

    def test_other_syscalls_on_the_same_file_still_fire(self, settings):
        """Subsumption is keyed by syscall, not by file."""
        frames = _Frames(
            syscall_sets=self.GETTID["syscall_sets"],
            counts=_counts(
                [
                    ("A", "prog", "gettid", 0.0, 0.0),
                    ("B", "prog", "gettid", 3.0, 0.0),
                    ("A", "prog", "read", 4.0, 0.0),
                    ("B", "prog", "read", 5.0, 0.0),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        counts = out[out["kind"] == KIND_COUNT]
        assert list(counts["syscall"]) == ["read"]

    def test_other_files_are_untouched(self, settings):
        """A set departure on one file must not silence another file."""
        frames = _Frames(
            syscall_sets=_sets(
                [
                    ("A", "p1", "openat", True),
                    ("B", "p1", "openat", True),
                    ("B", "p1", "gettid", True),
                    ("A", "p2", "gettid", True),
                    ("B", "p2", "gettid", True),
                ]
            ),
            counts=_counts(
                [
                    ("A", "p1", "gettid", 0.0, 0.0),
                    ("B", "p1", "gettid", 3.0, 0.0),
                    ("A", "p2", "gettid", 1.0, 0.0),
                    ("B", "p2", "gettid", 4.0, 0.0),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        counts = out[out["kind"] == KIND_COUNT]
        assert list(counts["file"]) == ["p2"]

    def test_arguments_of_a_new_syscall_are_subsumed(self, settings):
        """B opens a path A never opens *because* B never calls openat at all."""
        frames = _Frames(
            syscall_sets=_sets(
                [
                    ("A", "prog", "read", True),
                    ("B", "prog", "read", True),
                    ("B", "prog", "connect", True),
                ]
            ),
            args=_args(
                [
                    ("A", "prog", "read", "/etc/hosts", True),
                    ("B", "prog", "read", "/etc/hosts", True),
                    ("B", "prog", "connect", "/var/run/nscd", True),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_SET]

    def test_arguments_of_a_shared_syscall_still_fire(self, settings):
        """The real argument finding -- same call, different value -- survives."""
        frames = _Frames(
            syscall_sets=_sets(
                [
                    ("A", "prog", "openat", True),
                    ("B", "prog", "openat", True),
                    ("B", "prog", "gettid", True),
                ]
            ),
            args=_args(
                [
                    ("A", "prog", "openat", "/etc/passwd", True),
                    ("B", "prog", "openat", "/lib/evil.so", True),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        args = out[out["kind"] == KIND_ARG]
        assert len(args) == 1
        assert args.iloc[0]["variant_value"] == "/lib/evil.so"

    def test_missing_syscall_direction_also_subsumes(self, settings):
        """A makes it, B does not: the -N count row is the same event again."""
        frames = _Frames(
            syscall_sets=_sets(
                [
                    ("A", "prog", "openat", True),
                    ("B", "prog", "openat", True),
                    ("A", "prog", "gettid", True),
                ]
            ),
            counts=_counts(
                [
                    ("A", "prog", "gettid", 3.0, 0.0),
                    ("B", "prog", "gettid", 0.0, 0.0),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_SET]
        assert out.iloc[0]["detail"] == "missing syscall type"

    def test_nothing_suppressed_when_no_set_departure(self, settings):
        """The ordinary path must be untouched."""
        frames = _Frames(
            syscall_sets=_sets(
                [("A", "prog", "read", True), ("B", "prog", "read", True)]
            ),
            counts=_counts(
                [("A", "prog", "read", 4.0, 0.0), ("B", "prog", "read", 5.0, 0.0)]
            ),
            ngrams=_ngrams(
                [
                    ("A", "prog", "openat→read→close", True, 1),
                    ("B", "prog", "read→openat→close", True, 1),
                ]
            ),
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert set(out["kind"]) == {KIND_COUNT, KIND_SEQ}


class TestSequenceSubsumptionIsPerNgram:
    """The sequence detector emits one row per file, so subsumption there has to
    filter n-grams rather than rows -- otherwise a genuine reordering would be
    lost the moment the file also had a new syscall type."""

    def test_ngrams_containing_the_new_syscall_are_dropped(self):
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "a→b→c", True, 1),
                ("B", "prog", "gettid→b→c", True, 1),
            ]
        )
        out = detect_sequence(
            df, "basic", "A", "B", NONE, explained={"prog": frozenset({"gettid"})}
        )
        assert out.empty

    def test_unexplained_reordering_survives_alongside(self):
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "gettid→b→c", True, 1),  # explained
                ("B", "prog", "c→b→a", True, 1),  # genuine reordering
            ]
        )
        out = detect_sequence(
            df, "basic", "A", "B", NONE, explained={"prog": frozenset({"gettid"})}
        )
        assert len(out) == 1
        assert out.iloc[0]["magnitude"] == pytest.approx(1.0)
        assert "c→b→a" in out.iloc[0]["detail"]
        assert "gettid" not in out.iloc[0]["detail"]

    def test_new_syscall_anywhere_in_the_window_counts(self):
        """Not just the first position -- any member explains the window."""
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "a→gettid→c", True, 1),
                ("B", "prog", "a→b→gettid", True, 1),
            ]
        )
        out = detect_sequence(
            df, "basic", "A", "B", NONE, explained={"prog": frozenset({"gettid"})}
        )
        assert out.empty

    def test_substring_syscall_names_are_not_confused(self):
        """`read` must not match inside `pread64`; the split is on the separator."""
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "pread64→b→c", True, 1),
            ]
        )
        out = detect_sequence(
            df, "basic", "A", "B", NONE, explained={"prog": frozenset({"read"})}
        )
        assert len(out) == 1

    def test_explained_defaults_to_no_filtering(self):
        df = _ngrams(
            [
                ("A", "prog", "a→b→c", True, 1),
                ("B", "prog", "gettid→b→c", True, 1),
            ]
        )
        assert len(detect_sequence(df, "basic", "A", "B", NONE)) == 1


class TestReq5VariantNondeterminism:
    """The baseline is reproducible on this program; the variant is not.

    The case that motivated the detector: A emits ``bla`` exactly twice in every
    single trace, and B's count wanders. No other detector can state that. The
    count detector builds its floor out of the variant's own spread, so the
    wandering cancels the finding it is evidence for; the set detector sees
    ``bla`` on both sides and says nothing.
    """

    TAGS = ["A"]

    def _detect(self, rows, tags=None, ignore=NONE):
        sets, counts = _from_reps(rows)
        return detect_instability(
            sets, counts, "basic", "A", tags or self.TAGS, "B", ignore
        )

    def test_variant_count_wanders_against_a_constant_baseline(self):
        """A: 2,2,2,2,2. B: 1,3,4,2,5. Flagged -- and by nothing else."""
        rows = [
            ("A", "prog", "bla", [2, 2, 2, 2, 2]),
            ("B", "prog", "bla", [1, 3, 4, 2, 5]),
        ]
        out = self._detect(rows)
        assert len(out) == 1
        r = out.iloc[0]
        assert r["kind"] == KIND_INSTAB
        assert r["syscall"] == "bla"
        assert r["baseline_value"] == "constant 2"
        assert r["magnitude"] == pytest.approx(4.0)  # 5 - 1
        assert "5 distinct counts" in r["variant_value"]

        # The premise: the count detector really is blind here. Means are
        # 2.0 vs 3.0, and the variant's own sd of ~1.41 is the floor.
        _, counts = _from_reps(rows)
        assert detect_count(counts, "basic", "A", "B", NONE, NONE, 0.5).empty

    def test_variant_drops_the_syscall_in_some_reps(self):
        """A: 2,2,2,2,2. B: 1,3,4,2,-. The absent rep counts as zero."""
        out = self._detect(
            [
                ("A", "prog", "bla", [2, 2, 2, 2, 2]),
                ("B", "prog", "bla", [1, 3, 4, 2, None]),
            ]
        )
        assert len(out) == 1
        r = out.iloc[0]
        assert r["magnitude"] == pytest.approx(4.0)  # 4 - 0, not 4 - 1
        assert "absent in 1" in r["variant_value"]

    def test_that_case_is_no_longer_reported_as_a_missing_syscall(self):
        """The regression this detector exists to fix.

        `detect_set` used to compare two *stable* sets, so a variant that
        emitted `bla` in four reps of five had it missing from its stable set
        and was reported as having lost the syscall -- while in fact making it
        more often than the baseline on average.
        """
        sets, _ = _from_reps(
            [
                ("A", "prog", "bla", [2, 2, 2, 2, 2]),
                ("B", "prog", "bla", [1, 3, 4, 2, None]),
            ]
        )
        assert detect_set(sets, "basic", "A", "B", NONE).empty

    def test_a_genuinely_missing_syscall_is_still_a_set_departure(self):
        """The other half: never emitted once really is an absence."""
        sets, _ = _from_reps(
            [
                ("A", "prog", "bla", [2, 2, 2]),
                ("A", "prog", "read", [1, 1, 1]),
                ("B", "prog", "read", [1, 1, 1]),
            ]
        )
        out = detect_set(sets, "basic", "A", "B", NONE)
        assert list(out["syscall"]) == ["bla"]
        assert out.iloc[0]["detail"] == "missing syscall type"

    def test_flaky_baseline_is_never_blamed_on_the_variant(self):
        """The mirror case is deliberately out of scope, in both shapes."""
        both = self._detect(
            [
                ("A", "prog", "bla", [1, 3, 4, 2, None]),
                ("B", "prog", "bla", [1, 3, 4, 2, None]),
            ]
        )
        assert both.empty

        steady_variant = self._detect(
            [
                ("A", "prog", "bla", [1, 3, 4, 2, 5]),
                ("B", "prog", "bla", [2, 2, 2, 2, 2]),
            ]
        )
        assert steady_variant.empty

    def test_reproducible_variant_is_silent_even_when_it_differs(self):
        """A count difference is the count detector's business, not this one."""
        out = self._detect(
            [
                ("A", "prog", "bla", [2, 2, 2]),
                ("B", "prog", "bla", [9, 9, 9]),
            ]
        )
        assert out.empty

    def test_new_and_intermittent(self):
        """A never emits it; B emits it sometimes. Silent under every other
        detector: too unstable for the set detector, too small for count."""
        out = self._detect(
            [
                ("A", "prog", "read", [1, 1, 1]),
                ("B", "prog", "read", [1, 1, 1]),
                ("B", "prog", "connect", [1, None, 1]),
            ]
        )
        assert list(out["syscall"]) == ["connect"]
        assert out.iloc[0]["baseline_value"] == "never emitted"

    def test_baseline_disagreeing_between_builds_is_not_determinism(self):
        """Constant within each control group, different between them.

        Nothing about the variant can be attributed here: the baseline's own
        rebuilds do not agree, so "the measurement is reproducible on this
        program" is false however steady any single group looks.
        """
        out = self._detect(
            [
                ("A", "prog", "bla", [2, 2, 2]),
                ("A@1.0", "prog", "bla", [3, 3, 3]),
                ("B", "prog", "bla", [1, 4, 2]),
            ],
            tags=["A", "A@1.0"],
        )
        assert out.empty

    def test_baseline_missing_the_call_in_one_group_is_not_determinism(self):
        out = self._detect(
            [
                ("A", "prog", "bla", [2, 2, 2]),
                ("A@1.0", "prog", "read", [1, 1, 1]),
                ("B", "prog", "bla", [1, 4, 2]),
            ],
            tags=["A", "A@1.0"],
        )
        assert out.empty

    def test_ignored_syscalls_are_skipped(self):
        out = self._detect(
            [
                ("A", "prog", "futex", [2, 2, 2]),
                ("B", "prog", "futex", [1, 4, 2]),
            ],
            ignore=frozenset({"futex"}),
        )
        assert out.empty

    def test_files_the_baseline_never_traced_are_skipped(self):
        out = self._detect(
            [
                ("A", "other", "bla", [2, 2, 2]),
                ("B", "prog", "bla", [1, 4, 2]),
            ]
        )
        assert out.empty

    def test_a_pooled_frame_reports_nothing_rather_than_guessing(self):
        """`_counts` omits the rep-shape columns, as `pool_counts` does."""
        sets, _ = _from_reps([("A", "prog", "bla", [2, 2, 2])])
        pooled = _counts(
            [("A", "prog", "bla", 2.0, 0.0), ("B", "prog", "bla", 3.0, 1.4)]
        )
        assert detect_instability(
            sets, pooled, "basic", "A", self.TAGS, "B", NONE
        ).empty


class TestInstabilitySubsumption:
    def _frames(self, rows):
        sets, counts = _from_reps(rows)
        return _Frames(syscall_sets=sets, counts=counts)

    def test_a_large_count_shift_subsumes_the_wobble(self, settings):
        """B: 20,21,22 against a constant 2. The +19 is the finding; that it
        also wobbles by one is already visible as `variant_sd`."""
        frames = self._frames(
            [
                ("A", "prog", "bla", [2, 2, 2]),
                ("B", "prog", "bla", [20, 21, 22]),
            ]
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_COUNT]

    def test_the_wobble_survives_when_the_means_agree(self, settings):
        frames = self._frames(
            [
                ("A", "prog", "bla", [2, 2, 2, 2, 2]),
                ("B", "prog", "bla", [1, 3, 4, 2, 5]),
            ]
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_INSTAB]

    def test_a_new_syscall_type_subsumes_it(self, settings):
        """Stably new *and* wobbling: the set row is the sharper statement."""
        frames = self._frames(
            [
                ("A", "prog", "read", [1, 1, 1]),
                ("B", "prog", "read", [1, 1, 1]),
                ("B", "prog", "gettid", [3, 4, 5]),
            ]
        )
        out = all_departures(frames, settings, "basic", "B", NONE)
        assert list(out["kind"]) == [KIND_SET]
        assert out.iloc[0]["syscall"] == "gettid"
