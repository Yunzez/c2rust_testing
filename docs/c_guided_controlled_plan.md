# Controlled C-guided companion experiment

Status: implementation and smoke validation. This document is binding before
any full-budget result is read.

## Question

Does C-side feedback expose source behavior that a translation-guided corpus
does not reach? This is a reach diagnostic, not a defect oracle and not a claim
about theoretical maximum coverage.

## Experimental unit

One translated artifact (one RQ4 cell). Its archived-built boundary set is
rebuilt exactly once with the current frozen harness generator. A missing
boundary invalidates the cell.

Two fresh campaigns then run sequentially on that same harness source and
binaries:

1. `C2R_MODE=rust-only` produces the Rust-guided corpus;
2. `C2R_MODE=c-only` produces the C-guided corpus.

The arms use byte-identical initial seed files, libFuzzer PRNG seed, wall-clock
budget, `max_len`, timeout, RSS limit, and boundary set. The archived corpus is
used only to recover the original initial seed files; it is not an experimental
arm. The archived Rust coverage remains a labelled reference only.

Both corpora are subsequently replayed on both C and Rust. We report
side-specific function/region reach and the matched-function `both`, `C-only`,
`Rust-only`, and `neither` sets. Region percentages across languages are never
subtracted.

Tulip uses the original automatic `seed_fixed` condition in this diagnostic.
Its later deterministic-grid campaign remains a separate seed-refinement
experiment and is never silently substituted here.

## Resource control

Only one cell may run at a time. It owns CPUs 0--27; no other fuzzing, coverage
build, confirmation, or stripping job runs concurrently. Within a cell all
boundary harnesses run concurrently, matching the archived application-level
policy: 3,600 seconds wall per cell, not one CPU-hour per harness. The two arms
run sequentially under the same policy. Their order is recorded.

This controls the comparison within a cell. It does not turn application-level
wall time into a normalized per-boundary CPU budget, so the paper must not rank
applications by fuzzing efficiency.

## Persistence and provenance

Formal output lives under
`~/c2rust_archive/cg_controlled/<cell>/attempt-N/`; `/tmp` contains rebuildable
work only. Each arm atomically persists its final corpus and campaign record
before its completion marker. `DONE.json` is written last. The driver records:

- Git HEAD and SHA-256 of the runner, generator, C-reach analyzer, and cell
  driver;
- exact boundary and initial-seed manifests;
- archived parameter source and all effective parameters;
- both campaign corpora, coverage exports, per-input outcomes, and checks.

A per-cell lock prevents duplicate attempts. A valid result requires the exact
archived-built boundary set, identical parameters and initial seeds across
arms, nonempty C/Rust measurements, and complete C-side replay counts.

## Execution order

1. Short smoke: qsort x c2rust, 15 seconds per arm.
2. Full-budget pilots, one at a time: qsort x c2rust, lodepng x c2rust,
   lil x C2SaferRust, and tulip x c2rust.
3. Inspect the frozen aggregate table. Expand beyond the pilots only if the
   C-guided arm adds interpretable reach beyond termination/profile artifacts.

The four pilots were selected before full-budget execution: a faithful small
control, a construction-limited application, a known translation-blocked
application, and a semantic-domain-limited application.

## Interpretation

The C-only mode still executes common harness decoding code; “C-guided” means
common harness edges plus C-side target edges, not literally a binary containing
only C. A C-side normal return is reach evidence, not proof of C definedness.
Exclusive reach enters the defect manifest only after the existing isolated,
sanitized, UB-aware confirmation pipeline.
