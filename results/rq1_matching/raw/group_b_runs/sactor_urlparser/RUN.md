# SACTOR × urlparser — run record (2026-09-02)

**Cell status: PARTIAL.** First-ever run of this cell. SACTOR emitted analyzable Rust for
8 of the 22 functions in the TU (unidiomatic) and 7 (idiomatic, with a `function_name_map.json`),
then stopped: the global `URL_SCHEMES` (`char *URL_SCHEMES[177]`) failed all 5 translation
attempts, so `url_is_protocol` and every function that transitively depends on it
(`url_get_protocol`, `url_parse`, `url_get_*` ×9, `url_inspect`, `main`) was **never attempted**
by SACTOR's dependency-ordered translator. Scored on a separate PARTIAL line, never in the
primary table.

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`:
  `_close_type_closure`, self-recursion tolerance, spec regex). No other change.
- `sactor.toml`: gpt-5.1 via LiteLLM, key `os.environ/OPENAI_API_KEY`. `crown` (rebuilt this
  session, nightly-2023-01-26) and `sactor` (uv venv) on PATH.
- Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result --continue-run-when-incomplete -c .../sactor/sactor.toml`
- SACTOR verifies each function by compiling the C harness (`-Og -g` added by SACTOR), swapping
  in the Rust function as a cdylib, and running the 6 samples; a failure is retried up to 5×.

## Source version / harness (our input; SACTOR needs an executable + I/O samples)
- `url.h` = jwerle url.h as shipped in CROWN/Laertes/PtrTrans (identical copies).
- `driver.c` (ours, 70 lines): argv `<url> <scheme-word>`; prints `url_is_protocol` /
  `url_is_ssh` of the word, every `url_get_*`, then `url_parse → url_data_inspect → url_free`,
  then `url_inspect`. 6 samples, all full-form URLs (protocol+auth+host+port+path+query+hash) —
  the only inputs on which the C reference is deterministic (its `get_part` sscanf's into a
  1-byte malloc; RQ4 note 27).
- `compile_commands.json`: `gcc -I<dir> -U_FORTIFY_SOURCE -c -o driver.o driver.c`.
  `-U_FORTIFY_SOURCE` is needed because SACTOR's own `-Og` makes Ubuntu's fortify abort the
  reference on the sscanf overflow (`run1_aborted_fortify.log`).
- Expected outputs stored line-stripped, no trailing newline, because that is the form SACTOR
  compares (`run2_aborted_sample_normalization.log`).

## Runs
| log | outcome |
|---|---|
| `run1_aborted_fortify.log` | 2 LLM calls; C reference aborted under fortify → harness fix |
| `run2_aborted_sample_normalization.log` | 3 LLM calls; expected-output whitespace mismatch → samples normalized |
| `run3.log` (+ `run3_result/`) | 20 LLM calls. 7 fns passed; `url_is_protocol` failed 5×; **crashed at `get_part`**: `RuntimeError: Failed to link project-level harness` — `libget_part.so: undefined reference to strrwd / strff` (`run3_get_part_link_attempt.json`). Cause: `strff`, `strrwd`, `get_part` are `static` in url.h, so SACTOR's per-function cdylib cannot resolve them from the C object. A linkage-only property of the input, so it was adapted for run 4. |
| `run4_aborted_sactor_not_on_path.log` | 1 LLM call; our launch error (`sactor` not on PATH) |
| **`run4.log` (+ `run4_result/`, `run4_input/`)** | **the archived result**, 38 LLM calls (20 unidiomatic, 16 idiomatic, 2 other). Input = run 3 input with `static` removed from `strff`, `strrwd`, `get_part` in `url.h` (3 lines; no semantic change). |

## Completion state (run 4)
- Unidiomatic phase: **8/22 functions** passed all 6 samples — `strdup`, `url_data_inspect`,
  `url_free`, `show`, `strff`, `strrwd`, `url_is_ssh`, `get_part`; struct `url_data` 1/1.
  Global `URL_SCHEMES`: 5/5 attempts fail (`error[E0277]: *mut i8 cannot be shared between
  threads safely` for a `static` array of C strings; repairs then E0308). `url_is_protocol`
  therefore fails 5/5 (same E0277 on its inlined copy of the table). The 13 dependents were
  never attempted (`unidiomatic_failure_info.json`: attempts recorded only for the 9 tried).
  SACTOR result: `MAX_ATTEMPTS_EXCEEDED`; combined crate does not build (duplicate `use`
  imports, `combined/unidiomatic/`).
- Idiomatic phase (`--continue-run-when-incomplete`): 7 functions emitted
  (`translated_code_idiomatic/functions/`, `specs/function_name_map.json` — all 7 keep their C
  names), `get_part` not reached; run ended with `RuntimeError: Dependency 'url_is_protocol'
  ... should have been translated before use` (`batch_summary.json` status `failed`).
- Output class: **partial, non-building, name map for 7 fns**. Analyzable Rust: 8 unidiomatic
  `.rs` + 7 idiomatic `.rs` (single-function files, not a crate).
- Cost: `llm_stat_unidiomatic.json` = 20 queries, 32.8k in / 27.7k out tokens; idiomatic phase
  16 prompts (no stat file written). Whole cell incl. aborted attempts ≈ 65 prompts ≈ $3–4.

## Why this is a shipped-configuration failure, not a harness artifact
The blocking item is SACTOR's translation of a C global array of string literals into a Rust
`static` of raw pointers, which rustc rejects (not `Sync`); five repair attempts do not find a
`&'static str` encoding. The run-3 link failure was an input-linkage issue and was fixed on the
input side (no translator change); nothing else was patched, no cost breaker raised, no
LLM-only/ablation configuration used.
