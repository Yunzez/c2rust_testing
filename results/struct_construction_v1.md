# POD struct-by-pointer construction + boundary census — 2026-06-23

The boundary census (Stage A of the harvester) revealed that the discriminative signal for a
learned `P(valid|x_f)` lives in **struct-pointer boundaries**, not the easy scalar/buffer ones
(which are nearly all valid). This step adds **POD struct-by-pointer construction** to the
generator — the single richest source of *both* validity labels — and stands up the static census.

## Boundary census (Stage A) — `scripts/harvest_census.py`

Unit = one matched C↔Rust function (a candidate STU boundary), not a program. Pure libclang +
`features.py`, **no build/fuzz**. Output: `dataset/boundaries_static.jsonl`, `results/boundary_census_v1.md`.

- **85 candidate boundaries** across 18 programs (4.7× the 18 program-entries).
- **59 constructible** (69%) after adding struct support — 15 already `pub`, 44 are `static`
  internals exposable via `#[no_mangle] pub` (+ dropping `static` on the C side for link).
- **26 hard-gated**, with precise reasons: `struct_ptr_field` 11 (invariant-bearing structs —
  `DynArray{int* data; len; cap}`, `HashTable{Slot* slots; …}`), `callback` 3, `struct_value`/`array_value` 2.

## What "POD struct" construction does

- `describe_type` now recurses into `RECORD` types → a struct descriptor (fields in declaration
  order) with a **POD verdict**: POD = every field is a scalar or fixed array of scalars. A pointer
  / nested-struct / union / fn-ptr field makes it non-POD, and we keep the **precise reason** — a
  strong static feature ("input struct holds a pointer" ⇒ far more likely an invalid boundary).
- A `T*`/`const T*` to a POD struct → role `inout_struct`/`input_struct`. The harness decodes one
  struct value field-by-field (same bytes on all three decoders), gives each side its own `Copy`,
  passes `&mut`/`&` → `*mut translated::T` / `*const translated::T`, and compares **field-wise**
  after (`a.f != b.f || …`; scalars and `[T;N]` both impl `PartialEq`, so no derive needed).
- Non-POD structs stay gated with the precise reason (the negative-candidate set).

## Harvester plumbing (for Stage B)

- `gen_diff_harness.py`: `--ignore-schema` (force inference; the on-disk schema is keyed to the
  program's canonical entry, not the boundary being harvested) and `--expose-entry` (make a private
  `static` boundary callable: `#[no_mangle] pub` on the Rust side, strip `static` on the C side so
  the renamed `c_<entry>` symbol links).
- `classify_artifact.py`: same struct decode (C driver + Rust-only) + `--ignore-schema` +
  `--crate-dir` (point at the exposed per-boundary project).

## The key result (the whole thesis in one boundary)

`opcode_dispatch::vm_pop(VM*)` — `VM = {int32 stack[64]; int32 sp}` is **POD-constructible**, the
harness builds and runs — yet the boundary is **INVALID**: `sp` carries a value-invariant that
random construction violates, so `stack[--sp]` reads out of bounds. The pipeline auto-labels it
**C_UB_CONFIRMED** (`index 774713228 out of bounds`; Rust-only panics). A `ctx_t`-style struct with
no index invariant would stay VALID. So **POD-constructible ≠ valid** — exactly the discriminative,
*natural* negative the dataset needs, no injection required.

## Verified

- 83/83 unit tests (+12 struct: descriptor POD verdict, `io_struct` classify, `*mut translated::T`
  decl, per-side `&mut`, declaration-order literal + two copies, field-wise compare, derive→validate
  round-trip byte-identical to inference, non-POD precise gate).
- 17/17 harness byte-identity regression unchanged (struct support perturbs no existing harness).
- `harness_schema --all` derive+validate clean on all 17 entries (no clobber).
- vm_pop end-to-end: harness builds + runs + crashes; `classify_artifact` → C_UB_CONFIRMED.

## Next

**Stage B — the validity-labeled dataset.** Generalize `run_g1_matrix` over the 59 constructible
boundaries (exposing `static` internals): build + short fuzz + classify each → NO_DIVERGENCE_OBSERVED vs
FALSE_DIVERGENCE (C-UB / crash) vs … joined with `features.py` → `dataset/boundaries.jsonl`. Expect a
natural mix (e.g. `ctx_t`→VALID, `vm_*`→C_UB) — the first balanced training set. Then per-program
grouped-CV baseline for `P(valid|x_f)`; later G3 refactors (positive robustness) + injected negatives.
