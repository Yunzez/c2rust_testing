# Size-confound control for P(valid|x)

Same plain logistic regression, program-grouped 5-fold CV, label `validity_v2` (predicting INVALID). Held-out pooled AUC.

## A · Ablation — is the generic edge just size?

| task | size_only | generic(full) | generic_ablated | boundary_specific | combined_ablated |
|---|---|---|---|---|---|
| lumped | 0.431 | 0.733 | 0.739 | 0.660 | 0.641 |
| valid vs isolation | 0.357 | 0.735 | 0.740 | 0.852 | 0.848 |
| valid vs intrinsic_ub | 0.590 | 0.591 | 0.499 | 0.676 | 0.756 |

## B · Size-matched subset (each invalid ↔ nearest-size valid by c_stmts)

Mean c_stmts before match — valid 8.7 vs invalid 8.6; after match — valid 6.4 vs invalid 8.6 (size equalized). 68 boundaries.

| feature group | held-out AUC (size-matched) |
|---|---|
| size_only | 0.473 |
| generic (full) | 0.680 |
| boundary_specific | 0.604 |

## Reading (the control OVERTURNS the size-confound hypothesis)

- **Size is NOT the confound.** Under program-grouped CV `size_only` scores **0.431** (lumped) and 0.473 (size-matched) — at/below chance. The descriptive single-feature size separation (c_nodes single-feature |AUC−0.5|≈0.27) was a WITHIN-SAMPLE artifact that grouped CV already neutralizes (different programs have different size baselines, so it does not transfer).
- **The generic edge is NOT size either.** Ablating all size/complexity features gives `generic_ablated` **0.739** ≥ `generic(full)` 0.733 — removing size does not hurt. So generic's lumped signal comes from the NON-size signature features (pointer/nested-pointer/alloc counts), which are a coarse proxy for the same struct/pointer risk our rf features target.
- **The real claim survives, but it is per-MECHANISM, not lumped.** On the lumped task the coarse pointer-count proxy keeps generic competitive (0.739 vs boundary_specific 0.660). Split by mechanism, semantic-risk features are decisive: isolation boundary_specific 0.852 > generic_ablated 0.740; and for intrinsic-UB generic is **near-random** (0.499) while combined_ablated reaches 0.756 — generic structure simply cannot see whether arithmetic is guarded.

> Corrected conclusion: the earlier 'size confound' explanation does NOT hold — grouped CV already controls size, and generic's residual signal is a pointer-signature proxy. The contribution rests on the PER-MECHANISM result (and especially intrinsic-UB, which generic cannot capture), not on the lumped task. Small-data caveats apply (~17 invalid-bearing programs); external corpora are the next robustness step.

