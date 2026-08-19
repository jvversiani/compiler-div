"""The five departure detectors.

Each answers a different question about "does B behave like A on this file", and
each carries its own noise floor derived from the control passes.

============  ======================================  ===================
detector      question                                noise floor
============  ======================================  ===================
set           does B make a syscall A never makes?    control set difference
count         does B make N more of some syscall?     control count spread
argument      does B pass a value A never passes?     control argument sets
sequence      does B order calls in a way A never     control stable n-grams
              does?
instability   is B nondeterministic where A is not?   baseline determinism,
                                                      per (file, syscall)
============  ======================================  ===================

The first four report a **stable** fact: present in every rep of B, absent in
every rep of A. Facts that flicker are noise by definition, and the intersection
across reps drops them before any comparison happens.

``instability`` is the deliberate exception, and it exists because that
intersection is lossy in a way the other four cannot see. A variant whose count
for one syscall wanders over ``1,3,4,2,5`` while the baseline sits at a constant
``2`` differs from the baseline in a way no stability-based detector can state:
the count detector's floor absorbs it (the variant's own spread *is* the floor),
and the set detector sees the syscall present on both sides. Where flicker in
the variant coincides with determinism in the baseline, the flicker is not
noise -- it is the finding, and it is attributable, because the baseline proved
on this very program that the measurement can be reproducible.

The asymmetry is intentional. The mirror case -- baseline flickers, variant is
steady -- is *not* reported here. It says something about the instrument rather
than about the variant, and it rests on far weaker evidence: the baseline gets
``builds x passes x reps`` traces to reveal its instability while the variant
gets ``trace.reps`` to prove its steadiness. Concluding "the variant is
deterministic" from that sample would be claiming much more than it supports.

Note on the sequence detector: it is a *set difference over stable n-grams*, not
a divergence between distributions. Distribution metrics (JS included) normalise
by total volume, which makes a fixed-size difference score high on a small
program and ~0 on a large one -- they end up ranking by program size. The set
difference has no denominator and therefore no size bias.
"""

from __future__ import annotations

import pandas as pd

from ..config import Settings
from ..store.aggregate import observed_tags

# Departure kinds.
KIND_SET = "set"
KIND_COUNT = "count"
KIND_ARG = "argument"
KIND_SEQ = "sequence"
KIND_INSTAB = "instability"

#: Separator the store uses between the syscalls of an n-gram.
_NGRAM_SEP = "\u2192"

DEPARTURE_COLUMNS = [
    "config",
    "pair",
    "variant",
    "file",
    "kind",
    "syscall",
    "detail",
    "magnitude",
    "baseline_value",
    "variant_value",
]


def _empty() -> pd.DataFrame:
    return pd.DataFrame(columns=DEPARTURE_COLUMNS)


def _stable_sets(df: pd.DataFrame, value_col: str) -> dict:
    """``(file) -> set(values)`` restricted to stable rows, for one compiler."""
    if df.empty:
        return {}
    sub = df[df["stable"]]
    out: dict[str, set] = {}
    for f, g in sub.groupby("file"):
        out[str(f)] = set(g[value_col])
    return out


def _observed_sets(df: pd.DataFrame, value_col: str) -> dict:
    """``(file) -> set(values)`` over **every** rep, stable or not.

    The counterpart to :func:`_stable_sets`, and the two are not
    interchangeable. Claiming a compiler *does* something needs stability;
    claiming it does **not** needs total absence. Using the stable set for both
    makes "flickered once in five reps" indistinguishable from "never happened".
    """
    if df.empty:
        return {}
    out: dict[str, set] = {}
    for f, g in df.groupby("file"):
        out[str(f)] = set(g[value_col])
    return out


# ---------------------------------------------------------------------------
# Subsumption: one behavioural change, one departure
# ---------------------------------------------------------------------------
#
# The four detectors are independent by construction, so a single behavioural
# change trips as many of them as it structurally can. A syscall type the
# baseline never makes necessarily also has a positive count delta, necessarily
# also sits inside n-grams the baseline never produced, and -- if it is
# argument-carrying -- necessarily passes values the baseline never passed. The
# same event is then reported three or four times, once per detector, which
# inflates the departure counts and makes a single finding look like a cluster.
#
# The set detector is the sharpest and the most specific of the four (see the
# module docstring), so it wins: where a departure is fully accounted for by
# "this syscall type is new/gone", the subordinate detectors stay quiet and the
# set row is the sole record. Nothing is suppressed that the set row does not
# already assert -- the reduction is in duplication, not in evidence.


