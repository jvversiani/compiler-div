# Interpreting the results

How to read the outputs, and what each one licenses you to say.

---

## Read in this order

1. **`figures/taxonomy/`**: is the `program_dependent` column empty?
2. **`figures/equivalence/`**: how many files differ, and which detector fired?
3. **`behavior.xlsx`, sheet `taxonomy`**: every signature with its evidence
4. **`figures/layout_probe/`**: for each `conditional` departure, does
   membership track binary layout?
5. **`figures/noise_floor/`**: what the controls found; is the floor plausible?

Everything else is supporting detail.

---

## Reading the taxonomy sheet

Columns, worst class first:

| column | what to do with it |
|---|---|
| `class` | the prior. Start with `program_dependent`. |
| `n_files_affected` / `n_files_corpus` | breadth. 672/672 is uniform; 59/672 wants explanation. |
| `magnitude_fixed` | `True` = every affected file shows the same delta. A clean offset. |
| `magnitude_values` | the actual deltas. `1` means `+1` everywhere. |
| `rho_vs_sloc` | does the delta scale with the program? |
| `layout_best_rho`, `layout_best_source` | does *membership* track binary layout, and which fact? |
| `layout_status` | why a rho is missing, when it is |
| `selfdiff_n_files`, `selfdiff_verdict` | did the baseline do this to *itself* across two builds? `SAME mechanism` means the finding is not cleanly attributable to the variant. |
| `affected_files` | the complete list, for your own follow-up |

### Worked example: a variant with `read(+1)`

A variant makes exactly one extra `read` than the
baseline, on part of the corpus:

```
syscall            read
kind               count
class              conditional
n_files_affected   59
n_files_corpus     672
magnitude_fixed    True
magnitude_values   1
rho_vs_sloc        (none -- magnitude is fixed, nothing to correlate)
layout_best_rho    ?           <- THE ANSWER
layout_best_source size
layout_status      ok
selfdiff_n_files   0           <- and the baseline never did this to itself
```

`layout_best_rho` is the number that settles it.

**Strong |rho| (> 0.5) with `layout_best_source: size`:** membership tracks binary size. The mechanism is
layout: the variant's different link layout shifts the length of a file the
runtime reads at startup (`/proc/self/maps` is the usual culprit), and for
binaries sitting near a read-chunk boundary it crosses into one more `read`.
Benign, and now demonstrated rather than asserted. You can write:

> The variant's binaries make one additional `read` at startup in 59 of 672
> programs. Membership in that set correlates with binary size (Spearman rho =
> X, p = Y), consistent with a startup read crossing a chunk boundary under a
> different link layout. The difference is attributable to linking, not to code
> generation.

**Weak |rho| (< 0.3) with `layout_status: ok`:** membership does *not* track
layout. The layout hypothesis is not supported and you need another explanation.
Look at `affected_files`: do they share a structural property (threads, file
I/O, networking)? That would point at semantics, which is a very different
finding.

**`layout_status` is not `ok`:** the probe could not compute. That is evidence of
nothing. Fix the cause (install `binutils` for `readelf`), then re-run `acquire`
followed by `analyze`. `analyze` alone is not enough: the layout facts are read
from the store, and a sweep that ran without `readelf` recorded size-only rows.
Those rows carry `probe_ok = false`, so `acquire` re-collects them (recompiling
the binaries it needs) instead of skipping them as already done.

### Worked example: a variant on a different runtime

A variant built against a different standard library or sysroot typically shows
two things at once: a syscall present in **every** file (say `gettid` at
672/672) → `uniform`, and others on a subset (`openat`/`newfstatat` at ~188/672)
→ `conditional`, with the same layout question as above.

Resist the urge to add the uniform one to `excluded_syscalls` because it "is
obviously just startup". Excluding it makes the corpus look more identical than
it is, and removes the very evidence for the claim you would want to make.
`uniform` is where such a difference belongs: a constant startup fingerprint of
the variant's runtime, reported as such.

---

## Reading the equivalence figure

Each differing file is attributed to its **sharpest** detector, so the bars sum
to the number of differing files rather than double-counting:

- **set** (red): a syscall type A never makes. Strongest signal.
- **argument** (orange): a value A never passes. Strong.
- **sequence** (purple): an order A never produces. Medium.
- **instability** (brown): A is reproducible here, B is not. Medium.
- **count** (blue): more of something A already does. Weakest.

`equivalent` means **no detector fired**: not merely that mean counts landed
within some tolerance. A tolerance-based test silently absolves any file whose
noise happens to exceed the delta; requiring every detector to stay silent does
not.

---

## Why one change is one row: set subsumption

