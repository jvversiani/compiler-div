"""Noise floors from the nested control grid.

Two floors, and the distinction between them is the point:

**Run noise** -- differences between trace *passes of the same build*. The
binary is byte-identical, so anything that moves is execution nondeterminism:
ASLR, scheduler, futex ordering, DNS retry. A floor on *execution*.

**Build noise** -- differences between *builds* of the same source with the same
compiler. Link layout, embedded paths, timestamps. A finding like a variant's
extra ``read`` is invisible to a rerun -- the same binary reads the same bytes
about itself -- so only a rebuild can say whether the baseline's own builds vary
that way. Without it, "stable across reruns" rules out run noise and nothing
else, and the claim "this is a property of the variant's toolchain" is not
supported.

Because the passes are **nested inside** the builds, both classes are measured
directly. The flat predecessor (one rerun, one rebuild) had to infer the build
class by subtracting the run class from a single recompilation: an estimate from
n=2, which detects *a* difference but says nothing about whether it reproduces.
With three builds the per-file floor becomes an actual sample of what the
baseline does to itself, which is what :func:`..detectors.detect_count`
consumes.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass, field
from itertools import combinations

import pandas as pd

from ..config import Settings


@dataclass
class NoiseFloor:
    """Derived noise classes and per-file floors for one config.

    Attributes
    ----------
    run_noisy:
        Syscalls differing between passes of the same build in >=
        ``auto_noisy_min_files`` files. Pure execution nondeterminism.
    build_noisy:
        Syscalls differing between builds (but not within one) in >= the same
        threshold. Build nondeterminism in the baseline compiler itself.
    run_diff_files / build_diff_files:
        ``syscall -> number of files differing``, for the report.
    n_files_rerun / n_files_rebuild:
        How many files each control actually covered.
    n_builds / n_passes:
        The control grid that produced all of the above. Reported so a reader
        can tell a clean floor from an under-sampled one.
    unstable_seq_files:
        Files where two passes of the same build disagree on their stable
        n-gram set -- the sequence oracle is unreliable there and its findings
        are marked accordingly.
    rebuild_delta:
        ``(file, syscall) -> max |delta|`` over every pair of builds. This is
        the per-file build floor, and it is the important one.

        A corpus-wide "is this syscall ever noisy" verdict is a blunt
        instrument: on 672 programs, three jittery outliers would disqualify a
        syscall in the 669 where it is rock solid. Worse, it is exactly the
        wrong shape for the finding at hand -- if the baseline's own rebuilds
        shift ``read`` in 6 files and a variant shifts it in 59, the global
        filter deletes all 59 because of the 6.

        Per file, the question is answerable properly: *did the baseline do this
        to itself, on THIS program?* Where it did, the variant's delta is
        suppressed. Where it did not, the delta stands.

        Taking the **max** over build pairs rather than a single A-vs-rebuild
        delta is what more builds buy: each additional build can only reveal
        more of the baseline's own variance, never less, so the floor rises
        monotonically toward the truth and the departures that survive it are
        better licensed.
    """

    run_noisy: frozenset[str] = frozenset()
    build_noisy: frozenset[str] = frozenset()
    run_diff_files: dict[str, int] = field(default_factory=dict)
    build_diff_files: dict[str, int] = field(default_factory=dict)
    n_files_rerun: int = 0
    n_files_rebuild: int = 0
    n_builds: int = 1
    n_passes: int = 1
    unstable_seq_files: frozenset[str] = frozenset()
    rebuild_delta: dict[tuple[str, str], float] = field(default_factory=dict)

    @property
    def all_noisy(self) -> frozenset[str]:
        """Everything excluded from significance: run noise + build noise."""
        return self.run_noisy | self.build_noisy

    def classify_syscall(self, syscall: str) -> str:
        if syscall in self.run_noisy:
            return "run_noise"
        if syscall in self.build_noisy:
            return "build_noise"
        return "stable"

    def build_floor(self, file: str, syscall: str) -> float:
        """Per-file build-variance floor. 0.0 when the baseline was stable."""
        return self.rebuild_delta.get((str(file), str(syscall)), 0.0)


def _count_diff_files(
    counts: pd.DataFrame, config: str, left: str, right: str, tol: float
) -> tuple[dict[str, int], int]:
    """Files where each syscall's mean count differs between two tags."""
    if counts is None or counts.empty or "config" not in counts.columns:
        return {}, 0
    sub = counts[counts["config"] == config]
    a = sub[sub["compiler"] == left].set_index(["file", "syscall"])["mean_count"]
    b = sub[sub["compiler"] == right].set_index(["file", "syscall"])["mean_count"]
    if a.empty or b.empty:
        return {}, 0

    joined = pd.concat([a.rename("a"), b.rename("b")], axis=1).fillna(0.0)
    files_a = set(a.index.get_level_values("file"))
    files_b = set(b.index.get_level_values("file"))
    common = files_a & files_b
    joined = joined[joined.index.get_level_values("file").isin(common)]

    differ = joined[(joined["a"] - joined["b"]).abs() > tol]
    out = differ.reset_index().groupby("syscall")["file"].nunique().to_dict()
    return {str(k): int(v) for k, v in out.items()}, len(common)


