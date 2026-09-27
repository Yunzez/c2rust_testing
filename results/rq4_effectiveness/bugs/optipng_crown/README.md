# optipng × CROWN: pipeline crashes in the ownership-analysis phase

**A second large real-world library that defeats CROWN's pipeline** (after cJSON, which crashed in the
*rewrite* phase). On optipng, CROWN dies one phase earlier — in **ownership inference during `analyse`** —
so no analysis artifacts are produced and `rewrite` can never run. CROWN cannot lift optipng out of the box.

## What happened (C → c2rust → CROWN recipe)

- **c2rust base**: optipng's shipped c2rust output (`laertes_benchmarks/optipng`, 57 files / ~95k LOC,
  bundling libpng + zlib + gifread; 0 macOS symbols, Linux-clean). Builds clean under nightly-2023-01-26
  (693 warnings, 0 errors). Needed the CROWN feature flags `#![feature(strict_provenance)]` +
  `#![feature(raw_ref_op)]` added to the crate root (CROWN's `preprocess`/`explicit-addr` inject code
  using them) — same adjustment as qsort.
- **preprocess**: succeeds after the feature-flag fix.
- **analyse**: **PANICS.** Solves 1087 functions successfully, then aborts:
  ```
  thread 'rustc' panicked at 'assertion failed: fitter.next().is_none()',
    crates/analysis/src/ownership/infer.rs:658:5
  ```
  The crash is on **`src::libpng::png::png_create_png_struct`** — its ownership signature solves fine at
  **precision 1** and **precision 2** (`(&, &, _, _, &, _, _) -> &move & & …`), but the **precision-3**
  pass trips the `fitter.next().is_none()` assertion in CROWN's ownership constraint solver. `analyse.sh`
  exits 101; `analysis_results/` is empty.
- **rewrite**: cannot run — it requires the ownership/mutability/fatness JSON that analyse never wrote.

Separately, CROWN's `explicit-addr` transform miscompiles optipng's CLI `main` (6 errors:
`error[E0277]: *mut std::env::Args is not an iterator`, `error[E0745]: cannot take address of a
temporary`). These are non-fatal to the solver (analysis proceeded through 1087 functions despite them),
but they show the preprocess codegen is also not clean on this crate. The **hard blocker is the ownership
panic**.

## Determinism / attribution

The failure is an assertion violation inside CROWN's own constraint solver (deterministic by nature),
triggered by a specific function (`png_create_png_struct`) at a specific precision (3). It is a defect in
**CROWN**, not in the c2rust base (which compiles and runs) or the original C.

## Master-table cell

`optipng / CROWN` → **✗(analyse panic)** — `ownership/infer.rs:658` assertion on `png_create_png_struct`
@ precision 3. Parallel to cJSON's `✗(rewrite crash)` but at the earlier *analyse* phase. No CROWN
translation exists to differentially test.

## Significance

Two of the largest, most realistic libraries in the matrix (cJSON 3.2k LOC recursive parser; optipng
~95k LOC codec bundle) both **break CROWN's pipeline outright** — cJSON in rewrite, optipng in analysis.
Reinforces that real-world code at scale defeats the safety lifter; CROWN's clean results (lodepng, lil,
genann, quadtree) are on smaller, more regular crates.

## Files
- `analyse_panic_evidence.txt` — grep excerpt: the precision-1/2 successes, the precision-3 panic, the
  explicit-addr errors, solved-count, exit code.
- Full log: `scratchpad/optipng_analyse.log`; workspace `scratchpad/crown_optipng_ws/`.
