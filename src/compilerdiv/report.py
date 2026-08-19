"""Report writers: the workbook and terminal output.

Two workbooks, with distinct audiences:

``behavior.xlsx``
    Everything about behavioral equivalence, in one place: the per-file
    résumé, the individual departures with their evidence, the taxonomy that
    classifies them, the corpus-wide per-syscall statistics, and the control
    passes those rest on. A ``readme`` sheet glosses every column of every
    sheet.

``benchmark.xlsx``
    Geomeans and fold-changes. A different question -- cost, not behavior.

The presentation layer lives here and nowhere else. The analysis frames keep
column names that suit the code (``magnitude``, ``variant`` + ``variant_label``,
``layout_rho_size``); the projections below rename, split and prune them into
something a reader can interpret without the source. Keeping that split means a
column can be renamed for humans without touching a detector or a plot.
"""

from __future__ import annotations

from pathlib import Path

import numpy as np
import pandas as pd

from .config import Settings
from .stats.detectors import KIND_ARG, KIND_COUNT, KIND_INSTAB, KIND_SEQ, KIND_SET
from .stats.enrich import ARG_NONE_SEEN, ARG_NOT_TRACED
from .stats.equivalence import JS_CAVEAT
from .stats.taxonomy import CLASS_ORDER, CLASS_PRIOR

WORKBOOK = "behavior.xlsx"

#: Sheet order in the workbook. ``readme`` first so it is what opens.
SHEETS = [
    "readme",
    "per_file",
    "departures",
    "taxonomy",
    "per_syscall_stats",
    "noise_floor",
]


# ---------------------------------------------------------------------------
# Projections: analysis frame -> sheet
# ---------------------------------------------------------------------------


def _merge_variant(df: pd.DataFrame) -> pd.DataFrame:
    """Collapse ``variant`` + ``variant_label`` into one ``B (mrustc)`` column.

    The letter is load-bearing (it names the output folder) and the label is
    what a reader recognises. Two columns to say that is one too many.
    """
    if df.empty or "variant_label" not in df.columns:
        return df
    out = df.copy()
    out["variant"] = [
        f"{v} ({lab})" for v, lab in zip(out["variant"], out["variant_label"])
    ]
    return out.drop(columns="variant_label")


#: ``(kind, sign of magnitude) -> direction``. Replaces the ``±1`` that the
#: ``set`` detector used to smuggle through the numeric ``magnitude`` column.
_DIRECTION = {
    (KIND_SET, True): "new",
    (KIND_SET, False): "missing",
    (KIND_COUNT, True): "more",
    (KIND_COUNT, False): "fewer",
    (KIND_ARG, True): "new value",
    (KIND_ARG, False): "new value",
    (KIND_SEQ, True): "reordered",
    (KIND_SEQ, False): "reordered",
    (KIND_INSTAB, True): "unstable",
    (KIND_INSTAB, False): "unstable",
}

#: Kinds where ``magnitude`` is an actual quantity. For ``set`` it was a
#: direction flag and for ``argument`` a constant ``1.0``; both are blanked so
#: nobody averages them.
_QUANTITATIVE = {KIND_COUNT, KIND_SEQ, KIND_INSTAB}

#: Kinds whose finding cannot be read off `delta` alone and need the prose.
_NEEDS_EVIDENCE = {KIND_SEQ, KIND_INSTAB}

DEPARTURE_SHEET_COLUMNS = [
    "config",
    "variant",
    "file",
    "kind",
    "syscall",
    "direction",
    "delta",
    "baseline_mean",
    "variant_mean",
    "baseline_sd",
    "variant_sd",
    "baseline_arg",
    "variant_arg",
    "arg_status",
    "baseline_selfdiff",
    "evidence",
]