def _syscalls_by_file(rows: pd.DataFrame) -> dict[str, frozenset[str]]:
    """``file -> syscalls a sharper detector has already reported there``.

    Used twice. For the set detector, both directions qualify: a syscall the
    *baseline* makes and the variant does not produces a negative count delta
    for exactly the same reason, and that duplicate is no more informative than
    the positive one. For the count detector it marks the ``(file, syscall)``
    pairs whose magnitude cleared the floor, which subsumes the weaker
    observation that the magnitude also wobbled.
    """
    if rows.empty:
        return {}
    return {str(f): frozenset(g["syscall"]) for f, g in rows.groupby("file")}


def _merge_explained(*maps: dict[str, frozenset[str]]) -> dict[str, frozenset[str]]:
    """Union several ``file -> syscalls`` maps, keeping every file in any of them."""
    out: dict[str, frozenset[str]] = {}
    for m in maps:
        for f, scs in m.items():
            out[f] = out.get(f, frozenset()) | scs
    return out


def _drop_subsumed(
    df: pd.DataFrame, explained: dict[str, frozenset[str]]
) -> pd.DataFrame:
    """Drop per-syscall rows already accounted for by a set departure.

    Applies to the count and argument detectors, whose rows are keyed by
    ``(file, syscall)`` and so map onto a set departure exactly. The sequence
    detector needs finer treatment -- its rows are per file -- and filters its
    own n-grams instead.
    """
    if df.empty or not explained:
        return df
    keep = [
        sc not in explained.get(str(f), frozenset())
        for f, sc in zip(df["file"], df["syscall"])
    ]
    return df[pd.Series(keep, index=df.index)].reset_index(drop=True)


# ---------------------------------------------------------------------------
# 1. Set detector
# ---------------------------------------------------------------------------


def detect_set(
    syscall_sets: pd.DataFrame,
    config: str,
    baseline: str,
    variant: str,
    ignore: frozenset[str],
) -> pd.DataFrame:
    """Syscall types stably present under ``variant`` and never once emitted by
    ``baseline`` (and vice versa).

    This is requirement 1: baseline has no ``write``, variant has ``write``.
    It is the sharpest of the five -- a syscall type appearing in every rep of B
    and no rep of A is very hard to explain as noise.

    The two directions use **different** notions of the other side, on purpose.
    A presence claim needs stability, so the side being credited with the
    syscall is read from its stable set. An absence claim needs the syscall to
    be missing from every rep, so the side being credited with *not* making it
    is read from its observed set. Comparing two stable sets instead -- which is
    what this did originally -- turns "flickered in four reps of five" into
    "absent", and reports a variant that makes the call more often than the
    baseline as having lost it. Those cases are real findings, but they belong
    to :func:`detect_instability`, which can state them correctly.
    """
    sub = syscall_sets[syscall_sets["config"] == config]
    a = _stable_sets(sub[sub["compiler"] == baseline], "syscall")
    b = _stable_sets(sub[sub["compiler"] == variant], "syscall")
    a_seen = _observed_sets(sub[sub["compiler"] == baseline], "syscall")
    b_seen = _observed_sets(sub[sub["compiler"] == variant], "syscall")

    rows = []
    for f in sorted(set(a_seen) & set(b_seen)):
        only_b = (b.get(f, set()) - a_seen[f]) - ignore
        only_a = (a.get(f, set()) - b_seen[f]) - ignore
        for sc in sorted(only_b):
            rows.append(
                {
                    "config": config,
                    "pair": f"{variant}v{baseline}",
                    "variant": variant,
                    "file": f,
                    "kind": KIND_SET,
                    "syscall": sc,
                    "detail": "new syscall type",
                    "magnitude": 1.0,
                    "baseline_value": "absent",
                    "variant_value": "present",
                }
            )
        for sc in sorted(only_a):
            rows.append(
                {
                    "config": config,
                    "pair": f"{variant}v{baseline}",
                    "variant": variant,
                    "file": f,
                    "kind": KIND_SET,
                    "syscall": sc,
                    "detail": "missing syscall type",
                    "magnitude": -1.0,
                    "baseline_value": "present",
                    "variant_value": "absent",
                }
            )
    return pd.DataFrame(rows, columns=DEPARTURE_COLUMNS) if rows else _empty()