def _unstable_sequence_files(
    ngrams: pd.DataFrame, config: str, pass_tags: list[str]
) -> frozenset[str]:
    """Files where two passes of the same build disagree on their stable n-grams.

    On an identical binary the stable n-gram sets should match. Where they do
    not, per-thread ordering is not reproducible for that program and any
    sequence finding against a variant is unreliable.

    Every pass of the build is checked, not just the first two: a program whose
    ordering wobbles once in three sessions is exactly as unreliable as one that
    wobbles every time, and the extra passes are there to catch it.
    """
    if ngrams is None or ngrams.empty or "config" not in ngrams.columns:
        return frozenset()
    sub = ngrams[ngrams["config"] == config]
    if sub.empty or len(pass_tags) < 2:
        return frozenset()

    maps: list[dict[str, set]] = []
    for tag in pass_tags:
        g = sub[(sub["compiler"] == tag) & sub["stable"]]
        if g.empty:
            continue
        maps.append({str(f): set(gg["ngram"]) for f, gg in g.groupby("file")})
    if len(maps) < 2:
        return frozenset()

    bad: set[str] = set()
    for left, right in combinations(maps, 2):
        bad |= {f for f in set(left) & set(right) if left[f] != right[f]}
    return frozenset(bad)


def _worst_over_pairs(
    counts: pd.DataFrame,
    config: str,
    pairs: list[tuple[str, str]],
    tol: float,
) -> tuple[dict[str, int], int, dict[tuple[str, str], float]]:
    """Aggregate several tag-vs-tag comparisons into one verdict.

    Returns ``(files differing per syscall, files compared, worst per-file
    delta)``. A file counts against a syscall if **any** pair disagrees there,
    and the per-file delta is the **largest** any pair produced: both directions
    are "how badly does the baseline disagree with itself, at worst", which is
    the only honest way to read a floor.
    """
    diff_files: dict[str, set[str]] = defaultdict(set)
    worst: dict[tuple[str, str], float] = {}
    n_compared = 0

    for left, right in pairs:
        per_syscall, n = _count_diff_files(counts, config, left, right, tol)
        n_compared = max(n_compared, n)
        for syscall, deltas in _deltas_by_file(
            counts, config, left, right, tol
        ).items():
            for f, d in deltas.items():
                diff_files[syscall].add(f)
                key = (f, syscall)
                worst[key] = max(worst.get(key, 0.0), abs(d))
        # `per_syscall` is only used for its keys when a syscall differs in a
        # file the delta pass filtered out; the union above is authoritative.
        for syscall in per_syscall:
            diff_files.setdefault(syscall, set())

    return {s: len(fs) for s, fs in diff_files.items()}, n_compared, worst


