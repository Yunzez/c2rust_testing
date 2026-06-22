# G1 Validation v1 — differential fuzzing at selected STUs (2026-06-22)

First end-to-end run of the full pipeline: STU selected → harness generated → fuzzed → divergence
classified. Goal of G1: on **equivalent** c2rust output, the **harness false-divergence rate**
should be ~0 (and the generator should produce no spurious panics of its own).

## Setup

- Generator: `tools/stu_selector/gen_diff_harness.py` (libclang signature → Rust FFI → byte-cursor
  differential harness; C oracle symbols renamed to `c_<fn>`).
- Runner: `scripts/run_g1.sh` (build, fuzz 40 s, force-kill the runaway LibAFL fuzzer by unique
  binary name, report CLEAN/DIVERGENCE).
- Programs chosen to cover the generator's parameter shapes:
  - `intmath_eval` — pure scalars (run separately, ~30 min)
  - `rle_encode` — `const buf+len` input + `mut buf+cap` output + scalar return
  - `ht_run` — `int* + len` array
  - `rpn_eval` — `const buf+len` input + `int64_t*` out-scalar

## Result

| Program | Shapes | G1 |
|---|---|---|
| intmath_eval | scalars | **CLEAN** (0 divergence, ~30 min) |
| rle_encode | in-buf + out-buf + return | **CLEAN** |
| ht_run | int array | **CLEAN** |
| rpn_eval | in-buf + out-scalar | **DIVERGENCE** — and it is *real* |

## The rpn_eval divergence is a genuine finding, not a harness artifact

The panic is at `translated/.../rpn_eval.rs:51`, inside the **translated Rust code** (not the
harness's comparison `panic!`):

```
attempt to multiply with overflow      //  *out = lhs * rhs;   (opcode '*')
```

Call order is C-first, Rust-second. The C oracle returned normally; the Rust translation panicked.
Root cause — the canonical c2rust divergence:

- **C side:** `int64_t lhs * rhs` overflow is **signed-integer overflow = Undefined Behavior**.
  clang `-O1` (no UBSan) silently wraps and returns `RPN_OK`.
- **Rust side:** c2rust copies `lhs * rhs` verbatim; the cargo-fuzz build enables
  `-Cdebug-assertions`, so Rust's overflow check **panics**.

So C is silently-UB and Rust panics on the same input → a real behavioral divergence.

## Interpretation

1. **Harness generator validated.** The three CLEAN results and the fact that the rpn_eval
   panic comes from the *translated code* (not the comparison logic) mean the generator produces
   **0 false divergences** of its own across all parameter shapes. The pipeline works end-to-end.
2. **First real C↔Rust divergence found**, and it is exactly the class the proposal's Oracle
   Principle anticipated: **C-UB-induced Rust panic**. This is a *finding to classify*, not a
   harness failure.
3. **G1's "false-divergence rate" must be split:**
   - harness artifacts (the thing G1 must drive to ~0) → **0 here** ✓
   - genuine divergences rooted in C UB (signed overflow, etc.) → a **finding category**, to be
     labeled, not eliminated. The STU input-domain / oracle should tag signed-overflow inputs as
     C-UB (per `docs/stu_selection.md` §5 / proposal Oracle Principle), so the divergence taxonomy
     separates "real translation bug" from "C-UB → Rust panic".

## Next

- Add a divergence **classifier**: C-UB (overflow/OOB via UBSan) vs genuine translation bug vs
  harness artifact. Re-run rpn_eval with the C oracle under UBSan to confirm the overflow is the
  C-side UB, auto-labeling this divergence.
- Broaden the G1 batch to the rest of the handled corpus; expect more overflow-class divergences
  in the numeric programs.
