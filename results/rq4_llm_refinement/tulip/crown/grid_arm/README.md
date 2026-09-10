# tulip × CROWN — grid seed arm (2026-09-10)

One harness build (212 harnesses), one 600 s rust-only fork-mode campaign seeded with the grid seeds
(byte-identical to the c2rust cell's: 121 boundaries × 6, `seeds/manifest.json`), libFuzzer seed 42,
`-max_len 65536`. No base arm (baseline = the archived automatic campaign, `results/rq3_coverage/tulip/crown/`,
whose 600 s reproduction was exact on c2rust and Laertes). Universe = the archived tests build (213 functions /
9 219 regions); the smoke suite passes 12/12 on this translation and reaches 8 616 = 0.935 — a reach reference.

| arm | initial corpus | final corpus | fn | regions t=0 (seeds only) | regions final | Δ fuzzing beyond seeds | region cov |
|---|---:|---:|---:|---:|---:|---:|---:|
| baseline — archived automatic campaign (3 600 s), default 64-byte seed | 213 | 1 814 | 212/213 | 3 180 | 3 197 | 17 | 0.345 → **0.347** |
| grid — + 6 seeds per seeded boundary, every `options` element ∈ {1, 2, 3, 5, 10, 20} (600 s) | 939 | 3 351 | 213/213 | 8 409 | 8 595 | 186 | 0.912 → **0.932** |

Per harness the grid arm is identical to c2rust's to ±1 region on 12 harnesses (6 lower, 6 higher, both sums
6) and equal elsewhere; the absolute counts 8 409 → 8 595 are the same as c2rust's, and the fraction is higher
only because CROWN's universe is 79 regions smaller. Together with C2SaferRust (0.902, the gap being S15's
`ti_adx`) and Laertes (8 563; 0.649 artifact-level because of C11's severed initialiser, 0.920 without it),
the same seeds reach the same code on all four translations: the effect is a property of the C library's
option domain, not of one translator.

## Archived here

`grid/` (analysis result.json + identity lists, funnel.json, campaign_params.json, snapshots.json,
corpus.tar.gz); `t0/` (t0_status.json; seeds-only result.json for the default seed and the grid seeds);
`COMPARE.md` (per-boundary, harness-level counts); `seeds/manifest.json`. The 4 731 rust-only crash artifacts
on 51 boundaries are replayed and sample-confirmed by the queued replay chain.

## Combined replay and sampled confirmation of the grid corpus (2026-09-10)

Replay of the 3 351 final-corpus inputs: **3 276 normal, 41 ub-gated, 18 divergence, 16 signal**. Confirmation
(sample 200) over those and the 4 944 rust-only artifacts: **3 911 `ub_associated_termination`, 20
`ub_associated`, 18 `confirmed_divergence`**, no termination. The 18 divergences are the NaN family (ti_kvo 8,
ti_kama 3, ti_bbands / ti_dema / ti_fisher / ti_hma / ti_tema 1 each, ti_ultosc_start 2), verified element by
element (`nan_probe.json`): NaN payload differences in f64 rows and `!=` on bit-identical NaN options.
**0 defects** — the second faithful translation is as clean as c2rust on the seeded corpus.
