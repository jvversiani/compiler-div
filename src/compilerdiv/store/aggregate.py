"""Raw streams -> tidy analysis frames.

Everything downstream consumes long-format DataFrames -- detectors take frames
and return frames. Nested ``defaultdict`` pivots are deliberately avoided: they
make every consumer responsible for the shape, which is where fragility
accumulates.

The "stable" concept appears here and is used by three of the four detectors.
A fact is *stable* for a (compiler, file) when it holds in **every** trace rep.
For sets (syscall types, n-grams, argument values) that means intersecting
across reps; for counts it means the mean plus a dispersion measure. This is
what makes the detectors self-calibrating on threaded programs: the scheduler
scrambles which n-grams appear, the intersection across reps shrinks, and the
detector goes quiet on its own rather than firing spuriously.
"""

from __future__ import annotations

import pandas as pd

from .raw import ARGS, BENCH, CORPUS, COUNTS, ELF, RawStore, SEQUENCE

#: Columns :func:`counts_agg` produces. ``min_count`` / ``max_count`` /
#: ``n_distinct`` describe the *shape* of the rep sample rather than its centre:
#: a mean of 2.5 with sd 1.1 could be ``2,3`` or ``1,4``, and the instability
#: detector needs to tell those apart. Note that :func:`pool_counts` does not
#: carry them -- "the min over nine control groups" is not a quantity anyone
#: should reason about -- so they are defined only on unpooled rows.
_COUNT_COLUMNS = [
    "config",
    "compiler",
    "file",
    "syscall",
    "mean_count",
    "std_count",
    "min_count",
    "max_count",
    "n_distinct",
    "n_reps",
]


def counts_agg(store: RawStore) -> pd.DataFrame:
    """Per-syscall count statistics over the reps of one (compiler, file).

    Only reps in which the syscall actually fired contribute: ``strace`` reports
    what happened, so a rep without the call has no row and is **not** a zero.
    ``n_reps`` is therefore "reps that saw this call", not "reps run" -- compare
    it against :func:`stable_syscall_sets` to find the difference.

    Columns: :data:`_COUNT_COLUMNS`.
    """
    raw = store.read(COUNTS)
    if raw.empty:
        return pd.DataFrame(columns=_COUNT_COLUMNS)
    raw = raw.drop_duplicates(
        subset=["config", "compiler", "file", "rep", "syscall"], keep="last"
    )
    out = (
        raw.groupby(["config", "compiler", "file", "syscall"])["count"]
        .agg(
            mean_count="mean",
            std_count=lambda x: float(x.std(ddof=0)),
            min_count="min",
            max_count="max",
            n_distinct="nunique",
            n_reps="count",
        )
        .reset_index()
    )
    out["std_count"] = out["std_count"].fillna(0.0)
    return out


def stable_syscall_sets(store: RawStore) -> pd.DataFrame:
    """Syscall types present in *every* rep of a (config, compiler, file).

    Columns: config, compiler, file, syscall, n_reps_present, n_reps_total, stable
    """
    raw = store.read(COUNTS, columns=["config", "compiler", "file", "rep", "syscall"])
    if raw.empty:
        return pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "syscall",
                "n_reps_present",
                "n_reps_total",
                "stable",
            ]
        )
    totals = (
        raw.groupby(["config", "compiler", "file"])["rep"]
        .nunique()
        .rename("n_reps_total")
        .reset_index()
    )
    present = (
        raw.drop_duplicates(["config", "compiler", "file", "rep", "syscall"])
        .groupby(["config", "compiler", "file", "syscall"])["rep"]
        .nunique()
        .rename("n_reps_present")
        .reset_index()
    )
    out = present.merge(totals, on=["config", "compiler", "file"], how="left")
    out["stable"] = out["n_reps_present"] == out["n_reps_total"]
    return out


