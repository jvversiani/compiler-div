"""The analysis pipeline.

Reads only the raw store, writes only reports and figures. Never compiles,
never traces: acquisition and analysis meet at the raw log and nowhere else, so
you can iterate on thresholds and plots without re-running a multi-hour sweep.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path

import pandas as pd

from .config import Settings
from .corpus import load_corpus, validate_corpus
from .plots import bench as bench_plots
from .plots import behavior as behavior_plots
from .plots.theme import PlotContext
from .report import (
    print_bench_report,
    print_equivalence_report,
    write_behavior,
    write_benchmark,
)
from .stats import bench as bench_stats
from .stats.detectors import all_departures
from .stats.enrich import enrich_departures
from .stats.equivalence import (
    per_file_js,
    per_file_verdict,
    per_syscall_logratios,
    syscall_significance,
)
from .stats.noise import build_vs_variant, derive_noise, noise_table
from .stats.taxonomy import classify
from .store.aggregate import Frames, PooledBaseline
from .store.raw import RawStore


@dataclass
class AnalysisResult:
    per_file: pd.DataFrame = field(default_factory=pd.DataFrame)
    departures: pd.DataFrame = field(default_factory=pd.DataFrame)
    taxonomy: pd.DataFrame = field(default_factory=pd.DataFrame)
    significance: pd.DataFrame = field(default_factory=pd.DataFrame)
    noise: pd.DataFrame = field(default_factory=pd.DataFrame)
    build_compare: pd.DataFrame = field(default_factory=pd.DataFrame)
    bench_geomeans: pd.DataFrame = field(default_factory=pd.DataFrame)
    bench_foldchange: pd.DataFrame = field(default_factory=pd.DataFrame)
    figures: list[Path] = field(default_factory=list)
    workbooks: list[Path] = field(default_factory=list)


def _concat(parts: list[pd.DataFrame]) -> pd.DataFrame:
    parts = [p for p in parts if p is not None and not p.empty]
    return pd.concat(parts, ignore_index=True) if parts else pd.DataFrame()


def analyze(
    settings: Settings, store: RawStore, *, verbose: bool = True
) -> AnalysisResult:
    # Before anything is computed or written: one corpus, one language. Checked
    # here and not only in load_corpus so a mixed corpus aborts up front rather
    # than after the workbooks are already on disk.
    validate_corpus(settings.corpus_path, settings.languages)

    frames = Frames(store)
    res = AnalysisResult()

    if frames.counts.empty and frames.bench.empty:
        raise RuntimeError(
            f"no data in the raw store at {store.root}. Run `compilerdiv acquire` first."
        )

    pf_parts, dep_parts, tax_parts, sig_parts, noise_parts = [], [], [], [], []
    bc_parts = []
    fig_dir = settings.out_path / "figures"

    for config in settings.config_names:
        counts_c = frames.counts[frames.counts["config"] == config]
        if counts_c.empty:
            continue

        floor = derive_noise(frames, settings, config)
        noise_parts.append(noise_table(floor, config))
        bc = build_vs_variant(frames, settings, config)
        bc_parts.append(bc)

        if not bc.empty:
            same = bc[bc["verdict"].str.startswith("SAME")]
            if not same.empty and verbose:
                print(
                    f"[{config}] [!] {len(same)} variant finding(s) sit on top of the "
                    "baseline's own build variance:"
                )
                for r in same.itertuples(index=False):
                    print(
                        f"      {r.syscall} / {r.variant_label}: baseline differs in "
                        f"{r.rebuild_n_files} files ({r.rebuild_deltas}), variant in "
                        f"{r.variant_n_files} ({r.variant_deltas}), overlap {r.n_overlap}"
                    )
                print(
                    f"[{config}]     -> see the build_vs_variant sheet; these are not "
                    "cleanly attributable to the variant."
                )

        if verbose:
            ctl = settings.controls
            print(
                f"\n[{config}] noise floor "
                f"({ctl.builds} builds x {ctl.passes} passes x "
                f"{settings.trace.reps} reps = {ctl.n_groups * settings.trace.reps} "
                "baseline traces per program):"
            )
            print(
                f"  within a build : {floor.n_files_rerun} files compared, "
                f"{len(floor.run_noisy)} run-noisy syscalls"
                + (
                    f" ({', '.join(sorted(floor.run_noisy))})"
                    if floor.run_noisy
                    else ""
                )
            )
            if floor.n_files_rebuild:
                print(
                    f"  between builds : {floor.n_files_rebuild} files compared, "
                    f"{len(floor.build_noisy)} additionally build-noisy"
                    + (
                        f" ({', '.join(sorted(floor.build_noisy))})"
                        if floor.build_noisy
                        else ""
                    )
                )
            else:
                print(
                    "  between builds : not run (controls.builds = 1). Without it, a "
                    "difference that is stable across reruns cannot be distinguished "
                    "from build nondeterminism in the baseline itself."
                )
            if ctl.builds < 3:
                print(
                    f"  [!] {ctl.builds} builds estimates build variance from n="
                    f"{ctl.builds}: it detects a difference but cannot say whether "
                    "it reproduces. Raise controls.builds to 3+."
                )
            if floor.unstable_seq_files:
                print(
                    f"  sequence oracle unreliable on {len(floor.unstable_seq_files)} "
                    "files (passes of one build disagreed on their n-grams)"
                )

        noisy = floor.all_noisy
        variants = [v for v in settings.variant_keys if v in set(counts_c["compiler"])]

        # The detectors compare against the baseline pooled over ALL its control
        # groups. Using the primary group alone would spend the control budget
        # on a better noise floor and then ignore it, reporting the baseline's
        # own build quirks as departures of the variant. `derive_noise` above
        # deliberately keeps the unpooled frames -- it compares the groups.
        pooled = PooledBaseline(frames, settings, config)

        missing = set(settings.baseline_tags()) - set(counts_c["compiler"])
        if missing and verbose:
            print(
                f"[{config}] [!] {len(missing)} of "
                f"{len(settings.baseline_tags())} baseline control groups are "
                f"absent from the store ({', '.join(sorted(missing))}).\n"
                f"[{config}]     The grid in the config does not match what was "
                "acquired, so the noise floor and the pooled baseline are both "
                "weaker than configured. Re-acquire with `--reset`."
            )

        cfg_pf, cfg_dep, cfg_tax, cfg_sig = [], [], [], []
        for variant in variants:
            dep = all_departures(pooled, settings, config, variant, noisy, floor=floor)
            dep = enrich_departures(dep, pooled, floor, settings, config, variant)
            cfg_dep.append(dep)

            # Classification runs before the per-file roll-up: `worst_class`
            # lets per_file sort by what matters rather than by how many.
            tax = classify(dep, pooled, floor, settings, config, variant, bc)
            cfg_tax.append(tax)

            pf = per_file_verdict(
                dep,
                pooled,
                settings,
                config,
                variant,
                floor.unstable_seq_files,
                taxonomy=tax,
            )
            cfg_pf.append(pf)

            logr = per_syscall_logratios(
                pooled.counts, config, settings.baseline, variant
            )
            sig = syscall_significance(
                logr, config, settings.baseline, variant, noisy, departures=dep
            )
            cfg_sig.append(sig)

        pf_all = _concat(cfg_pf)
        dep_all = _concat(cfg_dep)
        tax_all = _concat(cfg_tax)
        sig_all = _concat(cfg_sig)

        pf_parts.append(pf_all)
        dep_parts.append(dep_all)
        tax_parts.append(tax_all)
        sig_parts.append(sig_all)

        # -- figures for this config ---------------------------------------
        banner = ""
        if settings.detect.excluded_syscalls:
            banner = "excluded from count stats: " + ", ".join(
                sorted(settings.detect.excluded_syscalls)
            )
        ctx = PlotContext(out_dir=fig_dir, config=config, banner=banner)

        if verbose:
            print(f"[{config}] plotting ...")

        behavior_plots.plot_equivalence(pf_all, settings, ctx)
        behavior_plots.plot_taxonomy(tax_all, settings, ctx)
        behavior_plots.plot_departure_matrix(tax_all, settings, ctx)
        behavior_plots.plot_layout_probe(tax_all, pf_all, frames, settings, ctx)
        behavior_plots.plot_noise_floor(noise_table(floor, config), ctx)
        behavior_plots.plot_top_divergent(pf_all, settings, ctx)
        behavior_plots.plot_total_syscalls(frames.counts, settings, ctx)
        behavior_plots.plot_volcano(sig_all, settings, ctx)
        behavior_plots.plot_effect_sizes(sig_all, settings, ctx)

        # The noise overlay is the baseline against a rerun of its own binary --
        # unpooled on purpose, since it is exactly the run-to-run scale that the
        # variants' divergence has to be read against.
        noise_js = (
            per_file_js(
                frames.counts, config, settings.baseline, settings.rerun_tags(0)[0]
            )
            if settings.controls.rerun_enabled
            else pd.DataFrame()
        )
        behavior_plots.plot_js_divergence(pf_all, noise_js, settings, ctx)
        res.figures.extend(ctx.written)

    res.per_file = _concat(pf_parts)
    res.departures = _concat(dep_parts)
    res.taxonomy = _concat(tax_parts)
    res.significance = _concat(sig_parts)
    res.noise = _concat(noise_parts)
    res.build_compare = _concat(bc_parts)

    # -- benchmark ---------------------------------------------------------
    bench = frames.bench
    if not bench.empty:
        summary = bench_stats.summarize(bench, settings)
        res.bench_geomeans = summary.geomeans
        res.bench_foldchange = summary.foldchange
        if verbose:
            print("\n[bench] plotting ...")
        res.figures.extend(
            bench_plots.plot_all_bench(
                bench,
                summary.geomeans,
                summary.foldchange,
                settings,
                fig_dir / "benchmark",
            )
        )

    # -- corpus figure -----------------------------------------------------
    try:
        programs = load_corpus(settings.corpus_path)
        p = bench_plots.plot_sloc_density(programs, fig_dir / "corpus")
        if p:
            res.figures.append(p)
    except (FileNotFoundError, ValueError):
        pass

    # -- workbooks ---------------------------------------------------------
    out = settings.out_path
    res.workbooks.append(
        write_behavior(
            out,
            res.per_file,
            res.departures,
            res.taxonomy,
            res.significance,
            res.noise,
        )
    )
    if not res.bench_geomeans.empty:
        res.workbooks.append(
            write_benchmark(out, res.bench_geomeans, res.bench_foldchange)
        )

    if verbose:
        print_equivalence_report(res.per_file, res.taxonomy, settings)
        print_bench_report(res.bench_geomeans, res.bench_foldchange, settings)

    return res
