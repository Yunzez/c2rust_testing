# Results Index — single entry point (reorganised 2026-09-01)

Authoritative evaluation design: **[`EVALUATION_PLAN.md`](EVALUATION_PLAN.md)**.
The paper should mirror that file. Historical directory names are retained so
that evidence links remain stable; they no longer determine RQ numbering.

| RQ | question | directory | evidence state |
|---|---|---|---|
| **RQ1** | Matching accuracy — does the matcher find the true C↔Rust correspondences? | [`rq1_matching/`](rq1_matching/) | **measured, awaiting user label review** — one merged ten-library table (decision 2026-09-02): 40 complete translator outputs (31 name-preserving + 9 renaming), 4,202 pairs, 9 genuine renamed pairs; library-level macro P/R **0.829 / 0.874**, renamed correct **7/9** (name equality 0/9); 6 PARTIAL outputs listed with ᵖ, excluded. All 20 PtrTrans/SACTOR cells attempted. **Preliminary: group-B labels not yet user-reviewed.** |
| **RQ2** | Effectiveness — how many real translation defects does the complete validator find? | [`rq4_effectiveness/`](rq4_effectiveness/) *(legacy name)* | **21 currently catalogued across 8/10 libraries and 5 rewriting systems; final count awaits exact-source confirmation audit** (S14 added 2026-09-03) |
| **RQ3** | Taxonomy — what recurring mechanisms cause the confirmed defects? | [`rq4_effectiveness/defect_manifest.md`](rq4_effectiveness/defect_manifest.md) | **seven primary families covering the same 21 defects** (`semantic computation substitution` added 2026-09-03 for S14) |
| **RQ4** | Coverage beyond shipped tests — validator vs. the suites that accepted the translations | [`rq3_coverage/`](rq3_coverage/) *(legacy name)* | **none executed** — no paired run exists |

Supporting ablations (explicitly **not** RQs): [`ablations/`](ablations/).
Superseded plans and per-round notes: [`archive/`](archive/).

---

## RQ1 — Matching Accuracy → [`rq1_matching/`](rq1_matching/)

