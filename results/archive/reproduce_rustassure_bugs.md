# Reproducing RustAssure's reported bugs + UB-gate re-examination (2026-06-29, WIP)

Goal: run OUR differential oracle on the same buggy LLM translations RustAssure reported
(`bug_description.csv`, 16 rows / ~8 unique bugs across libcsv, u8c, optipng), to (a) cross-validate
(do we catch them?) and (b) re-examine each under our UB-free gate (real bug vs UB-driven false positive).

## What's available in the artifact (no tokens)

RustAssure ships MANY per-function translations across 9 model/run variants under
`u8c/test/archive/individual-funcs_<model>_<date>__complete/`. The buggy versions ARE present (LLM was
run 3×; some runs are correct, some buggy). Confirmed by inspection:
- **u8encode_** (GPT-4o, 2025-04-09): `s[len as usize] = 0;` is **UNGUARDED**; for `ch` out of Unicode
  range none of the branches set `len`, so `len = -1` → `s[usize::MAX]` → **panic**. (The 2025-02-11
  GPT-4o run added `if len > 0 {...}` and is correct — shows the nondeterminism.)
- **u8next_** (GPT-4o): present; invalid-UTF-8 returns a different `*ch` than C.
- **scan_option** (optipng): present as `scan_option_case/scan_option.rs` (renamed to `fn r`).
- libcsv buggy fns (`csv_write2`, `csv_set_blk_size`, `csv_set_delim`): **translations NOT shipped**
  (only csv_free[empty]/csv_get_delim/csv_set_term_func in outputs-simple) → would need token regen.

## The reproduction blocker: per-shape idiomatic bridge (the "hard new piece")

Each buggy function has a *different* idiomatic Rust signature that the harness generator must marshal to:
| function | C sig | Rust translation sig | bridge needed |
|---|---|---|---|
| u8encode_ | `int(int ch, char* s)` | `(i32, Option<&mut [u8]>)` | bare output buffer (no length param) **+ C/Rust elem split (char=i8 vs u8)** |
| u8next_ | `int(const char* txt, int* ch)` | `(*const u8, &mut i32)` | input_string raw-ptr type (i8→u8 cast) |
| scan_option | `int(const char*, char*, size_t, char**)` | raw c_char ptrs + `*mut *const c_char` | pointer-to-pointer table |

Bridges added so far (committed): input_buffer → `&Box<[T]>` / `&[T]` / `Vec<T>`; output_array →
`Option<&mut [T]>` / `&mut [T]`. The `out_arr` decode already allocates a fixed-cap buffer (what
u8encode_ needs) — the remaining gaps are (1) schema validation allowing `output_array` on a bare
mutable `ptr` (not just `T(*)[N]`), and (2) a **C-side vs Rust-side elem type split** (C `char*`=i8
buffer, Rust `&mut [u8]`), since the generator currently uses one elem for both sides.

## The UB-gate re-examination (the scientific point)

Each reproduced divergence is classified by the in-loop UB-free gate:
- **u8encode_** = a REAL bug the gate KEEPS: `ch` out of Unicode range is **UB-free in C** (the function
  is designed to return -1 for it; it writes only `*s='\0'` at s[0]). C has no UB → gate does NOT reject
  → the Rust panic is reported. This is the mirror image of the sign_extend demo (where the panic was a
  FALSE positive on a C-UB input and the gate suppressed it). Together they show the gate cuts both ways:
  suppress UB-artifact divergences, keep genuine ones.

## Status

- Found and confirmed the buggy translations in the artifact (no tokens needed for u8c/optipng).
- Generator bridges extended for two shape classes; full reproduction of u8encode_/u8next_/scan_option
  needs the remaining bridge cases above (bare-cap output buffer + elem split; input_string ptr cast;
  T** table). These are incremental generator work, scoped here.
- Already-proven complement: sign_extend (UB-gate suppresses a false positive); head-to-head clean runs
  on clip / aptx_bin_search (both translations). See `results/libopenaptx_ubgate_demo.md`.
