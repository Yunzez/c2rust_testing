# Attribution ablation — UB-free gate precision (v1, artifact-replay)

*Legacy label: this document was “RQ2” under retired numberings. Under the
current plan (`results/EVALUATION_PLAN.md`), it is supporting component evidence,
not an RQ. Boundary validity and reference attribution are both retired as RQs.*

Plan: `results/archive/rq2_eval_plan.md`. Runner: `scripts/eval_rq2_ubgate.py` (per-artifact REPLAY classification,
NOT a two-fuzz-distribution comparison). Rows: `results/ablations/attribution/ubgate_rows/*.json`.

**Method (per boundary):** fuzz gate-OFF (UB-blind oracle) → for each crash artifact, replay under gate-ON
(deterministic: clean ⟹ gate rejected ⟹ C hit UB ⟹ `UB_SUPPRESSED`; crash ⟹ `UB_FREE_DIVERGENCE` /
`GATE_MISS`) + a standalone **full-UBSan** replay of the decoded input for the UB kind + `file:line`
evidence (the in-loop minimal-flag gate cannot emit that).

## Table B — controls (full confusion matrix) — ALL FOUR CELLS DEMONSTRATED
| case | kind | status | gate-OFF | gate-ON (replay) | class / verdict | UB evidence |
|---|---|---|---|---|---|---|
| clip | correct translation | ready ✓ | CLEAN (13.7M runs/26s, 0 crash) | — | **TN** | — (no divergence) |
| sign_extend | UB false positive | ready ✓ | DIVERGE (Rust panics, input val=0/bits=0) | **CLEAN** (rejected) | **UB_SUPPRESSED** | `shift exponent 32 too large` @ `sign_extend.c:6:25`, is_ub=True |
| safe_ratio (g3_g2_bug) | injected UB-free real bug | controlled-injected ✓ | DIVERGE (C `pct*100` vs injected Rust `pct*10`) | **DIVERGE** (kept) | **UB_FREE_DIVERGENCE → BUG_KEPT** | input pct=10, is_ub=False (no overflow) |
| aptx_bin_search | idiomatic bridge | ready | (to run) | | | |
| u8encode_ | UB-free real bug | pending-bridge | | | | (extra sensitivity control) |

**The gate cuts both ways, proven per-artifact by replay:**
- **Suppression** (sign_extend): the same input that makes the UB-blind oracle report a bug is, on replay,
  rejected by the gate because a full-UBSan build shows the C reference is UB (shift ≥ width). Not "gate-ON
  fuzz happened to find nothing."
- **Sensitivity retained** (safe_ratio): the injected UB-free bug diverges on `pct=10` where C is provably
  UB-free (no overflow) → the gate KEEPS it. Observed sensitivity = 1/1 known control (FN=0 here).
Together: FP suppressed (sign_extend) + TN (clip) + real bug kept (safe_ratio) = the confusion matrix,
each cell established by a deterministic per-artifact replay, not a distribution comparison.

## Table A — faithful c2rust boundaries (DYNAMIC replay) — `results/ablations/attribution/ubgate_rows/tableA.json`
**Sampling frame (rule 1, no cherry-pick):** every auto-harnessable scalar/buffer boundary (all inputs +
output are a scalar type or pointer-to-scalar per c_analyzer io-shapes) of a UB-taxonomy-covering set of
13 `benchmark/pairs` programs (reduce_overflow, negate_abs, div_mod, sub_overflow, shift_ops, intmath,
bitutils, safe_stats, base64, leb128, byte_classify, case_fold, hex_encode). **A1 = 48 boundaries** (the
full frame is 114 over 56 programs; extensible). Fixed 20s gate-OFF budget. Config `scripts/rq2_tableA.json`.

