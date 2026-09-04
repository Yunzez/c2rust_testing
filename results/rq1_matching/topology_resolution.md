# Call-graph resolution on translated crates — a threat to RQ1

Measured 2026-09-01 with `tools/stu_selector/analyzer` (rust-analyzer HIR).
Script: `scripts/rq1_topology_resolution.py`. Machine-readable rows + tool versions:
`rows/topology.json`; raw analyzer output per artifact: `raw/topology/<label>.analyzer.json`.
(Re-run from the committed script reproduces the table below exactly.)

## Why this was measured

The matcher's largest single contribution is call-graph topology: on the raw-LLM micro corpus
it lifts recall from 0.820 (node-only) to 0.876, and on the `lil` homogeneous cluster from
0.742 to 0.984. If the call graph cannot be recovered from an artifact, the matcher degrades to
its node-only configuration on that artifact.

An earlier draft claimed degradation from **raw** `raw_edges` counts. That was not valid
evidence: `raw_edges` contains duplicates and non-local targets (`Option::as_deref` and other
stdlib calls), and there was no control. The metrics below are:

- **unique local edges (≠ self)** — deduplicated `(from, to)` pairs where both endpoints are
  functions defined in the crate, self-loops removed;
- **edges per function** — the density that topology propagation actually consumes;
- **unresolved rate** — `indirect_calls` (`kind: call_unresolved`) as a fraction of all
  observed call sites.

Every PtrTrans artifact is compared against **compiling translations of the same library**.

## Measurements

| artifact | compiles | fns | unique local edges | edges/fn | local sites | non-local | unresolved | unresolved rate |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| qsort × PtrTrans | **yes** | 3 | 2 | 0.67 | 6 | 19 | 0 | 0.000 |
| qsort × c2rust *(control)* | yes | 3 | 2 | 0.67 | 5 | 6 | 0 | 0.000 |
| quadtree × PtrTrans | **yes** | 17 | 20 | 1.18 | 35 | 54 | 31 | 0.258 |
| quadtree × c2rust *(control)* | yes | 32 | 46 | 1.44 | 75 | 181 | 10 | 0.038 |
| quadtree × CROWN *(control)* | yes | 35 | 51 | 1.46 | 82 | 409 | 21 | 0.041 |
| **bzip2 × PtrTrans** | **no** | 61 | **2** | **0.03** | 2 | 26 | 50 | **0.641** |
| bzip2 × c2rust *(control)* | yes | 124 | 211 | 1.70 | 467 | 2127 | 29 | 0.011 |
| bzip2 × c2rust, CROWN input *(control)* | yes | 123 | 208 | 1.69 | 463 | 2082 | 29 | 0.011 |
| bzip2 × CROWN *(control)* | yes | 123 | 208 | 1.69 | 463 | 2323 | 52 | 0.018 |
| **lodepng × PtrTrans** | **no** | 252 | **4** | **0.02** | 4 | 20 | 4 | 0.143 |
| lodepng × c2rust *(control)* | yes | 237 | 523 | 2.21 | 785 | 1341 | 1553 | 0.422 |
| lodepng × CROWN *(control)* | yes | 237 | 523 | 2.21 | 785 | 2492 | 1718 | 0.344 |

## What this shows

**The predictor is compilation, not the translator and not the library.** PtrTrans's two
compiling artifacts have normal call-graph density — qsort matches its c2rust control exactly
(0.67 vs 0.67) and quadtree is the same order as its controls (1.18 vs 1.44 / 1.46). PtrTrans's
two non-compiling artifacts have **1–2% of the density of any compiling translation of the same
library**: bzip2 0.03 against 1.69–1.70, lodepng 0.02 against 2.21.

**Two different failure mechanisms, and the unresolved rate alone does not separate them.**
For bzip2 the analyzer sees call sites but cannot resolve them (0.641 unresolved). For lodepng
it barely sees call sites at all — 24 total across 252 functions — so the unresolved rate is a
misleadingly low 0.143. Conversely lodepng's *compiling* controls carry a high unresolved rate
(0.422 / 0.344) and still yield 523 edges. **Report resolved edge density; the unresolved rate
is a diagnostic, not the measure.**

**Side observation:** CROWN's ownership lift leaves the call graph essentially unchanged —
bzip2 208 edges before and after, lodepng 523 before and after. Whatever CROWN reshapes, it is
not the call structure.

## Consequence for RQ1

PtrTrans is the only shipped system in the corpus that renames functions, so it is the only
place a name-independent matcher is genuinely required. It contributes four held-out artifacts
(`results/rq1_matching/SPLIT.md`), and **two of them — bzip2 and lodepng, the two largest —
give the matcher almost no topology to work with.** On those two the matcher is effectively
restricted to its node-only configuration.

This must be stated as a limitation of the RQ1 result rather than left for a reader to derive.
It also bounds what can be concluded from those two artifacts: a low score there is evidence
about matching *under degraded static analysis*, not about matching under renaming.

Open question, not yet investigated: whether the analyzer can be made to resolve calls in a
non-compiling crate (rust-analyzer performs name resolution without full typecheck, so some
recovery may be possible), or whether these two artifacts must be reported with the limitation
attached.

## Reproduction

For each crate: run `analyzer <crate-dir>`, then from the emitted JSON take
`local = {f.name for f in functions}`; count deduplicated `raw_edges` whose `to` resolves to a
local function (by full name or leaf after `::`), dropping self-loops; divide by `len(local)`;
and take `len(indirect_calls) / (len(raw_edges) + len(indirect_calls))` as the unresolved rate.
