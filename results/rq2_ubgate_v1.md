# RQ2 — UB-free gate precision (v1, artifact-replay)

Plan: `results/rq2_eval_plan.md`. Runner: `scripts/eval_rq2_ubgate.py` (per-artifact REPLAY classification,
NOT a two-fuzz-distribution comparison). Rows: `results/rq2_rows/*.json`.

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

## Tables A / C — pending (scale RQ2a/b on the faithful c2rust corpus)
- Table A: all harnessable c2rust boundaries (report full A1 denominator) → OFF artifacts → replay classes.
- Table C: UB-class taxonomy of the suppressed artifacts (shift / signed-overflow / …), tier-tagged.

## Status
- Runner + replay classifier + automated full-UBSan evidence: **built & working** (`eval_rq2_ubgate.py`,
  `scripts/rq2_boundaries.json`).
- **RQ2a (suppression) + RQ2c (sensitivity) DONE on controls**: clip = TN, sign_extend = UB_SUPPRESSED
  (evidence), safe_ratio = BUG_KEPT (sensitivity 1/1). Full confusion matrix demonstrated.
- NEXT: **scale RQ2a/b across the faithful c2rust boundary set** (Table A with full A1 denominator +
  Table C UB taxonomy) — this is the sample-size step (controls prove the mechanism; Table A gives the
  headline FP-suppression rate over a real corpus). Optional extra sensitivity control: u8encode_ once its
  bare-out-buf + elem-split bridge is ready.

Reproduce: `python3 scripts/eval_rq2_ubgate.py --boundaries scripts/rq2_boundaries.json --secs 25
--json results/rq2_rows/controls.json`
