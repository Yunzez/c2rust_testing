# Released-artifact baseline comparison

Generated from `results/baseline_artifacts/results.json`; do not edit counts by hand.

## Funnel

| Baseline | Submitted | Accepted | Compiled | Completed | Detected | Detected / completed |
|---|---:|---:|---:|---:|---:|---:|
| RustAssure | 34/36 | 34/36 | 12/36 | 11/36 | 5/36 | 5/11 |
| FLOURINE | 33/36 | 20/36 | 17/36 | 16/36 | 14/36 | 14/16 |
| VERT | 36/36 | 32/36 | 14/36 | 1/36 | 0/36 | 0/1 |

## Final outcomes

| Baseline | Detected | Missed | Unsupported | Analysis failure | Compile failure | Not run |
|---|---:|---:|---:|---:|---:|---:|
| RustAssure | 5 | 6 | 2 | 1 | 22 | 0 |
| FLOURINE | 14 | 2 | 3 | 14 | 3 | 0 |
| VERT | 0 | 1 | 4 | 21 | 10 | 0 |

## Defect IDs

### RustAssure

- `detected`: C12, C16, C4, S15, S21
- `missed`: C13, C2, S14, S16, S17, S6
- `unsupported_input`: C5, S13
- `analysis_failure`: C3
- `compile_failure`: C1, C10, C11, C15, C6, C7, C8, C9, S1, S10, S11, S12, S18, S19, S2, S20, S3, S4, S5, S7, S8, S9
- `not_run`: none

### FLOURINE

- `detected`: C1, C12, C2, C4, S1, S15, S16, S2, S21, S4, S5, S6, S7, S9
- `missed`: C16, S17
- `unsupported_input`: C5, S13, S8
- `analysis_failure`: C10, C11, C15, C3, C7, C8, S10, S11, S12, S14, S18, S19, S20, S3
- `compile_failure`: C13, C6, C9
- `not_run`: none

### VERT

- `detected`: none
- `missed`: C13
- `unsupported_input`: C3, C5, S13, S8
- `analysis_failure`: C12, C16, C2, C4, C7, C8, C9, S1, S10, S11, S12, S14, S15, S16, S17, S2, S21, S3, S4, S7, S9
- `compile_failure`: C1, C10, C11, C15, C6, S18, S19, S20, S5, S6
- `not_run`: none
