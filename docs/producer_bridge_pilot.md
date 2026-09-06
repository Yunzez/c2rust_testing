# Producer bridge — genann-only pilot

*Binding design, 2026-09-05. Extends `harness_plan_architecture.md`; changes nothing in it. The
trimmed version agreed after review: no canonicalizer, no runtime field reads, no generic `T*`
inference. Scope is one library. Generalisation to cJSON/lil is a separate decision after the
five genann cells have run.*

## 1. The gap it closes

A parameter `T*` where `T` is a struct carrying pointers is rejected today ("needs invariant
reconstruction"). On genann that is every API but `genann_init` and the 2015 activation leaves:
planner reaches 5/12 (2015 source) and 1/15 (1.0.0 source). SACTOR's cached-sigmoid headline
lives in `genann_act_sigmoid_cached(const genann*, double)` and is unreachable.

The library's own lifecycle is explicit and short:

```
fuzz bytes → topology scalars + values
          → genann_init(inputs, hidden_layers, hidden, outputs)   (each side builds its own object)
          → target(ann, …)
          → genann_free(ann)
```

The harness for a boundary taking `T*` becomes that **operation sequence**. It is generated from
the plan, never hand-written; a `T*` the rules below cannot bridge stays a plan failure with its
reason, exactly as before.

## 2. Selecting the producer — derivable, auditable, or fail

A **candidate producer** for parameter type `T*` is a boundary that satisfies all four:

1. it exists on both sides — **implemented as a same-name lookup in the Rust translation**, which
   is what every translator in this study preserves; a renamed pair would need the RQ1 matcher's
   C→Rust name map threaded through the planner (producer and destructor recorded with both names),
   and today fails honestly with "not present in the Rust translation";
2. its C return type, after typedef resolution and with `const` ignored, has **canonical pointee
   type `T`**, and the returned object satisfies the target's mutability (a `T*` target may not
   be fed by a `const T*` producer; a `const T*` target may be fed by either);
3. it takes **no `T*` itself** (pilot is depth 1; `genann_copy(const genann*) → genann*` is out);
4. every one of its parameters is plannable by the existing InputPlan (`genann_read(FILE*)` is out).

If several remain, order by evidence from the artifact, never by name: first, a producer whose
result the shipped test/example driver passes to this target (a call-graph fact of `test.c` /
`example*.c`, not a schema) — applied only when those drivers are in the pair's compilation
database, which the genann pairs (one TU, `genann.c`) are not; then the fewest parameters. On genann
rules 1–4 leave exactly one candidate, so neither tie-break was exercised. The implementation
(`harness_plan._plan_producer`) additionally restricts pilot producers to **scalar-only parameters**
and requires the Rust producer to return a **raw pointer** to `T`; both are recorded as exclusion
reasons in `producer_alternatives`. The plan records `producer`,
`producer_alternatives` (each with its exclusion reason), and `producer_evidence`. Zero
candidates → `no producer for T*: …` and the boundary fails construction.

On genann the four rules leave exactly `genann_init`.

**Destructor**: the boundary that takes exactly `T*`, returns `void`, and transitively reaches
`free` (or a file-scope function-pointer alias of it, `cJSON_free = free`). This is an
**assumption, not an ownership inference**: the rule does not prove that what gets freed is the
parameter or one of its fields. It was checked by hand on the two instances it has been applied to
(`genann_free`, `cJSON_Delete`); before a third library it should become a BodyAnalyzer check that
the parameter (or a field path from it) reaches the `free` call. None → the object is not freed and
the plan says `lifecycle: not claimed`; leaks are not findings (`detect_leaks=0`). When the
**target itself is the destructor**, the object is gone after the call on both sides: no post-state
comparison, no second free — only the two terminations are compared.

## 3. Determinism of the two producers

The C producer and the Rust producer must build the same object from the same scalars. Randomness
is found **along the call graph**: the existing `effectful_functions()` fixpoint is reused with the
source set `{rand, random, drand48, lrand48, rand_r, srand}` — `genann_init → genann_randomize →
rand` is the path that a body-only scan misses.

If a randomness source is reachable:
- C side: `srand(SEED)` immediately before the producer call;
- Rust side: the translation is scanned for the same libc symbols (`libc::rand`, `rand()` via an
  `extern "C"` block). If found, `libc::srand(SEED)` immediately before the Rust producer call. Both
  sides share one libc PRNG in the harness process, so resetting *each* side is what makes the two
  objects equal rather than consecutive draws.
- If the Rust side reaches no libc randomness but the C side does (an idiomatic translation using
  another RNG), the producer bridge **fails for that translation**: `producer randomness not
  re-seedable on the Rust side`. Not bridged means not bridged.

