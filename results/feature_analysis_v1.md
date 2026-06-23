# Feature-vs-validity analysis v1 (no model)

Authoritative label `validity_v2`. Binary: **66 valid : 34 invalid** (invalid = isolation_invariant + intrinsic_ub). weak/excluded dropped. Source: `boundaries_v2.jsonl`. AUC is rank-based (no deps); the dataset is small so treat separations as descriptive, not significance-tested.

## Key finding

**No single current static feature strongly separates valid from invalid** — best |AUC-0.5| ≈ 0.27 lumped, ≈ 0.31 per-mechanism. The only consistent (weak) signal is size/complexity: invalids are slightly SMALLER/simpler (fewer nodes, loops, lower cyclomatic), because `invalid_intrinsic_ub` are tiny scalar-arithmetic functions and `invalid_isolation_invariant` are small accessors. The signature counts (`n_pointer_params`, `n_nested_pointer_params`, `returns_pointer`, …) do NOT separate. **Implication:** validity is not explained by generic complexity/signature features; it needs boundary-specific features that capture the semantic risk — e.g. *unguarded signed arithmetic on a scalar* (predicts intrinsic_ub) and *a struct/buffer indexed by a trusted field without a bounds check* (predicts isolation_invariant). Engineering those features is the next step BEFORE any model; the weak generic baseline here is the motivation.

## Lumped: valid vs ALL invalid (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| c_nodes | 41.23 | 38.65 | 0.23 | 0.27 |
| r_nodes | 57.70 | 59.82 | 0.26 | 0.24 |
| c_cyclomatic | 3.70 | 3.47 | 0.27 | 0.23 |
| r_cyclomatic | 3.79 | 3.68 | 0.27 | 0.23 |
| c_stmts | 8.71 | 8.56 | 0.27 | 0.23 |
| rf_compares | 2.48 | 1.94 | 0.28 | 0.22 |
| r_stmts | 8.97 | 8.50 | 0.29 | 0.21 |
| rf_struct_index_field | 0.09 | 0.47 | 0.69 | 0.19 |
| rf_unmasked_field_index | 0.00 | 0.71 | 0.68 | 0.18 |
| rf_struct_ptr | 0.12 | 0.47 | 0.67 | 0.17 |

## valid vs invalid_intrinsic_ub (15) (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| r_nodes | 57.70 | 48.40 | 0.29 | 0.21 |
| c_nodes | 41.23 | 34.27 | 0.29 | 0.21 |
| rf_internal | 0.45 | 0.07 | 0.31 | 0.19 |
| d_nodes | 16.83 | 14.13 | 0.33 | 0.17 |
| size_ratio | 1.38 | 1.24 | 0.35 | 0.15 |
| rf_compares | 2.48 | 2.00 | 0.35 | 0.15 |
| c_cyclomatic | 3.70 | 3.40 | 0.35 | 0.15 |
| c_stmts | 8.71 | 8.93 | 0.36 | 0.14 |
| r_cyclomatic | 3.79 | 3.60 | 0.36 | 0.14 |
| r_pointer_intensity | 4.55 | 3.13 | 0.37 | 0.13 |

## valid vs invalid_isolation_invariant (19) (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| rf_struct_index_field | 0.09 | 0.84 | 0.88 | 0.38 |
| rf_struct_ptr | 0.12 | 0.84 | 0.86 | 0.36 |
| c_nodes | 41.23 | 42.11 | 0.19 | 0.31 |
| r_cyclomatic | 3.79 | 3.74 | 0.19 | 0.31 |
| c_cyclomatic | 3.70 | 3.53 | 0.20 | 0.30 |
| c_stmts | 8.71 | 8.26 | 0.20 | 0.30 |
| rf_unmasked_field_index | 0.00 | 1.21 | 0.79 | 0.29 |
| rf_field_index | 0.02 | 1.21 | 0.78 | 0.28 |
| r_stmts | 8.97 | 8.74 | 0.23 | 0.27 |
| rf_compares | 2.48 | 1.89 | 0.23 | 0.27 |

## Stratified by boundary scope

| scope | valid | invalid | other |
|---|---|---|---|
| public | 36 | 26 | 4 |
| internal | 30 | 8 | 23 |

## Negative mechanism breakdown

| mechanism | n |
|---|---|
| invalid_isolation_invariant | 19 |
| invalid_intrinsic_ub | 15 |

## Label provenance (audit_status x validity_v2)

| audit_status | valid | invalid | weak/excluded |
|---|---|---|---|
| auto | 43 | 29 | 0 |
| reviewed | 23 | 5 | 15 |
| n/a | 0 | 0 | 12 |

## Per-program grouped-CV feasibility

- 36 programs contribute ≥1 labeled boundary (grouped folds = group by program so train/test never share a program).
- 14 programs have BOTH classes; 17 have ≥1 invalid, 33 have ≥1 valid.
- Recommendation: program-grouped K-fold (K≈5) is feasible (36 groups); stratify so each fold has invalids.

| program | valid | invalid |
|---|---|---|
| opcode_dispatch | 0 | 7 |
| div_mod | 3 | 2 |
| negate_abs | 3 | 2 |
| reduce_overflow | 3 | 2 |
| shift_ops | 3 | 2 |
| bitutils | 4 | 0 |
| intmath | 4 | 0 |
| safe_stats | 2 | 2 |
| sub_overflow | 3 | 1 |
| bounded_queue | 1 | 2 |
| bounded_stack | 1 | 2 |
| bracket_balance | 3 | 0 |
| byte_classify | 3 | 0 |
| gap_buffer | 1 | 2 |
| hash_table | 3 | 0 |
| histogram | 1 | 2 |
| leb128 | 3 | 0 |
| tiny_vm | 2 | 1 |
| word_tokens | 3 | 0 |
| array_transforms | 2 | 0 |
| matrix_reduce | 2 | 0 |
| mergesort_search | 2 | 0 |
| pod_config | 2 | 0 |
| postfix_machine | 1 | 1 |
| prefix_runs | 2 | 0 |
| ring_buffer | 1 | 1 |
| rpn_eval | 0 | 2 |
| sorted_insert | 1 | 1 |
| unsafe_decode | 0 | 2 |
| case_fold | 1 | 0 |
| dynamic_array | 1 | 0 |
| glob_match | 1 | 0 |
| hex_encode | 1 | 0 |
| kv_config | 1 | 0 |
| rle_codec | 1 | 0 |
| state_machine | 1 | 0 |
