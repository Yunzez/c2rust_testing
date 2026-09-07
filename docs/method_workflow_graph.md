# Workflow figure specification

This file is a specification for redrawing the paper's method figure.  It is
deliberately much smaller than `method_harness_construction.md`: that document
is the reproducible implementation protocol; the figure should communicate the
three methodological decisions and the main data flow.

## What the reader should learn in ten seconds

The figure should answer exactly three questions:

1. **Which C and Rust functions correspond?**
2. **Can one generated logical input drive both interfaces?**
3. **Is an observed difference attributable to the translation?**

The central story is:

> match a boundary -> derive a harness plan -> fuzz the translation -> replay
> the saved inputs against both implementations -> confirm and attribute each
> candidate.

Do not try to visualize the complete runbook, every adapter, or every verdict.

## Required stages

### 1. Inputs and function matching

Show:

- version-matched **C source** and **Rust translation**;
- structural function matching;
- two possible outputs: a **matched boundary** or **abstain/unmatched**.

One small renamed-function example is useful.  The feature vectors, bipartite
assignment, confidence formula, and matcher implementation are prose-level
details and should not appear.

### 2. Generated harness plan

Replace the old binary "Harness eligibility" box with **Harness planning**.
The planner reads:

- the matched C signature and body, to derive a **C-shaped canonical input** and
  its storage relationships; and
- the Rust signature, to select a **lossless Rust bridge**.

The box should contain only these three named components, in this order:

1. **Input Planner** -- C-shaped values + storage, from C signature and body;
2. **Input Materialization** -- direct decoding or side-local producer;
3. **Rust Bridge** -- lossless adaptation to the Rust signature.

The names are what the method text explains; the figure only has to show that
planning *is* these three, in this order, feeding the harness.  Do not draw the
procedural steps (decode, materialize, bridge) as separate lines: they are what
the components do, not what the reader needs from the picture.

Oracle selection is **not** a box, and not a line either.  The return contract
is fixed by the comparison ladder, which stage 4 already draws; repeating it
under the planner only invited the reader to look for a fourth component.  Do
not draw the producer-selection algorithm, destructor inference, buffer-table
row inference, scalar clamps, or individual parameter adapters.

There must be a visible side exit:

> **construction failure (reason recorded)**

This is preferable to the old "not eligible" label: failure is the result of
attempting to derive a complete plan, not a preliminary rule that rejects all
pointers or complex return values.

Do not show a person selecting or editing a schema.  `HarnessPlan` and its
lowered schema are generated intermediate representations, not user input.

### 3. Generated paired harness

Show one fuzz byte stream decoded once into **decoded canonical values**, then
split into two independent realizations -- the generated harness that runs both
implementations on one logical input:

```text
                      decoded canonical values
                      /                      \
            C-side arguments            Rust-side arguments
            side-local storage          side-local storage
            C boundary call             Rust boundary call
```

The important invariant is **same values, no shared mutable storage**.  It
is not necessary to draw Cargo crates, C symbol renaming, shims, static
re-exports, build fixups, or execution-mode names here.

An optional **comparator plugin** should enter only at output comparison.  It
must not be drawn as part of input construction: plugins strengthen observation
of structured return objects but do not make unsupported inputs constructible.

### 4. Discovery and differential replay

This distinction is essential and must be explicit.  The current mainline is
not "run C, ask whether it is admissible, then run Rust" for every fuzz
iteration.

Draw two consecutive activities:

1. **Coverage-guided discovery (Rust only)** produces two *different* things,
   and they must be drawn as two boxes, not one: a **saved corpus**, which
   flows down into replay, and **Rust termination artifacts**, which leave
   sideways to the candidate node as the terminations channel.  Drawing them as
   one box ("saved corpus + crash artifacts") reads as if the artifacts were
   part of the corpus that is replayed, which they are not.
2. **Differential replay (same saved inputs)** is drawn as a fork and a join:
   *replay each saved fuzz input* forks to a **C execution** and a **Rust
   execution** and joins into the **comparison ladder**.  One small line under
   the fork -- *same saved bytes, side-local inputs* -- says why the two
   executions are comparable.

The comparison ladder should be one short line:

> termination -> scalar/output values -> pointer nullness -> harness-owned
> buffers -> optional structured comparator

Addresses are never compared.  Do not list globals, stdout, or filesystem
output: those are not general rungs in the implemented ladder.

The saved corpus of discovery is the corpus of replay: **one campaign, one
corpus**.  Do not draw a second fuzzing run for the comparison.

Both discovery terminations and replay differences feed a shared
**candidate** node (two channels: termination artifacts, replay differences).  The cheap in-loop UB gate may be omitted.  If included, it
must be labeled **noise filter**, never "oracle", "C admissible", or proof that
the C execution is defined.

### 5. Confirmation, attribution, and clustering

Show one **side-isolated confirmation** box with four concise labels:

- C-only, ASan + UBSan;
- Rust-only, ASan;
- combined replay;
- Rust-only, no sanitizer.

Its two high-level outcomes are enough:

- **recorded non-defect**: reference UB, instrumentation/out-of-contract
  effect, or non-reproduction;
- **reproducible translation divergence**.

Then show:

> cluster by failure site -> source-level diagnosis -> confirmed defect

A cluster may also resolve to **downstream blocked** boundaries (one defect in a
producer such as `lil_new()` shows on every boundary that takes the object);
one short label is enough.

