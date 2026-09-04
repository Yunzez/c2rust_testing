# PtrTrans × urlparser — run record (2026-09-02)

**Cell status: AVAILABLE (complete, builds).** First-ever run of this cell (July matrix had it
as "not run — UB gate"; the UB gate is irrelevant for static matching, so it was run now).

## Tool configuration
- PtrTrans-C2Rust (FudanSELab) checkout `b20d5bb`, working-tree patches as archived in
  `../_tool_patches/ptrtrans_b20d5bb_script.patch` (the three July path fixes: no `pub mod lib`
  self-import, struct-path FileNotFound guard, `usage_paths is None` guard). No other changes.
- Mode `--translate_mode Trans_PA` (full system: KG + SVF pointer analysis + Rust annotation),
  model `--model_name gpt-5.1`, key via `OPENAI_API_KEY` env (never written to disk here).
- Pointer-analysis pre-pass: bundled LLVM-14 `clang -S -emit-llvm -g -O0 -Xclang -disable-O0-optnone
  -fno-discard-value-names` on `urlparser_expanded_dealed/test.c`, `llvm-link`, then the prebuilt
  `ptrtrans_sa/pa_func` + `pa_struct` (both finished in < 1 s; reports in
  `input/urlparser_expanded_dealed/svf_analysis_output/*.json`).
- Library selection is a source edit: `script/main.py` `project_list = ['crown_dataset/urlparser']`.

## Source version / input layout
- C input = PtrTrans's own `dataset/crown_dataset/urlparser/` (jwerle `url.h` header-only library +
  70-line `test.c`, md5-identical to the CROWN / Laertes copies used in group A).
- Only input-side change: `compile_commands.json` rewritten from the upstream form
  (`cc -c -std=c99 -Wall -I. -o url-test test.c`, `directory` pointing at the upstream author's
  machine) to the same shape the other libraries use (`cc -I. -c -o test.o test.c`, local
  `directory`). Reason: PtrTrans's macro expander strips `-c`/`-o`/`*.o`/`*.c` but not the bare
  `url-test` operand, so the upstream command fails at `clang -E` (see `dry_run1.log`).
- Stale upstream KG cache `dataset/parsed_projects/urlparser_*` moved aside so the KG was
  re-extracted locally.

## Commands
1. `dry_run1.log` / `dry_run2.log`: dummy key, `$0`; produce `urlparser_expanded[_dealed]`, KG,
   and stop at the first LLM call (401). dry_run1 = failed expansion (upstream compile command);
   dry_run2 = clean, 1/1 file expanded.
2. PA pre-pass (see above).
3. `run1.log`: real key, `python main.py --translate_mode Trans_PA --model_name gpt-5.1`
   (cwd `PtrTrans-C2Rust/script`, backgrounded; started 02:06, finished 02:13 UTC).

## Completion state
- 26 metadata records: 1 project_tree, 2 file, 23 code units. Code units:
  15 `No_Fix_Compile_Success`, 5 `Fixed_1_Compile_Success`, 1 `Fixed_3_Compile_Success`
  (`url_parse`), **1 `Fixed_5_Compile_Failed` → stub revert (`url_get_auth`)**, 1 `Free_Function`
  (`url_free`, elided by design — the crate owns memory; no Rust counterpart).
- `cargo check` passes (20 warnings). Crate: `src/lib.rs`, `src/common/url_mod.rs` (1099 lines,
  20 functions), `src/test.rs` (`main` + 2 helper fns).
- Output class: **complete, builds; 1 stub (`url_get_auth`), 1 elided (`url_free`)**.
- Cost: PtrTrans does not log token usage; `urlparser_runLog.txt` = 438 KB of prompts+responses
  (≈110k tokens total), so well under $5 at gpt-5.1 prices. Not measured exactly.

## Archive contents
- `PA_trans_projects/urlparser/` — the emitted crate (`.git` per-unit history and `target/` dropped — PtrTrans commits build artifacts into it; per-unit code is in the metadata jsonl)
- `PA_trans_projects/urlparser_Trans_PA_trans_metadata.jsonl`, `urlparser_runLog.txt`
- `input/` — staged C input, expanded sources, KG json, SVF reports (`.ll`/`.dot` dumps dropped)