def project_departures(df: pd.DataFrame) -> pd.DataFrame:
    """Split the overloaded ``magnitude`` / ``detail`` pair into honest columns.

    ``detail`` was a rounded restatement of ``magnitude`` with the syscall name
    glued on -- ``connect(-3)`` for a delta of ``-3.4``. ``magnitude`` meant four
    different things depending on ``kind``. Here they become ``direction`` (a
    word), ``delta`` (an exact number, blank where there was never a quantity)
    and ``evidence`` (free text only where no number can say it).
    """
    if df.empty:
        return pd.DataFrame(columns=DEPARTURE_SHEET_COLUMNS)

    out = _merge_variant(df).copy()
    mag = pd.to_numeric(out["magnitude"], errors="coerce")

    out["direction"] = [
        _DIRECTION.get((str(k), bool(m > 0)), "") for k, m in zip(out["kind"], mag)
    ]
    out["delta"] = [
        float(m) if str(k) in _QUANTITATIVE else np.nan
        for k, m in zip(out["kind"], mag)
    ]

    # Two kinds have a finding that no single number states. For sequence rows
    # it is which n-grams appeared and how big the stable sets were; for
    # instability rows it is the shape of the rep sample on both sides, which
    # `detail` already spells out in full.
    def _evidence(kind: str, detail: str, base_v: str, var_v: str) -> str:
        if kind == KIND_SEQ:
            return f"{detail} (baseline {base_v}, variant {var_v})"
        return detail if kind in _NEEDS_EVIDENCE else ""

    out["evidence"] = [
        _evidence(str(k), str(d), str(bv), str(vv))
        for k, d, bv, vv in zip(
            out["kind"], out["detail"], out["baseline_value"], out["variant_value"]
        )
    ]
    return out.reindex(columns=DEPARTURE_SHEET_COLUMNS).reset_index(drop=True)


PER_FILE_SHEET_COLUMNS = [
    "config",
    "variant",
    "file",
    "equivalent",
    "worst_class",
    "n_departures",
    "n_set",
    "n_count",
    "n_arg",
    "n_seq",
    "n_instab",
    "top_syscalls",
    "seq_oracle_reliable",
    "js_divergence",
]

TAXONOMY_SHEET_COLUMNS = [
    "config",
    "variant",
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
    "layout_best_rho",
    "layout_best_source",
    "layout_status",
    "selfdiff_n_files",
    "selfdiff_deltas",
    "selfdiff_verdict",
    "affected_files",
]


def _project(df: pd.DataFrame, columns: list[str]) -> pd.DataFrame:
    if df.empty:
        return pd.DataFrame(columns=columns)
    return _merge_variant(df).reindex(columns=columns).reset_index(drop=True)


# ---------------------------------------------------------------------------
# The glossary
# ---------------------------------------------------------------------------

