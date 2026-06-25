# Roadmap — STU frontier (Layer 2). The anti-drift plan.

> **Why this file exists.** We spent months deepening a *component* (per-function harness-validity
> prediction) and mistook it for the project. This document fixes the narrative anchor and sequences
> the work so we stop drifting into detail. Read this before starting any new task. Created 2026-06-25.

## North star (one sentence)

> Show that selecting the differential-testing boundary at the **right call-graph abstraction level**
> (the STU frontier) **reduces false divergences while preserving coverage** — we are not predicting
> "which function is fuzzable," we are solving "at which layer of the translated program to test."

If a proposed task does not move us toward the **headline table** (below), it is out of scope.

## Layering — freeze Layer 1, work Layer 2

**Layer 1 — FROZEN (do not polish).** boundary-validity dataset · validity taxonomy · harness
generator v0.5 · classifier/audit · risk features · P(valid|x) baseline · external-validity evidence.

Its paper role, stated verbatim (the **freeze statement**, also in `stu_selection.md`):

> *We use exhaustive boundary harvesting only to construct labels and to train/evaluate the
> harness-validity risk model. At deployment time the system does NOT fuzz all boundaries; it
> statically selects an STU frontier on the call graph. Layer 1 is the risk estimator, not the
> main algorithm.*

**Layer 2 — ACTIVE.** call-graph frontier selection + the strategy-comparison experiment (G3).

## The headline result (the paper hinges on this one table)

| strategy | #harness | covered funcs | invalid rate | false-divergence rate | true-bug recall |
|---|---|---|---|---|---|
| public/root entry | low | high | high | high | ? |
| all constructible | high | high | medium | medium | high |
| leaf-only | high | low | low | low | low/med |
| **STU frontier** | medium | high-ish | **low** | **low** | high |

Column → ground-truth dependency:
- `#harness`, `covered funcs` — **computable now** (structural).
- `invalid rate` — computable **on labeled corpora** (the existing v4 boundaries). For a *new* deep
  Layer-2 corpus that has not been harvested/audited, first report **predicted** invalid risk
  (P(valid|x)), then audit a **sampled/selected** subset — do **not** let this column drag us back
  into full exhaustive harvesting of the new corpus.
- `false-divergence rate` — **needs G3** (semantics-preserving perturbation gives the "no real bug
  here, so any divergence is false" oracle). This is the keystone column.
- `true-bug recall` — **needs G2** (injected real bugs). Last.

## Hard prerequisite (the blind spot the earlier plan missed)

**Corpus depth.** Frontier selection only matters when the call graph has multiple layers so that
"which layer to test" is a real choice. Current corpus is shallow (median ≈3 functions/program;
musl externals are single-function leaves). A selector run on 3-function programs collapses all four
strategies into the same answer — the headline table would be vacuous. **Layer 2 needs fewer, deeper
programs with genuine internal call hierarchies.** Note the two layers want different corpora: Layer 1
(risk model) is fine with many shallow functions (incl. musl); Layer 2 (frontier) needs depth.

## Sequenced steps (with "done when")

- **Step 0 — plumbing + depth audit.** Verify `callgraph.py` / `rust_callgraph` / `mapping.py`
  produce reliable C & Rust call graphs + C↔Rust mapping on the deepest current programs
  (hash_table, opcode_dispatch, tiny_vm). Measure real depth/branching.
  *Done when:* we have a yes/no on plumbing and a factual read on whether current programs have
  enough depth, i.e. whether Step 1 must add deep programs.
- **Step 1 — Layer-2 corpus (pilot).** Add/curate **3–5 deep** real, c2rust-clean programs with
  multi-level call graphs **for the pilot**; scale to more **only if** the G3 table shows
  insufficient variance or a reviewer-risk gap (not by default). *Done when:* plumbing produces
  correct graphs+mapping on them.
- **Step 2 — selector v1 (simple, hard threshold).** Reuse `frontier.py`; `valid_region(f) =
  mapped ∧ constructible ∧ risk(f) ≤ T ∧ (children valid_region or absorbed Rust-only helpers)`;
  conservative sink for unsupported/cyclic/SCC; v1 scope = 1:1 matches + absorb a Rust-only helper
  only if called inside one mapped parent. Frontier = maximal valid_region antichain.
  *Done when:* it emits a frontier + per-STU subtree coverage + a sink reason for each unselected
  high node. **No tunable weighted sum yet** (avoids the hand-tuned-weights critique).
- **Step 3 — partial table now.** Compute the 3 ground-truth-free columns
  (#harness / covered funcs / invalid rate) across all four strategies. *Done when:* the half-table
  exists as a sanity check that frontier differs from root/leaf/all.
- **Step 4 — G3 (the headline).** Apply 2–3 semantics-preserving perturbations
  (helper extraction · inline/outline · wrapper insertion) to ~5 programs first; measure whether
  naive function-level fuzzing misreports structural misalignment as divergence while the frontier
  sinks to the correct region. *Done when:* the false-divergence column of the headline table is
  filled and frontier < baselines. Then scale.

  **Operational definition — a divergence is a FALSE divergence iff ALL hold** (fix this now to avoid
  classification arguments mid-experiment):
  1. the harness reports a behavioral mismatch (C vs Rust), AND
  2. the perturbation applied is **semantics-preserving** (by construction of the G3 transform), AND
  3. the C oracle is **UBSan/ASan-clean** on the diverging input (no intrinsic UB triggered), AND
  4. the mismatch is **attributable to the boundary/structure** (alignment artifact), **not** to an
     injected bug (G3 injects none) — i.e. it disappears when tested at the correct frontier region.
- **Step 5 — (defer) region-weighted risk.** `risk(region)`, `coverage(region)`,
  `score = coverage − λ·risk − μ·cost`, greedy bottom-up antichain. Introduce weights only once G3
  gives a curve to tune/validate against.
- **Step 6 — (later) G2 bug injection.** Inject real translation bugs; show the frontier still
  detects them (recall column) — guards against "you only cut false positives but miss bugs."

## Guardrails — DO NOT, unless Step 4's table demonstrably needs it

- ✗ more risk-feature engineering
- ✗ more generator input-type coverage
- ✗ more corpus *breadth* (more small flat programs / more musl single-functions)
- ✗ more P(valid|x) model sophistication beyond the baseline

Each is "frozen Layer 1." Touch only if the headline experiment forces it, and say so explicitly.

## Definition of done (project level)

The headline table populated (incl. G3 false-divergence column, then G2 recall), the
distance-vs-false-divergence curve, and the narrative reframed per the freeze statement. That is the
paper's main result; everything else is supporting evidence.
