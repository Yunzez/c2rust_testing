# E3 Master Table — per-function hit-DEPTH (rows = library, cols = tool)

**The paper's Table 3 — the credibility backstop for E1.** Same shape as E1/E2: **rows = the 10 E1
libraries, columns = the 6 shipped translators.** Each cell asks one question about that tool's actual
Rust translation of that library: **when the tool declared this code "done" (compiles / passes its own
check), how many times had each function actually been executed — vs how deep our fuzzing drives it?**

## The metric (locked 2026-07-10)

**Cell = `theirs / ours`**, where each number is a **per-function execution count summarised as the
median** (with the **min** reported alongside as the strongest line — "even the *least*-exercised
function was hit ≥ min times"):

- **`ours`** = median per-function entry count after a **uniform 1-hour coverage-guided libFuzzer run**
  per cell, obtained by replaying the grown corpus through a `-C instrument-coverage` build and reading
  each function's `count` via `llvm-cov export`. Reported as a **lower bound** (corpus-replay counts the
  saved corpus, not every one of the run's executions — see caveat).
- **`theirs`** = the per-function execution count under the tool's *own* acceptance criterion. **For
  almost every cell this is 0 by construction** (see the their-side table below) — the tools accept a
  translation on *compiles / fewer-unsafe / cargo-check*, which executes nothing. So the typical cell is
  literally **`0 / 1487`**. The point needs no statistics: **their validation ran the function zero
  times; ours ran it thousands.**

**What E3 claims (non-tautological):** current C→Rust translation is accepted with **near-zero
per-function execution evidence**, so bugs survive into shipped output; a uniform 1-hour coverage-guided
fuzz exercises every function O(10³–10⁵)× deeper, which is *why* the E1 bugs sit in functions the tools
had passed. It does **NOT** claim "X% correct" (depth bounds *scrutiny*, not correctness) and does **not**
showcase the matcher (plain fuzzing reaches the same code — depth-vs-their-evidence is the point).

## Cell legend

| mark | meaning |
|---|---|
| `theirs / ours` | median per-fn execution count under the tool's own acceptance **/** under our 1-h fuzz (min in the per-cell detail); `ours` is a lower bound |
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

## The matrix (fillable state derived from E1 final)

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
   run's full execution stream (qsort: 36-file corpus → quickSort 1487, but the 1-h run itself executed
   ~9.5M times). We report the corpus-replay median as a floor; against `theirs = 0` a floor is plenty.
2. **Median, not mean.** Hot leaf functions (loop-called) have heavy-tailed counts (qsort: swap 4211 vs
   partition 726); the mean is meaningless, the median honest, the **min** the strongest defensible line.
3. **Uniform 1-h budget per cell** — fixes corpus size across cells so counts are comparable within the
   protocol; fast libraries (qsort ~63 k exec/s) simply pile up more, which only widens the gap.
4. **ASan is OFF for E3** (`--sanitizer=none`). ASan is E1's *crash oracle* (it turns silent OOB into an
   abort to find bugs); E3 only measures depth, so we disable it — faster, and it lets the corpus grow
   past OOB-read-into-mapped-memory bugs that ASan would have aborted. What still crashes without ASan =
   genuine hard faults (stack overflow from non-termination, wild-pointer SIGSEGV, heap corruption);
   those inputs are unavoidable and handled by (5).
5. **Crash cells** (a minority — most bugs are silent: checksum/reshaping/parse bugs don't fault) are
   censused by **per-process replay of the real fuzzer corpus, merging survivors** → `ours` is a lower
   bound. Framing: on a hard-fault cell our 1-h fuzz *crashed the translation the tool shipped* — the
   tool's acceptance had executed that function **0 times**, so a truncated-but-nonzero depth over
   0 is already the whole point (empirically confirmed: qsort×C2SaferRust median 921 / min 454 vs 0,
   with 22/36 inputs SIGSEGV-ing even ASan-off — the int→usize non-termination is a genuine hard fault).

## Validated prototype — qsort × C2SaferRust (the buggy WIP)

Real cargo-fuzz harness (`fuzz/qsort_c2saferrust/`, C-compare unread for depth): 1-h fork-mode run
drives the translated crate (and en route hits the ASan heap-overflow = E1 bug #1), and per-function
depth over the real corpus gives
**swap 4211 / partition 726 / quickSort 1487** vs the tool's **0** — cell reads `0 / 726` (median). This
anchors the pipeline end-to-end; the runner generalises it across the ∅ cells above.
