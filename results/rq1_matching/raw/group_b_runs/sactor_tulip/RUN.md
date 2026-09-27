# SACTOR × tulip — run record (2026-09-02)

**Cell status: PARTIAL, non-building.** First archived run of this cell (the July-2026 attempt was
"produced but lost"). SACTOR emitted Rust for **69 functions (70 translations; one per TU, the
first function in dependency order)** but verified none of them: every TU fails at
"Failed to link project-level harness" or earlier. No `translated_code_*/functions/*.rs` was
written by the tool; the Rust exists only in the structured log and was extracted to
`run1_extracted_rust/` (+ `run2_extracted_rust/`) by `scripts/rq1_sactor_extract_log_rust.py`.
Scored on a separate PARTIAL line, never in the primary table.

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`).
  No translator change, no cost breaker, no LLM-only/ablation configuration.
- `sactor.toml`: gpt-5.1 via LiteLLM, key `os.environ/OPENAI_API_KEY`. `crown` and `sactor`
  (uv venv) on PATH.
- Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result --continue-run-when-incomplete -c .../sactor/sactor.toml`

## Source version / harness
- C = local tulipindicators checkout `tools/frameworks/tulipindicators` (`be18abb`, v0.9.1-8,
  the 0.9.2-era tree used by every other tulip cell): `indicators.c`, `indicators.h`,
  `indicators/*.c` (104 files), `utils/buffer.c`, `candles.c`, plus upstream **`sample.c`
  unmodified** as the executable (byte-identical to the checkout; tulip ships it as the CLI
  demo: `sample <indicator> <options…>` prints the indicator applied to a fixed 15-row IBM
  series). No C source edit.
- `compile_commands.json` (`input/`): 108 entries `gcc -I<root> -I<root>/utils -c -o X.o X.c`
  (sample.c, indicators.c, candles.c, utils/buffer.c, indicators/*.c).
- `test_samples.json`: 12 invocations — `sma 5`, `ema 5`, `rsi 4`, `macd 3 6 4`, `bbands 5 2`,
  `atr 4`, `stoch 5 3 3`, `adx 4`, `wma 4`, `cci 5`, `obv`, `crossover` — expected output =
  the C `sample` binary's stdout. `test_task.json` = 12 × `sactor run-tests --type bin
  ./test_samples.json %t <i> --feed-as-args`.

## Runs
| log | outcome |
|---|---|
| **`run1.log` (+ `run1_result/`)** | 02:46 → 03:16 UTC, 108/108 TUs `failed` (`batch_summary.json`). 70 "Translated function" LLM outputs, 0 verified. |
| `run2_37tus.log` (+ `run2_result/`) | 04:05 → 04:27 UTC, 108/108 `failed` again (see below). Re-submission of the 37 TUs that failed before any LLM call in run 1, with `-I<root>/indicators` added to their compile_commands entries (`input/compile_commands_run2_37tus.json`). |

## Completion state (run 1)
Per-TU failure classes from `run1_result/batch_summary.json` (108 TUs):

| class | TUs | mechanism |
|---|---|---|
| `Failed to link project-level harness for function <first fn>` | **69** | SACTOR verifies a function by linking the C harness against a cdylib of that one Rust function. `sample.c` reaches every indicator through the `ti_indicators[]` function-pointer table in `indicators.c`; the table (and with it every `ti_X`, `ti_X_start` symbol the harness references) lies outside SACTOR's per-function link closure, so the link step fails for the very first function of every TU. 69 functions were translated before this (mostly `ti_X_start`, plus `ti_version`, `ti_buffer_new`), all with verdict "link failure". Same mechanism as the July RQ4 note ("link-closure blind to fn-ptr table, 217 undef"). |
| gcc failure on SACTOR's relocated expanded TU | **37** | SACTOR expands each TU to `/tmp/sactor/<tmp>/expanded_<name>.c` and compiles it with its own project-level `-I` set. The 37 SIMPLE1-family indicators (`abs acos add adx adxr asin atan atr ceil cos cosh di div dm dx exp floor fosc linreg linregintercept linregslope ln log10 mul natr round sin sinh sqrt sub tan tanh todeg torad tr trunc tsf`) `#include "simple1.h"` / `"dx.h"`-style same-directory headers, which are not on that path → `fatal error: simple1.h: No such file or directory`. No LLM call for these TUs. |
| `Dependency 'ti_find_indicator' … should have been translated before use` | 1 | `sample.c`: its dependency in another TU was never verified. |
| `Struct 'tc_result' not found` | 1 | `candles.c`: SACTOR's parser does not resolve the struct from `candles.h`. |

- Output class: **partial, non-building, no tool name map** (SACTOR writes
  `function_name_map.json` only in the idiomatic phase, which was never reached). Analyzable
  Rust: 70 log-extracted unidiomatic function bodies (`run1_extracted_rust/`, `index.json`),
  each carrying the tool's verdict in a header comment.
- Cost: SACTOR writes `llm_stat.json` only when a TU completes, so none exists. From the log:
  70 unidiomatic translation prompts (one per TU; the link failure aborts the TU before any repair round) ≈ **$5**
  (estimate, not measured).

## Run 2 (the 37 pre-LLM failures)
Rationale: those 37 TUs never reached the translator, so run 1 says nothing about SACTOR's
translation of them. Adding an include path to *our* compile_commands is an input-side
adaptation (no translator change). Result: **SACTOR ignores per-entry `-I` flags** — its
relocated-TU compile command still shows only `<root>`, `<root>/utils` and system includes
(`run2_37tus.log`), so the same 37 TUs fail the same way and, for the few that get past
preprocessing, hit the run-1 link failure. Even a direct gcc with `-I indicators` fails on
`TI_REAL` because SACTOR's partial macro expansion has already stripped the `indicators.h`
typedef from the expanded TU. Recorded as "failed to emit analyzable Rust under the shipped
configuration" for these TUs; the run-2 log and extracted Rust are archived as-is
(`run2_result/`, `run2_extracted_rust/`).

**Run 2 outcome (04:05 → 04:27 UTC):** SACTOR does not restrict itself to the compile_commands
entries either — it re-discovered and re-ran all **108** TUs from the project directory
(`run2_result/batch_summary.json`: 69 link failures, 37 gcc-expanded failures, 1 dependency,
1 struct — the same partition as run 1). 70 translations for 69 functions again
(`run2_extracted_rust/`; independent LLM samples of the same functions, so they are a second
draw, not new coverage). Because the whole project was re-run, cost ≈ **$5** (not the $2
expected for 37 TUs). Whole cell ≈ $10, estimate.

## Archive contents
- `input/` — `sample.c`, `test_samples.json`, `test_task.json`, `compile_commands.json`,
  `compile_commands_run2_37tus.json` (C sources are the unmodified `tools/frameworks/tulipindicators` tree)
- `run1.log`, `run1_result/` (`batch_summary.json`, per-TU `unidiomatic_failure_info.json`, `logs/`;
  `target/` dropped), `run1_extracted_rust/` (70 `.rs` + `index.json`)
- `run2_37tus.log`, `run2_result/`, `run2_extracted_rust/`
