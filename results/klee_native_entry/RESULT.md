# Native-entry exposure of confirmed defects

Date: 2026-09-22

## Method description for RQ4 (draft)

**Native-entry exposure.** We additionally examine whether the confirmed
defects can be exposed through the artifacts' existing application entries.
For each defect, we inspect the available C/Rust entries and the C call path
to the affected operation. Using the archived witness and source-level
diagnosis, we specify a target condition over the relevant arguments or
execution branch.

We prepare native inputs using archived inputs, project samples, and manually
constructed fixtures, together with KLEE-assisted symbolic execution. In
symbolic runs, selected arguments or file contents are made symbolic while
the remaining invocation is fixed. KLEE executes the C program from its
existing entry, with a target check in a separate analysis build. For
concrete fixtures, the check verifies that the invocation reaches the
specified condition. Symbolic searches receive an initial two-hour budget;
unresolved cases are rerun with an eight-hour limit under the same input
model and solver configuration.

We then replay the resulting inputs through the C and Rust application
entries without repairing their computations. We compare termination,
standard output and error, and designated output files, and apply our C-side
qualification and source-level confirmation procedure. A case exposes the
target defect only when the observed difference is attributable to that
defect. We separately record executions blocked by an earlier Rust failure
and executions that reach the faulty rewrite without a process-visible
difference for that input. This study measures exposure of known defects
under the constructed invocations; it does not measure blind discovery
performance.

## Input preparation and the role of KLEE

The known defect guides input preparation. The evaluator specifies the
entry, invocation, input channels, and target condition; KLEE does not
automatically infer these from the defect report. Depending on the case,
the condition checks an archived logical input or defect-relevant arguments
or a branch. It is not uniformly a byte-for-byte reconstruction of the
original function witness.

Input construction need not solve an entire structured file format
symbolically. For example, the OptiPNG directory-argument case keeps a
shipped, valid PNG fixed and makes only the relevant directory argument
symbolic. Other OptiPNG cases use shipped or constructed PNG fixtures and
check that the C execution enters the relevant checksum or decompression
branch. The bzip2 sorting case uses archived compression input as a concrete
native file. These concrete runs validate a selected path; they are not
automatic input-synthesis successes.

KLEE's target checks belong to the C analysis build. The ordinary paired
replay determines what the translated program actually does, including
whether it stops before the target. Analysis overlays and runtime
compatibility adaptations are recorded with the case recipes; the replay
uses the original computations. Each case retains its invocation, input
fixture, target condition, and replay evidence for inspection.

## Current recorded results

The inventory covers all 36 confirmed defects and currently classifies
19 as eligible for native-entry analysis under its entry, path, and
C-reference criteria. Eighteen cases completed a C-side target check and
ordinary paired replay. Replay exposed and attributed the same defect in
3 cases; 14 cases were blocked by an earlier failure in the Rust execution,
and 1 executed the faulty rewrite without changing the configured process
observables. The remaining eligible case, Lil C3, is currently recorded as
an unsuccessful symbolic search after the eight-hour rerun, although a
separate concrete native input is already available.

```text
audited cases                                  36
eligible cases in the current inventory (E)     19
target-checked and replayed cases (archived L)   18
same-defect process-level exposures (P)          3
```

The archived counts yield `L/E = 18/19` and `P/L = 3/18`, but `L` combines
symbolic input synthesis with target checks on concrete fixtures. The first
ratio must not be described as KLEE's automatic synthesis success rate, and
neither ratio measures blind defect-discovery recall. The exposure accounting
still needs the C3 reconciliation described below before it is used in the
paper.

All 18 recorded target-check successes occurred before the ten-minute
checkpoint of their recorded runs. The archived cumulative count consequently
remains 18 at 10m, 30m, 1h, 2h, and 8h. These times do not include manual input
preparation or preceding setup work and are not end-to-end construction
times. C3 has a concrete native input within the frozen nine-byte file bound,
but KLEE emitted no target assertion in either its two-hour primary or
identical-recipe eight-hour run. That outcome describes bounded symbolic
search failure, not absence of a native input or an unreachable target.

## Accounting to reconcile before paper integration

The input-preparation procedure accepts both constructed fixtures and
symbolically generated inputs. C3's existing concrete input must therefore
receive the same formal paired-replay and attribution treatment as the other
fixtures. Its symbolic-search outcome should be recorded separately from its
native-entry exposure outcome. Existing prechecks suggest an earlier Rust
failure, but this documentation update does not promote that observation
into a new scored replay result.

The raw inventory and summary retain their existing counts and historical
`lifted_*` field/status names. This report clarifies their interpretation;
it does not silently reclassify C3 or claim that all 18 inputs were synthesized
automatically. The final native-entry exposure denominator and outcomes must
be reconciled before copying the numbers into RQ4.

## Exclusions

Seventeen cases were excluded before solver outcome was considered:

| Reason | Cases |
|---|---:|
| No accepted application/native entry | 10 |
| Existing entry has no path that can materialize the function witness | 4 |
| Required input has no qualified C reference | 3 |

The no-path findings use source/call-site evidence; no KLEE timeout was used
to infer absence of a path. Details are in `eligibility_notes.md` and the
36-row `inventory.json`.

## Interpretation

Native-entry testing can reveal that a translated program is faulty, but an
earlier failure may prevent the same execution from exposing additional
defects. In the 18 recorded replays, 14 target defects were masked by earlier
failures in the Rust executions. Those failures can themselves produce a
process-level difference; they are not evidence that native-entry testing
failed to identify the artifact as faulty. Function-level validation allows
internal operations to be examined separately, enabling defects beyond an
earlier failing operation to be identified.

The exclusion categories have different meanings. Missing usable entries
and unavailable native paths delimit the scope of the existing application
entry. An unqualified C reference instead prevents attribution through that
execution and must not be presented as an entry-availability advantage.

The result should not be reported as proof that process-level testing cannot
find the remaining defects. In particular, a run that executes faulty code
without a process-visible difference establishes that observation only for
the tested input and configured process oracle. The study supports a benefit
of testing internal operations separately, not a general discovery-rate
comparison with CLI fuzzing or a proof of optimal native-entry testing.

## Recalculation

`summary.json` is derived from `inventory.json` with:

```sh
jq -f results/klee_native_entry/summarize.jq \
  results/klee_native_entry/inventory.json
```

The cumulative checkpoint field and C3 solver-limit note are appended from
`recipes/lil_c3/checkpoints.md`; all other counts are recomputed directly from
the per-case statuses.

This repository snapshot includes the report, configuration, inventory,
summary, and counting script. Per-case recipes, traces, and KLEE outputs
referenced above remain in the local experiment archive and are not included
in this summary-only snapshot.