#: ``(sheet, column, meaning)``. This is the workbook's only readme, and
#: ``src/tests/test_report.py`` asserts it covers every column of every sheet
#: exactly -- a hand-maintained readme drifts, a tested one cannot.
GLOSSARY: list[tuple[str, str, str]] = [
    # -- per_file ---------------------------------------------------------
    ("per_file", "config", "Compiler-flag configuration the row was measured under."),
    ("per_file", "variant", "Compiler under test, as folder letter and name."),
    ("per_file", "file", "Corpus program (source file stem)."),
    (
        "per_file",
        "equivalent",
        "TRUE when no detector fired: this program's behavior is "
        "indistinguishable from the baseline's.",
    ),
    (
        "per_file",
        "worst_class",
        "Worst taxonomy class among this file's departures (see the taxonomy "
        "sheet). The sheet is sorted by it: program_dependent first.",
    ),
    ("per_file", "n_departures", "Total departures on this file, all detectors."),
    ("per_file", "n_set", "Departures from the set detector (a syscall type)."),
    ("per_file", "n_count", "Departures from the count detector (how many)."),
    ("per_file", "n_arg", "Departures from the argument detector (a value)."),
    ("per_file", "n_seq", "Departures from the sequence detector (ordering)."),
    (
        "per_file",
        "n_instab",
        "Departures from the instability detector: the baseline was "
        "reproducible on this program and the variant was not.",
    ),
    ("per_file", "top_syscalls", "Up to 5 syscalls involved, for scanning."),
    (
        "per_file",
        "seq_oracle_reliable",
        "FALSE when two trace passes of one build disagreed on their stable "
        "n-grams for this program; "
        "sequence findings here are not trustworthy.",
    ),
    (
        "per_file",
        "js_divergence",
        "Jensen-Shannon divergence of the count histograms. DESCRIPTIVE ONLY -- "
        "never gates a verdict. See the notes below.",
    ),
    # -- departures -------------------------------------------------------
    ("departures", "config", "Compiler-flag configuration."),
    ("departures", "variant", "Compiler under test, as folder letter and name."),
    ("departures", "file", "Corpus program the departure was observed on."),
    (
        "departures",
        "kind",
        "Which detector fired: set (a syscall type the baseline never makes), "
        "count (more/fewer of one it does), argument (a value it never passes), "
        "sequence (an ordering it never produces), instability (one the "
        "baseline makes reproducibly and the variant does not).",
    ),
    (
        "departures",
        "syscall",
        "The syscall. For sequence rows this is the pipe-joined set of n-gram "
        "head syscalls, not a single call.",
    ),
    (
        "departures",
        "direction",
        "new / missing (set), more / fewer (count), new value (argument), "
        "reordered (sequence), unstable (instability).",
    ),
    (
        "departures",
        "delta",
        "Exact signed difference: mean count difference for count rows, number "
        "of new stable n-grams for sequence rows, and for instability rows the "
        "span of the variant's counts (absent reps counted as zero). BLANK for "
        "set and argument rows, where there is no quantity -- read `direction` "
        "instead.",
    ),
    (
        "departures",
        "baseline_mean",
        "Baseline's mean count of this syscall on this file, over trace.reps "
        "reps. A fraction is an average, not a count -- see the notes below.",
    ),
    ("departures", "variant_mean", "Same, under the variant."),
    (
        "departures",
        "baseline_sd",
        "Standard deviation behind baseline_mean. Non-zero means the baseline "
        "itself varied run to run on this program.",
    ),
    ("departures", "variant_sd", "Same, under the variant."),
    (
        "departures",
        "baseline_arg",
        "Normalised argument values the baseline stably passed to this syscall "
        "on this file. Empty cells are explained by arg_status.",
    ),
    ("departures", "variant_arg", "Same, under the variant."),
    (
        "departures",
        "arg_status",
        f"Why the argument columns are what they are: 'captured', "
        f"'{ARG_NONE_SEEN}', or '{ARG_NOT_TRACED}' (widen trace.arg_syscalls "
        "and re-acquire to change that).",
    ),
    (
        "departures",
        "baseline_selfdiff",
        "How far the baseline disagreed with ITSELF on this (file, syscall) "
        "across two builds of the same source. 0 means the difference is the "
        "variant's; non-zero means read taxonomy.selfdiff_verdict first.",
    ),
    (
        "departures",
        "evidence",
        "Free text where no number suffices -- the new n-grams, for sequence "
        "rows. Blank everywhere else.",
    ),
    # -- taxonomy ---------------------------------------------------------
    ("taxonomy", "config", "Compiler-flag configuration."),
    ("taxonomy", "variant", "Compiler under test, as folder letter and name."),
    ("taxonomy", "syscall", "The syscall this signature is about."),
    ("taxonomy", "kind", "Which detector produced it (see the departures sheet)."),
    ("taxonomy", "class", "Assigned class. Priors are listed at the bottom."),
    ("taxonomy", "prior", "What the class implies, in words."),
    ("taxonomy", "n_files_affected", "Corpus programs carrying this signature."),
    ("taxonomy", "n_files_corpus", "Programs compared for this variant."),
    ("taxonomy", "frac_affected", "n_files_affected / n_files_corpus."),
    (
        "taxonomy",
        "magnitude_fixed",
        "TRUE when the delta is the same on every affected file -- the mark of "
        "a structural offset rather than program-dependent work.",
    ),
    ("taxonomy", "magnitude_values", "The distinct delta values observed."),
    ("taxonomy", "magnitude_median", "Median delta across affected files."),
    (
        "taxonomy",
        "rho_vs_sloc",
        "Spearman rho of |delta| against source lines, over affected files. "
        "High means the delta grows with the program: the program_dependent "
        "signal.",
    ),
    (
        "taxonomy",
        "layout_best_rho",
        "Strongest correlation between BEING affected and a static layout fact. "
        "High means membership tracks link layout, not program behavior -- "
        "evidence of benignity.",
    ),
    (
        "taxonomy",
        "layout_best_source",
        "Which layout fact layout_best_rho came from: size, segments or sections.",
    ),
    (
        "taxonomy",
        "layout_status",
        "Why a layout correlation is missing, when it is. 'no layout signal' "
        "and 'could not compute' license opposite conclusions.",
    ),
    (
        "taxonomy",
        "selfdiff_n_files",
        "Files where the baseline differed from ITSELF in this syscall across "
        "two builds. 0 is the clean case.",
    ),
    ("taxonomy", "selfdiff_deltas", "The distinct deltas the baseline showed itself."),
    (
        "taxonomy",
        "selfdiff_verdict",
        "Whether the variant's finding is the same phenomenon as the baseline's "
        "own build variance. 'SAME mechanism' means the finding is NOT cleanly "
        "attributable to the variant.",
    ),
    ("taxonomy", "affected_files", "Full membership, semicolon-separated."),
    # -- per_syscall_stats ------------------------------------------------
    ("per_syscall_stats", "config", "Compiler-flag configuration."),
    ("per_syscall_stats", "pair", "Variant versus baseline, as compiler keys."),
    ("per_syscall_stats", "variant", "Compiler under test (folder letter)."),
    ("per_syscall_stats", "syscall", "The syscall tested."),
    ("per_syscall_stats", "n_files", "Programs where both compilers were measured."),
    (
        "per_syscall_stats",
        "n_files_differ",
        "Programs where this syscall differs by ANY detector, not only by count.",
    ),
    ("per_syscall_stats", "median_log_ratio", "Median log((variant+1)/(baseline+1))."),
    ("per_syscall_stats", "median_fold_change", "exp(median_log_ratio)."),
    ("per_syscall_stats", "wilcoxon_stat", "Wilcoxon signed-rank statistic."),
    (
        "per_syscall_stats",
        "p_value",
        "Wilcoxon p, COUNT-BASED only: it cannot see an argument or ordering "
        "difference, so a nonzero n_files_differ with p=1 is not a mystery.",
    ),
    (
        "per_syscall_stats",
        "noisy",
        "TRUE when a control pass showed the baseline differing from itself in "
        "this syscall somewhere in the corpus. Never marked significant.",
    ),
    ("per_syscall_stats", "departure_kinds", "Which detectors fired for this syscall."),
    ("per_syscall_stats", "p_adj_bh", "p_value after Benjamini-Hochberg FDR."),
    ("per_syscall_stats", "significant_5pct", "p_adj_bh < 0.05 AND not noisy."),
    # -- noise_floor ------------------------------------------------------
    ("noise_floor", "config", "Compiler-flag configuration."),
    ("noise_floor", "syscall", "The syscall."),
    (
        "noise_floor",
        "n_files_differ_rerun",
        "Files where two trace passes of the SAME binary gave different counts. "
        "Pure run-to-run nondeterminism.",
    ),
    ("noise_floor", "n_files_compared_rerun", "Files the rerun passes covered."),
    (
        "noise_floor",
        "n_files_differ_rebuild",
        "Files where two BUILDS of the same source gave different counts. Build "
        "nondeterminism in the baseline itself.",
    ),
    ("noise_floor", "n_files_compared_rebuild", "Files the extra builds covered."),
    (
        "noise_floor",
        "n_builds",
        "Independent compilations of the baseline behind these numbers "
        "(controls.builds). 2 detects build variance; 3+ says whether it "
        "reproduces.",
    ),
    (
        "noise_floor",
        "n_passes",
        "Independent trace sessions per build (controls.passes). Differences "
        "within a build can only be run noise.",
    ),
    (
        "noise_floor",
        "class",
        "run_noise, build_noise, or stable, from the two counts above.",
    ),
]

