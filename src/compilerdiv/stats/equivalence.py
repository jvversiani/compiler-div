"""Per-file equivalence verdicts, significance testing, descriptive divergence.

The headline number the paper needs is per-file: for each program, is the
variant's behavior distinguishable from the baseline's, and by which detector?
That verdict is derived from the departures, not from a divergence threshold.

Jensen-Shannon divergence over count histograms is computed here too, but it is
**descriptive only** and never gates a verdict. It normalises by total syscall
volume, so a fixed +3 delta scores high on a 60-syscall program and ~0 on a
5000-syscall one: the statistic ends up ranking by program size rather than by
compiler difference. It is retained because it appeared in earlier results and
readers may expect it; every plot and column carries the caveat.
"""

from __future__ import annotations

import numpy as np
import pandas as pd
from scipy import stats as sstats

from ..config import Settings
from .detectors import KIND_ARG, KIND_COUNT, KIND_INSTAB, KIND_SEQ, KIND_SET
from .taxonomy import CLASS_ORDER

EPS = 1.0  # additive smoothing for log-ratios

# THIS IS A DISCLAIMER FOR JS DIV ANALYSIS
JS_CAVEAT = (
    "JS divergence is descriptive only: it normalises by total syscall volume, "
    "so a fixed-size difference scores high on small programs and ~0 on large "
    "ones. Verdicts come from the departure detectors, not from this metric."
)


def js_divergence(pa: np.ndarray, pb: np.ndarray) -> float:
    """Jensen-Shannon divergence between two count vectors, in bits."""
    a, b = pa.sum(), pb.sum()
    if a == 0 or b == 0:
        return float("nan")
    p, q = pa / a, pb / b
    m = 0.5 * (p + q)

    def _kl(x, y):
        mask = x > 0
        return float(np.sum(x[mask] * np.log2(x[mask] / y[mask])))

    return float(0.5 * _kl(p, m) + 0.5 * _kl(q, m))


def per_file_js(
    counts: pd.DataFrame, config: str, baseline: str, variant: str
) -> pd.DataFrame:
    """Descriptive JS divergence per file. Columns: file, js_divergence, ..."""
    sub = counts[counts["config"] == config]
    a = sub[sub["compiler"] == baseline]
    b = sub[sub["compiler"] == variant]
    if a.empty or b.empty:
        return pd.DataFrame(
            columns=["file", "js_divergence", "total_baseline", "total_variant"]
        )

    am = a.set_index(["file", "syscall"])["mean_count"]
    bm = b.set_index(["file", "syscall"])["mean_count"]
    joined = pd.concat([am.rename("a"), bm.rename("b")], axis=1).fillna(0.0)

    files_a = set(am.index.get_level_values("file"))
    files_b = set(bm.index.get_level_values("file"))
    rows = []
    for f in sorted(files_a & files_b):
        g = joined.xs(f, level="file")
        va = g["a"].to_numpy(dtype=float)
        vb = g["b"].to_numpy(dtype=float)
        rows.append(
            {
                "file": str(f),
                "js_divergence": js_divergence(va, vb),
                "total_baseline": float(va.sum()),
                "total_variant": float(vb.sum()),
            }
        )
    return pd.DataFrame(rows)


#: Class name -> severity rank, worst first. Files inherit the rank of their
#: worst departure so the sheet sorts by *what matters*, not by how much of it.
_CLASS_RANK = {c: i for i, c in enumerate(CLASS_ORDER)}


def _worst_class_lookup(
    taxonomy: pd.DataFrame | None, config: str, variant: str
) -> dict[tuple[str, str], str]:
    """``(syscall, kind) -> class`` for one (config, variant)."""
    if taxonomy is None or taxonomy.empty:
        return {}
    sub = taxonomy[(taxonomy["config"] == config) & (taxonomy["variant"] == variant)]
    return {
        (str(s), str(k)): str(c)
        for s, k, c in zip(sub["syscall"], sub["kind"], sub["class"])
    }


def _worst_class(
    signatures: set[tuple[str, str]], lookup: dict[tuple[str, str], str]
) -> str:
    """Worst class among a file's departure signatures, or "" if it has none."""
    classes = [lookup[s] for s in signatures if s in lookup]
    if not classes:
        return ""
    return min(classes, key=lambda c: _CLASS_RANK.get(c, len(_CLASS_RANK)))


