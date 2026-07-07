# G1 support matrix (in-loop UB-free gate ON)

DUR=15s/program; shared LibAFL build; each artifact classified by classify_artifact.py.
Labels: NO_DIVERGENCE_OBSERVED (full run, no artifact) / FUZZER_EXITED_EARLY / UNSUPPORTED_SIGNATURE / BUILD_FAIL / per-artifact classifier labels.

| program | entry | generator_supported | built | elapsed_seconds | terminated_by_timeout | artifact_count | root_cause_count | label |
|---|---|---|---|---|---|---|---|---|
| div_mod | mod_signed_i32 | True | True | 1.0 | False | 1 | 1 | C_UB_CONFIRMED |
