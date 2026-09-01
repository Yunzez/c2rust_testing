# RQ2 Boundary Census Protocol — v0.2 DRAFT (pre-registered; awaiting advisor sign-off)

Status: **DRAFT 2026-09-01, revision 2 after a three-lens adversarial review
(methodology / executability / fidelity; run wf_25724479). Nothing below has been executed.**
The census runs only after this document is approved and frozen. Guiding instruction (verbatim,
binding):

> Judge contract comparability, not behavioral equivalence; include both internal restructuring and
> boundary-interface reshaping; and use an independent human ground truth rather than the selector's
> own output.

Changes from v0.1, forced by the review: a `valid-on-restricted-domain` verdict (v0.1's binary rule
auto-invalidated the boundaries S1/S12 were confirmed on, deleting our own headline defects);
engineered — not asserted — independence (sealed selector outcomes, contamination denylist,
C-side-authoritative evidence rule); a falsifiable M2 (fixed denominator 20, pre-registered budgets,
an independent mutation arm); an executable enumeration order (correspondence first, qualification
second); rules for non-function regions, SCCs, splits/merges, and stubs; an exhaustive freeze
inventory.

---

## 0. Two measurements, two units, one estimand

| measurement | unit | question |
|---|---|---|
| **M1 boundary validity** | candidate boundary (enumerated independently of the selector) | can this C–Rust pair be *fairly compared*? |
| **M2 defect preservation** | confirmed defect (denominator = 20, fixed) | does ≥1 valid boundary reach and expose it? |

Execution evidence (records, coverage) is a supporting layer, never a sample count. **RQ2 is an
estimation study, not a yes/no test**: the primary quantities are the per-artifact M1 verdict ×
selector-outcome distributions; the pooled rate P(valid | selected) is secondary and, if quoted,
carries a cluster-aware (per-artifact bootstrap) interval — candidate boundaries within one
library×translator cell share failure modes and are not independent. Sample size is reported only
after the census: *X candidates from Y buildable artifacts over Z libraries × W translators*, with
the excluded universe (artifact count, estimated C-function count) reported next to Y.
Pre-declared workload estimate so the commitment is eyes-open: X ≈ 500–700 over ~25 buildable
artifacts (measured lower bounds: CROWN bzip2 21/143 lifted sigs, Laertes optipng 46 + 165 invented
inits, C2SaferRust optipng ≥64, PtrTrans cJSON ~90 reshaped of 94 translated); §4's family batching
is the corresponding mitigation.

## 1. The load-bearing distinction, with its evidence rule

**Boundary validity is contract comparability. Behavioral equivalence is decided by differential
testing afterwards. They never mix.** A mistranslated qsort is still a valid boundary; a rater who
marks a boundary invalid because the Rust side is buggy commits the one disqualifying error this
protocol exists to prevent.

Because translated code has no spec besides its body, the guard must be structural, not aspirational:

- **The contract is established from the C side only** (C source, C documentation, C call sites),
  and transported through the declared type mapping.
- **The Rust side contributes only its declared interface** — signature, parameter/return types,
  which globals and out-parameters are part of the interface. The rater judges whether the Rust
  *interface can carry* the C contract, never whether the body honors it.
- Translator-generated Rust comments may be recorded but are never deciding evidence (the tool must
  not testify in its own defense). Call-site citations are restricted to domain/precondition facts.
- Calibration example (binding): S8's `valuestring = None` assignment is **not** validity evidence —
  the `&mut cJSON` interface *can* carry `valuestring`, so the boundary is valid and the `None` is a
  divergence for §9 to find.

## 2. Artifact registry (freeze step 0)

Before any enumeration, commit a registry listing **every** library × translator artifact with:

- **buildable**: operational — `cargo build` exit 0 under a pinned toolchain, build command recorded;
  partially-compiling crates are excluded by this single rule, never case-by-case;
- **small-subject tag**: written criterion (≤ ~5 C functions or single-file toy); tagged artifacts
  contribute no candidates to the headline X but form a separately reported stratum — they are NOT
  removed from M2 (see §10); qsort is the worked example *and* an M2 contributor;
