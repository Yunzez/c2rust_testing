# RQ1 reporting decision — 2026-09-02 (user + advisor, via GPT discussion)

**Status: BINDING for the paper's `tab:matching-accuracy`.** Replaces the previous Group A / Group B
two-panel presentation (group A part of `rq1_assembled_v1.md`; primary table in `group_b_status.md` §2).
Those documents remain the **provenance** for their halves (artifact lists, labels, per-cell runs, PARTIAL
scores, open review items) — only the *presentation* is superseded.

Numbers below are re-derived by `scripts/rq1_merged_table.py` from `rows/group_a_table.json` and
`rows/group_b_full.json` (output: `rows/merged_table.json`). Re-run the script before writing anything into
the paper. The user updates the paper themselves; nothing here has been written into `evaluation.tex`.

---

## The decision (user's text, recorded verbatim)

**RQ1 reporting update — replace the previous Group A / Group B presentation**

1. Keep the RQ title as "RQ1: Matching Accuracy. How accurately does our matcher identify corresponding
   functions between the original C program and its Rust translation?" "Accuracy" is appropriate. Do not
   claim the matcher is novel, general-purpose, or state of the art; it is an enabling component whose job is
   to provide sufficiently accurate correspondences for downstream differential validation.

2. Merge Group A and Group B into one main table. They use the same matcher and differ only in how ground
   truth is obtained: for name-preserving artifacts, identifiers are hidden from the matcher and name equality
   is the known ground truth; for renaming artifacts, correspondence ground truth is established manually.
   This is explained in the caption / setup text, not shown as two panels.

3. Table structure: Library | Tools | Pairs (renamed) | Matcher P / R | Renamed correct. Use the existing tool
   symbols. A superscript p may identify PARTIAL outputs, but PARTIAL outputs are excluded from row scores
   and from Overall.

4. Current combined result under the library-level macro rule: 10 libraries, 40 complete translator outputs,
   4,202 correspondence pairs, 9 genuine renamed pairs, matcher macro precision 0.829, macro recall 0.874,
   renamed correct 7/9. Re-derive from `group_a_table.json` and `group_b_full.json` before writing into the
   paper.

5. Aggregation: average complete tool outputs within each library, then average the ten libraries with equal
   weight. Pair counts are evidence volume only. No micro-average. No dev / eval / held-out split. cJSON and
   lil remain in the main ten-library result. No deployment / abstention / P@C / coverage columns. Keep the
   strict implemented-function rule; no STUBs in the primary correspondence count.

6. Prose: name equality recovers 0/9 genuinely renamed pairs; the matcher recovers 7/9. Do not present 7/9 as
   a statistically stable population estimate; it is the observed result over all genuine renames in the
   available complete artifacts.

7. Interpretation: "The matcher is not a standalone research contribution. It is evaluated to establish that
   the alignment component is adequate for constructing the function boundaries used by the validator. It
   supports 40 complete artifacts across all ten libraries and recovers seven of the nine naturally renamed
   implemented functions." Do not claim: state-of-the-art function matching; a novel matching algorithm;
   complete recovery of translation maps.

8. Paper novelty remains separate from RQ1: the differential-validation methodology that makes structurally
   different C and Rust executions comparable at function boundaries; normalization and observation of the
   relevant cross-language program state; attribution that distinguishes translation defects from C-side UB
   and reference-version problems; and the systematic evaluation across six translators and ten real
   libraries, including the confirmed defect corpus and taxonomy.

---

## Merged table (re-derived 2026-09-02, matches item 4 exactly)

Forced configuration; strict implemented-function rule; PARTIAL outputs (ᵖ) listed but excluded from row
scores and Overall. Symbols: ∘ c2rust △ Laertes ◇ CROWN • C2SaferRust ★ PtrTrans × SACTOR.

