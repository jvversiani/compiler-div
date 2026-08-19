"""Raw store: round-trip, resume, fingerprint guard, aggregation.

OOP layout under the ``store`` marker (``pytest -m store``). The module-level
``store`` fixture builds an initialised store on a temp path; the shared
``settings`` fixture comes from conftest.
"""

from __future__ import annotations

import pytest

from compilerdiv.config import Settings, TraceSettings
from compilerdiv.store.aggregate import Frames
from compilerdiv.store.raw import (
    BENCH,
    CORPUS,
    COUNTS,
    ELF,
    SEQUENCE,
    FingerprintMismatch,
    RawStore,
    storage_backend,
)

pytestmark = pytest.mark.store


@pytest.fixture
def store(tmp_path, settings):
    s = RawStore(tmp_path / "raw", settings.fingerprint())
    s.init(settings)
    return s


class TestRoundTrip:
    def test_roundtrip(self, store):
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        store.add(COUNTS, ["basic", "A", "p0", 2, "read", 4])
        store.flush()
        df = store.read(COUNTS)
        assert len(df) == 2
        assert set(df["syscall"]) == {"read"}

    def test_read_missing_stream_is_empty_not_error(self, store):
        df = store.read(SEQUENCE)
        assert df.empty
        assert "ngram" in df.columns

    def test_flush_clears_buffer(self, store):
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        assert store.buffered() == 1
        store.flush()
        assert store.buffered() == 0
        store.flush()  # second flush must not duplicate
        assert len(store.read(COUNTS)) == 1

    def test_multiple_parts_concatenated(self, store):
        """Each flush writes its own part; reads concatenate them."""
        for i in range(3):
            store.add(COUNTS, ["basic", "A", f"p{i}", 1, "read", 4])
            store.flush()
        assert len(store.parts(COUNTS)) == 3
        assert len(store.read(COUNTS)) == 3

    def test_parts_sort_in_write_order(self, store):
        for i in range(5):
            store.add(COUNTS, ["basic", "A", f"p{i}", 1, "read", i])
            store.flush()
        names = [p.name for p in store.parts(COUNTS)]
        assert names == sorted(names)
        assert list(store.read(COUNTS)["count"]) == [0, 1, 2, 3, 4]


class TestBackend:
    def test_storage_backend_is_one_of_two(self, store):
        assert storage_backend() in {"parquet", "csv"}

    def test_parts_have_backend_extension(self, store):
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        store.flush()
        expected = ".parquet" if storage_backend() == "parquet" else ".csv"
        assert all(p.suffix == expected for p in store.parts(COUNTS))


class TestResumeState:
    def test_bench(self, store):
        store.add(BENCH, ["basic", "A", "p0", 1, 0.5, 0.01, 1000, True, 0])
        store.add(BENCH, ["basic", "A", "p0", 2, 0.5, 0.01, 1000, True, 0])
        store.flush()
        st = store.resume_state()
        assert ("basic", "A", "p0", 1) in st.bench_done
        assert ("basic", "A", "p0", 2) in st.bench_done
        assert ("basic", "A", "p0", 3) not in st.bench_done

    def test_trace_max_rep(self, store):
        for rep in (1, 2):
            store.add(COUNTS, ["basic", "A", "p0", rep, "read", 4])
        store.flush()
        st = store.resume_state()
        assert st.trace_reps[("basic", "A", "p0")] == 2

    def test_elf_done_only_counts_successful_probes(self, store):
        """A size-only row (no readelf) must stay collectable.

        Otherwise installing binutils later cannot repair the store: the row is
        present, so the sweep skips it forever.
        """
        store.add(ELF, ["basic", "A", "ok", 100, 8, 27, "/lib/ld", 3, True])
        store.add(ELF, ["basic", "A", "degraded", 100, 0, 0, "", 0, False])
        store.flush()
        st = store.resume_state()
        assert ("basic", "A", "ok") in st.elf_done
        assert ("basic", "A", "degraded") not in st.elf_done