- **C-side provenance**: {exact C source archived at path / version-matched reconstruction /
  base-c2rust-crate-as-reference}. Where the reference is the base crate (Laertes / C2SaferRust /
  CROWN consumed c2rust crates; several original .c trees are absent), M1 measures comparability
  against the mechanically faithful base and the paper says so;
- **selector commit hash** (the frozen matcher + boundary-selection version).

Corrected corpus facts feeding the registry: bzip2×PtrTrans = 55/78 stub-reverts + whole-crate
assembly failure (unbuildable); lodepng×PtrTrans 241/255 Compile_Failed (unbuildable);
lil×PtrTrans compile-fail; genann×PtrTrans decl-only; the surviving real-library PtrTrans artifact
is cJSON (plus the small-tagged quadtree/qsort dataset crates). `tools/frameworks/` is gitignored:
every crate and C tree the census uses is archived into a tracked location at step 0.

## 3. Enumeration (independent of the selector, executable order)

The v0.1 order was circular (§3-qualification needs the counterpart that step 3 established).
Corrected order, per buildable artifact:

1. **Correspondence first.** A human establishes, for *every* C function, its ground-truth Rust
   counterpart-or-none. Tool maps and name equality are hints only; each GT pair records which hint
   seeded it plus one sentence of hint-independent verification (interior correspondence, call-graph
   position). Shipped maps are known 56% wrong on lodepng — "looks plausible" is not verification.
   For Laertes/C2SaferRust/CROWN this step is largely computable by diffing the tool crate against
   the name-preserving base c2rust crate; for SACTOR/PtrTrans it is a budgeted manual sweep.
2. **Qualification second.** Each (C fn, counterpart) pair is tagged under the two classes —
   **A internal restructuring** (split / merge / inline / delete / invent) and **B boundary
   reshaping** (ptr→slice, NULL-domain loss, out-param→return, error-code→`Option`/`Result`).
   Pure rename with 1:1 interior = RQ1, excluded here.
3. **Minimal-boundary rule third.** The candidate is the innermost C function enclosing each
   qualifying region; enclosing ancestors form its escalation chain.

Structural rules the v0.1 draft lacked (each was a hole a reviewer found):

- **Non-function C regions.** A C file-scope initializer (the `laertes_init_*` family: 277 invented
  functions, S3/S4/S5) is enclosed by no C function. Its candidate is the innermost C **API function
  whose contract depends on the initialized state** — one candidate per poisoned global (lowest
  common API ancestor of its readers), not one per reader. The rewritten c2rust `main()` wrapper
  (S13) attaches to C `main`, tagged A+B. A **Rust-side sweep** for translator-invented functions
  (the severed-init scanner exists) is reconciled into the census so invented code cannot hide.
