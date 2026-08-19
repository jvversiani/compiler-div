"""Append-only raw logs.

The raw logs are the contract between ``compilerdiv acquire`` and ``compilerdiv
analyze``. Acquisition streams to them and never reads them back except to
resume; analysis reads them and never writes. A crash loses at most
``flush_every`` binaries of work.

Storage is one file per flush under a per-stream directory. The format is
Parquet when a parquet engine is installed, CSV otherwise. Parquet is strongly
preferred on a full sweep: the syscall and n-gram streams reach tens of millions
of rows, and column pruning means resume reads four columns instead of the whole
table. The CSV fallback exists so the package runs without ``pyarrow``, at the
cost of disk and resume speed. :func:`storage_backend` reports which is active,
and ``compilerdiv doctor`` prints it.

A ``manifest.json`` records the settings fingerprint. Resuming with different
acquisition parameters is refused rather than silently producing a mixed
dataset -- the failure mode the old ``RESUME`` docstring warned about in prose.
"""

from __future__ import annotations

import json
import time
import uuid
from collections import defaultdict
from dataclasses import dataclass
from functools import lru_cache
from pathlib import Path

import pandas as pd

from ..config import Settings


@lru_cache(maxsize=1)
def storage_backend() -> str:
    """``"parquet"`` if an engine is importable, else ``"csv"``."""
    for mod in ("pyarrow", "fastparquet"):
        try:
            __import__(mod)
            return "parquet"
        except ImportError:
            continue
    return "csv"


_EXT = {"parquet": ".parquet", "csv": ".csv"}

# Stream names.
COUNTS = "counts"
SEQUENCE = "sequence"
ARGS = "args"
BENCH = "bench"
ELF = "elf"
CORPUS = "corpus"
ERRORS = "errors"

SCHEMAS: dict[str, list[str]] = {
    # One row per (config, compiler, file, rep, syscall).
    COUNTS: ["config", "compiler", "file", "rep", "syscall", "count"],
    # One row per (config, compiler, file, rep, n, ngram). Distinct n-grams
    # only -- multiplicity is not used by the set-difference detector.
    SEQUENCE: ["config", "compiler", "file", "rep", "n", "ngram", "n_threads"],
    # One row per (config, compiler, file, rep, syscall, normalized arg).
    ARGS: ["config", "compiler", "file", "rep", "syscall", "arg", "count"],
    # One row per (config, compiler, file, rep).
    BENCH: [
        "config",
        "compiler",
        "file",
        "rep",
        "compile_s",
        "exec_s",
        "size_b",
        "verified",
        "returncode",
    ],
    # One row per (config, compiler, file). ELF facts only: `sloc` is a property
    # of the source, not of any one build, and lives in CORPUS.
    ELF: [
        "config",
        "compiler",
        "file",
        "size_b",
        "n_segments",
        "n_sections",
        "interp",
        "n_dynamic_needed",
        "probe_ok",
    ],
    # One row per file. Source facts, written once per sweep and joined onto the
    # ELF frame at analysis time.
    CORPUS: ["file", "sloc"],
    ERRORS: ["config", "compiler", "file", "phase", "reason", "ts"],
}


class FingerprintMismatch(RuntimeError):
    """Raised when resuming against a raw log built with other settings."""


@dataclass
class ResumeState:
    """What acquisition has already recorded.

    Attributes
    ----------
    bench_done:
        ``(config, compiler, file, rep)`` tuples already timed.
    trace_reps:
        ``(config, compiler, file) -> highest rep recorded``.
    elf_done:
        ``(config, compiler, file)`` tuples with layout facts recorded.
    """

    bench_done: set[tuple[str, str, str, int]]
    trace_reps: dict[tuple[str, str, str], int]
    elf_done: set[tuple[str, str, str]]

    @classmethod
    def empty(cls) -> ResumeState:
        return cls(set(), defaultdict(int), set())