| artifact | contents |
|---|---|
| [rq1_reporting_decision_2026-09-02.md](rq1_matching/rq1_reporting_decision_2026-09-02.md) | **READ FIRST — binding presentation for `tab:matching-accuracy` (user + advisor, 2026-09-02).** Group A and group B merged into one table (same matcher; truth = hidden-name equality vs manual labels, explained in caption); columns Library / Tools / Pairs (renamed) / Matcher P/R / Renamed correct; 10 libraries, 40 complete outputs, 4,202 pairs, 9 renamed, macro 0.829/0.874, 7/9; PARTIAL outputs ᵖ excluded; interpretation and non-claims recorded verbatim. Numbers re-derived by `scripts/rq1_merged_table.py` → [rows/merged_table.json](rq1_matching/rows/merged_table.json). The group-A-only (0.938/0.939) and group-B-only (0.516/0.739) aggregates below are provenance, no longer paper numbers. |
| [group_b_status.md](rq1_matching/group_b_status.md) | **Group B provenance (canonical for group B, 2026-09-02; its §2 primary table superseded by the merged table above).** Binding reporting rules (all 10 libraries in one primary table, library = unit, no dev/eval / micro-average / deployment columns), the 10×2 PtrTrans/SACTOR availability matrix (AVAILABLE / PARTIAL / PRE-OUTPUT FAILURE) after the 2026-09-02 paid runs, the primary per-library group-B table from `rows/group_b_full.json`, PARTIAL cells scored on their own lines, per-run provenance + cost, and the list of labels needing independent review. Raw outputs and `RUN.md` per paid run in [raw/group_b_runs/](rq1_matching/raw/group_b_runs/). |
| [rq1_assembled_v1.md](rq1_matching/rq1_assembled_v1.md) | **Group A authority (v5, 2026-09-01); group-B / dev-eval / micro / deployment parts superseded by `group_b_status.md`.** `tab:matching-accuracy` in the paper's row order, group A for all ten libraries: eval set 26 artifacts / 8 libs / 3420 pairs, micro P/R 0.857/0.858 (macro 0.914), deploy precision 0.997 at coverage 0.614 — tulip (852 pairs, forced 0.554, 103 distinct static fingerprints for 213 fns) is the collapse case and is reported, not excluded; optipng 1653 pairs 0.941/0.943, 0.999@0.742. Table 1 now under the paper's aggregation rule (mean over tools per library, then over libraries; A overall 0.938/0.939, 0.993@0.784). Group B labeled + scored (strict: STUB rows are not pairs; lenient line alongside): 6/10 libraries, 7 outputs, 132 pairs (9 renamed); bzip2/lodepng PtrTrans crates are stub shells (52/64, 214/235); sactor_qsort deployment false accept `partition→quick_sort` confirmed. Forced vs deployment always separate. |
| [SPLIT.md](rq1_matching/SPLIT.md) | frozen library-disjoint dev/eval split: dev = cJSON, lil, `benchmark/pairs`; eval = the other 8 libraries. |
| [rows/group_a_full.json](rq1_matching/rows/group_a_full.json) | per-artifact P/R/abstention/coverage for the 31 group-A rows (24 + tulip 4 + optipng 3), with C-provenance, pruned-module records and a per-row fingerprint; produced by `scripts/rq1_name_preserving_full.py` (**frozen 2026-09-01**). Supersedes `cells/name_preserving_v1.json` (recall-only, July matcher). |
| [topology_resolution.md](rq1_matching/topology_resolution.md) | unique local resolved edges + unresolved-local-call rate per Rust artifact; shows non-building PtrTrans crates (bzip2, lodepng) at 1–2 % of the call-graph density of building same-library translations. |
| [rows/group_a_table.json](rq1_matching/rows/group_a_table.json) | per-library pooling of `group_a_full.json` in the paper's row order (Pairs / forced P,R / deploy P@C) plus eval/dev/all micro+macro aggregates; produced by `scripts/rq1_group_a_table.py`, which re-derives every integer from `raw/group_a/` and aborts on any disagreement with the rows. **The only sanctioned path from rows to `tab:matching-accuracy` group-A cells.** |
| [rows/topology.json](rq1_matching/rows/topology.json) + [raw/topology/](rq1_matching/raw/topology/) | machine-readable version of `topology_resolution.md` (per-artifact resolved-edge / unresolved-call counts) with tool-version block and per-artifact raw analyzer output; produced by `scripts/rq1_topology_resolution.py`. |
| [raw/group_a/](rq1_matching/raw/group_a/) | per-artifact archive for all 31 group-A cells: `c_analyzer.json`, `rust_analyzer.json`, `truth.json` (name-equality truth + ambiguous duplicates), `matcher_output.json` (forced + deployment pair lists, eps). Every `group_a_full.json` row carries a `fingerprint` (matcher/analyzer commits and hashes, C source + header hashes, Rust artifact hash, params). |
| [group_b_availability.md](rq1_matching/group_b_availability.md) | **SUPERSEDED by `group_b_status.md`** — 2026-09-01 library × {PtrTrans, SACTOR} inventory of *analyzable* renaming-translator output: 7 scaffolded artifacts (PtrTrans qsort, quadtree, bzip2, lodepng, cJSON; SACTOR qsort, genann); every other cell N/A with sub-class **never produced** vs **produced but not retained** and the evidence pointer. Defines real-renamed vs same-name pairs and the `STUB:x` label; Status section records that all 7 are labeled (unreviewed) and that bzip2/lodepng are stub shells. |
| [annotation/](rq1_matching/annotation/) | manual-labeling scaffolds for the group-B artifacts (15 cases after 2026-09-02: the original 7 plus ptrtrans_urlparser/genann/lil and sactor_urlparser/quadtree/tulip/lodepng/bzip2, all labeled, none user-reviewed); originally the 7 (buildable: PtrTrans qsort/quadtree/cJSON, SACTOR qsort/genann; non-buildable: PtrTrans bzip2/lodepng), one `sheet.csv` each with tool claim / name-eq / matcher priors and blank `truth`; `sheet.json` carries the fingerprint and raw matcher lists; duplicate Rust leaf names are flagged, never silently resolved. Produced by `scripts/rq1_group_b_scaffold.py`; raw analyzer/matcher output in [raw/group_b/](rq1_matching/raw/group_b/). **All 15 labeled** (`labels.json` per case, evidence note per row, `reviewed_by_user: false`), applied by `scripts/rq1_group_b_label.py`. |
| [rows/group_b_full.json](rq1_matching/rows/group_b_full.json) | per-artifact group-B scores (forced P/R, deployment P@C/abstention, name-eq baseline, real-renamed vs same-name subsets, tool-map claim precision on labeled pairs, lenient-with-stubs variant) plus the paper-rule per-library means and the macro-average with its explicit denominator (`overall` = 7/10 libraries / 9 outputs after 2026-09-02; `artifact_status: PARTIAL` cases are listed separately and excluded from it; the old `overall_eval_only` block is superseded); produced by `scripts/rq1_group_b_score.py`, which cross-checks the sheet's matcher columns against the raw matcher lists. **The only sanctioned path to `tab:matching-accuracy` group-B cells; warns while labels are unreviewed.** |
| [matcher_master_table.md](rq1_matching/matcher_master_table.md) | older library × tool matcher table (recall-only). Its numbers are superseded by `rows/group_a_full.json` where they overlap; still the source of the corpus-hygiene rule and the "enabler" retraction. |
| [matcher_ablation_v1.md](rq1_matching/matcher_ablation_v1.md) + [rows/](rq1_matching/rows/) | four-regime ablation on the LLM-transpiler micro corpus (**development data**). Headline: **precision .969 at coverage .73** under abstention; name-eq recall collapses to 0.0 under maximal rename. |
| [cells/](rq1_matching/cells/) | per-cell matcher data: `name_preserving_v1.json` (superseded), `rawllm/` (7 hand-labelled libraries, stress test), `ptrtrans_rename/`, `regression/` (SIGNAL_C gate) |
| [cells/mapping_audit/](rq1_matching/cells/mapping_audit/) | the July mechanical tool-map audit — PtrTrans's shipped lodepng map disagreed on 143/255 entries. The 2026-09-01 scaffold shows 63/226 claims are shifted 1–3 records against C source order and 21 are placeholders/libc names; **to be redone against manual truth** (see `rq1_assembled_v1.md`). |
| [align_qsort_ptrtrans/](rq1_matching/align_qsort_ptrtrans/) | the buildable qsort×PtrTrans cell behind the paper's `\AlignmentTable`: name-eq 2/3 correspondences and 0/1 defective contract boundaries, vs 3/3 and 1/1 for tool map / matcher / manual; 0 false divergences. **Single-seed (42) pilot under [PILOT_PROTOCOL.md](ablations/PILOT_PROTOCOL.md) — not a variance claim**, and it is the only RQ1 result `evaluation.tex` currently reports. |