def per_file_verdict(
    departures: pd.DataFrame,
    frames,
    settings: Settings,
    config: str,
    variant: str,
    unstable_seq_files: frozenset[str],
    taxonomy: pd.DataFrame | None = None,
) -> pd.DataFrame:
    """One row per file: which detectors fired, and the overall verdict.

    ``equivalent`` means no detector fired. That is a stronger and more
    defensible statement than the old ``exact_match``, which compared mean
    counts against a per-file noise tolerance and therefore silently absolved
    any file whose noise happened to exceed the delta.

    Passing ``taxonomy`` adds ``worst_class``: the worst prior among the file's
    own departures. Without it the sheet ranks by departure *count*, which puts
    a file with nine startup artifacts above one with a single
    ``program_dependent`` finding -- exactly backwards.
    """
    base = settings.baseline
    bench = frames.bench
    if not bench.empty:
        files = sorted(
            {
                str(f)
                for f in bench[
                    (bench["config"] == config) & (bench["compiler"] == variant)
                ]["file"].unique()
            }
        )
        base_files = {
            str(f)
            for f in bench[(bench["config"] == config) & (bench["compiler"] == base)][
                "file"
            ].unique()
        }
        files = [f for f in files if f in base_files]
    else:
        files = sorted({str(f) for f in departures["file"].unique()})

    sub = (
        departures[
            (departures["config"] == config) & (departures["variant"] == variant)
        ]
        if not departures.empty
        else departures
    )

    by_file: dict[str, dict] = {f: {} for f in files}
    syscalls_by_file: dict[str, list[str]] = {}
    sig_by_file: dict[str, set[tuple[str, str]]] = {}
    if not sub.empty:
        for f, g in sub.groupby("file"):
            f = str(f)
            if f not in by_file:
                by_file[f] = {}
            for kind, gg in g.groupby("kind"):
                by_file[f][kind] = sorted(set(gg["detail"]))
            syscalls_by_file[f] = sorted({str(s) for s in g["syscall"]})
            sig_by_file[f] = {(str(s), str(k)) for s, k in zip(g["syscall"], g["kind"])}

    worst = _worst_class_lookup(taxonomy, config, variant)
    js = per_file_js(frames.counts, config, base, variant).set_index("file")

    rows = []
    for f in files:
        d = by_file.get(f, {})
        n_set = len(d.get(KIND_SET, []))
        n_count = len(d.get(KIND_COUNT, []))
        n_arg = len(d.get(KIND_ARG, []))
        n_seq = len(d.get(KIND_SEQ, []))
        n_instab = len(d.get(KIND_INSTAB, []))
        total = n_set + n_count + n_arg + n_seq + n_instab
        rows.append(
            {
                "config": config,
                "variant": variant,
                "variant_label": settings.label(variant),
                "file": f,
                "equivalent": total == 0,
                "worst_class": _worst_class(sig_by_file.get(f, set()), worst),
                "n_departures": total,
                "n_set": n_set,
                "n_count": n_count,
                "n_arg": n_arg,
                "n_seq": n_seq,
                "n_instab": n_instab,
                "top_syscalls": ", ".join(syscalls_by_file.get(f, [])[:5])
                + ("..." if len(syscalls_by_file.get(f, [])) > 5 else ""),
                "seq_oracle_reliable": f not in unstable_seq_files,
                "js_divergence": (
                    float(js.loc[f, "js_divergence"]) if f in js.index else float("nan")
                ),
            }
        )
    out = pd.DataFrame(rows)
    if out.empty:
        return out
    out["_ord"] = out["worst_class"].map(_CLASS_RANK).fillna(len(_CLASS_RANK))
    return (
        out.sort_values(
            ["_ord", "n_departures", "js_divergence"], ascending=[True, False, False]
        )
        .drop(columns="_ord")
        .reset_index(drop=True)
    )


def benjamini_hochberg(pvals) -> np.ndarray:
    """BH step-up FDR correction. Returns adjusted p-values."""
    p = np.asarray(pvals, dtype=float)
    n = len(p)
    if n == 0:
        return p
    order = np.argsort(p)
    ranked = p[order] * n / (np.arange(n) + 1)
    ranked = np.minimum.accumulate(ranked[::-1])[::-1]
    out = np.empty(n)
    out[order] = np.clip(ranked, 0, 1)
    return out


