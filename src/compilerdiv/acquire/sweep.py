"""The acquisition sweep.

Orchestration only: every measurement lives in :mod:`.build` or :mod:`.trace`,
every write in :mod:`..store.raw`.

Per (config, compiler) the sweep does three passes:

1. **bench** -- compile each program ``bench.reps`` times, timing the compile,
   measuring size, running the binary untraced and verifying its output.
2. **trace** -- trace the surviving binary ``trace.reps`` times.
3. **controls** (baseline only) -- ``controls.builds`` independent compilations
   of the same source, each traced under ``controls.passes`` independent
   sessions.

The control grid is nested, and the nesting is what makes a finding like a
variant's ``read(+1)`` attributable:

* passes of the **same** build differ only by run-to-run nondeterminism;
* different **builds** differ by that plus build nondeterminism.

Both are therefore measured rather than inferred. A flat design -- one rerun and
one rebuild -- can only estimate the build class by subtracting the run class
from a single recompilation, which is n=2 and cannot distinguish a reproducible
property of the compiler from a one-off linker hiccup.

Build 0 pass 0 is the baseline itself and keeps the user's key; the rest are
generated tags (``A@1.2`` = build 1, rerun 2). See :mod:`..config`.
"""

from __future__ import annotations

import shutil
import sys
import time
from dataclasses import dataclass
from pathlib import Path

from ..config import Settings
from ..corpus import Program, load_corpus
from ..store.raw import (
    ARGS,
    BENCH,
    CORPUS,
    COUNTS,
    ELF,
    ERRORS,
    RawStore,
    ResumeState,
    SEQUENCE,
)
from .build import compile_one, elf_facts, run_binary
from .trace import ArgNormalizer, TraceResult, ngram_profile, trace_once


@dataclass
class SweepStats:
    compiled: int = 0
    compile_failed: int = 0
    unverified: int = 0
    traced: int = 0
    trace_failed: int = 0
    skipped: int = 0


