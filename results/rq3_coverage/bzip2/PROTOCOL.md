# RQ4 — coverage beyond shipped tests: bzip2 protocol

*Directory name `rq3_coverage/` is legacy; this is current **RQ4** (see `results/INDEX.md`).*

Frozen 2026-09-03, after the first attempt was rejected (see `c2rust_diagnostic_pilot/`).

## The question

For one translated artifact, compare the coverage of **the shipped acceptance suite** with the
coverage of **the whole validator** — every eligible matched boundary, run differentially under one
fixed budget — measured on the *same* translated Rust source under the *same* instrumentation and
against the *same* denominator.

## What "ours" means

Not one harness. The validator's coverage of an artifact is the **union over all eligible matched
pairs**, so the pipeline is:

1. **Matched pairs.** Read the frozen RQ1 matcher output for the cell
   (`results/rq1_matching/raw/group_a/<cell>/matcher_output.json`, `forced` list). No re-matching.
2. **Harness eligibility.** Decided by the generator itself, not by hand: an entry is eligible iff
   `gen_diff_harness.resolve(..., infer=True)` yields items/abi for it — i.e. iff the current
   automatic generator can construct one logical input for both sides and compare the outputs.
   Every rejection is recorded with the generator's own message.
3. **Harness generation.** One differential harness per eligible pair, from the frozen generator
   with `--ub-free` (and `--expose-entry` for C `static` entries). Each harness is
   `input → C oracle (UBSan-instrumented) → UB gate → Rust → compare`; an input on which C trips UB
   is rejected **before** Rust runs.
4. **One artifact-level budget.** A single fixed wall-clock budget for the artifact, split evenly
   across the harnesses that build, run **sequentially** so each really gets its slice of one
   machine. bzip2: **3600 s total**.
5. **Coverage.** Each harness's campaign corpus is replayed once through a coverage-instrumented
   build of the *same* harness (`cargo fuzz coverage`), so the C gate is still in the loop and the
   Rust coverage collected is by construction the coverage of inputs that passed the gate and
   reached Rust.
6. **Union.** Function and region **identities** are unioned across harnesses. Per-harness
   percentages are never averaged and per-harness counts are never summed.
7. **Compare** against the shipped suite on the same artifact, same scope, same denominator.

## Counts reported per cell

`matched pairs` → `eligible` → `harnesses that build` → `harnesses that execute` (produce a corpus
and coverage), plus total fuzz wall-clock, then the coverage table. Each drop between those four
numbers is explained by name.

## Scope

A path whitelist over the translated **library** sources. Adapters, drivers, fuzz targets, the
translated CLI *program*, build scripts, std and dependencies are excluded. Both sides use the
identical scope and denominator.

## Denominator

The tests-side build carries `-C link-dead-code`, so its in-scope identity set is complete and is
used as the universe. Ours-side identities outside that universe are counted and reported, never
added: `--expose-entry` rewrites one signature line per harness, which shifts the columns of the
regions on that line.

## Instrumentation

Same toolchain both sides (`nightly-2025-09-01`, LLVM 21.1.0), same
`-C instrument-coverage -C codegen-units=1`, `llvm-profdata`/`llvm-cov` taken from that same
toolchain (never the system LLVM). Builds go to a scratch `CARGO_TARGET_DIR`; the shipped artifact
is never overwritten.

## Outcomes

`PAIRED`, `TEST-UNAVAILABLE`, `FUZZ-CORPUS-UNAVAILABLE`, `NON-BUILDING`, `PARTIAL`,
`EXECUTION-FAILED`. **None of these is ever converted to 0 % coverage.** Compilation or
`cargo check` is not a test; a test command that executes zero tests is unavailable, not 0 %.

## Adapters and tool fixes, declared

Anything that is not translated code is listed in the cell's `RUN.md`. For bzip2 that is: the C
amalgamation (the generator takes one .c per pair), the flattened single-file Rust crate (it takes
one .rs), Linux shims for the macOS libc symbols the transpiled crate references, the three-line
CLI `main` wrapper, and two documented fixes to a scratchpad copy of the harness generator that a
real library exposes for the first time (canonical return type; renaming C **globals** as well as C
functions, without which the oracle and the translation share one storage location).
