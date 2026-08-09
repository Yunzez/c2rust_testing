# E3 Master Table — per-function hit-DEPTH (rows = library, cols = tool)

**The paper's Table 3 — the credibility backstop for E1.** Same shape as E1/E2: **rows = the 10 E1
libraries, columns = the 6 shipped translators.** Each cell asks one question about that tool's actual
Rust translation of that library: **when the tool declared this code "done" (compiles / passes its own
check), how many times had each function actually been executed — vs how deep our fuzzing drives it?**

## The metric (locked 2026-07-10)

**Cell = `theirs / ours`**, where each number is a **per-function execution count summarised as the
median** (with the **min** reported alongside as the strongest line — "even the *least*-exercised
function was hit ≥ min times"):

- **`ours`** = median per-function entry count after a coverage-guided libFuzzer run per cell, obtained
  by replaying the grown corpus through a `-C instrument-coverage` build and reading each function's
  `count` via `llvm-cov export`. Reported as a **lower bound** (corpus-replay counts the saved corpus,
  not every one of the run's executions — see caveat).
  **Two medians are reported per cell and they must not be conflated:**
  - `median_all` — median over **all** censused functions, unreached ones counted as 0. **This is the
    honest headline number**; it is what a claim of the form "ours ≫ theirs" has to be measured on.
  - `median_reached` — median over functions with count > 0, reported **alongside the reach fraction
    `n_reached / n_functions`**, and only meaningful together with it.

  In **9 of 33 cells `median_all` is 0 — identical to `theirs`** (bzip2 ×3, cJSON ×2, lodepng ×2,
  optipng ×2). Those cells say *"our fuzzing reaches part of the API surface very deeply and never
  reaches the rest"* — a real finding, **not** a depth advantage. Multi-API-surface libraries (a PNG
  codec's encoder half, bzip2's file-I/O layer) are where this bites.
- **`theirs`** = the per-function execution count under the tool's *own* acceptance criterion. **For
  almost every cell this is 0 by construction** (see the their-side table below) — the tools accept a
  translation on *compiles / fewer-unsafe / cargo-check*, which executes nothing. So the typical cell is
  literally **`0 / 1487`**. This is a statement about the *acceptance criterion*, not a measured contrast
  between two testing campaigns: `theirs = 0` holds **by construction** of `cargo check`.

**Baseline honesty — `theirs = 0` is a definition, not a measurement.** A reviewer will (correctly) read
"median 27,043 vs 0" as a strawman if `0` is left to stand alone, because nobody claims `cargo check`
executes code. The defensible framing is narrow: *the tools ship translations whose acceptance evidence
contains zero executions of the translated function*. The comparison that would carry real weight — and
which E3 does **not** currently make — is against **the library's own shipped test suite** run on the
translated Rust (bzip2, cJSON, lodepng, lil all ship tests). Until that is measured, E3 claims *"the
acceptance criterion carries no execution evidence"*, **not** *"we test N× harder than they do."*

**What E3 claims:** C→Rust translations are accepted with **near-zero per-function execution evidence** —
the tools' acceptance criteria execute the translated code 0 times — while coverage-guided fuzzing reaches
a substantial part of the same API surface O(10³–10⁸)× deep. It is a statement about how little evidence
the acceptance criterion carries; it is the paper's credibility backstop, not its headline.

**What E3 does NOT claim:**
- **Not** "X% correct" — depth bounds *scrutiny*, not correctness.
- **Not** a matcher demonstration — plain fuzzing reaches the same code.
- **Not a causal explanation of the E1 bugs.** An earlier revision said the depth gap is "*why* the E1
  bugs sit in functions the tools had passed." That inference does not hold: `theirs = 0` is constant
  across every cell — the 20 bug cells *and* the 16 certificate cells alike — and a constant cannot
  explain the variation between them. The honest statement is weaker: the acceptance criterion supplies
  no per-function execution evidence **either way**, which is what makes silent defects survivable in
  principle; it does not predict *which* cells contain them.

## Cell legend

| mark | meaning |
|---|---|
| `theirs / ours` | median per-fn execution count under the tool's own acceptance **/** under our fuzz run. `median_all` (zeros included) is the headline; `median_reached` only ever with its reach fraction; `ours` is a lower bound |
| `∅` | **runnable artifact — queued to run** (our work list) |
| `∅★` | queued **and this cell holds a confirmed E1 bug** — run first (the punchline: a bug in a function executed 0× by the tool) |
| `⊘` | urlparser — the transpiled Rust inherits the C `url_parse` heap-overflow, so it ASan-aborts near entry; depth would be shallow up-to-crash and uninformative. Low-value, deprioritised (could be run as a crash-cell if wanted) |
| `▽` | tool's rewrite surface is minimal (CROWN×tulip) — nothing depth-informative |
| `—` | no runnable Rust (inherits E1's `✗`/`—` — the tool never produced a testable artifact) |

**Pure-Rust measurement — the C oracle is not used.** Depth is a Rust-side quantity (how deep our fuzz
drives the *translated* functions); `llvm-cov` counts the Rust crate only. Differential comparison
against C is an **E1** concern (finding divergences) — E3 needs no oracle. We therefore **reuse the
existing E1 harnesses as-is**: their C side simply goes unread for depth (or is dropped). Input legality
comes from each harness's **call-contract** (e.g. qsort's `low=0, high=len−1`), not a C-side UB gate.

**Harness reuse (no rebuild).** The per-signature byte→parameter decoder is the hard part and already
exists:
- **Ready cargo-fuzz (coverage-guided) harnesses:** `fuzz/qsort_example` (c2rust), `fuzz/qsort_c2saferrust`
  (bug cell, validated), `fuzz/lil_coverage`, `fuzz/urlparser_example` — run directly.
- **Bug-cell drivers already written** (reshaped ABIs solved during E1): `rq1_bugs/{cjson_ptrtrans,
  qsort_ptrtrans,bzip2_crown,optipng_laertes,utf8_panic_c2saferrust}/…driver/diff.rs`.
- **Generator + rundiff for the rest:** `tools/stu_selector/gen_diff_harness.py` emits a cargo-fuzz
  project for supported signatures; `laertes_benchmarks/*/rundiff.rs` already encode each library's call
  contract (lift the byte→param logic into a libFuzzer target).

Runner: `scripts/eval_rq3_depth.py` (locate-or-generate harness → build instrumented → 1 h `cargo fuzz
run` fork-mode → `cargo fuzz coverage` / per-process replay for crashing cells → `llvm-cov export` per-fn
`count` → median+min → delete the ~4.6 GB target, keep corpus + JSON). Validated on qsort×C2SaferRust
(below).

## The matrix (fillable state derived from E1 final) — ⚠️ HISTORICAL PLANNING TABLE

> **This is the 2026-07-10 *work plan*, kept for provenance. Every `∅` below has since been run.**
> **The actual results are in the final section of this document — read that one instead.**

| library | domain | ~#fn | c2rust | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans |
|---|---|---:|---|---|---|---|---|---|
| **qsort** | sorting | 3 | ∅ | ∅ | ∅★ | ∅ | ∅ | ∅★ |
| **urlparser** | URL parsing | 21 | ⊘ | ⊘ | ⊘ | ⊘ | — | — |
| **quadtree** | spatial tree | 24 | ∅ | — | — | ∅ | — | ∅ |
| **genann** | neural net | ~20 | ∅ | ∅ | ∅ | ∅ | ∅★ | — |
| **cJSON** | JSON parser | 58 | ∅ | — | ⊘ | — | — | ∅★ |
| **lil** | interpreter | 145 | ∅ | ∅ | ∅★ | ∅ | — | — |
| **lodepng** | PNG codec | 235 | ∅ | — | — | ∅ | — | — |
| **bzip2** | compressor | 64 | ∅ | ∅★ | ∅★ | ∅★ | — | — |
| **tulipindicators** | indicators | ~100 | ∅ | ∅ | ∅★ | ▽ | — | — |
| **optipng** | PNG optimizer | ~400 | ∅ | ∅★ | ∅★ | — | — | — |

**~32 fillable cells** (∅/∅★), **11 of them bug cells** (∅★). The `—`/`⊘`/`▽` cells inherit E1's
outcome — a sparse column is itself a finding (**SACTOR has only 2 runnable cells; that its per-function
verification can't even produce testable output is the E1 story, restated**). E3 is not "empty where
blank" — it is exactly as full as each tool's translations are runnable.

### Run order (value-first, serial — user's call 2026-07-10)
1. **The 11 bug cells first** — each is the money shot `0 / <deep>`: qsort×{C2SaferRust, PtrTrans},
   cJSON×PtrTrans, genann×SACTOR, lil×C2SaferRust, bzip2×{Laertes, C2SaferRust, CROWN}, tulip×C2SaferRust,
   optipng×{Laertes, C2SaferRust}.
2. **Then the certificate cells, small→large library** (qsort → quadtree → genann → lil → lodepng →
   bzip2 → tulip → optipng), reusing each library's harness (C-compare unread) across its tool columns.

## The "theirs" side — mostly 0, documented once

| tool | acceptance criterion | per-fn executions at acceptance |
|---|---|---|
| **c2rust** | mechanical transpile (no validation) | **0** |
| **Laertes** | compiles + fewer-unsafe | **0** |
| **C2SaferRust** | compiles + fewer-unsafe | **0** |
| **CROWN** | compiles + ownership-lift, unsafe reduced | **0** |
| **PtrTrans** | passes its own `cargo check` gate | **0** (check ≠ run) |
| **SACTOR** | per-function FFI test vs C | **O(1)** — e.g. genann embedded tests hit each fn a handful of times |

So `theirs = 0` for every cell except the SACTOR column, where it is a small constant (and genann×SACTOR
is *still* a bug cell — 100% wrong under those O(1) tests, the sharpest `O(1) / deep` contrast).

## Method caveats (reviewer-facing, stated up front)

1. **`ours` is a lower bound.** `llvm-cov`'s per-fn `count` is over the *replayed final corpus*, not the
   run's full execution stream (qsort: 36-file corpus → quickSort 1487, but the run itself executed
   ~9.5M times). We report the corpus-replay median as a floor; against `theirs = 0` a floor is plenty.
2. **Median, not mean.** Hot leaf functions (loop-called) have heavy-tailed counts (qsort: swap 4211 vs
   partition 726); the mean is meaningless, the median honest, the **min** the strongest defensible line.
3. **The execution budget is NOT uniform across cells, so counts are NOT comparable cell-to-cell.**
   Cells ran with libFuzzer `-runs=N` (not `-max_total_time`, which `_Exit()`s and skips the profraw
   flush), and `N` was set per cell to fit the library's speed and the wall-clock available: **N ranges
   from 4,000 (optipng×Laertes) to 2,000,000 (qsort×PtrTrans, genann×c2rust) — a 500× spread.** Each
   cell records its own `runs_budget`.
   - **Valid:** `ours` vs `theirs` *within* a cell, and tool-vs-tool *within one library* where N was
     held fixed — which it was for the four **tulip** cells (all 30,000) and **genann** (10⁶, c2rust 2×10⁶).
   - **Invalid:** raw magnitudes *across* libraries.
   An earlier revision claimed a "uniform 1-hour budget per cell" and used it to justify cross-cell
   comparability. That claim was **false** — the protocol migrated from time-based to runs-based partway
   through the campaign (for the flush reason above) and the prose was never updated. Cross-cell
   magnitude claims that depended on it have been removed.
4. **ASan is OFF for E3** (`--sanitizer=none`). ASan is E1's *crash oracle* (it turns silent OOB into an
   abort to find bugs); E3 only measures depth, so we disable it — faster, and it lets the corpus grow
   past OOB-read-into-mapped-memory bugs that ASan would have aborted. What still crashes without ASan =
   genuine hard faults (stack overflow from non-termination, wild-pointer SIGSEGV, heap corruption);
   those inputs are unavoidable and handled by (5).
