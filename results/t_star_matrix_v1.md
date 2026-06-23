# T** step 1 — rectangular pointer table (matrix int**) — 2026-06-23

Second nested-pointer model (kept separate per review; **not** a reuse of the flat
`input_fixed_array_buffer`). Adds `int **` matrix parameters like
`matrix_transpose_and_sum(int **mat, size_t rows, size_t cols)`.

## What it does

- The unified `describe_type` already yields `pointer → pointer → scalar i32`; `_param_from_descriptor`
  turns that into `kind: ptr_ptr`. `classify` pairs a `ptr_ptr` followed by **two** scalars into the
  rectangular table (one following scalar ⇒ the `char**` string-table model, deferred).
- New role **`input_rectangular_pointer_table`** (`decode: rectangular_pointer_table`) with
  `outer_length_param` (rows), `inner_length_param` (cols), `elem`, and bounded `outer_max`/`inner_max`
  (default 16 — dimensions drive the rows×cols allocation, so they are bounded; cf. bounded_scalar).
- **Layout = pointer table over independent per-row backing.** The harness decodes rows/cols (each one
  byte, bounded), reads `rows×cols` elements **once** into `Vec<Vec<i32>>`, clones it per side, builds a
  `Vec<*mut i32>` row-pointer table per side, and passes `table.as_mut_ptr()` → `*mut *mut i32` (exactly
  c2rust's translation). All three decoders (differential, C-only `int**`, Rust-only) consume the same
  bytes and use the same layout. After the call the two backings are compared (catches any mutation
  divergence).

matrix_reduce schema: `mat:input_rectangular_pointer_table, rows:length, cols:length`. Generated
extern: `fn c_matrix_transpose_and_sum(mat: *mut *mut i32, rows: usize, cols: usize) -> i64`.

## Reuse (not the fixed-array role)

Shares exactly what the review asked: the **canonical type descriptor**, **ABI-order** call/signature
generation, **bounded_scalar** (for rows/cols), and the **3-way shared byte-decode spec**. The schema
semantics are distinct from `input_fixed_array_buffer`.

## Verified

- 59/59 unit tests (+T**: items mapping, `*mut *mut` decl, per-side ABI call, dims-then-data decode,
  validation incl. missing outer_max).
- 16/16 harness byte-identity regression — the 12 stay byte-identical; matrix_reduce added as a
  new-capability golden (graph_dfs likewise). No perturbation of existing output.
- matrix_reduce **builds and runs**: 368,100 executions in 30 s, **0 objectives → NO_DIVERGENCE_OBSERVED**
  (c2rust translated the int** matrix faithfully).

## Next

Review the rectangular-table abstraction, then `input_string_pointer_table` for `char**`
(word_tokens: pointer table over independent NUL-terminated backings — `ptr_ptr` + one length scalar),
then callback binding (array_map_reduce fn-ptr).
