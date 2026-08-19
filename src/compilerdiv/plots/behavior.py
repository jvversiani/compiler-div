"""Behavioral plots.

The plots are ordered by how much weight the argument puts on them:

* ``equivalence`` -- the headline: per pair, how many files are equivalent, and
  which detector broke the rest.
* ``taxonomy`` -- every departure signature by class. The empty
  ``program_dependent`` column is the claim.
* ``departure_matrix`` -- signature x affected-file-count, the manual-review map.
* ``layout_probe`` -- membership vs binary size, for the conditional class.
* ``noise_floor`` -- what the controls found, so the floor is auditable.
* ``volcano`` / ``effect_sizes`` -- retained from the original pipeline.
* ``js_divergence`` -- retained, explicitly captioned as descriptive.
"""

from __future__ import annotations

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from matplotlib.patches import Patch

from ..config import Settings
from ..corpus import display_name, truncate
from ..stats.detectors import KIND_ARG, KIND_COUNT, KIND_INSTAB, KIND_SEQ, KIND_SET
from ..stats.taxonomy import CLASS_ORDER
from .theme import CLASS_COLORS, KIND_COLORS, PlotContext, empty_note, jitter

KINDS = [KIND_SET, KIND_ARG, KIND_SEQ, KIND_INSTAB, KIND_COUNT]


#: Detector kind -> the per-file count column that records whether it fired.
KIND_NCOL = {
    KIND_SET: "n_set",
    KIND_ARG: "n_arg",
    KIND_SEQ: "n_seq",
    KIND_INSTAB: "n_instab",
    KIND_COUNT: "n_count",
}

EQUIV_COLOR = "#55A868"


def plot_equivalence(
    per_file: pd.DataFrame, settings: Settings, ctx: PlotContext
) -> None:
    """Headline: equivalent vs differing files per pair, in two panels.

    Left, the verdict: each differing file is attributed to its *sharpest*
    detector so the bar sums to the file count -- the number the paper quotes.
    That single attribution is deliberately lossy: a file that trips both the
    set and the argument detector shows only as ``set`` there, and the argument
    departure disappears.

    Right restores it. Detector *incidence* counts a file under **every**
    detector it triggered, so a program with a ``gettid`` set departure and a
    ``write`` argument departure contributes to both bars. The columns overlap
    by design and do not sum to the file count; that is the point -- it is the
    only view in which co-occurring departures are all visible.
    """
    if per_file.empty:
        return
    pairs = sorted(per_file["variant"].unique())
    x = np.arange(len(pairs))

    equiv: list[int] = []
    totals: list[int] = []
    sharpest: dict[str, list[int]] = {k: [] for k in KINDS}
    incidence: dict[str, list[int]] = {k: [] for k in KINDS}
    for v in pairs:
        g = per_file[per_file["variant"] == v]
        totals.append(len(g))
        equiv.append(int(g["equivalent"].sum()))
        diff = g[~g["equivalent"]]
        for k in KINDS:
            sharpest[k].append(0)
            incidence[k].append(int((diff[KIND_NCOL[k]] > 0).sum()))
        for _, r in diff.iterrows():
            if r["n_set"]:
                sharpest[KIND_SET][-1] += 1
            elif r["n_arg"]:
                sharpest[KIND_ARG][-1] += 1
            elif r["n_seq"]:
                sharpest[KIND_SEQ][-1] += 1
            elif r["n_instab"]:
                sharpest[KIND_INSTAB][-1] += 1
            else:
                sharpest[KIND_COUNT][-1] += 1

    fig, (ax1, ax2) = plt.subplots(
        1, 2, figsize=(3.1 * len(pairs) + 7, 6), gridspec_kw={"wspace": 0.25}
    )
    xticklabels = [
        f"{settings.label(v)}\nvs {settings.label(settings.baseline)}" for v in pairs
    ]

    # -- left: verdict (sums to file count) --------------------------------
    ax1.bar(x, equiv, 0.5, color=EQUIV_COLOR)
    bottom = np.array(equiv, dtype=float)
    for k in KINDS:
        vals = np.array(sharpest[k], dtype=float)
        if vals.sum() == 0:
            continue
        ax1.bar(x, vals, 0.5, bottom=bottom, color=KIND_COLORS[k])
        bottom += vals
    for i, (e, t) in enumerate(zip(equiv, totals)):
        if e:
            ax1.text(i, e / 2, str(e), ha="center", va="center", fontsize=10)
        if t - e:
            ax1.text(
                i, e + (t - e) / 2, str(t - e), ha="center", va="center", fontsize=10
            )
    ax1.set_xticks(x)
    ax1.set_xticklabels(xticklabels)
    ax1.set_ylabel("number of files")
    ax1.set_title("files by verdict\n(differing → sharpest detector)", fontsize=10)
    ax1.margins(y=0.12)

    # -- right: detector incidence (a file may appear under several) --------
    n_k = len(KINDS)
    bw = 0.8 / n_k
    any_incidence = any(sum(incidence[k]) for k in KINDS)
    if any_incidence:
        # Same problem as departure_matrix: a detector that fires on 1 of 672
        # files is ~0.15% of the axis, so the bar renders as a hairline and its
        # colour -- which is *which detector* fired, the point of the panel --
        # is invisible. Floor every non-zero bar at a visible height while
        # keeping the axis honest: ticks stay linear and the true count is
        # printed above each bar.
        peak = max(max(incidence[k]) for k in KINDS)
        min_h = peak * 0.015
        for i, k in enumerate(KINDS):
            off = x + (i - (n_k - 1) / 2) * bw
            vals = np.array(incidence[k], dtype=float)
            heights = np.where(vals > 0, np.maximum(vals, min_h), 0.0)
            ax2.bar(off, heights, bw, color=KIND_COLORS[k])
            for xi, val, h in zip(off, vals, heights):
                if val:
                    ax2.text(xi, h, str(int(val)), ha="center", va="bottom", fontsize=8)
        ax2.set_xticks(x)
        ax2.set_xticklabels(xticklabels)
        ax2.margins(y=0.15)
    else:
        empty_note(ax2, "no differing files")
        ax2.set_xticks([])
    ax2.set_ylabel("differing files with this departure")
    ax2.set_title(
        "detector incidence\n(a file counts under every detector it trips)",
        fontsize=10,
    )

    handles = [Patch(color=EQUIV_COLOR, label="equivalent")] + [
        Patch(color=KIND_COLORS[k], label=k) for k in KINDS
    ]
    fig.legend(
        handles=handles,
        loc="lower center",
        bbox_to_anchor=(0.5, -0.02),
        ncol=n_k + 1,
        fontsize=8,
        frameon=False,
    )
    fig.suptitle(
        f"Behavioral equivalence"
        f"{f' — {ctx.config}' if ctx.config != 'basic' else ''}",
        fontsize=13,
    )
    ctx.save(fig, "equivalence")


