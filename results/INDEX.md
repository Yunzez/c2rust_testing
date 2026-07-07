# Results Index — single entry point (2026-07-04)

**North star: [PROJECT_RESET_2026-07-03.md](PROJECT_RESET_2026-07-03.md)** — semantic-diff bugs
(C≠Rust, both terminate, UB-free inputs) are the one class fuzz-Rust-alone cannot find; novelty =
matcher-enabled differential testing for structure-non-preserving translators. Deliverable =
**certify-or-find**, not bug-count. All paper findings C-backed + ASan/UBSan-gated.

## ★ Headline findings (the paper's core evidence)

| finding | tool | numbers | archive |
|---|---|---|---|
| **crc32 empty-chunk reset** (`is_null`→`is_empty`) | C2SaferRust, optipng | 1M-trial differential; all diffs len==0; end-to-end PNG corruption | [rq1_bugs/crc32_c2saferrust/](rq1_bugs/crc32_c2saferrust/) |
| **cJSON `parse_string` cluster**: `\u` escapes dead (empty-slice `input_end` at CALL SITE), valuestring=None data loss, non-UTF-8 reject | **PtrTrans FSE'26 + gpt-5.1** | 40,133 / 120,050 UB-free divergences; callee faithful standalone → unit tests can't find it | [rq1_bugs/cjson_ptrtrans/](rq1_bugs/cjson_ptrtrans/) |

Same root-cause family across two independent published tools: **ptr→slice lift silently changes
boundary semantics**. Full bug list: [rq1_bug_table.md](rq1_bug_table.md),
[rq1_semantic_diffs.md](rq1_semantic_diffs.md).

## Equivalence certificates (the "certify" half)

- **bignum × gpt-5-mini**: 27 fns × 500k inputs, 0 diffs → [llm_fidelity_v1.md](llm_fidelity_v1.md)
  (contemporary LLMs are faithful-or-fail, NOT silently wrong; kills the naive bug-count pitch)
- **genann × all 4 lifters**: `genann_run` C≡c2rust≡CROWN 300k/0 bit-exact →
  [rq1_genann_matrix.md](rq1_genann_matrix.md)
- **cJSON × c2rust**: round-trip 100k/0 (mechanical baseline faithful) →
  [rq1_cjson_matrix.md](rq1_cjson_matrix.md)

## Tool matrices & rounds

- [rq1_cjson_matrix.md](rq1_cjson_matrix.md) — cJSON vs ALL tools: SACTOR fail / CROWN crash /
  C2SaferRust blocked / **PtrTrans completes-but-buggy (24/118 stubs + silent cluster)** / c2rust faithful
- [rq1_genann_matrix.md](rq1_genann_matrix.md) — genann multi-lifter, all clean
- [rq1_sactor_round.md](rq1_sactor_round.md), [rq1_c2saferrust_round.md](rq1_c2saferrust_round.md),
  [rq1_crown_recon.md](rq1_crown_recon.md) — per-tool campaign notes
- [rq1_autosweep.md](rq1_autosweep.md) — base-vs-WIP Rust sweep (**INTERNAL scouting only, not a paper method**)

## Method evals

- **E1b mutation recall**: 27/27 = 100%, three-layer denominator → [mutation_recall_v1.md](mutation_recall_v1.md)
- **UB gate**: [rq2_ubgate_v1.md](rq2_ubgate_v1.md) — divergence counts as bug only on UB-free inputs
- **E2 matcher**: [rq3_matcher_v1.md](rq3_matcher_v1.md) — name-independent matching (bignum 92%,
  lil 61% → call-graph topology propagation planned)
- **E3 coverage**: [rq1_coverage_scope_plan.md](rq1_coverage_scope_plan.md) — plan

## Related work

- [related_work_landscape.md](related_work_landscape.md) — RustAssure (ASE'25) primary competitor;
  niche (UB-gate + matcher + diff-fuzzing) unoccupied

## Archive

Pre-RESET / superseded / one-off docs live in [archive/](archive/) (frontier-selection era, old RQ2/RQ3
plans, corpus inventories, CRUST-bench investigations, one-off demos). History in git.
