# STU Selection: The Differential-Testing Frontier

> **Status.** This document reconstructs the core (and only genuinely novel) contribution of
> the project after the prior, un-pushed work was lost to an SSD failure (2026-06-22).
> It supersedes the informal "smallest testing unit" framing. See [`mtu_definition.md`](./mtu_definition.md)
> for the entry/exit checkpoint mechanism, which is now demoted to a *localization* tool used
> **after** a divergence is found, not a selection mechanism.

## 0. One-paragraph summary

When C is translated to Rust (by C2Rust or an LLM), the function structure is **not 1:1**:
functions get split, inlined, wrapped, or have helpers extracted. Running differential fuzzing
at an arbitrary function boundary therefore conflates **real translation bugs** with
**false divergences** caused by a badly chosen boundary (unsynchronizable state, non-normalized
output, FFI mismatch, UB, allocator/padding differences). The project's job is to **automatically
select boundaries at which differential testing is both reliable and high-coverage** — the
*differential-testing frontier*. Structural distance is only one *estimator* of that frontier,
never the definition.

## 1. What an STU is (precise definition)

> Given a C program, its Rust translation, and a cross-language function mapping, an **STU** is a
> pair of entry points together with their **closed execution region**, such that the region has
> (a) input state that can be initialized consistently across both languages, (b) observable
> semantics that can be compared after normalization, and (c) cross-language correspondence whose
> reliability satisfies a given **harness-validity risk bound**.
>
> The **STU frontier** is the set of *maximal* such regions in the call graph that satisfy the risk
> constraint while maximizing covered program logic (a risk-bounded, coverage-maximal antichain).

Key consequences of this definition:

- We are **not** looking for the *smallest* unit. We are looking for the *highest* boundary that is
  still trustworthy — small enough to be a valid harness, large enough to cover real logic.
- "Equivalence" is **not assumed** from structure. Structure only *estimates* harness validity.

## 2. Why "structural alignment ⇒ a divergence is a real bug" is too strong

Structural similarity is **neither** sufficient for semantic equivalence **nor** sufficient for a
correct differential harness. Even with near-identical call graph and CFG, false divergences arise from:

- struct padding, pointer/address identity, allocator behavior;
- `errno`, global state, floating-point environment/rounding;
- undefined or unspecified behavior in the C side;
- output state that was not correctly **normalized** before comparison;
- a Rust helper that changes structure while the overall semantics are fully equivalent.

Therefore the target is the **verifiable / differential-testing frontier**. Structural distance is a
*feature*, an estimator of the probability that a harness at this boundary is valid — not the oracle.

## 3. Regions, not function trees (1:N / N:1 mapping)

The naive recursive rule "f is clean iff f and all callees are clean" is **too strict**: a C callee
that Rust inlines, or splits into two helpers, can still preserve boundary semantics. Node-level 1:1
isomorphism forces valuable complex functions to collapse down to leaves.

Instead we compare **closed boundary subgraphs**, allowing:

- 1:N and N:1 mappings between C and Rust functions;
- Rust-only helpers absorbed into the parent region;
- comparison of the **region's** external calls, state reads/writes, and observable outputs.

Distance is defined over two program **regions**, not single functions:

```
D( R_C(f), R_R(g) )
```

where `R_C(f)` and `R_R(g)` are the call regions rooted at the mapped entry points `f` (C) and
`g` (Rust), after mapping-driven merging of helpers.

## 4. Call graph → SCC DAG (not a call tree)

Real programs have recursion, strongly connected components, function pointers / dynamic dispatch,
shared callees, and external library calls. So:

1. Build the **call graph** (both sides), not a tree.
2. Condense it into its **SCC DAG**.
3. Compute the frontier **bottom-up over the DAG**.
4. **Unresolvable indirect calls** (function pointers, dynamic dispatch) are treated as
   *uncertainty* feeding the risk model, or as a hard gate when they cross the boundary — never
   silently assumed absent.

## 5. Hard gates vs soft costs (reclassified)

Determinism is **not** a blanket hard gate. Time, RNG, allocator, and IO can often be handled by
mocking, record/replay, or state normalization. The genuine hard conditions for a valid STU are:

1. Input state can be **initialized consistently** across both languages.
2. **Termination** is controllable (bounded / can be bounded).
3. Observable output can be **compared** (after normalization).
4. The C execution does **not** trigger UB on the tested domain.
5. Boundary dependencies can be **isolated or modeled**.

Signature **FFI-compatibility is not a hard gate** either. Nested pointer graphs can be constructed
by an input generator — at higher cost and risk — so signature complexity is a **soft cost**, not an
exclusion.

## 6. Don't collapse distance into a hand-weighted sum (yet)

A fixed `D = Σ wᵢ·dᵢ` invites the reviewer critique "the weights are hand-tuned." First version:
keep the **feature vector** `x_f` and learn the probability that a boundary yields a valid harness:

```
P( valid harness | x_f )
```

Then select the frontier by **constrained optimization** over the SCC DAG:

```
maximize_S   Σ_{f∈S} coverage(f) − λ·risk(f) − μ·cost(f)
subject to   the regions in S do not redundantly cover each other
             (S forms an antichain in the DAG)
```

The hand-weighted score is retained only as an **interpretable baseline** to compare the learned
model against.

### Candidate features `x_f` (estimators only)

- call-graph correspondence: count/share of unmapped callees in the region (1:N/N:1 aware);
- control-flow distance: Δ basic blocks, Δ cyclomatic complexity, Δ loop-nesting between regions;
- side-effect surface: globals touched, heap allocations, IO at the region boundary;
- input-domain fuzzability: how synthesizable the entry inputs are (scalars/arrays cheap; nested
  pointer graphs / opaque handles costly);
- boundary uncertainty: unresolved indirect calls, external library calls crossing the boundary;
- normalization burden: how much output state must be canonicalized to compare.

## 7. The hardest experiment: the false-divergence oracle

"C2Rust output is correct by default" **cannot** be used as ground truth: C2Rust itself can mistranslate,
the original C may rely on UB, and "fuzzing found no divergence" does not prove equivalence. The
evaluation is split into three groups:

- **G1 — Known semantics-preserving.** Manually curated, UBSan-clean benchmarks treated as equivalent.
- **G2 — Injected real translation bugs.** Manually/semi-automatically planted translation defects;
  tests whether STU-selected frontiers **detect** them (bug recall / precision).
- **G3 — Injected pure structural refactors, semantics preserved (the critical group).** Systematically
  generated semantics-preserving transformations — inline/outline, helper extraction, loop rewriting,
  data-structure wrapping — to test whether STU selection **avoids false alarms**. G3 is what makes the
  frontier claim defensible.

## 8. Novelty positioning

A standalone AST/CFG/call-graph similarity metric reads as heuristic engineering. The contribution is
the **boundary selection**, stated as:

1. Define the **differential-testing frontier** for cross-language differential fuzzing.
2. Formalize fuzz-boundary selection as **constrained frontier / antichain selection on the call graph**.
3. A **cross-language region-alignment + harness-validity risk model**.
4. Empirical evidence that automatically selected frontiers improve:
   - valid-divergence **precision**,
   - bug **detection** (recall),
   - reachable source **coverage**,
   - harness generation/execution **cost**.

## 9. Core evaluation metrics

- true-bug **precision**;
- true-bug **recall**;
- **false-divergence rate**;
- covered source-program logic (reachable coverage);
- harness generation + execution **cost**.

## 10. How this maps onto the surviving code

- `tools/clang_cp_inserter/` → grow into `tools/stu_selector`: reuse the Clang LibTooling scaffolding,
  add call-graph extraction, region construction, and feature-vector computation. (Rust side: MIR /
  `syn` / rust-analyzer for the matching region.)
- `tools/gen_fuzz_target.py` → largely unchanged; the fuzz entry points come from the selected
  frontier instead of being hand-specified.
- `docs/mtu_definition.md` (entry/exit checkpoints) → **post-hoc localization**: once a divergence is
  found in an STU, insert checkpoints to localize which sub-region diverged.

## 11. Recommended scope for first reconstruction

Start with **C2Rust only** (name-preserving `#[no_mangle]` output makes cross-language mapping nearly
free), build the frontier selector + the **distance-vs-false-divergence** curve (the headline G3
result), and defer LLM transpilers (which need semantic matching) to a second phase.
