# Attribution ablation — mutation recall v1 (negative control)

*Legacy label: this document was “RQ1b” under the retired E1/E2/E3 numbering (see `results/INDEX.md`).*

The recall half of the comparator claim, closing the loop with the UB-gate study: that study shows the
UB-attributed oracle reports **no false** translation bugs on faithful c2rust output
(0 vs 13 for a UB-blind oracle, `results/ablations/attribution/ubgate_v1.md`); this experiment shows the
same pipeline **misses no real** UB-free semantic bugs when they exist. Plan:
`results/archive/mutation_recall_eval_plan.md`.

## Question

Do injected UB-free semantic translation bugs survive the UB gate and get detected
by the differential oracle — with per-bug evidence that the triggering input is
UB-free on the C side?

## Setup

- **Base translations:** faithful c2rust output (`benchmark/pairs/*`), all drawn from
  programs of the RQ2 48-boundary frame. On the un-mutated base these boundaries are
  TN (no divergence), so any post-injection divergence is attributable purely to the
  mutation. Using c2rust (name-preserving) also decouples recall from the matcher
  (matcher = RQ3): pairs are matched by name; the experiment isolates the ORACLE's recall.
- **Mutation unit:** `(program, entry boundary, operator)`; the Rust translation body is
  patched (unique textual find→replace, must still compile — enforced by a dry-run
  `rustc --emit=metadata` check); the C original is the oracle.
- **Operators (4 classes, UB-free only):** operator replacement (`+↔^`, `%↔/`, `<<↔>>`,
  `^↔|`, `wrapping_sub↔wrapping_add`, drop `!`), constant perturbation (shift masks,
  guard return constants, saturation constants, lane masks `0xff→0x7f`), off-by-one
  (`<=↔<` loop bound, mask `31→30`, `return n → n+1`), guard weakening (drop or
  neutralize a guard **on the Rust side only**; the C reference keeps its guard).
  NO pointer/deref/memory mutations (those introduce UB and would be — correctly —
  gated, muddying recall).
- **Detection pipeline:** `scripts/eval_mutation_recall.py` reuses the RQ2 runner
  (`eval_rq2_ubgate.run_boundary`) wholesale: fuzz the mutated pair gate-OFF, replay
  every artifact gate-ON, then a standalone full-UBSan C build re-executes the decoded
  triggering input. **DETECTED_UB_FREE == the RQ2 classifier's `UB_FREE_DIVERGENCE`**:
  gate-ON still diverges AND the C execution on that input is sanitizer-clean
  (`is_ub:false` recorded per detection).
- **Budget / seeds:** 25 s libFuzzer campaign per mutant (build time excluded from the
  timed window). The main campaign (25 mutants) is a **single-run fixed-budget**
  campaign; the 3-mutant addendum ran with fixed `-seed=1`; a flakiness sanity check
  re-ran 5 representative mutants under 3 further fixed seeds (below). All valid
  mutants were detected within the budget in every run.
- **Independent mutation-validity oracle** (`scripts/mut_equiv_oracle.py`): the recall
  denominator is decided WITHOUT our fuzzer. For each mutant, the C entry (all exported
  symbols renamed `c_*`) and the mutated Rust staticlib are linked into one binary that
  sweeps a fixed structured grid of interesting inputs (boundary values, masks, spread
  points; op-selector args get every dispatch arm; 18–2268 points per mutant, release
  codegen so arithmetic wraps as in the fuzz build). All base entries are total and
  UB-free over their whole domain (guarded/masked/wrapping by construction), so every
  sweep point is a legal comparison. A mutant is **valid (non-equivalent)** iff some
  UB-free input makes C and mutated-Rust differ (a crash/trap on a legal input also
  counts); **equivalent** iff zero divergences over the grid. Equivalent mutants are
  excluded from the denominator — they are not missed bugs.

## Main result

Three-layer denominator (rows: `results/ablations/attribution/mut_rows/m2.json` + `m2_add.json`; oracle:
`m2_oracle.json` + `m2_add_oracle.json`):

| | count |
|---|---|
| injected mutants | **28** |
| — equivalent (independent oracle) | 1 |
| — invalid / build-fail / UB-only | 0 |
| **valid non-equivalent UB-free mutants** | **27** |
| **detected (`UB_FREE_DIVERGENCE`, with C-UBSan-clean input evidence)** | **27** |
| **recall** | **27/27 = 100%** |

Per operator:

| operator | injected | equivalent | valid | detected | recall |
|---|---|---|---|---|---|
| operator replacement | 14 | 0 | 14 | 14 | 14/14 |
| constant perturbation | 7 | 0 | 7 | 7 | 7/7 |
| guard weakening | 3 | 0 | 3 | 3 | 3/3 |
| off-by-one | 4 | 1 | 3 | 3 | 3/3 |
| **total** | **28** | **1** | **27** | **27** | **100%** |

