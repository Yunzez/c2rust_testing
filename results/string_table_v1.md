# T** step 2 — string-pointer table (char** words) — 2026-06-23

Third nested-pointer model. The `char**` case is kept **distinct** from the rectangular
`int**` table because its inner backings are independent, NUL-terminated, and per-item
different lengths — they cannot be flattened into a `rows × cols` rectangle. Entry:
`uint64_t fold_unique_words(char **words, size_t count)` → c2rust `*mut *mut i8`.

## What it does

- `describe_type` already yields `pointer → pointer → scalar i8`; `_param_from_descriptor` turns
  that into `kind: ptr_ptr`. `classify` now disambiguates by the trailing scalars: a `ptr_ptr`
  followed by **two** scalars ⇒ rectangular table (rows, cols); followed by **one** scalar ⇒ the
  new **string-pointer table** (`count`). (Previously the one-scalar case raised "not yet supported".)
- New role **`input_string_pointer_table`** (`decode: string_pointer_table`) with `length_param`
  (count), `elem`/`elem_width`, a bounded `count_max` (default 16 — one decoded byte, drives the
  table size), and the v1 `mutation: "backing_observable"` contract.
- **Layout = pointer table over independent NUL-terminated backings.** The harness decodes `count`
  (one byte, bounded), then for each string reads a `take_vec_i8` (1-byte length %64, then that many
  bytes) and pushes a trailing `0` — exactly the `input_string` decode, repeated `count` times into a
  `Vec<Vec<i8>>`. It clones the backing per side, builds a `Vec<*mut i8>` pointer table per side, and
  passes `table.as_mut_ptr()` → `*mut *mut i8` (c2rust's translation). All three decoders
  (differential, C-only `char**`, Rust-only) consume identical bytes. After the call the two backings
  are compared, so any in-place mutation divergence is caught even though `fold_unique_words` is
  read-only.

word_tokens schema: `words:input_string_pointer_table, count:length`. Generated extern:
`fn c_fold_unique_words(words: *mut *mut i8, count: usize) -> u64`.

## Reuse (distinct role, shared machinery)

Shares the **canonical type descriptor**, **ABI-order** call/signature generation, the per-item
**NUL-string decode** (`take_vec_i8` + push 0, same bytes as `input_string`), the dual-backing
compare pattern from the rectangular table, and the **3-way shared byte-decode spec**. The schema
role is separate from both `input_string` (single string) and `input_rectangular_pointer_table`
(rectangular `T**`).

## Verified

- 71/71 unit tests (+10 char**: classify one-scalar→string-table, items mapping, `*mut *mut i8`
  decl, per-side ABI call, count-then-NUL-strings decode, backing compare, validation incl.
  count_max>255 and missing-mutation, `validate_against_signature` accepts the new ptr_ptr role).
- 17/17 harness byte-identity regression — the 12 stay byte-identical; word_tokens added as a
  new-capability golden (alongside graph_dfs, matrix_reduce). No perturbation of existing output.
- Independent C-only (`char**` driver, UBSan+ASan) and Rust-only drivers both compile and run on a
  crafted multi-word input — no UB, no panic — confirming the classify_artifact decoders agree with
  the harness byte-for-byte.
- word_tokens **builds and runs**: 84,720 executions in 30 s, **0 objectives →
  NO_DIVERGENCE_OBSERVED** (c2rust translated the char** string table faithfully).

## Next

Only the callback / fn-ptr program (array_map_reduce `map_then_reduce`) remains
UNSUPPORTED_SIGNATURE. After callback binding: G3 semantics-preserving refactors → G2 injected
bugs → learned `P(valid|x_f)` with per-program grouped CV.
