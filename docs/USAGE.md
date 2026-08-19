# Usage

## Installation

```bash
git clone <repo> && cd compilerdiv
pip install -e ".[fast]"
```
---

## Commands

| command | what it does | cost |
|---|---|---|
| `compilerdiv preflight` | verify strace traces and every compiler builds a probe | seconds |
| `compilerdiv acquire` | the sweep: compile, verify, trace | **hours** |
| `compilerdiv analyze` | reports and figures from the raw log | seconds |
| `compilerdiv all` | acquire then analyze | hours |
| `compilerdiv corpus` | corpus stats and the SLOC figure | seconds |
| `compilerdiv doctor` | inspect the raw store | seconds |

All take `-c/--config` (default `compilerdiv.yaml`) and `-q/--quiet`.

`acquire` runs `preflight` automatically unless you pass `--skip-preflight`.

### acquire

```bash
compilerdiv acquire                      # resume if a raw log exists, else start
compilerdiv acquire --start Sorting_alg  # skip corpus entries sorted before this
compilerdiv acquire --reset              # DELETE the raw log and start over
compilerdiv acquire --skip-preflight     # you already ran it
```

Interrupting with Ctrl-C flushes the buffer and exits cleanly. Re-run `acquire`
to pick up where it stopped. A hard kill (SIGKILL, OOM, node failure) loses at
most `flush_every` binaries.

### analyze

Reads only the raw log. Re-run it freely: retuning `uniform_threshold` or
`program_dependent_rho` and re-plotting costs seconds and does **not** invalidate
the sweep (those settings are deliberately excluded from the fingerprint).

### Contributing

You should install this way:
```bash
pip install -e ".[dev]"     
```
If you are willing to contribute.

It is recommanded to use **make** instead of **compilerdiv** since it implements additional usefull commands (Check **make help**)

---

## Resume semantics

The raw store is append-only and keyed by `(config, compiler, file, rep)`.
`acquire` reads what is already recorded and skips it.

Binaries are recompiled **untimed** when they are needed for a missing trace rep
but their bench rows already exist. Timing is never re-recorded for work that was
already measured.

### The fingerprint guard

`manifest.json` stores a hash of every setting that affects the *contents* of the
log:

- compilers (keys, commands, cwd, arg order)
- configs (names, flags, unsupported pairs)
- `trace.reps`, `trace.wrapper`, `trace.capture_args`, `trace.arg_syscalls`,
  `trace.capture_sequence`, `trace.ngram_sizes`
- `bench.reps`, `bench.verify_output`
- `controls.builds`, `controls.passes`

Change any of these and `acquire` **refuses to resume**:

```
The existing raw log was acquired with different settings.
  stored:  235b2afc4b7594ce
  current: f9dba181e21e30cd
...
Either restore the old settings, or re-acquire from scratch with --reset
```

The guard is enforced rather than left to you to remember
Presentation and analysis settings (colours, labels, `count_tol`,
`uniform_threshold`, `program_dependent_rho`, `excluded_syscalls`,
`departure_ignore`) are **not** in the fingerprint. 

---

## The bundled Rust corpus

