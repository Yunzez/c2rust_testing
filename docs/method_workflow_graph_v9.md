# Workflow figure specification (v9)

This specification supersedes the v8 workflow figure. The replacement uses
two panels: the upper panel explains how a harness is planned and built; the
lower panel opens that harness and shows its three runtime modes. The figure
must distinguish the two plugin interfaces and must not imply that either
plugin changes the differential verdict.

## What the reader should learn

1. Function matching decides which C and Rust boundaries correspond.
2. A `HarnessPlan` records one logical input, its independent C/Rust
   realizations, and the observations to compare.
3. A resource-realization plugin can supply construction knowledge that the
   generic planner cannot derive; a comparator plugin can only strengthen the
   final observation.
4. One generated binary supports Rust-only discovery, C-only isolation, and
   combined differential replay.
5. Discovery and judgment are separate: Rust-only fuzzing grows the corpus;
   combined mode replays that corpus and compares the two executions.

## Panel A -- Plan and construct an oracle-bearing harness

Draw this panel left to right:

```text
 C application ----\
                    > Function matching -> matched boundary
 Rust translation -/                         |
                                              v
                                      Generic HarnessPlan inference
                                              |
                            success ----------+---------- abstain
                              |                              ^
                              |                              |
                              |        optional resource-realization plugin
                              |        (representation + materialization)
                              |                              |
                              +------------------------------+
                                              |
                                              v
                                         HarnessPlan
                               logical values and storage
                               C/Rust argument views
                               direct / producer / in-place init
                               observations + provenance
                                              |
                                              v
                                      Deterministic emitter
                                              |
                 optional comparator plugin --+--> structured-state rung
                                              |
                                              v
                              generated native paired harness
                                              |
                                     build + preflight gate
                                   /                       \
                         executable                         recorded failure
```

The resource-realization plugin enters only after generic planning cannot
derive a complete construction. The planner validates its declarations and
persists their identity and hash in the plan. It does not inject arbitrary
target or comparison code.

The comparator plugin enters the observation side of harness construction. It
canonicalizes an otherwise opaque result or object on each side and contributes
one optional rung to the fixed comparison ladder. It cannot construct inputs,
repair a translation, or decide whether a mismatch is a defect.

Use different dashed labels for the two plugins:

- **resource-realization plugin** -- input/construction extension;
- **comparator plugin** -- output/observation extension.

Do not connect the comparator plugin to the input planner, and do not connect
the realization plugin directly to emitted source.

## Panel B -- Inside one generated harness

Use `quickSort` as the concrete example. Begin with one decode, not two:

```text
 fuzz bytes
     |
     v
 decode once according to HarnessPlan
     |
     v
 logical values: arr, low, high
     |
     +----------------------------+
     |                            |
     v                            v
 C realization                 Rust realization
 arr_C + copied scalars        arr_R + copied scalars
 independent storage          independent storage
 c_quickSort(...)             quick_sort(...)
```

Put the invariant immediately below this split:

> Same logical values; no shared mutable storage.

For a resource-valued input, the realization boxes may say “direct decode or
side-local producer/in-place initializer.” Do not put application-specific
lifecycle details into the `quickSort` example.

### Runtime mode switch

Below the input split, show that the same generated binary selects one of three
paths. A compact three-row layout is preferable to three duplicated harnesses:

| Mode | C side | Gate | Rust side | Result and use |
|---|---|---|---|---|
| **Rust only** | skipped | -- | Rust call | coverage feedback and Rust termination artifacts; discovery |
| **C only** | C call | -- | skipped | isolated reference evidence; confirmation and reach diagnostics |
| **Combined** | C call | C-UB noise gate | Rust call | fixed comparison ladder; saved-corpus replay |

The combined row must show the order:

```text
C call
  |
  v
C-side lightweight UB gate
  | UB observed                  | no configured UB observed
  v                              v
reject input / record reason     Rust call
                                   |
                                   v
                         fixed comparison ladder
                                   |
                       equal ------+------ different
                         |                   |
                      normal             candidate
```

Label the UB gate **noise gate**, not “C is defined” or “valid input.” It is a
cheap in-loop filter. Full C-side ASan/UBSan attribution happens later.

The comparison ladder should contain only:

```text
termination -> scalar/output values -> pointer nullness
            -> harness-owned buffers -> optional structured comparator
```

Addresses are never compared. The optional structured rung should carry the
same comparator-plugin marker used in Panel A.

### Discovery, replay, and confirmation

On the right edge of Panel B, show how the modes are scheduled:

```text
Rust-only fuzzing ----> saved corpus --------------------+
       |                                                 |
       +----> Rust termination artifacts ----+           v
                                              +--> candidate
saved corpus ----> combined replay ----------+
                                                    |
                                                    v
                         side-isolated confirmation
                C only + Rust only + combined + Rust no-sanitizer
                                                    |
                         recorded non-defect <------+----> reproducible divergence
                                                                  |
                                                         cluster + diagnose
                                                                  |
                                                           confirmed defect
```

The corpus and termination artifacts must be separate outputs of discovery.
The figure should not imply that C participates in coverage-guided Rust
discovery.

## Visual hierarchy

- Blue: C source, storage, and execution.
- Orange: Rust translation, storage, and execution.
- Green/teal: persisted `HarnessPlan` and successful normal flow.
- Purple dashed: optional plugin declarations and confirmation.
- Red: candidate or confirmed divergence only.
- Gray dashed: abstention, construction/build failure, or recorded non-defect.

Both plugin boxes should have an “optional, explicit provenance” subtitle. Do
not use red for a crash before confirmation.

## Details that remain in prose

Do not draw matching feature vectors, assignment equations, individual decoder
roles, plugin manifest fields, exact budgets, coverage identities, all
confirmation verdicts, C-guided diagnostic campaigns, or the defect taxonomy.

## Suggested caption

> **Validator construction and execution.** Function matching aligns C and
> Rust boundaries, after which the planner derives a persisted HarnessPlan for
> one logical input and two independent realizations. An optional
> resource-realization plugin supplies construction knowledge when generic
> derivation abstains, while a separate comparator plugin can extend only the
> final observation. The generated binary supports Rust-only discovery,
> C-only isolation, and combined replay. Combined mode filters C executions
> that trigger the lightweight UB gate before running Rust and applying the
> fixed comparison ladder; saved candidates undergo side-isolated confirmation
> before defect promotion.