def per_syscall_logratios(
    counts: pd.DataFrame, config: str, baseline: str, variant: str
) -> pd.DataFrame:
    """Per (file, syscall) log ratio of variant to baseline mean counts."""
    sub = counts[counts["config"] == config]
    a = sub[sub["compiler"] == baseline].set_index(["file", "syscall"])["mean_count"]
    b = sub[sub["compiler"] == variant].set_index(["file", "syscall"])["mean_count"]
    if a.empty or b.empty:
        return pd.DataFrame(
            columns=["file", "syscall", "count_baseline", "count_variant", "log_ratio"]
        )

    joined = pd.concat([a.rename("a"), b.rename("b")], axis=1).fillna(0.0).reset_index()
    files_a = set(a.index.get_level_values("file"))
    files_b = set(b.index.get_level_values("file"))
    joined = joined[joined["file"].isin(files_a & files_b)]
    joined["log_ratio"] = np.log((joined["b"] + EPS) / (joined["a"] + EPS))
    return joined.rename(columns={"a": "count_baseline", "b": "count_variant"})


def syscall_significance(
    logr: pd.DataFrame,
    config: str,
    baseline: str,
    variant: str,
    noisy: frozenset[str],
    departures: pd.DataFrame | None = None,
) -> pd.DataFrame:
    """Wilcoxon signed-rank per syscall over files, with BH correction.

    Syscalls in ``noisy`` (run noise or build noise, from the control passes)
    are tested and reported but never marked significant: a difference that the
    baseline shows against itself is not evidence about the variant.

    ``n_files_differ`` counts every file in which the syscall diverges by *any*
    detector, not only by count. Passing ``departures`` folds the set,
    argument, and sequence detectors into that count: a syscall whose only
    difference is an argument value (the tampered ``write``) has an identical
    count log-ratio of zero yet genuinely differs, and would otherwise read as
    ``n_files_differ = 0``. The Wilcoxon test itself stays count-based -- it has
    no notion of an argument value -- so ``p_value`` and ``significant_5pct``
    describe count divergence only; ``departure_kinds`` names what actually
    fired so a nonzero ``n_files_differ`` with ``p_value = 1`` is not a mystery.
    """
    if logr.empty:
        return pd.DataFrame()

    dep_files: dict[str, set[str]] = {}
    dep_kinds: dict[str, set[str]] = {}
    if departures is not None and not departures.empty:
        dsub = departures[
            (departures["config"] == config) & (departures["variant"] == variant)
        ]
        for sc, g in dsub.groupby("syscall"):
            dep_files[str(sc)] = {str(f) for f in g["file"]}
            dep_kinds[str(sc)] = {str(k) for k in g["kind"]}

    rows = []
    for sc, g in logr.groupby("syscall"):
        sc = str(sc)
        x = g["log_ratio"].to_numpy(dtype=float)
        n_nonzero = int(np.count_nonzero(x))
        median_lr = float(np.median(x))
        if n_nonzero < 1:
            stat, pval = 0.0, 1.0
        else:
            try:
                stat, pval = sstats.wilcoxon(
                    x, zero_method="wilcox", alternative="two-sided"
                )
            except ValueError:
                stat, pval = 0.0, 1.0

        files_arr = g["file"].to_numpy()
        g_files = {str(f) for f in files_arr}
        count_differ = {str(f) for f in files_arr[x != 0]}
        differ_files = count_differ | (dep_files.get(sc, set()) & g_files)
        rows.append(
            {
                "config": config,
                "pair": f"{variant}v{baseline}",
                "variant": variant,
                "syscall": sc,
                "n_files": int(g["file"].nunique()),
                "n_files_differ": len(differ_files),
                "median_log_ratio": median_lr,
                "median_fold_change": float(np.exp(median_lr)),
                "wilcoxon_stat": float(stat),
                "p_value": float(pval),
                "noisy": sc in noisy,
                "departure_kinds": "|".join(sorted(dep_kinds.get(sc, set()))),
            }
        )
    df = pd.DataFrame(rows)
    df["p_adj_bh"] = benjamini_hochberg(df["p_value"].to_numpy())
    df["significant_5pct"] = (df["p_adj_bh"] < 0.05) & (~df["noisy"])
    return df.sort_values(
        ["significant_5pct", "p_adj_bh", "syscall"], ascending=[False, True, True]
    ).reset_index(drop=True)
