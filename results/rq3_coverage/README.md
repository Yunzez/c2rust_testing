# RQ4 — Coverage Beyond Shipped Tests

**Status: no evidence. Nothing has been run for this RQ.**

> **RQ4.** *How much code does our differential validator exercise beyond the test suites shipped with
> existing translators and translated libraries?* See `results/EVALUATION_PLAN.md`.

The directory retains its historical `rq3_coverage` name so existing evidence
links do not break.

## What the experiment requires

For the same translated artifact, instrumented with one common coverage mechanism and run under a
reported common budget:

1. run the **shipped acceptance tests** (the suite that was used to accept that translation);
2. run **our differential fuzzing campaign**;
3. report function, region, and branch coverage for each; and
4. partition the reached code into three sets: shipped-only, both, validator-only.

The claim is about *exploration*, not correctness, and the comparison is against the tests that accepted
the shipped translations — never against a fuzzer supplied by another system.

## What exists today and why it does not count

[`../rq4_effectiveness/reach_census.md`](../rq4_effectiveness/reach_census.md) (33/33 cells) is a
**one-sided** reach measurement: the "their tests" side is 0 by construction, so it cannot answer a
paired question. It also carries two limits that any reuse must inherit — 9 of its 33 cells have median
0, and its execution budgets span 4,000 to 2,000,000 runs (500×), so cross-library magnitudes are
invalid. It is retained under RQ4 as an interpretation limit on defects and bounded no-difference
results, which is how `evaluation.tex` cites it.

## Open questions to settle before running

- **"Shipped tests" is undefined for tools that ship no suite.** The definition must be written down per
  system before any cell runs, not chosen per cell afterwards.
- **Common budget.** A previous uniform-budget claim in this project was retracted; the budget, seeds,
  and stopping rule must be pre-registered per cell.
- **Common coverage mechanism** across a C reference and a Rust translation, so the three-way partition
  is meaningful rather than an artifact of two different instrumentations.
