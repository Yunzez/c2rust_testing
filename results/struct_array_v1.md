# T* + len array modeling — struct-array buffers — 2026-06-23

Addresses the audit's generator-mis-modeling finding (`results/audit_v1.md`): a `T* + size`
**array** parameter was modeled as a *single* element, producing a spurious OOB crash that was
(correctly) audited down to `excluded_generator`. This adds proper **struct-array buffer**
construction so such boundaries get the right label.

## What it does

- `classify`: a POD-struct pointer **followed by a `size_t` length** is now an array of that many
  structs (role `inout_struct_array` / `input_struct_array`), e.g.
  `ht_insert_into(Slot *slots, size_t cap, …)`. The `size_t` requirement disambiguates from the
  single-struct case: `op_add(VM *vm, uint8_t operand)` keeps `vm` a single struct (operand is not a
  length), and `vm_pop(VM *vm)` (no following scalar) stays single.
- Decode (all 3 decoders byte-identical): read `len` (one byte, bounded ≤64), build
  `Vec<translated::Slot>` of `len` structs (each field decoded in declaration order via the existing
  POD-struct literal), pass `as_mut_ptr()` → `*mut translated::Slot`. Mutable arrays are compared
  **element-wise, field-wise** after the call (c2rust structs don't derive `PartialEq`, so
  `Vec<T> != Vec<T>` is unavailable).
- Full 9 touchpoints + `harness_schema` (ROLES/DECODES/_ROLE_DECODE/derive/validate/
  validate_against_signature) + `classify_artifact` (C driver + Rust-only + ABI call).

## Why ht_insert_into is now correctly VALID (not invalid)

Once `slots` is sized to `cap`, the function is **memory-safe for any cap**: the probe loop is
`for (probe = 0; probe < cap; probe++)` and indexing is `idx & (cap-1) ≤ cap-1 < cap` (the mask
bounds the index regardless of power-of-two). The previous "invalid (C_CRASH)" was purely our
single-element mis-modeling. With the fix: **403,605 executions, 0 artifacts → NO_DIVERGENCE_OBSERVED
(valid).** The C and Rust drivers in `classify_artifact` also compile and run on the struct-array.

## Scope: 1 of the 3 audit-flagged boundaries fixed cleanly

- **`hash_table::ht_insert_into`** — fixed (struct-array). Recovers as valid.
- **`mergesort_search::merge_runs` / `msort_range`** — NOT this shape. They are `int *a, int *tmp,
  size_t lo, mid, hi`: two buffers sliced by three index params with ordering invariants
  (`a` ≥ hi, `tmp` ≥ hi-lo, lo ≤ mid ≤ hi). This is not `T*+len` and cannot be soundly inferred by
  adjacency; it needs an explicit **sliced-buffer schema** (future work). They remain
  `excluded_generator` in the frozen dataset, which is still accurate.

## Frozen dataset v1 is NOT mutated

This is a forward-looking generator capability: the frozen `dataset/boundaries.jsonl` (v1) keeps
`ht_insert_into` as `excluded_generator` (it was harvested before the fix). A **re-harvest (v2)**
will pick up the fix and relabel it valid; v2 is the right place to apply it (alongside any external
corpus), not a retro-edit of v1.

## Verified

- 95/95 unit tests (+8 struct-array: items mapping, `*mut translated::T` decl, per-side ABI call,
  Vec-sized-by-len decode, element-wise compare, validate incl. missing length_param, real-signature
  `Slot* + size_t → io_struct_arr`).
- 17/17 harness byte-identity regression unchanged (struct-array perturbs no existing harness).
- `harness_schema --all --check`: 0 drift, 0 invalid.
- ht_insert_into builds + fuzzes (403k exec, valid); classify_artifact C/Rust drivers compile + run.

## Next

`merge_runs`/`msort_range` sliced-buffer schema, then either callback binding or — higher value —
scale the harvester to external c2rust-clean programs for dataset v2 (the real lever for a solid
negative class), then the `P(valid|x_f)` grouped-CV baseline.