class Sweeper:
    def __init__(self, settings: Settings, store: RawStore, *, verbose: bool = True):
        self.s = settings
        self.store = store
        self.verbose = verbose
        self.norm = ArgNormalizer(settings.detect.arg_normalizers)
        self.stats = SweepStats()

    # -- helpers -----------------------------------------------------------

    def _log(self, msg: str) -> None:
        if self.verbose:
            print(msg, flush=True)

    def _err(self, config: str, comp: str, stem: str, phase: str, reason: str) -> None:
        sys.stderr.write(f"[ERROR] {config}/{comp}/{stem} [{phase}]: {reason}\n")
        self.store.add(
            ERRORS,
            [config, comp, stem, phase, reason, time.strftime("%Y-%m-%dT%H:%M:%S")],
        )

    def _maybe_flush(self, counter: int) -> None:
        if counter % self.s.flush_every == 0:
            self.store.flush()

    def _bin_dir(self, comp: str, config: str) -> Path:
        return self.s.work_path / config / comp

    def _record_trace(
        self, config: str, tag: str, stem: str, rep: int, tr: TraceResult
    ) -> None:
        """Write one trace's three views into the raw store."""
        self.store.extend(
            COUNTS,
            [[config, tag, stem, rep, sc, n] for sc, n in tr.counts.items()],
        )
        if self.s.trace.capture_sequence:
            prof = ngram_profile(tr, self.s.trace.ngram_sizes)
            self.store.extend(
                SEQUENCE,
                [[config, tag, stem, rep, n, g, tr.n_threads] for n, g in prof],
            )
        if self.s.trace.capture_args:
            rows = []
            for sc, counter in tr.args.items():
                for arg, n in counter.items():
                    rows.append([config, tag, stem, rep, sc, arg, n])
            self.store.extend(ARGS, rows)

    # -- passes ------------------------------------------------------------

    def _bench_pass(
        self,
        config: str,
        comp: str,
        programs: list[Program],
        state: ResumeState,
        out_dir: Path,
    ) -> dict[str, Path]:
        """Timed compile + verified run. Returns surviving binaries by stem."""
        built: dict[str, Path] = {}
        failed: set[str] = set()
        label = self.s.label(comp)

        for rep in range(1, self.s.bench.reps + 1):
            ok_n = fail_n = skip_n = 0
            for prog in programs:
                if prog.stem in failed:
                    continue
                if (config, comp, prog.stem, rep) in state.bench_done:
                    skip_n += 1
                    continue

                out = out_dir / prog.stem
                cr = compile_one(self.s, self.s.compiler(comp), config, prog, out)
                if not cr.ok:
                    fail_n += 1
                    failed.add(prog.stem)
                    built.pop(prog.stem, None)
                    if rep == 1:
                        self._err(config, comp, prog.stem, "compile", cr.error)
                        self.stats.compile_failed += 1
                    continue

                er = run_binary(self.s, out, prog)
                if not er.verified and not self.s.bench.keep_unverified:
                    self._err(
                        config,
                        comp,
                        prog.stem,
                        "verify",
                        er.detail or "output mismatch",
                    )
                    self.stats.unverified += 1
                    failed.add(prog.stem)
                    built.pop(prog.stem, None)
                    continue

                self.store.add(
                    BENCH,
                    [
                        config,
                        comp,
                        prog.stem,
                        rep,
                        round(cr.wall_s, 6),
                        round(er.wall_s, 6),
                        cr.size_b,
                        bool(er.verified),
                        int(er.returncode),
                    ],
                )
                built[prog.stem] = out
                ok_n += 1
                self.stats.compiled += 1
                self._maybe_flush(ok_n)

            self.store.flush()
            self._log(
                f"  [{label}/{config}] bench rep {rep}/{self.s.bench.reps}: "
                f"built={ok_n} failed={fail_n} skipped={skip_n}"
            )

        return built

    def _ensure_binaries(
        self,
        config: str,
        comp: str,
        programs: list[Program],
        need: set[str],
        built: dict[str, Path],
        out_dir: Path,
    ) -> None:
        """Recompile (untimed) any binary needed for tracing but not on disk.

        Happens on resume, when the bench rows were already recorded and the
        binaries were cleaned up.
        """
        rebuilt = 0
        by_stem = {p.stem: p for p in programs}
        for stem in sorted(need):
            if stem in built and built[stem].is_file():
                continue
            prog = by_stem.get(stem)
            if prog is None:
                continue
            out = out_dir / stem
            cr = compile_one(self.s, self.s.compiler(comp), config, prog, out)
            if cr.ok:
                built[stem] = out
                rebuilt += 1
            else:
                self._err(config, comp, stem, "recompile", cr.error)
        if rebuilt:
            self._log(
                f"  [{self.s.label(comp)}/{config}] recompiled {rebuilt} binaries "
                "(untimed) for missing trace reps"
            )

    def _elf_pass(
        self,
        config: str,
        comp: str,
        built: dict[str, Path],
        state: ResumeState,
    ) -> None:
        """Record static layout facts once per binary."""
        n = 0
        for stem, path in built.items():
            if (config, comp, stem) in state.elf_done:
                continue
            if not path.is_file():
                continue
            f = elf_facts(path)
            self.store.add(
                ELF,
                [
                    config,
                    comp,
                    stem,
                    f.size_b,
                    f.n_segments,
                    f.n_sections,
                    f.interp,
                    f.n_dynamic_needed,
                    f.probe_ok,
                ],
            )
            if f.probe_ok:
                state.elf_done.add((config, comp, stem))
            n += 1
            self._maybe_flush(n)
        self.store.flush()

    def _trace_pass(
        self,
        config: str,
        tag: str,
        built: dict[str, Path],
        state: ResumeState,
    ) -> None:
        """Trace each binary up to ``trace.reps`` under the given tag."""
        todo = {}
        for stem in built:
            done = state.trace_reps.get((config, tag, stem), 0)
            if done < self.s.trace.reps:
                todo[stem] = done + 1

        if not todo:
            self._log(f"  [{tag}/{config}] tracing: nothing to do")
            return

        n_runs = sum(self.s.trace.reps - (r - 1) for r in todo.values())
        self._log(
            f"  [{tag}/{config}] tracing: {len(todo)} binaries, {n_runs} runs ..."
        )

        done_bins = 0
        for stem, first_rep in todo.items():
            binary = built[stem]
            for rep in range(first_rep, self.s.trace.reps + 1):
                tr = trace_once(self.s, binary, self.norm)
                if not tr.ok:
                    self.stats.trace_failed += 1
                    self._err(config, tag, stem, "trace", tr.error)
                    continue
                self._record_trace(config, tag, stem, rep, tr)
                self.stats.traced += 1
            done_bins += 1
            if done_bins % self.s.flush_every == 0:
                self.store.flush()
                self._log(f"    ... {done_bins}/{len(todo)} binaries")
        self.store.flush()

    def _build_pass(
        self,
        config: str,
        build: int,
        programs: list[Program],
        state: ResumeState,
    ) -> None:
        """One extra build of the baseline, traced under every pass tag.

        The binary must be distinct from the ``A`` one, otherwise this
        degenerates into another rerun and measures nothing new -- hence the
        separate output directory per build.

        All passes of this build trace the *same* binary: differences between
        them are run noise, differences against another build's passes are build
        noise. That separation is the whole reason the controls are nested.
        """
        base = self.s.baseline
        tags = self.s.pass_tags(build)
        out_dir = self._bin_dir(tags[0], config)
        out_dir.mkdir(parents=True, exist_ok=True)

        needed = [
            p
            for p in programs
            if any(
                state.trace_reps.get((config, t, p.stem), 0) < self.s.trace.reps
                for t in tags
            )
        ]
        if not needed:
            self._log(f"  [build {build}/{config}] nothing to do")
            return

        self._log(
            f"  [build {build}/{config}] recompiling {len(needed)} programs with "
            f"{self.s.compiler(base).label} into fresh binaries ..."
        )
        built: dict[str, Path] = {}
        for prog in needed:
            out = out_dir / prog.stem
            cr = compile_one(self.s, self.s.compiler(base), config, prog, out)
            if cr.ok:
                built[prog.stem] = out
            else:
                self._err(config, tags[0], prog.stem, "compile", cr.error)

        # Layout facts belong to the build, not to a trace session, so they are
        # recorded once against its primary tag.
        self._elf_pass(config, tags[0], built, state)
        for tag in tags:
            self._trace_pass(config, tag, built, state)

        if not self.s.paths.keep_binaries:
            shutil.rmtree(out_dir, ignore_errors=True)

    # -- entry point -------------------------------------------------------

    def run(self, start: str | None = None) -> SweepStats:
        programs = load_corpus(self.s.corpus_path, self.s.languages, start=start)
        self._log(f"Corpus: {len(programs)} programs from {self.s.corpus_path}")

        # Source facts, once per file rather than once per (config, compiler,
        # file). Re-emitted on every sweep; `keep="last"` deduplication in the
        # aggregate layer makes that idempotent.
        self.store.extend(CORPUS, [[p.stem, p.sloc] for p in programs])
        self.store.flush()

        state = self.store.resume_state()
        if state.bench_done or state.trace_reps:
            self._log(
                f"[resume] {len(state.bench_done)} bench rows, "
                f"{len(state.trace_reps)} traced keys already present; skipping them."
            )

        for config in self.s.config_names:
            comps = self.s.compilers_for(config)
            self._log(
                f"\n=== config {config} | "
                f"{', '.join(self.s.label(c) for c in comps)} ==="
            )

            for comp in comps:
                out_dir = self._bin_dir(comp, config)
                out_dir.mkdir(parents=True, exist_ok=True)

                built = self._bench_pass(config, comp, programs, state, out_dir)

                # A binary is needed if it still owes trace reps *or* layout
                # facts. Without the second clause a store whose traces are
                # complete but whose ELF pass was skipped (no readelf at the
                # time) could never be repaired: nothing would rebuild the
                # binary for the probe to read.
                need = {
                    p.stem
                    for p in programs
                    if state.trace_reps.get((config, comp, p.stem), 0)
                    < self.s.trace.reps
                    or (config, comp, p.stem) not in state.elf_done
                }
                self._ensure_binaries(config, comp, programs, need, built, out_dir)
                self._elf_pass(config, comp, built, state)
                self._trace_pass(config, comp, built, state)

                # The rerun controls for build 0 trace the binaries already in
                # hand -- that is what makes them reruns rather than rebuilds.
                if comp == self.s.baseline:
                    for tag in self.s.rerun_tags(0):
                        self._trace_pass(config, tag, built, state)

                if not self.s.paths.keep_binaries:
                    shutil.rmtree(out_dir, ignore_errors=True)

            for build in range(1, self.s.controls.builds):
                self._build_pass(config, build, programs, state)

        self.store.flush()
        return self.stats