#: Prose that does not belong to any one column.
NOTES: list[str] = [
    "MEANS, NOT COUNTS. baseline_mean / variant_mean average over trace.reps "
    "traces of the same binary. A value like 1.60 is an average (e.g. 4,1,1,1,1 "
    "over 5 reps), not a fractional syscall; baseline_sd / variant_sd tell you "
    "how much it moved.",
    "A departure is always a STABLE fact: present in every rep of the variant "
    "and absent in every rep of the baseline. Facts that flicker are noise by "
    "definition and are dropped before any comparison.",
    "The set detector SUBSUMES the others. Where a departure is fully explained "
    "by 'this syscall type is new or gone', the count, argument and sequence "
    "detectors stay quiet so one behavioral change yields one row, not four. "
    "This is why n_arg can be 0 while argument values still differ.",
    "ARGUMENTS ARE ONLY RECORDED for syscalls listed in trace.arg_syscalls, and "
    "only the first quoted string, normalised. arg_status says which case a "
    "blank cell is. Widening that list changes the acquisition fingerprint and "
    "requires `acquire --reset`.",
    "NOISE CLASSES ARE CORPUS-WIDE; the count detector's floor is PER FILE. A "
    "syscall can therefore be noisy=TRUE in per_syscall_stats and still produce "
    "count departures: the corpus-wide class says the baseline wobbled "
    "somewhere, the per-file floor says it did not wobble HERE. Read "
    "baseline_selfdiff on the row and selfdiff_verdict on the taxonomy row.",
    JS_CAVEAT,
    "Benignity is not statically decidable: a one-syscall payload and a linker "
    "artifact look identical to any automatic rule. The classes below carry "
    "different priors; the verdict is yours.",
    "conditional: check layout_best_rho. A strong correlation means membership "
    "tracks link layout, not program behavior -- benign.",
    "program_dependent: the delta scales with program size. This is the only "
    "class in which a payload doing per-item work can hide.",
]


