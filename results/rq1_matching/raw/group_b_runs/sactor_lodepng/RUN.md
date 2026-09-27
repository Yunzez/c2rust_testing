# SACTOR × lodepng — run record (2026-09-02)

**Cell status: PARTIAL, non-building.** First archived run of this cell (the July-2026 attempt,
`results/rq4_effectiveness/bugs/lodepng_sactor/README.md`, stopped at 16/66 functions under our
own cost breaker and its Rust was not retained → "produced but lost"). This run used **no cost
breaker**; SACTOR itself stopped after **74 of the 235 functions** in `lodepng.c` (unidiomatic
phase): 53 verified, 20 exhausted their 6 attempts on a tool-side scaffold conflict, and the
74th (`lodepng_gtofl`) failed SACTOR's per-function link step, which aborts the TU; the
remaining 161 were never attempted.
Scored on a separate PARTIAL line, never in the primary table.

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`).
  No translator change, no cost breaker, no LLM-only/ablation configuration.
- `sactor.toml`: gpt-5.1 via LiteLLM, key `os.environ/OPENAI_API_KEY`. `crown`, `sactor` on PATH.
- Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result --continue-run-when-incomplete -c .../sactor/sactor.toml`
- Launched under `timeout --signal=INT --kill-after=120 5400` (an external 90-minute wall-clock
  cap, declared here for honesty). **The cap did not fire**: the run ended by SACTOR's own
  error at 40 minutes.

## Source version / harness (`input/`)
- C = `lodepng.c` / `lodepng.h` from PtrTrans's `dataset/crown_dataset/lodepng/` (md5
  `b26de23b…`, byte-identical to `tools/frameworks/crown/c-code/lodepng/lodepng.c`, i.e. the
  same source every other lodepng cell uses). No C source edit. 235 functions
  (`Function order` list in `run1.log:23`; matches the 235 C rows of the ptrtrans_lodepng sheet).
- `driver.c` = the July driver (`driver <w> <h>`: `lodepng_encode32` of a generated w×h image, then `lodepng_decode32`
  of the result; prints `enc <err> <pngsize>` and `dec <err> <w> <h> <pixel-sum>`); `test_samples.json` = the July 4
  samples, all 4 reproduced by the C reference before launch. `test_task.json` = 4 ×
  `sactor run-tests --type bin ./test_samples.json %t <i> --feed-as-args`.
- `compile_commands.json`: 2 entries (`gcc -I<dir> -c -o X.o X.c` for `lodepng.c`, `driver.c`).

## Run (`run1.log`, 04:31:54 → 05:11:54 UTC)
`run1_result/batch_summary.json`:

| TU | status | detail |
|---|---|---|
| `lodepng.c` | failed | unidiomatic phase, 74/235 functions reached (dependency order), then **`Failed to link project-level harness for function lodepng_gtofl`** |
| `driver.c` | failed | `Dependency 'lodepng_encode32' of type 'function' should have been translated before use` (never reached) |

Per-unit outcome (`run1_result/lodepng.c__06d02e56/unidiomatic_failure_info.json`, 101 units):

| class | units | mechanism |
|---|---|---|
| function verified (1 attempt) | 42 | compiled, linked into the harness, 4/4 samples pass |
| function verified (2 attempts) | 11 | first attempt rejected (signature mismatch 8, syntax 2, compile 1), second passes |
| **function failed 6/6** | **20** | every attempt dies in SACTOR's *embedding* crate with `E0428: the name LodePNGColorType is defined multiple times` + `E0117` + `E0599`: SACTOR's harness scaffold emits `pub type LodePNGColorType = c_uint;` for the C typedef **and** injects the translated `enum LodePNGColorType` it produced earlier, so any function whose signature or body touches the colour type cannot compile regardless of what the LLM writes (`lodepng_color_mode_init/equal`, `lodepng_is_{greyscale,alpha,palette}_type`, `lodepng_has_palette_alpha`, `LodePNG{UnknownChunks,Text,IText}_init`, `rgba16ToPixel`, `getPixelColorRGBA16`, `lodepng_convert_rgb`, `readChunk_{tRNS,bKGD,tIME,pHYs,gAMA,cHRM,sRGB,sBIT}`). Same E0117 class as the July run. |
| **function link failure → TU abort** | **1** | `lodepng_gtofl`: Rust compiles, but the cdylib references `lodepng_addofl`, which is `static` in `lodepng.c` and therefore not exported from the C object SACTOR links against. Reproduced by re-running SACTOR's exact link line on its build dir: `undefined reference to 'lodepng_addofl'`. SACTOR treats a link failure as fatal for the TU (same mechanism as the tulip cell), so the remaining 161 functions were never attempted. |
| struct / enum / global | 18 / 2 / 7 | all emitted on attempt 1 |

- Output class: **partial, non-building, no tool name map** (`function_name_map.json` is written
  only in the idiomatic phase, never reached). Analyzable Rust: 53 verified function files
  (`translated_code_unidiomatic/functions/`) + 21 unverified last attempts recovered from
  `run1_result/logs/sactor-20260902T043148.jsonl` (`run1_extracted_rust/`, 175 translations
  for 74 functions, `index.json` carries every attempt's verdict). All 74 keep their C names.
  Scored artifact = `assembled_unidiomatic.rs` (types + 53 verified + 21 last attempts, header
  says which is which).
- Cost: no `llm_stat*.json` (written only when a TU completes). From the log: 175 translation /
  repair prompts (gpt-5.1) → **≈ $5** (estimate, not measured; a repair prompt carries the
  function, its dependencies and the rustc output, ~10k tokens).

## Why this is a shipped-configuration failure
Both blockers are inside SACTOR's verification scaffold, not in the LLM output: the duplicate
`LodePNGColorType` definition is generated by the tool, and the per-function link closure
cannot see `static` C helpers. Working around either would mean patching the translator (or
editing `lodepng.c` to un-`static` its helpers), which the reporting rules exclude. Recorded as
"failed to emit a complete artifact under the shipped configuration".

## Archive contents
- `input/` — `lodepng.c`, `lodepng.h`, `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json`
- `run1.log`, `run1_result/` (`batch_summary.json`, `lodepng.c__06d02e56/{config.json,
  unidiomatic_failure_info.json, translated_code_unidiomatic/{enums,global_vars,structs,functions}}`,
  `driver.c__585b2fa3/config.json`, `logs/` with the structured jsonl; `target/`, `.so`, `.o` dropped)
- `run1_extracted_rust/` (175 `.rs` + `index.json`)
- `assembled_unidiomatic.rs` — the scored artifact
