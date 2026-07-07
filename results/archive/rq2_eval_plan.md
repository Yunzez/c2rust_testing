# RQ2 evaluation plan v2 (UB-free gate — precision of the differential oracle)

Authoritative plan for RQ2. Mirrors `results/rq3_eval_plan.md`. v2 folds in the artifact-replay redesign
(the one substantive rigor fix) + claim-precision wording. Survives compact.

## RQ2 (tightened)
> Does the in-loop **UB-free gate** suppress UB-induced false-positive divergences that a UB-blind
> differential oracle would report as translation bugs, **while preserving detection of known UB-free bug
> controls** (observed sensitivity)?

Not "we find bugs" (RQ1). Not a global "no false negatives" claim — finite fuzzing cannot prove that. RQ2
is PRECISION: a divergence counts as a bug **only on a UB-free input** (the differential-oracle principle;
Csmith/EMI/Alive2 normative line). We prove suppression **per artifact** (deterministic replay), and prove
sensitivity is retained on K known UB-free bug controls.

## Mechanism (built) — two roles, do NOT conflate
- **In-loop gate (fast reject)** = `gen_diff_harness.py --ub-free` (commit 9593a1a): C oracle UBSan
  minimal-runtime; each `__ubsan_handle_*_minimal` sets a flag (no print, no location). Harness resets
  flag → runs C → if flag set, `return` (reject input) → else run Rust + compare. Fast, but carries NO UB
  kind / source location.
- **Post-hoc replay (evidence)** = a FULL-UBSan (`-fsanitize=…` WITHOUT minimal-runtime, with source
  location) build of the C oracle, run on a single artifact input to emit the UB kind + `file:line`.
  This is where the diagnostic evidence comes from — the in-loop flag alone cannot.
- **gate-OFF** = harness without `--ub-free` = a **UB-blind differential oracle**, matching the assumption
  of tools that do not filter C UB (unlike Fluorine/RustAssure, which do not define bug counting over
  UB-free inputs — put the named comparison in positioning, NOT "gate-OFF = Fluorine").
- Real libfuzzer-sys 0.4; coverage `inline-8bit-counters,pc-table,trace-cmp`.

## Definitions
- **boundary** = a matched (C fn, Rust fn) differentially-fuzzed unit (harness auto-generated).
- **artifact** = a concrete input libFuzzer saved when a divergence/crash fired.
- **divergence** = C vs Rust observably differ (return / panic-crash / out buffer).
- **UB-free input** = the C reference run trips no sanitizer (checked by replay).
- **real bug** = a divergence on a UB-free input (normative; = boundary-validity).

## UB classes (three-tier — be honest about what the in-loop gate can and cannot do)
1. **Recoverable UBSan UB** — signed overflow, shift-out-of-range, most bounds/null/unreachable. The
   in-loop flag gate SUPPRESSES these (main claim).
2. **Hard-trap UB** — integer divide-by-zero / INT_MIN÷-1 (may SIGFPE even with `-recover`), segfault-like
   faults. NOT guaranteed suppressed in-loop (no signal/longjmp gate in v1). Classified post-hoc as
   `GATE_MISS (hard-trap)`; we do NOT build signal gating for v1 — we report them honestly.
3. **Memory UB (ASan territory)** — OOB/heap. Out of the v1 mainline (in-loop ASan is state-unstable in a
   same-process fuzzer). Handled only post-hoc IF a memory-UB false positive actually appears; not built
   speculatively.
Main RQ2 claim scopes to class 1. Classes 2–3 are classified, not claimed-suppressed.

## Metrics — artifact-level (deterministic, not a two-fuzz-distribution comparison)
For every gate-OFF artifact `x`, replay `x` under (full-UBSan C) and (gate-ON harness) → classify:
- `UB_SUPPRESSED` — C trips UBSan on x AND gate-ON rejects x (the suppression case; attach UB kind+loc)
- `UB_FREE_DIVERGENCE` — C is UB-free on x AND divergence persists (a candidate REAL bug → investigate)
- `GATE_MISS` — C UB but gate-ON still diverges/crashes (hard-trap, or a gate bug — report honestly)
- `REPRO_FAIL` — x does not re-trigger (fuzzer nondeterminism / env) — reported, not hidden
Suppression is defined **per artifact by replay**, never by "gate-ON fuzz found nothing".

Aggregate:
- **FP suppression** = UB_SUPPRESSED / (all OFF artifacts that reproduce). 
- **Observed sensitivity** = on K known UB-free bug controls, fraction still flagged gate-ON (target K/K).
- Headline: "Across A1 harnessable c2rust boundaries, the UB-blind oracle produced N artifacts on M
  boundaries; replay classifies X% as UB_SUPPRESSED, Y% as UB_FREE_DIVERGENCE (candidate bugs), Z% hard-
  trap/repro-fail; on K UB-free controls the gate retained detection K/K."

## Structure
- **RQ2a — artifact-level suppression (precision core):** replay-classify every gate-OFF artifact.
- **RQ2b — gate-ON survivor search:** run gate-ON fuzz same budget; any survivor is UB-free by construction
  (unless hard-trap/gate-miss) → classify. Finds candidate real bugs on the "faithful" corpus.
- **RQ2c — sensitivity controls:** known UB-free bugs (injected `g3_g2_bug`; `u8encode_` if bridge ready);
  gate-ON MUST still find them.

