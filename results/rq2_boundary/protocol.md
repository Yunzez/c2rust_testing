# RETIRED — RQ2 Boundary Validity Protocol v0.3

> **SUPERSEDED 2026-09-02 by `results/EVALUATION_PLAN.md`.** This proposed RQ
> was never executed. Boundary validity is no longer a research question; this
> file is retained only to preserve the design history. Do not implement or
> cite its sample definition.

## 1. Question and scope

> **RQ2. Given a correct C–Rust function correspondence, does the selected
> pair form a valid differential-testing boundary?**

RQ2 is conditional on RQ1. RQ1 determines whether the C and Rust functions
correspond; RQ2 takes the correctly matched pairs selected as harness
boundaries and asks whether comparing them is fair.

RQ2 does **not**:

- re-evaluate matcher precision or recall (RQ1);
- prove that the two implementations are behaviorally equivalent;
- estimate defect-discovery effectiveness (RQ4); or
- count multiple boundaries around one defect as independent samples.

A mistranslated function can still be a valid boundary. Validity means that
the two interfaces expose the same logical operation on a stated common input
domain, not that their implementations produce the same result.

## 2. Unit and sample

The primary unit is one **selected boundary pair** `(f_C, f_R)`.

The RQ2 sample contains every pair that satisfies all of the following:

1. RQ1's independent ground truth confirms the correspondence;
2. the frozen boundary selector chooses the pair as a harness boundary;
3. the artifact is buildable and belongs to a real library; and
4. the pair exhibits either internal restructuring or boundary-interface
   reshaping.

We use two descriptive tags:

- **Internal restructuring:** a split, merge, inline, deletion, or invention
  changes the internal function decomposition. A pure rename is an RQ1 case,
  not an RQ2 case.
- **Interface reshaping:** the boundary changes how its logical contract is
  represented, for example pointer-to-slice, nullable-pointer-to-`Option`,
  output-parameter-to-return-value, or error-code-to-`Result`.

The exact sample size is not estimated in advance. After applying these rules,
we report:

> **N selected boundary pairs from Y buildable artifacts, covering Z libraries
> and W translators.**

Pairs from the same artifact are reported together because they share a
translation and harnessing pipeline. Individual fuzz inputs and divergence
records measure the strength of one boundary experiment; they are not added to
the sample size.

Small programs such as qsort may be used as worked examples and pipeline
checks. Microbenchmarks are reported separately and do not contribute to the
headline real-library sample.

## 3. Inputs inherited from RQ1

Before RQ2 begins, freeze:

- the RQ1 ground-truth correspondence table;
- the matcher and boundary-selector revisions;
- the registry of buildable real-library artifacts; and
- the resulting list of selected, correctly matched boundary pairs.

An incorrect pair, missed correspondence, or matcher abstention remains an
RQ1 outcome. RQ2 deliberately conditions on correct correspondence so that
function matching and boundary validity are not measured twice.

## 4. Boundary-contract audit

For every sampled boundary pair, a human rater examines whether the two
interfaces support a common logical comparison. The audit considers:

1. **Logical operation:** the functions represent the same source-level task.
2. **Input domain:** their inputs can be mapped from the same logical values.
3. **Observable outputs:** return values, output memory, globals, and process
   effects can be mapped to the same logical observations.
4. **Preconditions:** the comparison does not silently give one side a
   different valid-input domain or required program state.

The audit uses the C contract and call sites to establish the intended
operation. The Rust signature determines whether that contract can be
represented. The Rust body and differential result are not used to decide
validity; otherwise a real translation defect could be mislabeled as an
invalid boundary.

Each pair receives one verdict:

- **fully valid:** the complete relevant C contract can be compared;
- **valid on a restricted domain:** a stated non-trivial shared domain can be
  compared, but part of the C domain is unrepresentable;
- **invalid:** no meaningful shared boundary contract can be constructed; or
- **cannot adjudicate:** the available source and documentation do not support
  a defensible decision.

Restricted domains must be named explicitly, such as “non-NULL inputs only.”
They remain visible in the results and are never merged into fully valid
boundaries.

A second human independently reviews every restricted, invalid, and unclear
verdict plus a preselected sample of fully valid verdicts. We report raw
agreement and explain how disagreements were resolved. This review is
completed before fuzzing results are examined.

## 5. Adapter specification

Every fully valid or restricted-domain pair receives a short, written adapter
specification before execution. It records:

- the logical input and its C and Rust representations;
- the permitted input domain and any declared restriction;
- the state initialized before each call; and
- the return values, memory, globals, stdout, and exit status to compare.

An adapter may exclude only C-side undefined behavior, documented
precondition violations, or a restriction already recorded by the contract
audit. The implementation must not change the translation logic.

## 6. Execution and instrumentation

For each fully valid or restricted-domain pair:

1. generate equivalent logical inputs for both interfaces;
2. execute the C and Rust boundaries under the same recorded input corpus;
3. instrument both sides to confirm that the restructured internal region is
   actually exercised rather than bypassed; and
4. compare the complete observable state defined by the adapter specification.

Coverage is supporting evidence, not the RQ2 outcome. We record the number of
internal C and Rust functions or source regions reached by each boundary. For
omitted initialization or other non-function transformations, region or state
evidence replaces per-function coverage.

The corpus, budget, seeds, and stopping rule are recorded per boundary. Input
records and divergence counts are reported per boundary and are never pooled
as independent samples.

## 7. Divergence triage

A divergence at a valid boundary is classified through the existing
confirmation pipeline:

```text
valid boundary + divergence
    -> invalid or version-mismatched C reference
    -> adapter implementation contradicts its frozen specification
    -> boundary-contract verdict requires correction
    -> genuine translation defect (reported under RQ4)
```

Adapter and verdict corrections are logged, independently checked, and rerun.
A newly confirmed translation defect is an RQ4 result; it does not turn the
boundary into an invalid RQ2 sample.

## 8. Measurements

RQ2 reports three primary measurements:

1. **Contract validity:** counts of fully valid, restricted-domain, invalid,
   and cannot-adjudicate boundary pairs.
2. **Interior execution:** how many valid pairs actually exercise the
   restructured region on both sides.
3. **Boundary-induced divergence:** how many boundary pairs produce a false
   difference attributable to the boundary specification or adapter rather
   than to the translated program.

Results are reported per artifact first and summarized by library,
translator, and transformation tag. We also report the median and range of
the actually executed interior size so that small call graphs cannot dominate
the evidence.

RQ2 does not maintain a second 20-defect denominator. Known defects may be
used as stress cases and illustrations, but defect recovery is counted once
under RQ4.

## 9. Execution order

1. Complete and freeze RQ1 ground truth.
2. Run the frozen boundary selector and construct the RQ2 sample.
3. Complete the contract audit and adapter specifications.
4. Pilot the instrumentation on a small worked example.
5. Run all qualifying real-library boundary pairs.
6. Triage divergences and publish the per-boundary evidence table.

The pilot validates the machinery only. The paper's RQ2 result comes from the
complete qualifying real-library sample defined in Section 2.
