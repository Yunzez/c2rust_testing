# tulip × c2rust — three-arm seed experiment (2026-09-09)

Same 212 harness binaries (generator sha256[:16] = 3d43f75aea807da1, built once in the base arm and reused with
`--reuse-bins`), three 600 s rust-only fork-mode campaigns, libFuzzer seed 42, `-max_len 65536`, fresh
corpus per arm. Universe = the cell's archived tests build (9 298 regions, `raw/tests_coverage.json`).

| arm | initial corpus | final corpus | regions t=0 (seeds only) | regions t=600 | Δ fuzzing beyond seeds | region cov |
|---|---:|---:|---:|---:|---:|---:|
| base — cell.py's 64-byte default seed | 213 | 1 707 | 3 180 | 3 197 | 17 | 0.342 → **0.344** |
| random — + 6 length-matched pseudorandom seeds per seeded boundary | 939 | 3 279 | 3 473 | 3 602 | 129 | 0.374 → **0.387** |
| grid — the same bytes, every `options` element ∈ {1, 2, 3, 5, 10, 20} | 939 | 3 453 | 8 409 | 8 595 | 186 | 0.904 → **0.924** |

The archived automatic campaign (3 600 s) is 3 197 / 9 298 = 0.344: the base arm reproduces it region for
region in 600 s. The shipped smoke suite reaches 8 616 = 0.927 and is a reach REFERENCE only.

**What is archived here.** Per arm: `analysis/` (c2r_coverage result.json + the four identity lists),
`funnel.json`, `campaign_params.json`, `snapshots.json`, `corpus.tar.gz` (the final corpus). `t0/`:
`t0_status.json` (how each harness's seed-only replay was obtained: batch, or per-input after a seed
panicked and killed the batch) and each arm's seeds-only result.json. `seeds/manifest.json`: the byte
layout of every seeded boundary, the option offsets, and the sha256 of all 121 × 6 × 2 seeds; the seed
bytes themselves are regenerated deterministically by
`python3 scripts/rq4/materialize_seeds.py --plans results/rq3_coverage/tulip/c2rust/plans.json --out DIR`
(PRNG seed 7, plan sha256 recorded in the manifest). `COMPARE.md`: the per-boundary table.
Scripts: `scripts/rq4/seed_experiment/`.

**Not done here.** The rust-only crash artifacts (base 516, random 4 518, grid 4 161) were not replayed
or confirmed; the archived cell's confirmation (243 `ub_associated_termination`, all the C-side
float→int cast) covers the class, and the seeded corpora were not run through the combined replay.
The 92 boundaries without a fuzz-filled `options` array carry only the default seed in every arm.
The llvm-cov exports (196 MB per arm) are not archived.

## Combined replay and sampled confirmation of the grid corpus (2026-09-10)

`replay_cell.py` over the 3 453 final-corpus inputs, C and Rust side by side with the comparison ladder on:
**3 366 normal, 51 ub-gated, 20 divergence, 16 signal**. `confirm_cell.py --sample 200` over those and the
4 374 rust-only crash artifacts: **3 720 `ub_associated_termination`** (the C-side `(int)` cast of an
out-of-range or NaN option, as in the archived cell), **20 `ub_associated`** (`ti_dpo`, `ti_zlema`, `ti_linreg`,
`ti_linregintercept`: the C reference itself faults under ASan on the input — an input-model gap of the harness,
not a translation difference), 4 not reproducible, and **20 `confirmed_divergence`**.

The 20 confirmed divergences on the negative control were re-examined one by one with an instrumented copy of
each harness that prints every differing element on both sides (`nan_probe.py`, `nan_probe.json`):

| boundary | inputs | what differs |
|---|---:|---|
| ti_kvo | 12 | every differing output element is NaN on both sides — C carries the input row's NaN payload (`0x7ffe27b1…`), Rust the canonical NaN (`0xfff8…`) |
| ti_bbands, ti_dema, ti_fisher, ti_hma, ti_kama, ti_tema | 1 each | same: NaN vs NaN, payload only |
| ti_ultosc_start | 2 | the two sides' `options` arrays are bit-identical (one element is NaN); the harness compares this read-only array with `Vec<f64> !=`, and NaN ≠ NaN |

So none is a value difference: they are two NaN blind spots of the generated oracle — the bitwise `to_bits`
comparison of f64 rows treats NaNs with different payloads as different, and the `!=` comparison of a
`plan_arr` treats identical NaNs as different. Neither appeared in the archived campaigns because random
options almost always trip the C-side float-cast UB gate before any comparison runs; the grid seeds' legal
options let the comparison run on rows that contain NaN filler. **Verdict: 0 defects on the negative control;
20 oracle artefacts (NaN).** Fix for the next generator version: floats compare equal iff bits are equal or both
are NaN, for rows and plan arrays alike (`docs/rq4_runbook.md`). The 20 verdicts stay `confirmed_divergence`
in `confirm_sample/` as the pipeline produced them; this section is the adjudication.

## Re-adjudication under generator 0.9 (NaN-equivalent oracle, 2026-09-10)

The harness binaries were rebuilt with generator 0.9 (`replay_gen09/funnel.json` records its hash) and the SAME
archived grid corpus and rust-only artifacts were replayed and confirmed again, without re-fuzzing. Replay:
3368 `normal`, 51 `ub-gated`, 18 `nan_equivalent`, 16 `signal`. Confirmation (sample 200): 3720 `ub_associated_termination`, 20 `ub_associated`, 4 `not_reproducible`. Every NaN-only difference now reports `nan_equivalent`
instead of `confirmed_divergence`; the verdict table above (0.8 oracle) is kept for provenance, this one is the
adjudication of record. Files: `grid/replay_gen09/` (divergences, confirm_sample, funnel.json, replay.log).