## Table A — all harnessable faithful c2rust boundaries (denominator reported; anti-cherry-pick)
| boundaries (A1) | OFF artifacts | reproduced | UB_SUPPRESSED | UB_FREE_DIVERGENCE | GATE_MISS | REPRO_FAIL |
|--:|--:|--:|--:|--:|--:|--:|
| N_all | … | … | … | … | … | … |
Report A1 = ALL harnessable boundaries (the denominator), and A2 = the subset with OFF artifacts. The
suppression rate is over A2 artifacts, but A1 must be shown so it is not cherry-picked. Seed corpus =
`ub_free_rescan_v1` (134 boundaries, statically all UB-backed) run DYNAMICALLY now.

## Table B — controls (confusion matrix; mark STATUS honestly)
| case | bug kind | status | OFF | ON | class | outcome |
|---|---|---|--:|--:|---|---|
| clip | — | ready ✓ | clean | clean | — | TN |
| sign_extend | shift≥width (C UB) | ready ✓ | DIVERGE(bits=0) | clean | UB_SUPPRESSED | FP suppressed ✓ |
| u8encode_ | UB-free (C returns −1) | **pending-bridge** | DIVERGE | DIVERGE(expected) | UB_FREE_DIVERGENCE | sensitivity (if runnable) |
| aptx_bin_search | ? | ready ✓ | … | … | … | (to run) |
| g3_g2_bug | injected, UB-free | controlled-injected | DIVERGE | DIVERGE | UB_FREE_DIVERGENCE | sensitivity ✓ |
`ready` = staged + runs; `pending-bridge` = needs a harness bridge (u8encode_: bare out-buf + elem-split)
before it counts as evidence; `controlled-injected` = we inserted a known UB-free bug.

## Table C — UB taxonomy of suppressed artifacts
| UB class | tier | count | example | in-loop suppressed? | evidence source |
|---|---|--:|---|---|---|
| shift ≥ width | 1 recoverable | … | sign_extend bits=0 | yes | full-UBSan replay |
| signed overflow | 1 recoverable | … | … | yes | full-UBSan replay |
| divide-by-zero | 2 hard-trap | … | … | not guaranteed | replay (SIGFPE) |
| OOB/bounds | 3 memory | … | … | post-hoc only | ASan replay (if needed) |

## Hard artifacts / rebuttals
1. "The gate just hides real bugs." → structural invariant (gate rejects only inputs where UBSan flagged
   the **C** run → cannot hide a UB-free divergence) + RQ2c controls flagged K/K + per-suppressed UB kind.
2. "ON just didn't search to that input." → we do NOT infer suppression from ON finding nothing; every OFF
   artifact is REPLAYED under gate-ON and full-UBSan (deterministic per-artifact classification).
3. "How do you know suppressed = genuinely UB?" → the full-UBSan replay diagnostic (UB kind + C file:line).
4. "faithful c2rust isn't truly bug-free." → we do NOT assume it; any UB_FREE_DIVERGENCE survivor is a
   CANDIDATE real bug and investigated/reported (RQ2b). Prior static rescan found 0 survivors — consistent.
5. Definition of "real bug" = UB-free divergence — [[ub-differential-oracle-principle]] (normative).

## Data
1. **Faithful c2rust corpus** (Table A / RQ2a-b): scalar/buffer boundaries the auto-bridge harnesses, from
   the CROWN c2rust corpus (`corpus_inventory_v1.md`); seed = `ub_free_rescan_v1` 134-boundary set.
2. **Controls** (Table B / RQ2c): `tools/headtohead/libopenaptx/` (clip, sign_extend, aptx_bin_search;
   u8encode_ via `setup_libopenaptx.sh`, pending its bridge) + injected `g3_g2_bug`.

## DoD
1. Runner `scripts/eval_rq2_ubgate.py`: per boundary — fuzz gate-OFF (collect artifacts), **replay each
   artifact under gate-ON and full-UBSan**, classify (UB_SUPPRESSED / UB_FREE_DIVERGENCE / GATE_MISS /
   REPRO_FAIL) with UB kind+loc; also run gate-ON fuzz (RQ2b survivor search). Emit
   `{boundaries, artifacts:[{class, ub_kind, loc}], suppression_rate, sensitivity, by_class}` JSON +
   `results/rq2_ubgate_v1.md` (Tables A/B/C).
2. Every UB_SUPPRESSED artifact carries its full-UBSan diagnostic.
3. RQ2c: all known UB-free controls flagged gate-ON (observed sensitivity K/K).

## Execution order
1. Build the **replay classifier** around the 2 ready controls (sign_extend → UB_SUPPRESSED; clip → TN):
   fuzz OFF → replay artifact under full-UBSan + gate-ON → class. This is the RQ2a skeleton + evidence path.
2. RQ2c sensitivity: injected `g3_g2_bug` (+ u8encode_ if its bridge is ready) → gate-ON still flags.
3. Scale RQ2a/b across the faithful c2rust boundary set → Table A + C.
4. (only if a memory-UB FP appears) post-hoc ASan classification.

## Open decisions (defaults; flag to change)
- Table A denominator = the `ub_free_rescan` 134-boundary set, run dynamically; grow if bigger N wanted.
- Sanitizer: UBSan only in v1 mainline; ASan post-hoc, on demand.
- Fuzz budget: 25s/boundary (≈500k exec/s proven); longer for the headline.
- Rust build: debug asserts ON (overflow-panic on a C-UB input = correctly gated; on a UB-free input = a
  candidate bug).
