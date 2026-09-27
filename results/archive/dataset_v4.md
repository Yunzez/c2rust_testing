# Boundary dataset v4 — external validity (real musl/base64) — 2026-06-24

v4 adds the first **external** boundaries: 8 real upstream C programs (base64 public-domain + 7 musl
MIT, transpiled clean under c2rust 0.22.1), harvested with generator v0.5. `dataset/boundaries_v4.jsonl`
supersedes v3 as the authoritative dataset; `validity_v2` is the authoritative label. Full story and
sanitizer evidence: `results/external_validity_v1.md`; provenance: `benchmark/pairs/EXTERNAL_PROVENANCE.md`.

## Distribution (134 boundaries = 127 authored + 7 external)

| validity_v2 | n | Δ vs v3 |
|---|---|---|
| valid | 73 | +4 |
| invalid_isolation_invariant | 20 | +1 |
| invalid_intrinsic_ub | 17 | +2 |
| weak_exclude | 7 | = |
| excluded_generator | 5 | = |
| excluded (build/gen return-type) | 12 | = |

**Binary: 73 valid : 37 invalid.** The 7 external boundaries split 4 valid : 1 isolation : 2 intrinsic;
2 more external boundaries (mu_memcmp, mu_memchr) are census-excluded (`const void*` not constructible).

## What v4 establishes

- **Both negative mechanisms reproduce on un-authored code, sanitizer-confirmed**: base64_encode's
  output-size precondition → ASan heap-overflow → isolation; mu_atoi (signed `n*10` overflow) and
  mu_llabs (LLONG_MIN negation) → UBSan → intrinsic-UB.
- **Three tooling gaps fixed** to run real libs (sibling headers, non-size_t lengths — committed in
  the generator; `mu_` rename for libc symbol collision — a corpus convention).
- **Two feature families generalized** (external code revealed they were overfit to authored operator
  forms): intrinsic-UB `rf_mul` + `rf_negate`; isolation `rf_unsized_output`.

## Analysis on v4 (the contribution holds and strengthens)

Program-grouped 5-fold CV, held-out AUC (external programs are their own groups → genuine transfer):

| task | generic | boundary-specific | combined |
|---|---|---|---|
| lumped | 0.690 | 0.702 | 0.735 |
| valid vs isolation | 0.767 | **0.865** | 0.795 |
| valid vs intrinsic_ub | 0.591 | **0.749** | **0.777** |

Per-mechanism advantage holds; intrinsic-UB strengthens vs v3 (0.676 → 0.749) from `rf_mul`/`rf_negate`
capturing real-world overflow. Size-confound control on v4: `size_only` ≈ 0.36 (≤ chance) — size is
still not the confound. Caveats (rf_unsized_output looseness, base64_decode label, small data) in
`external_validity_v1.md`. See `validity_baseline_v1.md`, `validity_baseline_size_control_v1.md`,
`feature_analysis_v1.md` (all re-run on v4).
