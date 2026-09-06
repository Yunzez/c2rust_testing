# RQ4 — coverage beyond shipped tests: bzip2

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md).
Status 2026-09-05: **four cells complete under the plan pipeline** (c2rust, Laertes, CROWN, C2SaferRust),
3 600 s each, serial, one campaign and one corpus per cell (PROTOCOL §4). SACTOR (`✗(parse)`) and PtrTrans
(`✗(compile)`) produce no runnable artifact and stay N/A with their E1 evidence. The earlier hand-schema
cell is kept under [`c2rust_handschema_superseded/`](c2rust_handschema_superseded/) and is not a comparison point.

## Cell table (per-tool `RUN.md` carries the procedure, deviations and limits)

| tool | tests side (suite through the translation) | planned / built / exported of 64 | corpus | fn tests | fn ours | reg tests | reg ours | only-ours reg | divergences on replay | confirmed |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---|
| **c2rust** | **PASS 6/6** — baseline | 19 / 19 / 16 | 1 338 | 51/66 (0.773) | 46/66 (0.697) | 7 007/8 789 (0.797) | 7 090 (**0.807**) | **481** (only-tests 398) | **0** / 1 338 | **0** of 1 046 sampled |
| Laertes | TEST-FAILS 0/6 | 19 / 19 / 16 | 974 | — | 45/82 (0.549) | — | 6 206/10 065 (0.617) | — | **299** / 974 | **532 / 532** on the public boundaries: 299 divergence (S3, two boundaries) + 233 termination (**C8**, `incs`) |
| CROWN | TEST-ADAPTER-FAILS | 19 / 19 / 16 | 1 003 | — | 51/74 (0.689) | — | 5 414/9 084 (0.596) | — | **242** / 1 003 | Decompress 113/113 (S11); Compress 97 divergence (S10) + 4 termination; `fallbackSort` 32/32 `bhtab` — one root cause (`SET_BH` `\|=` → `=`) |
| C2SaferRust | TEST-FAILS 0/6 | 17 / 17 / 15 | 302 | — | 15/69 (0.217) | — | 1 158/8 227 (0.141) | — | **3** / 302 | `mmed3` 3/3 (S14) |

`—` = the suite is not a baseline (PROTOCOL §2): universe from a link-dead-code build, partition Ours / Neither.
Raw region counts are per-translation identities and are **not comparable across tools**; compare the
fractions and the candidate counts. All four cells pass the four sanity checks (`cells.json`).

## What this library says

1. **Only one of four translations passes the library's own acceptance suite.** Laertes, CROWN and
   C2SaferRust fail it outright (0/6, adapter does not compile, 0/6). That is a result about the
   translations, reported before any fuzzing number.
2. **The negative control holds at every layer.** On c2rust the same generator, budget and corpus give
   0 replay divergences, 0 confirmed artifacts, and 0 artifacts at all on the two public boundaries.
3. **Every catalogued bzip2 defect the pipeline could reach, it re-found with no hand work**: S3 (Laertes,
   on both its boundaries), S10 and S11 (CROWN), S14 (C2SaferRust). S12 sits on a public boundary the
   generator cannot bridge for C2SaferRust's reshaped API and is not re-confirmed.
4. **One new defect, C8** (Laertes, zeroed shell-sort increment table): predicted by the severed-init
   scanner, realised by a fuzzer-found input, corroborated by the suite's empty compress outputs, and
   confirmed 506/506 with the C side clean under ASan + full UBSan. See `laertes/RUN.md` §7.
5. **A line-level root cause for CROWN's compressor** (C7 / S10): the `SET_BH` bit-set became a plain
   store when CROWN removed c2rust's reborrow idiom. Found through an internal boundary (`fallbackSort`,
   32/32 value divergences with C in contract), which the public boundary alone had only shown as
   layout-dependent crashes.
6. **The budget amendment was necessary for one cell**: 300 → 3 600 s adds +1.2 % (c2rust) and +1.0 %
   (Laertes) of regions, but **+8.6 % on CROWN**, whose decompressor was far from saturated at five
   minutes; function coverage is flat from 300 s in all three.

## Gaps and limits

- The 46 000–60 000 termination artifacts per tool on the internal sort routines are the input model's
  out-of-contract inputs, identical in shape on all four tools; adjudicated at a 200-per-channel sample
  and reported with their totals (PROTOCOL §4, adjudication depth).
- CROWN's Compress and `fallbackSort` wild-address ASan reports (2 645 + 523 + 5 109) are never claimed
  on their own (layout dependence); the claimable evidence is the `bhtab` value divergence.
- Single campaigns: run-to-run variance was measured once at ≈5 % on one boundary; no repeat campaigns.
- `BZ2_bz__AssertH__fail` (exits) and `mainSort` (out of contract at the first input) export no coverage
  on any tool.
- C2SaferRust's coverage measures the generator's reach on a reshaped API, not the translation.

## Files

`tests_side_results.json` (suite outcomes recomputed from preserved outputs), `cells.json` (the table's
source), per tool: `RUN.md`, `funnel.json`, `plans.json`, `analysis/` (+ `analysis@300s`, `@1800s`),
`snapshots.json`, `divergences/` (inputs + outcomes), `confirm*/` (verdicts gzipped, clusters),
`harnesses/<b>/` (generated fuzz target, build.rs, coverage log), `candidates_sample/`,
`candidates_manifest.json.gz`, `corpus.tar.gz`, `harness_exports.tar.gz`, `artifact_hashes.json`,
`fuzz_logs/` (head + tail). Pairs: `benchmark/pairs/rq4/bzip2_<tool>/`.
