# bzip2 × C2SaferRust: `mmed3` — median-of-three rewritten as minimum

**Semantic computation substitution.** The C helper computes the *median* of three bytes; the
translation computes their *minimum*. The two agree on 0.59 % of the input domain.

## The two implementations

C — `tools/frameworks/crown/c-code/bzip2/blocksort.c:583` (`excerpt_c.c`), bzip2 1.0.8. The name
says what it is: **m**edian of **3**.

```c
static __inline__ UChar mmed3 ( UChar a, UChar b, UChar c )
{
   UChar t;
   if (a > b) { t = a; a = b; b = t; };
   if (b > c) {
      b = c;
      if (a > b) b = a;
   }
   return b;                       /* median */
}
```

C2SaferRust — `fuzz/bzip2_wip_e3/src/blocksort.rs:822` (== the shipped
`laertes_benchmarks/bzip2_WIP/blocksort.rs`), `excerpt_rust.rs`:

```rust
 fn mmed3(a: u8, b: u8, c: u8) -> u8 {
    let mut min = a;
    if b < min { min = b; }
    if c < min { min = c; }
    min                            // minimum, not median
}
```

The rewrite kept the signature and the name and replaced the computation.

## Correctness unit and strength: exhaustive, per invocation

The unit is **one invocation of `mmed3`**, and the input domain is finite and small enough to
enumerate completely. `mmed3_census.c` runs both bodies over every `(u8, u8, u8)` triple:

```
exhaustive u8^3: 16777216 triples, 16679040 differ (99.41%)
example: a=0 b=1 c=2 -> C median 1, C2SaferRust min 0
```

The 0.59 % that agree are the triples whose median happens to equal their minimum.

**This count is not comparable with the `divergent / valid records` column of the other manifest
rows.** Those are per-OBS-cell record counts on one fuzzing seed (unit: one library input record);
this is an exhaustive enumeration of one function's own input domain (unit: one invocation). The two
units must never be pooled (manifest rule U1).

## Independent detection by the automatic differential harness

Found by the schema-driven differential harness generated for this boundary, not by inspection:
`raw/differential_harness.rs` (generator 0.6, schema `raw/schema_mmed3.json`, C oracle under the
in-loop UBSan gate). In a 600 s campaign it reported **13,602** `divergence: return value` events;
`raw/crash-71853c…` is one saved input, and replaying it reproduces:

```
panicked at fuzz_targets/bzip2_c2saferrust_ft.rs:71:33: divergence: return value
```

The boundary takes three scalars and no pointers, so no input-model artifact can explain the
divergence.

## Downstream effect: bounded, and deliberately not overstated

`mmed3` has **exactly one call site**, in both the C and the translation
(`excerpt_callsite_c.c`, `excerpt_callsite_rust.rs`): `mainQSort3` uses the returned byte as the
**pivot value** for its three-way partition.

- Established: the helper returns the wrong value on 99.41 % of its input domain, and the wrong
  value flows into pivot selection.
- Expected consequence: worse pivots, i.e. partition balance and therefore sorting work, with
  bzip2's own `budget` mechanism falling back to `fallbackSort` when the budget is exhausted.
- **Not established, and not claimed: that this changes bzip2's compressed output.** Both sorters
  produce a correct order, so an output difference is not expected — but it was not demonstrated
  either way, because this artifact cannot be run end to end: its CLI aborts earlier in
  `sendMTFValues` (see candidate CAND-3).

Any statement about this defect should therefore be: *an internal helper returns a wrong value on
99.41 % of inputs; the known downstream effect is pivot selection and potential performance
degradation.*

## Provenance: `base-c2rust-as-reference`

C2SaferRust is a **Rust-to-Rust** rewriter, and both ends of the rewrite it performed are in the
repository, so the defect can be pinned to the rewrite step itself without needing the original `.c`:

| | file | `mmed3` returns |
|---|---|---|
| C2SaferRust's **input** | `tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/blocksort.rs:884` (base c2rust) | **median** — a line-for-line transliteration of the C, temp-swap included (`excerpt_base_c2rust.rs`) |
| C2SaferRust's **output** | `tools/frameworks/c2saferrust/laertes_benchmarks/bzip2_WIP/blocksort.rs:822` | **minimum** (`excerpt_rust.rs`) |

The base translation is correct; the rewrite is not. This holds regardless of whether the original
`blocksort.c` was byte-identical to the in-repo copy (`bzip2_WIP` ships no `.c`), which is why this
row is `base-c2rust-as-reference` rather than `unknown`. The C is quoted for context only, from
`tools/frameworks/crown/c-code/bzip2/blocksort.c:583` (bzip2 1.0.8). sha256 of all three files:
`raw/source_hashes.txt`.

## Files

`mmed3_census.c` + `census_output.txt` (exhaustive enumeration), `excerpt_c.c` / `excerpt_base_c2rust.rs` / `excerpt_rust.rs`
(the C, C2SaferRust's input, and C2SaferRust's output), `excerpt_callsite_c.c` / `excerpt_callsite_rust.rs` (the single call site),
`raw/` (the generated differential harness, its schema, a saved divergence-triggering input, the
campaign's last libFuzzer line, source hashes).
