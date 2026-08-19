"""Plot theme and save helpers.

``PlotContext`` replaces the old ``_PLOT_BANNER`` module global that
``analyze_strace_config`` mutated: the banner is now passed explicitly, so two
configs can never race or leak state into each other's figures.
"""

from __future__ import annotations

import warnings
from dataclasses import dataclass, field
from pathlib import Path

import matplotlib

# The backend must be selected before pyplot is imported, so these imports are
# intentionally not at the top of the module.
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402  pylint: disable=wrong-import-position
import numpy as np  # noqa: E402  pylint: disable=wrong-import-position

CLASS_COLORS = {
    "program_dependent": "#C44E52",
    "conditional": "#DD8452",
    "uniform": "#CCB974",
    # Only reachable for set/sequence departures, which have no per-file floor
    # to threshold against. Count departures are never classed as noise: their
    # floor is per (file, syscall) and has already answered the question.
    "build_noise": "#8C8C8C",
    "run_noise": "#BBBBBB",
}

KIND_COLORS = {
    "set": "#C44E52",
    "argument": "#DD8452",
    "sequence": "#8172B3",
    "instability": "#937860",
    "count": "#4C72B0",
}


@dataclass
class PlotContext:
    """Everything a plot needs that is not data."""

    out_dir: Path
    config: str
    banner: str = ""
    dpi: int = 130
    written: list[Path] = field(default_factory=list)

    def save(self, fig, name: str, *, subdir: bool = True) -> Path:
        """Save a figure as ``<out>/<name>/<name>_<config>.png``."""
        d = self.out_dir / name if subdir else self.out_dir
        d.mkdir(parents=True, exist_ok=True)
        path = d / f"{name}_{self.config}.png"
        # Figures with a figure-level legend (positioned manually via
        # bbox_to_anchor) are not laid out by tight_layout, which then warns
        # "Axes not compatible with tight_layout". The placement is intentional
        # and the output is correct, so silence just that benign warning rather
        # than let it spam every run.
        with warnings.catch_warnings():
            warnings.filterwarnings(
                "ignore",
                message=".*not compatible with tight_layout.*",
                category=UserWarning,
            )
            if self.banner:
                fig.tight_layout(rect=(0, 0.04, 1, 1))
                fig.text(
                    0.5,
                    0.008,
                    self.banner,
                    ha="center",
                    va="bottom",
                    fontsize=7,
                    color="#777777",
                    style="italic",
                    wrap=True,
                )
            else:
                fig.tight_layout()
        # ``bbox_inches="tight"`` re-crops to include artists tight_layout does
        # not lay out -- figure-level legends and the banner -- and ``pad_inches``
        # keeps them off the border rather than flush against it.
        fig.savefig(path, dpi=self.dpi, bbox_inches="tight", pad_inches=0.25)
        plt.close(fig)
        self.written.append(path)
        return path


def jitter(n: int, center: float, spread: float = 0.08, seed: int = 0) -> np.ndarray:
    rng = np.random.default_rng(seed)
    return center + (rng.random(n) - 0.5) * 2 * spread


def empty_note(ax, text: str) -> None:
    ax.text(
        0.5,
        0.5,
        text,
        ha="center",
        va="center",
        transform=ax.transAxes,
        color="#888888",
    )
    ax.set_xticks([])
    ax.set_yticks([])