# ---------------------------------------------------------------------------
# 2. Count detector
# ---------------------------------------------------------------------------


def detect_count(
    counts: pd.DataFrame,
    config: str,
    baseline: str,
    variant: str,
    ignore: frozenset[str],
    noisy: frozenset[str],
    tol: float,
    build_floor: dict[tuple[str, str], float] | None = None,
) -> pd.DataFrame:
    """Per-syscall mean-count deltas exceeding the file's own noise floor.

    The floor is per (file, syscall) and combines three empirical sources:

    * ``std_count`` under either compiler -- run-to-run jitter on this program;
    * ``build_floor`` -- how far the baseline disagreed with *itself* on this
      program across two builds of the same source (the rebuild controls);
    * ``tol`` -- a constant minimum.

    The build term is what makes the floor honest. A corpus-wide "this syscall
    is noisy" verdict would delete a real finding in 669 files because of 3
    outliers; per file, the question is *did the baseline do this to itself,
    HERE?* -- and only where it did is the variant's delta suppressed.
    """
    sub = counts[counts["config"] == config]
    a = sub[sub["compiler"] == baseline].set_index(["file", "syscall"])
    b = sub[sub["compiler"] == variant].set_index(["file", "syscall"])
    if a.empty or b.empty:
        return _empty()

    joined = a[["mean_count", "std_count"]].join(
        b[["mean_count", "std_count"]], how="outer", lsuffix="_a", rsuffix="_b"
    )
    joined = joined.fillna(0.0).reset_index()

    files_a = set(a.index.get_level_values("file"))
    files_b = set(b.index.get_level_values("file"))
    common = files_a & files_b
    joined = joined[joined["file"].isin(common)]
    joined = joined[~joined["syscall"].isin(ignore | noisy)]
    if joined.empty:
        return _empty()

    delta = joined["mean_count_b"] - joined["mean_count_a"]

    floor = joined[["std_count_a", "std_count_b"]].max(axis=1)
    if build_floor:
        bf = pd.Series(
            [
                build_floor.get((str(f), str(s)), 0.0)
                for f, s in zip(joined["file"], joined["syscall"])
            ],
            index=joined.index,
        )
        floor = pd.concat([floor, bf], axis=1).max(axis=1)
    floor = floor.clip(lower=tol)

    hit = delta.abs() > floor
    hits = joined[hit].copy()
    if hits.empty:
        return _empty()

    hits["delta"] = delta[hit]
    rows = []
    for r in hits.itertuples(index=False):
        sign = "+" if r.delta > 0 else "-"
        rows.append(
            {
                "config": config,
                "pair": f"{variant}v{baseline}",
                "variant": variant,
                "file": str(r.file),
                "kind": KIND_COUNT,
                "syscall": r.syscall,
                "detail": f"{r.syscall}({sign}{abs(r.delta):.0f})",
                "magnitude": float(r.delta),
                "baseline_value": f"{r.mean_count_a:.2f}",
                "variant_value": f"{r.mean_count_b:.2f}",
            }
        )
    return pd.DataFrame(rows, columns=DEPARTURE_COLUMNS)


# ---------------------------------------------------------------------------
# 3. Argument detector
# ---------------------------------------------------------------------------