- **Recursion.** Regions are computed on the SCC-condensed call graph; an SCC is one region whose
  candidate is its entry-point set; escalation chains use the immediate dominator in the condensed
  DAG (cJSON's parse_value/parse_array/parse_object cycle is one region).
- **Splits and merges.** A split counterpart is recorded as a set with a designated entry point
  (named at GT time; the selector is correct iff it selects the entry point). A merge yields one
  candidate per C function with the shared counterpart flagged, so X is reproducible.
- **Stubs.** A counterpart whose body is empty, panic-only, or a constant default with no data flow
  from parameters (mechanical test) is recorded as **no counterpart — visible partial translation**:
  its own reporting column, outside the valid/invalid denominator. This prevents the 24 cJSON stub
  groups from either flooding RQ4 with fake defects or flooding M1 with invalids.
- **Toolchain independence.** The C-side function inventory is produced by a tool sharing no code
  with the matcher stack (clang AST dump / ctags over the registry's C source), cross-checked
  against `c_analyzer.py` output; the reconciliation diff is part of the frozen census. (The shared
  analyzer once had a silent dedup bug; correlated omission would grade the selector on a universe
  missing exactly its own blind spots.)

## 4. M1 adjudication

**Rubric.** Per candidate with a counterpart, an evidence sheet rates: (1) logical operation,
(2) logical input domain, (3) observable outputs, (4) preconditions — each
`same / correspondent / correspondent-on-restricted-domain / different / unclear`, citing §1-lawful
evidence only.

**Verdicts.**

- **valid** — criteria 1–4 all `same`/`correspondent`;
- **valid-on-restricted-domain** — comparable on a stated *proper* sub-domain of the C contract
  (e.g. non-NULL inputs). The exclusion is named in the verdict, carried verbatim into the §7
  adapter spec, and reported as the boundary's class-B comparability loss. Divergences on the shared
  domain are real divergences. *(This verdict is what preserves S1/S12: comparable on non-NULL
  inputs, where their divergences were in fact confirmed — while still reporting the lost NULL
  domain as a finding.)*
- **invalid** — no semantics-preserving correspondence on any non-trivial shared domain, or the
  shared domain is contract-meaningless;
- **cannot adjudicate** — triggers only: (i) contract undeterminable from C source + docs in 30
  minutes, (ii) region undelimitable, (iii) any criterion still `unclear` after consultation.
  **Pre-registered ceiling: if cannot-adjudicate exceeds 15% of adjudicated candidates, the rubric
  is declared inadequate and revised — before any fuzzing — with full re-adjudication.**

**Calibration set (binding, adjudicated first, included in the artifact):** S1 (crc32_z → 
valid-on-restricted-domain, non-NULL), S12 (BZ2_bzBuffToBuffCompress → same verdict class),
S8 (valid; `None` is divergence evidence, not validity evidence), C1 (sentinel domain `high=-1`
inexpressible at the reshaped signature → preservation assessed at the innermost enclosing boundary
where inputs are expressible).

**Batching.** Adjudication proceeds by idiom family per artifact (Laertes severed-init, CROWN
`Option<&mut>` lift, PtrTrans ptr→slice, …): one full evidence sheet per family plus k = 5
pre-registered random member spot-checks, members listed in the frozen record; individual sheets for
singletons and anything unclear.

**Contamination guards (engineered independence).**

- Evidence-sheet agents (opus) run under an explicit **denylist**: `defect_manifest.*`,
  `rq1_bugs*`, `results/pilots/**/RESULT*` must not be read; each sheet carries a declaration of
  files consulted. Sheets state the C contract from declarations + docs *before* call sites are
  examined.
- A bug noticed during adjudication goes into a quarantine log and may not appear in any verdict
  rationale.
- Raters are blind to selector output (sealed, §5), to tool maps beyond the hint pass, and to the
  defect manifest during adjudication. The primary rater confirms or overrides every proposed
  verdict; the **override rate is reported as a first-class number** (an override rate near zero is
  evidence the human layer added nothing).
- **Reliability**: a second human rater — not an author of the matcher, blind to the manifest —
  blindly re-judges a sample stratified by class tag and proposed verdict, n ≥ 50 (or 20%,
  whichever is larger); report raw agreement, Cohen's κ, and prevalence-adjusted AC1 at criterion
  and verdict level; pre-registered floor κ ≥ 0.6, below which the full census is double-rated.
  The κ study and all disagreement resolutions complete **before** the verdict freeze.

## 5. Selector observation (sealed until after verdict freeze)

At step 0 the frozen selector runs once per artifact; its outputs are committed to a **sealed file
diffed against GT only after the §6 verdict freeze**, so no rater sees what the selector chose.
Outcomes form a partition keyed on GT state:

| GT state | selector outcome cells |
|---|---|
| counterpart exists | selected counterpart (**correct**) · escalated to a valid ancestor (**over-escalation**, own cell) · selected non-counterpart (**wrong**) · abstained (**false abstain**) |
| no counterpart, valid enclosing boundary exists | escalated to the innermost valid enclosing boundary (**correct**) · escalated elsewhere valid (**over-escalation**) · abstained without enclosing (**missed escalation**) · selected non-counterpart (**wrong**) |
| nothing comparable at any level | abstained (**correct**) · anything selected (**wrong**) |

Every escalation target appearing in GT or in selector output **enters the adjudication set** and
receives a full §4 sheet before outcomes are scored — no asserted-but-unrated validity.

## 6. Freeze inventory (binding order; every object gets a commit hash cited in the paper)

- **Step 0**: artifact registry (§2) · selector commit hash · sealed selector outputs · M2 per-defect
  annotations {class tag, NA-with-exposure-evidence} (§10) · small-subject partition · calibration
  verdicts.
- **Step 1**: candidate list + GT correspondences + reconciliation diff.
- **Step 2**: evidence sheets + verdicts + **adapter specs** (the §7 mapping is part of the verdict
  record — criterion 2 is judged against a written mapping, not a future one) + completed κ study.
- **Step 3**: adapter implementations + per-(defect, boundary) fuzz budgets, seeds, stopping rules
  (§8).
- **Step 4**: fuzzing and coverage runs.

After step 4, two amendment channels only, both logged, both reported by count in the paper, both
re-running affected cells: **adapter-bug** (implementation demonstrably contradicts the frozen spec —
cite spec sentence and code line) and **verdict-error** (requires independent second-rater
confirmation). Nothing else about verdicts, specs, budgets, NA assignments, or the registry changes
after results are visible.

## 7. Adapter specs

Each valid or restricted-domain boundary gets a written input/output mapping. **An adapter may
exclude only (a) inputs outside the C contract (UB, precondition violations) and (b) the exclusions
named in a `valid-on-restricted-domain` verdict.** Any other exclusion reopens adjudication through
the verdict-error channel. Per-boundary excluded-subdomain lists are published so a reader can audit
what was never fuzzed.

## 8. Execution budgets (pre-registered)

Per (boundary) cell: fixed total-execution budget, recorded seed policy, and stopping rule, frozen
at step 3 — "not preserved" must not be flippable by quietly fuzzing longer (this project has
already retracted one uniform-budget claim; the mistake is not repeated). Coverage instrumentation
per §RQ2-instrumentation plan; a cell's record and coverage counts are per-cell quantities and are
never summed across rows (manifest rule U1 imported verbatim).

## 9. Divergence triage on valid boundaries (feeds RQ4)

```
valid boundary + divergence
    → C-side UB                    (isolated ASan+UBSan replay + reference-version provenance)
    → adapter bug                  (frozen-spec contradiction; §6 amendment channel)
    → M1 verdict error             (second-rater-confirmed; §6 amendment channel)
    → genuine translation defect   (→ RQ4 confirmation pipeline)
```

Divergence counts are reported pre- and post-amendment. Scope note: this outlet measures
*manufactured* (false-positive) divergences only; an adapter that *suppresses* divergences is not
detectable here — that one-sidedness is stated in the paper.

## 10. M2 defect preservation

**Denominator = 20, always.** Headline reports the full partition
`preserved p + not-preserved q + not-applicable r = 20`; a rate over applicable defects may appear
only beside it. **NA requires exposure evidence, not structure**: NA only where the defect is
already demonstrably exposed at its own name-preserved, unreshaped boundary in the existing record,
cited per defect; NA assignments freeze at step 0, before any run. Defect-hosting artifacts
contribute M2 boundaries regardless of small-subject tags.

Coverage clause, corrected for omission defects: preservation requires covering the C-side region
implicated by the root cause and, where the defect mechanism executes Rust code, the corresponding
Rust region; for omission defects (S3/S4: the invented initializer is never called — the absence
*is* the bug) the Rust orphan's zero coverage is itself recorded as mechanism evidence, not a
failure of clause (a).

**Conditioning statement + independent arm.** All 20 defects were discovered by this project's own
differential apparatus, so M2 over the manifest is a *consistency* check, not an unbiased discovery
probability; the paper says so. The falsifiable arm: the 27 independently validated UB-free mutants
(RQ1b corpus, oracle-independent ground truth) run through the same valid-boundary machinery, giving
M2 a channel that genuinely can fail.

## 11. Reporting

- Headline: X / Y / Z / W; M1 verdict distribution × selector-outcome partition, per-artifact
  first, pooled with cluster-bootstrap CI second; both `valid/(valid+invalid)` and
  `valid/all-adjudicated` shown, cannot-adjudicate and partial-translation columns visible.
- Per-boundary row: library, translator, C fn, class tag, GT counterpart (or none / partial), sealed
  selector outcome, verdict (+ restricted-domain exclusion if any), |interior_C|, |interior_R|,
  interior coverage per side, per-cell records (U1: never summed).
- Median + range of |interior|; per-library and per-translator rollups. qsort is the worked example;
  small-subject and micro strata reported separately, never silently merged into the headline.
