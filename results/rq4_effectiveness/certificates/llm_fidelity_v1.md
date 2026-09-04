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

## Finding 3 — model gradient (gpt-5-nano): NOT monotone; small functions stay faithful

Re-translated the same 10 with **gpt-5-nano** (`out_nano/`):
- **Compile: 9/10** (only `bignum` fails — borrow checker E0502). NOTE this is *better* than
  gpt-5-mini's 8/10, and the failing program differs → **compile-failure is per-program noise,
  NOT monotone with model strength.** "Cheaper model → more compile failures" does not hold.
- **Differential (bridged): hex_encode, rle_codec, leb128 all CLEAN** under the UB-free gate —
  including `rle_codec`, which gpt-5-mini failed to compile but gpt-5-nano translated
  *correctly*. Even the cheaper model is semantically faithful on these small functions.

## Honest conclusion
- **On small, self-contained functions, both models translate C→Rust semantically faithfully.**
  **0 silent behavioral divergences in 6 bridged differential tests** across 2 models
  (hex_encode mini+nano, leb128 mini+nano, rle_codec nano, rpn_eval mini) — including the
  harder `rpn_eval` bridge with error codes faithfully aligned (Rust `RpnError` discriminants
  match C's `RPN_ERR_*` 1:1, so the comparison is fair on both result value and error code).
  The only defects are **compile failures** (caught by rustc), not silent semantic bugs.
- This is an honest negative for "differential testing catches silent LLM bugs" *on this
  corpus*: the functions are too small/simple to mistranslate semantically. The likely place
  for a real semantic divergence is **larger/stateful/complex functions** and the **harder
  bridges not yet tested** (rpn_eval `Result`/error-code, base64 edge cases, bignum structs) —
  exactly where error conventions and edge cases differ.

## Next (decision pending)
- **Bigger / stateful functions** (lil interpreter, real libraries) — most likely to surface a
  genuine semantic divergence; the small-function corpus is a poor hunting ground.
- **Harder bridges** (rpn_eval Result→status+out, base64) — semantic-mismatch-prone by design.
- Compile-failure rate (mini 2/10, nano 1/10) is a real but weaker motivation (compiler
  already catches it); the differential value needs a *silent* divergence, still to be found.
- Auto-generate the bridge shim from the matcher's io-shape correspondence.
