# RQ2 — UB-free gate precision (v1, artifact-replay)

Plan: `results/rq2_eval_plan.md`. Runner: `scripts/eval_rq2_ubgate.py` (per-artifact REPLAY classification,
NOT a two-fuzz-distribution comparison). Rows: `results/rq2_rows/*.json`.

**Method (per boundary):** fuzz gate-OFF (UB-blind oracle) → for each crash artifact, replay under gate-ON
(deterministic: clean ⟹ gate rejected ⟹ C hit UB ⟹ `UB_SUPPRESSED`; crash ⟹ `UB_FREE_DIVERGENCE` /
`GATE_MISS`) + a standalone **full-UBSan** replay of the decoded input for the UB kind + `file:line`
evidence (the in-loop minimal-flag gate cannot emit that).

## Table B — controls (confusion matrix) — DONE for the 2 ready cases
| case | kind | status | gate-OFF | gate-ON (replay) | class | UB evidence |
|---|---|---|---|---|---|---|
| clip | correct translation | ready ✓ | CLEAN (13.7M runs/26s, 0 crash) | — | — | **TN** |
| sign_extend | UB false positive | ready ✓ | DIVERGE (Rust panics, input val=0/bits=0) | **CLEAN** (rejected) | **UB_SUPPRESSED** | `shift exponent 32 too large` @ `sign_extend.c:6:25` |
| aptx_bin_search | ? | ready | (to run) | | | |
| u8encode_ | UB-free real bug | pending-bridge | | | | (sensitivity control) |
| g3_g2_bug | injected UB-free bug | controlled-injected | | | | (sensitivity control) |

The suppression is proven **per artifact by replay**: the same input that makes the UB-blind oracle report
a bug is, on replay, rejected by the gate because a full-UBSan build shows the C reference is UB
(shift ≥ width) on it. Not "gate-ON fuzz happened to find nothing."

## Tables A / C — pending (scale RQ2a/b on the faithful c2rust corpus)
- Table A: all harnessable c2rust boundaries (report full A1 denominator) → OFF artifacts → replay classes.
- Table C: UB-class taxonomy of the suppressed artifacts (shift / signed-overflow / …), tier-tagged.

## Status
- Runner + replay classifier + automated full-UBSan evidence: **built & working** (`eval_rq2_ubgate.py`,
  `scripts/rq2_boundaries.json`).
- Controls: clip = TN, sign_extend = UB_SUPPRESSED (with evidence). RQ2a skeleton + evidence path done.
- NEXT: RQ2c sensitivity (injected `g3_g2_bug` + `u8encode_` when its bridge is ready — gate-ON must still
  flag) → then scale RQ2a/b across the faithful c2rust boundary set (Tables A + C).

Reproduce: `python3 scripts/eval_rq2_ubgate.py --boundaries scripts/rq2_boundaries.json --secs 25
--json results/rq2_rows/controls.json`