5. **Crash cells** (a minority — most bugs are silent: checksum/reshaping/parse bugs don't fault) are
   censused by **per-process replay of the real fuzzer corpus, merging survivors** → `ours` is a lower
   bound. Framing: on a hard-fault cell our fuzz *crashed the translation the tool shipped* — the
   tool's acceptance had executed that function **0 times**, so a truncated-but-nonzero depth over
   0 is already the whole point (empirically confirmed: qsort×C2SaferRust median 921 / min 454 vs 0,
   with 22/36 inputs SIGSEGV-ing even ASan-off — the int→usize non-termination is a genuine hard fault).

## Validated prototype — qsort × C2SaferRust (the buggy WIP)

Real cargo-fuzz harness (`fuzz/qsort_c2saferrust/`, C-compare unread for depth): 1-h fork-mode run
drives the translated crate (and en route hits the ASan heap-overflow = E1 bug #1), and per-function
depth over the real corpus gives
**swap 4211 / partition 726 / quickSort 1487** vs the tool's **0** — cell reads `0 / 726` (median). This
anchors the pipeline end-to-end; the runner generalises it across the ∅ cells above.

## E3 results table (COMPLETE 2026-07-18; medians corrected 2026-08-09)

**Cell format: `median_all` · `median_reached` r `[reached/total]`, plus that cell's `-runs` budget.**

- **`median_all`** (bold when 0) — median per-function execution count over **all** censused functions,
  unreached counted as 0. **The headline number.**
- **`median_reached` r** — median over reached functions only (the previous convention). Not
  interpretable without the bracket beside it.
- `[reached/total]` — how much of the censused API surface the fuzzing actually executed.
- `N=` — that cell's `-runs` budget. **Budgets differ by up to 500×; magnitudes are NOT comparable
  between libraries.** Comparable within a library where N is fixed (tulip all 30,000; genann 10⁶).
- *(fl)* = crash-cell corpus-replay floor · **CRASH-ALL** = faults on all/valid input (a finding, not a
  number) · `—` = no artifact (per E1) · `n/a` = `per_fn` not retained, so `median_all` needs a re-run;
  reach is >½ there, so it differs from `median_reached` by less than one order statistic.

| library | c2rust | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans |
|---|---|---|---|---|---|---|
| **qsort** | 41,439,382 · 41,439,382 r [3/3]<br><sub>N=1,000,000</sub> | 38,102 · 38,102 r [3/3] *(fl)*<br><sub>N=1,000,000</sub> | 153 · 153 r [3/3] *(fl)*<br><sub>N=1,000,000</sub> | 40,119,635 · 40,119,635 r [3/3]<br><sub>N=1,000,000</sub> | n/a · 49,183,812 r [6/8]<br><sub>N=1,000,000</sub> | 226,029,454 · 226,029,454 r [3/3]<br><sub>N=2,000,000</sub> |
| **urlparser** | — | — | — | — | — | — |
| **quadtree** | 178,753 · 178,753 r [16/17] *(fl)*<br><sub>N=1,000,000</sub> | — | — | 71,307 · 71,307 r [16/17] *(fl)*<br><sub>N=1,000,000</sub> | — | n/a · 127,103,674 r [18/19]<br><sub>N=100,000</sub> |
| **genann** | 1,606,774 · 3,213,548 r [7/11]<br><sub>N=2,000,000</sub> | 817,574 · 1,635,148 r [7/11]<br><sub>N=1,000,000</sub> | 818,835 · 1,637,670 r [7/11]<br><sub>N=1,000,000</sub> | 816,519 · 1,633,038 r [7/11]<br><sub>N=1,000,000</sub> | 6 · 15 r [6/9] *(fl)*<br><sub>N=1,000,000</sub> | — |
| **cjson** | **0** · 220 r [3/37] *(fl)*<br><sub>N=1,000,000</sub> | — | — | — | — | **0** · 3,868,007 r [6/121]<br><sub>N=1,000,000</sub> |
| **lil** | 164 · 389 r [38/51] *(fl)*<br><sub>N=1,000,000</sub> | 173 · 480 r [35/51] *(fl)*<br><sub>N=1,000,000</sub> | **CRASH-ALL** | 158 · 344 r [34/43] *(fl)*<br><sub>N=1,000,000</sub> | — | — |
| **lodepng** | **0** · 1,000,000 r [25/75]<br><sub>N=1,000,000</sub> | — | — | **0** · 1,000,000 r [23/75]<br><sub>N=1,000,000</sub> | — | — |
| **bzip2** | **0** · 99,463 r [16/35]<br><sub>N=100,000</sub> | **0** · 99,657 r [16/35]<br><sub>N=100,000</sub> | **CRASH-ALL** | **0** · 3 r [9/35] *(fl)*<br><sub>N=100,000</sub> | — | — |
| **tulip** | 27,043 · 27,043 r [173/224]<br><sub>N=30,000</sub> | 27,272 · 27,272 r [173/224]<br><sub>N=30,000</sub> | 27,068 · 27,068 r [173/224]<br><sub>N=30,000</sub> | 27,138 · 27,138 r [173/224]<br><sub>N=30,000</sub> | — | — |
| **optipng** | **0** · 8,510 r [150/374]<br><sub>N=50,000</sub> | **0** · 247 r [33/374] *(fl)*<br><sub>N=4,000</sub> | **CRASH-ALL** | — | — | — |

**33 cells filled — fillable table COMPLETE** (every runnable translation measured).

### The honest reading

**In 9 of 33 cells `median_all` is 0 — the same value as `theirs`.** There, our fuzzing
drives *part* of the API surface very deep and never reaches the rest: bzip2 ×3 (16/35, file-I/O layer
unreached), cJSON ×2 (3/37, 6/121), lodepng ×2 (25/75, 23/75 — the entire encoder half unreached),
optipng ×2 (150/374, 33/374). That is a real finding about **where fuzzing goes**, and it is not a depth
advantage. Reporting only the reached-median, as the previous revision did, concealed it.

The cells carrying a genuine depth result are the high-reach ones: **tulip** (173/224 across all four
tools at fixed N=30,000, median ~27k — the cleanest tool-vs-tool comparison here), **genann** (7/11,
~0.8–1.6M), **qsort** (3/3), **quadtree** (16–18/19).

### Cross-cutting findings
- **CRASH-ALL trio** (lil/bzip2/optipng × C2SaferRust): C2SaferRust's idiomatic rewrites fault on
  all/valid input where the c2rust base runs to full depth — each corroborates an E1 bug *through
  execution depth* (lil c:1 = NonNull-from-null in hm_destroy; optipng c:1 s:2 = deadly signal on every
  PNG; bzip2 c:1 s:1). **Unaffected by the median correction** — these are qualitative outcomes.
- **qsort — the safety paradox**: on the same 3-function quicksort, **CROWN** (a dedicated safety-lifter)
  kept it **100% unsafe** (fatness analysis can't prove `arr.offset(j)` bounds → 0 pointers lifted,
  output ≡ c2rust), while **SACTOR** (LLM) produced **fully safe** `&mut [i32]` + `split_at_mut`. Both
  run at the same order of depth (~40–49M; 3/3 and 6/8 reached) as raw-pointer c2rust — **safety cost no
  execution depth.** Same library, same reach, opposite safety outcomes.
- **tulip = uniform full depth**: all 4 tools reach 173/224 at a **fixed** N=30,000, median ~27k. The one
  methodologically clean cross-tool comparison in the table — and a *negative* result: a well-behaved
  numeric library every tool translates soundly. Contrast with **lil** (3 crash-floor + 1 crash-all).
- **PtrTrans idiomatic never crashes where the base does**: quadtree×PtrTrans runs to full depth without
  faulting while c2rust's translation of the same library is a crash cell. *Stated qualitatively on
  purpose* — the previous revision compared 127M against 178,753, but those are a `total-exec` and a
  `corpus-replay-floor` number at different budgets (10⁵ vs 10⁶), so the ratio meant nothing. The
  crash-vs-no-crash contrast is the finding and needs no ratio.

### Open items
1. **`theirs = 0` is definitional.** Supplement it with the libraries' **own shipped test suites** run
   against the translated Rust (bzip2, cJSON, lodepng, lil all ship tests).
2. **Two cells (`qsort×SACTOR`, `quadtree×PtrTrans`) report `n/a` for `median_all`** — `recensus_mangled.py`
   did not retain `per_fn` and the `/tmp` llvm-cov caches were reclaimed. Re-run to close.
3. **Budgets are not uniform.** Either re-run at fixed N, or keep per-cell N visible (done) and make no
   cross-library magnitude claims (done).