#: Classes always shown in the taxonomy plot. The noise classes are appended
#: only when non-empty: count departures can never be classed as noise (their
#: floor is per-file), so those columns are usually zero and just add width.
#: They can still be non-zero for set/sequence departures, which have no
#: per-file quantity to threshold, and are worth seeing when they are.
BASE_CLASS_ORDER = [c for c in CLASS_ORDER if c not in ("run_noise", "build_noise")]


def _plot_classes(taxonomy: pd.DataFrame) -> list[str]:
    extra = [c for c in ("build_noise", "run_noise") if (taxonomy["class"] == c).any()]
    return BASE_CLASS_ORDER + extra


def plot_taxonomy(taxonomy: pd.DataFrame, settings: Settings, ctx: PlotContext) -> None:
    """Departure signatures by class."""
    if taxonomy.empty:
        return
    classes = _plot_classes(taxonomy)
    pairs = sorted(taxonomy["variant"].unique())
    fig, axes = plt.subplots(
        1, len(pairs), figsize=(5.0 * len(pairs), 5.5), squeeze=False
    )

    for i, v in enumerate(pairs):
        ax = axes[0][i]
        g = taxonomy[taxonomy["variant"] == v]
        counts = [int((g["class"] == c).sum()) for c in classes]
        colors = [CLASS_COLORS[c] for c in classes]
        bars = ax.bar(range(len(classes)), counts, color=colors, width=0.6)
        ax.bar_label(bars, fontsize=9, padding=2)
        ax.set_xticks(range(len(classes)))
        ax.set_xticklabels([c.replace("_", "\n") for c in classes], fontsize=8)
        ax.set_ylabel("departure signatures")
        ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")
        ax.margins(y=0.18)

    fig.suptitle(
        f"Departure taxonomy{f" - {ctx.config}" if ctx.config != "basic" else ""}\n"
    )
    ctx.save(fig, "taxonomy")


