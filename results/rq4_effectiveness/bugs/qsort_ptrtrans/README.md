# qsort × PtrTrans (FSE'26, gpt-5.1, Trans_PA): the sort that doesn't sort

**Verdict: `s:1`** — semantic-difference bug, headline-class. On the 30-line, 3-function textbook
quicksort, PtrTrans produces a translation that **compiles cleanly, never panics, passes PtrTrans's own
cargo-check verification gate — and does not sort**.

## Numbers
- **34,012 / 50,000 = 68.02% divergence** on random arrays (sizes 0–256, mixed extremes/dups/negatives),
  C reference under ASan+UBSan for the whole batch (UB gate clean: every diff is on UB-free input).
- **All 34,012 diverging outputs are not even sorted** (not a stability/tie-order artifact).
- The 32% agreeing cases are the trivial ones (n ≤ 1, and small-n coincidences).
- Minimal repro (n=5): C sorts `[3,1,2,5,4]` → `1 2 3 4 5`; PtrTrans Rust → `2 5 3 1 4`.
  Even n=3 diverges: `[43, INT_MIN, 94]` → RS `94 INT_MIN 43`.

## Mechanism: index arithmetic error in the `split_at_mut` reshaping
PtrTrans reshapes `int arr[]` → `Option<&mut [i32]>` and rewrites C's `swap(&arr[i], &arr[j])` through
`split_at_mut` to satisfy the borrow checker (can't take two `&mut` into one slice directly):

```rust
let (left, right) = arr.split_at_mut(j_usize.max(i_usize)); // split at j (when i <= j)
a_ref = left.get_mut(i_usize);            // element i  — correct
b_ref = right.get_mut(j_usize - i_usize); // element j + (j-i) = 2j-i — WRONG (should be right[0])
```

`right[0]` *is* element j; the correct second index is `0`. The generated `j - i` addresses element
`2j − i` instead — partition swaps the **wrong element**. Same bug repeated in the post-loop
`swap(&arr[i+1], &arr[high])` block (`right.get_mut(high - ip1)` → element `2·high − (i+1)`).

**The silencer:** when `2j − i` falls outside the slice, `get_mut` returns `None` — and the reshaped
`swap(Option, Option)` was *defensively designed to no-op on None*. So the wrong-index swap either
corrupts a far element or silently does nothing. No panic, no bounds crash, ever. The defensive
None-handling that makes the translation "safe" is exactly what makes the bug invisible.

## Why this is the thesis in miniature
- Fuzzing the Rust alone finds nothing: no crash, no UB, no panic — just wrong values.
- PtrTrans's own success bar (compiles + cargo check) passes this translation.
- One concrete differential execution against the C original exposes it on ~2/3 of all inputs.
- It is the same *call-site/reshaping contract* bug class as cJSON's `parse_string` empty-slice
  (headline #2) — the ptr→slice lift changes index/bound semantics at the rewrite boundary — but here
  distilled to 30 LOC with a one-line root cause.

## Method note
qsort is NOT in PtrTrans's shipped crown_dataset — we extended the corpus (same as bzip2): added
`crown_dataset/qsort/` (qsort.c + compile_commands.json), generated SVF reports with our compiled
`pa_func`/`pa_struct` on clang-14 IR, ran the full Trans_PA pipeline (gpt-5.1). All 3 units translated;
crate compiles (quickSort needed 1 repair round). Differential: batched 50k-case driver pair
(C: `c_batch.c` with ASan/UBSan; Rust: `rust_batch_driver.rs` wrapping the translation verbatim).
Do not say "PtrTrans claimed qsort" — this is our corpus extension of their method.

## Files
- `original_qsort.c` — the 30-line C source
- `translated_qsort.rs` — PtrTrans output, verbatim (see the two `split_at_mut` blocks)
- `c_batch.c`, `rust_batch_driver.rs`, `gen_and_diff.py` — the differential harness
- `minimal_repro_input.txt` — `[3,1,2,5,4]`
- Full generated crate: `tools/frameworks/ptrtrans_rebuild/PtrTrans-C2Rust/Code_Package/dataset/PA_trans_projects/qsort/`