| Library | Tools | Pairs (renamed) | Matcher P / R | Renamed correct |
|---|---|---:|---|---:|
| qsort | ∘ • △ ★ × | 15 (2) | 0.867 / 0.867 | 1/2 |
| urlparser | ∘ • ◇ △ ★ ×ᵖ | 102 (0) | 0.912 / 0.930 | — |
| quadtree | ∘ ◇ ★ ×ᵖ | 65 (0) | 0.961 / 0.961 | — |
| genann | ∘ • ◇ △ ★ × | 74 (0) | 0.905 / 0.924 | — |
| cJSON | ∘ ★ | 135 (1) | 0.699 / 0.779 | 1/1 |
| lil | ∘ • ◇ △ ★ᵖ | 563 (0) | 0.961 / 0.961 | — |
| lodepng | ∘ ◇ ★ ×ᵖ | 478 (4) | 0.668 / 0.869 | 3/4 |
| bzip2 | ∘ • ◇ △ ★ ×ᵖ | 265 (2) | 0.823 / 0.952 | 2/2 |
| tulip | ∘ • ◇ △ ×ᵖ | 852 (0) | 0.554 / 0.554 | — |
| optipng | ∘ • △ | 1653 (0) | 0.941 / 0.943 | — |
| **Overall (10 libraries, 40 complete outputs)** | | **4,202 (9)** | **0.829 / 0.874** | **7/9** |

Name equality recovers 0/9 genuinely renamed pairs (by construction — the pairs are renamed); the matcher
recovers 7/9. The two misses are: qsort × SACTOR (1 of 2 renamed pairs) and lodepng × PtrTrans (1 of 4);
details in `group_b_status.md` §2/§5.

### Derivation (what the script does)

- **Group A artifacts** (31, name-preserving; truth = hidden-name equality): from
  `rows/group_a_table.json → per_library[lib].per_artifact_counts[key]`, per-artifact
  P = `f_correct / f_matched`, R = `f_correct / scorable`, pairs = `scorable`, renamed = 0.
- **Group B COMPLETE artifacts** (9; truth = manual labels, `reviewed_by_user: false`): from
  `rows/group_b_full.json → rows[case]` with `artifact_status == "COMPLETE"`, P/R = `forced.precision/recall`,
  pairs = `scorable`, renamed = `real_renamed.pairs`, correct = `real_renamed.forced_correct`.
- **Group B PARTIAL artifacts** (6: lil ★, urlparser ×, quadtree ×, tulip ×, lodepng ×, bzip2 ×): listed with ᵖ,
  excluded from every number. Their separate scores stay in `group_b_status.md` §3.
- **Row** = unweighted mean of the library's complete outputs; **Overall** = unweighted mean of the ten rows.
  Pair and renamed counts are sums (evidence volume only).
- Group A per-library numbers and the 0.938/0.939 group-A-only aggregate are untouched; the merged aggregate
  is lower because the 9 renaming-translator outputs (0.516/0.739 on their own) now enter their library means.

### Consistency with earlier binding rules (`group_b_status.md` §0)

Everything there still holds (forced only; unit = library; no micro / dev-eval / deployment; STUB ≠ genuine;
labels preliminary; PARTIAL separate; pre-output failures = "failed to emit analyzable Rust under the shipped
configuration"). What changes: **(a)** group A and group B no longer appear as separate tables or aggregates
in the paper; **(b)** the `name-eq R` column is dropped from the table and becomes one sentence of prose
(0/9 vs 7/9); **(c)** N/A rows disappear — every library now has ≥ 2 complete outputs, so the table has no
N/A cells (lil/tulip/optipng were N/A only in the group-B-only view). The group-B-only primary table
(7 libraries, 9 outputs, 161 pairs, 0.516/0.739) is **no longer a paper number**; keep it in
`group_b_status.md` as the per-group provenance breakdown.

### Still owed by the user before the paper is final

- Independent review of the 9 renamed pairs and the 7 STUB-vs-genuine calls in `group_b_status.md` §5
  (all Claude labels are `reviewed_by_user: false`).
- Copy the table above into `evaluation.tex` (I do not edit the paper).