The detectors are independent, so a single behavioural change trips as many of
them as it structurally can. A syscall type the baseline never makes is also
`+N` of that syscall, also sits inside n-grams the baseline never produced, and also passes values the baseline never passed. Left
alone, `gettid` under `rustc_gcc` reported three times per file.

So the set detector **subsumes** the others. Where it fires for a
`(file, syscall)`, the count, argument and instability detectors stay silent
there, and the sequence detector drops the n-grams containing that syscall at
n-gram granularity, so a file whose reordering is *also* genuine still reports
the part the set row does not explain.

The count detector subsumes `instability` in turn: where the variant's mean
shifted far enough to clear a floor *already widened by its own jitter*, the
shift is the finding and the jitter is a detail of it.
What this does and does not mean when reading the `departures` sheet:

- A `count` row now always means **more of something the baseline already
  does**. Ask "is it new?" of the `set` rows.
- Departure totals are no longer inflated by structurally-implied duplicates,
  so they are comparable across variants. On the Rust corpus this took `CvA`
  from 2023 rows to 680 without removing a single finding: 672 `gettid` set
  rows stayed, and their 673 count and 670 sequence shadows went.
- Nothing is hidden. Every suppressed row is implied by a set row on the same
  file and syscall. If you want the raw per-detector view, the aggregated
  inputs are still in `results/raw`.

This makes the `departures` sheet agree with the equivalence figure above, which
already attributed each file to its sharpest detector alone.

---

## Reading an `instability` row

These say: **on this program the baseline was reproducible and the variant was
not.** They exist because the other four detectors structurally cannot say it.
A variant whose count for one syscall runs `1,3,4,2,5` while the baseline sits
at a constant `2` is not reported by the count detector and not by the set detector, which sees the syscall present on both sides.

Read the row like this:

| column | says |
|---|---|
| `direction` | always `unstable` |
| `delta` | span of the variant's counts, absent reps counted as zero |
| `evidence` | the rep sample on both sides, in words |
| `variant_sd` | non-zero by construction; the size of the wobble |

Two shapes reach this detector, and `evidence` distinguishes them:

- `baseline constant 2`: a syscall both compilers make, reproducibly under A
  and not under B.
- `baseline never emitted`: a syscall **new** to the variant that appears in
  only some reps. Silent everywhere else: too unstable for the set detector,
  usually too small for count. Worth a look, since a payload that fires
  intermittently lands here and nowhere else.

The bar for "the baseline is reproducible" is deliberately high.

### What is deliberately not here

**A flickers, B is steady** is not reported, and its absence
is not an oversight. It describes the instrument rather than the variant: a
`(file, syscall)` where the baseline is unreliable is one where *no* variant can
be adjudicated, for all variants at once. It also rests on far weaker evidence.
Instability needs one disagreeing rep to prove; steadiness needs a disagreement
never to appear, and the budget is lopsided: the baseline gets
`builds × passes × reps` traces to reveal its wobble while each variant gets
`trace.reps` to prove it has none. With `trace.reps: 3`, a variant that flakes
one run in five looks perfectly steady about half the time. "B is deterministic"
is not a claim that sample can carry.

---

## The noise floor figure

Two bars per syscall:

- **within a build** (grey): passes of the same binary. Run-to-run noise.
- **between builds** (orange): separate compilations. Build + run.

What to look for:

**Orange much taller than grey** → that syscall varies between builds of the
*same* compiler. Any variant difference in it is uninterpretable.

**`n_builds` of 2** → build variance is estimated from a single pair. That
detects a difference but cannot say whether it reproduces, and the terminal
flags it:

> [!] 2 builds estimates build variance from n=2: it detects a difference but
> cannot say whether it reproduces. Raise controls.builds to 3+.

**Between-builds bar absent** → you ran with `controls.builds: 1`. Without it, a
difference that is stable across reruns cannot be distinguished from build
nondeterminism in the baseline itself. Turn it on for the run you write up.

---

## What NOT to conclude

**Do not read the JS divergence plot as an oracle.** It ranks by program size.
The median is meaningless. It is in the outputs for comparability with results
reported that way elsewhere; the caveat is stamped on the figure.

**Do not say "the compilers are equivalent."** Say: *no departure was observed
above the measured noise floor, with the following detectors and the following
known blind spots.* The blind spots are real: argument capture is path-only,
the sequence oracle is weak on threaded programs, `unshare -Ur` changes the
environment.

**Do not treat `uniform` as proven benign.** It is *almost certainly*
toolchain startup, because a payload firing identically on an empty program
isn't doing anything program-specific. But "almost certainly" is the prior, not
a proof. You looked at it; say so.

**Do not call this DDC.** It is a behavioral surrogate.