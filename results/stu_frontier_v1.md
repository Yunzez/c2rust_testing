# STU frontier selector v1 — strategy comparison (Layer 2, Steps 2-3)

Hard-threshold bottom-up antichain; fixed interpretable risk (no model, no training). Cells are **#harness / covered-funcs (risk-exposed harnesses)** — all computable without fuzzing. A *risk-exposed* harness reaches a RISKY/BLOCKED node (a likely false-divergence source). Fewer exposed at comparable coverage = better boundary choice.

| program | funcs | root | all-constructible | leaf-only | **STU frontier** |
|---|--:|---|---|---|---|
| tinyexpr | 29 | 11/29 (0) | 4/18 (0) | 16/17 (0) | **3/18 (0)** |
| bignum | 27 | 9/27 (4) | 27/27 (9) | 17/17 (3) | **17/18 (0)** |
| regex | 18 | 2/18 (1) | 11/17 (2) | 8/8 (0) | **8/9 (0)** |
| hash_table | 8 | 1/8 (1) | 3/8 (3) | 3/3 (1) | **0/0 (0)** |
| opcode_dispatch | 8 | 5/8 (5) | 8/8 (7) | 3/3 (2) | **1/1 (0)** |
| div_mod | 5 | 3/5 (2) | 5/5 (2) | 4/4 (2) | **1/3 (0)** |
| kv_config | 5 | 1/5 (1) | 5/5 (1) | 2/2 (0) | **3/4 (0)** |
| matrix_reduce | 5 | 1/5 (1) | 5/5 (4) | 3/3 (2) | **1/1 (0)** |
| negate_abs | 5 | 3/5 (2) | 5/5 (2) | 4/4 (2) | **1/3 (0)** |
| reduce_overflow | 5 | 3/5 (3) | 5/5 (5) | 4/4 (4) | **0/0 (0)** |
| shift_ops | 5 | 3/5 (1) | 5/5 (1) | 4/4 (1) | **2/4 (0)** |
| tiny_vm | 5 | 1/5 (1) | 5/5 (3) | 3/3 (1) | **2/2 (0)** |
| bitutils | 4 | 1/4 (1) | 4/4 (1) | 3/3 (0) | **3/3 (0)** |
| intmath | 4 | 1/4 (0) | 4/4 (0) | 3/3 (0) | **1/4 (0)** |
| mergesort_search | 4 | 1/4 (1) | 4/4 (3) | 2/2 (1) | **1/1 (0)** |
| rpn_eval | 4 | 1/4 (1) | 4/4 (2) | 3/3 (1) | **2/2 (0)** |
| safe_stats | 4 | 4/4 (2) | 4/4 (2) | 4/4 (2) | **2/2 (0)** |
| sub_overflow | 4 | 2/4 (0) | 4/4 (0) | 3/3 (0) | **2/4 (0)** |
| word_tokens | 4 | 1/4 (0) | 4/4 (0) | 3/3 (0) | **1/4 (0)** |
| array_map_reduce | 3 | 1/3 (0) | 0/0 (0) | 2/2 (0) | **0/0 (0)** |
| array_transforms | 3 | 2/3 (0) | 3/3 (0) | 2/2 (0) | **2/3 (0)** |
| bounded_queue | 3 | 3/3 (3) | 3/3 (3) | 3/3 (3) | **0/0 (0)** |
| bounded_stack | 3 | 3/3 (3) | 3/3 (3) | 3/3 (3) | **0/0 (0)** |
| bracket_balance | 3 | 2/3 (0) | 3/3 (0) | 2/2 (0) | **2/3 (0)** |
| byte_classify | 3 | 1/3 (0) | 3/3 (0) | 2/2 (0) | **1/3 (0)** |
| gap_buffer | 3 | 3/3 (3) | 3/3 (3) | 3/3 (3) | **0/0 (0)** |
| glob_match | 3 | 1/3 (1) | 3/3 (3) | 1/1 (1) | **0/0 (0)** |
| histogram | 3 | 3/3 (1) | 3/3 (1) | 3/3 (1) | **2/2 (0)** |
| leb128 | 3 | 1/3 (0) | 3/3 (0) | 2/2 (0) | **1/3 (0)** |
| base64 | 2 | 2/2 (2) | 2/2 (2) | 2/2 (2) | **0/0 (0)** |
| hex_encode | 2 | 1/2 (0) | 2/2 (0) | 1/1 (0) | **1/2 (0)** |
| pod_config | 2 | 2/2 (1) | 2/2 (1) | 2/2 (1) | **1/1 (0)** |
| postfix_machine | 2 | 1/2 (0) | 2/2 (0) | 1/1 (0) | **1/2 (0)** |
| ring_buffer | 2 | 2/2 (2) | 2/2 (2) | 2/2 (2) | **0/0 (0)** |
| sorted_insert | 2 | 1/2 (1) | 2/2 (2) | 1/1 (1) | **0/0 (0)** |
| mu_atoi | 1 | 1/1 (1) | 1/1 (1) | 1/1 (1) | **0/0 (0)** |
| mu_llabs | 1 | 1/1 (1) | 1/1 (1) | 1/1 (1) | **0/0 (0)** |
| mu_memchr | 1 | 1/1 (0) | 0/0 (0) | 1/1 (0) | **0/0 (0)** |
| mu_memcmp | 1 | 1/1 (0) | 0/0 (0) | 1/1 (0) | **0/0 (0)** |

