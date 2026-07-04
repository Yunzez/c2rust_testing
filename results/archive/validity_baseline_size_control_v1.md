# Size-confound control for P(valid|x)

Same plain logistic regression, program-grouped 5-fold CV, label `validity_v2` (predicting INVALID). Held-out pooled AUC.

## A · Ablation — is the generic edge just size?

| task | size_only | generic(full) | generic_ablated | boundary_specific | combined_ablated |
|---|---|---|---|---|---|
| lumped | 0.362 | 0.690 | 0.709 | 0.702 | 0.711 |
| valid vs isolation | 0.279 | 0.767 | 0.640 | 0.865 | 0.820 |
| valid vs intrinsic_ub | 0.581 | 0.591 | 0.549 | 0.749 | 0.770 |

## B · Size-matched subset (each invalid ↔ nearest-size valid by c_stmts)

Mean c_stmts before match — valid 9.1 vs invalid 8.8; after match — valid 7.1 vs invalid 8.8 (size equalized). 74 boundaries.

| feature group | held-out AUC (size-matched) |
|---|---|
| size_only | 0.334 |
| generic (full) | 0.667 |
| boundary_specific | 0.685 |

## Reading (the control OVERTURNS the size-confound hypothesis)

- **Size is NOT the confound.** Under program-grouped CV `size_only` scores **0.362** (lumped) and 0.334 (size-matched) — at/below chance. The descriptive single-feature size separation (c_nodes single-feature |AUC−0.5|≈0.27) was a WITHIN-SAMPLE artifact that grouped CV already neutralizes (different programs have different size baselines, so it does not transfer).
- **The generic edge is NOT size either.** Ablating all size/complexity features gives `generic_ablated` **0.709** ≥ `generic(full)` 0.690 — removing size does not hurt. So generic's lumped signal comes from the NON-size signature features (pointer/nested-pointer/alloc counts), which are a coarse proxy for the same struct/pointer risk our rf features target.
- **The real claim survives, but it is per-MECHANISM, not lumped.** On the lumped task the coarse pointer-count proxy keeps generic competitive (0.709 vs boundary_specific 0.702). Split by mechanism, semantic-risk features are decisive: isolation boundary_specific 0.865 > generic_ablated 0.640; and for intrinsic-UB generic is **near-random** (0.549) while combined_ablated reaches 0.770 — generic structure simply cannot see whether arithmetic is guarded.

> Corrected conclusion: the earlier 'size confound' explanation does NOT hold — grouped CV already controls size, and generic's residual signal is a pointer-signature proxy. The contribution rests on the PER-MECHANISM result (and especially intrinsic-UB, which generic cannot capture), not on the lumped task. Small-data caveats apply (~17 invalid-bearing programs); external corpora are the next robustness step.

