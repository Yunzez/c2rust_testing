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

**Layer 3 — dynamic evaluation / OPTIONAL smoke (architecture decision 2026-06-25).** Dynamic
fuzzing is the **oracle and safety-net, never the selector**: (a) G3/G2 measure false-divergence /
recall to prove the *static* frontier works; (b) an optional short smoke may validate selected
harnesses before long fuzzing. The main method stays **static** (folding dynamic into selection would
dissolve the novelty into "try harnesses, see which don't crash"). **Corollary: no per-case `rf_`
hacking** — because Layer 3 measures static quality and catches residual false positives, the static
selector is allowed to be imperfect; an uncovered mechanism is a *measured limitation to report*, not a
cue to add another feature. `rf_input_clamp` is the one cheap shield proxy, capped. See
`stu_selection.md` §0b.

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

- **Step 0 — plumbing + depth audit. ✅ DONE (2026-06-25)** — `scripts/callgraph_depth_audit.py`,
  report `results/callgraph_depth_audit_v1.md`. Findings: **plumbing healthy** (48/48 pairs run clean,
  name-match mapping coverage 1.00). **Depth is the binding constraint**: only **7/48** programs have a
  real frontier choice (chain ≥3 + an internal node); max depth 5 (hash_table, the only rich one);
  41/48 are flat (depth ≤2; musl externals are depth-1 leaves). **Conclusion: Step 1 (add deep
  programs) is REQUIRED** — confirmed by data, not guess.
- **Step 1 — Layer-2 corpus (pilot). ✅ DONE (2026-06-25)** — 3 deep real libs vendored
  (`benchmark/pairs/{regex,bignum,tinyexpr}`, provenance `DEEP_CORPUS_PROVENANCE.md`): regex
  (Unlicense, 18 funcs / depth 8 / 6 internal), bignum (Unlicense, 27 / depth 5 / 5), tinyexpr
  (zlib, 29 / depth 5 / 4) — all transpile clean, mapping coverage 1.00, spanning matcher /
  arithmetic / parser. Scale beyond these only if the G3 table shows insufficient variance.
- **Step 2 — selector v1. ✅ DONE (2026-06-25)** — `scripts/stu_frontier.py`. Fixed interpretable
  static risk (no model/training): BLOCKED = not constructible/mapped; RISKY = unguarded signed UB
  (intrinsic) or unmasked struct-field index (isolation); T admits only OK. **Key correctness fix vs
  the naive rule:** constructibility is a STANDALONE-node property (a non-constructible helper does
  NOT poison a constructible parent that builds it) — only RISKY PROPAGATES up. `selectable(node) =
  constructible∧mapped ∧ no reachable RISKY`; frontier = maximal selectable antichain (top-down from
  roots); emits selected STUs + subtree coverage + sink reasons.
- **Step 3 — partial table. ✅ DONE (2026-06-25)** — `results/stu_frontier_v1.md`. Cells =
  #harness / covered-funcs / RISKY-exposed, across root / all-constructible / leaf / frontier.
  Frontier guarantees 0 RISKY-exposure **by construction** while keeping good coverage where the risk
  is localizable (bignum 17/18, regex 8/9) vs baselines that carry exposure (bignum all 27/27 with 9
  exposed); it honestly collapses to 0 where the UB is pervasive/unavoidable (hash_table, reduce_overflow).
  **Caveat: 0-exposure is definitional, not empirical** — whether avoiding RISKY nodes actually avoids
  false divergences (and testing them produces them) is what **G3 must measure**. This table is a
  sanity check + G3 setup, NOT the headline proof.
- **Step 4 — G3 (the headline). 🔄 IN PROGRESS — Case A ✅ (2026-06-25).** Design + oracle:
  `docs/g3_pilot_design.md` (translator-independent: Path 1 constructed-equivalence + Path 2 reachability;
  two flags `name_preserving_mapping`/`translation_trusted`; selector-as-triage when translation untrusted).
  Case A (`benchmark/pairs/g3_case_a`): helper `scale` → false divergence (C_UB_CONFIRMED), api `scale_pct`
  → clean (1.09M execs) — same code, higher boundary, divergence vanishes. Case C
  (`benchmark/pairs/g3_case_c`, struct-cursor isolation): helper `ring_at` → false divergence (OOB), api
  `ring_get` (`head %= CAP`) → clean (700k execs). **Empirical strategy table** via `scripts/g3_eval.py`
  (`results/g3_eval_v1.md`): leaf/all light up false divergences on both; **frontier v2 wins on arithmetic
  (0 false-div, full coverage) but COLLAPSES on isolation (clamp shield ≠ `%`-mask invariant)** — a
  measured limitation, reported not patched (no per-case rf). Caveat: 2-level cases make `root`==ideal api;
  a 3+-level case is needed to show frontier > root.

  Apply 2–3 semantics-preserving perturbations
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
