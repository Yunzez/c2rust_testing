 ## Problems in the current figure

  ### 1. Stage 3 incorrectly shows boundary selection

  The current figure selects quickSort ↔ quick_sort while greying out partition ↔ partition. This represents
  the retired frontier-selection design.

  The current method does not choose one outer boundary or discard a matched function because an ancestor
  exists. It retains every aligned pair whose interface can be represented faithfully by the harness
  generator.

  This is especially misleading because the paper explicitly reports that the partition boundary exposes the
  qsort defect.

  Replace Boundary selection with:

  > Harness eligibility

  This stage determines whether an aligned pair can be represented, not whether it is safe from UB.

  Show:

  aligned pair
      ↓
  schema supported?
    yes → generate harness
    no  → explicit non-harnessable outcome

  Do not use the terms frontier, STU, entry-most, caller climbing, or boundary selection.

  ### 2. Harness generation and execution are currently duplicated

  Stage 4 currently shows call C and call Rust, while Stage 5 shows the calls again.

  Stage 4 should only describe construction:

  > fuzz bytes → one logical input → C representation / Rust representation

  It should also indicate that the harness defines which observable outputs will be recorded. It must not show
  either program executing yet.

  ### 3. Stage 5 omits the online C-reference gate

  The most important correction is that C and Rust are not executed unconditionally or in parallel.

  For every generated input, the order is:

  run C reference
         ↓
  C check passes?
     no  → exclude input
     yes → run Rust

  Use “C check passes?” or “C execution admissible?” rather than “UB-free,” because sanitizer-clean execution
  is not a formal proof that no UB exists.

  Add a compact annotation:

  in-process: recoverable UBSan
  out-of-process: isolated ASan+UBSan

  These are alternative harness paths, not two checks applied online to every input.

  The rejected-input branch must terminate before Rust execution and before state comparison.

  ### 4. Stage 6 should receive only admitted executions

  Rename or retain this stage as:

  > Observable-state comparison

  Show the normalized channels:

  - return or termination outcome;
  - designated output memory;
  - relevant globals;
  - stdout;
  - exit status.

  A difference in any applicable channel produces a candidate divergence.

  Do not imply that every function necessarily has every channel.

  ### 5. Stage 7 is incomplete

  The existing figure shows isolated ASan+UBSan replay but only says that it “drops C-side UB.” Confirmation
  checks more than that.

  Every saved candidate divergence undergoes:

  isolated ASan+UBSan replay
  source-version/provenance check
  deterministic replay
  adapter and correspondence inspection
  manual root-cause analysis

  It must have two visible outcomes:

  excluded candidate                  confirmed translation defect

  Examples of excluded causes can appear in small text:

  - detected C-side UB;
  - wrong C source revision;
  - non-reproducible C behavior;
  - adapter error;
  - incorrect correspondence.

  Do not draw a direct path from candidate divergence to defect.

  ## Required seven stages

  Use these exact stage titles:

  1. Inputs
  2. Function matching
  3. Harness eligibility
  4. Paired harness generation
  5. UB-aware execution
  6. Observable-state comparison
  7. Confirmation and triage

  ## Suggested contents

  ### Stage 1 — Inputs

  Show:

  - version-matched C source;
  - Rust translation;
  - optional translator metadata.

  The C version relationship matters because using the wrong C revision can manufacture a false difference.

  ### Stage 2 — Function matching

  Show correct pairs and an unmatched or abstained case, for example:

  quickSort ↔ quick_sort
  partition ↔ partition
  swap ↔ abstain/unmatched

  The matcher proposes correspondences; it does not prove behavioral equivalence.

  ### Stage 3 — Harness eligibility

  Show that all supported aligned pairs continue independently.

  A pair may become non-harnessable because its interface cannot be represented faithfully, such as an
  unsupported callback or opaque stateful handle. It must not be rejected merely because the C function could
  exhibit UB on some input.

  ### Stage 4 — Paired harness generation

  Show:

  fuzz bytes
      ↓
  one logical input schema
     ↙                ↘
  C arguments       Rust arguments

  Also show a small observable-state schema box.

  ### Stage 5 — UB-aware execution

  This is the central control-flow panel:

  run C
    ↓
  C check passes?
   ↙            ↘
  no             yes
  exclude input  run Rust

  Make the conditional ordering visually unmistakable.

  ### Stage 6 — Observable-state comparison

  Show C and Rust normalized observations side by side. A mismatch becomes a red candidate divergence.

  ### Stage 7 — Confirmation and triage

  Show the full replay and classification pipeline, followed by two outcomes:

  - grey or purple: excluded / unresolved candidate;
  - red: confirmed translation defect.

  Red should be reserved for an observed mismatch or confirmed defect—not merely a sanitizer check.

  ## Visual constraints

  - Preserve blue for C and orange for Rust.
  - Use purple for attribution/sanitizer validation.
  - Use grey for unsupported or excluded paths.
  - Use red only for divergence and confirmed translation defects.
  - Keep the figure readable at paper width.
  - Avoid dense prose inside boxes.
  - Make the primary left-to-right flow obvious.
  - The “exclude input” branch in Stage 5 must not reconnect to Rust or comparison.
  - The figure should not imply that every retained corpus record is a unique fuzz execution or defect.
  - Do not show numerical evaluation results in this method figure.

  ## Suggested caption

  > Seven-stage validation workflow. All faithfully representable aligned pairs are retained. For each
  > generated input, the validator executes C first and excludes inputs that fail the applicable online C-
  > reference check before invoking Rust. A mismatch in normalized observable state becomes a candidate
  > divergence, which is reported as a translation defect only after isolated sanitizer replay, source-
  > provenance checking, deterministic replay, and manual root-cause analysis.

  Please produce both:

  1. an editable HTML source;
  2. the rendered PDF suitable for replacing c2rust_paper/figure/workflow.pdf.

  Do not modify the paper text or overwrite the existing PDF until the revised figure has been reviewed.