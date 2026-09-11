# HarnessPlan Resource-Realization Plugin Plan

## 1. Purpose

The current planner deliberately abstains when it cannot establish how a C
parameter corresponds to a translated Rust representation and how the same
logical value should be materialized on both sides.  Adding another hard-coded
bridge for each translation would make the core generator
application-specific.  This work therefore delivers an **extension
mechanism**: a HarnessPlan resource-realization plugin API, one reference
plugin, and a documented template that another user can copy for a different
library or translated representation.

The core abstraction is a **logical resource** with three parts:

```text
logical resource
  = C representation        (type, parameter role, argument view)
  + Rust representation     (type, argument view)
  + materialization         (create / pass / cleanup on each side)
```

The planner already derives all three parts for the shapes it understands
(scalars, arrays, strings, nullable owned objects returned by a producer) and
already owns a closed vocabulary of argument views (`*mut T`, `*const T`,
`&T`, `&mut T`, `Option<&T>`, `Option<&mut T>`, `Option<Box<T>>`, slices).
A plugin supplies the part the planner could not derive for a boundary it
abstained on — in version 1, the materialization — and **selects** the views
from that vocabulary; it never adds conversion code.  The plugin is not a
generated harness and not an alternative planner.  The existing planner
remains responsible for checking the resource, producing the persisted
`HarnessPlan`, and invoking the unchanged harness emitter, comparison ladder,
and confirmation pipeline.

The intended workflow is:

```text
matched boundary
      |
generic planning succeeds --------------------> HarnessPlan
      |
generic planning abstains: "no producer returns T*; no in-place initializer declared"
      v
compatible resource-realization plugin
      |
planner validates both representations, ownership, and phases
      v
ordinary persisted HarnessPlan ----------------> existing emitter
```

The purpose is not to maximize LodePNG coverage or to infer every useful
realization automatically.  LodePNG supplies a concrete reference
implementation because its archived abstention is exactly the missing
materialization: the planner looks for a *producer* (a function returning
`LodePNGState*`) and finds none, while the library establishes the resource
by an **in-place initializer** (`lodepng_state_init(T*)`).  What the reference
plugin adds is therefore

```text
stack allocation -> in-place initializer -> target -> cleanup
```

CROWN's translation additionally lets the same resource appear under two
views in one harness (`Option<&mut T>` for init/cleanup, `*mut T` for the
target); those views the emitter already supports, so the plugin only selects
them.  The reference plugin demonstrates the in-place-construction extension,
not that the planner has learned `Option<&mut T>`.  Additional realization
families and library plugins are independent follow-up work.

The automatic configuration remains the primary evaluation result.  If a
resource-realization plugin is used in an experiment, its result is reported
separately as **plugin-assisted** reach.

## 2. Version-1 scope

Version 1 demonstrates one materialization:

```text
C:    stack T -> init(T*)              -> target(T*)
Rust: stack T -> init(Option<&mut T>)  -> target(*mut T)
                                               |
                                      side-local cleanup
```

This is suitable for `LodePNGState` in the CROWN translation (the c2rust
translation is the same realization under a single view, `*mut T`
throughout).  The reference plugin says that the C raw pointer, the Rust
optional mutable borrow used by the initializer, and the Rust raw pointer used
by the target are views of the same side-local logical resource.  It does not
permit the reference plugin to fill pointer-rich fields manually.  That
restriction is a property of this reference realization, not a claim that the
plugin API already captures every future construction protocol.

The following capabilities are explicitly deferred:

- arbitrary-depth API sequences;
- callback or environment synthesis (`FILE *`, sockets, external handles);
- grammar or file-format generation;
- manual reconstruction of recursive object graphs;
- repairing a missing, stubbed, or failing translated producer;
- changing output comparison or divergence attribution;
- `T ** + length` callee-allocated outputs (a possible later plugin/API
  extension).

## 3. Trust boundary

A resource-realization plugin may describe how each side realizes the same
logical resource, but it must not decide whether the two executions agree.  In particular, it may not:

1. call or replace the target itself;
2. inspect the C result to construct the Rust input, or vice versa;
3. suppress a panic, signal, timeout, sanitizer report, or comparison result;
4. alter the comparison ladder or the confirmation verdict;
5. bypass a translated constructor that is itself the behavior under test;
6. introduce fixed semantic values that silently narrow the declared input
   domain.

