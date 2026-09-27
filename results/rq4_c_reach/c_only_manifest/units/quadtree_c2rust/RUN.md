# C-guided companion — quadtree_c2rust

Only the C side was fuzzed and measured in this run. Rust coverage was not recalculated.
The earlier C replay of the archived Rust-guided corpus is a labelled reference.

Concurrent boundary fuzzers: 17 (hard cap 28).

| corpus | C functions | C regions |
|---|---:|---:|
| archived Rust-guided, replayed on C | 20 / 24 (0.833) | 106 / 205 (0.517) |
| new C-guided | 20 / 24 (0.833) | 106 / 205 (0.517) |

Checks: `{'build_set_exact': True, 'only_c_was_fuzzed': True, 'fuzzer_cap_respected': True, 'c_measurement_nonempty': True, 'c_input_count_exact': True}`.

This is a reach diagnostic. C-side completion is not proof of defined behavior, and new candidates are not defects without confirmation.