def detect_args(
    args: pd.DataFrame,
    config: str,
    baseline: str,
    variant: str,
    ignore: frozenset[str],
) -> pd.DataFrame:
    """Argument values stably passed by ``variant`` and never by ``baseline``.

    This is requirement 2: A does ``openat("/etc/passwd")``, B does
    ``openat("/lib/x.so")``. Counts are identical, so the count detector is
    blind; the set of normalised argument values is not.

    Only the forward direction is reported -- a value the *variant* passes that
    the baseline never does. The mirror ("a value the baseline passed that the
    variant no longer does") is deliberately **not** emitted: for a substituted
    argument the two are the same event, and reporting both double-counts a
    single change (``write("Bom dia")`` would appear once as new-under-variant
    and again as absent-under-variant). The detector answers "does B pass a
    value A never passes?", which is the tamper-relevant direction.

    Arguments are compared per syscall, as sets. Values that vary across reps
    (a tmp path, an ASLR address the normaliser missed) are unstable and never
    reach this comparison.
    """
    sub = args[args["config"] == config]
    if sub.empty:
        return _empty()
    a = sub[(sub["compiler"] == baseline) & sub["stable"]]
    b = sub[(sub["compiler"] == variant) & sub["stable"]]
    if a.empty or b.empty:
        return _empty()

    a_map: dict[tuple[str, str], set[str]] = {}
    for (f, sc), g in a.groupby(["file", "syscall"]):
        a_map[(str(f), str(sc))] = set(g["arg"])
    b_map: dict[tuple[str, str], set[str]] = {}
    for (f, sc), g in b.groupby(["file", "syscall"]):
        b_map[(str(f), str(sc))] = set(g["arg"])

    files_a = {k[0] for k in a_map}
    files_b = {k[0] for k in b_map}
    common_files = files_a & files_b

    rows = []
    for key in sorted(set(a_map) | set(b_map)):
        f, sc = key
        if f not in common_files or sc in ignore:
            continue
        av = a_map.get(key, set())
        bv = b_map.get(key, set())
        for arg in sorted(bv - av):
            rows.append(
                {
                    "config": config,
                    "pair": f"{variant}v{baseline}",
                    "variant": variant,
                    "file": f,
                    "kind": KIND_ARG,
                    "syscall": sc,
                    "detail": f"{sc}({arg!r}) not seen under baseline",
                    "magnitude": 1.0,
                    "baseline_value": "|".join(sorted(av)[:5]) or "(none)",
                    "variant_value": arg,
                }
            )
    return pd.DataFrame(rows, columns=DEPARTURE_COLUMNS) if rows else _empty()


# ---------------------------------------------------------------------------
# 4. Sequence detector
# ---------------------------------------------------------------------------


def detect_sequence(
    ngrams: pd.DataFrame,
    config: str,
    baseline: str,
    variant: str,
    ignore: frozenset[str],
    max_examples: int = 3,
    explained: dict[str, frozenset[str]] | None = None,
) -> pd.DataFrame:
    """n-grams stably produced by ``variant`` and never stably by ``baseline``.

    This is requirement 3, handled the only way it can honestly be handled:
    every n-gram must survive the intersection across all reps of *both*
    compilers before it is compared. For a single-threaded program the
    intersection is the whole set and the detector is sharp. For a threaded one
    the scheduler removes most n-grams from the intersection, and the detector
    quietly loses power instead of firing on interleavings.

    n-grams containing an ignored syscall (allocator / scheduler churn) are
    dropped: a single ``futex`` in the middle of a window would otherwise make
    the whole window unstable-but-not-dropped.

    ``explained`` maps a file to the syscalls a set departure already reported
    there. An n-gram containing one of them is new only *because* that syscall
    is new, so it is dropped for the same reason ignored syscalls are -- at
    n-gram granularity, not row granularity, so a file whose reordering is
    genuine still reports the part that a set departure does not explain. A row
    is emitted only if something survives.
    """
    sub = ngrams[ngrams["config"] == config]
    if sub.empty:
        return _empty()
    a = sub[(sub["compiler"] == baseline) & sub["stable"]]
    b = sub[(sub["compiler"] == variant) & sub["stable"]]
    if a.empty or b.empty:
        return _empty()

    def _drop_ignored(df: pd.DataFrame) -> pd.DataFrame:
        if not ignore:
            return df
        mask = df["ngram"].map(
            lambda g: not any(part in ignore for part in g.split(_NGRAM_SEP))
        )
        return df[mask]

    a = _drop_ignored(a)
    b = _drop_ignored(b)

    a_map: dict[str, set[str]] = {str(f): set(g["ngram"]) for f, g in a.groupby("file")}
    b_map: dict[str, set[str]] = {str(f): set(g["ngram"]) for f, g in b.groupby("file")}
    threads = (
        sub.groupby("file")["n_threads"].max().to_dict() if "n_threads" in sub else {}
    )

    by_set = explained or {}
    rows = []
    for f in sorted(set(a_map) & set(b_map)):
        new = b_map[f] - a_map[f]
        ex = by_set.get(f)
        if ex:
            new = {g for g in new if not ex & set(g.split(_NGRAM_SEP))}
        if not new:
            continue
        examples = sorted(new)[:max_examples]
        nt = int(threads.get(f, 1) or 1)
        suffix = f" [{nt} threads: sequence oracle is weak]" if nt > 1 else ""
        rows.append(
            {
                "config": config,
                "pair": f"{variant}v{baseline}",
                "variant": variant,
                "file": f,
                "kind": KIND_SEQ,
                "syscall": "|".join(sorted({g.split(_NGRAM_SEP)[0] for g in examples})),
                "detail": f"{len(new)} new stable n-gram(s){suffix}: "
                + "; ".join(examples),
                "magnitude": float(len(new)),
                "baseline_value": f"{len(a_map[f])} stable n-grams",
                "variant_value": f"{len(b_map[f])} stable n-grams",
            }
        )
    return pd.DataFrame(rows, columns=DEPARTURE_COLUMNS) if rows else _empty()


