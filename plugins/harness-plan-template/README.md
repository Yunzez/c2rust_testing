# HarnessPlan resource-realization plugin -- template

Design and rationale: `docs/construction_recipe_plugin_plan.md` (read sections 3, 5 and 8 before
writing one). Reference implementation: `plugins/lodepng-harness-plan/plugin.toml`
(`LodePNGState` for `lodepng_inspect`, c2rust and CROWN). Loader and vocabulary:
`tools/stu_selector/realization_plugin.py`; the checks: `harness_plan._realize_with`.

A resource-realization plugin is a **trusted, versioned TOML manifest** -- no C, no Rust. It
supplies the one thing the generic planner could not derive for a boundary it abstained on with

    signature: struct-invariant param <p>: <T> ...; no producer returns <T>*; no in-place initializer declared

namely the **materialization** of that parameter: the library establishes a caller-owned `<T>`
in place (`T x; t_init(&x); target(&x); t_cleanup(&x);`). The manifest binds the initializer
and cleanup on each side and **selects** the argument views from the emitter's closed vocabulary.
It is a different namespace and kind from the comparator plugins (`--plugins`, `plugins/cjson`),
which are code for *output* comparison.

## How to copy it

1. `cp -r plugins/harness-plan-template plugins/<library>-harness-plan`, delete this README's
   copy and `invalid_sample.toml`, fill every `<placeholder>` in `plugin.toml`. A manifest with
   placeholders left in is rejected at load (`name` must be `[A-Za-z0-9_.-]+`).
2. Plan the boundary with and without it:

       python3 tools/stu_selector/harness_plan.py --pair benchmark/pairs/rq4/<pair> --entry <fn>
       python3 tools/stu_selector/harness_plan.py --pair benchmark/pairs/rq4/<pair> --entry <fn> \
           --realization-plugins plugins/<library>-harness-plan/plugin.toml

   The first must fail with the abstention above (if it plans, the plugin is not needed and is
   never consulted -- check 10). The second must print `OK ... origin=plugin <name>@<version>`.
3. Build and replay (the cell pipeline does the same: `scripts/c2r_funnel.py --realization-plugins`):

       python3 tools/stu_selector/gen_diff_harness.py --pair benchmark/pairs/rq4/<pair> --entry <fn> \
           --rust-entry <fn> --plan --ub-free --c-source <unit.c> \
           --realization-plugins plugins/<library>-harness-plan/plugin.toml \
           --plan-json plan.json --out <dir>
       # pair-level packaging (module re-exports, static strip) as in scripts/c2r_funnel.py:fixups
       (cd <dir> && CARGO_TARGET_DIR=<somewhere on /home> RUSTUP_TOOLCHAIN=nightly-2025-09-01 cargo fuzz build)
       C2R_MODE=combined  <bin> -runs=1 <valid input>      # expect C2R_OUTCOME kind=normal phase=5
       C2R_MODE=c-only    <bin> -runs=1 <valid input>
       C2R_MODE=rust-only <bin> -runs=1 <valid input>
       C2R_MODE=combined  <bin> corpus/ -max_total_time=60  # then replay corpus/ + artifacts

   `plan.json` carries the `origin` record: plugin name, version, manifest sha256, both
   representations, every selected view, the hooks with source locations, the ten validation
   results, the plugin-owned assumptions and every rejected alternative.
4. Run `python3 scripts/realization_plugin_test.py` (loader + planner behaviour on the reference
   plugin and on the invalid sample) and `python3 scripts/gen_harness_regression.py` (the
   reference harnesses are golden entries; no-plugin entries must stay byte-identical).

## What the core checks (rejects with `construction unsupported: check N: ...`)

| # | check | how |
|---|-------|-----|
| 1 | C target parameter is `<T>*` with the declared constness; layout | libclang: kind, `sizeof`/`alignof` (storage is 8-byte aligned; larger alignment is refused) |
| 2 | Rust target parameter is the declared view of `<T>`; all-zero bytes are a valid `<T>` | normalised signature vs. the view's pattern; recursive field check of the translated struct (scalars, raw pointers, `Option<_>`, arrays, translation structs) |
| 3 | initializer and cleanup exist on both sides; `[requires]` satisfied; an initializer is declared | C definition index; public `fn` in the translation |
| 4 | lifecycle signatures accept the declared views | C: `void f(<T>*)` exactly, constness vs. view; Rust: one parameter matching the view, unit return |
| 5 | the target view neither clones, leaks nor narrows | every vocabulary view is non-owning; the resource is not decoded from fuzz bytes |
| 6 | initializer inputs are constructible | v1: single-argument lifecycle functions only |
| 7 | initialization is deterministic | C call graph: no `rand`/effectful reach (or the producer bridge's libc re-seed rule) |
| 8 | cleanup cannot consume/free the resource before the target | cleanup is emitted after the target (FREE phase); neither target nor lifecycle function passes the pointer itself to `free`/`realloc` (depth-1 escape facts) |
| 9 | conservative BodyFacts field check | every top-level field the target touches (depth-1 callees included) is written by the initializer or has its address handed to a callee there; the callee-established fields are recorded as a plugin-owned assumption |
| 10 | no override of a usable generic plan | consulted only after the producer search abstained; the abstention is recorded |

Structural errors are rejected when the manifest is **loaded**, before any planning or emission:
unknown `kind` (a comparator manifest), an unknown key, a non-identifier value, a view outside the
closed vocabulary (`invalid_sample.toml` demonstrates this), `storage` other than `stack`,
several Rust bindings without distinct `binding` labels, a bound function missing from `[requires]`.

## What stays the plugin author's obligation (recorded as `assumptions` in the plan)

- the two representations denote the **same logical resource** (the core checks shapes, not meaning);
- the initializer **establishes the invariant** the target relies on -- the core shows the target's
  fields are written or handed to callees by the initializer, not that those callees are correct;
- the cleanup releases only what the initializer/target allocated, never the object's own storage;
- both sides are given zero-filled storage before the initializer runs (a harness choice, stated in
  the plan); an initializer that relies on garbage to detect "already initialised" is not a v1 fit.

## What a plugin cannot do (there is no place in a manifest for it)

Call or replace the target; inspect one side to build the other; suppress a panic, signal, timeout,
sanitizer report or comparison; alter the ladder or confirmation; bypass a translated constructor
that is itself under test; fix semantic values that narrow the input domain. A translation whose
initializer is stubbed, whose representation has no zero value (`Box`, `Vec`, references), or whose
state needs a callback/file/object graph is `construction unsupported` -- stop (plan section 8).

## Where realization shows up

- plan: `origin.origin == "plugin"`; the input spec's `c_decoder` is `realized_resource`, its
  `rust_bridge` is `view:<target view>`;
- harness: `let mut <p>_c: [u64; N]` (C storage) and `let mut <p>_r: translated::<T> = zeroed()`,
  then `c2r_phase(C2R_PH_PRODUCER)` -> initializer (C side inside the UB gate) -> target ->
  `c2r_phase(C2R_PH_FREE)` -> cleanup, in every mode (`combined`, `c-only`, `rust-only`);
- funnel (`scripts/c2r_funnel.py`): `planned_by = "plugin"` and a separate `planned (plugin)` count;
  results obtained this way are **plugin-assisted** and never replace the automatic results.
