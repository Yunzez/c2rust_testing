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

The full manifest below uses one representative with the widest archived-built
boundary set for each C source version (qsort keeps SACTOR, the preselected
low-Rust-reach diagnostic).  The hash is of the compiled C source named by the
cell, not of translation-specific support files.

| unit | representative | C-source SHA-256 prefix | boundaries |
|---|---|---|---:|
| bzip2 | c2rust | `d4378051f781` | 19 |
| cJSON, compact source | c2rust | `aa1b902f9408` | 39 |
| cJSON, PtrTrans source | PtrTrans | `298581a04a36` | 9 |
| genann, 2015 source | c2rust | `49128acbe4fa` | 10 |
| genann, 1.0.0 source | SACTOR | `df0dee7dc389` | 13 |
| lil | Laertes | `abd896337a4a` | 51 |
| lodepng | c2rust | `19efa9e42f8a` | 54 |
| optipng | C2SaferRust | `493775c01168` | 96 |
| qsort | SACTOR | `b857f308fbdb` | 3 |
| quadtree | c2rust | `19871b2aaea0` | 17 |
| tulip | c2rust | `d174602cb962` | 212 |
| urlparser | c2rust | `68fc1ce6d2bb` | 20 |

The two cJSON sources and two genann sources are distinct units and are never
silently merged.  For applications with more than 28 boundaries, the same
cell is divided into deterministic boundary batches; each boundary still gets
the full archived campaign budget, and coverage is unioned only after every
batch validates.

The completion controller is `scripts/rq4/run_c_only_manifest.py`.  It keeps
the archived funnel order and partitions it into consecutive batches of at
most 28 boundaries.  Each batch persists the exact C function and region
identities (not only aggregate counts) in `c_identities.json.gz`; application
coverage is the Boolean union of those identities after all batch-level
checks pass.  Percentages and aggregate counts are never added across batches.

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
- Before every batch, the controller rechecks the full hashes of itself and
  the C-only driver, verifies the frozen C-source hash for the realization,
  and refuses to start while any external libFuzzer fork supervisor is live.

## Completion

Completion requires every unit in the finalized manifest to have a valid
`FINAL/DONE.json`, a complete C corpus replay count, nonempty C function and
region universes, archived C coverage exports, and an aggregate table generated
from those final records.  Partial snapshots, smoke runs, and earlier Claude
pilots are not formal results.

The full-manifest controller writes `SUMMARY.json`, `SUMMARY.md`, and finally
`MANIFEST_DONE.json` only after it verifies 12/12 valid units, one C export per
boundary, C-only protocols throughout, and a recorded maximum of no more than
28 concurrent boundary fuzzers.