`SEED` is a fixed constant of the harness (not fuzz-controlled): it is a determinism device, not an
input. No dynamic repeatability check in the pilot.

## 4. Inputs of the target — from shared scalars, never from the object

Array parameters of the target whose extent the C body derives from a field of the object
(`genann_run` reads `ann->inputs`) get their extent from **the producer's own decoded parameter**
(`n_inputs` = the value fed to `genann_init`), which both sides share by construction. One logical
array is decoded once and copied to two buffers of that length. A field that may have been
translated wrongly is never used to define the test input; whether the field was written correctly
is a *behaviour under test*, observed through the target's outputs.

The plan records the relation it relied on (`extent inputs ← producer param inputs`) so it is
visible that this is an assumption about the C source, established by reading `genann_init`'s
assignment `ret->inputs = inputs` — the BodyAnalyzer proves it or the extent is `unknown` and the
policy cap applies, as today.

## 5. What is compared, and what a divergence means

The fixed ladder is unchanged: termination → scalar → pointer nullness → known buffer contents →
plugin. The harness emits one phase marker per sequence step (`C2R_PH_PRODUCER`, `C2R_PH_TARGET`,
`C2R_PH_FREE`) on each side, so every outcome carries the step it happened in.

- Producer returns NULL on both sides → the sequence ends, outcome `normal` (an invalid topology is
  a legal input rejected by both).
- NULL on one side only → `divergence` at phase producer (nullness).
- A trap or panic → the phase marker names the step.
- A value divergence on the target's outputs is recorded as a **sequence divergence**:
  `init → target → free`. The pilot does **not** claim to separate a wrong producer from a wrong
  target; there is no canonical producer-state comparator, and the two objects' pointer and
  function-pointer fields differ by construction. Confirmation localises only what the phase marker
  and the panic site localise. Saying less than the data supports is the point.

**Known limit of the pilot's oracle (found at first generation, 2026-09-05 05:10).** `genann_run`
returns `double const*` — an interior pointer into the produced object's `output` array — and
rung 3 compares a returned pointer by nullness only. The run's output *values* are therefore not
compared in the pilot; the same holds for `genann_train` (void: termination + the `inputs`/
`desired_outputs` arrays only). Value-level comparison on genann comes from the activation
boundaries (`double` returns, rung 2), which is where SACTOR's cached-sigmoid headline lives.
Lifting `genann_run` to a value oracle needs the deferred §4 relation — "the returned pointer
addresses `ann->outputs` doubles, and `ann->outputs` is the producer's `outputs` parameter" —
proved from `genann_init`'s `ret->outputs = outputs` and `genann_run`'s return expression. That is
a follow-up, not part of the pilot; acceptance 6.2(b) is read accordingly: the multiply-accumulate
mutation is expected to be *invisible* to the pilot, and the cached-sigmoid mutation visible.

## 6. Acceptance for the pilot

1. On all five genann translations, `init → run → free` replays deterministically and the c2rust
   cell — the negative control — shows **zero divergences** on its coverage corpus.
2. Two seeded negative mutations, each in a throwaway copy of a translation, are detected:
   a. *producer*: one initial weight altered in the Rust `genann_init` path;
   b. *target*: the multiply-accumulate in the Rust `genann_run` altered, and separately the
      cached-sigmoid lookup broken.
   The detections carry the phase marker (a) → producer, (b) → target.
3. Planned and coverage rise on every tool; the *expectation* is 5/12 → ≥10/12 and 1/15 → ≥10/15,
   recorded as expectation, not as a gate.
4. Only after the five cells have run is generalisation (cJSON `cJSON_Parse → cJSON*`,
   lil `lil_new → lil_t`) decided.

### Status 2026-09-05 05:15 (implementation landed; cells not yet run)

| criterion | status |
|---|---|
| 1. deterministic, no spurious divergence | `genann_run` on all five translations: 30–60 s combined, 0 crash / 0 timeout / 0 divergence (c2rust 152k execs; CROWN through `Some(&mut *ann_r)`) |
| 2(b′). a real target-side value defect is visible through the sequence | SACTOR `genann_act_sigmoid_cached` via init → target → free: **1391 divergences in 1392 executions**, replay `kind=divergence phase=4 detail=return value`, both sides run normally alone — headline #32, reached with zero hand work; c2rust control 7.9M execs, 0 events |
| 2(a), 2(b) mutations | not run; (a) and the multiply-accumulate (b) are invisible by construction (see the oracle limit above), so they would only re-state that limit |
| 3. planned counts and coverage | 5/12 → **10/12** (c2rust, Laertes, C2SaferRust); 1/15 → **9/15** (CROWN), **13/15** (SACTOR). **Coverage, genann × c2rust, same 3 600 s campaign, ablation by unioning only the five non-bridged harnesses: regions 131/573 (0.229) → 462/573 (0.806), functions 6/12 → 10/12; the shipped minctest suite reaches 513/573 (0.895), only-tests = `genann_read`/`genann_write` (`FILE*`), only-ours 19 regions** |
| 4. generalisation | **the five cells have run (2026-09-05 11:48, `results/rq3_coverage/genann/`)**: ablation on every tool 0.166–0.231 → 0.707–0.815 of regions; four negative controls clean (0 divergences, 0 confirmed); SACTOR #32 confirmed 51/51 on three boundaries none of which is reachable without the bridge. Generalisation to cJSON (`cJSON_Parse → cJSON*`, with the existing comparator plugin for object state) and lil is now a decision, not a hope; the four adjustments it needs are listed in the session notes (producer params beyond scalars, driver-evidence ordering, exclusion of targets with two produced parameters, plugin-canonicalised object comparison) |