## Frontier detail (deep programs)

### tinyexpr (29 funcs, 29 matched, 0 rust-only helpers)
- selected STU roots: `e, pi, te_interp`
  - add: not constructible as a standalone boundary
  - comma: not constructible as a standalone boundary
  - divide: not constructible as a standalone boundary
  - mul: not constructible as a standalone boundary
  - negate: not constructible as a standalone boundary
  - npr: not constructible as a standalone boundary
  - fac: not constructible as a standalone boundary
  - ncr: not constructible as a standalone boundary
### bignum (27 funcs, 27 matched, 0 rust-only helpers)
- selected STU roots: `bignum_and, bignum_from_string, bignum_add, bignum_assign, bignum_cmp, bignum_dec, bignum_init, bignum_sub, _lshift_word, bignum_from_int, bignum_inc, bignum_rshift, bignum_lshift, bignum_or, bignum_is_zero, bignum_to_string, bignum_xor`
  - bignum_isqrt: sunk past RISKY _rshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - _rshift_one_bit: sunk past RISKY _rshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - bignum_mul: sunk past RISKY bignum_mul (unguarded signed UB op (intrinsic-UB risk))
  - bignum_mod: sunk past RISKY _lshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - bignum_divmod: sunk past RISKY _lshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - bignum_div: sunk past RISKY _lshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - _lshift_one_bit: sunk past RISKY _lshift_one_bit (unguarded signed UB op (intrinsic-UB risk))
  - bignum_pow: sunk past RISKY bignum_mul (unguarded signed UB op (intrinsic-UB risk))
### regex (18 funcs, 18 matched, 0 rust-only helpers)
- selected STU roots: `re_compile, matchdigit, matchalphanum, matchwhitespace, ismetachar, matchmetachar, matchrange, matchdot`
  - re_match: sunk past RISKY matchcharclass (unguarded signed UB op (intrinsic-UB risk))
  - re_matchp: sunk past RISKY matchcharclass (unguarded signed UB op (intrinsic-UB risk))
  - matchpattern: sunk past RISKY matchcharclass (unguarded signed UB op (intrinsic-UB risk))
  - matchone: sunk past RISKY matchcharclass (unguarded signed UB op (intrinsic-UB risk))
  - matchcharclass: sunk past RISKY matchcharclass (unguarded signed UB op (intrinsic-UB risk))
  - re_print: not constructible as a standalone boundary
### hash_table (8 funcs, 8 matched, 0 rust-only helpers)
- selected STU roots: `(none)`
  - ht_run: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
  - ht_free: not constructible as a standalone boundary
  - ht_init: not constructible as a standalone boundary
  - ht_insert: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
  - ht_insert_into: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
  - ht_hash: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
  - ht_grow: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
  - ht_lookup: sunk past RISKY ht_hash (unguarded signed UB op (intrinsic-UB risk))
### opcode_dispatch (8 funcs, 8 matched, 0 rust-only helpers)
- selected STU roots: `dispatch_table`
  - op_add: sunk past RISKY vm_push (unmasked struct-field index (isolation risk))
  - vm_push: sunk past RISKY vm_push (unmasked struct-field index (isolation risk))
  - vm_pop: sunk past RISKY vm_pop (unmasked struct-field index (isolation risk))
  - op_dup: sunk past RISKY vm_push (unmasked struct-field index (isolation risk))
  - op_mul: sunk past RISKY vm_push (unmasked struct-field index (isolation risk))
  - op_push: sunk past RISKY vm_push (unmasked struct-field index (isolation risk))
  - run_program: sunk past RISKY run_program (unguarded signed UB op (intrinsic-UB risk))

## Failures

- `graph_dfs`: RecursionError: maximum recursion depth exceeded
- `linked_list`: RecursionError: maximum recursion depth exceeded
