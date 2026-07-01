# RQ2 evaluation plan (UB-free gate — precision of the differential oracle)

Authoritative plan for RQ2. Mirrors `results/rq3_eval_plan.md`. Survives compact.

## RQ2 (tightened)
> Does the in-loop **UB-free gate** suppress the UB-induced false-positive divergences that a naive
> differential oracle (Fluorine / RustAssure — neither filters UB) would report as translation bugs,
> **without** suppressing any real bug (no false negatives)?

Not "we find bugs" (that's RQ1). RQ2 is PRECISION: a divergence counts as a bug **only on a UB-free
input** (the differential-oracle principle; Csmith/EMI/Alive2 normative line). The gate reclassifies
"divergence caused by C-side UB" from bug → non-bug. Measured on **faithful c2rust** (semantics-preserving
by construction → ~0 real bugs, so every naive divergence there is a UB false positive) plus known-bug
controls to prove specificity.

## Mechanism (already built — `gen_diff_harness.py --ub-free`, commit 9593a1a)
- C oracle compiled with UBSan minimal-runtime: `signed-integer-overflow,shift,integer-divide-by-zero,
  bounds,null,unreachable` + `-fsanitize-recover=all` + `-fsanitize-minimal-runtime`; each
  `__ubsan_handle_*_minimal` sets a flag instead of aborting.
- Harness loop: reset flag → run C → **if C tripped UB, `return` (reject input, no comparison)** → else
  run Rust and compare. So a divergence is reported ONLY on a UB-free input.
- **gate-OFF** = the same harness without `--ub-free` = count EVERY divergence = the naive oracle =
  the competitors' stance (baseline).
- **gate-ON** = `--ub-free` = count only UB-free divergences = ours.
- Real libfuzzer-sys 0.4 (honors `-max_total_time`); coverage `inline-8bit-counters,pc-table,trace-cmp`.

## Definitions
- **boundary** = a matched (C fn, Rust fn) differentially-fuzzed unit (harness auto-generated).
- **divergence** = fuzzing finds an input where C vs Rust observably differ (return / panic-crash / out buffer).
- **UB-free input** = the C reference run trips no sanitizer.
- **real bug** = a divergence on a UB-free input (the normative definition; = boundary-validity).
- Confusion matrix per boundary: TP (real bug, flagged), TN (correct, clean), FP (UB divergence — gate's
  job to remove), FN (real bug the gate wrongly hid — MUST be 0).

## Metrics
- Per boundary: `diverge_off` (naive), `diverge_on` (gated), and if suppressed the **UBSan class**.
- **FP suppression rate** on faithful c2rust = (Σ diverge_off − Σ diverge_on) / Σ diverge_off. Target: gate
  drives naive FPs → ~0; any gate-ON survivor is EITHER a genuine c2rust bug (a finding) or a gate miss.
- **Sensitivity retention** on known-bug controls = fraction of real bugs still flagged gate-ON (must be
  100% → FN=0).
- Headline: "the naive oracle reports N divergences on faithful c2rust; M/N (X%) are UB-induced false
  positives our gate suppresses; all K real bugs survive → 0 false negatives."

## Table A — FP suppression on faithful c2rust (the main result)
| boundary | fuzz budget | diverge OFF | diverge ON | suppressed | UB class |
|---|--|--:|--:|--:|---|
| … per harnessed c2rust function … | 25s | … | … | … | signed-overflow / shift / … |
| **TOTAL** | | N | ~0 | M (X%) | |
Prior static re-label (`ub_free_rescan_v1.md`): on this corpus ALL recorded divergences were UB-backed →
gate-ON should be ~0. RQ2 = the DYNAMIC (in-loop UB-free fuzzing) version of that, not a static relabel.

## Table B — specificity controls (worked confusion matrix; the "gate doesn't hide bugs" proof)
| boundary | source | OFF | ON | class | outcome |
|---|---|--:|--:|---|---|
| clip | libopenaptx GPT-4o | clean | clean | — | **TN** (correct translation) |
| sign_extend | libopenaptx GPT-4o | DIVERGE (bits=0) | clean | shift ≥ width (C UB) | **FP suppressed** ✓ |
| u8encode_ | libopenaptx GPT-4o | DIVERGE | **DIVERGE** | input UB-free in C (returns −1) | **real bug KEPT** ✓ (FN=0) |
| aptx_bin_search | libopenaptx (2 variants) | … | … | … | (to run) |
| injected bug (g3_g2_bug / safe_ratio) | controlled | DIVERGE | **DIVERGE** | UB-free | **TP** ✓ |
The invariant that makes FN=0 structural: the gate ONLY rejects inputs where UBSan flagged the **C** run;
it CANNOT hide a UB-free divergence. u8encode_ + the injected bug are the live proof.

## Table C — taxonomy of suppressed divergences (what the naive oracle gets wrong)
| UB class (UBSan) | example | why the naive oracle mis-reports it |
|---|---|---|
| shift ≥ width | sign_extend bits=0 → `val<<32` | Rust panics / wraps differently; input is out-of-contract |
| signed overflow | … | C UB; Rust debug-assert panics |
| integer divide-by-zero | … | … |
| OOB / bounds | … | needs ASan (see ablation) |
Each row = a false-positive class a UB-blind oracle (Fluorine/RustAssure) would file as a bug.

## Baseline & ablations
- **Baseline = gate-OFF** (the naive oracle = competitors ignore UB). This IS the comparison.
- **Sanitizer scope**: UBSan-only (default) vs **UBSan+ASan** column — ASan catches OOB/heap UB that
  UBSan misses; report whether it suppresses additional FPs (some memory-corruption divergences).
- **Rust build mode**: debug (overflow asserts ON — matches c2rust intent, surfaces overflow) is the
  default; an overflow panic on a UB (C-overflowing) input is correctly gated, on a UB-free input is a
  real finding. Document; optionally a release column.

## Hard artifacts / rebuttals
1. "The gate just hides bugs." → Table B (u8encode_ + injected survive) + the structural invariant (gate
   rejects only UBSan-flagged **C** inputs). **Log the UBSan diagnostic (UB kind + C file:line) for every
   suppressed case** as evidence the suppressed input was genuinely UB. (hard artifact)
2. "How do you know suppressed = genuinely UB?" → the attached UBSan report is the proof.
3. "c2rust faithful isn't truly bug-free." → any gate-ON survivor on faithful c2rust is a REAL c2rust bug
   (report it — bonus finding) or a gate miss (investigate). `ub_free_rescan_v1` found 0 survivors — consistent.
4. Definition of "real bug" = UB-free divergence — [[ub-differential-oracle-principle]], normative
   (Csmith/EMI/Alive2), not our invention.

## Data
1. **Faithful c2rust corpus** (Table A): scalar/buffer function boundaries the auto-bridge harnesses,
   drawn from the CROWN c2rust corpus (`results/corpus_inventory_v1.md`, ~19 aligned programs) — pick the
   functions that (a) harness cleanly and (b) exhibit UB divergences. Prior `ub_free_rescan` corpus is the
   seed (134 boundaries relabeled; all UB-backed).
2. **Controls** (Table B): libopenaptx head-to-head pairs already staged under
   `tools/headtohead/libopenaptx/` (clip, sign_extend, aptx_bin_search; u8encode_ via
   `setup_libopenaptx.sh`) + the injected bug (g3_g2_bug / safe_ratio).

## DoD
1. Runner `scripts/eval_rq2_ubgate.py`: per boundary, generate gate-OFF + gate-ON harnesses, fuzz both for
   a fixed budget, record {diverge_off, diverge_on, ub_class, ubsan_report}, emit
   `{boundaries:[…], suppression_rate, sensitivity, by_class}` JSON + `results/rq2_ubgate_v1.md` (Tables A/B/C).
2. Every suppressed divergence carries its UBSan diagnostic.
3. Sensitivity control passes: all known real bugs flagged gate-ON (FN=0).

## Execution order
1. Wire the runner around the two ready controls (sign_extend, u8encode_) → Table B skeleton + FN=0 check.
2. Add clip + aptx_bin_search → fill Table B.
3. Harness a batch of faithful c2rust scalar/buffer functions → Table A (FP suppression) + Table C taxonomy.
4. ASan-scope ablation column.

## Open decisions (defaults chosen; flag if you disagree)
- Corpus size for Table A: start with the `ub_free_rescan` boundary set (already known UB-backed) run
  DYNAMICALLY; expand if a bigger N is wanted. (rework-safe: mechanism/metric frozen, only N grows.)
- Sanitizer: UBSan default + ASan ablation column.
- Fuzz budget: 25s/boundary (demo-proven throughput ~500k exec/s); bump for the headline run.
- Build mode: debug asserts on (document the overflow-panic-on-UB-input = gated).
