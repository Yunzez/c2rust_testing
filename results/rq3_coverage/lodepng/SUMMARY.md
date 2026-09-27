# RQ4 — reach of the generated campaign: lodepng

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md)
(§2 amendment 2026-09-07); plan: [`../../../docs/rq4_plan_ten_libraries.md`](../../../docs/rq4_plan_ten_libraries.md);
denominator: [`../../../docs/rq4_denominator_decision_2026-09-08.md`](../../../docs/rq4_denominator_decision_2026-09-08.md).
Status 2026-09-09: **two cells complete and verified**, 3 600 s each, one campaign and one corpus per
cell, `-max_len 4096`, seed 42, generator **0.8**. C source: `lodepng.c` (single unit, the version each
translation consumed, hashed in the pair). Only c2rust and CROWN produced a buildable artifact (E1).

**Tests side**: lodepng ships no test target → TEST-UNAVAILABLE on both; universe = the translation's own
instrumented objects.

## Cell table

| tool | planned / built of 235 | exported | corpus | fn ours | reg ours | confirmed (sample) |
|---|---:|---:|---:|---|---|---|
| **c2rust** | 64 / 54 | 47 | 879 | 54 / 236 (0.229) | 1 675 / 13 260 (0.126) | **0** (1 520 ub_associated, 317 ub_associated_termination, 7 inconclusive) |
| **CROWN** | 57 / 54 | 47 | 880 | 54 / 257 (0.210) | 1 449 / 14 332 (0.101) | **0** (1 521 / 349 / 7) |

## What this library says

1. **The funnel is decided at the signature.** 171 (c2rust) and 178 (CROWN) of 235 matched boundaries fail
   to plan for one reason class: struct-invariant parameters — `LodePNGInfo` (nested structs), `ucvector`,
   the bit reader/writer and `HuffmanTree` (owning pointer fields), and `T**` outputs the callee
   allocates. The frozen plan generator constructs none of these, and the paper reports that as a named
   limit of input construction, not as coverage.
2. **Nothing confirmed on either cell.** After adjudication every sampled candidate is C-side
   (`ub_associated*`) or inconclusive. Four boundaries are pre-accepted C-side crash-alls (a chunk pointer
   must address a real chunk header; `uivector_cleanup` frees memory the harness owns).
3. **Twelve rows were false positives of the classifier, not of the translations.** `Adam7_interlace` (5)
   and `Adam7_deinterlace` (1) on each cell had been labelled `confirmed_termination` because C alone timed
   out under ASan+UBSan while the translation returned — no reference execution, hence no finding. The
   classifier now returns `inconclusive` there and both cells were re-classified offline from the archived
   verdict rows (`confirm_sample/summary.json` carries the note). No re-run.

## Gaps, deviations and limits

- **Input-model gap, recorded for the next generator version**: `lodepng_chunk_find`/`lodepng_chunk_next`
  take a `(begin, end)` range that the planner models as two independent buffers, so the C side reads
  between unrelated allocations (layout luck → `ub_associated`). Not applied mid-chain.
- lodepng × c2rust's coverage analysis was recovered on 2026-09-09 after its scratch directory had been
  deleted: harnesses regenerated deterministically, archived exports re-aligned with an explicit
  `--path-map` (`analysis/recovery.json`); 0 functions outside the universe.
- CROWN's confirmation was recovered from the kept cell directory after the post had skipped it.

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md with §7 prose, funnel.json, plans.json, analysis/
[+ recovery.json on c2rust], divergences/, confirm_sample/, candidates_sample/, corpus.tar.gz,
harness_exports.tar.gz, artifact_hashes.json, raw/denominator.json). Pairs: `benchmark/pairs/rq4/lodepng_{c2rust,crown}/`
(+ `preflight_accept.txt`, `PROVENANCE.json`). Manifest: no new entries.
