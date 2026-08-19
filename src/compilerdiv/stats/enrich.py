"""Attach the evidence a departure row cannot carry on its own.

The detectors answer "did this differ?" and stop there. Three questions follow
immediately and none of them are answerable from a departure row as emitted:

*What values were passed?*
    A ``write`` that vanishes and a ``write`` that says something else are very
    different findings. The raw store keeps the normalised argument values;
    :func:`enrich_departures` joins them on, and records **why** a cell is empty
    (``arg_status``) so "no argument" is never confused with "not measured".

*How solid are the numbers?*
    A count departure reports means over ``trace.reps`` reps. A mean of ``1.60``
    is not "1.6 syscalls" -- it is ``4,1,1,1,1``, an unstable program. Without
    the standard deviation beside it the fraction looks like a bug.

*Did the baseline do this to itself, here?*
    ``floor.rebuild_delta`` already knows, per (file, syscall), how far two
    builds of the same source drifted apart. Attaching it lets a reader judge a
    single row without cross-referencing a corpus-wide noise table that was
    never about this file.

None of this changes a verdict. It is all context for one, which is why it lives
here rather than in the detectors.
"""

from __future__ import annotations

import pandas as pd

from ..config import Settings
from .detectors import KIND_ARG
from .noise import NoiseFloor

#: Separator between multiple argument values in one cell.
ARG_SEP = " | "

#: ``arg_status`` values.
ARG_CAPTURED = "captured"
ARG_NONE_SEEN = "no string argument observed"
ARG_NOT_TRACED = "not in trace.arg_syscalls"

ENRICHED_COLUMNS = [
    "variant_label",
    "baseline_mean",
    "variant_mean",
    "baseline_sd",
    "variant_sd",
    "baseline_arg",
    "variant_arg",
    "arg_status",
    "baseline_selfdiff",
]


def _count_lookup(
    counts: pd.DataFrame, config: str, compiler: str
) -> dict[tuple[str, str], tuple[float, float]]:
    """``(file, syscall) -> (mean, sd)`` for one compiler."""
    if counts.empty:
        return {}
    sub = counts[(counts["config"] == config) & (counts["compiler"] == compiler)]
    return {
        (str(f), str(s)): (float(m), float(d))
        for f, s, m, d in zip(
            sub["file"], sub["syscall"], sub["mean_count"], sub["std_count"]
        )
    }


def _arg_lookup(
    args: pd.DataFrame, config: str, compiler: str
) -> dict[tuple[str, str], list[str]]:
    """``(file, syscall) -> sorted stable argument values`` for one compiler."""
    if args.empty:
        return {}
    sub = args[
        (args["config"] == config) & (args["compiler"] == compiler) & args["stable"]
    ]
    out: dict[tuple[str, str], list[str]] = {}
    for (f, sc), g in sub.groupby(["file", "syscall"]):
        out[(str(f), str(sc))] = sorted(str(a) for a in g["arg"])
    return out


def enrich_departures(
    departures: pd.DataFrame,
    frames,
    floor: NoiseFloor,
    settings: Settings,
    config: str,
    variant: str,
    *,
    max_args: int = 8,
) -> pd.DataFrame:
    """Add argument, dispersion and self-difference evidence to departure rows.

    Returns a copy with :data:`ENRICHED_COLUMNS` appended. Every added column is
    defined for every row -- a missing measurement is an explicit marker, not a
    silent blank, because a reader cannot tell those apart in a spreadsheet.

    ``baseline_mean`` / ``variant_mean`` are filled for **every** kind, not only
    ``count``. A set departure that says only "``gettid`` is new" throws away how
    much of it there is; ``0.00 -> 2.00`` is strictly more informative and costs
    a dict lookup. Sequence rows are the exception: their ``syscall`` field is a
    pipe-joined set of n-gram heads, which is not a syscall and joins to nothing.
    """
    if departures.empty:
        out = departures.copy()
        for col in ENRICHED_COLUMNS:
            out[col] = pd.Series(dtype="object")
        return out

    base = settings.baseline
    label = settings.label(variant)
    traced = frozenset(settings.trace.arg_syscalls)

    a_counts = _count_lookup(frames.counts, config, base)
    b_counts = _count_lookup(frames.counts, config, variant)
    a_args = _arg_lookup(frames.args, config, base)
    b_args = _arg_lookup(frames.args, config, variant)

    def _join(vals: list[str]) -> str:
        shown = ARG_SEP.join(vals[:max_args])
        return shown + (
            f"{ARG_SEP}(+{len(vals) - max_args} more)" if len(vals) > max_args else ""
        )

    rows = []
    for r in departures.itertuples(index=False):
        f, sc, kind = str(r.file), str(r.syscall), str(r.kind)
        key = (f, sc)

        a_mean, a_sd = a_counts.get(key, (float("nan"), float("nan")))
        b_mean, b_sd = b_counts.get(key, (float("nan"), float("nan")))

        av, bv = a_args.get(key, []), b_args.get(key, [])
        if kind == KIND_ARG:
            # The row is about one specific value; the detector already put the
            # baseline's set and that value in place. Re-deriving them here
            # would widen `variant_arg` to every value the variant passes and
            # lose which one is the finding.
            baseline_arg = str(r.baseline_value)
            variant_arg = str(r.variant_value)
            status = ARG_CAPTURED
        else:
            baseline_arg, variant_arg = _join(av), _join(bv)
            if sc not in traced:
                status = ARG_NOT_TRACED
            elif not av and not bv:
                status = ARG_NONE_SEEN
            else:
                status = ARG_CAPTURED

        rows.append(
            {
                "variant_label": label,
                "baseline_mean": a_mean,
                "variant_mean": b_mean,
                "baseline_sd": a_sd,
                "variant_sd": b_sd,
                "baseline_arg": baseline_arg,
                "variant_arg": variant_arg,
                "arg_status": status,
                "baseline_selfdiff": floor.build_floor(f, sc),
            }
        )

    extra = pd.DataFrame(rows, index=departures.index)
    return pd.concat([departures, extra], axis=1)