The C and Rust resources are allocated independently from the same decoded
logical values.  Producer and initializer calls remain inside the existing
phase reporting and C-definedness checks.

For example, a plugin must not construct a cJSON object by hand for PtrTrans:
that artifact's `cJSON_New_Item` is a `None` stub, and bypassing it would repair
the translation and hide the observed reach ceiling.

## 4. Plugin interface

Each plugin is an explicitly trusted, versioned manifest.  Version 1 contains
no C or Rust support code: it selects a C type/role, states the corresponding
Rust types and views, and binds existing lifecycle functions.  Raw source
snippets are never interpolated from manifest strings into a fuzz target.  A
provisional CROWN binding is:

```toml
[plugin]
kind = "harness-plan-resource-realization"
library = "lodepng"
name = "lodepng-state"
version = 1

[resource]
c_type = "LodePNGState"
logical_role = "initialized-resource"

[resource.c]
parameter_type = "LodePNGState *"
storage = "stack"
initializer = "lodepng_state_init"
initializer_view = "raw-mut"
target_view = "raw-mut"
cleanup = "lodepng_state_cleanup"
cleanup_view = "raw-mut"

[resource.rust]
type = "LodePNGState"
storage = "stack"
initializer = "lodepng_state_init"
initializer_view = "option-mut-borrow"
target_view = "raw-mut"
cleanup = "lodepng_state_cleanup"
cleanup_view = "option-mut-borrow"

[requires]
c_functions = ["lodepng_state_init", "lodepng_state_cleanup"]
rust_functions = ["lodepng_state_init", "lodepng_state_cleanup"]
```

The exact manifest syntax should follow the existing comparator-plugin loader
where possible, but input resource-realization and output comparison plugins
remain different interfaces and namespaces.  Each view name belongs to the
emitter's existing closed vocabulary (`raw-mut`, `raw-const`, `borrow`,
`mut-borrow`, `option-borrow`, `option-mut-borrow`, …); the plugin selects a
view, and version 1 rejects an unknown view instead of accepting
plugin-provided conversion code.

The planner expands an accepted realization into existing plan concepts:

```text
logical resource and storage
C type and argument view
Rust type and argument view
initializer-specific adaptations
target-specific adaptations
cleanup-specific adaptations
phase markers
```

The emitted harness remains deterministic and inspectable: the serialized plan
records both type representations, every selected view, the lifecycle hooks,
and the exact plugin content hash.  The planner validates types, symbol
availability, phases, and ownership shape.  The manifest remains explicitly
trusted for the semantic assertion that these representations denote the same
logical resource.

## 5. Compatibility and validation

A realization is applicable only when all checks succeed:

1. the C target parameter resolves to the declared C type and role;
2. the Rust target parameter resolves to the declared Rust type and view;
3. the initializer and cleanup functions exist on both sides;
4. their normalized signatures accept the declared side-specific views;
5. the target accepts the declared Rust view without cloning, leaking, or
   narrowing the represented C cases;
6. initializer inputs are already constructible by the generic planner;
7. initialization is deterministic, or an existing auditable seed-reset rule
   applies;
8. cleanup cannot consume or free the resource before the target call;
9. a conservative BodyFacts check finds no target field read that is clearly
   left unestablished by initialization; the plan also records the plugin-owned
   assumption that the initializer establishes the resource invariant;
10. the plugin does not override a usable generic plan.

Failure of any check produces `construction unsupported` with the exact reason;
it never falls through to generated code and a compiler repair loop.  The
generic planner's own abstention for this shape is reworded to name the
extension point: `no producer returns T*; no in-place initializer declared`
(today: `no producer for T*: no function returns it`).

Each resulting `HarnessPlan` records:

```text
origin = plugin
plugin name, version, and content hash
logical resource type
C type/role and Rust type/adaptation
initializer and cleanup
owner plus initializer, target, and cleanup views
validation evidence and source locations
all rejected alternatives and reasons
```

## 6. Deliverables and implementation phases

### Phase A -- loader and plan representation

1. Add a resource-realization plugin loader beside, but separate from, the
   comparator plugin loader.
2. Add an optional plugin-origin record to `HarnessPlan` without changing plans
   that do not use a plugin.
3. Resolve C and Rust functions and normalized types using the planner's
   existing analyzers and name map.
