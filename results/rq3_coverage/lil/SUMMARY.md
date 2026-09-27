# RQ4 — coverage beyond shipped tests: lil

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md);
planner: the producer bridge (`lil_new()` → target → `lil_free`, `lil_alloc_double` / `lil_alloc_list` for value and
list parameters; [`docs/producer_bridge_pilot.md`](../../../docs/producer_bridge_pilot.md)).
Status 2026-09-06: **four cells complete**, 3 600 s each, one campaign and one corpus per cell. lil ships no test
suite (a CLI and demo scripts), so every cell is TEST-UNAVAILABLE and the universe is the link-dead-code denominator
(`tests_side_results.json`). C source: the 3 518-line `lil.c` every crate in the Laertes benchmark set came from.

## Cell table

| tool | tests side | planned / built of 145 | corpus | fn ours | reg ours | replay: terminations / ub-gated | confirmed (sample) |
|---|---|---:|---:|---:|---:|---:|---|
| **c2rust** | TEST-UNAVAILABLE | 51 / 50 (3 accepted crash-all) | 4 761 | 143/151 (**0.947**) | 4 999/5 730 (**0.872**) | 7 / 5 | **0** of 816 (negative control) |
| **Laertes** | TEST-UNAVAILABLE | 51 / 51 (3 accepted crash-all) | 4 907 | 144/183 (0.787)¹ | 5 028/6 143 (0.818) | 7 / 7 | **0** of 816 (negative control) |
| **C2SaferRust** | TEST-UNAVAILABLE | 47 / 47 (**23 crash-all**) | 340 | 25/154 (0.162) | 362/5 751 (0.063) | 27 / 0 | **57 `confirmed_termination`, one site → C9**; 18 boundaries downstream-blocked |
| **CROWN** | TEST-UNAVAILABLE | 42 / 42 (3 accepted crash-all) | 1 859 | 127/134 (**0.948**) | 5 294/6 409 (**0.826**) | 3 / 1 | **67 `confirmed_termination` on `lil_parse`, one site → C10** |

¹ Laertes' universe includes its own runtime (`laertes_init_*`, `__laertes_array`), unreachable from any boundary;
compare only-ours counts (144 / 5 028), not the fraction.

Refused by the planner, identically in every cell: 58 `fnc_*` builtins (`lil_value_t* argv`, a table of produced
objects), 15 `expreval_t*` (the expression evaluator's state), 8 boundaries taking two produced objects; the rest of
the 145 are private helpers the plan reaches or C `static`s exposed per entry.

## What this library says

1. **Two new defects, both by the plan pipeline, both on translations the earlier evidence had not charged.**
   - **C9, lil × C2SaferRust:** `register_stdcmds` hands Rust string literals to C without a NUL terminator; the first
     `hm_get` runs `strlen` past the literal, so `lil_new()` never completes. The reach census had recorded this
     translation as CRASH-ALL without a cause; this is the cause, at line level, confirmed with C in contract on
     every sampled input. One defect, 19 boundaries showing it, 18 of them only because they take the object the
     producer cannot build.
   - **C10, lil × CROWN:** `lil_subst_to_list` lost `if (!words) words = lil_alloc_list();` — NULL where C returns an
     empty list — and `fnc_enveval` dereferences it. CROWN's lil was an **E1 certificate** (111 043 records, 0 diffs);
     the certifying corpus never ran `enveval` on a malformed list. The generated `lil_parse` harness reached it in the
     first hour.
2. **The two faithful translations are clean.** c2rust and Laertes: 0 confirmed of 816 sampled each; everything is
   `ub_associated` (lil's own out-of-contract reads on the C side), `instrument_only`, `inconclusive` or not
   reproducible. Coverage 0.87–0.83 of regions from the producer-bridged campaign alone.
3. **Crash-all is a first-class outcome now.** 23 of C2SaferRust's boundaries never grew a corpus; their coverage
   "failed" because a crashing process writes no profile, and the funnel says so (`campaign_status.crash_all`) instead
   of "export failed". The three parser internals (`ateol`, `get_dollarpart`, `next_word`) are crash-all in every
   translation for a reason the harness cannot remove — they need a parse in progress — and are recorded in the
   pairs' `preflight_accept.txt`, never promoted.

## Gaps, deviations and limits

- **A harness bug voided `lil_parse` in the first two cells** (a length-0 buffer was a dangling pointer, and lil
  calls `strlen` on it). It was re-fuzzed alone for 3 600 s after the fix and merged (`<tool>/deviations.json`); it
  had the machine to itself, unlike the cell's concurrent campaign. This is what made the preflight step mandatory
  (`../PROTOCOL.md` §3 amendment).
- c2rust: `lil_list_size` unbuilt (generator `size_t`-return bug, fixed after the start); coverage phase finished by
  `finish_cell.py` after a decode error. CROWN: started twice (15 `static` entries failed to re-export under CROWN's
  `src` namespace; fixed, full re-run with preflight). Generator hashes are recorded from the CROWN cell on; the
  earlier cells' harnesses predate that.
- No paired cell: lil has no suite, so there is no tests-side coverage to compare against.
- Single campaign per cell; the confirmation is a 200-per-channel sample, not the full adjudication (the archives keep
  every artifact's hash and the first 500 per channel).

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md with §7 prose, funnel.json, plans.json, analysis/,
divergences/, confirm_sample/, confirmed_inputs/ (C2SaferRust, CROWN), candidates_sample/, candidates_manifest.json.gz,
corpus.tar.gz, harness_exports.tar.gz, artifact_hashes.json, raw/denominator.json, deviations.json where applicable).
Pairs: `benchmark/pairs/rq4/lil_{c2rust,laertes,c2saferrust,crown}/` (+ `preflight_accept.txt`).
Manifest entries: `results/rq4_effectiveness/defect_manifest.md` C9, C10.
