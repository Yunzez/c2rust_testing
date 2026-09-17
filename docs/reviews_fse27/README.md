# FSE 2027 mock review panel (2026-09-14)

Five Opus agents each profiled one member of the published FSE 2027 Research Papers PC
(https://conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee),
read that member's three most recent papers, then reviewed `c2rust_paper/` content-only
(citations, formatting and wording deliberately ignored) in that member's standards.

| Lens | PC member | Profile | Review | Decision |
|---|---|---|---|---|
| Fuzzing / oracles | Manuel Rigger (NUS) | pc_rigger.md | review_rigger.md | Weak Reject |
| Rust / unsafe / FFI | Joshua Sunshine (CMU) | pc_sunshine.md | review_sunshine.md | Weak Accept |
| Compiler testing / UB | Shaohua Li (CUHK) | pc_li.md | review_li.md | Weak Reject |
| LLM4SE / benchmarks | Jialun Cao (HKUST/Imperial) | pc_cao.md | review_cao.md | Weak Reject |
| Empirical methodology | Marcel Böhme (MPI-SP) | pc_boehme.md | review_boehme.md | Weak Reject |

## Weaknesses raised independently by three or more reviewers

1. **No measurement of the validator's own precision or recall.** No seeded-defect /
   mutation recall study; the number of confirmed divergences *rejected* at source-level
   triage is never stated, so 1,931 -> 36 is unaudited (Rigger, Li, Cao, Böhme).
2. **The headline 36 is not accounted for by the funnel.** Only 22 of 36 manifest in the
   sampled records; the rest rest on earlier driver-level campaigns that §4 does not
   describe (Rigger; Cao and Böhme raise the counting unit: 7 Laertes defects = one
   severed-init pattern, 10 of 36 from one cell, per-system counts unnormalised).
3. **The negative control is under-reported in the text.** The archived data shows 0 of
   8,962 adjudicated c2rust records reach "confirmed"; the paper text scopes the funnel
   to "the five restructuring systems" and never states that number (Li, Cao, Böhme,
   Sunshine). Note: the *current* Figure (confirmation_funnel.py) already carries the
   c2rust row 8,962 -> 8,527 -> 371 -> 0; the caption and §6.2 text do not say so.
4. **RQ5 comparison is one-directional on the authors' own positives**, so the baselines
   have no precision, and the adapter failures (RustAssure 22 C-fail, FLOURINE 14 A-fail)
   are not shown to be the baselines' fault (Rigger, Li).
5. **The oracle is unqualified on one side or the other.** C side: "full UBSan" is never
   enumerated (Li). Rust side: no Miri / aliasing model / opt-level, on predominantly
   unsafe Rust (Sunshine).
6. **n = 1 campaigns behind RQ4's stochastic comparisons**; no repetitions, variance,
   test or effect size (Böhme).
7. **Single-rater triage**: defects, families and cluster->defect merges have no second
   coder, no kappa, no operational rule for "one faulty rewrite" (Böhme, Cao).

## Internal inconsistencies found by the reviewers (checkable now)

- §5.2 "15 of the 25 restructuring artifacts" vs Table 6's 27 restructuring artifacts (Cao).
- Table 8 FLOURINE row 33/20/17/16 with A-fail = 14 does not decompose under the caption's
  semantics; the RustAssure row does (Sunshine).
- Table 7 M = 213 for all four Tulip artifacts vs §5.1's statement that ε = 0.01 abstains on
  21.6 % of pairs, "most" in Tulip (Sunshine).
- Table 4(a) reports macro precision 0.938; pair-weighted is ≈ 0.87 because tulip's 0.554
  carries 852 pairs (Cao).

## What every reviewer credited

All five re-derived the funnel (176,507 -> 14,470 -> 13,515 -> 3,365 -> 1,931), the
M/P/B totals (4,222 / 1,854 / 1,662) and Tables 5/6 against `results/` and found them exact.
