# bzip2 × Laertes: uncalled static-table init → corrupt CRC (checksum-corruption class, 2nd instance)

**A second, independent instance of the crc32 headline class** (checksum silently broken by translation)
— now in a *different* tool (Laertes, the rule-based OOPSLA'21 lifter) with a *cleaner, systematic*
mechanism than C2SaferRust's crc32.

## Mechanism (fully localized)

Laertes lowers C's global const-array initializers into a **runtime init function** but **never emits
the call**:

```rust
// base c2rust (faithful): the table is a compile-time static initializer
pub static mut BZ2_crc32Table: [UInt32; 256] = [0, 0x4c11db7, 0x9823b6e, /* ...256 real values... */];

// Laertes: zeroed static + an init fn that is DEFINED BUT NEVER CALLED
pub static mut BZ2_crc32Table: [c_uint; 256] = [0,0,0, /* ...all 256 zeros... */];
unsafe fn laertes_init_BZ2_crc32Table() { BZ2_crc32Table = [0, 0x4c11db7, /* real values */]; }
```

**38 `laertes_init_*` functions are defined across the crate; total call sites: 0.** Every global that C
initialized with a non-trivial initializer — `BZ2_crc32Table`, `BZ2_rNums` (randomization table), and the
rest — is left **all-zero at runtime**. For bzip2 this degenerates the CRC recurrence
`blockCRC = (blockCRC<<8) ^ table[(blockCRC>>24)^byte]` (the table term is always 0), so every compressed
stream carries a **wrong CRC**.

## Effect (silent — BZ_OK returned)

`BZ2_bzBuffToBuffCompress` returns **BZ_OK**; the Huffman-coded payload is byte-identical to C, but the
32-bit block CRC and combined CRC are wrong. Canonical `bunzip2 -t` → *"data integrity (CRC) error"*.
Example (`"A"` → archived): C writes CRC `19 93 9b 6b`; Laertes writes `00 00 00 ff`
(`c_correct_a.bz2` vs `laertes_corrupt_a.bz2`).

## Quantification (canonical-bunzip2 oracle)

100-record sample: **9 valid / 91 CRC-error / 0 other-corrupt / 0 crash** (the 9 valid are degenerate
tiny/empty inputs). Larger inputs additionally hit the zeroed `BZ2_rNums`/blocksort tables and can crash.
**91% of inputs → silently corrupt (CRC-invalid) compressed output.**

## Attribution (airtight)
base c2rust compress is **byte-identical to C** (0 diffs / 76 records; `'A'`/rec examples exact) — its
`BZ2_crc32Table` is a working static initializer. Laertes rewrote it into the broken zero+uncalled-init
form. C-side ASan/UBSan clean (established in `../bzip2_crown/`). → the divergence localizes entirely to
**Laertes's static-initializer lowering**.

## Why it matters
- **Second independent tool** exhibiting the checksum-corruption class (after C2SaferRust crc32) — the
  class is not tool-specific; it is a recurring failure mode of C→Rust translation of checksum code.
- **Systematic, not a one-off**: it is a *lowering* defect (const-initializer → uncalled runtime init)
  that zeroes **every** non-trivially-initialized global — a whole category of silent bugs from one root
  cause, invisible to "does it compile / is it less unsafe" checks.
- **Compiles + reduces unsafe + is runtime-wrong**: exactly the gap the project targets — a published
  translation that passes the tool's own success criteria yet is not semantics-preserving.

## Files
- `crctable_zeroed_static.rs` — the zeroed static + the never-called init fn
- `c_correct_a.bz2` / `laertes_corrupt_a.bz2` — C's valid vs Laertes's CRC-corrupt output for `"A"`
- Drivers/oracle reused from `../bzip2_crown/` (same corpus, same canonical-bunzip2 roundtrip oracle)

## Master-table cell
`bzip2 / Laertes` → **s:1** (systematic silent CRC corruption; crashes on some larger inputs from the
same zeroed-table root cause).
