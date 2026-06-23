# G1 support matrix

DUR=30s per program; classifier = classify_artifact.py

| program | entry | generator_supported | built | artifact_count | label |
|---|---|---|---|---|---|
| array_map_reduce | map_then_reduce | False | False | 0 | UNSUPPORTED_SIGNATURE |
| bitutils | bitutils_eval | True | True | 0 | CLEAN |
| dynamic_array | da_run | True | True | 0 | CLEAN |
| glob_match | glob_match | True | True | 0 | CLEAN |
| graph_dfs | count_reachable | False | False | 0 | UNSUPPORTED_SIGNATURE |
| hash_table | ht_run | True | True | 0 | CLEAN |
| intmath | intmath_eval | True | True | 0 | CLEAN |
| kv_config | kv_parse | True | True | 0 | CLEAN |
| leb128 | leb128_roundtrip | True | True | 0 | CLEAN |
| linked_list | ll_run | True | True | 0 | CLEAN |
| matrix_reduce | matrix_transpose_and_sum | False | False | 0 | UNSUPPORTED_SIGNATURE |
| mergesort_search | sort_and_find | True | True | 0 | CLEAN |
| opcode_dispatch | run_program | True | True | 1 | C_UB_CONFIRMED |
| rle_codec | rle_encode | True | True | 0 | CLEAN |
| rpn_eval | rpn_eval | True | True | 1 | C_UB_CONFIRMED |
| state_machine | simulate | True | True | 0 | CLEAN |
| tiny_vm | vm_run | True | True | 0 | CLEAN |
| word_tokens | fold_unique_words | False | False | 0 | UNSUPPORTED_SIGNATURE |
