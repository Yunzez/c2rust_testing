# ptr-to-array support (P3 nested-pointer step 1) — 2026-06-23

First of the three nested-pointer input models (kept separate per review). Adds support for
**pointer-to-fixed-array** parameters like `graph_dfs`'s `const size_t (*edges)[2]`.

## What it does

- `parse_entry_signature` recognizes a pointer whose pointee is a `CONSTANTARRAY` and records
  `{kind: ptr_array, elem, elem_w, inner_extent, const}` (libclang: POINTER → CONSTANTARRAY,
  element_type `const size_t`, element_count 2).
- New schema role **`input_fixed_array_buffer`** (`decode: fixed_array_vector`) with explicit
  `inner_extent` and `length_param`. `harness_schema.derive` produces it; `validate_against_signature`
  checks elem / elem_width / **inner_extent** / const.
- Layout = **contiguous flat storage**, identical on both sides: the harness builds a
  `Vec<[usize; 2]>` and passes `as_ptr()` → `*const [usize; 2]` (exactly c2rust's translation).
  All three decoders (differential harness, C-only `T (*)[N]`, Rust-only) consume the same bytes
  (1 count byte %64, then count×inner elements) and use the same flat layout.

graph_dfs schema derives cleanly: `n:scalar, edges:input_fixed_array_buffer(extent 2), m:length`.
Generated extern: `fn c_count_reachable(n: usize, edges: *const [usize; 2], m: usize) -> i64`.

## Verified

- 44/44 unit tests (incl. ptr-to-array: items mapping, `*const [usize; 2]` decl, ABI-order call,
  graph_dfs schema role/extent).
- 15/15 harness byte-identity regression — the **12 stay byte-identical**, glob/kv reviewed, and
  **graph_dfs added as a new-capability golden**. So ptr-to-array did not perturb existing output.
- graph_dfs **builds** and **runs** end-to-end (harness feeds the ptr-to-array correctly).

## Honest G1 caveat (a real finding, orthogonal to ptr-to-array)

graph_dfs's G1 is dominated by its **unbounded `n`** (vertex count): a raw 8-byte fuzz value makes
the adjacency-list allocation `malloc(n · …)` explode → AddressSanitizer `allocation-size-too-big`,
and with `allocator_may_return_null=1` → OOM. The classifier conservatively labels the OOM artifact
**UNKNOWN** (no UBSan signal, not a clean crash attribution) — correct: it is **not** a translation
divergence, it is resource exhaustion from an unbounded size input, shared by C and Rust.

This motivates a future schema annotation for **size-like scalars** (e.g. a `max_value` / bounded
domain for `n`) so the generic harness doesn't feed allocation sizes that trivially exhaust memory.
That is out of scope for ptr-to-array and tracked for later.

## Next

Review whether `input_fixed_array_buffer` (flat, inner_extent) is the right reusable abstraction
before T** (matrix_reduce `int**`, word_tokens `char**`) — which is a different model (pointer
table + separate backing allocations), then callback binding.
