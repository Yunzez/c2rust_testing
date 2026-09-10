# tulip × Laertes — base / grid seed experiment (2026-09-09)

Same 212 harness binaries (built once in the base arm, reused with `--reuse-bins`), two 600 s rust-only
fork-mode campaigns, libFuzzer seed 42, `-max_len 65536`, fresh corpus per arm; grid seeds byte-identical to the
c2rust cell's (121 boundaries × 6, `seeds/manifest.json`). Universe = the cell's archived rlib denominator
(`raw/denominator.json`, 216 functions / 13 191 regions; the smoke suite SIGSEGVs on this translation, so there
is no tests-side reference). No random arm on this cell (user decision: the length effect was measured on c2rust).

| arm | initial corpus | final corpus | fn t=600 | regions t=0 (seeds only) | regions t=600 | Δ fuzzing beyond seeds | region cov (artifact level) |
|---|---:|---:|---:|---:|---:|---:|---:|
| base — cell.py's 64-byte default seed | 213 | 1 751 | 211/216 | 3 155 | 3 169 | 14 | 0.239 → **0.240** |
| grid — + 6 seeds per seeded boundary, every `options` element ∈ {1, 2, 3, 5, 10, 20} | 939 | 3 335 | 212/216 | 8 384 | 8 563 | 179 | 0.636 → **0.649** |

The archived automatic campaign (3 600 s) is 3 168 / 13 191 = 0.240: the base arm reproduces it to one region.

## Why the fraction is 0.649 where c2rust's is 0.924 — verified decomposition of the denominator

The Laertes universe holds 3 functions / 3 893 regions that c2rust's does not; the 223 functions the two
universes share have **identical region counts function by function** (Laertes − c2rust = 0), so the whole
difference sits in those three:

| translator-added function | regions | reachable from any boundary? |
|---|---:|---|
| `indicators_index::laertes_init_ti_indicators` — the severed initialiser of the 105-entry `ti_indicators` table | **3 887** | no: defined, never called (this is defect **C11**'s root cause) |
| `indicators::adxr::ti_buffer::new`, `…::Default::default` — two helpers Laertes added | 6 | no |

Excluding the severed initialiser alone: grid **8 563 / 9 304 = 0.920**, base 3 169 / 9 304 = 0.341 — the same
reach as c2rust (0.924 / 0.344). The artifact-level figure stays the primary one (the denominator rule of
`docs/rq4_denominator_decision_2026-09-08.md` makes no reachability exclusions); the scoped figure is reported
beside it with this decomposition, and the 0.649 is itself a symptom of C11, not of the seeds.
(Computed from the two cells' `raw/denominator.json` exports, matching functions by demangled name.)

## Archived here

Per arm: `analysis/` (result.json + identity lists), `funnel.json`, `campaign_params.json`, `snapshots.json`,
`corpus.tar.gz`. `t0/`: `t0_status.json` (batch vs per-input replay per harness; 1 base harness failed to build
its coverage binary) and the seeds-only result.json per arm. `COMPARE.md`: per-boundary table — harness-level
counts from `per_harness` (the llvm-cov exports were deleted after the application-level analysis to stay
under the scratchpad's file quota), so per-function totals are n/a on this cell. Rust-only crash artifacts
(base 1 751-corpus campaign: see funnel `artifacts`) are not replayed or archived.

## Combined replay and sampled confirmation of the grid corpus (2026-09-10)

Replay of the 3 335 final-corpus inputs: **3 256 normal, 39 ub-gated, 23 divergence, 17 signal**. Confirmation
(sample 200 per boundary per channel) over those and the 4 809 rust-only artifacts: **3 880
`ub_associated_termination`**, **20 `ub_associated`** (C reference faults under ASan on the input: harness
input-model gap), **23 `confirmed_divergence`**, **3 `confirmed_termination`**.

- The 3 terminations are all `ti_find_indicator`, frame `strcmp` on the zero page — **C11**, the severed
  `ti_indicators` initialiser, re-found by the seeded corpus (same site, same signature as the archived cell).
- The 23 divergences are the same NaN family as on c2rust, verified element by element (`nan_probe.json`):
  ti_kvo 13, ti_kama 3, ti_bbands / ti_dema / ti_fisher / ti_hma / ti_tema 1 each — every differing element NaN
  on both sides (payload only); ti_ultosc_start 2 — bit-identical `options` containing NaN, compared with `!=`.
  **0 new defects; C11 re-found.**