def stable_ngrams(store: RawStore) -> pd.DataFrame:
    """n-grams present in *every* rep of a (config, compiler, file).

    The intersection across reps is the whole point: it drops n-grams that owe
    their existence to a particular scheduler interleaving, without needing to
    know which program is threaded.

    Columns: config, compiler, file, n, ngram, n_reps_present, n_reps_total,
             stable, n_threads
    """
    raw = store.read(SEQUENCE)
    if raw.empty:
        return pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "n",
                "ngram",
                "n_reps_present",
                "n_reps_total",
                "stable",
                "n_threads",
            ]
        )
    raw = raw.drop_duplicates(
        subset=["config", "compiler", "file", "rep", "n", "ngram"], keep="last"
    )
    totals = (
        raw.groupby(["config", "compiler", "file"])["rep"]
        .nunique()
        .rename("n_reps_total")
        .reset_index()
    )
    threads = (
        raw.groupby(["config", "compiler", "file"])["n_threads"]
        .max()
        .rename("n_threads")
        .reset_index()
    )
    present = (
        raw.groupby(["config", "compiler", "file", "n", "ngram"])["rep"]
        .nunique()
        .rename("n_reps_present")
        .reset_index()
    )
    out = present.merge(totals, on=["config", "compiler", "file"], how="left")
    out = out.merge(threads, on=["config", "compiler", "file"], how="left")
    out["stable"] = out["n_reps_present"] == out["n_reps_total"]
    return out


def stable_args(store: RawStore) -> pd.DataFrame:
    """Argument values present in *every* rep of a (config, compiler, file).

    Columns: config, compiler, file, syscall, arg, n_reps_present,
             n_reps_total, stable, mean_count
    """
    raw = store.read(ARGS)
    if raw.empty:
        return pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "syscall",
                "arg",
                "n_reps_present",
                "n_reps_total",
                "stable",
                "mean_count",
            ]
        )
    raw = raw.drop_duplicates(
        subset=["config", "compiler", "file", "rep", "syscall", "arg"], keep="last"
    )
    totals = (
        raw.groupby(["config", "compiler", "file"])["rep"]
        .nunique()
        .rename("n_reps_total")
        .reset_index()
    )
    present = (
        raw.groupby(["config", "compiler", "file", "syscall", "arg"])
        .agg(n_reps_present=("rep", "nunique"), mean_count=("count", "mean"))
        .reset_index()
    )
    out = present.merge(totals, on=["config", "compiler", "file"], how="left")
    out["stable"] = out["n_reps_present"] == out["n_reps_total"]
    return out


def bench_agg(store: RawStore) -> pd.DataFrame:
    """Per-file rep means of compile time, exec time, size.

    Only verified rows are aggregated: an unverified binary's timings describe
    a program that did not do what it was supposed to.

    Columns: config, compiler, file, compile_s, exec_s, size_b, n_reps
    """
    raw = store.read(BENCH)
    if raw.empty:
        return pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "compile_s",
                "exec_s",
                "size_b",
                "n_reps",
            ]
        )
    raw = raw.drop_duplicates(subset=["config", "compiler", "file", "rep"], keep="last")
    if "verified" in raw.columns:
        raw = raw[raw["verified"].astype(bool)]
    if raw.empty:
        return pd.DataFrame(
            columns=[
                "config",
                "compiler",
                "file",
                "compile_s",
                "exec_s",
                "size_b",
                "n_reps",
            ]
        )
    return (
        raw.groupby(["config", "compiler", "file"])
        .agg(
            compile_s=("compile_s", "mean"),
            exec_s=("exec_s", "mean"),
            size_b=("size_b", "mean"),
            n_reps=("rep", "count"),
        )
        .reset_index()
    )


_ELF_COLUMNS = [
    "config",
    "compiler",
    "file",
    "size_b",
    "n_segments",
    "n_sections",
    "interp",
    "n_dynamic_needed",
    "probe_ok",
    "sloc",
]


def elf_agg(store: RawStore) -> pd.DataFrame:
    """Static layout facts, one row per (config, compiler, file).

    ``sloc`` is stored once per file in CORPUS and joined on here, so consumers
    keep seeing it as a column of this frame.
    """
    raw = store.read(ELF)
    if raw.empty:
        return pd.DataFrame(columns=_ELF_COLUMNS)
    out = raw.drop_duplicates(subset=["config", "compiler", "file"], keep="last")

    corpus = store.read(CORPUS)
    if corpus.empty:
        out = out.assign(sloc=pd.NA)
    else:
        corpus = corpus.drop_duplicates(subset=["file"], keep="last")
        out = out.merge(corpus, on="file", how="left")
    return out.reindex(columns=_ELF_COLUMNS)


