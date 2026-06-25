# Call-graph plumbing + depth audit (Layer 2, Step 0)

48/48 pairs processed; 0 plumbing failures.

**Frontier choice** = C call graph has a longest chain >= 3 AND >= 1 internal node (neither pure root nor pure leaf). Only these programs make root/leaf/all/frontier strategies differ — i.e. only these can populate the headline table meaningfully.

| program | C funcs | C depth | C internal | maxfan | recurse | indirect | map cov | rust_only | frontier? |
|---|--:|--:|--:|--:|--:|--:|--:|--:|:--:|
| hash_table | 8 | 5 | 4 | 4 | 0 | 0 | 1.00 | 0 | YES |
| dynamic_array | 7 | 3 | 1 | 5 | 0 | 0 | 1.00 | 0 | YES |
| kv_config | 5 | 3 | 2 | 3 | 0 | 0 | 1.00 | 0 | YES |
| matrix_reduce | 5 | 3 | 1 | 3 | 0 | 0 | 1.00 | 0 | YES |
| tiny_vm | 5 | 3 | 1 | 2 | 1 | 0 | 1.00 | 0 | YES |
| mergesort_search | 4 | 3 | 1 | 2 | 1 | 0 | 1.00 | 0 | YES |
| glob_match | 3 | 3 | 1 | 1 | 1 | 0 | 1.00 | 0 | YES |
| opcode_dispatch | 8 | 2 | 0 | 2 | 0 | 1 | 1.00 | 0 | — |
| state_machine | 6 | 2 | 0 | 1 | 0 | 1 | 1.00 | 0 | — |
| div_mod | 5 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| graph_dfs | 5 | 2 | 0 | 4 | 1 | 0 | 1.00 | 0 | — |
| linked_list | 5 | 2 | 0 | 4 | 1 | 0 | 1.00 | 0 | — |
| negate_abs | 5 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| reduce_overflow | 5 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| shift_ops | 5 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| bitutils | 4 | 2 | 0 | 3 | 0 | 0 | 1.00 | 0 | — |
| intmath | 4 | 2 | 0 | 3 | 1 | 0 | 1.00 | 0 | — |
| rpn_eval | 4 | 2 | 0 | 3 | 0 | 0 | 1.00 | 0 | — |
| sub_overflow | 4 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| word_tokens | 4 | 2 | 0 | 3 | 0 | 0 | 1.00 | 0 | — |
| array_map_reduce | 3 | 2 | 0 | 2 | 0 | 2 | 1.00 | 0 | — |
| array_transforms | 3 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| bracket_balance | 3 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| byte_classify | 3 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| leb128 | 3 | 2 | 0 | 2 | 0 | 0 | 1.00 | 0 | — |
| hex_encode | 2 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| postfix_machine | 2 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| rle_codec | 2 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| sorted_insert | 2 | 2 | 0 | 1 | 0 | 0 | 1.00 | 0 | — |
| safe_stats | 4 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| bounded_queue | 3 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| bounded_stack | 3 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| gap_buffer | 3 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| histogram | 3 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| prefix_runs | 3 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| base64 | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| case_fold | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| graph_bfs | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| pod_config | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| ring_buffer | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| unsafe_decode | 2 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_atoi | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_llabs | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_memchr | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_memcmp | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_strlen | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_strncmp | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |
| mu_strspn | 1 | 1 | 0 | 0 | 0 | 0 | 1.00 | 0 | — |

## Verdict

- **Programs with a real frontier choice: 7/48.** The rest are too shallow (depth<3 or no internal node) for frontier selection to differ from root/leaf/all.
- C call-graph depth: max 5, median 2.
- Mapping health: mean name-match coverage 1.0 (c2rust #[no_mangle] => near-free mapping); total rust_only (absorbed-helper candidates) 0.
- Plumbing: 48/48 pairs ran clean.