6 programs (div_mod, negate_abs, shift_ops, sub_overflow, intmath, bitutils); every
detection carries the decoded triggering input plus a standalone full-UBSan C replay
showing that input is UB-free (`is_ub:false`, empty `ub_reports`, clean exit).

Time-to-detect (the 18 timed runs: addendum + flakiness; build excluded):
**median 0.21 s, max 1.13 s** against the 25 s budget. The original 25-mutant campaign
predates the timing instrumentation; for it we report only that all detections
occurred within the single 25 s budget.

Oracle divergence density over valid mutants spans **0.02–1.0** (median 0.54): the set
is not composed of only easy high-density bugs; the low-density tail (0.02–0.09:
`im_isqrt_const`, `su_sat_min_const`, `su_sat_max_const`, `im_isqrt_loop`) and one
single-input case (`dm_div_intmin_drop`) were all detected.

## Per-mutant table

| id | program | entry | operator | oracle density | detected | evidence input (C UB-free) |
|---|---|---|---|---|---|---|
| dm_final_or | div_mod | div_mod_safe | op-repl (`+→\|`) | .213 | ✓ | 1027423498, 10 |
| dm_mod_zero_ret | div_mod | div_mod_safe | guard-weak (b==0: `a→0`) | .053 | ✓ | 10, 0 |
| dm_div_zero_ret | div_mod | div_mod_safe | const (b==0: `0→1`) | .056 | ✓ | 0, 0 |
| dm_mod_op | div_mod | div_mod_safe | op-repl (`%→/`) | .870 | ✓ | 168430090, 2570 |
| dm_div_op | div_mod | div_mod_safe | op-repl (`/→%`) | .870 | ✓ | 1970631978, 122 |
| dm_div_intmin_drop | div_mod | div_mod_safe | guard-weak (drop INT_MIN/−1) | single-input | ✓ | −2147483648, −1 |
| na_final_sub | negate_abs | negate_abs_safe | op-repl (add→sub) | .889 | ✓ | 10 |
| na_negu_const | negate_abs | negate_abs_safe | const (`+1→+2`) | 1.0 | ✓ | 0 |
| na_negu_op | negate_abs | negate_abs_safe | op-repl (drop `!`) | 1.0 | ✓ | 0 |
| na_abs_guard | negate_abs | negate_abs_safe | op-repl (`<→>`) | .889 | ✓ | 10 |
| so_mask15 | shift_ops | shift_ops_safe | const (mask `31→15`) | .398 | ✓ | 4294967082, 11007 |
| so_i32_mask30 | shift_ops | shift_ops_safe | off-by-one (mask `31→30`) | .469 | ✓ | 3270900602, 2617 |
| so_xor_and | shift_ops | shift_ops_safe | op-repl (`^→&`) | .806 | ✓ | 10, 0 |
| so_u32_shift | shift_ops | shift_ops_safe | op-repl (`<<→>>`) | .661 | ✓ | 41353730, 2 |
| so_i32_shift | shift_ops | shift_ops_safe | op-repl (`<<→>>`) | .661 | ✓ | 4188928429, 249 |
| su_wrap_add | sub_overflow | sub_overflow_safe | op-repl (sub→add) | .889 | ✓ | 1263225841, 19275 |
| su_xor_or | sub_overflow | sub_overflow_safe | op-repl (`^→\|`) | .852 | ✓ | 10, 0 |
| su_sat_max_const | sub_overflow | sub_overflow_safe | const (MAX→MIN) | .093 | ✓ | 1242218399, −2122219135 |
| **su_sat_off1** | sub_overflow | sub_overflow_safe | off-by-one (`>` → `>=`) | **0 (equivalent)** | — (excluded) | — |
| su_sat_min_const | sub_overflow | sub_overflow_safe | const (MIN→MAX) | .080 | ✓ | −1061109750, 2117074944 |
| su_sat_final_xor | sub_overflow | sub_overflow_safe | op-repl (`-→^`) | .537 | ✓ | 134217728, 35338 |
| im_gcd_base | intmath | intmath_eval | guard-weak (base `a→b`) | .142 | ✓ | 0, 251658240, 0 |
| im_ipow_op | intmath | intmath_eval | op-repl (modmul rem→add) | .133 | ✓ | 2, 0, 2^63 |
| im_isqrt_loop | intmath | intmath_eval | off-by-one (`<=→<`) | .041 | ✓ | 3, 3677410304, 0 |
| im_isqrt_const | intmath | intmath_eval | const (`n<2→n<3`) | .020 | ✓ | 3, 2, 2450949931008 |
| bu_pack_mask | bitutils | bitutils_eval | const (lane mask `0xff→0x7f`) | .032 | ✓ | 2, 9056768, 0 |
| bu_xor3_or | bitutils | bitutils_eval | op-repl (`^→\|`) | .092 | ✓ | 3, 3, 3 |
| bu_popcount_off1 | bitutils | bitutils_eval | off-by-one (`return n→n+1`) | .286 | ✓ | 0, 0, 0 |

