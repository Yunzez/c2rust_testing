# C-guided application reach — C-only completion plan

Status: binding protocol after the user's 2026-09-12 scope decision.  It keeps
the persistence, provenance, input, budget, and C-coverage rules of
`docs/c_guided_run_workflow.md` and `docs/c_reach_plan.md`, but supersedes their
Rust remeasurement and 37-cell scheduling steps.

## Measurement

For each selected C realization:

1. rebuild exactly the archived-built boundary set with generator 0.9.2 and
   `--c-coverage`;
2. recover the archived campaign parameters and its non-generated initial
   inputs;
3. run a fresh `C2R_MODE=c-only` campaign for the archived 3,600-second budget;
4. persist the corpus atomically, then replay it only on C;
5. export C function and region coverage from every instrumented C object;
6. write `DONE.json` last, after build-set, input-count, coverage, and resource
   checks pass.

No Rust-guided campaign is rerun and the new C corpus is not replayed for Rust
coverage.  Earlier Rust-guided-to-C replay numbers remain labelled references.

## Experimental units

One representative is used where translations share the same C source and
input realization.  A distinct C source version is a distinct unit.  The
initial small-application batch is frozen as:

| unit | representative | archived-built boundaries |
|---|---|---:|
| qsort | SACTOR | 3 |
| genann (2015 C) | c2rust | 10 |
| quadtree | c2rust | 17 |

The full manifest is finalized from C-source and C-input-realization
fingerprints before larger applications run.  In particular, genann × SACTOR
and cJSON × PtrTrans use different C sources from their sibling cells and are
not silently merged.

## Resource and persistence rules

- Concurrent boundary fuzzers across all running units must never exceed 28.
- All processes are restricted to CPUs 0–27.
- Units may run concurrently when their combined active boundary count is at
  most 28.  The initial schedule is qsort (3) plus genann (10), followed by
  quadtree (17) while genann may still be active: maxima 13 and 27.
- Formal data lives under
  `/home/yunzez/c2rust_archive/cg_c_only_v3/<unit>/attempt-N/`; `/tmp` contains
  rebuildable work only.
- Every interrupted or invalid attempt remains present and is never promoted
  to `FINAL`.
- No running script is edited.  A valid attempt is renamed to `FINAL` only
  after its `DONE.json` checks are independently inspected.

## Completion

Completion requires every unit in the finalized manifest to have a valid
`FINAL/DONE.json`, a complete C corpus replay count, nonempty C function and
region universes, archived C coverage exports, and an aggregate table generated
from those final records.  Partial snapshots, smoke runs, and earlier Claude
pilots are not formal results.