# ---------------------------------------------------------------------------
# 5. Instability detector
# ---------------------------------------------------------------------------


class _BaselineFacts:
    """What the baseline does *identically* in every rep of every control group.

    ``constant`` holds a ``(file, syscall)`` only if the baseline emitted it the
    same number of times in every rep of every group -- so the bar is
    "reproducible across reruns *and* across rebuilds", the strongest statement
    the control grid can make. Everything weaker is left out, which means a
    variant is never accused of introducing nondeterminism the baseline also
    has.

    ``emitted`` is every pair seen at least once, and it is what separates the
    two ways of being deterministic. A pair in ``emitted`` but not in
    ``constant`` is a baseline that wobbles: nothing there is attributable to a
    variant. A pair in neither, on a file the baseline traced, is a baseline
    that deterministically never makes the call.

    ``files`` and ``traces`` exist to read absence correctly: a syscall missing
    from a file the baseline never traced is not evidence of anything, and
    "absent" means little without knowing how many traces looked for it.
    """

    __slots__ = ("constant", "emitted", "files", "traces")

    def __init__(
        self,
        constant: dict[tuple[str, str], float],
        emitted: set[tuple[str, str]],
        files: set[str],
        traces: dict[str, int],
    ):
        self.constant = constant
        self.emitted = emitted
        self.files = files
        self.traces = traces


def _baseline_facts(
    sets: pd.DataFrame, counts: pd.DataFrame, tags: list[str]
) -> _BaselineFacts:
    """Summarise the baseline's self-agreement across its control groups."""
    base_c = counts[counts["compiler"].isin(tags)]
    base_s = sets[sets["compiler"].isin(tags)]
    if base_c.empty:
        return _BaselineFacts({}, set(), set(), {})

    # How many groups traced each file, so "present in every group" is checked
    # against what was actually measured rather than against the declared grid.
    groups_per_file = base_c.groupby("file")["compiler"].nunique().to_dict()
    # `n_reps_total` repeats on every syscall row of a group, so it has to be
    # taken once per group before the groups are summed.
    per_group = base_s.drop_duplicates(["compiler", "file"])
    traces = per_group.groupby("file")["n_reps_total"].sum().to_dict()

    agg = base_c.groupby(["file", "syscall"]).agg(
        n_groups=("compiler", "nunique"),
        worst_sd=("std_count", "max"),
        lo=("mean_count", "min"),
        hi=("mean_count", "max"),
    )
    # Present in every rep of its group, for every group. Without this a
    # syscall the baseline emits in 2 of 5 reps -- at a constant count each
    # time -- would look constant: `counts_agg` averages only the reps that
    # saw the call.
    always = base_s.groupby(["file", "syscall"])["stable"].all()
    agg = agg.join(always.rename("always"), how="left")

    constant: dict[tuple[str, str], float] = {}
    emitted: set[tuple[str, str]] = set()
    for (f, sc), r in agg.iterrows():
        key = (str(f), str(sc))
        emitted.add(key)
        if not bool(r["always"]):
            continue
        if int(r["n_groups"]) != int(groups_per_file.get(key[0], 0)):
            continue  # some build of the baseline never made this call
        if float(r["worst_sd"]) > 0.0 or float(r["hi"]) != float(r["lo"]):
            continue  # the count moved, within a group or between them
        constant[key] = float(r["lo"])

    return _BaselineFacts(
        constant,
        emitted,
        set(groups_per_file),
        {str(k): int(v) for k, v in traces.items()},
    )


