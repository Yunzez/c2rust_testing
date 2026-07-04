# RQ3 human-port candidates: small C libs with independent idiomatic Rust reimplementations

For the RQ3 "human port" row — (C, hand-written idiomatic Rust reimpl, hand-labeled C↔Rust map).
Goal: genuine human renaming/restructuring with recoverable function correspondence.

## Key tension (matters for selection)
**The easiest pairs to hand-label are the most LITERAL translations (keep names, 1:1) — but those are
the WEAKEST test of a name-independent matcher.** The pairs that actually rename/method-ify (tinyexpr,
QOI, RustCrypto trait-wrapping) are slightly harder to label but are the REAL test. → report idiomaticity
per pair so we can't be accused of cherry-picking literal pairs.

## Ranked candidates (feasible-to-label first)

| C lib | C LOC/#fn | Rust reimpl | idiomatic? | correspondence | license | label |
|---|---|---|---|---|---|---|
| xoshiro/xoroshiro | ~160 / 3 | rand_xoshiro | idiomatic traits | ~100% | CC0 / MIT-Apache | Easy (tiny, few fns) |
| PCG (pcg-c-basic) | ~98 / 6 | rand_pcg | idiomatic | ~80-90% | Apache-MIT / MIT-Apache | Easy |
| **tinyexpr** | ~600 / ~29 | **tinyexpr-rs** (kondrak) | mild (Result, structs; keeps recursive-descent fns) | ~80-90% near-1:1 | Zlib / MIT-Apache | **Easy** |
| TweetNaCl | ~700 / ~25 | tweetnacl (sodalite) | LOW (literal translit) | ~95-100% | PD / MIT-Apache | Easy (but weak test) |
| jsmn | ~470 / ~6 | jsmn (95th) | struct methods | ~90% | MIT / MIT | Easy (⚠ avoid jsmn-rs = binding) |
| **heatshrink** | ~1000 / ~10 | **embedded-heatshrink** | faithful+idiomatic streaming | ~90% | ISC / MIT-Apache | **Easy** |
| MurmurHash3 | ~335 / ~7 | murmur3 (stusmall) | one-shot API | ~90% | PD / MIT-Apache | Easy-Med |
| LEB128 (LLVM hdr) | ~257 / 4 | leb128 (gimli) | idiomatic traits | ~100% | Apache / MIT-Apache | Easy (⚠ C anchor is C++ hdr) |
| ChaCha20 (ref) | ~200 / ~5 | chacha20 (RustCrypto) | trait-wrapped, core 1:1 | ~90% core | PD / MIT-Apache | Easy |
| MD5 | ~300 / ~5 | md-5 (RustCrypto) | traits; transform↔compress | ~80-90% | RFC / MIT-Apache | Easy-Med |
| **QOI** | ~500 / 4-6 | **qoi-rust** (aldanor) | **heavy: free-fns→struct methods** | conceptual 1→many | MIT / MIT-Apache | **Medium (best restructuring test)** |
| SipHash | ~185 / 1+macros | siphasher | decomposed to streaming Hasher | ~100% logic, 1→many | CC0 / MIT-Apache | Medium |
| CRC-32/IEEE | ~1200 / ~12 | crc32fast | idiomatic | ~60-80% | Zlib / MIT-Apache | Med |
| SHA-256/1 | ~300 / ~4 | sha2/sha1 (RustCrypto) | compress 1:1, update/final refactored | ~80-90% core | PD / MIT-Apache | Medium |
| cJSON | ~3000 / 78 | cjson-rs (ThomasJenkinson, clean-room) | public API by name; internals reimpl'd | ~100% public, 60-80% internal | MIT / MIT | Medium (largest good one) |
| xxHash | ~8-10k / 40-55 | twox-hash / xxhash-rust | streaming→Hasher | ~60-80% (XXH3 hard) | BSD / MIT/BSL | Med-Hard |
| BLAKE3 | C+Rust same repo | blake3 (official) | parallel modules, SIMD | ~85-95% portable | CC0/Apache | Medium (same authors, 2 langs) |

## Recommended start (3 + 1)
1. **tinyexpr → tinyexpr-rs** — sweet spot: ~29 fns (real eval set), structure mirrors C yet idiomatic.
   **BONUS: tinyexpr is ALSO one of our raw-LLM seeds** → same C, two renamers (LLM vs human) = a
   controlled within-program comparison.
2. **heatshrink → embedded-heatshrink** — non-crypto/non-parser streaming algo (~10 fns, ~90%), corpus
   diversity.
3. **QOI → qoi-rust** — the hard test: free-functions become struct methods, mapping is conceptual not
   name-based (exactly what name-based oracles can't do).
+ first validate the labeling workflow on a tiny one: **xoshiro256 / PCG** (label in minutes).

## On-topic bonus (c2rust framing)
**lodepng → lodepng-rust** (kornelski): pure-Rust reimpl that was produced by the **Citrus C→Rust
converter then refactored idiomatically** — i.e. the transpile-then-cleanup workflow. Larger (~6k LOC C),
correspondence "degraded over time" (method-ified) → Med-Hard, but the single most on-topic
machine-converted-then-humanized artifact for our STU/c2rust story.

## AVOID
- **FFI bindings (not reimpl)**: sds crate, stb_image (servo), jsmn-rs, all Monocypher crates,
  cityhash-sys, crates.io cjson-rs (≠ the GitHub repo).
- **Total rewrites (no lineage)**: serde_json/json-rust (vs cJSON), csv-core (vs libcsv), url (servo),
  image/fontdue/lewton (vs stb_*), rust-ini (vs inih, push-vs-pull), tar (vs microtar), base64/hex crates
  (concept-only), httparse (re-architected from picohttpparser, ~40-55%).
- **Traceable but NOT idiomatic (good labels, weak test)**: stb_image_rust (auto-translated, mechanical),
  tweetnacl (manual literal) — use only as recall-ceiling controls, not rename-robustness tests.
