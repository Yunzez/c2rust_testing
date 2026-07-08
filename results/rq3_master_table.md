# E3 Master Table — coverage + depth of each tool-translation (rows = library, cols = tool)

**The paper's Table 3 — the credibility backstop for E1.** Same shape as E1/E2: **rows = the 10 E1
libraries, columns = the 6 shipped translators.** Each cell is the same contrast, on that tool's actual
Rust translation of that library: **what the tool's own tests reach vs what our differential fuzz reaches.**

Two metrics, both free from one `-C instrument-coverage` build + `llvm-cov` per cell:
1. **Coverage** — `their-cov / our-cov` (fn% or line%): do we exercise **at least as much** of the
   translated Rust as the tool's own tests? Answers "did we look everywhere they did, and more."
2. **Hit-DEPTH** — median executions per function, ours vs theirs (in the per-cell detail): the causal
   part coverage can't claim — *their validation is shallow (O(1) hits/fn) → bugs survive; we hit
   O(10⁴–10⁵)× deeper → we find them.*

**Cell = `their / ours`** (mirrors E2's two-number cell). Rust side only (each tool emits Rust and
validates on Rust; C stays the differential oracle). raw-LLM is NOT a column — E3 tests the *shipped
tools* (collaborator's call).

## Cell legend

| mark | meaning |
|---|---|
| `their / ours` | coverage under the tool's own tests **/** under our differential fuzz (fn% or line%); depth ratio in the per-cell detail |
| `∅ᵀ` | artifact exists, **not yet run** |
| `∅ᵀ★` | not yet run **and this cell holds a confirmed E1 bug** — priority (punchline: bug in a shallow-validated fn) |
| `—` | no runnable artifact (this tool produced no parseable/compiling Rust for this library — inherits E1's ✗) |

Method per cell: build that tool's translated crate once with `-C instrument-coverage`; run (a) the tool's
/ library's tests → llvm-cov → per-fn `count` + fn/line %, (b) our differential fuzz → same; the cell
shows `their-cov / our-cov`, the detail shows median-depth `theirs → ours`. Runner:
`scripts/eval_rq3_depth.py` (TBD); mechanism proven on the fft_crust prototype below.

## The matrix

| library | domain | ~#fn | c2rust | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans |
|---|---|---:|---|---|---|---|---|---|
| **qsort** | sorting | 3 | ∅ᵀ | ∅ᵀ | ∅ᵀ★ | ∅ᵀ | ∅ᵀ | ∅ᵀ★ |
| **urlparser** | URL parsing | 21 | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ | — | — |
| **quadtree** | spatial tree | 24 | ∅ᵀ | — | — | ∅ᵀ | — | ∅ᵀ |
| **genann** | neural net | ~20 | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ |
| **cJSON** | JSON parser | 58 | ∅ᵀ | — | — | — | — | ∅ᵀ★ |
| **lil** | interpreter | 145 | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ | — | — |
| **lodepng** | PNG codec | 235 | ∅ᵀ | — | — | ∅ᵀ | — | — |
| **bzip2** | compressor | 64 | ∅ᵀ | ∅ᵀ★ | ∅ᵀ | ∅ᵀ | — | — |
| **tulipindicators** | indicators | ~100 | ∅ᵀ | ∅ᵀ | ∅ᵀ | ∅ᵀ | — | — |
| **optipng** | PNG optimizer | ~400 | ∅ᵀ | ∅ᵀ | ∅ᵀ★ | — | — | — |

Same `—` pattern as E2 (no artifact where the tool failed to translate). `∅ᵀ★` cells = the 5 confirmed
E1 bugs (qsort×C2SaferRust int→usize · qsort×PtrTrans unsorted · cJSON×PtrTrans parse_string ·
bzip2×Laertes zeroed-table · optipng×C2SaferRust crc32) — fill these first: the punchline is a bug in a
function the tool's own tests hit shallowly.

## Method prototype — fft_crust (SACTOR)

Not a corpus cell (a SACTOR test example) — proves the pipeline + anchors the "shallow" number.
**SACTOR validated the whole fft program with 6 test samples**; its 8 internal functions are each hit a
handful of times by the tool's own correctness check. Buildable crate + 6 baseline samples + C oracle all
present → produces the per-cell `their-cov / our-cov` + median-depth the grid uses. (The 6-sample fact is
already in the eval plan §6: the input space beyond it is unverified.)

## What E3 says / does NOT say

- **Says:** we exercise ≥ what the tool's own tests do (coverage), and we hit each function O(10⁴–10⁵)×
  deeper (depth) — so the bugs we found sit in functions their shallow validation had passed.
- **Does NOT say:** "X% correctly translated" (coverage ≠ correctness — it bounds scope). And it does not
  showcase the matcher (plain fuzzing reaches the same code; depth-vs-their-tests is the point).
