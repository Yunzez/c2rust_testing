# Seed refinement on tulip — deterministic definition-derived grid seeds (2026-09-09/10)

*An ablation on top of the automatic RQ4 baseline. The 37-cell results stand unchanged; the shipped smoke suite
is a reach reference only; no LLM anywhere. Rule: [`SEED_RULE.md`](SEED_RULE.md) (frozen; identical bytes on
all four translations). Follow-up plan for the other libraries: `docs/seeding_policy_plan.md`.*

## The question

tulip's automatic campaigns enter 212 of 213 functions but cover a third of the regions. The cause was known
(options are doubles the indicator casts to `int` and rejects when out of range) and one thing was not: in the
generated harness the `options` array is decoded AFTER the 4 096-double input row, so any input shorter than
32 780 bytes decodes `options[0] = 0.0` and the indicator returns at its first guard. Is the gap input length,
the legal-value domain, or the fuzzing budget?

## Design

Same harness binaries per cell, 600 s rust-only fork campaigns, libFuzzer seed 42, `-max_len 65536`, fresh corpus
per arm; coverage measured on the final corpus (t=600) and on the initial seeds alone (t=0, one instrumented
build per harness replaying only the seeds; per-input fallback when a seed panics), so the seed's own reach and
the fuzzer's increment are separated. Arms: **base** (cell.py's 64-byte default seed), **random** (six
length-matched pseudorandom seeds per boundary; c2rust only), **grid** (the same bytes with every `options`
element overwritten by {1, 2, 3, 5, 10, 20}; 121 of 213 boundaries have a fuzz-filled `options`). The base arm
reproduced the archived 3 600 s campaigns to within one region on c2rust and Laertes, so the other two cells use
the archived campaign as baseline.

## Result

| translation | universe | automatic baseline (archived) | grid, seeds only (t=0) | grid, after 600 s | Δ fuzz | smoke suite |
|---|---:|---:|---:|---:|---:|---:|
| c2rust | 9 298 | 3 197 (**0.344**) | 8 409 | 8 595 (**0.924**) | 186 | 8 616 (0.927) |
| Laertes | 13 191 ¹ | 3 168 (**0.240**; 0.341 ¹) | 8 384 | 8 563 (**0.649**; 0.920 ¹) | 179 | — (SIGSEGV) |
| C2SaferRust | 9 306 | 3 167 (**0.340**) | 8 203 | 8 393 (**0.902**) | 190 | — (panics) |
| CROWN | 9 219 | 3 197 (**0.347**) | 8 409 | 8 595 (**0.932**) | 186 | 8 616 (0.935) |

c2rust also: random arm 3 473 → 3 602 (0.387), i.e. length alone is worth +4.3 points, legal values +58.

¹ Laertes' universe holds 3 893 regions in three translator-added functions no boundary reaches — 3 887 of them
in `laertes_init_ti_indicators`, the severed table initialiser that is defect C11; the 223 shared functions have
identical region counts. Artifact-level figures are primary; the scoped figure (÷ 9 304) is given beside them.

## What it says

1. **The bottleneck was the legal control values, not length or budget.** Six grid seeds per boundary take every
   translation from a third of its regions to the smoke suite's reach; the fuzzer adds ~180 regions on top in
   600 s, and the base arm's 600 s adds 14–20 to its single seed. The gain is initial reachability, not better
   exploration.
2. **Not a translator-specific effect.** Identical seeds, near-identical absolute reach on all four (8 563–8 595).
3. **The residue is explainable per cell.** c2rust: 440 assert-failure arms (unreachable when the translation is
   correct), 127 regions of `ti_psar` (its guard needs `accel_max > accel_step`, which a uniform grid never
   satisfies — deliberately not fixed), 37 `size <= start` early returns, 19 data-dependent branches.
   C2SaferRust: one boundary, `ti_adx`, 211 regions behind **S15** (the options pointer cast to `i32`) — a
   catalogued semantic defect showing up as a reach gap. Laertes: C11 in the denominator.