def detect_instability(
    syscall_sets: pd.DataFrame,
    counts: pd.DataFrame,
    config: str,
    baseline: str,
    baseline_tags: list[str],
    variant: str,
    ignore: frozenset[str],
) -> pd.DataFrame:
    """Syscalls the baseline emits deterministically and the variant does not.

    Takes **unpooled** frames: the question is whether the baseline agrees with
    itself across all of its control groups, and pooling is precisely the
    operation that hides a group that disagreed.

    Two shapes of variant nondeterminism count, and both are invisible to the
    other four detectors:

    * the count moves between reps (``1,3,4,2,5`` against a constant ``2``) --
      the count detector's floor is built from that very spread, so the finding
      cancels itself out;
    * the syscall is absent from some reps (``1,3,4,2,-``) -- it drops out of
      the variant's stable set, so the set detector sees nothing new.

    The baseline's own determinism is established per ``(file, syscall)``, not
    corpus-wide: the claim is "on *this* program the measurement is reproducible
    and the variant broke it", which a corpus-wide verdict could neither support
    nor refute.

    A baseline that never emits the syscall at all is also deterministic, and a
    variant that emits it intermittently is then both new *and* unstable. That
    is reported here rather than by the set detector, which requires stable
    presence and would otherwise stay silent on it entirely.
    """
    sets = syscall_sets[syscall_sets["config"] == config]
    cnt = counts[counts["config"] == config]
    if sets.empty or cnt.empty:
        return _empty()
    if "min_count" not in cnt.columns:
        # A pooled frame, or one built by an older aggregate layer. Reporting
        # nothing is the honest response: the rep-level shape is not in here.
        return _empty()

    tags = observed_tags(cnt, config, list(baseline_tags))
    if not tags:
        return _empty()

    base = _baseline_facts(sets, cnt, tags)
    if not base.files:
        return _empty()

    var_c = cnt[cnt["compiler"] == variant]
    var_s = sets[sets["compiler"] == variant]
    if var_c.empty or var_s.empty:
        return _empty()

    presence = {
        (str(f), str(sc)): (int(p), int(t))
        for f, sc, p, t in zip(
            var_s["file"],
            var_s["syscall"],
            var_s["n_reps_present"],
            var_s["n_reps_total"],
        )
    }

    rows = []
    for r in var_c.itertuples(index=False):
        key = (str(r.file), str(r.syscall))
        f, sc = key
        if f not in base.files or sc in ignore:
            continue

        seen, total = presence.get(key, (int(r.n_reps), int(r.n_reps)))
        missing = seen < total
        if not missing and float(r.std_count) == 0.0:
            continue  # the variant is reproducible here too

        if key in base.constant:
            baseline_value = f"constant {base.constant[key]:g}"
        elif key in base.emitted:
            continue  # baseline wobbles here as well: nothing attributable
        else:
            baseline_value = "never emitted"

        # An absent rep is a count of zero even though the store holds no row
        # for it, so the span reaches down to 0 whenever presence flickered.
        lo = 0.0 if missing else float(r.min_count)
        span = float(r.max_count) - lo
        shape = f"{lo:g}-{r.max_count:g} over {total} reps"
        if int(r.n_distinct) > 1:
            shape += f", {int(r.n_distinct)} distinct counts"
        if missing:
            shape += f", absent in {total - seen}"

        rows.append(
            {
                "config": config,
                "pair": f"{variant}v{baseline}",
                "variant": variant,
                "file": f,
                "kind": KIND_INSTAB,
                "syscall": sc,
                "detail": (
                    f"{sc} nondeterministic under variant ({shape}); baseline "
                    f"{baseline_value} across {base.traces.get(f, 0)} traces"
                ),
                "magnitude": span,
                "baseline_value": baseline_value,
                "variant_value": shape,
            }
        )
    return pd.DataFrame(rows, columns=DEPARTURE_COLUMNS) if rows else _empty()


