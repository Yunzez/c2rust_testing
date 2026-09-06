# bzip2 × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 64 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 19 |
| built | 19 |
| executed (corpus > 0) | 19 |
| coverage exported | 16 |

Plan failures, by the generator's own reason:

- **17** × signature: struct-invariant param s: EState has pointer field 'strm' (needs invariant reco
- **11** × signature: unsupported pointer target for b: unsupported BZFILE
- **7** × signature: struct-invariant param strm: bz_stream has pointer field 'next_in' (needs invar
- **4** × signature: struct-invariant param s: DState has pointer field 'strm' (needs invariant reco
- **3** × signature: struct-invariant param f: FILE has pointer field '_IO_read_ptr' (needs invarian
- **2** × it flows into bzopen_or_bzdopen(), whose effect the harness cannot undo. What the boundary
- **1** × it flows into fopen(), whose effect the harness cannot undo. What the boundary consumes is

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `BZ2_bzBuffToBuffCompress` | no | 590 | 0 | normal 590 | batch |
| `BZ2_bzBuffToBuffDecompress` | no | 437 | 0 | normal 437 | batch |
| `BZ2_bz__AssertH__fail` | no | 1 | 2 | signal 1 | failed rc=1 |
| `BZ2_bzlibVersion` | no | 1 | 0 | normal 1 | batch |
| `BZ2_hbAssignCodes` | no | 45 | 0 | normal 29, ub-gated 16 | batch |
| `BZ2_hbCreateDecodeTables` | no | 52 | 26814 | normal 51, ub-gated 1 | batch |
| `BZ2_hbMakeCodeLengths` | no | 7 | 18 | normal 6, timeout 1 | per-input (6/7 completed) |
| `BZ2_indexIntoF` | no | 21 | 0 | normal 21 | batch |
| `bz_config_ok` | yes | 1 | 0 | normal 1 | batch |
| `default_bzalloc` | yes | 6 | 14 | normal 5, ub-gated 1 | failed rc=1 |
| `default_bzfree` | yes | 1 | 0 | normal 1 | batch |
| `fallbackQSort3` | yes | 19 | 4249 | normal 19 | batch |
| `fallbackSimpleSort` | yes | 21 | 3900 | normal 21 | batch |
| `fallbackSort` | yes | 57 | 0 | normal 57 | batch |
| `mainGtU` | yes | 5 | 10 | normal 4, signal 1 | per-input (4/5 completed) |
| `mainQSort3` | yes | 38 | 11109 | normal 37, signal 1 | per-input (37/38 completed) |
| `mainSimpleSort` | yes | 31 | 4079 | normal 30, signal 1 | per-input (30/31 completed) |
| `mainSort` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `mmed3` | yes | 4 | 0 | normal 4 | batch |

## 3. Tests side

Status **PASS**, 6/6 passed. 

Mode used for the partition: **measured**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 66 | 51 | 46 | 43 | 8 | 3 | 12 | 0.773 | 0.697 |
| regions | 8789 | 7007 | 7090 | 6609 | 398 | 481 | 1301 | 0.797 | 0.807 |

Sanity checks: function pass, region pass. Harnesses unioned: 17. Identities outside the universe (excluded, never added): 0 fn / 1 reg.

### Budget cross-check from the same campaign (hard-linked snapshots)

| budget | fn ours | reg ours | reg only-ours |
|---:|---:|---:|---:|
| 300 s | 46 (0.697) | 7005 (0.797) | 396 |
| 1800 s | 46 (0.697) | 7061 (0.803) | 452 |
| 3600 s | 46 (0.697) | 7090 (0.807) | 481 |

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `BZ2_bzBuffToBuffCompress` | 43 | 182 | 315 | 534 | 590 |
| `BZ2_bzBuffToBuffDecompress` | 278 | 352 | 378 | 417 | 437 |
| `BZ2_hbAssignCodes` | 44 | 45 | 45 | 45 | 45 |
| `BZ2_hbCreateDecodeTables` | 52 | 52 | 52 | 52 | 52 |
| `BZ2_hbMakeCodeLengths` | 4 | 7 | 7 | 7 | 7 |
| `BZ2_indexIntoF` | 21 | 21 | 21 | 21 | 21 |
| `default_bzalloc` | 6 | 6 | 6 | 6 | 6 |
| `fallbackQSort3` | 19 | 19 | 19 | 19 | 19 |
| `fallbackSimpleSort` | 21 | 21 | 21 | 21 | 21 |
| `fallbackSort` | 43 | 45 | 45 | 45 | 57 |
| `mainGtU` | 5 | 5 | 5 | 5 | 5 |
| `mainQSort3` | 36 | 37 | 38 | 38 | 38 |
| `mainSimpleSort` | 30 | 31 | 31 | 31 | 31 |
| `mmed3` | 4 | 4 | 4 | 4 | 4 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1314 |
| ub-gated | 18 |
| signal | 5 |
| timeout | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `BZ2_bz__AssertH__fail` | 2 / 2 | ub_associated 2 | 1 |
| `BZ2_hbCreateDecodeTables` | 200 / 26814 | ub_associated_termination 200 | 1 |
| `BZ2_hbMakeCodeLengths` | 18 / 18 | inconclusive 17, ub_associated 1 | 2 |
| `default_bzalloc` | 14 / 14 | ub_associated_termination 14 | 2 |
| `fallbackQSort3` | 200 / 4249 | instrument_only 1, ub_associated 199 | 6 |
| `fallbackSimpleSort` | 200 / 3900 | instrument_only 2, ub_associated 198 | 5 |
| `mainGtU` | 10 / 10 | ub_associated 10 | 7 |
| `mainQSort3` | 200 / 11109 | ub_associated 200 | 2 |
| `mainSimpleSort` | 200 / 4079 | instrument_only 2, ub_associated 198 | 3 |
| `mainSort` | 2 / 2 | ub_associated 2 | 1 |

Total: inconclusive 17, instrument_only 5, ub_associated 810, ub_associated_termination 214

<!-- prose -->



## 7. Procedure, deviations, and what is not established

**Procedure.** `scripts/rq4/cell.py` (plan → build → Rust-only campaign, 3 600 s, `-seed=42`, fork mode, 19
harnesses concurrent, fixed 64-byte seed + the shipped `sample{1,2,3}` inputs encoded into the harness
format) → `scripts/c2r_coverage.py --tests` against the shipped suite's export (6/6 pass, sha256-identical
to the export of the superseded hand-schema cell) → `scripts/rq4/replay_cell.py` (combined replay of the
saved corpus) → `scripts/rq4/confirm_cell.py` (sample 200 per channel; full on the two public boundaries).
Snapshots at 60/300/600/1 800 s are hard-linked corpus copies; the 300 s and 1 800 s coverage above is
re-collected from them, not from a second campaign.

**Deviations recorded.** (1) The protocol budget was amended 300 → 3 600 s on 2026-09-04 before this
cell ran (PROTOCOL.md §3). (2) `hbMakeCodeLengths`'s batch coverage replay hung on a looping input and
was killed by hand after ~5 min; the per-input fallback (6/7) is what the table shows. `cell.py` now passes
`-timeout=25` to the replay; every later cell ran with that fix. (3) `default_bzalloc`'s coverage was
re-collected after `collect()` gained `ASAN_OPTIONS=detect_leaks=0` (LSan had failed every replay of a
function that hands back a malloc'd pointer); its export adds no region the other harnesses had not
reached. (4) A first attempt at the four bzip2 cells ran them concurrently and died in the build phase
(`EDQUOT`); no data from it is used.

**What is established.** The validator reaches 7 090 of 8 789 regions (0.807) against the suite's 7 007
(0.797), with 481 regions only the validator reaches and 398 only the suite reaches; at function level
the suite reaches 5 more functions than the validator (51 vs 46). Coverage was still creeping on the two
format-consuming boundaries at the last checkpoint (+29 regions in the final half hour, 0 functions);
it is reported as decelerating, not saturated. **This is the negative control**: 1 338 corpus inputs
replayed with the C reference beside the translation produce 0 divergences; 1 046 sampled termination
artifacts adjudicate to 0 confirmed (810 `ub_associated`, 214 `ub_associated_termination`, 17
timeout-inconclusive, 5 `instrument_only`), and the two public boundaries carry no artifact at all.

**What is not established.** The 50 197 termination artifacts on the internal sort boundaries are the
input model's out-of-contract inputs, identical in shape on all four tools, adjudicated at the sample
(PROTOCOL.md §4, adjudication depth); they say nothing about c2rust. Run-to-run variance of a single
fork-mode campaign is real: an earlier 300 s run of the same generator reached 2 738 regions on
`BZ2_bzBuffToBuffDecompress` where this run's 300 s snapshot reached 2 527 (≈5 % on that boundary);
no repeat campaigns were run, so the per-cell numbers are single-run numbers. `BZ2_bz__AssertH__fail`
(C `exit(3)` on every input) and `mainSort` (out of contract at the first input) have no coverage
export. The superseded hand-schema cell's numbers (`../c2rust_handschema_superseded/`, 10 harnesses,
0.798 / only-ours 409) are history, not a comparison point.