# ---------------------------------------------------------------------------
# Pooling the baseline across its control groups
# ---------------------------------------------------------------------------
#
# With a nested control grid the baseline is not one sample but `builds x
# passes` of them. Comparing a variant against the primary group alone would
# use a ninth of the evidence and report the baseline's own build quirks as
# departures of the variant.
#
# Each stream pools differently, and the direction is always the conservative
# one -- a fact must be absent from EVERY baseline group before a variant can be
# said to have introduced it:
#
#   syscall_sets  stable in ANY group      => the baseline makes this call
#   args          union of stable values   => the baseline passes this value
#   ngrams        union of per-group sets  => the baseline produces this order
#   counts        pooled mean; sd widened by the spread BETWEEN groups
#
# The pooled frame is shaped exactly like a single compiler's, so the detectors
# take it without a signature change.


def _pooled_tag(baseline: str) -> str:
    """Compiler key the pooled frame carries. Never written to the store."""
    return baseline


def observed_tags(df: pd.DataFrame, config: str, tags: list[str]) -> list[str]:
    """The declared control groups that this frame actually contains.

    A store acquired under a different control grid -- or one still being
    filled -- holds fewer groups than the config declares. Pooling over the
    declared list regardless would treat every absent group as a measurement of
    zero, which is not a missing observation but a fabricated one.
    """
    if df.empty or "compiler" not in df.columns:
        return []
    present = set(df[df["config"] == config]["compiler"])
    return [t for t in tags if t in present]


def pool_sets(df: pd.DataFrame, config: str, tags: list[str]) -> pd.DataFrame:
    """Union of stably-present syscalls (or n-grams) over the baseline groups."""
    if df.empty or not tags:
        return df
    sub = df[(df["config"] == config) & df["compiler"].isin(tags)]
    if sub.empty:
        return sub
    key = "ngram" if "ngram" in sub.columns else "syscall"
    stable = sub[sub["stable"]]
    if stable.empty:
        return stable
    out = stable.drop_duplicates(subset=["config", "file", key], keep="first").copy()
    out["compiler"] = _pooled_tag(tags[0])
    return out.reset_index(drop=True)


def pool_args(df: pd.DataFrame, config: str, tags: list[str]) -> pd.DataFrame:
    """Union of stable argument values over the baseline groups."""
    if df.empty or not tags:
        return df
    sub = df[(df["config"] == config) & df["compiler"].isin(tags) & df["stable"]]
    if sub.empty:
        return sub
    out = sub.drop_duplicates(
        subset=["config", "file", "syscall", "arg"], keep="first"
    ).copy()
    out["compiler"] = _pooled_tag(tags[0])
    return out.reset_index(drop=True)


def pool_counts(df: pd.DataFrame, config: str, tags: list[str]) -> pd.DataFrame:
    """Mean over the baseline groups, with the between-group spread folded in.

    ``std_count`` becomes ``max(worst within-group sd, spread of the group
    means)``. The second term is what a single group cannot see: a syscall
    rock-steady inside every build but different between builds has zero
    within-group sd and is nonetheless not something a variant can be blamed
    for.

    A group missing a *syscall* contributes a zero, not a skipped row -- "this
    build never made the call" is a real observation about the spread, and
    dropping it would understate the variance. A group missing from the store
    **entirely** is different: it was never measured, and averaging a zero in
    for it would deflate every baseline mean by the fraction of groups absent.
    Only the groups present are pooled -- see :func:`observed_tags`.
    """
    tags = observed_tags(df, config, tags)
    if df.empty or not tags:
        return df
    sub = df[(df["config"] == config) & df["compiler"].isin(tags)]
    if sub.empty:
        return sub

    wide = sub.pivot_table(
        index=["file", "syscall"],
        columns="compiler",
        values="mean_count",
        fill_value=0.0,
    ).reindex(columns=tags, fill_value=0.0)
    within = (
        sub.groupby(["file", "syscall"])["std_count"]
        .max()
        .reindex(wide.index)
        .fillna(0.0)
    )
    reps = (
        sub.groupby(["file", "syscall"])["n_reps"].sum().reindex(wide.index).fillna(0)
    )

    out = pd.DataFrame(
        {
            "config": config,
            "compiler": _pooled_tag(tags[0]),
            "mean_count": wide.mean(axis=1),
            "std_count": pd.concat(
                [within, wide.std(axis=1, ddof=0).fillna(0.0)], axis=1
            ).max(axis=1),
            "n_reps": reps.astype(int),
        }
    ).reset_index()
    return out[
        ["config", "compiler", "file", "syscall", "mean_count", "std_count", "n_reps"]
    ]


