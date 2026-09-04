# SACTOR × quadtree — run record (2026-09-02)

**Cell status: PARTIAL, non-building.** Rerun of the July-2026 cell (`results/rq4_effectiveness/
bugs/quadtree_sactor/`, whose Rust lived in a scratchpad that no longer exists → "produced but
lost"). Outcome reproduces July exactly: the three leaf TUs translate and verify, the core
`quadtree.c` is **refused** (`ValueError: Circular dependencies for functions is not supported
yet`), the driver TU fails to link. Scored artifact = the **unidiomatic phase** (12 functions,
all SACTOR-verified); the idiomatic phase reached only `point.c` (2 functions). Scored on a
separate PARTIAL line, never in the primary table.

## Tool configuration
- SACTOR checkout `577c3d2` + the three July patches (`../_tool_patches/sactor_577c3d2.patch`).
  No translator change, no cost breaker, no LLM-only/ablation configuration.
- `sactor.toml`: gpt-5.1 via LiteLLM, key `os.environ/OPENAI_API_KEY`. `crown`, `sactor` on PATH.
- Command (cwd = harness dir):
  `sactor translate --type bin -C ./compile_commands.json --test-command-path ./test_task.json
  -r ./result --continue-run-when-incomplete -c .../sactor/sactor.toml`

## Source version / harness (`input/`)
- C = PtrTrans's `dataset/crown_dataset/quadtree/src/` (the copy the PtrTrans cell consumed):
  `point.c`, `bounds.c`, `node.c`, `quadtree.c`, `quadtree.h`, with the **three parser-level
  adaptations documented for the July run** (`bugs/quadtree_sactor/README.md`, "Method note"),
  re-applied identically: `(*key_free)(node->key)` → `key_free(node->key)` (node.c:31),
  `(*descent)(root)` / `(*ascent)(root)` → `descent(root)` / `ascent(root)` (quadtree.c:163,168),
  `INFINITY` → `1e308` (bounds.c:23-24, 4 occurrences). SACTOR's C resolver dies (`USR=None`) on
  explicit-deref call syntax and on `__builtin_inff`; the edits are semantically neutral.
- `driver.c` = the July driver (deterministic LCG insert/search over a fixed-bounds tree),
  `test_samples.json` = the July 12 samples; the reconstructed C reference reproduces all 12
  expected outputs byte-for-byte (checked before launch). `test_task.json` = 12 ×
  `sactor run-tests --type bin ./test_samples.json %t <i> --feed-as-args`.
- `compile_commands.json`: 5 entries `gcc -I<dir> -c -o X.o X.c` (quadtree, node, bounds,
  point, driver).

## Run (`run1.log`, 04:05 → 04:23 UTC)
`run1_result/batch_summary.json`:

| TU | status | detail |
|---|---|---|
| `point.c` | success | unidiomatic 2/2 fns + struct (2 queries); idiomatic 2/2 (6 queries), `specs/function_name_map.json` identity |
| `bounds.c` | success (unidiomatic) | 3/3 fns verified (4 queries; `quadtree_bounds_extend` needed 2 attempts). Idiomatic: struct `quadtree_bounds` failed 6/6, the 3 fns `blocked_by_failed_dependency` (44 queries) |
| `node.c` | failed | unidiomatic 7/7 fns + 3 structs verified (7 queries). Idiomatic: `quadtree_bounds` struct failed again → `Dependency 'quadtree_bounds_new' … should have been translated before use` |
| `quadtree.c` | failed | **`Circular dependencies for functions is not supported yet`** (`insert_` ↔ `split_node_`, plus self-recursion) — refused at dependency analysis, before any LLM call for this TU |
| `driver.c` | failed | `Failed to link project-level harness for function lcg_next` (the harness needs `quadtree_new/insert/search/free` from the refused TU); run then dies with `KeyError: …/driver.c` in SACTOR's project combiner (`run1.log` tail) |

- Output class: **partial, non-building, no usable tool map** (the only
  `function_name_map.json` is point.c's 2-entry identity map). Analyzable Rust: 12 unidiomatic
  function files + struct files, concatenated verbatim into `assembled_unidiomatic.rs`; 2
  idiomatic files (`run1_result/point.c__9ec7d037/translated_code_idiomatic/`). The 12
  `quadtree.c` functions (`quadtree_new`, `insert_`, `split_node_`, `find_`, `quadtree_walk`,
  …) have no Rust at all.
- Cost (from `llm_stat_*.json`, gpt-5.1): 63 queries, 105.9k input / 25.2k output tokens ≈ $0.4
  measured for the 4 stat files, plus the combiner/driver prompts not covered by a stat file
  (≈ 14 "Translated function" events in total) → **≈ $1** for the cell.

## Why this is a shipped-configuration failure
The refusal is SACTOR's own topological-order prerequisite (mutual recursion unsupported); the
July self-recursion patch does not cover it and nothing was patched here. Same failure class
as SACTOR × cJSON (`results/rq4_effectiveness/certificates/cjson_matrix.md`).

## Archive contents
- `input/` — adapted C sources, `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json`
- `run1.log`, `run1_result/` (per-TU `translated_code_{unidiomatic,idiomatic}/`, `*_failure_info.json`,
  `llm_stat_*.json`, `combined/`, `logs/`; `target/` dropped)
- `assembled_unidiomatic.rs` — the scored artifact (header says what it concatenates)