This order matters.  Confirmation/classification happens before human root
cause analysis; many artifacts or downstream boundaries can collapse to one
defect.

## Recommended layout

Use two rows and five numbered stages.  This preserves the readable rhythm of
the current figure without cramming the implementation protocol into it.

```text
  [1 Inputs] -> [2 Function matching] -> [3 Harness planning] -> [generated paired harness]
                     | abstain               | construction failure
                                                                  |
                                                                  v
  [5 Confirmation + attribution] <- [candidate] <- [4 Discovery + replay]
              |
              v
       cluster + diagnose
        /             \
 recorded non-defect   confirmed defect
```

Inside stage 4, use a small nested flow:

```text
Rust-only fuzzing -> saved corpus + crash artifacts
                         |
                         v
               same-input C/Rust replay
                         |
                  comparison ladder
                         |
                      candidate
```

The paired-harness visual from the old figure can be retained (it is the
generated harness that runs both implementations on one logical input), but it
should
sit between planning and execution rather than being a full numbered stage if
space is tight.

## What to remove from the old figure

The following elements are inaccurate or unnecessarily detailed now:

- **"Harness eligibility" as a signature-level yes/no test.**  Replace it with
  generated planning plus an explicit construction-failure exit.
- The `BZ2_bzReadOpen(FILE *)` mini-case occupying most of a stage.  It remains
  a good prose example, but the main workflow figure should use at most the
  short label `FILE*/opaque environment -> construction failure`.
- **"run C -> C execution admissible? -> run Rust"** as the fuzz loop.  The
  campaign is Rust-only; C/Rust comparison happens during replay of the saved
  corpus.
- A comparison table containing `globals`, `stdout`, and `exit status` as if
  every harness observes them.  Use the implemented fixed ladder instead.
- "isolated ASan+UBSan replay" as a single undifferentiated operation.  The
  attribution depends on side-isolated and sanitizer-free modes.
- Manual root-cause analysis before candidate classification.  Automated
  confirmation and clustering come first.
- Exact budgets, snapshot times, seeds, fork flags, phase numbers, outcome
  vocabulary, identity keys, universe construction, hashes, archive contents,
  and filenames.  These belong in the method text or evaluation protocol.
- The complete adapter inventory (`buffer_table`, `capacity_ptr`,
  `produced_object`, and so on).  One phrase—**direct adapters or library
  producer**—is sufficient in the main figure.
- All detailed confirmation verdict names.  Collapse them into recorded
  non-defects versus reproducible translation divergences.

## Visual conventions

Retain the useful color distinction from the current figure:

- blue: C-side artifacts or execution;
- orange: Rust-side artifacts or execution;
- purple: confirmation/attribution;
- red: candidate or confirmed translation difference;
- gray dashed: abstention, construction failure, or recorded non-defect.

Avoid using red for ordinary fuzz crashes before confirmation.  They are
candidates, not established defects.

The figure should contain no more than about 25 short text labels.  Prefer
arrows and grouping over explanatory sentences inside boxes.

## Attribution table

Stage 5 is the one place where a box in the figure can only *name* a decision
procedure.  It carries a single marker -- a circled **T** on the *Confirmation
and attribution* panel title -- keyed to a small **attribution table** set
below the figure: two columns (*Confirmation evidence*, *Classification*) and
five rows, no colour.  Source of truth: `scripts/c2r_campaign.py::classify`.

Two wordings in that table are not negotiable:

- the first row is about the **reference**, not about C's quality: on that
  input the C execution does not provide a clean reference, so nothing about
  the translation follows from it.  Never phrase it as C being "not
  memory-safe";
- a timeout in the combined run is a confirmed termination **only when the
  translation alone also times out** (tightened 2026-09-07); otherwise the hang
  is not attributable to one side, which is why the last row reads *timeout
  without side attribution*.

A pseudocode box for input planning was tried and **dropped**: it duplicated
the method text and invited a heuristic-by-heuristic review of the planner
rather than a reading of the workflow.  A matching block, **Algorithm C --
Function matching (name-independent)** (`tools/stu_selector/matcher.py`),
remains optional and off-figure.

## Suggested caption

> **Validator workflow.** After aligning C and Rust functions, the planner
> derives one C-shaped logical input and a lossless bridge that materializes
> independent storage for each implementation. Coverage-guided Rust execution
> supplies inputs for paired differential replay. Side-isolated sanitized and
> unsanitized replays attribute candidates before clustering and defect
> promotion; unmatched and unconstructible boundaries remain explicit.

## Details that remain in prose, not the figure

The following points are important but should be explained next to the figure:

- facts in a HarnessPlan retain source evidence;
- producer bridges use library constructors on both sides;
- the comparison strength is recorded per harness;
- sanitizer-clean C replay is not a proof of defined behavior;
- optional plugins are untrusted and translation-layout-dependent;
- coverage uses the Rust translation and is reported separately from defect
  confirmation;
- the preflight gate (every built harness is run on the empty input, C-only and
  Rust-only, and for one minute with the campaign's own parameters; a harness
  that crashes on the empty input or on every job is held for review before any
  budget is spent).  It is a mandatory gate (protocol amendment 2026-09-06) and
  it is protocol, not method: it is **not drawn in the figure**;
- every harness records the hash of the generator sources that built it.