4. Reject incompatible realizations before harness emission.

Acceptance: existing golden harnesses are byte-identical and all regression
tests remain green when no construction plugin is supplied.

### Phase B -- in-place realization path

1. Represent one logical resource, its C and Rust types, and the selected
   lifecycle/target views (from the closed vocabulary) in the plan.
2. Emit each side's declared realization over independent storage.
3. Place initializer and cleanup in the existing `PRODUCER` and `FREE` phases
   and C UB instrumentation.
4. Preserve normal `c-only`, `rust-only`, and `combined` modes.

Acceptance: the raw-pointer c2rust binding remains a clean control, and the
CROWN binding (`Option<&mut T>` for init/cleanup, `*mut T` for the target)
plans and builds without changing target or comparison semantics.
**Reverse acceptance:** the same manifest with the `initializer` entries
removed must fail planning with `construction unsupported` — this shows that
the plugin supplies the materialization, not the views.

### Phase C -- reference plugin

Implement `plugins/lodepng-harness-plan/` for `lodepng_inspect` in c2rust and
CROWN, for which:

- the current generic planner abstains;
- the C API exposes an initializer and cleanup;
- the C, c2rust, and CROWN functions are present and non-stubbed;
- CROWN's initializer and cleanup exercise the declared
  `Option<&mut LodePNGState>` adaptation;
- no format-specific constants are needed to create the state.

Run four checks:

1. generic plan fails with the archived reason;
2. plugin-assisted plan, build, and short execution succeed for both Rust
   representation families;
3. c2rust and CROWN paired replay are clean on the reference inputs;
4. a seeded negative mutation in either initializer or target is detected and
   attributed to the correct phase.

Do not use cJSON/PtrTrans as the reference because its translated producer is
missing behavior, not merely a representation adapter.

### Phase D -- reusable template and documentation

Add `plugins/harness-plan-template/` with commented placeholders for:

- manifest identity and compatibility requirements;
- C parameter type/role and Rust parameter type/adaptation;
- allocation, initializer, target-view, and cleanup hooks;
- plugin-owned assumptions and failure behavior;
- the closed adapter vocabulary and how an unsupported adapter is rejected;
- a minimal build-and-replay test.

Document how to copy the template, how plugin provenance appears in a
`HarnessPlan`, which checks the core performs, and which semantic obligations
remain the plugin author's responsibility.  Include one deliberately invalid
sample in the regression tests to demonstrate pre-emission rejection.

## 7. Validation and optional evaluation

The implementation is accepted when:

1. no-plugin generation remains byte-identical;
2. the reference plugin plans, builds, and runs in `c-only`, `rust-only`, and
   `combined` modes;
3. a c2rust negative control produces no divergence or lifecycle error;
4. incompatible symbols, types, or ownership shapes are rejected before code
   emission;
5. the serialized plan contains plugin identity, version, content hash, hooks,
   and assumptions;
6. the template can be copied into a second dummy resource-realization plugin
   without changing the core generator;
7. the reverse acceptance of Phase B holds (manifest without initializer is
   rejected), and the reference plugin's emitted harness is a golden
   regression entry alongside the no-plugin goldens.

No full-library coverage increase is required to validate the plugin API.  If
plugin-assisted results are later included in the paper, they must not replace
the standard automatic results.  Such an ablation may report:

```text
matched -> planned -> built
whole-artifact function and region reach
paired target-body coverage
confirmed outcomes
```

for automatic and plugin-assisted configurations, together with the number of
boundaries using each realization and the number falling back to the generic
planner (the funnel counts `planned (auto)` and `planned (plugin)` separately;
the main table uses only the former).

The supported claim is:

> Explicit cross-language representation and construction knowledge can
> extend a bounded definition-driven planner without changing the paired-input
> semantics or differential oracle.

The unsupported claim is that these realizations are automatically inferred,
that the planner learned a new argument view, or that plugin-assisted reach
measures the automatic generator.  Motivating evidence for the construction
limit itself: `results/rq4_c_reach/` (lodepng reaches 56/235 C functions under
both Rust-guided and C-guided corpora; the PNG entry points were never
planned).

## 8. Version-1 stop conditions

Stop rather than expanding the plugin when:

- the translated initializer is stubbed, fails, or returns a different logical
  resource;
