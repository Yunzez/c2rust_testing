# tulip × C2SaferRust — grid seed arm (2026-09-10)

One harness build (212 harnesses), one 600 s rust-only fork-mode campaign seeded with the grid seeds
(byte-identical to the c2rust cell's: 121 boundaries × 6, `seeds/manifest.json`), libFuzzer seed 42,
`-max_len 65536`. No base arm on this cell (user decision after c2rust's and Laertes' 600 s base arms reproduced
their archived 3 600 s campaigns to within one region): the baseline is the archived automatic campaign,
`results/rq3_coverage/tulip/c2saferrust/`. Universe = the cell's archived rlib denominator (213 functions /
9 306 regions; the smoke suite panics on this translation, so there is no tests-side reference).

| arm | initial corpus | final corpus | fn | regions t=0 (seeds only) | regions final | Δ fuzzing beyond seeds | region cov |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline — archived automatic campaign (3 600 s), default 64-byte seed | 212 | 1 776 | 212/213 | 3 147 | 3 167 | 20 | 0.338 → **0.340** |
| grid — + 6 seeds per seeded boundary, every `options` element ∈ {1, 2, 3, 5, 10, 20} (600 s) | 938 | 3 477 | 213/213 | 8 203 | 8 393 | 190 | 0.881 → **0.902** |

(The baseline's t=0 is the default seed replayed alone through this run's harness binaries; its final number is
the archived campaign's.)

## The gap to c2rust's 0.924 is one boundary, and it is a catalogued defect

The two universes are the same size to within 8 regions (9 306 vs 9 298; the only functions present in one and
not the other are the out-of-scope drivers, and 64 shared functions differ by 1–6 regions of lifted code), so
the fractions are comparable. Per harness, C2SaferRust's grid arm trails c2rust's on 72 harnesses by 1–3 regions
(the lifted code's extra regions) and on **one by 211: `ti_adx` (31 vs 242)**. That is **S15**: C2SaferRust's
`ti_adx_start` casts the `options` *pointer* to `i32` instead of loading `options[0]`, so with a legal period the
computed start is garbage, `ti_adx` returns before its loops on every seed, and the grid cannot reach the body.
The seeds make a semantic defect visible as a reach gap; the archived cell found S15 through its divergences.

## Archived here

`grid/` (analysis result.json + identity lists, funnel.json, campaign_params.json, snapshots.json,
corpus.tar.gz); `t0/` (t0_status.json; seeds-only result.json for the default seed and for the grid seeds);
`COMPARE.md` (per-boundary, harness-level counts: the llvm-cov exports were deleted after the application-level
analysis, so per-function totals are n/a); `seeds/manifest.json`. The 4 476 rust-only crash artifacts on 105
boundaries are replayed and sample-confirmed by the queued replay chain, not here.

## Combined replay and sampled confirmation of the grid corpus (2026-09-10)

Replay of the 3 477 final-corpus inputs: **3 370 normal, 47 ub-gated, 35 divergence, 6 panic, 19 signal**.
Confirmation (sample 200) over those and the 4 689 rust-only artifacts: **3 767 `ub_associated_termination`**,
21 `ub_associated`, 64 not reproducible, **41 `confirmed_divergence`, 4 `confirmed_termination`, 8
`instrument_only`, 1 `out_of_contract_access`**.

- **S15 re-found, on its own boundaries and only there.** `ti_adx_start`: 16 confirmed divergences (the returned
  start differs: C computes `((int)options[0] − 1) × 2`, C2SaferRust `(options as i32 − 1) × 2` from the pointer),
  3 confirmed terminations and 5 instrument-only (the same expression overflows: `lib.rs:393 attempt to multiply
  with overflow` on a seed whose `options[0]` decodes to a denormal); `ti_adx`: 1 termination, 3 instrument-only,
  1 out-of-contract. 24 confirmed outcomes, one root cause, already catalogued.
- The remaining 25 divergences are the NaN family (ti_kvo 16, ti_kama 2, ti_bbands / ti_dema / ti_fisher /
  ti_hma / ti_tema 1 each, ti_ultosc_start 2), verified with `nan_probe.json` exactly as on c2rust: NaN payload
  differences in f64 rows, and `!=` on bit-identical NaN options. **0 new defects.**

## Re-adjudication under generator 0.9 (NaN-equivalent oracle, 2026-09-10)

The harness binaries were rebuilt with generator 0.9 (`replay_gen09/funnel.json` records its hash) and the SAME
archived grid corpus and rust-only artifacts were replayed and confirmed again, without re-fuzzing. Replay:
3372 `normal`, 47 `ub-gated`, 23 `nan_equivalent`, 16 `signal`, 12 `panic`, 7 `divergence`. Confirmation (sample 200): 3767 `ub_associated_termination`, 64 `not_reproducible`, 21 `ub_associated`, 18 `confirmed_divergence`, 5 `instrument_only`, 5 `confirmed_termination`, 1 `out_of_contract_access`. Every NaN-only difference now reports `nan_equivalent`
instead of `confirmed_divergence`; the verdict table above (0.8 oracle) is kept for provenance, this one is the
adjudication of record. Files: `grid/replay_gen09/` (divergences, confirm_sample, funnel.json, replay.log).
