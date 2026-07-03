# HEADLINE semantic-difference bug — C2SaferRust `crc32_z` (optipng), NULL-vs-empty conflation

**This is the project's first confirmed SEMANTIC DIFFERENCE** (Priority-0 class per
`results/PROJECT_RESET_2026-07-03.md`): both sides terminate, C is UB-free, and the outputs differ.
It is the *only* bug class fuzzing the Rust translation **alone** cannot find — the buggy Rust returns a
plausible wrong value with **no crash and no panic**, so it is invisible without the C-side oracle.
This is the raison d'être of differential testing for C→Rust, demonstrated on a **published** tool.

## The bug

`crc32_z(crc, buf, len)` — incremental CRC-32 (zlib, as vendored by optipng). The CRC-of-no-data
invariant is `crc32_z(crc, buf, 0) == crc` (a zero-length chunk must leave the running CRC unchanged);
zlib only returns 0 when the **pointer is NULL**.

| | guard | result on `(crc=X≠0, valid buf, len=0)` |
|---|---|---|
| **base** (faithful c2rust, `optipng/src/zlib/crc32.rs:2074`) | `if buf.is_null() { return 0 }` | **X** (correct) |
| **WIP** (C2SaferRust, `optipng_WIP/src/zlib/crc32.rs:2085`) | `if buf.is_empty() { return 0 }` | **0** (wrong) |

When C2SaferRust lifted the raw pointer `buf: *const u8` to a safe slice `buf: &[u8]`, it rewrote the
original NULL check `buf == Z_NULL` into `buf.is_empty()`. But a **null pointer** and a **valid,
zero-length buffer** are different things. The original returns 0 only for NULL; for a valid empty
buffer it returns the running `crc` unchanged. The lifted version returns 0 for *any* zero-length
input, so an incremental/streaming CRC that ever processes a zero-length chunk (an empty read, a final
flush with no bytes, or a plain `crc32(crc, buf, 0)` query) is **silently reset to 0** — data-integrity
corruption with no crash.

This is exactly a **structure-non-preserving translation** (pointer → slice) introducing a
value-divergence that only differential testing against the source can catch.

## Evidence (empirical, not by inspection)

`src/main.rs` builds BOTH the shipped base and WIP `crc32_z` (extracted verbatim, type aliases stubbed)
and compares them:

- **Direct cases**: `crc32_z(0x12345678, "", 0)` → base `0x12345678`, WIP `0x00000000`. (crc=0 is the
  one value where they coincide, because 0 is its own "reset".)
- **Sweep**: 1,000,000 random trials, buffer lengths 0..300 (fully exercising the base's byte-aligned
  *and* BYFOUR fast paths), mixed seed CRCs → **1,664 divergences, every single one at len==0.**
  On **every non-empty buffer the two agree exactly**: the table-index rewrite C2SaferRust applied is
  otherwise faithful; the *sole* semantic difference is the empty-buffer NULL/empty conflation.

```
cd results/rq1_bugs/crc32_c2saferrust && cargo +nightly-2025-09-01 run --release
```

## Classification (3-class taxonomy)

- **Class #1 — semantic difference.** ✅ C UB-free (no deref on empty; base's `is_null` is false for a
  valid pointer, loops skip), both terminate, `output(base) ≠ output(WIP)`.
- Attribution: same-source base c2rust = faithful C behavior (returns crc); divergence localizes to
  C2SaferRust's `is_null → is_empty` rewrite. Not class #2 (no UB) and not class #3 (no crash).

## Why single-program fuzzing cannot find this

Fuzzing the WIP alone: `crc32_z(X, "", 0)` returns `0` and does not crash — it looks like a perfectly
valid CRC result. There is no panic, no UB, no signal. Only comparison against the source's value (X)
reveals it is wrong. This is the class the whole project exists to find.

## Files
- `src/base.rs`, `src/wip.rs` — the two shipped `crc32_z` implementations (verbatim, cross-module type
  aliases stubbed to concrete `c_*` types; `#[no_mangle]` stripped to co-link; combine fns truncated).
- `src/main.rs` — differential driver (direct cases + 1M sweep).
- `source_excerpts.txt` — the two decisive guards in context, with file:line.
- Provenance: C2SaferRust published artifacts, `tools/frameworks/c2saferrust/laertes_benchmarks/`
  (`optipng` = base c2rust, `optipng_WIP` = C2SaferRust safety-lift). Upstream: vikramnitin9/c2saferrust.
