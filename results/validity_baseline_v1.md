# P(valid | x) baseline — generic vs boundary-specific features

Plain logistic regression, **program-grouped 5-fold CV** (train/test never share a program). Label `validity_v2`, predicting INVALID. 100 boundaries (66 valid : 34 invalid). Held-out pooled AUC per task × feature group:

| task | generic | boundary_specific | combined |
|---|---|---|---|
| lumped (all invalid) | 0.719 | 0.656 | 0.686 |
| valid vs isolation_invariant | 0.742 | 0.845 | 0.836 |
| valid vs intrinsic_ub | 0.581 | 0.672 | 0.741 |

## Honest reading

- **On the LUMPED task generic ≈ or > boundary-specific.** That generic edge is largely a **size confound**: the authored negatives (tiny UB functions, small accessors) are systematically smaller, so `c_nodes`/`c_stmts` predict INVALID within this corpus — a property unlikely to transfer to real code. combined < generic is small-data overfit (33 features, ~17 invalid-bearing programs).
- **Split by MECHANISM, the picture is the intended one:** for `invalid_isolation_invariant`, boundary-specific features win the grouped-CV baseline (**0.845** vs generic 0.742; single-feature `rf_struct_index_field` 0.88, `rf_unmasked_field_index` 0.79). For `invalid_intrinsic_ub`, **generic is near-random** (0.581) while combined reaches **0.741** — the signal is the guard×op INTERACTION (`rf_unguarded_ubop`) a linear model needs that engineered term to see.
- **Takeaway:** the semantic-risk features encode the real mechanism, but the lumped linear baseline is confounded by corpus size-bias and small data. Next: control the size confound (size-matched negatives / partial out size), per-mechanism or tree models, and external programs — before claiming a generalizing P(valid|x).

> This is a baseline (no tuning) on the audited `validity_v2`; AUC pooled across grouped folds; ~17 programs carry invalids, so treat magnitudes as indicative.

