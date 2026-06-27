# LLM-translation fidelity — motivation probe v1

Point the differential pipeline at **real LLM translations** (not injected bugs) to see
whether an LLM translator produces problems on its own — the motivation for differential
testing of non-faithful (renaming/idiomatic) translators. Model: **gpt-5-mini**, the same
10 translations as the matcher baseline (`experiments/llm_transpiler/out/`).

## Finding 1 — compilation: 2 of 10 do not even compile

| program | compiles? | defect |
|---|---|---|
| hex_encode, base64, bignum, leb128, rpn_eval, linked_list, hash_table, opcode_dispatch | ✅ (8/10) | — |
| **rle_codec** | ❌ | tuple element order/type confusion: helper returns `([u8;2], usize)` but the caller binds `(usize, [u8;2])` → `pair[0]` indexes a `usize`, `in_idx += consumed` adds an array (E0308/E0608/E0277) |
| **tinyexpr** | ❌ | missing lifetime specifier (E0106) ×4 |

**20% of gpt-5-mini translations fail to compile.** Note our name-independent matcher still
*paired* these (rust-analyzer is error-tolerant) — it is the differential pipeline's build
step that catches the non-compiling ones. (A weaker/cheaper model would raise this rate;
see "next".)

## Finding 2 — differential (compiling ones): gpt-5-mini is faithful on small functions

Bridged + fuzzed under the in-loop UB-free gate:

| program | result |
|---|---|
| hex_encode | NO_DIVERGENCE (CLEAN) |
| leb128 | NO_DIVERGENCE (CLEAN) |

gpt-5-mini correctly preserved semantics (incl. the leb128 error-return convention: C entry
and the LLM both return -1 on decode/encode failure). On *small* functions a strong model is
genuinely faithful — so a compelling **semantic-divergence** motivation needs either a
weaker/cheaper model (realistic: practitioners use them) or larger/more complex functions.

## Method — the C-ABI bridge shim (matcher → harness, the key piece)

The LLM emits **idiomatic** Rust (`encode_hex_lowercase(&[u8], &mut [u8]) -> usize`) whose
signature does NOT match the C ABI (`hex_encode(*u8, len, *u8, cap) -> size_t`). Instead of
changing the harness generator, append a tiny `#[no_mangle] extern "C"` **shim** to the LLM
crate that exposes the matched function under the C entry name with the C ABI, unpacking
pointers into slices:
```rust
#[no_mangle] pub unsafe extern "C" fn hex_encode(src:*const u8,len:usize,dst:*mut u8,cap:usize)->usize{
    let s = if len==0 {&[]} else {core::slice::from_raw_parts(src,len)};
    let d = if cap==0 {&mut []} else {core::slice::from_raw_parts_mut(dst,cap)};
    encode_hex_lowercase(s, d)          // the matcher gives this C-name ↔ Rust-name pair
}
```
Then `gen_diff_harness.py --pair <llm-diff-pair> --entry hex_encode --infer-schema --ub-free`
works unchanged (C oracle vs `translated::hex_encode` = the shim). The C↔Rust name + io-shape
correspondence comes from the matcher; today the shim is hand-written, later auto-generated
from the matcher's io-shapes. Hard cases (Option/Result/error-code, `&str`, structs) need
richer bridges — deferred.

Recipe: `experiments/llm_transpiler/diff/<name>/` = {source/ (orig C), translated/ (LLM
lib.rs + shim), build/compile_commands.json}; gitignored (derived from out/).

## Next
- **Model gradient (recommended):** re-translate the corpus with a cheaper model
  (gpt-5-nano / gpt-4o-mini) → expect a higher non-compile rate AND real semantic
  divergences. A "cheaper model → more translation bugs" table is stronger motivation than a
  single strong model, and exercises the pipeline on genuinely buggy translations.
- Bridge the harder compiling cases (base64, rpn_eval) for more semantic coverage.
- Auto-generate the bridge shim from the matcher's io-shape correspondence.