class PooledBaseline:
    """A :class:`Frames` view whose baseline is every control group at once.

    Wraps a real ``Frames`` and, for one config, replaces the baseline compiler's
    rows with the pooled ones while dropping the individual control tags. The
    detectors see a frame of exactly the usual shape and need no changes.

    Only the streams the detectors read are pooled. ``bench`` and ``elf`` pass
    through: benchmarks are never recorded for control tags, and layout facts
    are deliberately those of the primary build.

    :func:`..stats.noise.derive_noise` must be given the **unpooled** frames --
    it exists to compare the groups against each other, which pooling erases.
    """

    def __init__(self, frames: "Frames", settings, config: str):
        self._f = frames
        self._settings = settings
        self._config = config
        self._tags = settings.baseline_tags()
        self._cache: dict[str, pd.DataFrame] = {}

    def _pooled(self, name: str, source: pd.DataFrame, fn) -> pd.DataFrame:
        if name not in self._cache:
            present = observed_tags(source, self._config, self._tags)
            if source.empty or len(present) < 2:
                # Nothing to pool: one group, or a store from another grid.
                # Leaving the frame untouched degrades to the old single-group
                # behaviour instead of inventing rows for groups never measured.
                self._cache[name] = source
            else:
                controls = set(present)
                kept = source[
                    (source["config"] != self._config)
                    | (~source["compiler"].isin(controls))
                ]
                pooled = fn(source, self._config, self._tags)
                self._cache[name] = (
                    pd.concat([kept, pooled], ignore_index=True)
                    if not pooled.empty
                    else kept
                )
        return self._cache[name]

    @property
    def counts(self) -> pd.DataFrame:
        return self._pooled("counts", self._f.counts, pool_counts)

    @property
    def syscall_sets(self) -> pd.DataFrame:
        return self._pooled("syscall_sets", self._f.syscall_sets, pool_sets)

    @property
    def ngrams(self) -> pd.DataFrame:
        return self._pooled("ngrams", self._f.ngrams, pool_sets)

    @property
    def args(self) -> pd.DataFrame:
        return self._pooled("args", self._f.args, pool_args)

    @property
    def bench(self) -> pd.DataFrame:
        return self._f.bench

    @property
    def elf(self) -> pd.DataFrame:
        return self._f.elf

    @property
    def unpooled(self) -> "Frames":
        """The frames before pooling, with the control groups still separate.

        Pooling answers "what does the baseline do, taken as a whole", which is
        the right question for every detector that asks whether the variant
        introduced something. It is the wrong question for one that asks whether
        the baseline is *self-consistent*: the union hides a group that
        disagreed. Such a detector takes this instead, exactly as
        :func:`..stats.noise.derive_noise` does.
        """
        return self._f


class Frames:
    """Lazily-computed bundle of every analysis frame."""

    def __init__(self, store: RawStore):
        self._store = store
        self._cache: dict[str, pd.DataFrame] = {}

    def _get(self, name: str, fn) -> pd.DataFrame:
        if name not in self._cache:
            self._cache[name] = fn(self._store)
        return self._cache[name]

    @property
    def counts(self) -> pd.DataFrame:
        return self._get("counts", counts_agg)

    @property
    def syscall_sets(self) -> pd.DataFrame:
        return self._get("syscall_sets", stable_syscall_sets)

    @property
    def ngrams(self) -> pd.DataFrame:
        return self._get("ngrams", stable_ngrams)

    @property
    def args(self) -> pd.DataFrame:
        return self._get("args", stable_args)

    @property
    def bench(self) -> pd.DataFrame:
        return self._get("bench", bench_agg)

    @property
    def elf(self) -> pd.DataFrame:
        return self._get("elf", elf_agg)

    @property
    def unpooled(self) -> "Frames":
        """Already unpooled. Present so consumers need not care which they hold."""
        return self