**Gap:** group-B labels are Claude-produced and not yet user-reviewed (1235 C-function rows over 15 cases, evidence
note per row); numbers are preliminary until `reviewed_by_user` flips. Primary group-B table covers 7 of
10 libraries (9 tool outputs); lil, tulip and optipng have only PARTIAL or no output — see
`group_b_status.md` §1/§6 for the stage-specific reason per cell (all 20 cells now attempted; the
2026-09-02 Gate 0/1 protocol closed bzip2 × as PARTIAL and optipng × as pre-output failure at $0).
Group-A tulip/optipng values and all group-B values are measured but not yet in the paper.

## Retired RQ2 — Boundary Validity → [`rq2_boundary/`](rq2_boundary/)

| artifact | contents |
|---|---|
| [protocol.md](rq2_boundary/protocol.md) | **SUPERSEDED, never executed.** Boundary validity is no longer a research question. |

No work from this draft is required by the current evaluation plan.

Nearest surrogates elsewhere, none of which is an RQ2 census: `rq4_effectiveness/defect_manifest.md`'s
per-defect contract-boundary column (defect-selected, not census-selected), the single
`rq1_matching/align_qsort_ptrtrans/` boundary, and `ablations/attribution/ubgate_v1.md`'s 48 boundaries
(micro-benchmarks, and it measures UB attribution rather than comparability).

## RQ4 — Coverage Beyond Shipped Tests → [`rq3_coverage/`](rq3_coverage/)

