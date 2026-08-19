"""The departure taxonomy.

Requirement 4: distinguish benign compiler differences from tamper-relevant
ones. Deciding benignity statically is not possible -- ``read(+1)`` and a
one-syscall payload look identical to any automatic rule. What *is* possible is
to partition the departures into classes with different priors, attach the
evidence for each, and hand it over for manual review.

The classes:

``run_noise``
    The syscall differs between trace passes of one build. Not a compiler
    difference at all.

``build_noise``
    Stable within a build but differs between builds. The baseline's own
    builds vary this way, so a variant differing here says nothing.

``uniform``
    The same delta, in the same syscall, in essentially every file
    (>= ``uniform_threshold`` of the corpus). Program-independent by
    construction -- a payload that fires identically everywhere including on a
    one-line empty program is not doing anything program-specific. A variant
    whose runtime always calls ``gettid`` at startup lands here.

``conditional``
    A fixed-magnitude delta present in a *subset* of files. The interesting
    class: a variant's ``read(+1)`` on part of the corpus is the archetype.
    Benign if membership tracks link layout; suspicious if it tracks what the
    program does. The layout probe below is what separates those, and it is
    evidence for a human, not a verdict.

``program_dependent``
    Delta magnitude correlates with program size or activity. The only class in
    which a payload doing real work can hide, and empirically empty for both
    variants -- which is precisely the property that makes the corpus useful as
    a detector for RQ3.

The layout probe correlates membership-in-the-affected-set against binary size,
segment count, and section count (Spearman, via point-biserial on the binary
membership vector). It tests the hypothesis directly: if the files carrying a
variant's extra ``read`` are the ones whose startup read crosses a chunk
boundary, membership tracks layout and not semantics.
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np
import pandas as pd
from scipy import stats as sstats

from ..config import Settings
from .detectors import KIND_COUNT, KIND_INSTAB
from .noise import NoiseFloor

CLASS_RUN_NOISE = "run_noise"
CLASS_BUILD_NOISE = "build_noise"
CLASS_UNIFORM = "uniform"
CLASS_CONDITIONAL = "conditional"
CLASS_PROGRAM_DEPENDENT = "program_dependent"

#: Ordered worst-first, for reports.
CLASS_ORDER = [
    CLASS_PROGRAM_DEPENDENT,
    CLASS_CONDITIONAL,
    CLASS_UNIFORM,
    CLASS_BUILD_NOISE,
    CLASS_RUN_NOISE,
]

CLASS_PRIOR = {
    CLASS_RUN_NOISE: "not a compiler difference",
    CLASS_BUILD_NOISE: "baseline's own builds vary this way",
    CLASS_UNIFORM: "program-independent; almost certainly toolchain startup",
    CLASS_CONDITIONAL: "fixed delta in a subset; check the layout probe",
    CLASS_PROGRAM_DEPENDENT: "scales with the program; REVIEW CLOSELY",
}


@dataclass
class LayoutProbe:
    """Correlation of departure membership against static layout facts.

    ``status`` records *why* a correlation is missing, which matters: on a real
    corpus a bare NaN is indistinguishable from "no correlation", and the two
    licence opposite conclusions. "no layout signal" is evidence the departure
    is not a linking artifact; "could not compute" is evidence of nothing.
    """

    rho_size: float = float("nan")
    p_size: float = float("nan")
    rho_segments: float = float("nan")
    p_segments: float = float("nan")
    rho_sections: float = float("nan")
    p_sections: float = float("nan")
    rho_sloc: float = float("nan")
    p_sloc: float = float("nan")
    status: str = "ok"

    def _layout_candidates(self) -> list[tuple[str, float]]:
        return [
            (name, v)
            for name, v in (
                ("size", self.rho_size),
                ("segments", self.rho_segments),
                ("sections", self.rho_sections),
            )
            if not np.isnan(v)
        ]

    def best_layout_rho(self) -> float:
        vals = [abs(v) for _, v in self._layout_candidates()]
        return max(vals) if vals else float("nan")

    def best_layout_source(self) -> str:
        """Which layout variable ``best_layout_rho`` came from.

        Reported alongside the value so the three per-variable columns need not
        be: knowing the correlation is with *segment count* rather than *binary
        size* changes the reading, but the two losers never do.
        """
        cands = self._layout_candidates()
        return max(cands, key=lambda kv: abs(kv[1]))[0] if cands else ""


def _safe_spearman(x: np.ndarray, y: np.ndarray) -> tuple[float, float]:
    if len(x) < 4 or np.all(x == x[0]) or np.all(y == y[0]):
        return float("nan"), float("nan")
    try:
        r = sstats.spearmanr(x, y)
        return float(r.statistic), float(r.pvalue)
    except Exception:
        return float("nan"), float("nan")


def _layout_probe(
    affected: set[str], all_files: list[str], elf: pd.DataFrame
) -> LayoutProbe:
    """Point-biserial-style Spearman of membership vs layout facts."""
    if elf.empty:
        return LayoutProbe(status="no ELF data (readelf unavailable?)")
    if not affected:
        return LayoutProbe(status="no affected files")
    if len(all_files) < 4:
        return LayoutProbe(status="corpus too small (<4 files)")

    e = elf.drop_duplicates("file").set_index("file")
    files = [f for f in all_files if f in e.index]
    if len(files) < 4:
        return LayoutProbe(status="fewer than 4 files have ELF facts")

    member = np.array([1.0 if f in affected else 0.0 for f in files])
    if member.sum() == 0:
        return LayoutProbe(status="no affected file has ELF facts")
    if member.sum() == len(member):
        return LayoutProbe(
            status="every file affected (uniform: layout cannot discriminate)"
        )

    out = LayoutProbe()
    computed = []
    constant = []
    for col, (rho_a, p_a) in {
        "size_b": ("rho_size", "p_size"),
        "n_segments": ("rho_segments", "p_segments"),
        "n_sections": ("rho_sections", "p_sections"),
        "sloc": ("rho_sloc", "p_sloc"),
    }.items():
        if col not in e.columns:
            continue
        vals = pd.to_numeric(e.loc[files, col], errors="coerce").to_numpy(dtype=float)
        mask = ~np.isnan(vals)
        if mask.sum() < 4:
            continue
        if np.all(vals[mask] == vals[mask][0]):
            constant.append(col)
            continue
        rho, p = _safe_spearman(member[mask], vals[mask])
        setattr(out, rho_a, rho)
        setattr(out, p_a, p)
        if not np.isnan(rho):
            computed.append(col)

    if not computed:
        detail = f" (constant across corpus: {', '.join(constant)})" if constant else ""
        out.status = f"no usable layout variable{detail}"
    elif constant:
        out.status = f"ok (skipped constant: {', '.join(constant)})"
    return out


def _classify_one(
    syscall: str,
    kind: str,
    group: pd.DataFrame,
    n_corpus: int,
    floor: NoiseFloor,
    elf: pd.DataFrame,
    all_files: list[str],
    settings: Settings,
) -> dict:
    """Classify one (syscall, kind) departure across the corpus."""
    affected = {str(f) for f in group["file"]}
    n_affected = len(affected)
    mags = group["magnitude"].to_numpy(dtype=float)

    # The noise classes are a CORPUS-WIDE verdict: "this syscall is unstable
    # somewhere". They only apply to detectors that had no better instrument.
    #
    # `count` has one. Its floor is per (file, syscall) and already folds in the
    # baseline's own build variance for that exact program, so a count departure
    # that reached this point has cleared the noise question on the only ground
    # that matters -- the file it was observed on. Re-applying the global verdict
    # here would overrule the finer measurement with the coarser one, and would
    # grey out `read` on 240 files because the baseline wobbled on 6.
    #
    # `instability` has a per-file instrument too, and a stricter one: it fires
    # only where the baseline was byte-for-byte reproducible on this very
    # program, across every rerun and every rebuild. A corpus-wide "this syscall
    # is flaky somewhere" verdict cannot overrule a direct measurement saying it
    # was not flaky here.
    #
    # The set and sequence detectors have no per-file quantity to threshold, so
    # for them the global class still stands.
    if kind in (KIND_COUNT, KIND_INSTAB):
        klass = None
    else:
        cls = floor.classify_syscall(syscall)
        if cls == "run_noise":
            klass = CLASS_RUN_NOISE
        elif cls == "build_noise":
            klass = CLASS_BUILD_NOISE
        else:
            klass = None

    probe = _layout_probe(affected, all_files, elf)

    # Is the magnitude fixed?
    uniq = np.unique(np.round(mags, 3))
    fixed_magnitude = len(uniq) == 1
    frac = n_affected / n_corpus if n_corpus else 0.0

    if klass is None:
        # Does the magnitude scale with program size? Correlate |delta| against
        # SLOC over the affected files only -- a payload doing per-item work
        # produces a delta that grows with the program.
        rho_scale = float("nan")
        if not elf.empty and n_affected >= 4 and not fixed_magnitude:
            e = elf.drop_duplicates("file").set_index("file")
            fs = [f for f in group["file"] if f in e.index]
            if len(fs) >= 4:
                sub = group[group["file"].isin(fs)]
                sloc = pd.to_numeric(
                    e.loc[[str(f) for f in sub["file"]], "sloc"], errors="coerce"
                ).to_numpy(dtype=float)
                mg = np.abs(sub["magnitude"].to_numpy(dtype=float))
                m = ~np.isnan(sloc)
                if m.sum() >= 4:
                    rho_scale, _ = _safe_spearman(mg[m], sloc[m])

        if (
            not np.isnan(rho_scale)
            and abs(rho_scale) >= settings.detect.program_dependent_rho
        ):
            klass = CLASS_PROGRAM_DEPENDENT
        elif fixed_magnitude and frac >= settings.detect.uniform_threshold:
            klass = CLASS_UNIFORM
        elif fixed_magnitude:
            klass = CLASS_CONDITIONAL
        else:
            # Varying magnitude but no size correlation: still conditional, but
            # a human should look -- it is not a clean fixed offset.
            klass = CLASS_CONDITIONAL
    else:
        rho_scale = float("nan")

    return {
        "syscall": syscall,
        "kind": kind,
        "class": klass,
        "prior": CLASS_PRIOR[klass],
        "n_files_affected": n_affected,
        "n_files_corpus": n_corpus,
        "frac_affected": round(frac, 4),
        "magnitude_fixed": bool(fixed_magnitude),
        "magnitude_values": ", ".join(f"{v:g}" for v in uniq[:6])
        + ("..." if len(uniq) > 6 else ""),
        "magnitude_median": float(np.median(mags)),
        "rho_vs_sloc": round(rho_scale, 4) if not np.isnan(rho_scale) else None,
        # Kept for the layout-probe figure, which scatters binary size and must
        # annotate with the size correlation specifically. Not carried into the
        # workbook -- `layout_best_rho` + `layout_best_source` say more there.
        "layout_rho_size": (
            round(probe.rho_size, 4) if not np.isnan(probe.rho_size) else None
        ),
        "layout_best_rho": (
            round(probe.best_layout_rho(), 4)
            if not np.isnan(probe.best_layout_rho())
            else None
        ),
        "layout_best_source": probe.best_layout_source(),
        "layout_status": probe.status,
        "affected_files": "; ".join(sorted(affected)),
    }


#: Verdict text when no pair of builds ever caught the baseline disagreeing
#: with itself on this syscall.
SELFDIFF_CLEAN = "baseline stable across rebuilds"


def _selfdiff_lookup(build_compare: pd.DataFrame | None, variant: str) -> dict:
    """``syscall -> (n_files, deltas, verdict)`` from the between-build comparison."""
    if build_compare is None or build_compare.empty:
        return {}
    sub = build_compare[build_compare["variant"] == variant]
    return {
        str(r.syscall): (int(r.rebuild_n_files), str(r.rebuild_deltas), str(r.verdict))
        for r in sub.itertuples(index=False)
    }


def classify(
    departures: pd.DataFrame,
    frames,
    floor: NoiseFloor,
    settings: Settings,
    config: str,
    variant: str,
    build_compare: pd.DataFrame | None = None,
) -> pd.DataFrame:
    """Classify every (syscall, kind) departure for one (config, variant).

    Returns one row per departure signature, with the evidence attached. This is
    the taxonomy sheet of the behavior workbook.

    ``build_compare`` is the output of :func:`..stats.noise.build_vs_variant`.
    Its ruling belongs here rather than in a sheet of its own: the question it
    answers -- *is the variant's finding the same phenomenon the baseline shows
    against itself?* -- is a property of a departure signature, and a reader
    asking it is already looking at this row.
    """
    cols = [
        "config",
        "variant",
        "variant_label",
        "syscall",
        "kind",
        "class",
        "prior",
        "n_files_affected",
        "n_files_corpus",
        "frac_affected",
        "magnitude_fixed",
        "magnitude_values",
        "magnitude_median",
        "rho_vs_sloc",
        "layout_rho_size",
        "layout_best_rho",
        "layout_best_source",
        "layout_status",
        "selfdiff_n_files",
        "selfdiff_deltas",
        "selfdiff_verdict",
        "affected_files",
    ]
    if departures.empty:
        return pd.DataFrame(columns=cols)

    sub = departures[
        (departures["config"] == config) & (departures["variant"] == variant)
    ]
    if sub.empty:
        return pd.DataFrame(columns=cols)

    elf_all = frames.elf
    elf = (
        elf_all[
            (elf_all["config"] == config) & (elf_all["compiler"] == settings.baseline)
        ]
        if not elf_all.empty
        else elf_all
    )

    bench = frames.bench
    corpus_files = (
        sorted({str(f) for f in bench[bench["config"] == config]["file"].unique()})
        if not bench.empty
        else sorted({str(f) for f in sub["file"].unique()})
    )
    n_corpus = len(corpus_files)

    selfdiff = _selfdiff_lookup(build_compare, variant)

    rows = []
    for (sc, kind), g in sub.groupby(["syscall", "kind"]):
        rec = _classify_one(
            str(sc), str(kind), g, n_corpus, floor, elf, corpus_files, settings
        )
        rec["config"] = config
        rec["variant"] = variant
        rec["variant_label"] = settings.label(variant)
        n_sd, deltas, verdict = selfdiff.get(str(sc), (0, "", SELFDIFF_CLEAN))
        rec["selfdiff_n_files"] = n_sd
        rec["selfdiff_deltas"] = deltas
        rec["selfdiff_verdict"] = verdict
        rows.append(rec)

    out = pd.DataFrame(rows)
    out["_ord"] = out["class"].map({c: i for i, c in enumerate(CLASS_ORDER)})
    out = out.sort_values(["_ord", "n_files_affected"], ascending=[True, False])
    return out.drop(columns="_ord")[cols].reset_index(drop=True)


def summarize_classes(taxonomy: pd.DataFrame) -> pd.DataFrame:
    """Per (config, pair, class) counts, for the terminal report."""
    if taxonomy.empty:
        return pd.DataFrame(
            columns=["config", "variant", "class", "n_signatures", "n_files_max"]
        )
    return (
        taxonomy.groupby(["config", "variant", "class"])
        .agg(n_signatures=("syscall", "count"), n_files_max=("n_files_affected", "max"))
        .reset_index()
    )