4. **The decoder layout is a generator design issue for the next version**: small control fields should be
   decoded before large buffers.

## Not done / caveats

The grid values are a fixed set chosen from tulip's guard shapes, not derived from the plans (SEED_RULE.md); the
rule was frozen before the last three cells ran and never adjusted. Seeds longer than `-max_len` (16 boundaries,
up to 98 316 bytes) are executed whole and decoded — they break the initial reachability barrier; whether the
fuzzer keeps mutating at that length is not established. Rust-only crash artifacts of the grid campaigns
(4 161 / 2 113-corpus Laertes / 4 476 / 4 731) and the combined replay + sampled confirmation of the four grid
corpora: see the section below.

## Combined replay and confirmation of the grid corpora

Every grid corpus was replayed C-and-Rust side by side (`replay_cell.py`) and its candidates plus the campaign's
rust-only crash artifacts adjudicated by the four-channel confirmation (`confirm_cell.py --sample 200`). The first
pass, under generator 0.8, returned 86 `confirmed_divergence` verdicts across the four cells; a per-input probe
(`nan_probe.py`) showed every one outside S15 to be a NaN artefact of the generated oracle — f64 rows compared
by bits (C keeps an input NaN's payload, Rust emits the canonical NaN) and read-only arrays compared with `!=`.
That rule is now executable: generator **0.9** compares numeric float outputs NaN-equivalently and reports
`nan_equivalent` when that was the only difference. The corpora were then re-replayed and re-confirmed under 0.9
without re-fuzzing (`<cell>/grid/replay_gen09/`); this is the adjudication of record:

| translation | replay (0.9): normal / ub-gated / nan_equivalent / signal (+panic, divergence) | confirmation (0.9, sample) | verdict |
|---|---|---|---|
| c2rust | 3368 / 51 / 18 / 16 | 3720 `ub_associated_termination`, 20 `ub_associated`, 4 not reproducible | **0 defects** |
| Laertes | 3258 / 39 / 21 / 17 | 3880 `ub_associated_termination`, 20 `ub_associated`, **3 `confirmed_termination`** | **C11 re-found** (`ti_find_indicator`, zero-page `strcmp`); nothing else |
| C2SaferRust | 3372 / 47 / 23 / 16 (+12 panic, 7 divergence) | 3767 `ub_associated_termination`, 21 `ub_associated`, 64 not reproducible, **18 `confirmed_divergence` + 5 `confirmed_termination`** (+5 instrument_only, 1 out_of_contract) | **S15 re-found**, all on `ti_adx_start` / `ti_adx` (the options pointer cast to `i32`; panic-vs-divergence split varies with ASLR); nothing else |
| CROWN | 3278 / 41 / 16 / 16 | 3911 `ub_associated_termination`, 20 `ub_associated` | **0 defects** |

Per cell, `nan_equivalent` + 2 equals the 0.8 pass's NaN-family count (the 2 are `ti_ultosc_start` inputs whose
`options` are bit-identical NaNs, now `normal`). The `ub_associated` rows are the C reference faulting under ASan
on the input (`ti_dpo`, `ti_zlema`, `ti_linreg`, `ti_linregintercept`): a harness input-model gap, not a
translation difference. The 37 archived cells are not re-run: their verdicts were all UB-gated or on integer
and byte outputs, and they stay bound to generator 0.8's hashes.

## Files

`SEED_RULE.md`; `c2rust/three_arm/`, `laertes/two_arm/`, `c2saferrust/grid_arm/`, `crown/grid_arm/` (each: README,
COMPARE.md, per-arm analysis + funnel + params + corpus.tar.gz, t0/, seeds/manifest.json); `pilot/` (the ti_sma
probe and the layout validation). Scripts: `scripts/rq4/materialize_seeds.py`, `scripts/rq4/seed_experiment/`.
