# Feature-vs-validity analysis v1 (no model)

Authoritative label `validity_v2`. Binary: **73 valid : 37 invalid** (invalid = isolation_invariant + intrinsic_ub). weak/excluded dropped. Source: `boundaries_v4.jsonl`. AUC is rank-based (no deps); the dataset is small so treat separations as descriptive, not significance-tested.

## Key finding

**No single current static feature strongly separates valid from invalid** — best |AUC-0.5| ≈ 0.27 lumped, ≈ 0.31 per-mechanism. The only consistent (weak) signal is size/complexity: invalids are slightly SMALLER/simpler (fewer nodes, loops, lower cyclomatic), because `invalid_intrinsic_ub` are tiny scalar-arithmetic functions and `invalid_isolation_invariant` are small accessors. The signature counts (`n_pointer_params`, `n_nested_pointer_params`, `returns_pointer`, …) do NOT separate. **Implication:** validity is not explained by generic complexity/signature features; it needs boundary-specific features that capture the semantic risk — e.g. *unguarded signed arithmetic on a scalar* (predicts intrinsic_ub) and *a struct/buffer indexed by a trusted field without a bounds check* (predicts isolation_invariant). Engineering those features is the next step BEFORE any model; the weak generic baseline here is the motivation.

## Lumped: valid vs ALL invalid (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| c_nodes | 44.68 | 42.43 | 0.25 | 0.25 |
| r_nodes | 64.25 | 67.43 | 0.28 | 0.22 |
| c_cyclomatic | 3.92 | 3.54 | 0.28 | 0.22 |
| rf_compares | 2.47 | 1.84 | 0.28 | 0.22 |
| c_stmts | 9.07 | 8.78 | 0.28 | 0.22 |
| r_cyclomatic | 4.00 | 3.81 | 0.29 | 0.21 |
| r_stmts | 9.70 | 9.68 | 0.31 | 0.19 |
| rf_struct_index_field | 0.08 | 0.43 | 0.68 | 0.18 |
| rf_unmasked_field_index | 0.00 | 0.65 | 0.66 | 0.16 |
| c_loops | 0.77 | 0.32 | 0.34 | 0.16 |

## valid vs invalid_intrinsic_ub (17) (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| c_nodes | 44.68 | 34.71 | 0.30 | 0.20 |
| r_nodes | 64.25 | 50.76 | 0.30 | 0.20 |
| rf_internal | 0.44 | 0.06 | 0.31 | 0.19 |
| rf_compares | 2.47 | 1.82 | 0.34 | 0.16 |
| c_cyclomatic | 3.92 | 3.35 | 0.35 | 0.15 |
| d_nodes | 20.00 | 16.06 | 0.35 | 0.15 |
| c_stmts | 9.07 | 8.53 | 0.35 | 0.15 |
| r_pointer_intensity | 5.92 | 3.53 | 0.36 | 0.14 |
| rf_signed | 0.66 | 0.94 | 0.64 | 0.14 |
| fuzzability | 0.84 | 0.91 | 0.63 | 0.13 |

## valid vs invalid_isolation_invariant (20) (AUC = P(neg scores higher); 0.5 = no signal)

| feature | mean(valid) | mean(neg) | AUC | |sep| |
|---|---|---|---|---|
| rf_struct_index_field | 0.08 | 0.80 | 0.86 | 0.36 |
| rf_struct_ptr | 0.11 | 0.80 | 0.85 | 0.35 |
| r_cyclomatic | 4.00 | 3.90 | 0.22 | 0.28 |
| c_nodes | 44.68 | 49.00 | 0.22 | 0.28 |
| c_cyclomatic | 3.92 | 3.70 | 0.22 | 0.28 |
| rf_unmasked_field_index | 0.00 | 1.15 | 0.78 | 0.28 |
| c_stmts | 9.07 | 9.00 | 0.23 | 0.27 |
| rf_compares | 2.47 | 1.85 | 0.23 | 0.27 |
| rf_field_index | 0.01 | 1.15 | 0.77 | 0.27 |
| r_stmts | 9.70 | 10.65 | 0.25 | 0.25 |

## Stratified by boundary scope

| scope | valid | invalid | other |
|---|---|---|---|
| public | 41 | 29 | 3 |
| internal | 32 | 8 | 21 |

## Negative mechanism breakdown

| mechanism | n |
|---|---|
| invalid_isolation_invariant | 20 |
| invalid_intrinsic_ub | 17 |

## Label provenance (audit_status x validity_v2)

| audit_status | valid | invalid | weak/excluded |
|---|---|---|---|
| auto | 0 | 0 | 0 |
| reviewed | 4 | 1 | 5 |
| n/a | 0 | 0 | 12 |

## Per-program grouped-CV feasibility

- 42 programs contribute ≥1 labeled boundary (grouped folds = group by program so train/test never share a program).
- 15 programs have BOTH classes; 20 have ≥1 invalid, 37 have ≥1 valid.
- Recommendation: program-grouped K-fold (K≈5) is feasible (42 groups); stratify so each fold has invalids.

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
| array_transforms | 3 | 0 |
| bounded_queue | 1 | 2 |
| bounded_stack | 1 | 2 |
| bracket_balance | 3 | 0 |
| byte_classify | 3 | 0 |
| gap_buffer | 1 | 2 |
| hash_table | 3 | 0 |
| histogram | 1 | 2 |
| leb128 | 3 | 0 |
| mergesort_search | 3 | 0 |
| prefix_runs | 3 | 0 |
| tiny_vm | 2 | 1 |
| word_tokens | 3 | 0 |
| matrix_reduce | 2 | 0 |
| pod_config | 2 | 0 |
| postfix_machine | 1 | 1 |
| ring_buffer | 1 | 1 |
| rpn_eval | 0 | 2 |
| sorted_insert | 1 | 1 |
| unsafe_decode | 0 | 2 |
| base64 | 1 | 1 |
| case_fold | 1 | 0 |
| dynamic_array | 1 | 0 |
| glob_match | 1 | 0 |
| hex_encode | 1 | 0 |
| kv_config | 1 | 0 |
| rle_codec | 1 | 0 |
| state_machine | 1 | 0 |
| mu_atoi | 0 | 1 |
| mu_llabs | 0 | 1 |
| mu_strlen | 1 | 0 |
| mu_strncmp | 1 | 0 |
| mu_strspn | 1 | 0 |
