"""Benchmark statistics: size, compile time, exec time.

Kept from the original pipeline, because the normalisation choice there was
right: metrics are aggregated **per file with the geometric mean**, not summed.
Summing would let one 600-line program dominate 600 hello-worlds; the geomean
gives every program equal weight, which matches the pairwise-vs-baseline framing
used for the syscall analysis.

Reported both as an absolute geomean and as a fold-change against the baseline
(geomean and median of the per-file ratio X/X_A).
"""

from __future__ import annotations

from dataclasses import dataclass

import numpy as np
import pandas as pd
from scipy.stats import gmean

from ..config import Settings

METRICS = [
    ("size", "size_b", "size (KiB)", 1 / 1024),
    ("compile", "compile_s", "compile (s)", 1.0),
    ("exec", "exec_s", "exec (s)", 1.0),
]


def _clip_pos(arr) -> np.ndarray:
    return np.clip(np.asarray(arr, dtype=float), 1e-9, None)


@dataclass
class BenchSummary:
    geomeans: pd.DataFrame
    foldchange: pd.DataFrame


def geomeans(bench: pd.DataFrame, settings: Settings) -> pd.DataFrame:
    """Per (config, compiler) geometric means over files."""
    rows = []
    for config in settings.config_names:
        for key in settings.compilers_for(config):
            sub = bench[(bench["config"] == config) & (bench["compiler"] == key)]
            if sub.empty:
                continue
            rec = {
                "config": config,
                "compiler": key,
                "label": settings.label(key),
                "n_files": int(sub["file"].nunique()),
            }
            for name, col, _, _ in METRICS:
                rec[name] = float(gmean(_clip_pos(sub[col].to_numpy())))
            rows.append(rec)
    return pd.DataFrame(
        rows,
        columns=["config", "compiler", "label", "n_files", "size", "compile", "exec"],
    )


def foldchange(bench: pd.DataFrame, settings: Settings) -> pd.DataFrame:
    """Per-file ratio X/X_A, summarised by geomean and median."""
    base = settings.baseline
    rows = []
    for config in settings.config_names:
        b = bench[(bench["config"] == config) & (bench["compiler"] == base)].set_index(
            "file"
        )
        if b.empty:
            continue
        for key in settings.compilers_for(config):
            if key == base:
                continue
            v = bench[
                (bench["config"] == config) & (bench["compiler"] == key)
            ].set_index("file")
            common = b.index.intersection(v.index)
            if len(common) == 0:
                continue
            rec = {
                "config": config,
                "compiler": key,
                "pair": f"{settings.label(key)}/{settings.label(base)}",
                "n_files": len(common),
            }
            for name, col, _, _ in METRICS:
                ratio = _clip_pos(v.loc[common, col].to_numpy()) / _clip_pos(
                    b.loc[common, col].to_numpy()
                )
                rec[f"{name}_geomean"] = float(gmean(ratio))
                rec[f"{name}_median"] = float(np.median(ratio))
            rows.append(rec)
    cols = ["config", "compiler", "pair", "n_files"]
    for name, _, _, _ in METRICS:
        cols += [f"{name}_geomean", f"{name}_median"]
    return pd.DataFrame(rows, columns=cols)


def summarize(bench: pd.DataFrame, settings: Settings) -> BenchSummary:
    if bench.empty:
        return BenchSummary(pd.DataFrame(), pd.DataFrame())
    return BenchSummary(geomeans(bench, settings), foldchange(bench, settings))
