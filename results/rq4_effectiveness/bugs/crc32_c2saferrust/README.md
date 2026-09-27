# HEADLINE semantic-difference bug — C2SaferRust `crc32_z` (optipng), NULL-vs-empty conflation

**This is the project's first confirmed SEMANTIC DIFFERENCE** (Priority-0 class per
`results/archive/PROJECT_RESET_2026-07-03.md`): both sides terminate, C is UB-free, and the outputs differ.
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
cd results/rq4_effectiveness/bugs/crc32_c2saferrust && cargo +nightly-2025-09-01 run --release
```

## Multi-tool contrast — same function, three tools, only one is wrong

The `laertes_benchmarks/` corpus ships outputs from **multiple published translators on the same C
source**: `optipng` (base c2rust) / `optipng_WIP` (**C2SaferRust**, LLM-based) / `optipng_laertes`
(**Laertes**, Emre et al. OOPSLA'21, rule-based pointer-lifting). Running the same `crc32_z(crc, buf, 0)`
through all three (`three_way()` in `src/main.rs`):

```
input crc    SOURCE(base)  C2SaferRust      Laertes
0x12345678   0x12345678    0x00000000 [BUG] 0x12345678 [ok]
0xdeadbeef   0xdeadbeef    0x00000000 [BUG] 0xdeadbeef [ok]
```

**Same source function, three tools: C2SaferRust is silently wrong, Laertes is faithful.** Laertes (a
principled, conservative lifter) left `crc32_z` as raw-pointer c2rust code with the original `is_null`
guard — byte-identical to base; C2SaferRust (aggressive LLM rewrite) lifted it to a slice and broke the
NULL/empty distinction. This is the multi-tool payoff of differential testing: **it tells you which
tool's output you can trust on which function** — a spectrum from aggressive-LLM (bugs) to
principled-lifter (faithful), placed with evidence rather than assumed. Single-program fuzzing of any one
output cannot produce this comparison (no oracle).

## Attribution — C2SaferRust marked this translation SUCCESS

C2SaferRust's own `optipng_WIP/log.txt` records the outcome of each rewrite chunk:

```
...::src::zlib::crc32::crc32_z)  Chunk root, Success, 3     <-- the buggy function, marked SUCCESS
...::src::zlib::crc32::crc32)    Chunk root, Failure, 5
```

So the tool **believed `crc32_z` was correctly translated and shipped it as a success** — its own
verification never exercised the `(nonzero crc, empty buffer)` case. This is the strongest form of
attribution: the defect is a **silent failure the tool did not detect**, not a known-bad output. (It
mirrors the SACTOR / C2SaferRust theme that sample-based verification misses the untested edge space.)

## Corroboration — the SAME empty-input bug in the sibling checksum `adler32_z`

zlib's other checksum, `adler32_z`, is broken the same way in the same crate
(`optipng_WIP/src/zlib/adler32.rs:48`): the base's `if buf.is_null() { return 1 }` became
`if len == 0 { return 1 }`, so a running Adler-32 hitting a zero-length chunk is **reset to the seed
`1`** instead of preserved (base returns `adler`; WIP returns `1`; confirmed empirically, e.g.
`adler32_z(0x12345678, buf, 0)` → base `0x12345678`, WIP `0x00000001`). This makes the empty-input
checksum-reset a **systematic C2SaferRust pattern across both zlib checksum lifts**, not a one-off.
(Caveat: unlike `crc32_z`, `adler32_z` was marked `Failure` in the log and is *additionally* grossly
miscompiled on non-empty input — wrong sums + an out-of-bounds panic — so it is a messier example;
`crc32_z` remains the clean, otherwise-faithful headline.)

## Classification (3-class taxonomy)

- **Class #1 — semantic difference.** ✅ C UB-free (no deref on empty; base's `is_null` is false for a
  valid pointer, loops skip), both terminate, `output(base) ≠ output(WIP)`.
- Attribution: same-source base c2rust = faithful C behavior (returns crc); divergence localizes to
  C2SaferRust's `is_null → is_empty` rewrite. Not class #2 (no UB) and not class #3 (no crash).

## End-to-end reachability — the bug corrupts real PNG output

`crc32_z` is not an unused API corner: optipng accumulates the **IDAT chunk CRC incrementally** through
it. In `optipng_WIP/src/optipng/optim.rs`:

```
1581:  crt_idat_crc = crc32(0, sig_IDAT.as_ptr(), 4) ...   // seed CRC with the "IDAT" tag
1612:  crt_idat_crc = crc32(crt_idat_crc, data, length) ... // fold in each output write segment
       ... png_save_uint_32(buf, crt_idat_crc) ...          // then this CRC is written into the file
```

Incremental CRC is **segmentation-invariant by design** — splitting the same bytes across more/fewer
write callbacks must not change the result. The WIP bug breaks exactly that: **any write segment with
`length == 0`** (a zero-byte flush from libpng, or a spec-legal empty IDAT split) **resets the running
CRC to 0**, so the CRC written into the PNG is wrong and the file fails CRC validation.

`idat_demo()` in `src/main.rs` reproduces the accumulation with the shipped base and WIP `crc32_z`:

```
segmented [5,5]   : base=0xe221bc33 wip=0xe221bc33  ok
segmented [5,0,5] : base=0xe221bc33 wip=0xb2a113e5  *** WRONG CRC WRITTEN TO PNG ***
```

Same payload, one extra zero-length segment: the correct (base) CRC is unchanged; the WIP CRC changes →
a corrupt IDAT. (Caveat: this drives the real accumulation *loop* with the shipped primitives; a
full-binary demonstration — building optipng_WIP and finding an input where libpng emits a zero-length
IDAT write — is future work. The defect in the primitive is certain; this shows it reaches the file's
CRC through the actual code path.)

## The real pipeline finds it (end-to-end validation)

Beyond the hand-written module comparison, the actual DUET pipeline finds this silent bug automatically.
`tools/stu_selector/gen_oop_harness.py --pair <crc pair> --entry crc32_z` builds a UBSan/ASan C oracle
(faithful zlib `crc32_z`, NULL-guarded) + a cargo-fuzz crate calling the C2SaferRust `crc32_z` natively,
and compares **return values**. Running `cargo fuzz run`:

```
divergence: C="ret:10" Rust="ret:0"
```

The fuzzer hits it near-instantly; the minimized trigger is a **single byte `0x0a`** → crc=10, empty
buffer. It is caught by value comparison, not a crash (the Rust returns a clean, well-typed `0`) — the
defining property of a silent semantic diff. Scratch: `scratchpad/h_crc_pipeline/`.

Harness gap noted: C2SaferRust's `crc32_z` keeps BOTH a slice `&[u8]` AND a redundant explicit
`len: usize`; the generator assumed the slice absorbed the length and emitted a 2-arg call, patched to
pass `len` (a "slice + redundant length parameter" shape to handle in the generator).

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