**Gap:** everything. Nothing in this repository runs a library's shipped test suite against a translated
crate and compares it with a validator campaign under a common coverage mechanism and budget.
`evaluation.tex:164` is the matching TODO.

⚠️ The 33-cell execution-depth census is **not** RQ4 evidence — it is a
one-sided reach measurement whose "their tests" side is 0 by construction. It now lives at
[`rq4_effectiveness/reach_census.md`](rq4_effectiveness/reach_census.md), which is how `evaluation.tex`
previously used it. At most it can qualify the interpretation of RQ2; it cannot
replace the paired RQ4 experiment.

## RQ2 and RQ3 — Effectiveness and Taxonomy → [`rq4_effectiveness/`](rq4_effectiveness/)

| artifact | contents |
|---|---|
| [defect_manifest.md](rq4_effectiveness/defect_manifest.md) / [.json](rq4_effectiveness/defect_manifest.json) | **canonical unit table**: 21 confirmed defects (C1–C7 crash, S1–S14 semantic) + 4 held-out candidates, 7 mechanism families, per-channel recovery booleans, provenance. Generated by [gen_defect_manifest.py](rq4_effectiveness/gen_defect_manifest.py) (`--build` regenerates .json then .md from the embedded tables). |
| [translation_matrix.md](rq4_effectiveness/translation_matrix.md) | the paper's `\TranslationMatrixTable`: 10 libraries × 6 systems, **60/60 cells measured** — defects / bounded no-difference results / process failures |
| [bugs_detailed.md](rq4_effectiveness/bugs_detailed.md) + [bugs/](rq4_effectiveness/bugs/) | bug catalogue with verbatim C+Rust, and 23 per-cell evidence archives |
| [bug_table.md](rq4_effectiveness/bug_table.md), [semantic_diffs.md](rq4_effectiveness/semantic_diffs.md) | the evidence rows the manifest cites for C5 and S12 |
| [reach_census.md](rq4_effectiveness/reach_census.md) + [reach_cells/](rq4_effectiveness/reach_cells/) | per-function execution-depth census, 33/33 cells. **Supporting only**: 9 cells have median 0, and budgets span 4k–2M runs (500×), so cross-library magnitudes are not comparable. Only tulip supports a clean tool-vs-tool reading (fixed N=30,000 across all four tools); genann is close but not uniform — 10⁶ runs for four tools and 2×10⁶ for its c2rust cell. |
| [severed_init_law.md](rq4_effectiveness/severed_init_law.md) / [.json](rq4_effectiveness/severed_init_scan.json) | Laertes severed-initialiser census: 277 `laertes_init_*` defined, 0 ever called, all 10 crates. *(Pending: retitle — it is a census, not a law, and never a predictor.)* |
| [certificates/](rq4_effectiveness/certificates/) | bounded no-difference evidence: `llm_fidelity_v1.md` (note: bignum is compile-only, **not** a certificate), `genann_matrix.md`, `cjson_matrix.md` |

## Ablations (supporting evidence, not RQs) → [`ablations/`](ablations/)

| artifact | contents |
|---|---|
| [PILOT_PROTOCOL.md](ablations/PILOT_PROTOCOL.md) | the binding pilot rules: one fixed seed (42), RESULT.md + result.json per cell, explicitly **not** a variance claim. Also governs the RQ1 align pilot. |
| [observation/](ablations/observation/) | 6 OBS cells (`obs_matrix/` ×5 + `obs_qsort_ptrtrans/`) behind `\ObservationAblationTable`. Covers **9 of the 21 defects**. Channels O-R / O-P(silent) / O-P(print) / O-S / O-F. |
| [attribution/](ablations/attribution/) | `urlparser/` (heap-overflow witness, needs isolated ASan+UBSan) and `lil/` (37 of 313 records excluded by both gates). Covers **0 of the 21 defects** — both are UB-exclusion cells with no confirmed translation divergence. Behind `\AttributionWitnessTable`. |
| [attribution/ubgate_v1.md](ablations/attribution/ubgate_v1.md) + [ubgate_rows/](ablations/attribution/ubgate_rows/) | UB-gate study: 48 faithful c2rust boundaries, 13 gate-off divergences all attributed, **0 false bug reports**. Micro-benchmark programs, not the 10 libraries. |
| [attribution/mutation_recall_v1.md](ablations/attribution/mutation_recall_v1.md) + [mut_rows/](ablations/attribution/mut_rows/) | negative control: **27/27** injected UB-free semantic bugs retained, three-layer denominator (28 injected / 1 equivalent / 27 valid) |