def plot_departure_matrix(
    taxonomy: pd.DataFrame, settings: Settings, ctx: PlotContext, top_n: int = 25
) -> None:
    """Per-signature affected-file counts -- the manual-review map."""
    if taxonomy.empty:
        return
    pairs = sorted(taxonomy["variant"].unique())
    fig, axes = plt.subplots(
        1, len(pairs), figsize=(7.5 * len(pairs), 7), squeeze=False
    )

    for i, v in enumerate(pairs):
        ax = axes[0][i]
        g = taxonomy[taxonomy["variant"] == v].head(top_n)
        if g.empty:
            empty_note(ax, "no departures")
            ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")
            continue
        g = g.iloc[::-1]
        labels = [
            f"{truncate(str(r.syscall), 18)} [{r.kind}]"
            for r in g.itertuples(index=False)
        ]
        colors = [CLASS_COLORS[c] for c in g["class"]]
        ypos = np.arange(len(g))
        n_corpus = int(g["n_files_corpus"].iloc[0]) if len(g) else 0

        # A 1-file signature on a 672-file corpus is ~0.15% of the axis: the bar
        # renders as a hairline and its colour -- which is the departure class,
        # the whole point of the plot -- becomes invisible. Draw every bar at a
        # minimum visible width while keeping the axis honest: the true count is
        # printed beside each bar and the tick marks stay linear.
        counts = g["n_files_affected"].to_numpy(dtype=float)
        min_w = max(n_corpus, counts.max() if len(counts) else 1) * 0.012
        widths = np.maximum(counts, min_w)

        ax.barh(ypos, widths, color=colors)
        ax.set_yticks(ypos)
        ax.set_yticklabels(labels, fontsize=8)

        if n_corpus:
            ax.axvline(n_corpus, ls="--", lw=1.2, color="#222222", alpha=0.5)
            # Sit the label left of the line when a bar reaches the full corpus,
            # otherwise its own count label lands on top of this one.
            at_full = bool((counts >= n_corpus * 0.98).any())
            ax.text(
                n_corpus,
                len(g) - 0.3,
                f"all {n_corpus} files" if at_full else f" all {n_corpus} files",
                fontsize=9,
                ha="right" if at_full else "left",
                va="bottom",
                color="#222222",
            )

        for yi, r in enumerate(g.itertuples(index=False)):
            n = int(r.n_files_affected)
            unit = "file" if n == 1 else "files"
            delta = f"Δ {r.magnitude_values}" if r.magnitude_fixed else "Δ varies"
            ax.text(
                widths[yi],
                yi,
                f"   {n} {unit}  /  {delta}",
                va="center",
                ha="left",
                fontsize=8,
                color="#333333",
            )

        ax.set_xlim(0, max(n_corpus, widths.max()) * 1.42)
        ax.set_ylim(-0.8, len(g) + 0.2)
        ax.set_xlabel("files affected")
        ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")

    # Legend lists only the classes actually plotted. A set/sequence departure
    # can still be classed as noise (those detectors have no per-file floor), so
    # a grey bar is possible; a fixed legend would leave it unlabelled.
    present = [c for c in CLASS_ORDER if (taxonomy["class"] == c).any()]
    if present:
        handles = [plt.Rectangle((0, 0), 1, 1, color=CLASS_COLORS[c]) for c in present]
        # Anchor fully below the axes (tight_layout overrides subplots_adjust,
        # so reserving bottom space that way does not survive) so the class
        # names clear the "files affected" x-label instead of landing on it; the
        # tight bbox at save time brings the strip back into frame with padding.
        fig.legend(
            handles,
            present,
            loc="upper center",
            bbox_to_anchor=(0.5, 0),
            ncol=len(present),
            fontsize=10,
            frameon=False,
        )
    fig.suptitle(
        f"Departure signatures by affected-file count{f" - {ctx.config}" if ctx.config != "basic" else ""}"
    )
    ctx.save(fig, "departure_matrix")


