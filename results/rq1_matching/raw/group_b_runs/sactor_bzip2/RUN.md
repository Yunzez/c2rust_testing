# SACTOR × bzip2 — Gate 0/1 record (2026-09-02)

**Cell status: PARTIAL, non-building.** First archived run of this cell. The July-2026 note
(`results/rq4_effectiveness/bugs/bzip2_sactor/README.md`, "C-parser failure (BZALLOC…)") is
superseded: with the two indirect-call macros re-spelled on the input side, SACTOR's resolver
accepts the project and the paid run reaches the LLM. SACTOR itself then stopped after
**32 of the 64 library functions** (unidiomatic phase): 27 verified, 3 exhausted their 6 attempts
on tool-side scaffold conflicts, and 2 (`flush_RL`, `bsPutUInt32`) compiled but failed SACTOR's
per-function link step, which aborts their TU; the remaining 32 (+ the driver's `main`) were never
attempted, several because SACTOR's batch order checked cross-TU dependencies before the defining TU had run. Scored on a separate PARTIAL
line, never in the primary table. Cost for the whole cell (three launches) ≈ **$2–2.5** (estimate).

## Protocol
User-accepted Gate 0/1 protocol: Gate 0 = $0 dummy-key dry runs (parse + dependency analysis run
before the 401), only semantically neutral *input-side* rewrites allowed (quadtree precedent), no
translator patch, no cost breaker raised, no LLM-only configuration. Gate 1 = one paid run, capped
at ≈$1–2, then stop whatever the state. Gate 0 passed on the second dry run; Gate 1 needed three
launches because the first two ended in tool-side failures with almost no output (both archived).

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`).
- `sactor.toml`: gpt-5.1 via LiteLLM, key `os.environ/OPENAI_API_KEY`. `crown`, `sactor` on PATH.
- Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result --continue-run-when-incomplete -c .../sactor/sactor.toml`
- Run 2 launched under `timeout --signal=INT --kill-after=120 1500` (an external 25-minute
  wall-clock cap, declared here for honesty). **The cap did not fire**: SACTOR ended by its own
  errors after 13 minutes.

## Source version / harness (`input/`)
- C = the Group A bzip2 scope (bzip2 1.0.8 library: `blocksort.c bzlib.c compress.c decompress.c
  huffman.c crctable.c randtable.c` + `bzlib.h bzlib_private.h`; same source as the PtrTrans and
  Group A bzip2 cells). 64 library functions (`Function order` lists in `run2.log`: blocksort 9,
  bzlib 41, compress 9, decompress 2, huffman 3) + `main` in the driver.
- `driver.c` (`driver <n> <pattern>`): `BZ2_bzBuffToBuffCompress` of a generated n-byte buffer,
  then `BZ2_bzBuffToBuffDecompress`; prints `comp <rc> <size>` and `decomp <rc> <len> <sum> <memcmp>`.
  `test_samples.json` = 4 cases (`1000 7`, `0 1`, `50000 13`, `100 255`), expected output from the
  C reference binary (`gcc -o driver_ref *.c`). `test_task.json` = 4 ×
  `sactor run-tests --type bin ./test_samples.json %t <i> --feed-as-args`.
- `compile_commands.json`: 6 entries `gcc -I<dir> -c -o X.o X.c`.

### Neutral input-side rewrites (all re-verified against the reference binary + 4 samples)
| file | rewrite | why |
|---|---|---|
| `bzlib_private.h` | `BZALLOC(nnn)` / `BZFREE(ppp)` macros: `(strm->bzalloc)(strm->opaque,(nnn),1)` → `strm->bzalloc(strm->opaque,(nnn),1)` (same for `bzfree`) | the July wall: SACTOR's resolver (`c_parser.py:783-823`) is fatal on a parenthesised callee; `x->f(...)` is skipped. Identical to the optipng `ZALLOC` rewrite. |
| `bzlib.c:1` | `#define _POSIX_C_SOURCE 200809L` before the first include | SACTOR parses with `-std=c99`, which hides `fdopen` (the log still shows the warning from SACTOR's own typedef-unfolding pass, which re-parses without the define; harmless) |
| `bzlib.c` | the whole of `crctable.c` and `randtable.c` pasted verbatim **at the top of `bzlib.c`** (right after `#include "bzlib_private.h"`, each preceded by a `neutral relocation` comment); the two files moved to `input/removed_table_tus/` and dropped from `compile_commands.json` | see run 1 / run 1b below — a data-only TU is invisible to SACTOR's link closure, and an `extern` declaration seen before the definition makes SACTOR crash. Tables are byte-identical; only their TU changed. |

## Gate 0 (`gate0/dry_run{1,2}.log`, `OPENAI_API_KEY=sk-dummy-gate0`)
| run | layout | outcome |
|---|---|---|
| 1 | original 7 TUs, macros re-spelled | parser + dependency analysis pass; `crctable.c`/`randtable.c` "fail" immediately (nothing to translate → SACTOR's `cp … combined.rs` finds no file); the 5 function-bearing TUs reach the LLM (401 on the dummy key) → **Gate 0 passed** |
| 2 | tables relocated into `bzlib.c` | all 5 library TUs reach the 401; `huffman.c` and `driver.c` stop on cross-TU order (expected, they depend on `bzlib.c`) |

## Gate 1 — three launches
### Run 1 (`run1_aborted_original_layout/run1.log`, 14:20 UTC, original 7-TU layout, ≈5 prompts)
Every TU's **first** function (`fallbackSimpleSort`, `bz_config_ok`, …) compiled and was then
rejected at the link step: `Failed to link project-level harness`. SACTOR does not log the linker
output; re-running its exact link line on the retained build dir
(`relink_evidence/run1_aborted_original_layout__fallbackSimpleSort_relink.txt`) gives
20 × `undefined reference to BZ2_crc32Table / BZ2_rNums`. Mechanism: SACTOR's per-function
harness links the object files of the *function-bearing* TUs only (`Project-level objects:` lists
blocksort/bzlib/compress/decompress/huffman/driver, never crctable/randtable), so the two
data-only TUs are simply absent from the link. Killed by me after the first TU failures.
### Run 1b (`run1_crash_tables_at_end/run1.log`, 14:24 UTC, tables appended at the *end* of `bzlib.c`, ≈10 prompts)
9 functions verified, then SACTOR **crashed** (uncaught `pyo3_runtime.PanicException` in
`rust_ast_parser.remove_mut_from_type_specifiers`: `not implemented: Item::Verbatim static mut
BZ2_crc32Table: [u32; 256];`). Mechanism: the global-variable prompt is built from libclang's
`referenced` cursor = the *first visible declaration*; with the definition at the end of the file
that is the `extern UInt32 BZ2_crc32Table[256];` in `bzlib_private.h`, the LLM faithfully emits an
uninitialised `static mut`, and SACTOR's AST pass panics on it. Whole batch dies.
### Run 2 (`run2.log`, `run2_result/`, 14:33:36 → 14:46:22 UTC, tables at the *top* of `bzlib.c`, 61 prompts)
`run2_result/batch_summary.json`:

| TU | status | detail |
|---|---|---|
| `bzlib.c` | failed | 20/41 functions reached; **`Failed to link project-level harness for function flush_RL`** |
| `compress.c` | failed | 5/9 reached; **`… for function bsPutUInt32`** |
| `blocksort.c` | failed | 3/9 reached (translated *before* bzlib.c in batch order); stopped at `fallbackQSort3` on the cross-TU dependency check `Dependency 'BZ2_bz__AssertH__fail' … should have been translated before use` (`run2.log:1334`) |
| `huffman.c` | failed | 3/3 verified, then the same `BZ2_bz__AssertH__fail` check (bzlib.c had aborted by then) |
| `decompress.c` | failed | 1/2 reached; stopped at `BZ2_decompress` on `Dependency 'BZ2_hbCreateDecodeTables' …` (huffman.c ran later in batch order; `run2.log:22646`) |
| `driver.c` | failed | `Dependency 'BZ2_bzBuffToBuffCompress' …` (never reached) |

Per-unit outcome (`run2_result/*/unidiomatic_failure_info.json`, 39 units):

| class | units | mechanism |
|---|---|---|
| function verified (1 attempt) | 23 | compiled, linked into the harness, 4/4 samples pass |
| function verified (2–4 attempts) | 4 | `copy_output_until_stop` 2, `unRLE_obuf_to_output_FAST` 2, `BZ2_indexIntoF` 2, `BZ2_hbMakeCodeLengths` 4 |
| **function failed 6/6** | **3** | `BZ2_bzReadGetUnused`: every attempt `E0277 bz_stream doesn't implement Debug` (SACTOR's harness derives/prints the struct it scaffolded); `BZ2_bzerror`: same + `E0277 *const i8 cannot be shared between threads safely`; `BZ2_bzCompressInit`: `E0255 the name size_t is defined multiple times` (scaffold `use libc::size_t` vs the LLM's typedef). All three are conflicts between SACTOR's embedding crate and the translated text. |
| **function link failure → TU abort** | **2** | `flush_RL` (bzlib.c) and `bsPutUInt32` (compress.c): Rust compiles, but the cdylib references `add_pair_to_block`/`init_RL` and `bsW`, which are `static` in the C files and hence not exported from the C objects SACTOR links against. Reproduced by re-running SACTOR's exact link line on its retained build dirs (`relink_evidence/run2__*_relink.txt`). SACTOR treats a link failure as fatal for the TU (same mechanism as the lodepng `lodepng_gtofl` and tulip cells). |
| struct / global | 4 / 3 | structs attempt 1; `BZ2_crc32Table`, `BZ2_rNums` attempt 1; `bzerrorstrings` verified on attempt **6** (5 rejected attempts: `&&[u8;N] as *const u8` casts, `*const i8` not `Sync`, mismatched types) |

- Never attempted: 32 library functions (blocksort 6 incl. `BZ2_blockSort`, bzlib 21 incl.
  `BZ2_bzCompress/Decompress`, `BZ2_bzBuffToBuff*`, `BZ2_bz{Read,Write}*`, compress 4 incl.
  `BZ2_compressBlock`, decompress 1 `BZ2_decompress`) + `main`.
- Output class: **partial, non-building, no tool name map** (`function_name_map.json` is written
  only in the idiomatic phase, never reached). All 32 attempted functions keep their C names.
  Analyzable Rust: 27 verified function files (`translated_code_unidiomatic/functions/`) + the
  5 unverified last attempts recovered from `run2_result/logs/sactor-20260902T143332.jsonl`
  (`run2_extracted_rust/`, 52 translations for 32 functions; `index.json`'s verdict strings are
  attributed from log adjacency and are unreliable — use `unidiomatic_failure_info.json`). Scored artifact = `assembled_unidiomatic.rs` (types/globals + 27 verified + 5 last
  attempts, header says which is which).
- Cost: no `llm_stat*.json` (written only when a TU completes). From the logs: run 1 ≈ 5,
  run 1b ≈ 10, run 2 = 61 translation/repair prompts (53 function + 8 global; gpt-5.1) →
  **≈ $2–2.5 for the cell** (estimate, not measured).

## Why this is a shipped-configuration failure
All four blockers are inside SACTOR, not in the LLM output: the link closure cannot see data-only
TUs or `static` C helpers, the global-variable prompt is built from the first declaration rather
than the definition, the harness scaffold conflicts (`Debug`, `size_t`) are generated by the tool,
and a single link failure aborts a TU and every TU depending on it. The two input-side rewrites
that were made (macro spelling, table relocation) are neutral; the remaining ones would require
un-`static`-ing bzip2's helpers or patching the translator, both excluded by the reporting rules.
Recorded as "failed to emit a complete artifact under the shipped configuration".

## Archive contents
- `input/` — the 5 library `.c` + 2 `.h` as run (with the rewrites above), `removed_table_tus/`
  (original `crctable.c`, `randtable.c`), `driver.c`, `test_samples.json`, `test_task.json`,
  `compile_commands.json`
- `gate0/` — `dry_run1.log`, `dry_run2.log`
- `run1_aborted_original_layout/run1.log`, `run1_crash_tables_at_end/run1.log`
- `run2.log`, `run2_result/` (`batch_summary.json`, 6 TU dirs with `unidiomatic_failure_info.json`
  + `translated_code_unidiomatic/{structs,global_vars,functions}`, `logs/` with the structured jsonl;
  `target/`, `.so`, `.o` dropped)
- `run2_extracted_rust/` (52 `.rs` + `index.json`)
- `relink_evidence/` — SACTOR's own link lines re-run on its retained build dirs (3 files)
- `assembled_unidiomatic.rs` — the scored artifact
