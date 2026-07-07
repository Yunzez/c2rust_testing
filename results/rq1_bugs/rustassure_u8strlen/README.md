# Semantic diff #6 — RustAssure (published) u8strlen, gpt-3.5-turbo backend

**Second published tool with a ready artifact** (RustAssure: "Differential Symbolic Testing for
LLM-Transpiled C-to-Rust Code", Palit, UC Davis). Its shipped artifact translates a UTF-8 library
(`u8c`) with multiple backend models. We differential-test the shipped translations against the C at $0.

## The bug (gpt-3.5-turbo translation of `int u8strlen(const char *s)`)

C counts UTF-8 codepoints by counting non-continuation bytes over the raw byte string:
`while (*s) { if ((*s & 0xC0) != 0x80) len++; s++; }`.

gpt-3.5-turbo produced: `s.chars().filter(|c| (*c as u8 & 0xC0) != 0x80).count()`. This iterates over
DECODED `char`s (codepoints), then truncates each codepoint to its low byte (`c as u8`) and applies the
byte-level continuation test — which no longer means anything. Any codepoint whose low 8 bits fall in
`0x80..=0xBF` is wrongly dropped from the count.

| input | C | gpt-3.5-turbo | gpt-4o |
|-------|---|---------------|--------|
| "©"   | 1 | **0** | 1 |
| "©©©" | 3 | **0** | 3 |
| "日本語" | 3 | **2** | 3 |
| "hello €world" | 12 | **11** | 12 |

Exhaustive over single codepoints U+0020..U+FFFF: **gpt-3.5-turbo 15,877 diffs; gpt-4o 0 diffs.**
All on VALID UTF-8 (so the `&str` domain-narrowing does not gate them), C is UB-free, no crash — a
silent wrong count. Confirmed empirically (`cargo run --release`).

## Model-quality spectrum WITHIN one published tool

Same function, RustAssure's own artifact, different backends:
- **gpt-3.5-turbo**: BUG (`.chars()` + low-byte truncation).
- **gpt-4o**: faithful (`s.as_bytes()` byte loop).
- **gpt-4o-mini**: faithful (raw-pointer byte loop).

## Methodological note
RustAssure's contribution is a *symbolic* (KLEE + edit-distance) similarity check. Concrete differential
fuzzing over the real multi-byte UTF-8 input space surfaces this divergence directly; whether RustAssure's
symbolic pass over a `&str` argument explores multi-byte sequences is exactly the coverage question our
method sidesteps. (We do not claim RustAssure's check passed it — no per-function verdict is shipped in
the artifact; we claim its produced translation is semantically wrong and our method proves it.)

Provenance: tools/frameworks/rustassure/src/python/inputs-complex/u8c/ (u8c.c + per-model archives).

## Bug #7 (same artifact, gpt-3.5-turbo) — u8next_ never decodes continuation bytes

`int u8next_(const char *txt, int *ch)` fully decodes one UTF-8 codepoint (lead byte masked, then
continuation bytes shifted in). gpt-3.5-turbo's translation identifies the byte `len` correctly but
`break`s after the first byte in every match arm — it **never reads the continuation bytes**, so `*ch`
is just the masked lead byte. "©"→2 (not 169), "€"→2 (not 8364), "😀"→0 (not 128512). Exhaustive over
single codepoints U+0020..U+10FFFF (against the shipped preprocessed C `.i` as oracle): **1,111,936 /
1,112,032 diverge** — every multi-byte codepoint. gpt-4o's translation carries the full state machine.
Repro: `u8next_/` (`cargo run --release`).