## Equivalent-mutant case study: `su_sat_off1`

The mutation changes `if b < 0 && a > INT32_MAX + b` to `a >= INT32_MAX + b` in the
saturating-subtract guard. At the equality boundary `a == INT32_MAX + b`, the
unguarded branch computes `a − b == INT32_MAX` — exactly the value the saturating
branch returns. Both branches therefore return the same result on every input: the
mutant is **semantically equivalent**, and the independent oracle confirms 0/324
divergences. It is excluded from the recall denominator rather than counted as a
miss — and it is precisely the trap a naive `detected/injected` recall number would
fall into (24/25 vs the correct 24/24 on the original set). Note the fuzzer plays no
part in this call: the oracle's exhaustive structured sweep, not "the fuzzer found
nothing", is what establishes equivalence.

## Hard case: `dm_div_intmin_drop` (single-input boundary, with C-UB-free proof)

The mutation deletes the `a == INT32_MIN && b == −1` guard **in the Rust translation
only**; the C reference keeps it. The only divergent input is exactly
`(INT32_MIN, −1)`: there the mutated Rust computes `i32::MIN / −1` and traps, while C
takes its (intact) guard and returns the defined fallback. The fuzzer found this
single point in ≤1.13 s in all four independent runs (original + 3 seeds), and the
artifact-replay evidence proves the input is legal for the C oracle: the standalone
full-UBSan C build executes `div_mod_safe(−2147483648, −1)` with **empty
`ub_reports` and exit 0** (`is_ub:false`, recorded in `results/ablations/attribution/mut_rows/m2.json`).
This is the required direction of the recall principle: the C-side execution is
defined on the detecting input; the divergence (here manifesting as a Rust-side trap
rather than a wrong return value) is purely the injected Rust bug.

## Flakiness sanity check

5 representative mutants spanning the density spectrum (single-input
`dm_div_intmin_drop`; low-density `im_isqrt_const` .02 and `su_sat_min_const` .08;
medium `so_mask15` .40; high `na_final_sub` .89), re-run under 3 further fixed seeds
(2, 3, 4): **15/15 detected**, time-to-detect 0.21–1.13 s
(`results/ablations/attribution/mut_rows/m2_flaky_seed{2,3,4}.json`). Detection of these mutants is not
seed-sensitive at the 25 s budget. We do not claim statistical stability beyond
this check for the remaining mutants (single run each).

## Threats

- **Fuzzing stochasticity:** the main campaign is single-run. Mitigated (not
  eliminated) by the 5×3-seed check, the fixed budget, and the fact that all
  detections landed ≥20× under budget.
- **Mutation representativeness:** hand-written textual mutations in 4 bounded
  operator classes on scalar-boundary programs. They mimic translation-bug shapes
  (wrong operator, wrong constant, boundary error, dropped guard) but are not a
  random sample of real LLM-translation faults; RQ1's real-bug findings complement
  this.
- **Scope — scalar boundaries only:** the UB-free evidence path currently decodes
  scalar inputs, so this experiment covers scalar `#[no_mangle]` entries. Buffer
  boundaries (leb128/hex/base64-style) need a buffer-aware evidence build and are
  future work; nothing in the pipeline prevents detection there — only per-input
  UB-free certification.
- **Independent-oracle scope:** the sweep grid is structured, not exhaustive over
  64-bit domains (u8-scale domains excepted); a mutant divergent only outside the
  grid could be misfiled as equivalent. For the one mutant so filed (`su_sat_off1`)
  equivalence is additionally proven analytically (above).
- **Base-translation faithfulness:** on the un-mutated base, all these boundaries are
  TN in RQ2's Table A (same frame), so pre-existing translation bugs do not inflate
  detection.

## Implementation note

The M1 smoke campaign also surfaced one classifier bug in the shared RQ2 replay
path (an over-broad ASan regex that matched libFuzzer's advisory note, mislabeling
real divergence panics as memory UB). After the fix we re-ran the full 48-boundary
RQ2 Table A and reproduced it exactly, boundary-for-boundary (
`results/ablations/attribution/ubgate_rows/tableA_verify.json`).

## Files

- runner: `scripts/eval_mutation_recall.py` (+ `--dry-run` spec validator, `--seed`)
- validity oracle: `scripts/mut_equiv_oracle.py`
- specs: `scripts/mut_m1.json` (smoke, 5), `scripts/mut_m2.json` (25),
  `scripts/mut_m2_add.json` (3), `scripts/mut_flaky5.json` (flakiness subset)
- rows: `results/ablations/attribution/mut_rows/{m1,m2,m2_add,m2_oracle,m2_add_oracle,m2_flaky_seed{2,3,4}}.json`
