# C-guided companion campaign — lodepng_c2rust

Same harnesses (generator `fd1f75b4716a1d45`, `--c-coverage`), same seed and libFuzzer parameters as the archived
campaign, `C2R_MODE=c-only`, budget 3600 s, 54 boundaries. CR = the archived Rust-guided corpus,
CC = this C-guided corpus. Reach only; no candidate from CC is adjudicated here.

## 2 x 2 (corpus x side); percentages are side-specific and never subtracted

| corpus | C functions | C regions | Rust functions | Rust regions |
|---|---|---|---|---|
| Rust-guided CR | 56 / 235 | 468 / 3874 (0.121) | 54 / 236 | 1675 / 13260 (0.126) |
| C-guided CC | 56 / 235 | 570 / 3874 (0.147) | 54 / 236 | 1738 / 13260 (0.131) |
| CR ∪ CC | 56 / 235 | 586 / 3874 (0.151) | 54 / 236 | – |

C-side inputs on CC: {'completed': 1488, 'crash': 10}; corpus sizes 1498 inputs.

## Matched-function sets (accepted pairs ∩ C scope ∩ Rust scope)

| corpus | pairs | both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous |
|---|---|---|---|---|---|---|
| CR | 178 | 43 | 3 (3) | 0 (0) | 132 | 57 |
| CC | 178 | 43 | 3 (3) | 0 (0) | 132 | 57 |
| CR ∪ CC | 178 | 43 | 3 (0) | 0 (0) | 132 | 57 |

`c_only` on CC: lodepng_chunk_check_crc, lodepng_chunk_generate_crc, lodepng_chunk_find

`c_only` on CR ∪ CC: lodepng_chunk_check_crc, lodepng_chunk_generate_crc, lodepng_chunk_find

## Campaign

| boundary | corpus | jobs | cov | crash | timeout |
|---|---|---|---|---|---|
| Adam7_deinterlace | 33 | 45380 | 158 | 0 | 60 |
| Adam7_interlace | 21 | 2579 | 153 | 0 | 66 |
| addColorBits | 12 | 1820059 | 134 | 51901 | 0 |
| addPaddingBits | 14 | 4732292 | 144 | 33543 | 0 |
| adler32 | 13 | 164367715 | 127 | 0 | 0 |
| alloc_string | 17 | 152793125 | 144 | 0 | 0 |
| alloc_string_sized | 24 | 52671391 | 163 | 13381 | 0 |
| checkColorValidity | 27 | 362289806 | 143 | 0 | 0 |
| countZeros | 1 | 109366 | 0 | 54682 | 0 |
| filterScanline | 134 | 68035448 | 250 | 3572 | 0 |
| getHash | 16 | 194845626 | 145 | 0 | 0 |
| getNumColorChannels | 9 | 380967338 | 117 | 0 | 0 |
| getValueRequiredBits | 4 | 371643120 | 105 | 0 | 0 |
| ilog2 | 9 | 347726562 | 131 | 0 | 0 |
| ilog2i | 9 | 347416638 | 132 | 0 | 0 |
| isGrayICCProfile | 28 | 261513477 | 217 | 0 | 0 |
| isRGBICCProfile | 28 | 258079908 | 217 | 0 | 0 |
| lodepng_chunk_ancillary | 6 | 303258596 | 124 | 0 | 0 |
| lodepng_chunk_check_crc | 1 | 56384 | 121 | 56383 | 0 |
| lodepng_chunk_data | 10 | 291552793 | 143 | 0 | 0 |
| lodepng_chunk_data_const | 10 | 294467159 | 143 | 0 | 0 |
| lodepng_chunk_find | 25 | 15599095 | 159 | 0 | 0 |
| lodepng_chunk_find_const | 50 | 8479828 | 195 | 45973 | 0 |
| lodepng_chunk_generate_crc | 1 | 190192 | 121 | 47500 | 0 |
| lodepng_chunk_length | 5 | 298847421 | 126 | 0 | 0 |
| lodepng_chunk_next | 16 | 140423263 | 167 | 0 | 0 |
| lodepng_chunk_next_const | 13 | 164784830 | 140 | 0 | 0 |
| lodepng_chunk_private | 8 | 280393500 | 134 | 0 | 0 |
| lodepng_chunk_safetocopy | 8 | 291537346 | 142 | 0 | 0 |
| lodepng_chunk_type | 13 | 125338355 | 119 | 0 | 0 |
| lodepng_chunk_type_equals | 19 | 146743159 | 158 | 0 | 0 |
| lodepng_crc32 | 14 | 159820300 | 128 | 0 | 0 |
| lodepng_error_text | 104 | 363587842 | 213 | 0 | 0 |
| lodepng_free | 1 | 340721488 | 95 | 0 | 0 |
| lodepng_get_bpp_lct | 12 | 366096646 | 128 | 0 | 0 |
| lodepng_get_raw_size_idat | 13 | 359220545 | 133 | 0 | 0 |
| lodepng_get_raw_size_lct | 21 | 333754558 | 151 | 0 | 0 |
| lodepng_gtofl | 9 | 313503947 | 126 | 0 | 0 |
| lodepng_huffman_code_lengths | 39 | 6718005 | 170 | 0 | 0 |
| lodepng_malloc | 4 | 868259 | 106 | 0 | 0 |
| lodepng_memcpy | 1 | 114656 | 0 | 57327 | 0 |
| lodepng_memset | 7 | 2858060 | 112 | 53137 | 0 |
| lodepng_read32bitInt | 5 | 303059050 | 126 | 0 | 0 |
| lodepng_realloc | 4 | 867537 | 106 | 0 | 0 |
| lodepng_set32bitInt | 5 | 309739797 | 120 | 0 | 0 |
| lodepng_strlen | 21 | 170862261 | 135 | 0 | 0 |
| paethPredictor | 9 | 353805082 | 126 | 0 | 0 |
| removePaddingBits | 48 | 808688 | 152 | 50740 | 0 |
| reverseBits | 22 | 182124060 | 129 | 0 | 0 |
| searchCodeIndex | 1 | 138 | 138 | 0 | 137 |
| uivector_cleanup | 1 | 57696 | 0 | 57695 | 0 |
| unfilter | 250 | 985893 | 326 | 43411 | 0 |
| unfilterScanline | 307 | 1447375 | 481 | 34783 | 0 |
| update_adler32 | 16 | 171742357 | 142 | 0 | 0 |

## Procedure, deviations, and what is not established

<!-- prose -->
