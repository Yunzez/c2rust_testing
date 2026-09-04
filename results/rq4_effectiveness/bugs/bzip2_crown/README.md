# bzip2 × CROWN: safety-lifter produces corrupt output + memory-unsafe code

**A published *safety* lifter (CROWN) translates bzip2 into code that is wrong or memory-unsafe on 71%
of inputs** — the irony is the headline: CROWN's entire purpose is to lift c2rust output into *safer*
Rust, yet here it *introduces* memory corruption and silent data corruption that the mechanical c2rust
baseline does not have.

## Attribution (airtight, same-source differential)

| implementation | compress | decompress (small=0, default) | decompress (small=1) |
|---|---|---|---|
| original C (bzip2 1.0.8) | correct | correct | correct |
| **base c2rust** (mechanical) | **== C byte-exact** | **correct** | correct |
| **CROWN** (safety lift of that same base) | **29% correct / 46% corrupt / 25% heap-crash** | **BZ_DATA_ERROR (-4) on valid data** | correct |

base c2rust is byte-identical to C on both directions → the faithful reference. CROWN is a lift of that
**same** c2rust base, so every divergence localizes to **CROWN's ownership/slice rewrite**, not to c2rust
and not to the original C. C-side ASan+UBSan on the full corpus: **0 reports** (all divergences are on
UB-free inputs).

## The three bugs (all in CROWN-declared-success code)

### 1. Corrupt compressed output — SILENT (semantic, Priority-0)
`BZ2_bzBuffToBuffCompress` returns **BZ_OK** but writes a byte-stream that the canonical `bunzip2`
rejects with *"Data integrity error"*. Example: input `"A"*4096`, blockSize100k=1 → CROWN emits 54 bytes
(C/base emit 46); canonical `bunzip2` on CROWN's 54 bytes → integrity error, 0 bytes out. The caller
believes compression succeeded and stores corrupt data — no crash, no error code. **This is the
silent-wrong-output class** (`crown_corrupt_A4096.bz2` archived).

### 2. Heap corruption — memory-unsafe (crash)
On ~25% of inputs `BZ2_bzBuffToBuffCompress` triggers `free(): invalid next size (normal)` (glibc heap
corruption abort). Memory unsafety introduced by a *safety* lifter.

### 3. Decompress default path broken (semantic)
`BZ2_bzBuffToBuffDecompress` with `small=0` (the **default** fast path) returns **BZ_DATA_ERROR** on
valid bzip2 data (canonical-`bunzip2`-verified valid); `small=1` (low-memory path) works. So CROWN's
rewrite broke the default decompress state machine. base c2rust decompresses correctly on both paths.

## Quantification (150-record sample, canonical-bunzip2 roundtrip oracle)

- compress: **43 roundtrip-OK / 69 corrupt-output / 38 heap-crash** (0 CROWN-reported errors — the
  corruption is entirely silent or via memory abort, never a clean error return)
- decompress small=0: fails on every valid stream tested (returns -4)

## Method
- Oracle: original bzip2 1.0.8 C (`BZ2_bzBuffToBuffCompress/Decompress`). Correctness oracle for
  compress = **canonical system `bunzip2`** roundtrip (independent third implementation).
- Drivers: `crown_compress_driver.rs`, `crown_decompress_driver.rs` (bins in `bzip2_crown/`, built with
  nightly-2023-01-26 + `darwin_shims.c` for the macOS ctype/stderr symbols the macOS-transpiled crate
  references — `_DefaultRuneLocale`, `__maskrune`, `__stderrp`, `__assert_rtn`).
- Corpus: 1608 records (empty/tiny/runs/all-same/alternating/binary + random), blockSize ∈ {1,5,9},
  workFactor ∈ {0,30,100,250}. Per-record process isolation (heap corruption would otherwise kill batch).
- UB gate: C oracle rebuilt with ASan+UBSan, full corpus, **0 reports**.

## Files
- `oracle_comp.c` — C compress oracle
- `crown_compress_driver.rs`, `crown_decompress_driver.rs` — CROWN drivers
- `darwin_shims.c` — macOS symbol shims for the transpiled crate
- `crown_corrupt_A4096.bz2` — a CROWN "successful" compress output that bunzip2 rejects
- Scratch: `scratchpad/bz_crown_diff/`

## Master-table cell
`bzip2 / CROWN` → **c:1 s:2** (heap-corruption crash; corrupt-compress + broken-decompress semantic).