- safe cleanup cannot be established;
- the realization requires reading one side to initialize the other;
- the required state depends on an undocumented callback, environment, or
  arbitrary object graph;
- supporting the example would require adding another construction family to
  the core rather than expressing it through the plugin hooks;
- completing the example would require format-specific seed generation or a
  multi-operation state machine.

These cases do not invalidate the extension mechanism.  They become requests
for later plugin/API versions rather than application-specific code in the
core generator.

## 9. Implementation status (2026-09-11, generator 0.10)

Implemented, uncommitted, measured on `benchmark/pairs/rq4/lodepng_c2rust` and
`lodepng_crown`, boundary `lodepng_inspect`:

- `tools/stu_selector/realization_plugin.py` -- loader (kind
  `harness-plan-resource-realization`, closed key set, identifier-only values,
  closed view vocabulary `raw-mut raw-const borrow mut-borrow option-borrow
  option-mut-borrow`, sha256 content hash), plan-origin record.
- `harness_plan.py` -- `_plan_realization` / `_realize_with` (checks 1-10),
  consulted only after `_plan_producer` abstains; `HarnessPlan.origin`
  (omitted from serialisation when None); the abstention reworded to
  `no producer returns T*; no in-place initializer declared`;
  `--realization-plugins`.  A manifest may declare several Rust bindings
  (`[[resource.rust]]`); each is verified on its own, the first that fits is
  taken and the others are recorded as rejected alternatives.
- `gen_diff_harness.py` -- role `realized_resource`: side-local zero-filled
  storage (`[u64; ceil(sizeof/8)]` for C from the C AST, `translated::T =
  zeroed()` for Rust, zero-validity of `T` verified), initializer in the
  PRODUCER phase inside the C UB gate, cleanup in the FREE phase, all three
  modes; `--realization-plugins`.
- `plugins/lodepng-harness-plan/plugin.toml` (reference),
  `plugins/harness-plan-template/` (template, README, invalid sample),
  `scripts/realization_plugin_test.py`, two plugin golden entries in
  `scripts/gen_harness_regression.py`, `--realization-plugins` pass-through and
  `planned_by` (`auto`/`plugin`) in `scripts/c2r_funnel.py`.
- One change outside the plugin path: `harness_plan._name_pairs` undoes the
  generator's Rust-keyword escape (`in` -> `in_`) before the buffer/length
  name relation, so `(const unsigned char* in, size_t insize)` pairs as
  buffer + length instead of a NUL string plus a free scalar the callee trusts.
  The 33 no-plugin golden entries are unchanged by it.

Measured (all commands and numbers in the review report):

| check | result |
|---|---|
| generic plan, both pairs | `FAIL ... no producer returns LodePNGState*; no in-place initializer declared` |
| plugin plan -> build -> run, c2rust and CROWN | planned (origin=plugin, bindings `raw-mut-lifecycle` / `option-mut-borrow-lifecycle`), built, `combined` / `c-only` / `rust-only` single runs `kind=normal phase=5` on 4 seeds |
| 60 s `combined` fuzz | c2rust 3,725,035 execs, 39 corpus files, 0 artifacts, 0 crashes; CROWN 3,692,746 execs, 42 files, 0 artifacts, 0 crashes |
| paired replay (both corpora + seeds, both binaries) | 81/81 `normal@phase5` on c2rust; 81/81 on CROWN |
| reverse acceptance (initializer entries removed) | `construction unsupported: ... check 3: the manifest declares no in-place initializer for LodePNGState on C` |
| unknown view (`box-by-value`) | rejected at load by both CLIs, exit 1, no output directory created |
| seeded mutations | Rust target 57->58: `kind=divergence phase=4 detail=return value` (c-only / rust-only normal); Rust/CROWN initializer passes `None`: `kind=panic phase=6` (PRODUCER) in combined and rust-only, c-only normal; C initializer `ignore_crc = 1`: `kind=divergence phase=4` on the bad-CRC seed, both single-side modes normal |
| golden | 33 no-plugin entries unchanged; 2 with-plugin entries frozen (35) |

Not established: any coverage number (no cell was run); attribution of an
initializer mutation that only changes state -- without a comparator for
`LodePNGState` the ladder sees it as a return-value divergence after the
target (phase 4), which is the correct attribution for what is observable;
the field check (9) is depth-1 and top-level-field granular and records what
it cannot decide as a plugin-owned assumption.