Two things the first cell surfaced (2026-09-05 06:10): (i) a **one-sided rejection guard**
(`if (inputs < 1) return 0;`) had no lowering — `bounded_scalar` with no upper bound raised in
`lower_to_schema`; it now lowers as a full-range scalar (values below the bound are legal inputs both
sides reject), which makes `genann_init` as a *target* allocate arbitrarily large objects: its
artifacts are overflow panics (`ub-gated` in the combined replay) and OOMs, expected noise. (ii) A
target that **returns** a fresh `T*` (`genann_copy`) leaks it — rung 3 compares nullness and nothing
frees the returned object — so a fork-mode child accumulates one object per execution and dies on the
rss limit (`oom-*` artifacts that replay normally). Freeing a returned `T*` only when the C body
provably returns a `malloc`/`calloc` result is the follow-up; "returns `T*` and a destructor exists"
would double-free a function that returns its own parameter.

## 6a. cJSON generalisation (2026-09-05, after the five genann cells)

Four changes, none to the comparison rules:

1. **Producer parameters** may be anything the existing InputPlan plans (scalars, strings, buffers),
   not scalars only; the producer's inputs are lowered by `lower_to_schema` and decoded / called by
   the same code as a target's, under the object's namespace. `cJSON_Parse(const char*)` and
   `cJSON_CreateDoubleArray(const double*, int)` are producers.
2. **Ordering of candidates** (all of which are legal sequences) — a **deterministic ranking
   heuristic**, not a proof that the "right" producer was chosen: the producer whose body reaches
   the most functions of the translation unit first (`cJSON_Parse` reaches the whole parser and can
   build any node; `cJSON_CreateString` reaches two helpers), then the one with fuzz-controlled
   inputs, then the one the shipped drivers (`<pair>/drivers/*.c`, compiled with the pair's flags,
   never linked) feed to this target, then the fewest parameters. Every candidate and its rank
   reason is recorded in the plan.
3. **Two produced objects in one call are refused** (`cJSON_AddItemToArray(array, item)`): after the
   call one owns the other, freeing both is a double free — a harness bug that would look like a
   crash. Ownership transfer between produced objects is not derived; the boundary fails with that
   reason.
4. **Object state through the comparator plugin.** When a plugin knows the produced type (cJSON's
   does), the two objects are canonicalised right after the producers — a difference there is the
   producer's, before the target ran — and again after the target — that difference is the target's
   effect on the object. This is the attributed oracle the trimmed pilot skipped on genann; on cJSON
   it is the existing plugin applied twice. It remains `structured-state`, never `full`, and the
   plugin remains translator-dependent.

Two consequences: with a produced object in play the boundary's **returned** pointer is never
freed (it may point into the object, as `cJSON_GetObjectItem` does; a fresh return leaks instead,
`detect_leaks=0`), and a target absent from the Rust translation is a plan failure (`--all`
enumerates the C side). Planning cJSON's 71 functions needed memoisation of the signature parse,
the producer plans and the destructor lookup; the ranking's reachability comes from the same
call-graph fixpoint that finds effects and randomness.

**cJSON × c2rust smoke (2026-09-05 14:00).** Planner 39 / 58 in 50 s; 18 boundaries take a produced
`cJSON*`, every one built by `cJSON_Parse` (ranked first of 14 candidates by reachability) and released by
`cJSON_Delete` (found once the call-graph fixpoint learned function-pointer aliases such as
`static void (*cJSON_free)(void*) = free;`); 9 boundaries refused for two produced objects.
`cJSON_GetObjectItem` and `cJSON_DeleteItemFromObject` build and run; the corpus replays 150 / 150
normal on both. The crash artifacts (≈ 600–1 100 per minute) are **the reference's own bug**: old
cJSON's `parse_string` writes one byte past its allocation on `\u` escapes, reproduced with the
`--c-sanitize` build C-only; the producer inherits it, so every produced boundary collects
`ub_associated` artifacts on `\u` inputs, and the two producer-stage object-state divergences seen
are the same overflow landing on different heap neighbours (`ub_associated_value` once the C-only
replay fires). The generator does not, and must not, hide a reference bug behind the producer.