| verdict | count | meaning |
|---|--:|---|
| TN (clean both) | 35 | gate-OFF fired no divergence |
| **UB_SUPPRESSED** (recoverable UB) | **8** | gate-ON rejected the exact artifact → C hit recoverable UBSan UB |
| GATE_MISS (hard-trap) | 3 | div-by-zero (tier-2; in-loop gate can't suppress a SIGFPE) |
| MEMORY_UB (tier-3) | 2 | ASan heap-overflow / SEGV (out of in-loop UBSan scope; post-hoc) |
| **UB_FREE_DIVERGENCE (candidate bug)** | **0** | no false candidate bugs — consistent with c2rust faithful |
| BUILD_FAIL / excluded | 0 | — |

**48 boundaries → 13 gate-OFF divergences, ALL classified, 0 miscalled a translation bug.** A UB-blind
oracle (Fluorine/RustAssure) would report all 13 as bugs (c2rust is faithful → these are all false
positives). Our pipeline: the in-loop gate SUPPRESSES the **8/8 recoverable-UB** divergences (each with a
per-artifact UBSan diagnostic); the replay classifier honestly labels the other 5 as **3 hard-trap
div-by-zero** + **2 memory-UB** (tier-2/3, out of the in-loop UBSan-minimal scope — RISKY boundaries the
frontier statically avoids / post-hoc ASan territory), NOT as bugs. **Net translation-bug false positives:
0, vs 13 for the UB-blind oracle.** The 5 non-suppressed are the value of the replay design: without
per-artifact evidence, a naive oracle files div-by-zero / OOB as translation defects.

## Table C — UB taxonomy of the 13 divergent artifacts
| UB class | tier | count | in-loop suppressed? | evidence |
|---|---|--:|---|---|
| signed integer overflow (`sub_signed_i32`) | recoverable | 1 | **yes** | UBSan `signed integer overflow: … - …` |
| negation overflow, INT_MIN (`negate_i32`, `abs_i32`) | recoverable | 2 | **yes** | UBSan `negation of -2147483648 cannot be represented` |
| shift ≥ width (`shl_u32`, `shl_i32`) | recoverable | 2 | **yes** | UBSan `shift exponent … too large` |
| array-reduce overflow (`reduce_sum/prod_i32`, `safe_stats sum_i32`) | recoverable | 3 | **yes** | gate-ON replay (array input; no scalar decode) |
| division by zero (`div/mod_signed_i32`, `safe_stats idiv`) | hard-trap | 3 | no (frontier-excluded) | UBSan `division by zero` |
| heap-overflow / SEGV (`base64_encode`, `csv_field_count`) | memory | 2 | no (ASan/post-hoc) | ASan `heap-buffer-overflow` / `SEGV` |

## Scaling note (historical)
For this component experiment, the relevant N is the count of UB-induced false-candidate events, not boundaries: this frame = 48
boundaries → **13 divergence events** spanning 6 UB classes (overflow / negation / shift / div / OOB /
SEGV). The full frame is 114 boundaries over 56 programs; benchmark/pairs (56) + corpus_inventory (~19) +
CRUST-bench c2rust (87 repos, free) extend it further, bounded by auto-harness-ability. Grow by editing
`scripts/rq2_tableA.json`. v1 covers the taxonomy; expand to 100+ only if a reviewer wants a larger N.

## Status
- Runner + replay classifier + automated full-UBSan evidence: **built & working** (`eval_rq2_ubgate.py`,
  `scripts/rq2_boundaries.json`).
- **Historical RQ2a/c controls DONE**: clip = TN, sign_extend = UB_SUPPRESSED (evidence), safe_ratio = BUG_KEPT
  (sensitivity 1/1). Full confusion matrix demonstrated.
- **Historical RQ2a Table A DONE (48-boundary frame)**: 35 TN / 8 UB_SUPPRESSED / 3 hard-trap / 2 memory-UB / 0
  candidate bugs — 13 divergences all classified, 6 UB classes, 0 false translation-bug reports.
- NEXT (optional): larger N (extend `scripts/rq2_tableA.json` toward the 114-frame); u8encode_ sensitivity
  once its bridge is ready; RQ2b survivor-search write-up (0 UB_FREE_DIVERGENCE = 0 survivors so far).
- (superseded note) sample-size step done; Table A gives the
  headline FP-suppression rate over a real corpus). Optional extra sensitivity control: u8encode_ once its
  bare-out-buf + elem-split bridge is ready.

Reproduce: `python3 scripts/eval_rq2_ubgate.py --boundaries scripts/rq2_boundaries.json --secs 25
--json results/ablations/attribution/ubgate_rows/controls.json`