# ---------------------------------------------------------------------------
# Driver
# ---------------------------------------------------------------------------


def all_departures(
    frames,
    settings: Settings,
    config: str,
    variant: str,
    noisy: frozenset[str],
    floor=None,
) -> pd.DataFrame:
    """Run all four detectors for one (config, variant) against the baseline.

    ``noisy`` is the corpus-wide class from the controls. It still gates the
    set-membership and sequence detectors, which have no per-file quantity to
    threshold against.

    The **count** detector is deliberately exempted from the *build* half of
    that class, because it has something better: ``floor.rebuild_delta`` gives
    the baseline's own build variance per (file, syscall). Filtering ``read``
    corpus-wide because the baseline's rebuilds shift it in 6 files would delete a
    finding in the other 666; the per-file floor suppresses exactly those 6 and
    leaves the rest answerable.

    Run noise stays a global exclusion for counts: it is a property of the
    syscall's execution, not of a particular build.

    The set detector runs first because it **subsumes** the others: a syscall
    type it reports for a file is withheld from the count, argument and
    instability detectors there, and n-grams containing it are withheld from the
    sequence detector, so one behavioural change yields one departure instead of
    four. See the subsumption note above ``_syscalls_by_file``.

    The count detector subsumes the instability detector in turn. Where a
    variant's mean shifted far enough to clear a floor *already widened by its
    own jitter*, the shift is the finding and the jitter is a detail of it --
    one visible anyway, since ``enrich`` puts ``variant_sd`` beside every row.
    Instability is reported where it is the *whole* finding: the means agree,
    or differ by less than the variant's own spread, and only the reproducibility
    differs.

    ``instability`` reads the **unpooled** frames. It asks whether the baseline
    agrees with itself across control groups, and pooling is exactly the step
    that hides a group that did not.
    """
    ignore = frozenset(settings.detect.departure_ignore)
    excluded = frozenset(settings.detect.excluded_syscalls)
    base = settings.baseline

    build_floor = floor.rebuild_delta if floor is not None else None
    run_noisy = floor.run_noisy if floor is not None else noisy

    set_rows = detect_set(
        frames.syscall_sets, config, base, variant, ignore | excluded | noisy
    )
    # Runs first, and its findings suppress the same event in the other four.
    explained = _syscalls_by_file(set_rows)

    count_rows = _drop_subsumed(
        detect_count(
            frames.counts,
            config,
            base,
            variant,
            ignore | excluded,
            run_noisy,
            settings.detect.count_tol,
            build_floor=build_floor,
        ),
        explained,
    )

    raw = getattr(frames, "unpooled", frames)
    instab_rows = _drop_subsumed(
        detect_instability(
            raw.syscall_sets,
            raw.counts,
            config,
            base,
            settings.baseline_tags(),
            variant,
            ignore | excluded,
        ),
        _merge_explained(explained, _syscalls_by_file(count_rows)),
    )

    parts = [
        set_rows,
        count_rows,
        _drop_subsumed(
            detect_args(frames.args, config, base, variant, ignore | excluded),
            explained,
        ),
        detect_sequence(
            frames.ngrams,
            config,
            base,
            variant,
            ignore | excluded | noisy,
            explained=explained,
        ),
        instab_rows,
    ]
    parts = [p for p in parts if not p.empty]
    if not parts:
        return _empty()
    return pd.concat(parts, ignore_index=True)