**Standing caveat:** every OBS/ATTR number here is single-seed (42), one run per cell. Record counts are
per-cell and non-summable — C6+S13 share the 201-record tulip cell; S7+S8+S9 share the 139-record cJSON cell.

## Other

- [related_work_landscape.md](related_work_landscape.md) — competitor map; deferred citation additions tracked separately.
- [archive/](archive/) — superseded plans (`FSE_PLAN.md`, `PROJECT_RESET_2026-07-03.md`, the old RQ-numbered
  eval plans) and per-tool round notes. Historical only; do not restore their structure.

---

## Rename map (2026-09-01)

Before this commit, the `rq1_`/`rq2_`/`rq3_` prefixes encoded the **old E1/E2/E3 numbering** and each
mapped to a different current RQ — `rq2_` named two unrelated studies and so did `rq3_`. All references
across `results/`, `scripts/`, `tools/`, and `fuzz/` were rewritten to repo-root-relative paths.

| old | new | now feeds |
|---|---|---|
| `rq1_master_table.md` | `rq4_effectiveness/translation_matrix.md` | current RQ2 (legacy directory name) |
| `rq1_bugs/`, `rq1_bugs_detailed.md`, `rq1_bug_table.md`, `rq1_semantic_diffs.md` | `rq4_effectiveness/bugs/`, `bugs_detailed.md`, `bug_table.md`, `semantic_diffs.md` | current RQ2 and RQ3 |
| `rq1_cjson_matrix.md`, `rq1_genann_matrix.md`, `llm_fidelity_v1.md` | `rq4_effectiveness/certificates/` | current RQ2 context |
| `defect_manifest.*`, `gen_defect_manifest.py`, `severed_init_*` | `rq4_effectiveness/` | current RQ2 and RQ3 |
| `rq3_master_table.md`, `rq3_cells/` | `rq4_effectiveness/reach_census.md`, `reach_cells/` | supporting only; not current RQ4 evidence |
| `rq2_master_table.md`, `rq2_cells/` | `rq1_matching/matcher_master_table.md`, `cells/` | RQ1 |
| `rq3_matcher_v1.md`, `rq3_rows/` | `rq1_matching/matcher_ablation_v1.md`, `rows/` | RQ1 |
| `pilots/align_qsort_ptrtrans/` | `rq1_matching/align_qsort_ptrtrans/` | RQ1 |
| `rq2_boundary_census_protocol.md` | `rq2_boundary/protocol.md` | retired, never executed |
| `rq2_ubgate_v1.md`, `rq2_rows/` | `ablations/attribution/ubgate_v1.md`, `ubgate_rows/` | ATTR ablation |
| `mutation_recall_v1.md`, `mut_rows/` | `ablations/attribution/` | ATTR ablation |
| `pilots/attr/{urlparser,lil}/` | `ablations/attribution/{urlparser,lil}/` | ATTR ablation |
| `pilots/obs_matrix/`, `pilots/obs_qsort_ptrtrans/` | `ablations/observation/` | OBS ablation |
| `pilots/PROTOCOL.md` | `ablations/PILOT_PROTOCOL.md` | both ablations + RQ1 pilot |
| `FSE_PLAN.md`, `PROJECT_RESET_2026-07-03.md` | `archive/` | historical |

Two errors in the previous index are corrected here: the matcher rows are in `rq1_matching/rows/`
(formerly `rq3_rows/`), **not** in the former `rq2_rows/`, which held UB-gate data; and the matcher table
is filled to **32 of 48 attemptable cells**, not 19.

Documents keep their original prose where it records experimental provenance;
legacy RQ labels do not override `EVALUATION_PLAN.md`. One known label question
remains: `rq4_effectiveness/severed_init_law.md` still calls itself “a predictive
defect class,” although the project established that it is a census and never a
predictor.
