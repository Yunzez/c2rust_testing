# RQ4 — coverage beyond shipped tests: cJSON

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md);
planner: the producer bridge, cJSON generalisation ([`docs/producer_bridge_pilot.md`](../../../docs/producer_bridge_pilot.md) §6a).
Status 2026-09-05: **two cells complete, c2rust and PtrTrans** (3 600 s each, one campaign, one corpus). The tests side is
TEST-UNAVAILABLE for every cJSON translation (only `cJSON.c` was translated; the C driver `tests/main.c` is used as
producer-selection evidence, never linked). PtrTrans's cJSON is **construction unsupported** under the frozen bridge
(producer returns `Option<&mut cJSON>`, `cJSON_Delete` undefined): its cell reaches two zero-argument functions (`cJSON_Version`, and `cJSON_GetErrorPtr` in its fresh-process state).

## Cell table

| tool | tests side | planned / built of 58 | corpus | fn ours | reg ours | divergences on replay | confirmed |
|---|---|---:|---:|---:|---:|---:|---|
| **c2rust** | TEST-UNAVAILABLE (denominator) | 39 / 39 (12 C `static`) | 8 796 | 49/59 (0.831) | 1 816/2 237 (**0.812**) | **0** / 8 796 (372 `ub-gated`) | **0** of 3 844 sampled |
| **PtrTrans** | TEST-UNAVAILABLE (denominator) | 15 / **2** of 113 (64 `cJSON*` construction unsupported; 10 need `cJSON_Delete`, 3 reshaped) | 2 | 2/121 (0.017) | 9/2 125 (**0.004**) | 0 / 2 | nothing to confirm |

Producer-bridge ablation, same campaign (`ablation_producer_bridge.json`): the 21 harnesses without a produced
object reach 29/59 functions (0.492) and 791/2 237 regions (**0.354**); with the 18 `cJSON_Parse`-fed boundaries,
0.831 / 0.812.

## What this library says

1. **The bridge generalises**: producer = `cJSON_Parse` (string input, ranked by reachability), destructor =
   `cJSON_Delete` (found through the `cJSON_free = free` alias), object state compared through the existing
   comparator plugin before and after the target. Regions 0.354 → 0.812.
2. **The negative control holds** once the producer is inside the UB gate: 0 divergences on 8 796 inputs,
   0 confirmed of 3 844 sampled. Before that fix the same corpus showed 149 "divergences" that were
   `cJSON_Parse`'s own `(int)double` UB — recorded in `c2rust/RUN.md` §7 as the deviation it was.
3. **The reference is the noisy party**: old cJSON's `parse_string` `\u` one-byte heap overflow (confirmed
   C-only under ASan) makes every produced boundary collect thousands of `ub_associated` artifacts. The
   generator does not hide a reference bug behind the producer; the archive keeps a sha256 manifest of all
   99 999 and the first 500 per channel.
4. **What the bridge refuses is listed, not guessed**: 9 boundaries take two produced objects (ownership
   transfer), 8 take `printbuffer*`, 1 `cJSON_Hooks*`, 1 an out-pointer.

## Gaps and limits

- No paired cell: no cJSON translation carries a transpiled suite, so there is no tests-side coverage to
  compare against — the validator's number stands alone, against the link-dead-code universe.
- PtrTrans is empty by construction, and the matrix says why (`ptrtrans/RUN.md` §7): the frozen bridge has no
  `Option<&mut T>` shape and the crate has no destructor; the decision not to build a one-translator bridge is
  recorded in the pilot doc. c2rust's faithful translation gives the control, not a defect hunt. The catalogued
  PtrTrans cJSON defects (S7–S9) came from the earlier hand `cJSON_Parse` campaign (`campaign_cJSON_Parse/`).
- The PtrTrans universe was recomputed once (bin-route denominator collapsed by cross-crate inlining; now from
  the rlib's objects — `scripts/rq4/rlib_universe.py`; all earlier universes verified identical both ways).
- The cell died once after its campaign (scratchpad file-count quota) and was finished from the intact
  campaign data; its discovery binaries were rebuilt once (producer gate). Both are in `c2rust/RUN.md` §7.
- Single campaign; no repeats.

## Files

`tests_side_results.json`, `cells.json`, `ablation_producer_bridge.json`, `c2rust/` (RUN.md, funnel.json,
plans.json, analysis/, divergences/, confirm_sample/, harnesses/, candidates_sample/, candidates_manifest.json.gz,
corpus.tar.gz, harness_exports.tar.gz, artifact_hashes.json, raw/denominator.json), `campaign_cJSON_Parse/`
and `harness_plans/` (the earlier, pre-bridge cJSON work). Pairs: `benchmark/pairs/rq4/cjson_{c2rust,ptrtrans}/`.
