# G1 support matrix

DUR=30s/program; shared LibAFL build; each artifact classified by classify_artifact.py.
Labels: NO_DIVERGENCE_OBSERVED (full run, no artifact) / FUZZER_EXITED_EARLY / UNSUPPORTED_SIGNATURE / BUILD_FAIL / per-artifact classifier labels.

| program | entry | generator_supported | built | elapsed_seconds | terminated_by_timeout | artifact_count | root_cause_count | label |
|---|---|---|---|---|---|---|---|---|
| array_map_reduce | map_then_reduce | False | False | None | None | 0 | 0 | UNSUPPORTED_SIGNATURE |
| bitutils | bitutils_eval | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| dynamic_array | da_run | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| glob_match | glob_match | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| graph_dfs | count_reachable | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| hash_table | ht_run | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| intmath | intmath_eval | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| kv_config | kv_parse | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| leb128 | leb128_roundtrip | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| linked_list | ll_run | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| matrix_reduce | matrix_transpose_and_sum | False | False | None | None | 0 | 0 | UNSUPPORTED_SIGNATURE |
| mergesort_search | sort_and_find | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| opcode_dispatch | run_program | True | True | 1.0 | False | 1 | 1 | C_UB_CONFIRMED |
| rle_codec | rle_encode | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| rpn_eval | rpn_eval | True | True | 1.0 | False | 1 | 1 | C_UB_CONFIRMED |
| state_machine | simulate | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| tiny_vm | vm_run | True | True | 30.0 | True | 0 | 0 | NO_DIVERGENCE_OBSERVED |
| word_tokens | fold_unique_words | False | False | None | None | 0 | 0 | UNSUPPORTED_SIGNATURE |
