# Same-corpus C reach — lodepng_crown

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `lodepng.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 56 / 235 (0.238) | 462 / 3874 (0.119) |
| Rust (archived campaign) | 54 / 257 (0.210) | 1449 / 14332 (0.101) |

Inputs replayed on the C side: {'completed': 867, 'crash': 8, 'timeout': 5} over 54 / 54 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/lodepng__crown/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 172 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 43 | 2 (2) | 0 (0) | 127 | 63 | 0 (0) | 3 (1) |

`c_only`: lodepng_chunk_check_crc→lodepng_chunk_check_crc, lodepng_chunk_generate_crc→lodepng_chunk_generate_crc

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| Adam7_deinterlace | ok | 9 | {'completed': 8, 'crash': 1} | 2/235 | 33/3873 |
| Adam7_interlace | ok | 14 | {'timeout': 5, 'completed': 8, 'crash': 1} | 2/235 | 33/3873 |
| addColorBits | ok | 6 | {'completed': 6} | 1/235 | 7/3873 |
| addPaddingBits | ok | 22 | {'completed': 21, 'crash': 1} | 3/235 | 15/3874 |
| adler32 | ok | 12 | {'completed': 12} | 2/235 | 8/3873 |
| alloc_string | ok | 20 | {'completed': 20} | 5/235 | 11/3873 |
| alloc_string_sized | ok | 1 | {'completed': 1} | 3/235 | 7/3873 |
| checkColorValidity | ok | 16 | {'completed': 16} | 1/235 | 39/3873 |
| countZeros | ok | 1 | {'crash': 1} | 0/235 | 0/3873 |
| filterScanline | ok | 91 | {'completed': 91} | 2/235 | 47/3874 |
| getHash | ok | 18 | {'completed': 18} | 1/235 | 10/3873 |
| getNumColorChannels | ok | 6 | {'completed': 6} | 1/235 | 3/3873 |
| getValueRequiredBits | ok | 4 | {'completed': 4} | 1/235 | 10/3873 |
| ilog2 | ok | 8 | {'completed': 8} | 1/235 | 10/3873 |
| ilog2i | ok | 8 | {'completed': 8} | 2/235 | 14/3873 |
| isGrayICCProfile | ok | 27 | {'completed': 27} | 1/235 | 9/3873 |
| isRGBICCProfile | ok | 26 | {'completed': 26} | 1/235 | 9/3873 |
| lodepng_chunk_ancillary | ok | 6 | {'completed': 6} | 1/235 | 1/3874 |
| lodepng_chunk_check_crc | ok | 1 | {'completed': 1} | 4/235 | 9/3874 |
| lodepng_chunk_data | ok | 10 | {'completed': 10} | 1/235 | 1/3874 |
| lodepng_chunk_data_const | ok | 10 | {'completed': 10} | 1/235 | 1/3874 |
| lodepng_chunk_find | ok | 1 | {'completed': 1} | 7/235 | 36/3874 |
| lodepng_chunk_find_const | ok | 45 | {'completed': 45} | 7/235 | 46/3874 |
| lodepng_chunk_generate_crc | ok | 1 | {'completed': 1} | 5/235 | 8/3874 |
| lodepng_chunk_length | ok | 5 | {'completed': 5} | 2/235 | 2/3874 |
| lodepng_chunk_next | ok | 15 | {'completed': 15} | 4/235 | 30/3874 |
| lodepng_chunk_next_const | ok | 14 | {'completed': 14} | 4/235 | 22/3874 |
| lodepng_chunk_private | ok | 8 | {'completed': 8} | 1/235 | 1/3874 |
| lodepng_chunk_safetocopy | ok | 8 | {'completed': 8} | 1/235 | 1/3874 |
| lodepng_chunk_type | ok | 14 | {'completed': 14} | 1/235 | 4/3874 |
| lodepng_chunk_type_equals | ok | 18 | {'completed': 18} | 2/235 | 10/3874 |
| lodepng_crc32 | ok | 13 | {'completed': 13} | 1/235 | 4/3874 |
| lodepng_error_text | ok | 5 | {'completed': 5} | 1/235 | 3/3874 |
| lodepng_free | ok | 1 | {'completed': 1} | 1/235 | 0/3873 |
| lodepng_get_bpp_lct | ok | 9 | {'completed': 9} | 2/235 | 4/3873 |
| lodepng_get_raw_size_idat | ok | 13 | {'completed': 13} | 1/235 | 0/3873 |
| lodepng_get_raw_size_lct | ok | 17 | {'completed': 17} | 3/235 | 5/3873 |
| lodepng_gtofl | ok | 9 | {'completed': 9} | 2/235 | 3/3873 |
| lodepng_huffman_code_lengths | ok | 23 | {'completed': 23} | 4/235 | 21/3874 |
| lodepng_malloc | ok | 4 | {'completed': 4} | 1/235 | 0/3873 |
| lodepng_memcpy | ok | 1 | {'crash': 1} | 0/235 | 0/3874 |
| lodepng_memset | ok | 7 | {'completed': 6, 'crash': 1} | 1/235 | 2/3874 |
| lodepng_read32bitInt | ok | 5 | {'completed': 5} | 1/235 | 0/3873 |
| lodepng_realloc | ok | 4 | {'completed': 4} | 1/235 | 0/3873 |
| lodepng_set32bitInt | ok | 5 | {'completed': 5} | 1/235 | 0/3873 |
| lodepng_strlen | ok | 17 | {'completed': 17} | 1/235 | 2/3873 |
| paethPredictor | ok | 7 | {'completed': 7} | 1/235 | 8/3873 |
| removePaddingBits | ok | 34 | {'completed': 33, 'crash': 1} | 3/235 | 12/3874 |
| reverseBits | ok | 11 | {'completed': 11} | 1/235 | 3/3873 |
| searchCodeIndex | ok | 1 | {'completed': 1} | 1/235 | 7/3873 |
| uivector_cleanup | ok | 1 | {'crash': 1} | 0/235 | 0/3873 |
| unfilter | ok | 185 | {'completed': 185} | 4/235 | 97/3873 |
| unfilterScanline | ok | 48 | {'completed': 48} | 1/235 | 13/3874 |
| update_adler32 | ok | 15 | {'completed': 15} | 1/235 | 7/3873 |

## Procedure, deviations, and what is not established

<!-- prose -->
