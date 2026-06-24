# P(valid | x) baseline — generic vs boundary-specific features

Plain logistic regression, **program-grouped 5-fold CV** (train/test never share a program). Label `validity_v2`, predicting INVALID. 100 boundaries (66 valid : 34 invalid). Held-out pooled AUC per task × feature group:

| task | generic | boundary_specific | combined |
|---|---|---|---|
| lumped (all invalid) | 0.719 | 0.656 | 0.686 |
| valid vs isolation_invariant | 0.742 | 0.845 | 0.836 |
| valid vs intrinsic_ub | 0.581 | 0.672 | 0.741 |

## Honest reading

- **On the LUMPED task generic ≈ or > boundary-specific.** A size-confound control (`results/validity_baseline_size_control_v1.md`) tested whether this is a size artifact and **found it is NOT**: under grouped CV `size_only` ≈ chance and ablating size does not hurt generic — grouped CV already neutralizes the within-sample size separation. generic's residual lumped signal is its NON-size signature features (pointer / nested-pointer / alloc counts), a coarse proxy for the same struct/pointer risk our rf features target. combined < generic is small-data overfit (33 features, ~17 invalid-bearing programs).
- **Split by MECHANISM, the picture is the intended one:** for `invalid_isolation_invariant`, boundary-specific features win the grouped-CV baseline (**0.845** vs generic 0.742; single-feature `rf_struct_index_field` 0.88, `rf_unmasked_field_index` 0.79). For `invalid_intrinsic_ub`, **generic is near-random** (0.581) while combined reaches **0.741** — the signal is the guard×op INTERACTION (`rf_unguarded_ubop`) a linear model needs that engineered term to see.
- **Takeaway:** the lumped task stays hard to claim because generic NON-size signature features (pointer / nested-pointer / alloc counts) act as coarse proxies for the same pointer/struct risk — the size hypothesis was tested and REJECTED (grouped CV already neutralizes size; see `validity_baseline_size_control_v1.md`). The robust claim is per-mechanism: semantic-risk features explain isolation invariants and intrinsic-UB better than generic structure, especially when arithmetic guards matter. Next: close generator gaps to grow labels, then external programs for generalization.

> This is a baseline (no tuning) on the audited `validity_v2`; AUC pooled across grouped folds; ~17 programs carry invalids, so treat magnitudes as indicative.