**The producer is inside the UB gate (fixed 2026-09-05 16:15).** The in-loop gate used to wrap only
the target's C call; UB inside the C producer (`cJSON_Parse`'s `(int)double` cast on out-of-range
numbers) then reached the producer-state comparison and was reported as a *divergence* on the
faithful translation (149 of 8 796 corpus inputs on cJSON × c2rust, all at the producer phase). The
generator now resets the gate before the C producer and rejects the input as `ub-gated` if it fires;
confirmation would have adjudicated them `ub_associated_value` regardless. cJSON's discovery binaries
were rebuilt with the gated generator and its replay / confirmation redone, the campaign kept.

**Not bridged, by decision (2026-09-05):** a producer that returns `Option<&mut T>` (PtrTrans's
`cJSON_Parse(Option<&[u8]>) -> Option<&mut cJSON>`). The pilot requires a raw pointer on the Rust
side, and the implementation is **frozen** at the shapes above. The rule for extending it: a bridge
shape is implemented only when the same representation family appears in **more than one**
translator's artifact *and* the producer is actually usable there (defined, not a stub, with a
destructor). A shape seen in one artifact is recorded as **construction unsupported** for those
boundaries — with the reason (`cJSON_New_Item` is a `None` stub, `cJSON_Delete` is undefined in
PtrTrans's crate) — never implemented for that one artifact, so that no bridge is tuned to a single
translator's output. PtrTrans × cJSON therefore runs with the direct boundaries only.

**Same shape, one spelling (lil, 2026-09-05):** c2rust keeps C's pointer typedefs as Rust aliases
(`pub type lil_t = *mut _lil_t;`) where Laertes and CROWN write the pointer out at every use. The
producer check normalises through `rust_type_aliases`, which now also resolves a bare
`*mut/*const Ident` alias (fn-pointer and `Option` aliases stay opaque). Without it the bridge
lit on Laertes (51/145) and CROWN (42/145) but not on the faithful control (15/145) — the
opposite of what a translator-independent bridge must do. This is a normalisation, not a shape.

**Plugin compatibility is checked, and incompatibility degrades (2026-09-05, after the PtrTrans cell).**
A comparator plugin's Rust half reads the translated struct's fields by name and calls the
translation's destructor. `plugins/cjson/plugin.toml` now lists what it needs (`[plugin.requires]`:
struct, fields, functions) and the generator checks the list against the translation before
linking the plugin. A translation that renames a field (PtrTrans `type_` for `type_0`) or lacks
`cJSON_Delete` cannot host the comparator, and by the comparison ladder that is a **degradation to
pointer nullness**, never a build failure: the 10 `cJSON_Create*` boundaries that failed with
E0425/E0609 on PtrTrans now build against nullness, and a 5-second probe already reports
`returned pointer nullness` on both tried boundaries (C non-null, PtrTrans `None` through its
`cJSON_New_Item` stub). The rerun is queued after tulip. This is the ladder implemented correctly,
not a PtrTrans adapter: the check is generic and c2rust's cell is unaffected (golden unchanged).

## 7. Non-goals (explicitly out of the pilot)

Generic inference over arbitrary `T*`; producers deeper than one level; canonical comparison of the
produced object; runtime reads of object fields for input shaping; dynamic repeatability checks;
any change to the ladder, the confirmation verdicts, or `PROTOCOL.md`.

## 8. Files and order of work

1. `tools/stu_selector/harness_plan.py` — candidate selection (§2), randomness fixpoint (§3),
   shared-scalar extents (§4); new InputSpec adapter `produced_object` with
   `{producer, params, destructor, seed_reset: c/rust/none, evidence, alternatives}`; plan failures
   with the exact reasons above.
2. `tools/stu_selector/gen_diff_harness.py` — lowering of `produced_object`: decode the producer's
   scalars, `srand`/`libc::srand`, call the producer on each side, NULL handling, target call,
   destructor, phase markers. Golden regression re-frozen only if an existing entry changes (none
   should; the adapter is new).
3. `scripts/gen_harness_regression.py` — one new golden entry for `genann_run`.
4. Negative mutations (§6.2) as a scratch script, results archived with the genann cells.
5. Five genann cells through the unchanged `cell.py → replay → recollect → confirm` chain.

## 9. Verification before any cell runs

`python3 scripts/gen_harness_regression.py` green; `harness_plan.py --all` on the five genann
pairs prints the new planned counts with every failure reason; a generated `genann_run` harness
builds and runs 60 s on c2rust with 0 divergences; both negative mutations fire.