The package ships with a ready-made corpus at **`src/codebase/`**: **672 self-contained
Rust programs** (~63k lines) scraped from [Rosetta Code](https://rosettacode.org)'s
`Category:Rust`. It is what `compilerdiv.yaml` points at (`paths.corpus_dir:
src/codebase`), so a fresh clone can go straight to `preflight` without writing a single
program.

It is an **example corpus, not a fixed part of the tool.** `compilerdiv` is
language-agnostic; the Rust corpus is here because the study that produced the tool
compared three Rust backends (`rustc`, `mrustc`, `rustc_codegen_gcc`). Point
`corpus_dir` at your own directory of `.c`, `.go`, or `.rs` files and nothing else
changes.

Every file already obeys the rules in [Adding programs to the
corpus](#adding-programs-to-the-corpus): one file, one `main`, `std` only, no stdin, no
argv, terminates on its own, and carries the `Expected output:` header that drives
`verify_output`. All 672 have a usable expected-output block, so the whole corpus is
checkable rather than merely runnable.

```bash
compilerdiv corpus     # program count, SLOC distribution, how many are checkable
```

**Licensing:** `compilerdiv` itself is MIT, but the corpus files are derived from Rosetta
Code and are licensed **GFDL 1.2**. Each file keeps the task name, source URL, and
license line in its header, preserve that block if you edit or redistribute one.

---

## Adding programs to the corpus

A corpus entry is one source file dropped into `paths.corpus_dir`. The measurement only means
something if the program obeys the rules below, because every program is
compiled by *N* toolchains, run repeatedly, and traced repeatedly, and anything
that makes two runs differ for reasons of its own shows up as compiler diversity.

### One file, one program, one `main`

The compiler is invoked as a fixed argv prefix plus **one** source path. There is
no build system, no link step you control, no second translation unit. A program
that needs a companion module cannot be expressed here.

It must define the language's ordinary entry point (`fn main()` in Rust,
`int main(void)` in C, `func main()` in Go) and it must terminate on its own.
The file stem becomes the program's identity in every report, so keep it unique
and stable: renaming a file makes the store treat it as a new program and
orphans the old rows.

### One language per corpus

Every file in `corpus_dir` must share **one** extension. A corpus mixing `.rs`
and `.c` is rejected before any work happens:

```
error: corpus at src/codebase mixes 2 source extensions:
  .c     3 file(s)   e.g. hello.c
  .rs  672 file(s)   e.g. Abbreviations_1.rs
A run applies one fixed compiler argv to every program, so a mixed corpus would
record one language's compile failures as findings about the variant. ...
```

The rule is by extension, not by language name, so `.cpp` alongside `.cc` is
rejected too even though both are C++.
Unrecognised files are
ignored entirely, so a stray `README.md` or `Makefile` is harmless.

`preflight`, `acquire`, `corpus` and `analyze` all enforce it, and `analyze`
checks before it computes or writes anything.

### No input, no environment

The program must produce identical output on every run, from nothing but itself:

- **No stdin.** Nothing feeds it. a program that blocks on input hits
  `exec_timeout_s` and is recorded as a failure.
- **No argv.** It is executed with no arguments.
- **Nothing that varies run to run**: If bench.verify_output is activated, undeterministic output (Threads, hashmaps) will fail.

### External libraries: avoid them

Only what the toolchain provides without extra flags.

The measurement compares syscalls,
a shared library brings its own `openat`/`mmap` traffic and its own version
skew between toolchains, so a dependency difference reads as a compiler
difference. For Rust, for example: `std` only, no crates.

### Clean up what you create

It is better practice for the program created files to be deleted before exiting.

### Carry the expected-output header

The header **is** the integration test (see
[`verify_output`](#verify_output--the-embedded-integration-test)). Put it at the top of the file, in the language's line-comment
style:

```rust
// =======================
// Expected output:
// Hello, world!
// =======================

fn main() {
    println!("Hello, world!");
}
```

`Expected output:` and `=======================` are literal sentinels, shared
across all languages. Only the comment marker changes. Parsing starts at the line *after* `Expected
output:` and stops at the next `=======================`. Every line between
them is expected stdout, with the comment marker (and one following space)
stripped.

Verify a new program before committing to a sweep:

```bash
compilerdiv corpus            # is it discovered? is it counted as checkable?
```

`Without expected:` in that output lists the programs with no usable block.

---

## Configuration reference

### `baseline`

Compiler key to compare everything against. Must be one of the configured
compilers. A key may not contain `@`: that separator is reserved for the
generated control tags (`A@1.2` = build 1, rerun 2 of the baseline).

### `paths`

| key | default | meaning |
|---|---|---|
| `project_root` | `.` | resolved relative to the **config file** |
| `corpus_dir` | `src/codebase` | where the corpus source files are (**one extension only**) |
| `work_dir` | `build/compilerdiv_work` | scratch, binaries during the sweep |
| `raw_dir` | `results/raw` | the append-only store |
| `out_dir` | `results` | workbooks and figures |
| `keep_binaries` | `false` | leave binaries on disk for `readelf`/`nm` |

### `compilers`

```yaml
compilers:
  - key: A                      # short id used in the raw log
    label: reference            # shown in plots and reports
    cmd: [cc]                   # argv prefix
    color: "#4C72B0"
  - key: B
    label: variant
    cmd: ["./build.sh", "compile"]
    cwd: toolchains/variant     # relative to project_root
    src_first: true             # source immediately after cmd, not last
```

Any compiler invocable as an argv prefix works. The corpus language follows from
`languages` and the file extensions it maps.

`src_first` exists for drivers that expect the source immediately after the
subcommand rather than at the end of the argv.If a compiler builds fine by hand but fails under `preflight` with a
complaint about a missing project file, try `src_first: true`.

### `configs`

```yaml
configs:
  - name: basic
    flags: []
  - name: O2
    flags: ["-O2"]
  - name: static
    flags: ["-static"]
    unsupported: [B]            # this compiler cannot link statically
```

Only `basic` is enabled by default. Each config multiplies the sweep cost, and
adding one changes the fingerprint.

### `trace`

| key | default | meaning |
|---|---|---|
| `reps` | `3` | trace runs per (binary, tag). More reps = stricter stability. |
| `wrapper` | `["unshare", "-Ur"]` | prefix before strace. `[]` to disable. |
| `strace_bin` | `strace` | path or name |
| `timeout_s` | `600` | per-run ceiling |
| `capture_args` | `true` | enables the argument detector |
| `arg_syscalls` | path-taking calls | which syscalls get argument capture |
| `capture_sequence` | `true` | enables the n-gram detector |
| `ngram_sizes` | `[3, 4, 5]` | n-gram widths |

Keep `arg_syscalls` narrow. Capturing arguments for `mmap`, for example, produces
noise-dominated garbage.

`reps` interacts with every detector: a fact is stable only if it holds in all
of them. `reps: 3` is the practical floor.

### `bench`

| key | default | meaning |
|---|---|---|
| `reps` | `3` | compile+exec timing samples per file |
| `exec_timeout_s` | `300` | per-run ceiling |
| `verify_output` | `true` | the embedded integration test |
| `keep_unverified` | `false` | record failing rows anyway (marked `verified=False`) |

```yaml
bench:
  verify_output: true      # on by default
  keep_unverified: false
```

With it on, `acquire` does this to each binary, for every compiler and every
config, before any tracing happens:

1. Run it untraced, with a `bench.exec_timeout_s` ceiling.
2. **Nonzero exit ⇒ fail.** (This check is unconditional, `verify_output: false`
   does not disable it.)
3. Compare stdout line-by-line against the header's expected block, under
   whitespace normalisation: runs of whitespace collapse to one space, and both
   ends are stripped.
4. A program **without** a usable block passes by default. Absence of an oracle
   is not evidence of failure.

The comparison is a **prefix** check: every expected line must match, and stdout
must not be shorter than the block, but extra trailing output is tolerated.

### `controls`

```yaml
controls:
  builds: 3    # independent compilations of the same source
  passes: 3    # independent trace sessions per build
```

The baseline is measured as a **nested grid**, not as two flat control passes.
Group (0,0) is `A` itself, the rest carry generated tags `A@<build>.<pass>`.

| comparison | binary | what a difference means |
|---|---|---|
| between **passes** of one build | identical | run-to-run noise (ASLR, scheduler, DNS retry) |
| between **builds** | different | that, plus build nondeterminism (link layout, embedded paths) |

Both classes are therefore measured rather than inferred. Both are needed to
attribute a difference to a variant's toolchain.

`builds: 2` detects build variance at all. `builds: 3` or more is what says
whether it *reproduces*, and the per-file build floor
(`rebuild_delta`, the max over build pairs) only becomes a real sample of the
baseline's own variance at that point.

**Cost is the product.** `builds x passes x trace.reps` baseline traces per
program, plus `builds` compiles: going from 2x2 to 3x3 more than doubles the
baseline half of the sweep. `preflight` prints the whole budget before you
commit hours to it.

The pre-nesting spelling (`rerun: true` / `rebuild: false`) is still accepted
and maps onto `passes: 2` / `builds: 1`.

Legacy note: the old tag names `A2` and `Arebuild` are gone. They are now
`A@0.1` and `A@1.0`.

### `detect`

| key | default | meaning |
|---|---|---|
| `count_tol` | `0.5` | minimum count delta to consider |
| `auto_noisy_min_files` | `3` | files a syscall must differ in (in the controls) to be classed noise |
| `uniform_threshold` | `0.95` | fraction of corpus for `uniform` vs `conditional` |
| `program_dependent_rho` | `0.5` | \|Spearman rho\| vs SLOC for `program_dependent` |
| `excluded_syscalls` | `[]` | dropped from count stats entirely |
| `departure_ignore` | allocator/scheduler churn | excluded from the detectors only |
| `arg_normalizers` | pids, tmp, addresses | regex substitutions before argument comparison |

`excluded_syscalls` and `departure_ignore` are **different machinery** and must
not be confused:

- `excluded_syscalls` → dropped from count statistics
- `departure_ignore` → dropped from the departure detectors
- noise classes → derived from the controls, excluded from significance

A syscall can legitimately be in one and not another.

Be sparing with both. Excluding a syscall because it looks like startup noise
also removes the only evidence that would have shown it was not: a departure
you never measure cannot be classified, and the taxonomy exists precisely to
separate benign startup traffic from real divergence without discarding it
first.

---

## Troubleshooting

### `preflight: strace ran but produced no parseable syscalls`

ptrace is confined. Check:

```bash
cat /proc/sys/kernel/yama/ptrace_scope     # 0 is permissive
strace -f -c /bin/true                      # does it work at all?
unshare -Ur strace -f -c /bin/true          # does the namespace help?
```

If the namespace helps, keep `trace.wrapper: ["unshare", "-Ur"]`. If plain
strace works, set `wrapper: []`.

### A compiler fails preflight

The full stderr is printed. If a compiler builds by hand but fails here with a
complaint about a missing project manifest or entry-point file, its driver is
dispatching on argument position and never saw the source: set `src_first: true`
for that compiler. Check `cwd` too: it is resolved relative to `project_root`,
not to your shell.

### `no program_dependent departures` and I expected some

For the benign variants that is the result. If you injected a payload and
it landed in `conditional` instead, check `rho_vs_sloc` in
the `taxonomy` sheet of `behavior.xlsx`. A payload with a *fixed* cost (one `openat` regardless of
program) is genuinely not program-dependent and will class as `conditional`.

### Layout probe says `no usable layout variable`

`readelf` is missing, or the corpus has constant binary sizes. Install
`binutils`, then re-run `acquire` (not just `analyze`) so the size-only rows are
re-collected. The status string names which columns were constant.

### `corpus ... mixes N source extensions`

Exactly what it says: `corpus_dir` holds more than one recognised extension, and
one compiler argv cannot honestly build both. The message lists each extension,
its file count, and an example. Move the odd files out, or narrow `languages:`
to the one you want. See
[One language per corpus](#one-language-per-corpus).

### A program fails `verify` but runs fine by hand

Check, in order:

1. **Is the output actually stable?** Run it a dozen times and diff. Unseeded
   RNG, map iteration order, and timestamps are the usual causes.
2. **Is it leaving a file behind?** The second run sees a filesystem the first
   did not. See [Clean up what you create](#clean-up-what-you-create).
3. **Is the header stale?** The expected block must match the program's *current*
   output exactly (modulo whitespace runs).
4. **Is it architecture-dependent?** Pointer widths, `usize` formatting, and
   float rounding legitimately differ from whatever machine the block was
   recorded on. Fix the block, or fall back to the sentinel. Do not turn
   `verify_output` off for the whole sweep to accommodate one program.

The `errors` stream holds the first mismatch for every failure. `compilerdiv
doctor` shows the row counts.

### The store is huge

You are on the CSV fallback. `pip install pyarrow`.

### I want to inspect a binary by hand

Set `paths.keep_binaries: true`. Binaries stay under
`<work_dir>/<config>/<compiler>/`.
