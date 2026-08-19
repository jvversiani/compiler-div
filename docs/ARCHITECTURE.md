# Architecture

## Layout

```
src/compilerdiv/
├── config.py           Settings dataclass tree; YAML loading; fingerprint
├── corpus.py           Program discovery, single-language guard,
│                       expected-output parsing, SLOC
├── analyze.py          Analysis orchestrator
├── report.py           Workbook + terminal writers
├── acquire/
│   ├── trace.py        strace invocation; parsing to counts/sequences/args
│   ├── build.py        compile, verified run, ELF facts
│   ├── preflight.py    fail-fast checks
│   └── sweep.py        orchestration only
├── store/
│   ├── raw.py          append-only store, resume state, fingerprint guard
│   └── aggregate.py    raw -> tidy frames; the "stable" concept
├── stats/
│   ├── detectors.py    the five detectors
│   ├── noise.py        run / build noise floors from the control grid
│   ├── taxonomy.py     classification + layout probe
│   ├── equivalence.py  per-file verdicts, significance, JS (descriptive)
│   └── bench.py        geomeans, fold-change
├── plots/
│   ├── theme.py        PlotContext, colours, save helpers
│   ├── behavior.py     equivalence, taxonomy, matrix, probe, volcano, JS
│   └── bench.py        size/compile/exec + SLOC density
└── cli/main.py         argparse entry point
```

## Data flow

```
corpus/*            (one language, whichever the config declares)
    │              validate_corpus: >1 extension -> MixedCorpusError, abort
    │
    ├─ acquire ──────────────────────────────────────────┐
    │   compile_one → run_binary (verified) → elf_facts   │
    │   trace_once × reps × {A, A@b.p controls, B, ...}   │
    │                                                     ▼
    │                                            results/raw/  ← the contract
    │                                            counts/ sequence/ args/
    │                                            bench/ elf/ errors/
    │                                            manifest.json
    │                                                     │
    └─ analyze ◄──────────────────────────────────────────┘
        Frames (tidy, lazily cached)
          ├─ derive_noise      → NoiseFloor (run_noisy, build_noisy)
          ├─ all_departures    → set | count | argument | sequence
          ├─ classify          → taxonomy + layout probe
          ├─ per_file_verdict  → equivalent / differing
          └─ bench.summarize   → geomeans, fold-change
              ↓
        results/*.xlsx + results/figures/
```

The raw store is the **only** boundary. `acquire` writes and never reads except
to resume. `analyze` reads and never writes. That is what lets you iterate on
thresholds without re-running hours of sweep.

## Key design decisions

### The `stable` concept

Defined once, in `store/aggregate.py`, used by three detectors: a fact is stable
for a (compiler, file) when it holds in **every** rep. For sets that means
intersecting across reps; for counts, mean plus dispersion.

This is what makes the detectors self-calibrating on threaded programs. Nothing
needs to know which programs are threaded.

It is also lossy, and `detect_instability` exists to recover what it drops. The
intersection turns "flickered in four reps of five" into the same thing as
"never happened", which is why the two directions of `detect_set` use different
notions of the other side: a *presence* claim reads the stable set, an *absence*
claim reads the observed set (`_observed_sets`). Comparing two stable sets reported a variant that makes a call more often than the
baseline as having lost it.

Where the variant's flicker meets a baseline that is reproducible on that exact
`(file, syscall)`, the flicker is the finding rather than the noise, and
`detect_instability` reports it. That detector alone takes the **unpooled**
frames (`PooledBaseline.unpooled`): it asks whether the baseline agrees with
itself across control groups, and pooling is the step that hides a group that
did not.

### Format-agnostic storage

Parquet when an engine is importable, CSV otherwise. Part files are named
`part-<epoch_ns>-<rand>` so lexicographic sort equals write order, which makes
the `keep="last"` dedup deterministic when a crash-and-resume produces duplicate
keys.

## Extension points

### Add a detector

1. Write `detect_x(frames_thing, config, baseline, variant, ...) -> DataFrame`
   in `stats/detectors.py`, returning `DEPARTURE_COLUMNS`.
2. Add it to `all_departures`. Decide where it sits in the subsumption order:
   `detect_set` runs first and its findings are withheld from the others, so a
   detector that would restate "this syscall type is new" must be wrapped in
   `_drop_subsumed` (rows keyed by `(file, syscall)`) or filter on `explained`
   itself (anything coarser, as `detect_sequence` does per n-gram).
3. Add a `KIND_X` constant and a colour in `plots/theme.py`, plus entries in
   `plots/behavior.py` (`KINDS`, `KIND_NCOL`, and the sharpest-detector chain in
   `plot_equivalence`).
4. Add an `n_x` column in `equivalence.per_file_verdict`.
5. In `report.py`: a `_DIRECTION` entry per magnitude sign, membership in
   `_QUANTITATIVE` if `magnitude` is a real quantity, the `n_x` column in
   `PER_FILE_SHEET_COLUMNS`, and a `GLOSSARY` line for it.
   `src/tests/test_report.py` asserts the glossary covers every column exactly.
6. In `taxonomy._classify_one`, decide whether the corpus-wide noise class
   applies. It does not for a detector carrying its own per-file instrument
   (`count` and `instability` both opt out); a global "flaky somewhere" verdict
   must not overrule a direct measurement saying it was not flaky here.

The taxonomy body and the plots otherwise consume `kind` generically.

### Add a raw stream

1. Add the name and schema to `SCHEMAS` in `store/raw.py`.
2. Emit rows from `Sweeper._record_trace` or a new pass.
3. Add an aggregator in `store/aggregate.py` and a `Frames` property.

### Change what the taxonomy considers "program size"

`_classify_one` correlates `|magnitude|` against `sloc` from the ELF frame.
Substitute another column (`size_b`, a runtime instruction count) there.
