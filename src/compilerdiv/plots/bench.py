"""Benchmark plots.

Preserved from the original pipeline, which got these right: one group per
config, one bar per compiler, geometric means; a fold-change panel against the
baseline; and a log-y per-file distribution. Only the plumbing changed (tidy
frames in, ``PlotContext`` out).

Also hosts the SLOC density plot that used to live in a standalone script -- it
is Figure 5 of the paper and belongs with everything else that produces figures.
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

from ..config import Settings
from ..stats.bench import METRICS

TITLES = {"size": "Binary size", "compile": "Compile time", "exec": "Execution time"}


def _save(fig, out_dir: Path, name: str, dpi: int = 130) -> Path:
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / f"{name}.png"
    fig.tight_layout()
    fig.savefig(path, dpi=dpi)
    plt.close(fig)
    return path


def plot_grouped(
    geo: pd.DataFrame,
    settings: Settings,
    metric: str,
    unit: str,
    scale: float,
    out_dir: Path,
) -> Path | None:
    """One group per config, one bar per compiler, geometric means."""
    if geo.empty:
        return None
    configs = settings.config_names
    x = np.arange(len(configs))
    keys = settings.compiler_keys
    width = 0.8 / max(len(keys), 1)

    fig, ax = plt.subplots(figsize=(max(8, 2.2 * len(configs) + 4), 6))
    seen = set()
    for ci, config in enumerate(configs):
        applicable = list(settings.compilers_for(config))
        k_n = len(applicable)
        for j, key in enumerate(applicable):
            row = geo[(geo["config"] == config) & (geo["compiler"] == key)]
            if row.empty:
                continue
            val = float(row[metric].iloc[0]) * scale
            offset = (j - (k_n - 1) / 2) * width
            bars = ax.bar(
                x[ci] + offset,
                val,
                width,
                label=settings.label(key) if key not in seen else None,
                color=settings.color(key),
            )
            seen.add(key)
            ax.bar_label(bars, fmt="%.1f", padding=2, fontsize=7, rotation=90)

    ax.set_ylabel(f"geometric-mean per-file {unit}")
    ax.set_title(f"{TITLES[metric]} (per-file geomean over programs)")
    ax.set_xticks(x)
    ax.set_xticklabels(configs)
    ax.set_xlabel("flag configuration")
    ax.legend(title="compiler")
    ax.margins(y=0.18)
    return _save(fig, out_dir, metric)


def plot_foldchange(
    fc: pd.DataFrame, settings: Settings, metric: str, unit: str, out_dir: Path
) -> Path | None:
    """Geomean fold-change vs the baseline."""
    if fc.empty:
        return None
    configs = settings.config_names
    x = np.arange(len(configs))
    others = settings.variant_keys
    width = 0.8 / max(len(others), 1)

    fig, ax = plt.subplots(figsize=(max(8, 2.2 * len(configs) + 4), 6))
    seen = set()
    for ci, config in enumerate(configs):
        present = [
            k
            for k in others
            if not fc[(fc["config"] == config) & (fc["compiler"] == k)].empty
        ]
        k_n = len(present)
        for j, key in enumerate(present):
            row = fc[(fc["config"] == config) & (fc["compiler"] == key)]
            val = float(row[f"{metric}_geomean"].iloc[0])
            offset = (j - (k_n - 1) / 2) * width
            bars = ax.bar(
                x[ci] + offset,
                val,
                width,
                label=(
                    f"{settings.label(key)} / {settings.label(settings.baseline)}"
                    if key not in seen
                    else None
                ),
                color=settings.color(key),
            )
            seen.add(key)
            ax.bar_label(bars, fmt="%.2fx", padding=2, fontsize=7, rotation=90)

    ax.axhline(1.0, ls="--", lw=0.9, color="k")
    ax.set_ylabel(f"fold-change ({unit.split()[0]}) vs baseline")
    ax.set_title(f"{TITLES[metric]}: variant / {settings.label(settings.baseline)}")
    ax.set_xticks(x)
    ax.set_xticklabels(configs)
    ax.set_xlabel("flag configuration")
    ax.legend(title=f"vs {settings.label(settings.baseline)} (= 1.0)")
    ax.margins(y=0.18)
    return _save(fig, out_dir, f"{metric}_foldchange")


def plot_distribution(
    bench: pd.DataFrame,
    settings: Settings,
    metric: str,
    col: str,
    unit: str,
    scale: float,
    out_dir: Path,
) -> Path | None:
    """Per-file distribution, log-y, one box per (config, compiler)."""
    if bench.empty:
        return None
    fig, ax = plt.subplots(figsize=(max(9, len(settings.config_names) * 2.6), 6))
    positions, data, colors, ticks, ticklabels = [], [], [], [], []
    slot = 0
    for config in settings.config_names:
        start = slot
        for key in settings.compilers_for(config):
            vals = bench[(bench["config"] == config) & (bench["compiler"] == key)][
                col
            ].to_numpy()
            if len(vals) == 0:
                continue
            positions.append(slot)
            data.append(np.clip(vals.astype(float), 1e-9, None) * scale)
            colors.append(settings.color(key))
            slot += 1
        if slot > start:
            ticks.append((start + slot - 1) / 2)
            ticklabels.append(config)
        slot += 1

    if not data:
        plt.close(fig)
        return None

    bp = ax.boxplot(
        data,
        positions=positions,
        widths=0.8,
        showfliers=False,
        patch_artist=True,
        medianprops={"color": "k"},
    )
    for patch, color in zip(bp["boxes"], colors):
        patch.set_facecolor(color)
    ax.set_yscale("log")
    ax.set_ylabel(f"per-file {unit} (log)")
    ax.set_title(f"{TITLES[metric]}: per-file distribution")
    ax.set_xticks(ticks)
    ax.set_xticklabels(ticklabels)
    ax.set_xlabel("flag configuration")
    handles = [
        plt.Rectangle((0, 0), 1, 1, color=settings.color(k))
        for k in settings.compiler_keys
    ]
    ax.legend(
        handles, [settings.label(k) for k in settings.compiler_keys], title="compiler"
    )
    return _save(fig, out_dir, f"{metric}_distribution")


def plot_all_bench(
    bench: pd.DataFrame,
    geo: pd.DataFrame,
    fc: pd.DataFrame,
    settings: Settings,
    out_dir: Path,
) -> list[Path]:
    written = []
    for metric, col, unit, scale in METRICS:
        for p in (
            plot_grouped(geo, settings, metric, unit, scale, out_dir),
            plot_foldchange(fc, settings, metric, unit, out_dir),
            plot_distribution(bench, settings, metric, col, unit, scale, out_dir),
        ):
            if p is not None:
                written.append(p)
    return written


def plot_sloc_density(programs, out_dir: Path) -> Path | None:
    """Figure 5: SLOC distribution across the corpus, with a KDE overlay."""
    counts = [p.sloc for p in programs]
    if not counts:
        return None

    min_i = int(np.argmin(counts))
    max_i = int(np.argmax(counts))

    fig, ax = plt.subplots(figsize=(10, 6))
    ax.hist(
        counts, bins=40, density=True, alpha=0.5, color="steelblue", edgecolor="white"
    )

    try:
        from scipy.stats import gaussian_kde

        if len(set(counts)) > 1:
            kde = gaussian_kde(counts)
            xs = np.linspace(min(counts), max(counts), 500)
            ax.plot(xs, kde(xs), color="darkorange", lw=2, label="KDE")
            ax.legend()
    except Exception:
        pass

    box = (
        f"Min: {counts[min_i]} lines\n     {programs[min_i].stem}\n\n"
        f"Max: {counts[max_i]} lines\n     {programs[max_i].stem}"
    )
    ax.text(
        0.98,
        0.05,
        box,
        transform=ax.transAxes,
        ha="right",
        va="bottom",
        fontsize=9,
        family="monospace",
        bbox={
            "boxstyle": "round,pad=0.5",
            "facecolor": "white",
            "edgecolor": "gray",
            "alpha": 0.9,
        },
    )
    ax.set_title(
        "Density of lines of code per program\n(excluding blank lines and comments)"
    )
    ax.set_xlabel("lines of code (SLOC)")
    ax.set_ylabel("density")
    ax.grid(True, alpha=0.3)
    return _save(fig, out_dir, "sloc_density")