def derive_noise(frames, settings: Settings, config: str) -> NoiseFloor:
    """Build the noise floor for one config from the nested control grid."""
    counts = frames.counts
    tol = settings.detect.count_tol
    min_files = settings.detect.auto_noisy_min_files
    ctl = settings.controls

    # Run noise: every pair of passes WITHIN a build, over all builds. The
    # binary is fixed inside each pair, so nothing here can be build variance.
    run_pairs = [
        pair
        for b in range(ctl.builds)
        for pair in combinations(settings.pass_tags(b), 2)
    ]
    run_diff, n_rerun, _ = _worst_over_pairs(counts, config, run_pairs, tol)
    run_noisy = frozenset(s for s, n in run_diff.items() if n >= min_files)

    # Build noise: every pair of BUILDS, compared at their primary pass. Each
    # additional build can only expose more of the baseline's own variance.
    build_pairs = list(combinations(settings.build_tags(), 2))
    build_diff, n_rebuild, rebuild_delta = _worst_over_pairs(
        counts, config, build_pairs, tol
    )
    # Build noise is only the *additional* class: a syscall already classed as
    # run noise would show up here too, and attributing it to the build would
    # be wrong.
    build_noisy = frozenset(
        s for s, n in build_diff.items() if n >= min_files and s not in run_noisy
    )

    unstable = _unstable_sequence_files(frames.ngrams, config, settings.pass_tags(0))

    return NoiseFloor(
        run_noisy=run_noisy,
        build_noisy=build_noisy,
        run_diff_files=run_diff,
        build_diff_files=build_diff,
        n_files_rerun=n_rerun,
        n_files_rebuild=n_rebuild,
        n_builds=ctl.builds,
        n_passes=ctl.passes,
        unstable_seq_files=unstable,
        rebuild_delta=rebuild_delta,
    )


def noise_table(floor: NoiseFloor, config: str) -> pd.DataFrame:
    """Long-format table of the noise floor, for the report workbook."""
    rows = []
    for s in sorted(set(floor.run_diff_files) | set(floor.build_diff_files)):
        rows.append(
            {
                "config": config,
                "syscall": s,
                "n_files_differ_rerun": floor.run_diff_files.get(s, 0),
                "n_files_compared_rerun": floor.n_files_rerun,
                "n_files_differ_rebuild": floor.build_diff_files.get(s, 0),
                "n_files_compared_rebuild": floor.n_files_rebuild,
                "n_builds": floor.n_builds,
                "n_passes": floor.n_passes,
                "class": floor.classify_syscall(s),
            }
        )
    return pd.DataFrame(
        rows,
        columns=[
            "config",
            "syscall",
            "n_files_differ_rerun",
            "n_files_compared_rerun",
            "n_files_differ_rebuild",
            "n_files_compared_rebuild",
            "n_builds",
            "n_passes",
            "class",
        ],
    )


def _deltas_by_file(
    counts: pd.DataFrame, config: str, left: str, right: str, tol: float
) -> dict[str, dict[str, float]]:
    """``syscall -> {file: delta}`` for files where the two tags differ."""
    if counts is None or counts.empty or "config" not in counts.columns:
        return {}
    sub = counts[counts["config"] == config]
    a = sub[sub["compiler"] == left].set_index(["file", "syscall"])["mean_count"]
    b = sub[sub["compiler"] == right].set_index(["file", "syscall"])["mean_count"]
    if a.empty or b.empty:
        return {}

    joined = pd.concat([a.rename("a"), b.rename("b")], axis=1).fillna(0.0)
    files_a = set(a.index.get_level_values("file"))
    files_b = set(b.index.get_level_values("file"))
    joined = joined[joined.index.get_level_values("file").isin(files_a & files_b)]

    joined = joined.assign(delta=joined["b"] - joined["a"])
    differ = joined[joined["delta"].abs() > tol].reset_index()

    out: dict[str, dict[str, float]] = defaultdict(dict)
    for r in differ.itertuples(index=False):
        out[str(r.syscall)][str(r.file)] = float(r.delta)
    return dict(out)


