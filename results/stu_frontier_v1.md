# STU frontier selector — strategy comparison (Layer 2, Steps 2-4)

Hard-threshold bottom-up antichain; fixed interpretable risk (no model, no training). Cells are **#harness / covered-funcs (risk-exposed)** — computable without fuzzing. **v1** = sink below RISKY (collapses where risk is central). **v2** = *guarded rise*: tolerate a RISKY callee when its call is shielded by an input clamp, so the frontier rises to the constraining boundary instead of collapsing. (v2 risk-exposed counts reachable RISKY even when shielded — a static over-count G3 corrects empirically.)

| program | funcs | root | all-constructible | leaf-only | frontier **v1** | frontier **v2** |
|---|--:|---|---|---|---|---|
| g3_three_level | 3 | 1/3 (1) | 3/3 (3) | 1/1 (1) | 0/0 (0) | **1/2 (1)** |

## Frontier detail (deep programs)

