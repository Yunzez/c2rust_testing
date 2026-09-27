# PtrTrans × genann — run record (2026-09-02)

**Cell status: AVAILABLE (complete, builds).** Replaces the July-2026 run whose crate was
scratch and not retained ("declaration-only core", translation_matrix note 24). This run is
the archived artifact for RQ1 group B.

## Tool configuration
- PtrTrans-C2Rust checkout `b20d5bb` + the three July path patches
  (`../_tool_patches/ptrtrans_b20d5bb_script.patch`). No translator change.
- `--translate_mode Trans_PA --model_name gpt-5.1`, key via `OPENAI_API_KEY` env.
- PA pre-pass: bundled LLVM-14 clang/llvm-link on `genann_expanded_dealed/genann.c`, then
  `pa_func` + `pa_struct` (reports in `input/genann_expanded_dealed/svf_analysis_output/`).
- Library selection: `script/main.py` `project_list = ['crown_dataset/genann']`.

## Source version / input layout
- C input = PtrTrans's own `dataset/crown_dataset/genann/` (codeplea genann, `genann.c` +
  `genann.h`; md5-identical to the CROWN/Laertes copies except for the edit below).
- `examples/`, `test.c`, `doc/`, `Makefile` and the upstream `compile_commands.json` moved to
  `_cli_aside/` (they are separate main-bearing TUs; the library is the single TU `genann.c`).
  New `compile_commands.json` = `cc -I. -c -o genann.o genann.c`, local `directory`.
- **One input-side C edit (`genann.c`, 8 diff lines, `_cli_aside/genann.c.orig` = original):**
  the four `unused` parameter-attribute uses (`const genann *ann unused, double a` in
  `genann_act_sigmoid`, `genann_act_sigmoid_cached`, `genann_act_linear`,
  `genann_act_threshold`) were removed. Reason: PtrTrans's macro "deal" step duplicates the
  function-header line for `__attribute__((unused))` parameters and produces broken C
  (`function definition is not allowed here`), so the as-shipped input fails at expansion —
  `dry_run1_asshipped_unused_macro_breaks_expander.log`. This expander defect is the mechanism
  behind July's "declaration-only core" artifact (the slicer then saw only declarations). The
  edit changes no semantics (`unused` only silences a warning).

## Commands
1. `dry_run1_asshipped_unused_macro_breaks_expander.log` — dummy key; as-shipped genann.c;
   expansion fails.
2. `dry_run2.log` — dummy key; edited genann.c; 1/1 file expanded, KG built, stops at the
   first LLM call (401).
3. PA pre-pass (see above; both SA binaries < 1 s).
4. `run1.log` — real key, `python main.py --translate_mode Trans_PA --model_name gpt-5.1`
   (cwd `PtrTrans-C2Rust/script`, backgrounded; 02:23 → 02:26 UTC).

## Completion state
- Metadata: 21 records = 1 project_tree, 2 file, **18 code units**: 11 `No_Fix_Compile_Success`,
  2 `Fixed_1`, 2 `Fixed_2`, **3 `Fixed_5_Compile_Failed` → stub reverts**, 1 `Free_Function`
  (`genann_free`, elided by design).
- The 3 stub reverts: `genann_randomize` (body "Stub implementation: does nothing") and the two
  **header-declaration units** `genann_act_threshold#genann.h#471`, `genann_act_linear#genann.h#473`
  (body `0.0`). PtrTrans's KG treats these two header declarations as units of their own; their
  stubs land in `src/common/genann_mod.rs`, and the real definitions
  (`genann_act_threshold#genann.c#1636`, `genann_act_linear#genann.c#1632`, both
  `No_Fix_Compile_Success`) then emit **no code** (`trans_rust_code` empty, `rust_definition_name []`)
  because the name already exists. Net effect: the crate's `genann_act_threshold` /
  `genann_act_linear` are stubs even though the translator "succeeded" on the definitions.
  This is a translator-side slicing artifact (same family as July's note 24) and is recorded
  as such; nothing was patched.
- `cargo check` passes (17 warnings). Crate: `src/lib.rs`, `src/genann.rs` (595 lines, 13 fns +
  `struct Genann`), `src/common/genann_mod.rs` (2 stub fns).
- Output class: **complete, builds; 3 stubs (`genann_randomize`, `genann_act_threshold`,
  `genann_act_linear`), 1 elided (`genann_free`)**. Function names all equal the C names;
  struct `genann` → `Genann`.
- Cost: not logged by the tool; `genann_runLog.txt` = 247 KB of prompts+responses (≈60k
  tokens) → ≈ $2 at gpt-5.1 prices. Estimate, not measured.

## Archive contents
- `PA_trans_projects/genann/` — the emitted crate (`.git` per-unit history and `target/` dropped — PtrTrans commits build artifacts into it; per-unit code is in the metadata jsonl)
- `PA_trans_projects/genann_Trans_PA_trans_metadata.jsonl`, `genann_runLog.txt`
- `input/genann/` (staged C incl. `_cli_aside/`), `input/genann_expanded[_dealed]/` (expanded
  sources, SVF reports; `.ll`/`.dot` dropped), `input/parsed_projects/` (KG json, ProjectInfo)
