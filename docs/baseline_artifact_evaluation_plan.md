# Released-validator artifact comparison

Status: active experiment plan, frozen 2026-09-13.

## Question

Can the released artifacts of RustAssure, FLOURINE, and VERT detect the 36
confirmed translation defects in `results/rq4_effectiveness/defect_manifest.json`?
This is an artifact-level comparison: we run the published implementation rather
than reimplementing a paper's method.

## Integrity rules

1. Baseline analysis, comparison, search, and classification logic is not patched.
2. We may perform setup described by the artifact, package an existing C/Rust pair
   in the artifact's documented input format, and supply required argument/type
   mappings. Every such adapter is archived and labelled manual or automatic.
3. The C and Rust computations are copied from the defect's frozen evidence. A
   semantic rewrite made merely to satisfy a baseline is forbidden.
4. A result is recorded at five successive gates:
   `submitted -> accepted -> compiled -> completed -> detected`.
5. Failure before `completed` is not called a missed defect. Final outcomes are
   `detected`, `missed`, `unsupported_input`, `compile_failure`, or
   `analysis_failure`.
6. A known concrete witness may be used only when the released artifact supports
   replay. In that case the result is labelled witness replay, not independent
   discovery. Symbolic search or artifact-native fuzzing is labelled detection only
   when the baseline itself produces the discrepancy.
7. Timeouts use the artifact's documented default. Any shorter smoke-test timeout
   is recorded separately and cannot establish `missed`.
8. An adapter may realize the original input contract (for example, allocate a
   fixed backing array, NUL-terminate generated string bytes, add a lossless name
   forwarding shim, or package the exact target and dependency closure). It may
   not evade a failure in the released baseline by changing target arity or return
   type, bundling otherwise supported scalar arguments solely for the emitter, or
   patching generated/baseline code. Such experiments may be retained as
   exploratory evidence, but are explicitly unscored. The scored outcome remains
   the released artifact's original failure.

## Frozen artifacts

Machine-readable identifiers and checksums are in
`results/baseline_artifacts/artifact_lock.json`. RustAssure is frozen at its
official repository HEAD and uses the authors' `rustify-klee` fork because the
released RustAssure repository references a KLEE checkout but omits its submodule
mapping. FLOURINE is frozen by the checksum of the paper's official tarball. VERT
is frozen by its Zenodo record and deposited-file checksum.

## Five-defect pilot

The pilot is fixed before running any baseline:

| ID | Observation needed | Why included |
|---|---|---|
| C1 | termination | Small qsort crash defect |
| S6 | mutated array | Void-returning qsort defect used in the motivation |
| S21 | scalar return | Pure return-value error in `crc32_combine` |
| S17 | output buffer | Return value is unchanged |
| C12 | global initialization / termination | Lost global table initialization |

These five cover crash, return, output-memory, mutated-input, and global-state
behavior. All have exact-source provenance in the canonical manifest.

## Execution order

1. Reproduce one shipped example for each artifact before submitting our pairs.
2. Run the five pilot defects through RustAssure, FLOURINE, and VERT wherever the
   documented artifact accepts an already-produced translation.
3. Audit every gate and classify failures. Do not repair baseline logic.
4. Expand to all 36 defects that satisfy each baseline's published input contract.
   The denominator remains all 36, with incompatibility reported explicitly.
5. Generate `SUMMARY.md`, `funnel.json`, and a LaTeX table from the same result
   records. The table reports both end-to-end detection (`detected / 36`) and
   detection conditional on completed analyses.

## Paper interpretation

`unsupported_input` measures artifact applicability, not verifier unsoundness.
`missed` is used only when analysis completes on the unmodified defective pair and
the released artifact reports no discrepancy. This distinction prevents build and
marshalling limitations from being presented as semantic false negatives.