class RawStore:
    """Append-only Parquet store with buffered writes."""

    def __init__(self, root: Path, fingerprint: str):
        self.root = root
        self.fingerprint = fingerprint
        self._buffers: dict[str, list[list]] = defaultdict(list)

    # -- lifecycle ---------------------------------------------------------

    def manifest_path(self) -> Path:
        return self.root / "manifest.json"

    def init(self, settings: Settings, *, reset: bool = False) -> None:
        """Create the store, validating or writing the fingerprint."""
        if reset and self.root.exists():
            import shutil

            shutil.rmtree(self.root)

        self.root.mkdir(parents=True, exist_ok=True)
        mp = self.manifest_path()

        if mp.is_file():
            existing = json.loads(mp.read_text())
            if existing.get("fingerprint") != self.fingerprint:
                raise FingerprintMismatch(
                    "The existing raw log was acquired with different settings.\n"
                    f"  stored:  {existing.get('fingerprint')}\n"
                    f"  current: {self.fingerprint}\n"
                    "Acquisition parameters (compilers, configs, trace.reps, "
                    "bench.reps, controls, capture flags) affect what is in the "
                    "log, so resuming would mix incompatible data.\n"
                    "Either restore the old settings, or re-acquire from scratch "
                    "with --reset (this deletes the existing raw log)."
                )
            return

        mp.write_text(
            json.dumps(
                {
                    "fingerprint": self.fingerprint,
                    "created": time.strftime("%Y-%m-%dT%H:%M:%S"),
                    "settings": _jsonable(settings.to_dict()),
                },
                indent=2,
            )
        )

    def stream_dir(self, stream: str) -> Path:
        return self.root / stream

    # -- writing -----------------------------------------------------------

    def add(self, stream: str, row: list) -> None:
        self._buffers[stream].append(row)

    def extend(self, stream: str, rows: list[list]) -> None:
        self._buffers[stream].extend(rows)

    def buffered(self) -> int:
        return sum(len(v) for v in self._buffers.values())

    def flush(self) -> None:
        """Write every buffer to a fresh part file.

        Part names are ``part-<epoch_ns>-<rand>``, so a lexicographic sort of
        the directory is also write order. That makes the ``keep="last"``
        deduplication in :mod:`..store.aggregate` deterministic: when a crash
        mid-flush and a subsequent resume produce two rows for the same key,
        the later write wins rather than whichever filename happened to sort
        higher.
        """
        backend = storage_backend()
        for stream, rows in list(self._buffers.items()):
            if not rows:
                continue
            d = self.stream_dir(stream)
            d.mkdir(parents=True, exist_ok=True)
            df = pd.DataFrame(rows, columns=SCHEMAS[stream])
            stamp = f"{time.time_ns():019d}"
            part = d / f"part-{stamp}-{uuid.uuid4().hex[:8]}{_EXT[backend]}"
            if backend == "parquet":
                df.to_parquet(part, index=False, compression="snappy")
            else:
                df.to_csv(part, index=False)
            self._buffers[stream] = []

    # -- reading -----------------------------------------------------------

    def parts(self, stream: str) -> list[Path]:
        d = self.stream_dir(stream)
        if not d.is_dir():
            return []
        return sorted(list(d.glob("*.parquet")) + list(d.glob("*.csv")))

    def read(self, stream: str, columns: list[str] | None = None) -> pd.DataFrame:
        """Read a full stream. Missing streams yield an empty typed frame.

        ``columns`` prunes at read time under Parquet; under CSV it prunes after
        parsing, which still bounds memory but not I/O.
        """
        parts = self.parts(stream)
        if not parts:
            return pd.DataFrame(columns=columns or SCHEMAS[stream])
        frames = []
        for p in parts:
            if p.suffix == ".parquet":
                frames.append(pd.read_parquet(p, columns=columns))
            else:
                frames.append(pd.read_csv(p, usecols=columns))
        out = pd.concat(frames, ignore_index=True)
        if columns is not None:
            out = out[columns]
        return out

    def resume_state(self) -> ResumeState:
        """Scan the store for completed work, reading only the key columns."""
        state = ResumeState.empty()

        b = self.read(BENCH, columns=["config", "compiler", "file", "rep"])
        for r in b.itertuples(index=False):
            state.bench_done.add((r.config, r.compiler, str(r.file), int(r.rep)))

        c = self.read(COUNTS, columns=["config", "compiler", "file", "rep"])
        if not c.empty:
            g = c.groupby(["config", "compiler", "file"])["rep"].max()
            for (cfg, comp, f), rep in g.items():
                state.trace_reps[(cfg, comp, str(f))] = int(rep)

        # Only rows whose probe actually ran count as done. A sweep on a machine
        # without readelf writes size-only rows; treating those as collected
        # would make the store unrepairable short of discarding it. Stores
        # written before probe_ok existed lack the column -- their ELF facts are
        # stale for other reasons too (see the n_segments fix), so they are
        # re-collected rather than trusted.
        try:
            e = self.read(ELF, columns=["config", "compiler", "file", "probe_ok"])
        except (KeyError, ValueError):
            e = pd.DataFrame(columns=["config", "compiler", "file", "probe_ok"])
        for r in e.itertuples(index=False):
            if bool(r.probe_ok):
                state.elf_done.add((r.config, r.compiler, str(r.file)))

        return state


def _jsonable(obj):
    """Recursively convert tuples to lists and Paths to str for JSON."""
    if isinstance(obj, dict):
        return {k: _jsonable(v) for k, v in obj.items()}
    if isinstance(obj, (list, tuple)):
        return [_jsonable(v) for v in obj]
    if isinstance(obj, Path):
        return str(obj)
    return obj
