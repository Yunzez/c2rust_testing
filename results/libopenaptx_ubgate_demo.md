# libopenaptx head-to-head: auto-generated harness + UB-gate demonstration (2026-06-29)

First results running OUR pipeline on the head-to-head corpus (RustAssure/Fluorine's libopenaptx,
GPT-4o translations). Validates (a) harness AUTO-generation on a new program (no hand-writing), and
(b) the UB-correct divergence counting that Fluorine/RustAssure lack.

## Infra fix (LibAFL → real libFuzzer)

`gen_diff_harness.py` previously emitted `libafl_libfuzzer`, which in this environment **ignores
`-max_total_time`/`-timeout` and hangs indefinitely on some inputs** (burned hours, escaped process-group
kills). Switched the generated `fuzz/Cargo.toml` to real `libfuzzer-sys = "0.4"` and changed the C-oracle
coverage from `trace-pc-guard` (rejected by modern libFuzzer) to `inline-8bit-counters,pc-table,trace-cmp`
(matches the Rust side). Now libFuzzer honors stop conditions and runs at ~500k exec/s. (User will report
the LibAFL `-max_total_time` bug upstream.)

## Methodology validation — clip (pure scalar)

`gen_diff_harness.py` auto-inferred 3 scalar params from the C signature and emitted the differential
cargo-fuzz project; fixed `expose_entry` to `pub`-ify idiomatic plain `fn` (was c2rust `extern "C"` only).
Result: **13,045,411 runs / 26 s, DONE, 0 crashes — CLEAN** (C `clip` ≡ GPT-4o-Rust `clip`). Harness was
generated, not written.

## UB-gate demonstration — sign_extend

`int32_t sign_extend(int32_t val, unsigned bits)` computes `shift = 32 - bits` then `(uint32_t)val <<
shift`. For `bits == 0`, `shift == 32`, so `val << 32` is **undefined behavior in C** (shift ≥ width) —
an out-of-contract input. GPT-4o's Rust translation panics there (`attempt to shift left with overflow`).

| run | result |
|---|---|
| **gate ON** (`--ub-free`) | 13,685,298 runs / 26 s, **CLEAN** — the UB input is rejected (C UBSan flags it) before Rust is called; on all UB-free inputs (bits ∈ 1..32) C ≡ Rust |
| **gate OFF** | **DIVERGENCE**: Rust panics on `bits=0`; libFuzzer reports a deadly signal + crash artifact |

**Interpretation:** the `bits=0` divergence is NOT a translation bug — the input is undefined in C, so the
caller must never pass it. A differential oracle WITHOUT UB discipline (Fluorine, RustAssure — neither
filters UB; RustAssure explicitly lists memory-corruption divergence as a false-positive source) would
report it as a bug = **false positive**. Our in-loop UB-free gate correctly excludes it and reports CLEAN.

This is a concrete, reproducible instance of our differentiator on a real, published LLM translation from
the competitors' own benchmark.

## Reproduce

```sh
# pairs under tools/headtohead/libopenaptx/<fn>_gpt4o/{source,translated,build}
python3 tools/stu_selector/gen_diff_harness.py --pair tools/headtohead/libopenaptx/sign_extend_gpt4o \
        --entry sign_extend --expose-entry --ub-free --infer-schema
cd fuzz_gen/sign_extend_gpt4o && cargo +nightly-2025-09-01 fuzz run sign_extend_gpt4o_ft -- -max_total_time=25
# gate OFF: regenerate without --ub-free --out fuzz_gen/sign_extend_gpt4o_nogate -> panics on bits=0
```

## Next

- Scalar functions done (clip clean, sign_extend = UB-gate demo). Remaining libopenaptx functions with
  pointer/struct params (aptx_bin_search `&Box<[i32]>`, aptx_check_parity `&[AptxChannel; N]`) need the
  C-ABI bridge extension to the generator (task B) — drive marshalling from matched io-shapes.
- Then cross-reference divergences against RustAssure's `bug_description.csv` (reproduce their 16) + apply
  the gate to re-examine which are UB-driven.
