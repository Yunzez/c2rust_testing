# C-guided companion — qsort_sactor

Only the C side was fuzzed and measured in this run. Rust coverage was not recalculated.
The earlier C replay of the archived Rust-guided corpus is a labelled reference.

Concurrent boundary fuzzers: 3 (hard cap 28).

| corpus | C functions | C regions |
|---|---:|---:|
| archived Rust-guided, replayed on C | 3 / 3 (1.000) | 10 / 10 (1.000) |
| new C-guided | 3 / 3 (1.000) | 10 / 10 (1.000) |

Checks: `{'build_set_exact': True, 'only_c_was_fuzzed': True, 'fuzzer_cap_respected': True, 'c_measurement_nonempty': True, 'c_input_count_exact': True}`.

This is a reach diagnostic. C-side completion is not proof of defined behavior, and new candidates are not defects without confirmation.
