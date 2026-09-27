# Head-to-head corpus (acquired 2026-06-28) — for comparison vs Fluorine & RustAssure

Cloned `github.com/davsec-lab/rustassure` (branch `configurations`) into the gitignored
`tools/frameworks/rustassure/`. It bundles the competitors' benchmark programs, translations, and — key —
their **ground-truth bug list**, so we can run OUR oracle (UB-correct differential fuzzing + matcher +
frontier) on the SAME programs and compare directly.

## What we got (under `src/python/`)

| asset | location | notes |
|---|---|---|
| C inputs | `inputs-complex/{libcsv,optipng,u8c,libbmp,libopenaptx}` | libcsv 1301, optipng 109k, u8c 737, libbmp 397, **libopenaptx 1640** LOC |
| **libopenaptx dual translations** | `inputs-complex/libopenaptx/{flourine,gpt4o}/*.rs` | per-function Rust from **Fluorine AND GPT-4o**; **name-preserving** (`fn aptx_check_parity` ↔ C) |
| RustAssure's libcsv translations (partial) | `outputs-simple/libcsv-{csv_free,csv_get_delim,csv_set_term_func}-trans` | only 3 fns shipped |
| **ground-truth bugs** | `src/bug_description.csv` | **16 bugs**: Libcsv 6, u8c 5, optipng 2 (+ libopenaptx via Fluorine head-to-head) |

`libopenaptx` is the **de-facto shared benchmark** (Fluorine's; RustAssure reused it) → enables a true
**3-way head-to-head** (C oracle vs Fluorine-Rust vs GPT4o-Rust) on the exact program both papers used.

## The 16 ground-truth bugs (samples — our targets to reproduce / re-examine)

- **libcsv/GPT-4o** `csv_write2` (NULL-buffer: Rust returns 0, C returns length), `csv_set_blk_size`
  (0-value not assigned in Rust), `csv_set_delim` ('\0' not assigned), `csv_free` (entry_size not zeroed
  when buf NULL).
- **optipng** `scan_option` ('=' handling: Rust appends '=' to option name).
- **u8c/GPT-4o** `u8encode_` (ch out of Unicode range → Rust indexes `s[usize::MAX]` → **panic**, C returns
  -1 gracefully), `u8next_` (invalid UTF-8 → Rust returns intermediate decoded value, C returns first byte).

## Why this is a strong head-to-head

1. **Reproduce**: run our differential fuzzing on libopenaptx (both translations) + the buggy
   libcsv/u8c/optipng functions → do we catch RustAssure's 16? (cross-validation → credibility).
2. **Find more**: RustAssure's symbolic oracle checks only return-vars, ≤~600 LoC; Fluorine's fuzzer is
   brittle (10/31 survive). Our deeper-state fuzzing should find divergences they miss.
3. **UB discipline (our differentiator)**: re-examine each of the 16 under our UB-free gate. e.g. u8c
   `u8encode_` panic fires on `ch` out of Unicode range — that input is **UB-free in C** (C returns -1),
   so our gate KEEPS it = a real bug (confirms theirs, on principled footing). Conversely, check if any of
   the 16 are actually UB-triggered (RustAssure admits memory-corruption false positives).
4. Translations are **name-preserving** → good for the ORACLE comparison, but NOT a matcher stress test
   (need renamed output for that — separate track).

## Gap / next

- We have full dual translations only for **libopenaptx**; libcsv has 3 shipped, u8c/optipng buggy-fn
  translations need extraction from the repo or regeneration (tokens). Start the head-to-head on
  **libopenaptx** (complete dual translations, the shared program) — build the C-oracle differential
  harness (reuse tulip robust trap-driver) for the ~20 aptx functions, vs Fluorine-Rust and GPT4o-Rust.
- Cross-reference each divergence against `bug_description.csv` and apply the UB-free gate.
