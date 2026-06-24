# Boundary dataset v3 — generator v0.5 (output-array + sliced-buffer) — 2026-06-24

v3 re-harvests the 40-program corpus with generator v0.5, which adds output-array and sliced-buffer
construction (body-usage driven). `dataset/boundaries_v3.jsonl` supersedes v2.1 as the authoritative
dataset; `validity_v2` is the authoritative label.

## Final distribution (127 boundaries)

| validity_v2 | n | Δ vs v2.1 |
|---|---|---|
| valid | 69 | +3 |
| invalid_isolation_invariant | 19 | = |
| invalid_intrinsic_ub | 15 | = |
| weak_exclude | 7 | = |
| excluded_generator | 5 | −3 |
| excluded (build/gen return-type) | 12 | = |

**Binary: 69 valid : 34 invalid** (negatives across 17 programs, both mechanisms). Label provenance:
44 verdicts reused from the v2.1 independent audit (label unchanged), 10 freshly reviewed this round.

## What v0.5 recovered (excluded_generator 8 → 5)

- **`prefix_sum_excl`** (output-array): a non-const SUBSCRIPTED out pointer (`dst[i]`) is an output
  array (cap-64), not a single out-scalar (`*result`, distinguished by subscripting). → valid.
- **`merge_runs`, `reverse_range`** (sliced-buffer): a pointer is not paired with a following
  slice-index scalar; bare usize indices are bounded AND chained monotone (`mid = lo + …`) so
  `a[lo..hi]` stays in bounds. → valid (merge_runs: 870k execs, no divergence).

Still `excluded_generator` (out of v0.5 scope): `msort_range` (slice indices used in a CALLEE, which
local body-usage misses), `graph_bfs`/`count_edges` (n×n matrix as a flat buffer), `copy_span`
(pointer-pair span), `free_matrix` (destructor — not differentially testable).

> A regression was caught and fixed mid-flight: the slice-index guard initially un-paired a lone
> `(const T* s, size_t count)` when `count` was both the length and used as `s[count-1]`
> (`stack_checksum`); restricted to the real sliced pattern (index followed by another usize).

## Analysis on v3 (the contribution holds on cleaner data)

P(valid|x) baseline, program-grouped 5-fold CV (held-out AUC):

| task | generic | boundary-specific | combined |
|---|---|---|---|
| lumped | 0.733 | 0.660 | 0.657 |
| valid vs isolation | 0.735 | **0.852** | 0.811 |
| valid vs intrinsic_ub | 0.591 | 0.676 | **0.736** |

Size-confound control (v3): `size_only` ≈ 0.43 (≤ chance) under grouped CV → size is not the
confound; generic's lumped edge is its pointer/signature features (a coarse proxy). Per-mechanism,
boundary-specific semantic-risk features win for isolation and lift intrinsic-UB from near-random
(generic 0.59) to 0.74 — stable vs v2.1 (0.845 / 0.741). See `validity_baseline_v1.md`,
`validity_baseline_size_control_v1.md`, `feature_analysis_v1.md`.
