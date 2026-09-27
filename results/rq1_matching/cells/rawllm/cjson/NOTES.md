# cJSON raw-LLM cell (2026-07-07)

- **Source**: `tools/c2rust_crustbench/out/cJSON/src/cJSON.c` — canonical cJSON (Dave Gamble 2009),
  750-line version, **58 C functions** (matches the E1 name-preserving cJSON scorable=58 → same corpus).
- **Translation**: gpt-5.1, disclosed rename-commanding prompt (`experiments/llm_transpiler/prompts/translate.md`).
  Output = idiomatic `JsonValue` enum crate, 44 analyzable Rust fns.
- **Minimal fix applied**: one string-literal transcription typo (`push_str("\\"")` → `push_str("\\\"")`,
  line 530) that made rust-analyzer bail and hide half the fns (14→44 after fix). Naming/structure untouched.

## Result
**matcher_recall = 0.55 (22/40)  ·  name_eq_recall = 0.15 (6/40)**
Ablation: baseline 0.40 → +signal-C 0.50 → +input-element-type 0.55 (input-type does nothing alone —
it only splits the array trio *within* signal-C's `array` tag group). Zero regression across 55
name-preserving libs (two harness runs), clean ablation. See ../../regression/SIGNAL_C.md. The pre-signal-C
analysis below is the 0.40 baseline and explains the failure mode both signals target. Residual not pursued:
True/Bool (bool-literal asymmetry) + ~8 accessor family (the hard core).

name-eq is NOT 0 here: the LLM kept 6 parse-core names verbatim (`parse_number/hex4/string/value/array/object`)
despite the rename instruction. Matcher still 2.6× name-eq, but far below genann 1.00 / urlparser 0.88.

## Why low — the finding
cJSON's 40 scorable fns split into two regimes:
- **Topology-rich recursive core** (parse_*/print_* mutually recursive, ~19 fns): matcher recovers ~16 —
  the call structure pins them even under renaming.
- **Flat leaf constructors/accessors** (`cJSON_Create*` ×12, Add/Detach, ~21 fns): all `()->Value` or
  `(scalar)->Value`, tiny bodies, **call nothing** → zero topology, homogeneous io-shape. The matcher
  permutes them (CreateNull→json_int_array, CreateTrue→as_number_pair, …).

**Takeaway**: matcher recall tracks *topological richness*, NOT raw function count. cJSON is large but
~half flat-leaf API → adversarial. This is the **counterexample to "more functions ⇒ easier"** and a
scaled-up version of the quadtree predicate-cluster miss. Motivates **signal-C (constants/literals)**:
`json_true` vs `json_false` differ only by a bool literal; the Create* leaves need a non-structural
discriminator. Also depressed by LLM-added helper attractors (`json_object_get`/`_get_mut` case-sensitive
getters the C lacked) stealing the GetObjectItem/GetArrayItem matches.

## Dissolved (18/58 — excluded from scorable, counted here)
cJSON_GetErrorPtr (Result), cJSON_strdup (String owns), cJSON_InitHooks (no alloc hooks), cJSON_New_Item
(no manual alloc), cJSON_Delete (Drop), pow2gt (subsumed by reserve), update (String::len), print_string
(merged into print_value string arm), suffix_object, create_reference, cJSON_AddItemToObjectCS (no key-own
distinction), AddItemReferenceToArray/Object, DeleteItemFromArray/Object (detach+Drop),
InsertItemInArray, ReplaceItemInArray, ReplaceItemInObject (folded into json_object_set upsert).
40 scorable + 18 dissolved = 58. ✓