class TestElfAgg:
    def test_sloc_joined_from_corpus(self, store):
        """sloc lives once per file in CORPUS, not per (config, compiler)."""
        store.add(ELF, ["basic", "A", "p0", 100, 8, 27, "/lib/ld", 3, True])
        store.add(ELF, ["basic", "B", "p0", 200, 9, 30, "/lib/ld", 3, True])
        store.add(CORPUS, ["p0", 42])
        store.flush()
        elf = Frames(store).elf
        assert set(elf["sloc"]) == {42}
        assert len(elf) == 2

    def test_missing_corpus_leaves_sloc_null(self, store):
        """Old stores have no CORPUS stream; the column must still exist."""
        store.add(ELF, ["basic", "A", "p0", 100, 8, 27, "/lib/ld", 3, True])
        store.flush()
        elf = Frames(store).elf
        assert "sloc" in elf.columns
        assert elf["sloc"].isna().all()


class TestFingerprintGuard:
    def test_refuses_mismatch(self, tmp_path, settings):
        s1 = RawStore(tmp_path / "raw", settings.fingerprint())
        s1.init(settings)

        changed = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            trace=TraceSettings(reps=99),
        )
        s2 = RawStore(tmp_path / "raw", changed.fingerprint())
        with pytest.raises(FingerprintMismatch, match="different settings"):
            s2.init(changed)

    def test_reset_discards_and_reinitialises(self, tmp_path, settings):
        s1 = RawStore(tmp_path / "raw", settings.fingerprint())
        s1.init(settings)
        s1.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        s1.flush()
        assert len(s1.read(COUNTS)) == 1

        changed = Settings(
            baseline="A",
            compilers=settings.compilers,
            configs=settings.configs,
            trace=TraceSettings(reps=99),
        )
        s2 = RawStore(tmp_path / "raw", changed.fingerprint())
        s2.init(changed, reset=True)
        assert s2.read(COUNTS).empty


class TestAggregation:
    def test_counts_agg_mean_and_std(self, store):
        for rep, n in ((1, 4), (2, 4), (3, 6)):
            store.add(COUNTS, ["basic", "A", "p0", rep, "read", n])
        store.flush()
        f = Frames(store)
        row = f.counts.iloc[0]
        assert row["mean_count"] == pytest.approx(14 / 3)
        assert row["std_count"] > 0
        assert row["n_reps"] == 3

    def test_stable_syscall_sets(self, store):
        # read in all 3 reps -> stable; write in 1 -> unstable
        for rep in (1, 2, 3):
            store.add(COUNTS, ["basic", "A", "p0", rep, "read", 4])
        store.add(COUNTS, ["basic", "A", "p0", 1, "write", 1])
        store.flush()
        f = Frames(store)
        d = f.syscall_sets.set_index("syscall")["stable"].to_dict()
        assert d["read"] is True or d["read"] == True  # noqa: E712
        assert not d["write"]

    def test_stable_ngrams_intersect_across_reps(self, store):
        for rep in (1, 2, 3):
            store.add(SEQUENCE, ["basic", "A", "p0", rep, 3, "a→b→c", 1])
        store.add(SEQUENCE, ["basic", "A", "p0", 1, 3, "x→y→z", 1])
        store.flush()
        f = Frames(store)
        d = f.ngrams.set_index("ngram")["stable"].to_dict()
        assert d["a→b→c"]
        assert not d["x→y→z"]

    def test_bench_agg_excludes_unverified(self, store):
        store.add(BENCH, ["basic", "A", "p0", 1, 0.5, 0.01, 1000, True, 0])
        store.add(BENCH, ["basic", "A", "p1", 1, 9.9, 9.9, 1, False, 139])
        store.flush()
        f = Frames(store)
        assert set(f.bench["file"]) == {"p0"}

    def test_duplicate_rows_deduplicated_last_wins(self, store):
        """A crash mid-flush then resume can produce duplicate keys.

        Part names are timestamp-prefixed, so sorted-filename order is write
        order and the later write genuinely wins.
        """
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 4])
        store.flush()
        store.add(COUNTS, ["basic", "A", "p0", 1, "read", 7])
        store.flush()
        f = Frames(store)
        assert len(f.counts) == 1
        assert f.counts.iloc[0]["mean_count"] == 7
        assert f.counts.iloc[0]["n_reps"] == 1
