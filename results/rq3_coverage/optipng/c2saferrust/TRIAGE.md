# optipng × C2SaferRust — root-cause triage of the confirmed sample (2026-09-09)

The cell's 200-per-boundary sample confirmed 1 199 rows on 11 boundaries (after the 2026-09-09
re-classification: 771 `confirmed_termination` + 397 `confirmed_divergence`, 31 moved to
`instrument_only`). Rows count inputs, not defects. Every boundary was read to its root cause in the
translation (`benchmark/pairs/rq4/optipng_c2saferrust/translated/optipng_c2saferrust.rs`) against the
pair's C (`source/zlib/*`, `source/libpng/pngwutil.c`, `source/pngxtern/*`); panic sites come from the
archived no-sanitizer replays (harness `lib.rs` line → pair line).

| boundary | rows (term. / div.) | panic site / diff | root cause | manifest |
|---|---:|---|---|---|
| `crc32` | 0 / 26 | return value | `crc32` hands its buffer to `crc32_z`; `is_null → is_empty` resets the CRC on an empty chunk | **S1 re-found** |
| `compress2` | 169 / 99 | `adler32_z` :62239 `index out of bounds`; output buffer | `adler32_z`'s 16-byte block loop builds a slice of the REMAINING length after `len -= 16` and reads 8 entries: out of bounds when fewer than 8 remain, wrong sums otherwise (the trailer of the deflate stream is the Adler-32) | **S2 re-found** |
| `compress` | 1 / 1 | same site; output buffer | same `adler32_z` rewrite | S2 |
| `adler32` | 1 / 1 | same site; return value | same `adler32_z` rewrite | S2 |
| `compress` | 200 / – | `compress_block` :85968 `attempt to shift left with overflow` | `send_bits` flush: C shifts a ush promoted to int by `bi_valid` (may be 16); Rust shifts a `u16` by 16. Debug: panic; release: masked shift ORs `val` into the flushed short — corrupted stream | **S19** (new) |
| `compress2` | 28 / – | same site | same | S19 |
| `uncompress` | 169 div | written length `destLen` | inflate MATCH: `if (state->offset > copy)` emitted as `if state as usize > copy` (the state pointer's address vs the byte count) → window branch on almost every match → `invalid distance too far back` on valid streams | **S20** (new) |
| `uncompress` | 4 / – | `inflate` :81147 null pointer dereference | same wrong branch with a wrapped copy count of 0 and no window: the byte loop reads NULL | S20 |
| `uncompress` | 48 / – | `inflate_fast` :75626 `unsafe precondition(s) violated` (from_raw_parts non-null) | the 'copy direct from output' branch lost `from = out - dist`; `from` stays NULL from its declaration (:75492) | **C15** (new) |
| `uncompress` | 31 → `instrument_only` | ASan build only (deadly signal; no-sanitizer replay normal) | the branch's overlapping `copy_from_slice` memcpy, which ASan's interceptor rejects; not a termination difference | none (instrument) |
| `crc32_combine`, `crc32_combine64` | 0 / 23 each | return value | `gf2_matrix_square` uses the OUTPUT array as the matrix (`gf2_matrix_times(square, mat[n])` for C's `(mat, mat[n])`) and the zeros-operator loop swaps `even`/`odd`. The 201 other sampled inputs per boundary are `ub_associated`: crc1 above 32 bits overflows C's own 32-entry matrix (stack-buffer-overflow in C) | **S21** (new) |
| `optimize_cmf` | 200 / – | :47856 `attempt to subtract with overflow` | `--z_cinfo` on unsigned 0: **defined wrap in C**, and a Rust release build wraps identically (u32); only the overflow check differs. Reachable only from a malformed CMF byte (CINFO 0). | **not a defect** — overflow-check artifact |
| `bmp_memset_bytes` | 117 / – | :58722 `unsafe precondition(s) violated` (`ptr::offset`) | harness input-model gap: `memset(ptr + offset, ch, len)` — the planner does not model libc sinks, `offset` is an unbounded scalar; both sides compute an out-of-bounds pointer (C silently under our UBSan set, Rust's debug precondition traps) | **not a defect** — out-of-contract input; planner rule recorded |
| `opng_free` | 3 / – | panic on `Box::from_raw(NULL)` | `free(ptr)` → `drop(Box::from_raw(ptr))` | **C13** |
| `opng_strcasecmp` | 0 / 55 | return value | byte-wise `tolower` compare → lossy UTF-8 decode | **S16** |

**Count.** 1 199 confirmed rows → **6 root causes catalogued** (S1, S2 re-found; S19, S20, C15, S21 new;
plus C13 and S16 promoted on 2026-09-09 from the same sample) and **2 non-defects** (an overflow check on
a C-defined wrap; a harness input-model gap). Nothing in the sample is left unexplained.

**Reading notes.**
- S19's `bi_valid == 16` arises whenever the non-flush path fills the bit buffer exactly
  (`bi_valid += len` reaching 16); the next `send_bits` takes the flush path with a shift of 16. UBSan's
  `shift` check is silent on the C side because `(ush)val` is promoted to `int`.
- S20's comparison reads, verbatim, `if state as *const _ as usize > copy as usize { // Compare the
  pointer value with copy` — the translator commented its own error.
- C15: no assignment to `from` exists between its declaration (`0 as *mut c_uchar`) and the slice
  construction; the window-copy branch above it is the only place `from` is set, and that branch
  `break`s before reaching the direct copy.
- S21's C-side overflow (201 rows) is zlib's own: `gf2_matrix_times` walks `mat` once per set bit of a
  64-bit `unsigned long`, and `even`/`odd` have 32 entries. Inputs with a 32-bit crc1 are the ones that
  compare the algorithms; they all diverge.
