# PtrTrans × lil — run record (2026-09-02)

**Cell status: PARTIAL (translator crashed at unit 95 of 131; partial crate builds).** Replaces the
July-2026 run whose crate was not retained (translation_matrix note 25, "128-function stub tangle").
This run is the archived artifact; it is scored on a separate PARTIAL line, never in the primary table.

## Tool configuration
- PtrTrans-C2Rust checkout `b20d5bb` + the three July path patches
  (`../_tool_patches/ptrtrans_b20d5bb_script.patch`). No translator change, no cost breaker, no
  LLM-only/ablation configuration.
- `--translate_mode Trans_PA --model_name gpt-5.1`, key via `OPENAI_API_KEY` env.
- PA pre-pass: bundled LLVM-14 clang on `lil_expanded_dealed/{lil.c,main.c}` → `llvm-link` →
  `linked_program.ll` (1.6 MB) → `pa_func` (7 s) + `pa_struct` (30 s); reports in
  `input/lil_expanded_dealed/svf_analysis_output/` (`.ll`/`.dot` dumps dropped).
- Library selection: `script/main.py` `project_list = ['crown_dataset/lil']`.

## Source version / input layout
- C input = PtrTrans's own `dataset/crown_dataset/lil/` (lil interpreter, `lil.c` + `lil.h` +
  `main.c` CLI; the same files the CROWN/Laertes copies carry).
- Only input-side change: `compile_commands.json` rewritten to two entries in the shape the other
  libraries use (`cc -I. -c -o lil.o lil.c`, `cc -I. -c -o main.o main.c`, local `directory`);
  the upstream file is kept at `input/lil/_cli_aside/compile_commands.upstream.json`.
- The July KG cache for lil (`dataset/parsed_projects/lil_*`) was moved to
  `parsed_projects/_july_local/` so the KG was re-extracted from this input (`dry_run1.log`:
  2/2 files expanded, tree `lil.c` + `main.c [Contain *main* Function]`).

## Commands
1. `dry_run1.log` — dummy key, `$0`; expansion + KG, stops at the first LLM call (401).
2. PA pre-pass (above).
3. `run1.log` — real key, `python main.py --translate_mode Trans_PA --model_name gpt-5.1`
   (cwd `PtrTrans-C2Rust/script`, backgrounded; 02:44 → 03:09 UTC, crashed).

## Completion state
- **Crash, not completion.** At the 95th of 131 code units (`fnc_write#lil.c#3484`) the
  translation prompt reached **807,401 input tokens** and OpenAI rejected it
  (`openai.error.InvalidRequestError: Input tokens exceed the configured limit of 272000 tokens`,
  `run1.log` tail). PtrTrans has no prompt-size guard, so the process exited; the remaining 36
  units were never attempted. Prompt growth is the tool's own context construction (accumulated
  module + PA annotations); deterministic for this input, so a rerun would stop at the same unit.
  Recorded as "failed to emit a complete artifact under the shipped configuration".
- Metadata (`lil_Trans_PA_trans_metadata.jsonl`, 83 records = 1 project_tree, 3 file, 79 code
  units covering 107 C ids; 75 of the units are functions): 62 `No_Fix_Compile_Success`,
  6 `Fixed_1`, 3 `Fixed_2`, 1 `Fixed_3`, 3 `Fixed_4`, **2 `Fixed_5_Compile_Failed` → stub reverts**,
  1 `Fixed_1_Compile_Failed` (`_expreval_t` typedef/struct unit — emitted a type, no function),
  1 `Free_Function` (`lil_free`, elided by design).
- The two stub reverts: `fnc_store` (+ its callback typedef) and the **11-function mutually
  recursive core** translated as one unit — `lil_parse_value, lil_parse, lil_free_value,
  lil_free_list, substitute, lil_free_env, lil_pop_env, lil_set_var, next_word, get_dollarpart,
  get_bracketpart` — all reverted to signature-only placeholders after 5 repair attempts
  (`AssertionError: replace_metas: []` inside PtrTrans's repair loop is caught by the tool and
  turned into "revert to stub"; tool behaviour, not ours).
- `cargo check` on the archived partial crate: **passes** (0 errors, 59 warnings). Crate:
  `src/lib.rs`, `src/common/lil_mod.rs` (3524 lines, 90 fns), `src/main_mod.rs` (155 lines, 5 fns).
- Output class: **partial (94/131 units reached, 79 emitted), builds; 12 stub functions
  (fnc_store + the 11-function parse/eval SCC), 1 elided (`lil_free`); 36 units never attempted**.
  Function names equal the C names throughout.
- Cost: PtrTrans does not log usage; `lil_runLog.txt` = 2.09 MB of logged prompts+responses
  (≈520k tokens) → ≈ $3–6 at gpt-5.1 prices; the rejected 807k-token request was not billed.
  Estimate, not measured.

## Archive contents
- `PA_trans_projects/lil/` — the emitted crate as left by the crash (`.git` per-unit history and
  `target/` dropped — PtrTrans commits build artifacts into it; per-unit code is in the metadata jsonl)
- `PA_trans_projects/lil_Trans_PA_trans_metadata.jsonl`, `lil_runLog.txt`
- `input/lil/` (staged C incl. `_cli_aside/`), `input/lil_expanded[_dealed]/` (expanded sources,
  SVF reports; `.ll`/`.dot`/doxygen output dropped), `input/parsed_projects/` (KG json, ProjectInfo)
- `dry_run1.log`, `run1.log` (ends in the 807k-token rejection traceback)