def glossary_frame() -> pd.DataFrame:
    """The ``readme`` sheet: column glossary, then notes, then class priors."""
    rows = [{"sheet": s, "column": c, "meaning": m} for s, c, m in GLOSSARY]
    rows += [{"sheet": "(note)", "column": "", "meaning": n} for n in NOTES]
    rows += [
        {"sheet": "(class)", "column": c, "meaning": CLASS_PRIOR[c]}
        for c in CLASS_ORDER
    ]
    return pd.DataFrame(rows, columns=["sheet", "column", "meaning"])


# ---------------------------------------------------------------------------
# Writers
# ---------------------------------------------------------------------------


def _write(xl, df: pd.DataFrame, sheet: str) -> None:
    if df is None or df.empty:
        pd.DataFrame({"note": ["no data"]}).to_excel(xl, sheet_name=sheet, index=False)
    else:
        df.to_excel(xl, sheet_name=sheet, index=False)


def build_sheets(
    per_file: pd.DataFrame,
    departures: pd.DataFrame,
    taxonomy: pd.DataFrame,
    significance: pd.DataFrame,
    noise: pd.DataFrame,
) -> dict[str, pd.DataFrame]:
    """Analysis frames -> the exact frames written to each sheet.

    Split out from :func:`write_behavior` so the glossary-completeness test can
    check the sheets without going through openpyxl.
    """
    return {
        "readme": glossary_frame(),
        "per_file": _project(per_file, PER_FILE_SHEET_COLUMNS),
        "departures": project_departures(departures),
        "taxonomy": _project(taxonomy, TAXONOMY_SHEET_COLUMNS),
        # Deliberately unprojected: this sheet is the corpus-wide statistical
        # view and is consumed as-is.
        "per_syscall_stats": significance,
        "noise_floor": noise,
    }


def write_behavior(
    out_dir: Path,
    per_file: pd.DataFrame,
    departures: pd.DataFrame,
    taxonomy: pd.DataFrame,
    significance: pd.DataFrame,
    noise: pd.DataFrame,
) -> Path:
    """Write the single behavioral workbook."""
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / WORKBOOK
    sheets = build_sheets(per_file, departures, taxonomy, significance, noise)
    with pd.ExcelWriter(path, engine="openpyxl") as xl:
        for name in SHEETS:
            _write(xl, sheets[name], name)
    return path


def write_benchmark(out_dir: Path, geo: pd.DataFrame, fc: pd.DataFrame) -> Path:
    out_dir.mkdir(parents=True, exist_ok=True)
    path = out_dir / "benchmark.xlsx"
    with pd.ExcelWriter(path, engine="openpyxl") as xl:
        _write(xl, geo, "geomeans")
        _write(xl, fc, "foldchange_vs_baseline")
    return path


# ---------------------------------------------------------------------------
# Terminal
# ---------------------------------------------------------------------------

RULE = "=" * 78