def plot_layout_probe(
    taxonomy: pd.DataFrame,
    per_file: pd.DataFrame,
    frames,
    settings: Settings,
    ctx: PlotContext,
    max_panels: int = 6,
    min_frac: float = 0.01,
) -> None:
    """For conditional departures: is membership explained by binary size?

    This is the plot that answers the ``read(+1)`` question. If the
    affected files cluster at a different binary size than the unaffected ones,
    membership is a linking artifact and not program behavior.

    Only departures affecting at least ``min_frac`` of the corpus are shown. A
    signature affecting one file cannot exhibit separation -- the correlation is
    a single point against 671 -- so plotting it wastes a panel and reports a
    rho that means nothing. The panels shown are the largest qualifying ones,
    and the grid shrinks when fewer qualify.
    """
    cond = (
        taxonomy[taxonomy["class"] == "conditional"]
        if not taxonomy.empty
        else pd.DataFrame()
    )
    if cond.empty:
        return
    elf = frames.elf
    if elf.empty:
        return
    e = elf[(elf["config"] == ctx.config) & (elf["compiler"] == settings.baseline)]
    if e.empty:
        return
    e = e.drop_duplicates("file").set_index("file")

    n_corpus = int(cond["n_files_corpus"].max())
    min_files = max(2, int(np.ceil(n_corpus * min_frac)))
    cond = cond[cond["n_files_affected"] >= min_files]
    if cond.empty:
        return

    rows = cond.head(max_panels)
    n = len(rows)
    ncols = min(3, n)
    nrows = int(np.ceil(n / ncols))
    fig, axes = plt.subplots(
        nrows, ncols, figsize=(5 * ncols, 4.2 * nrows), squeeze=False
    )

    for idx, r in enumerate(rows.itertuples(index=False)):
        ax = axes[idx // ncols][idx % ncols]
        affected = set(str(r.affected_files).split("; ")) if r.affected_files else set()
        pf = per_file[per_file["variant"] == r.variant]
        files = [f for f in pf["file"].astype(str) if f in e.index]
        if len(files) < 4:
            empty_note(ax, "not enough layout data")
            continue
        sizes = (
            pd.to_numeric(e.loc[files, "size_b"], errors="coerce").to_numpy(dtype=float)
            / 1024
        )
        member = np.array([f in affected for f in files])

        ax.scatter(
            jitter(int((~member).sum()), 0.0, 0.14),
            sizes[~member],
            s=12,
            alpha=0.45,
            color="#BBBBBB",
            label="unaffected",
        )
        ax.scatter(
            jitter(int(member.sum()), 1.0, 0.14),
            sizes[member],
            s=14,
            alpha=0.65,
            color=CLASS_COLORS["conditional"],
            label="affected",
        )
        ax.set_xticks([0, 1])
        ax.set_xticklabels(["unaffected", "affected"], fontsize=8)
        ax.set_ylabel("binary size (KiB)")
        rho = r.layout_rho_size
        rho_s = f"rho={rho:+.3f}" if rho is not None else "rho=n/a"
        ax.set_title(
            f"{r.syscall} [{r.kind}] {settings.label(r.variant)}\n"
            f"{r.n_files_affected}/{r.n_files_corpus} files, {rho_s}",
            fontsize=9,
        )
        ax.legend(fontsize=7)

    for idx in range(n, nrows * ncols):
        axes[idx // ncols][idx % ncols].set_visible(False)

    fig.suptitle(
        f"Layout probe for conditional departures{f" - {ctx.config}" if ctx.config != "basic" else ""}\n",
        fontsize=10,
    )
    ctx.save(fig, "layout_probe")


def plot_noise_floor(noise: pd.DataFrame, ctx: PlotContext, top_n: int = 20) -> None:
    """What the controls found, so the floor is auditable rather than asserted."""
    if noise.empty:
        return
    g = noise.copy()
    g["total"] = g["n_files_differ_rerun"] + g["n_files_differ_rebuild"]
    g = g.sort_values("total", ascending=False).head(top_n).iloc[::-1]
    if g.empty:
        return

    n_builds = int(g["n_builds"].iloc[0]) if "n_builds" in g else 2
    n_passes = int(g["n_passes"].iloc[0]) if "n_passes" in g else 2
    ypos = np.arange(len(g))
    fig, ax = plt.subplots(figsize=(9, max(4, 0.32 * len(g) + 2)))
    ax.barh(
        ypos - 0.2,
        g["n_files_differ_rerun"],
        0.38,
        label="within a build (run noise)",
        color="#8C8C8C",
    )
    ax.barh(
        ypos + 0.2,
        g["n_files_differ_rebuild"],
        0.38,
        label="between builds (build + run)",
        color="#DD8452",
    )
    ax.set_yticks(ypos)
    ax.set_yticklabels(g["syscall"], fontsize=8)
    ax.set_xlabel("files differing")
    ax.set_title(
        f"Noise floor from the control grid{f" - {ctx.config}" if ctx.config != "basic" else ""}\n"
        f"{n_builds} builds x {n_passes} trace passes of the baseline"
    )
    ax.legend(fontsize=8)
    ctx.save(fig, "noise_floor")


def plot_js_divergence(
    per_file: pd.DataFrame,
    noise_js: pd.DataFrame,
    settings: Settings,
    ctx: PlotContext,
) -> None:
    """Descriptive only. Caption carries the caveat."""
    if per_file.empty:
        return
    keys, labels, data = [], [], []
    if not noise_js.empty:
        keys.append(settings.rerun_tags(0)[0])
        labels.append(f"noise floor\n({settings.label(settings.baseline)} vs itself)")
        data.append(noise_js["js_divergence"].dropna().to_numpy())
    for v in sorted(per_file["variant"].unique()):
        vals = per_file[per_file["variant"] == v]["js_divergence"].dropna().to_numpy()
        if len(vals) == 0:
            continue
        keys.append(v)
        labels.append(f"{settings.label(v)}\nvs {settings.label(settings.baseline)}")
        data.append(vals)
    if not data:
        return

    fig, ax = plt.subplots(figsize=(1.9 * len(keys) + 3, 6))
    ax.boxplot(data, showfliers=False, widths=0.5, medianprops={"color": "k"})
    for i, (k, vals) in enumerate(zip(keys, data), start=1):
        ax.scatter(jitter(len(vals), i), vals, s=10, alpha=0.4, color=settings.color(k))
    ax.set_xticks(range(1, len(keys) + 1))
    ax.set_xticklabels(labels)
    ax.set_ylabel("Jensen-Shannon divergence (vs baseline)")
    n_files = max(len(v) for v in data)
    ax.set_title(
        f"Per-file syscall-count divergence{f" - {ctx.config}" if ctx.config != "basic" else ""} ({n_files} files)"
    )

    old = ctx.banner
    ctx.banner = ctx.banner + "  |  " if ctx.banner else ""
    ctx.save(fig, "js_divergence")
    ctx.banner = old


def plot_volcano(sig: pd.DataFrame, settings: Settings, ctx: PlotContext) -> None:
    """Retained from the original pipeline."""
    if sig.empty:
        return
    pairs = sorted(sig["variant"].unique())
    fig, axes = plt.subplots(
        1, len(pairs), figsize=(6.5 * len(pairs), 5.5), squeeze=False
    )
    for i, v in enumerate(pairs):
        ax = axes[0][i]
        df = sig[sig["variant"] == v]
        if df.empty:
            ax.set_visible(False)
            continue
        log2fc = df["median_log_ratio"].to_numpy() / np.log(2)
        neglogp = -np.log10(np.clip(df["p_adj_bh"].to_numpy(), 1e-300, 1))
        s = df["significant_5pct"].to_numpy()
        nz = df["noisy"].to_numpy()
        ax.scatter(log2fc[~s & ~nz], neglogp[~s & ~nz], c="#999999", s=26, label="n.s.")
        ax.scatter(
            log2fc[nz],
            neglogp[nz],
            c="#C44E52",
            marker="x",
            s=34,
            label="noisy (excluded)",
        )
        ax.scatter(
            log2fc[s],
            neglogp[s],
            c=settings.color(v),
            s=44,
            edgecolor="k",
            linewidth=0.4,
            label="significant",
        )
        for _, r in df[df["significant_5pct"]].iterrows():
            ax.annotate(
                r["syscall"],
                (
                    r["median_log_ratio"] / np.log(2),
                    -np.log10(max(r["p_adj_bh"], 1e-300)),
                ),
                fontsize=7,
                xytext=(3, 3),
                textcoords="offset points",
            )
        ax.axhline(-np.log10(0.05), ls="--", lw=0.8, color="k")
        ax.axvline(0, ls=":", lw=0.8, color="k")
        ax.set_xlabel("median log2 fold-change (variant / baseline)")
        ax.set_ylabel("-log10 BH-adjusted p")
        ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")
        ax.legend(fontsize=8)
    fig.suptitle(
        f"Syscall count fold-change vs significance{f" - {ctx.config}" if ctx.config != "basic" else ""}"
    )
    ctx.save(fig, "volcano")


def plot_effect_sizes(
    sig: pd.DataFrame, settings: Settings, ctx: PlotContext, top_n: int = 15
) -> None:
    """Retained from the original pipeline."""
    if sig.empty:
        return
    pairs = sorted(sig["variant"].unique())
    fig, axes = plt.subplots(
        1, len(pairs), figsize=(6.5 * len(pairs), 6.5), squeeze=False
    )
    for i, v in enumerate(pairs):
        ax = axes[0][i]
        df = sig[sig["variant"] == v].copy()
        if df.empty:
            ax.set_visible(False)
            continue
        df["l2fc"] = df["median_log_ratio"] / np.log(2)
        df = (
            df.reindex(df["l2fc"].abs().sort_values(ascending=False).index)
            .head(top_n)
            .iloc[::-1]
        )
        colors = [settings.color(v) if s else "#BBBBBB" for s in df["significant_5pct"]]
        ax.barh(df["syscall"], df["l2fc"], color=colors)
        ax.axvline(0, lw=0.8, color="k")
        ax.set_xlabel("median log2 fold-change (variant / baseline)")
        ax.set_title(
            f"{settings.label(v)} vs {settings.label(settings.baseline)}\n(solid = significant)"
        )
        ax.tick_params(axis="y", labelsize=7)
    fig.suptitle(
        f"Largest per-syscall count differences{f" - {ctx.config}" if ctx.config != "basic" else ""}"
    )
    ctx.save(fig, "effect_sizes")


def plot_total_syscalls(
    counts: pd.DataFrame, settings: Settings, ctx: PlotContext
) -> None:
    """Retained: per-file total syscall volume per compiler."""
    sub = counts[counts["config"] == ctx.config]
    if sub.empty:
        return
    totals = sub.groupby(["compiler", "file"])["mean_count"].sum().reset_index()
    keys = [
        k for k in settings.compilers_for(ctx.config) if k in set(totals["compiler"])
    ]
    if not keys:
        return
    data = [totals[totals["compiler"] == k]["mean_count"].to_numpy() for k in keys]

    fig, ax = plt.subplots(figsize=(1.8 * len(keys) + 3, 6))
    ax.boxplot(data, showfliers=False, widths=0.5, medianprops={"color": "k"})
    for i, (k, vals) in enumerate(zip(keys, data), start=1):
        ax.scatter(jitter(len(vals), i), vals, s=10, alpha=0.4, color=settings.color(k))
    ax.set_xticks(range(1, len(keys) + 1))
    ax.set_xticklabels([settings.label(k) for k in keys])
    ax.set_ylabel("mean total syscalls per run")
    ax.set_title(
        f"Total syscall volume per file{f" - {ctx.config}" if ctx.config != "basic" else ""}"
    )
    ctx.save(fig, "total_syscalls")


def plot_top_divergent(
    per_file: pd.DataFrame, settings: Settings, ctx: PlotContext, top_n: int = 30
) -> None:
    """Most-departing files per pair.

    Deliberately unannotated: a file can carry several departures and only the
    first would fit beside the bar, so the label would be an arbitrary sample
    presented as a summary. The full list per file is in the ``per_file`` sheet
    of ``equivalence.xlsx``.
    """
    if per_file.empty:
        return
    pairs = sorted(per_file["variant"].unique())
    fig, axes = plt.subplots(1, len(pairs), figsize=(8 * len(pairs), 8), squeeze=False)
    for i, v in enumerate(pairs):
        ax = axes[0][i]
        g = per_file[(per_file["variant"] == v) & (per_file["n_departures"] > 0)]
        g = g.head(top_n).iloc[::-1]
        if g.empty:
            empty_note(ax, "no departing files")
            ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")
            continue
        ypos = np.arange(len(g))
        ax.barh(ypos, g["n_departures"], color=settings.color(v))
        ax.set_yticks(ypos)
        ax.set_yticklabels([truncate(display_name(f)) for f in g["file"]], fontsize=6)
        ax.set_xlabel("number of departures")
        ax.set_title(f"{settings.label(v)} vs {settings.label(settings.baseline)}")
    fig.suptitle(
        f"Top {top_n} most-departing files{f" - {ctx.config}" if ctx.config != "basic" else ""}"
    )
    ctx.save(fig, "top_divergent")
