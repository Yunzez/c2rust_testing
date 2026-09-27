# RQ4 — coverage beyond shipped tests: genann

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md).
Planner: the **producer-bridge pilot** ([`docs/producer_bridge_pilot.md`](../../../docs/producer_bridge_pilot.md)) —
a `genann*` parameter is built on each side by the library's own `genann_init`, the target is called, `genann_free`
releases it; `srand(42)` before each producer call because `genann_init → genann_randomize → rand` (found by the
call-graph fixpoint). Status 2026-09-05: **five cells complete**, 3 600 s each, serial, one campaign and one corpus per
cell. PtrTrans genann is declaration-only (E1) and stays N/A.

C source per cell (version skew recorded, never pooled): c2rust, Laertes, C2SaferRust and **CROWN** consumed the
2015 genann (12 functions; CROWN is a Rust→Rust lifter applied to the c2rust translation — its first cell, paired
with genann-1.0.0 by mistake, was killed in its build phase and produced no number); SACTOR consumed genann-1.0.0
(15 functions). `tests_side_results.json` records both.

## Cell table (per-tool `RUN.md` carries procedure, deviations and limits)

| tool | tests side (minctest via the transpiled `test.rs`) | planned / built of C fns | corpus | fn tests | fn ours | reg tests | reg ours | only-tests | only-ours | divergences on replay | confirmed |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| **c2rust** | **PASS** — baseline | 10 / 10 of 12 | 183 | 11/12 (0.917) | 10/12 (0.833) | 513/573 (0.895) | 462 (**0.806**) | 70 | 19 | **0** / 183 | **0** of 401 sampled |
| Laertes | **PASS** — baseline | 10 / 10 of 12 | 190 | 13/17 (0.765) | 12/17 (0.706) | 518/587 (0.882) | 467 (0.796) | 70 | 19 | **0** / 190 | 0 of 401 |
| CROWN | **PASS** — baseline (`main_0` exposed at packaging level) | 10 / 10 of 12 | 167 | 11/12 (0.917) | 10/12 (0.833) | 514/574 (0.895) | 467 (0.814) | 68 | 21 | **0** / 167 | 0 of 401 |
| C2SaferRust | TEST-FAILS (`persist`: `genann_write(&mut File)` unwrap on EBADF) | 10 / 10 of 12 | 186 | — | 10/12 (0.833) | — | 459/563 (0.815) | — | — | **0** / 186 | 0 of 335 |
| SACTOR | TEST-UNAVAILABLE (no transpiled driver) | 13 / 13 of 15 | 311 | — | 15/21 (0.714) | — | 506/716 (0.707) | — | — | **51** / 311 | **51 `confirmed_divergence`** = #32 on three boundaries |

`—` = not a baseline (PROTOCOL §2): universe from a link-dead-code denominator, partition Ours / Neither. The two
functions the suite reaches and the validator never does are `genann_read` / `genann_write` (`FILE*` parameters,
not constructible); `only-ours` is `genann_act_linear` (the suite never selects the linear activation) plus branches
of `run` / `train` the suite does not take. Raw region counts are per-translation identities; compare fractions and
candidate counts across tools. All five cells pass the four sanity checks (`cells.json`).

## Producer bridge — same campaign, harnesses that need no bridge only (`ablation_producer_bridge.json`)

| tool | without the bridge | with | Δ regions |
|---|---:|---:|---:|
| c2rust | 6/12 fn, 131/573 reg (0.229) | 10 fn, 462 reg (0.806) | +331 |
| Laertes | 6/17, 131/587 (0.223) | 12, 467 (0.796) | +336 |
| C2SaferRust | 6/12, 130/563 (0.231) | 10, 459 (0.815) | +329 |
| CROWN | 6/12, 124/574 (0.216) | 10, 467 (0.814) | +343 |
| SACTOR | 5/21, 119/716 (0.166) | 15, 506 (0.707) | +387 |

Not a second run: the union of the five (SACTOR: one) harnesses whose parameters are scalars, from the same corpora.

## What this library says

1. **Three of five translations pass the library's own suite; the validator sits just below the suite on all three**
   (0.806 / 0.796 / 0.814 vs 0.895 / 0.882 / 0.895 of regions), and the gap is one thing: file I/O, which the
   generator does not construct. RQ4 does not need the validator to win on every library.
2. **The negative control holds at every layer on four translations**: 0 replay divergences on c2rust, Laertes,
   C2SaferRust and CROWN, and 0 confirmed artifacts in their samples; E1's certificates for these four are consistent.
3. **SACTOR's cached-sigmoid headline (#32) is re-found with no hand work**: 17 / 21 corpus inputs diverge on each of
   `genann_act_sigmoid_cached`, `genann_act_hidden_indirect`, `genann_act_output_indirect` (the 4 that agree are the
   `a < −15` / `a > 15` clamp paths that never read the table); all 51 confirm with the C side clean. None of the
   three boundaries is reachable without the producer bridge.
4. **The bridge is what makes genann measurable**: without it the planner reaches the activation leaves and
   `genann_init` only (0.17–0.23 of regions); with it, every API but the two `FILE*` ones.
5. **A driver-level observation on C2SaferRust**: its own transpiled suite aborts at `persist` — `FILE*` reshaped to
   `&mut std::fs::File`, `writeln!().unwrap()` on a bad descriptor. Not a fuzz finding; recorded in
   `tests_side_results.json`.

## Gaps and limits

- **Oracle limit of the pilot** (documented before the cells ran): `genann_run` returns an interior pointer into the
  object's `output` array and `genann_train` returns void, so their output *values* are not compared — rung 3
  (nullness) and the input arrays are. SACTOR's #32 is visible only because the activation boundaries return a
  `double`. Lifting `run`/`train` to a value oracle needs the deferred "returned pointer addresses `ann->outputs`
  doubles" relation.
- Artifacts are dominated by two harness-side noise classes, both adjudicated and none promoted: `genann_init` as
  a target takes full-range scalars (a one-sided guard has no upper bound) → overflow panics that the in-loop UB
  gate rejects on the C side (`ub_associated*`); `genann_copy` returns a fresh object nothing frees → rss-limit
  `oom-*` artifacts that replay normally (`not_reproducible`). On SACTOR the three cached-sigmoid boundaries also
  collect ~4 000 NaN inputs each that hit C's own `assert(!isnan(a))`, which SACTOR transpiled too (`ub_associated`).
- Corpora are small (8–53 inputs per boundary): floating-point inputs with few branches saturate at once; the
  3 600 s budget is far more than this library needs, and is kept for uniformity.
- Single campaigns, no repeats.
- PtrTrans: no runnable artifact (declaration-only).

## Files

`tests_side_results.json`, `cells.json`, `ablation_producer_bridge.json`; per tool: `RUN.md`, `funnel.json`,
`plans.json`, `analysis/`, `divergences/`, `confirm_sample/`, `harnesses/`, `candidates_sample/`,
`confirmed_inputs/` (SACTOR), `corpus.tar.gz`, `harness_exports.tar.gz`, `artifact_hashes.json`, `raw/`
(tests export, denominator, adapter source, run log). Pairs: `benchmark/pairs/rq4/genann_<tool>/`.