def _fmt_deltas(deltas: dict[str, float], limit: int = 6) -> str:
    """Distinct delta values, e.g. ``+1`` or ``-1, +1``."""
    vals = sorted({round(v, 3) for v in deltas.values()})
    shown = ", ".join(f"{v:+g}" for v in vals[:limit])
    return shown + ("..." if len(vals) > limit else "")


def build_vs_variant(
    frames, settings: Settings, config: str, tol: float | None = None
) -> pd.DataFrame:
    """Compare the baseline's own build variance against each variant's departures.

    The question this answers: a variant's ``read(+1)`` affects 59 files, and
    the rebuild controls show ``read`` differing in 6 files of the baseline
    against itself. Are those the same phenomenon?

    * **Disjoint files, different delta direction** -> different mechanisms; the
      variant finding stands on its own.
    * **Variant files contain the rebuild files, same fixed delta** -> the same
      mechanism, which the variant merely triggers more often. That is a much
      weaker claim and the paper must say so.

    Columns
    -------
    rebuild_n_files / rebuild_deltas / rebuild_files
        What the baseline does to itself across two builds of the same source.
    variant_n_files / variant_deltas
        What the variant does relative to the baseline.
    n_overlap / overlap_files
        Files appearing in both sets.
    same_delta_set
        Whether the two produce the same distinct delta values.
    verdict
        A one-line reading. Advisory, not a ruling.
    """
    cols = [
        "config",
        "syscall",
        "variant",
        "variant_label",
        "rebuild_n_files",
        "rebuild_deltas",
        "variant_n_files",
        "variant_deltas",
        "n_overlap",
        "same_delta_set",
        "verdict",
        "overlap_files",
        "rebuild_files",
    ]
    tol = settings.detect.count_tol if tol is None else tol
    base = settings.baseline

    # Pool every build pair: the baseline's self-difference is whatever ANY two
    # of its builds disagreed on, not just what the first rebuild happened to
    # show. With three builds this is a real sample of its own variance.
    rebuild: dict[str, dict[str, float]] = defaultdict(dict)
    for left, right in combinations(settings.build_tags(), 2):
        for syscall, deltas in _deltas_by_file(
            frames.counts, config, left, right, tol
        ).items():
            for f, d in deltas.items():
                prev = rebuild[syscall].get(f)
                if prev is None or abs(d) > abs(prev):
                    rebuild[syscall][f] = d
    if not rebuild:
        return pd.DataFrame(columns=cols)

    rows = []
    for variant in settings.variant_keys:
        var = _deltas_by_file(frames.counts, config, base, variant, tol)
        # Only syscalls the baseline is unstable in are worth comparing: those
        # are the ones whose variant findings are in question.
        for syscall in sorted(rebuild):
            rb = rebuild[syscall]
            vr = var.get(syscall, {})
            overlap = sorted(set(rb) & set(vr))

            rb_vals = {round(v, 3) for v in rb.values()}
            vr_vals = {round(v, 3) for v in vr.values()}
            same_delta = bool(vr_vals) and rb_vals == vr_vals

            if not vr:
                verdict = "variant does not differ here; baseline instability only"
            elif not overlap and not same_delta:
                verdict = "disjoint files and different deltas: different mechanisms"
            elif not overlap:
                verdict = "same deltas but disjoint files: likely different triggers"
            elif same_delta and set(rb) <= set(vr):
                verdict = (
                    "SAME mechanism: baseline's affected files are a subset of the "
                    "variant's, with identical deltas"
                )
            else:
                verdict = "partial overlap: inspect the file lists"

            rows.append(
                {
                    "config": config,
                    "syscall": syscall,
                    "variant": variant,
                    "variant_label": settings.label(variant),
                    "rebuild_n_files": len(rb),
                    "rebuild_deltas": _fmt_deltas(rb),
                    "variant_n_files": len(vr),
                    "variant_deltas": _fmt_deltas(vr) if vr else "(none)",
                    "n_overlap": len(overlap),
                    "same_delta_set": same_delta,
                    "verdict": verdict,
                    "overlap_files": "; ".join(overlap),
                    "rebuild_files": "; ".join(sorted(rb)),
                }
            )
    return pd.DataFrame(rows, columns=cols)
