# Current evaluation plan

**Status: authoritative as of 2026-09-02.** This file defines the current
research questions and reporting units. Files under `results/archive/` and the
retired boundary-validity/reference-attribution plans do not define the paper.

## RQ1 — Matching accuracy

> How accurately does our matcher identify corresponding functions between the
> original C program and its Rust translation?

Evaluate every analyzable non-partial output from the ten-library corpus. Use
hidden-name equality as ground truth for name-preserving translations and
manual source/provenance inspection for translators that may rename functions.
Each library receives equal weight in the reported macro-average. Report
precision, recall, the number of correspondence pairs, and recovery of the
naturally renamed pairs.

Current evidence: `rq1_matching/`. The current aggregate is 40 outputs, 4,202
pairs, macro precision/recall 0.829/0.874, and 7/9 naturally renamed pairs
recovered. Manual labels remain preliminary until independently reviewed.

## RQ2 — End-to-end effectiveness

> How effectively does our validator detect real defects in C-to-Rust
> translations across libraries and translation strategies?

Apply the complete validator to the ten-library by six-translator matrix.
Classify each cell as confirmed defect, bounded no-difference result, process
failure, partial translation, or excluded C-reference execution. Count unique
confirmed root causes, not divergence inputs, symptoms, or tested boundaries.

Current evidence is stored under the legacy directory name
`rq4_effectiveness/`. The current catalogue marks 20 root causes as confirmed
across five rewriting systems and eight libraries: 7 crash/panic
defects and 13 silent semantic defects. The final paper count is conditional on
the exact-source and isolated sanitizer-replay audit described below.

## RQ3 — Defect taxonomy

> What recurring mechanisms cause defects in C-to-Rust translations, and how
> do they manifest across translation strategies?

Classify each RQ2 defect by one primary root-cause family so that the category
counts sum to the RQ2 total. For every family, report affected tools/libraries,
a representative consequence, and why the translators' acceptance tests did
not expose it. Cross-cutting symptoms may be discussed but must not be counted
as additional defects.

Current primary taxonomy in `rq4_effectiveness/defect_manifest.md`:

| Primary mechanism | Defects |
|---|---:|
| Control-flow preservation failure | 3 |
| Byte-string domain narrowing | 5 |
| Ownership-state corruption | 3 |
| Null/empty conflation | 3 |
| Initialization loss or corruption | 3 |
| Interface-contract loss | 3 |
| **Total** | **20** |

RQ3 describes the observed corpus. It does not estimate population prevalence
from these 20 defects.

## RQ4 — Coverage beyond shipped tests

> How much code does our differential validator exercise beyond the test suites
> shipped with the translators or translated libraries?

For the same translated artifact and a common coverage mechanism, run the
shipped acceptance suite and the differential validator under a declared
budget. Report function and region/branch coverage, plus code reached only by
the tests, by both, and only by the validator. Coverage measures exploration,
not correctness.

Current status: not yet executed. The protocol lives in `rq3_coverage/`, whose
directory name is historical. The old 33-cell Rust-only reach census is not a
valid substitute because it has no shipped-test comparison and unequal budgets.

## Component analysis — not a research question

The paper separately changes one validation component at a time on a fixed
evidence set. Different rows have different denominators and must not be pooled
into one recall or accuracy number.

| Component varied | Fixed evidence set | Full pipeline | Reduced configuration | What it establishes |
|---|---|---:|---:|---|
| Observable state | 9 confirmed defects | 9/9 | return only: 6/9 | Return-only comparison misses state-only defects. |
| Driver-independent state capture | same 9 defects | 9/9 | silent process driver: 3/9 | Process-output sensitivity depends on what the driver prints. |
| Function alignment | qsort: 3 pairs, 1 defect | matcher: 3/3 pairs | name equality: 2/3 pairs | Name equality loses the defective public contract boundary, although the same unique defect remains visible through `partition`. |
| Isolated memory-UB replay | 1 urlparser witness | excludes 1/1 | in-loop UBSan misses it | Without isolated ASan+UBSan, heap UB survives as a false candidate. |
| Source provenance | 1 lil mismatch | identifies 1/1 | unchecked reference | A clean but wrong source revision looks like a translation difference. |

The first two rows measure missed confirmed defects. The last two measure false
candidates. The alignment row measures lost correspondences and contract
boundaries, not a lost unique defect.

## Confirmation policy and evidence audit

UB handling and source provenance are part of defect confirmation, not a
standalone RQ. A final defect must be reproducible, use the C source revision
that produced the translation, execute cleanly under isolated ASan+UBSan, and
survive manual root-cause analysis. The inventory in
`rq2_attribution/harness_manifest.md` is internal QA: its 114 rows are historical
harness/configuration records, not samples and not a paper result. It identifies
legacy cells that must be rebuilt or downgraded before the RQ2 total is frozen.

## Reporting units

- RQ1: a library is the aggregation unit; function pairs describe evidence
  volume.
- RQ2: one independently confirmed root cause is one defect.
- RQ3: the same RQ2 defects, assigned one primary mechanism each.
- RQ4: paired test-suite/validator coverage on the same artifact.
- Component analysis: the fixed case set named in each row.
- Fuzz inputs and divergence records are within-case evidence and are never
  summed into a defect or sample count.
