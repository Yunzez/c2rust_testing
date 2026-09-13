# Released-artifact baseline comparison

Generated from `results/baseline_artifacts/results.json`; do not edit counts by hand.

## Funnel

| Baseline | Submitted | Accepted | Compiled | Completed | Detected |
|---|---:|---:|---:|---:|---:|
| RustAssure | 3/36 | 3/36 | 2/36 | 2/36 | 1/36 |
| FLOURINE | 24/36 | 18/36 | 17/36 | 16/36 | 14/36 |
| VERT | 0/36 | 0/36 | 0/36 | 0/36 | 0/36 |

## Final outcomes

| Baseline | Detected | Missed | Unsupported | Analysis failure | Compile failure | Not run |
|---|---:|---:|---:|---:|---:|---:|
| RustAssure | 1 | 1 | 2 | 0 | 1 | 31 |
| FLOURINE | 14 | 2 | 3 | 7 | 1 | 9 |
| VERT | 0 | 0 | 36 | 0 | 0 | 0 |

## Defect IDs

### RustAssure

- `detected`: C12
- `missed`: S17
- `unsupported_input`: C5, S13
- `analysis_failure`: none
- `compile_failure`: C1
- `not_run`: C10, C11, C13, C15, C16, C2, C3, C4, C6, C7, C8, C9, S1, S10, S11, S12, S14, S15, S16, S18, S19, S2, S20, S21, S3, S4, S5, S6, S7, S8, S9

### FLOURINE

- `detected`: C1, C12, C2, C4, S1, S15, S16, S2, S21, S4, S5, S6, S7, S9
- `missed`: C16, S17
- `unsupported_input`: C5, S13, S8
- `analysis_failure`: C7, C8, S10, S11, S12, S14, S3
- `compile_failure`: C13
- `not_run`: C10, C11, C15, C3, C6, C9, S18, S19, S20

### VERT

- `detected`: none
- `missed`: none
- `unsupported_input`: C1, C10, C11, C12, C13, C15, C16, C2, C3, C4, C5, C6, C7, C8, C9, S1, S10, S11, S12, S13, S14, S15, S16, S17, S18, S19, S2, S20, S21, S3, S4, S5, S6, S7, S8, S9
- `analysis_failure`: none
- `compile_failure`: none
- `not_run`: none
