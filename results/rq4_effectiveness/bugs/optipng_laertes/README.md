# optipng × Laertes: zeroed zlib CRC table — THIRD checksum-corruption instance

The same failure mode as `../bzip2_laertes/` (headline #4), now in optipng's bundled **zlib**, and the
**third independent instance of the checksum-corruption class** across the project (after C2SaferRust
crc32 and Laertes bzip2). Notably it is the **same `crc32_z` function** that C2SaferRust broke a
*different* way (headline #1, empty-chunk reset) — one function, two tools, two distinct silent bugs.

## Mechanism (identical to bzip2_laertes)

optipng's zlib is compiled **without `DYNAMIC_CRC_TABLE`**, so `crc_table` is a **precomputed static
constant** read directly by `crc32_z` — there is NO runtime rebuild. Laertes lowered that static's
initializer into `laertes_init_crc_table()` and **never emitted the call**, leaving the table all-zero:

```rust
static mut crc_table: [[c_uint; 256]; 8] = [[0,0,0, ...all 8×256 zeros... ]];
unsafe fn laertes_init_crc_table() { crc_table = [[real values]]; }   // 0 call sites

pub unsafe extern "C" fn get_crc_table() -> *const c_uint {
    /* DYNAMIC_CRC_TABLE */          // ← comment only; no make_crc_table, no crc_table_empty guard
    return crc_table.as_ptr() ...;   // returns the zeroed static
}
```
`crc32_z` indexes `crc_table[0][(crc ^ byte) & 0xff]` directly; with the table all-zero the CRC
recurrence degenerates. base c2rust has the **real** table (`0x77073096, ...`) → faithful.

## Empirical confirmation (C-backed, UB-gated)

Driver calls the crate's `crc32(0, buf, len)` and `adler32(1, buf, len)`; oracle = **canonical system
zlib** (optipng's zlib is standard zlib; crc32/adler32 of a byte string are fixed values). Corpus:
200,006 byte strings (targeted + random, len 0–2000).

| pair | crc32 | adler32 |
|---|---|---|
| **C (system zlib) vs base c2rust** | **0 diffs** (faithful reference) | 0 diffs |
| **C vs Laertes** | **196,985 / 200,006 wrong (98.49%)** | **0 diffs** |

- The only crc agreements are len=0 (crc=0 trivially) + 11 coincidences. Examples: `crc32("a")` C=`e8b7be43`,
  Laertes=`ff000000`; `crc32("hello")` C=`3610a686`, Laertes=`ffffffff`.
- **adler32 is 100% faithful** — it is arithmetic (no table), so Laertes's zeroing cannot touch it. This
  cleanly isolates the bug to the table-driven CRC path.
- **ASan+UBSan on the C oracle, full corpus: 0 reports** — all divergences on UB-free inputs.
- Attribution airtight: base c2rust ≡ C byte-exact; Laertes zeroed the table → Laertes-introduced.

## Effect
`crc32`/`crc32_z` silently return wrong checksums (no crash, no error) on 98.5% of inputs. In optipng
this corrupts every PNG chunk CRC and the zlib/IDAT Adler-vs-CRC integrity — the caller believes the
checksum succeeded. Reachable through the whole PNG write/verify path.

## Files
- `oracle_zlib.c` — canonical system-zlib oracle (crc32 + adler32)
- `laertes_driver.rs` — driver ([[bin]] in optipng_laertes; identical one built in base optipng)
- `evidence_zeroed_crc_table.txt` — the zeroed static + uncalled init + no-rebuild `get_crc_table`
- Scratch: `scratchpad/optipng_lae_diff/`

## Master-table cell
`optipng / Laertes` → **s:1** (silent CRC corruption; systematic uncalled-init → zeroed zlib crc_table,
no runtime rebuild). Third independent checksum-corruption instance; same class as bzip2_laertes (#4).