def print_equivalence_report(
    per_file: pd.DataFrame, taxonomy: pd.DataFrame, settings: Settings
) -> None:
    print("\n" + RULE)
    print("BEHAVIORAL EQUIVALENCE")
    print(RULE)
    if per_file.empty:
        print("  no data")
        return

    for config in settings.config_names:
        cf = per_file[per_file["config"] == config]
        if cf.empty:
            continue
        print(f"\n[{config}]")
        for v, g in cf.groupby("variant"):
            n = len(g)
            eq = int(g["equivalent"].sum())
            print(
                f"  {settings.label(str(v)):22s} files={n:4d}  equivalent={eq:4d}  "
                f"differing={n - eq:4d}"
            )
            for kind, col in (
                ("set", "n_set"),
                ("argument", "n_arg"),
                ("sequence", "n_seq"),
                ("count", "n_count"),
            ):
                hit = int((g[col] > 0).sum())
                if hit:
                    print(f"      {kind:9s}: {hit:4d} files")
            bad = int((~g["seq_oracle_reliable"]).sum())
            if bad:
                print(
                    f"      [note] sequence oracle unreliable on {bad} files "
                    "(trace passes of one build disagreed)"
                )

    if taxonomy.empty:
        return

    print("\n" + RULE)
    print("DEPARTURE TAXONOMY  (benignity is a human call; these are the priors)")
    print(RULE)
    for config in settings.config_names:
        cf = taxonomy[taxonomy["config"] == config]
        if cf.empty:
            continue
        print(f"\n[{config}]")
        for v, g in cf.groupby("variant"):
            print(f"  {settings.label(str(v))} vs {settings.label(settings.baseline)}")
            for klass in CLASS_ORDER:
                gg = g[g["class"] == klass]
                if gg.empty:
                    continue
                print(f"    {klass:20s} ({CLASS_PRIOR[klass]})")
                for r in gg.itertuples(index=False):
                    mag = (
                        f"delta={r.magnitude_values}"
                        if r.magnitude_fixed
                        else f"delta varies (median {r.magnitude_median:+.1f})"
                    )
                    if r.layout_best_rho is not None:
                        layout = f" layout_rho={r.layout_best_rho:+.2f}"
                    elif r.layout_status and r.layout_status != "ok":
                        layout = f" layout: {r.layout_status}"
                    else:
                        layout = ""
                    print(
                        f"      - {r.syscall:16s} [{r.kind:9s}] "
                        f"{r.n_files_affected:4d}/{r.n_files_corpus} files  {mag}{layout}"
                    )
            pd_n = int((g["class"] == "program_dependent").sum())
            if pd_n == 0:
                print("    -> no program_dependent departures")


def print_bench_report(geo: pd.DataFrame, fc: pd.DataFrame, settings: Settings) -> None:
    print("\n" + RULE)
    print("BENCHMARK  (per-file geometric means; fold-change vs baseline)")
    print(RULE)
    if geo.empty:
        print("  no data")
        return
    for config in settings.config_names:
        g = geo[geo["config"] == config]
        if g.empty:
            continue
        print(f"\n[{config}]")
        print(
            f"  {'compiler':22s} {'size(KiB)':>11s} {'compile(s)':>11s} {'exec(s)':>9s} "
            f"{'size/A':>8s} {'comp/A':>8s} {'exec/A':>8s}"
        )
        for key in settings.compilers_for(config):
            row = g[g["compiler"] == key]
            if row.empty:
                continue
            r = row.iloc[0]
            if key == settings.baseline:
                ratios = ("1.00", "1.00", "1.00")
            else:
                f = fc[(fc["config"] == config) & (fc["compiler"] == key)]
                if f.empty:
                    ratios = ("n/a", "n/a", "n/a")
                else:
                    fr = f.iloc[0]
                    ratios = (
                        f"{fr['size_geomean']:.2f}",
                        f"{fr['compile_geomean']:.2f}",
                        f"{fr['exec_geomean']:.2f}",
                    )
            print(
                f"  {settings.label(key):22s} {r['size'] / 1024:11.1f} "
                f"{r['compile']:11.3f} {r['exec']:9.4f} "
                f"{ratios[0]:>8s} {ratios[1]:>8s} {ratios[2]:>8s}"
            )
