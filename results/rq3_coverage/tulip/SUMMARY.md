# RQ4 — coverage beyond shipped tests: tulipindicators

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md);
planner: the buffer-table capability (`docs/harness_plan_architecture.md`, addendum 2026-09-05 — `T**` indexed by
constants → one pointer parameter per row), which lifted the plannable set from 109 to 213 of 213 on every
translation. Status 2026-09-07: **four cells complete**, 3 600 s each, one campaign and one corpus per cell,
`-max_len 65536` for the table rows (`campaign_params.json`). C source: tulip 0.8.4 (`41e59fb33cef`); pair
source = one translation unit `tulip.c` over `indicators/*.c`, `utils/buffer.c`, `indicators_index.c`.

**Tests side** (`tests_side_results.json`): the shipped `smoke.c` over `tests/{atoz,extra,untest}.txt` through the
transpiled `smoke.rs`. c2rust and CROWN pass 12/12 → **acceptance baselines**; Laertes SIGSEGVs before any
output and C2SaferRust panics in its own buffer test (flexible array `vals[1]` → a one-element array) →
denominator universes.

## Cell table

| tool | tests side | planned / built of 213 | corpus | fn tests / ours | reg tests / ours | replay: div. / term. | confirmed (sample) |
|---|---|---:|---:|---:|---:|---:|---|
| **c2rust** | **PASS** (baseline) | 213 / 212 | 1 903 | 213 / 212 (both 212) | **0.927 / 0.344** (both 3 124, only-ours 73) | 0 / 0 (12 ub-gated) | **0** (243 `ub_associated_termination`) |
| **Laertes** | TEST-FAILS (SIGSEGV) | 213 / 212 | 1 813 | — / 211 of 216 | — / 0.240¹ | 0 / 1 | **2 `confirmed_termination` on `ti_find_indicator` → C11** |
| **C2SaferRust** | TEST-FAILS (buffer test) | 212 / 212 | 1 776 | — / 212 of 213 | — / 0.340 | 6 / 3 | **5 `confirmed_divergence` + 7 `confirmed_termination`, all `ti_adx_start` → S15** |
| **CROWN** | **PASS** (baseline) | 213 / 212 | 1 814 | 213 / 212 (both 212) | **0.935 / 0.347** (both 3 124, only-ours 73) | 0 / 0 (3 ub-gated) | **0** (243 `ub_associated_termination`, 1 not reproducible) |

¹ Laertes' universe (13 191 regions) includes its own runtime, unreachable from any boundary; compare only-ours
(3 168), not the fraction. The one unbuilt boundary on every cell is `ti_buffer_free` (planner exception on the
flexible array member of `ti_buffer`); C2SaferRust additionally refuses `ti_buffer_new` (`Box<ti_buffer>`).

## What this library says

1. **The first strict paired comparison in the study, and it is not flattering to the validator on regions.**
   On the two baseline cells the validator reaches every function the suite reaches (212/213 both) but
   only **a third of its regions** (0.344 vs 0.927). The cause is one thing, visible in every artifact:
   tulip's options are doubles the indicator casts to `int` and then rejects when out of range. A random
   double overflows the cast (C-side UB — all 243 `ub_associated_termination`, one per boundary:
   `-5.45e+245 is outside the range of representable values of type 'int'`) or lands outside `1..size`,
   and the indicator returns before its loops; the fuzzer gets no gradient through an early return, and
   every corpus saturated within its first minute (1 528 → 1 640 inputs from 60 s to 1 800 s). The
   rejection guard sits on a *local* derived from an array element (`period = (int)options[0]`), which
   the plan — derived from parameters and the guards on them — cannot see. The 73 only-ours regions are the
   `TI_INVALID_OPTION` paths the suite never takes. This is the input-model limit tulip exposes; it is
   recorded, not patched.
2. **Two new defects, both on translations the earlier evidence had certified or cleared.**
   - **C11, tulip × Laertes:** the static `ti_indicators[105]` table is all-default (`::new()`, NULL names)
     with its initialiser severed — the pattern the scanner had flagged for exactly this table and E3 had
     worked around with `Once`. `ti_find_indicator` faults on the zero page on every input. Caught by the
     preflight in one minute; confirmed under the zero-page rule (a NULL dereference is deterministic, not
     layout luck — `classify` now says so). E1's Laertes tulip was a certificate that never called the lookup.
   - **S15, tulip × C2SaferRust:** `ti_adx_start` casts the options **pointer** to `i32` instead of loading
     `options[0]` (`(options.offset(0) as i32 - 1) * 2`): wrong on every valid input, overflow panic when the
     address bits are large. One site — a text search suggested 19, a deterministic probe (options[0] = 5.0)
     showed 18 of them correct. E1's tulip × C2SaferRust defects (C6, S13) were driver-level.
3. **The two faithful translations are clean**: 0 confirmed of 243/244 sampled each; 0 divergences on
   1 891 / 1 811 replayed inputs.

## Gaps, deviations and limits

- The buffer-table capability was added for this library (user-approved after the planner reached 109/213
  without it); it is a C-side input-plan rule shared by all four translations, not a per-translator bridge,
  and is frozen after this run. Golden cases `ti_sma`, `ti_ad`, `ti_bbands`.
- Two chains died of the scratchpad byte quota before any campaign ran (per-harness rlibs in the shared
  cargo target, 18 MB debug-info binaries, coverage targets); the fixes (target pruning in both cargo
  profiles, stripped campaign binaries, per-harness coverage cleanup) are in `docs/rq4_runbook.md`. A third
  start was killed with a session restart after its builds; c2rust re-used them (`--reuse-bins`).
- Laertes was stopped by the preflight (`PREFLIGHT_REVIEW`, the first real one), reviewed, accepted with the
  reason in `tulip_laertes/preflight_accept.txt`, and re-run after the other cells; its three
  `ti_find_indicator` candidates were re-adjudicated after the zero-page rule (recorded in
  `laertes/RUN.md` §7 and `confirm_sample/summary.json`).
- The archive step failed once on `source/` subdirectories and was re-run; RUN.md numbers are from the
  archives.
- Single campaign per cell; the confirmation is the 200-per-channel sample.

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md with §7 prose, funnel.json, plans.json, analysis/,
divergences/, confirm_sample/, confirmed_inputs/ (Laertes, C2SaferRust), candidates_sample/,
candidates_manifest.json.gz, corpus.tar.gz, harness_exports.tar.gz, artifact_hashes.json, raw/, preflight
in funnel rows). Pairs: `benchmark/pairs/rq4/tulip_{c2rust,laertes,c2saferrust,crown}/` (+ Laertes'
`preflight_accept.txt`). Manifest entries: C11, S15.
