# Same-corpus C reach — optipng_c2rust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `optipng_multi.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 105 / 552 (0.190) | 1591 / 10394 (0.153) |
| Rust (archived campaign) | 106 / 555 (0.191) | 9970 / 37840 (0.263) |

Inputs replayed on the C side: {'crash': 63, 'completed': 2071} over 54 / 54 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/optipng__c2rust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 417 (out of scope: 4).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 81 | 0 (0) | 0 (0) | 336 | 131 | 4 (4) | 2 (1) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| DefaultError | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| DefaultWarning | ok | 16 | {'completed': 16} | 1/552 | 2/10394 |
| ErrorAlloc | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| add_one_chunk | ok | 5 | {'completed': 5} | 1/552 | 8/10394 |
| adler32 | ok | 32 | {'completed': 32} | 2/552 | 31/10394 |
| adler32_combine | ok | 10 | {'completed': 10} | 2/552 | 14/10394 |
| adler32_combine64 | ok | 10 | {'completed': 10} | 2/552 | 14/10394 |
| compress | ok | 411 | {'completed': 411} | 39/552 | 583/10394 |
| compress2 | ok | 488 | {'completed': 488} | 40/552 | 672/10394 |
| compressBound | ok | 8 | {'completed': 8} | 1/552 | 1/10394 |
| crc32 | ok | 30 | {'completed': 30} | 3/552 | 22/10394 |
| crc32_combine | ok | 68 | {'crash': 28, 'completed': 40} | 4/552 | 27/10394 |
| crc32_combine64 | ok | 68 | {'crash': 28, 'completed': 40} | 4/552 | 27/10394 |
| default_error_handler | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| default_warning_handler | ok | 16 | {'completed': 16} | 1/552 | 2/10394 |
| get_crc_table | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| get_ulong_i | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| get_ulong_m | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| get_ushort_i | ok | 3 | {'completed': 3} | 1/552 | 1/10394 |
| get_ushort_m | ok | 3 | {'completed': 3} | 1/552 | 1/10394 |
| opng_bitset_count | ok | 8 | {'completed': 8} | 1/552 | 3/10394 |
| opng_bitset_find_first | ok | 32 | {'completed': 32} | 1/552 | 6/10394 |
| opng_bitset_find_last | ok | 33 | {'completed': 33} | 1/552 | 6/10394 |
| opng_bitset_find_next | ok | 12 | {'completed': 12} | 1/552 | 9/10394 |
| opng_bitset_find_prev | ok | 11 | {'completed': 11} | 1/552 | 10/10394 |
| opng_get_alpha_row | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_os_test | ok | 44 | {'completed': 44} | 1/552 | 34/10394 |
| opng_os_test_eq | ok | 42 | {'completed': 42} | 1/552 | 13/10394 |
| opng_path_make_backup | ok | 20 | {'completed': 20} | 1/552 | 6/10394 |
| opng_path_replace_dir | ok | 58 | {'completed': 58} | 1/552 | 17/10394 |
| opng_path_replace_ext | ok | 64 | {'completed': 64} | 1/552 | 26/10394 |
| opng_strparse_rangeset_to_bitset | ok | 90 | {'completed': 90} | 1/552 | 53/10394 |
| png_access_version_number | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| png_do_write_interlace | ok | 28 | {'completed': 28} | 1/552 | 19/10394 |
| png_gt | ok | 6 | {'completed': 6} | 1/552 | 1/10394 |
| png_safecat | ok | 11 | {'completed': 11} | 1/552 | 13/10394 |
| png_save_int_32 | ok | 5 | {'completed': 5} | 2/552 | 2/10394 |
| png_save_uint_16 | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| png_save_uint_32 | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| png_zalloc | ok | 8 | {'completed': 8} | 1/552 | 5/10394 |
| png_zfree | ok | 1 | {'completed': 1} | 2/552 | 6/10394 |
| pngx_gif_error | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| pngx_gif_warning | ok | 16 | {'completed': 16} | 3/552 | 11/10394 |
| pngx_tiff_error | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| pngx_tiff_warning | ok | 15 | {'completed': 15} | 1/552 | 4/10394 |
| pnm_is_valid | ok | 29 | {'completed': 29} | 1/552 | 25/10394 |
| pnm_mem_size | ok | 32 | {'completed': 32} | 1/552 | 15/10394 |
| pnm_raw_sample_size | ok | 21 | {'completed': 21} | 1/552 | 16/10394 |
| uncompress | ok | 347 | {'completed': 347} | 18/552 | 561/10394 |
| zError | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| zcalloc | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| zcfree | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| zlibCompileFlags | ok | 1 | {'completed': 1} | 1/552 | 9/10394 |
| zlibVersion | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |

## Procedure, deviations, and what is not established

<!-- prose -->
