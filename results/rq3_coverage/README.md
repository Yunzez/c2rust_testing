# RQ4 — Coverage Beyond Shipped Tests

**Status 2026-09-04: pre-registered and under measurement. One paired cell exists but is
superseded; the current campaign is in flight.**

| | |
|---|---|
| runbook | [`docs/rq4_runbook.md`](../../docs/rq4_runbook.md) — the operational lessons (quota, serial cells, detached launches, noise classes, generator traps); read before running another library |
| protocol | [`PROTOCOL.md`](PROTOCOL.md) — pre-registered, budget amended 300 s → 3 600 s on 2026-09-04 *before* any cell under it produced a number |
| paired cells complete | **1** — bzip2 × c2rust, outcome PAIRED ([`bzip2/c2rust_handschema_superseded/RUN.md`](bzip2/c2rust_handschema_superseded/RUN.md)): fn 0.773 tests / 0.682 ours, region 0.797 / 0.798, only-ours 409 of 8 789 regions |
| …but produced by | the **retired hand-schema pipeline** (14 eligible → 10 harnesses, 35 hand-written constants). The plan generator reaches **19 of 64** boundaries on the same library, so that cell describes a pipeline we no longer have and its numbers are **not** the ones to quote. |
| **complete under the plan pipeline** | **bzip2 × {c2rust, Laertes, CROWN, C2SaferRust}**, 3 600 s each, serial — [`bzip2/SUMMARY.md`](bzip2/SUMMARY.md). c2rust: region 0.807 ours / 0.797 tests, only-ours 481, 0 divergences (negative control); Laertes 299 / CROWN 242 / C2SaferRust 3 replay divergences; S3, S10, S11, S14 re-found; **new defect C8** (Laertes zeroed `incs`) |
| **complete under the producer-bridge pilot** | **genann × {c2rust, Laertes, CROWN, C2SaferRust, SACTOR}** — [`genann/SUMMARY.md`](genann/SUMMARY.md). Three translations pass the suite; validator 0.806 / 0.796 / 0.814 vs suite 0.895 / 0.882 / 0.895 (gap = `FILE*` I/O); four negative controls clean; **SACTOR #32 re-found on three boundaries, 51/51 confirmed**; bridge ablation 0.17–0.23 → 0.71–0.82 of regions |
| **complete: cJSON × {c2rust, PtrTrans}** | [`cjson/SUMMARY.md`](cjson/SUMMARY.md) — producer bridge generalised (`cJSON_Parse` → target → `cJSON_Delete`, plugin object state); region ours **0.812** vs 0.354 without the bridge; negative control 0 / 8 796 replay divergences, 0 confirmed; PtrTrans **construction unsupported** under the frozen bridge (`Option<&mut T>` producer, no `cJSON_Delete`): 2 of 15 direct boundaries build, region 0.004 — recorded, not extended |
| **complete: lil × {c2rust, Laertes, C2SaferRust, CROWN}** | [`lil/SUMMARY.md`](lil/SUMMARY.md) — no suite (TEST-UNAVAILABLE ×4); c2rust region **0.872** / Laertes 0.818 / CROWN 0.826, both faithful translations 0 confirmed; **two new defects**: C9 (C2SaferRust: C-string literals without NUL — the root of its CRASH-ALL; 23 boundaries never run) and C10 (CROWN: dropped NULL→empty-list fallback in `lil_subst_to_list`, on an E1-**certified** cell) |
| in flight | tulip × 4 (all 213 functions plannable after the buffer-table capability; tests side c2rust/CROWN pass), then PtrTrans × cJSON again with the plugin-compatibility degradation |

A first attempt at these four cells ran them **concurrently** and died in the build phase at
`built 6/19` with `EDQUOT` — four simultaneous cargo target dirs exhausted the scratchpad's quota.
No data was produced, so no result was affected; the re-run is serial with a free-space
precondition. Recorded because the failure mode (counting cores and RAM, but not the filesystem the
builds actually write to) is a property of the harness, not of any translation.

> **RQ4.** *How much code does our differential validator exercise beyond the test suites shipped with
> existing translators and translated libraries?* See `results/EVALUATION_PLAN.md`.

The directory retains its historical `rq3_coverage` name so existing evidence
links do not break.

## What the experiment requires

For the same translated artifact, instrumented with one common coverage mechanism and run under a
reported common budget:

1. run the **shipped acceptance tests** (the suite that was used to accept that translation);
2. run **our differential fuzzing campaign**;
3. report function, region, and branch coverage for each; and
4. partition the reached code into three sets: shipped-only, both, validator-only.

The claim is about *exploration*, not correctness, and the comparison is against the tests that accepted
the shipped translations — never against a fuzzer supplied by another system.

## What exists today and why it does not count

[`../rq4_effectiveness/reach_census.md`](../rq4_effectiveness/reach_census.md) (33/33 cells) is a
**one-sided** reach measurement: the "their tests" side is 0 by construction, so it cannot answer a
paired question. It also carries two limits that any reuse must inherit — 9 of its 33 cells have median
0, and its execution budgets span 4,000 to 2,000,000 runs (500×), so cross-library magnitudes are
invalid. It is retained under RQ4 as an interpretation limit on defects and bounded no-difference
results, which is how `evaluation.tex` cites it.

## Open questions — settled in `PROTOCOL.md` before the first cell ran

- **"Shipped tests" is undefined for tools that ship no suite.** Settled as one rule applied per cell
  (§2): the tests side runs iff the project ships a test or acceptance target **and** that driver is
  present in the translation; otherwise the cell records `TEST-UNAVAILABLE` with its reason and still
  reports the validator side. We do not transpile a test driver ourselves — that would be our
  construction rather than the evidence that accepted the translation. The measured fact that
  **0 of 55 translated crates ship any test** is reported as motivation, never as the baseline.
- **Common budget.** Settled in §3 and pre-registered: 3 600 s wall, `-seed=42`, fixed stopping rule,
  no extension for a cell that looks interesting. Seeding is part of the measurement, not a
  convenience — the shipped-sample table in §3 shows a boundary going from 438 corpus inputs to 39
  when the samples are dropped.
- **Common coverage mechanism.** Settled in §5: one toolchain (`nightly-2025-09-01`) with its own
  `llvm-profdata`/`llvm-cov`, never the system LLVM; both sides compiled with the same flags so the
  partition is not an artifact of two instrumentations. Identity is `(file, start line)` for
  functions and `(file, l1, c1, l2, c2)` for regions — **never symbol names**, whose
  crate-disambiguator hashes differ per build.
