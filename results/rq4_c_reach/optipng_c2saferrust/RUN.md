# Same-corpus C reach — optipng_c2saferrust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `optipng_multi.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 133 / 552 (0.241) | 1470 / 10394 (0.141) |
| Rust (archived campaign) | 144 / 564 (0.255) | 9815 / 37297 (0.263) |

Inputs replayed on the C side: {'crash': 77, 'completed': 1715} over 96 / 96 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/optipng__c2saferrust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 399 (out of scope: 11).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 96 | 1 (1) | 8 (5) | 294 | 142 | 11 (9) | 5 (3) |

`c_only`: opng_free→opng_free

`rust_only`: opng_print_fsize_ratio→opng_print_fsize_ratio, opng_print_fsize_difference→opng_print_fsize_difference, opng_snprintf_impl→opng_snprintf_impl, opng_sprint_uratio_impl→opng_sprint_uratio_impl, opng_ulratio_to_factor_string→opng_ulratio_to_factor_string, fixedtables→fixedtables, updatewindow→updatewindow, inflate_table→inflate_table

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| DefaultError | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| DefaultWarning | ok | 16 | {'completed': 16} | 1/552 | 2/10394 |
| ErrorAlloc | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| add_one_chunk | ok | 5 | {'completed': 5} | 1/552 | 8/10394 |
| adler32 | ok | 1 | {'completed': 1} | 2/552 | 23/10394 |
| adler32_combine | ok | 10 | {'completed': 10} | 2/552 | 14/10394 |
| adler32_combine64 | ok | 10 | {'completed': 10} | 2/552 | 14/10394 |
| app_finish | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| app_init | ok | 1 | {'completed': 1} | 1/552 | 10/10394 |
| app_print_cntrl | ok | 6 | {'completed': 6} | 1/552 | 17/10394 |
| app_progress | ok | 6 | {'completed': 6} | 1/552 | 7/10394 |
| bmp_get_dword | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| bmp_get_word | ok | 3 | {'completed': 3} | 1/552 | 1/10394 |
| bmp_memset_bytes | ok | 10 | {'completed': 9, 'crash': 1} | 1/552 | 1/10394 |
| bmp_memset_halfbytes | ok | 15 | {'completed': 14, 'crash': 1} | 1/552 | 8/10394 |
| bmp_process_mask | ok | 13 | {'completed': 13} | 1/552 | 13/10394 |
| check_num_option | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| check_obj_option | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| check_power2_option | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| check_rangeset_option | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| compress | ok | 192 | {'completed': 192} | 39/552 | 561/10394 |
| compress2 | ok | 293 | {'completed': 293} | 38/552 | 619/10394 |
| compressBound | ok | 8 | {'completed': 8} | 1/552 | 1/10394 |
| crc32 | ok | 29 | {'completed': 29} | 3/552 | 22/10394 |
| crc32_combine | ok | 49 | {'crash': 14, 'completed': 35} | 4/552 | 27/10394 |
| crc32_combine64 | ok | 49 | {'crash': 14, 'completed': 35} | 4/552 | 27/10394 |
| err_option_arg | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| get_crc_table | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| get_ulong_i | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| get_ulong_m | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| get_ushort_i | ok | 3 | {'completed': 3} | 1/552 | 1/10394 |
| get_ushort_m | ok | 3 | {'completed': 3} | 1/552 | 1/10394 |
| opng_allow_chunk | ok | 1 | {'completed': 1} | 2/552 | 6/10394 |
| opng_bitset_count | ok | 8 | {'completed': 8} | 1/552 | 3/10394 |
| opng_bitset_find_first | ok | 9 | {'completed': 9} | 1/552 | 6/10394 |
| opng_bitset_find_last | ok | 33 | {'completed': 33} | 1/552 | 6/10394 |
| opng_bitset_find_next | ok | 12 | {'completed': 12} | 1/552 | 9/10394 |
| opng_bitset_find_prev | ok | 11 | {'completed': 11} | 1/552 | 10/10394 |
| opng_check_idat_size | ok | 4 | {'completed': 3, 'crash': 1} | 1/552 | 2/10394 |
| opng_clear_image_info | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| opng_destroy_image_info | ok | 1 | {'completed': 1} | 1/552 | 4/10394 |
| opng_finalize | ok | 1 | {'completed': 1} | 1/552 | 6/10394 |
| opng_finish_iterations | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| opng_free | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| opng_get_alpha_row | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_init_iteration | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_init_iterations | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_init_read_data | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| opng_init_write_data | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| opng_is_apng_chunk | ok | 1 | {'completed': 1} | 1/552 | 7/10394 |
| opng_is_image_chunk | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| opng_iterate | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_optimize_impl | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_os_test | ok | 43 | {'completed': 43} | 1/552 | 34/10394 |
| opng_os_test_eq | ok | 42 | {'completed': 42} | 1/552 | 13/10394 |
| opng_path_make_backup | ok | 20 | {'completed': 20} | 1/552 | 6/10394 |
| opng_path_replace_dir | ok | 57 | {'completed': 57} | 1/552 | 17/10394 |
| opng_path_replace_ext | ok | 67 | {'completed': 67} | 1/552 | 24/10394 |
| opng_print_fsize_difference | ok | 11 | {'crash': 11} | 0/552 | 0/10394 |
| opng_print_fsize_ratio | ok | 9 | {'crash': 9} | 0/552 | 0/10394 |
| opng_print_image_info | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| opng_str2ulong | ok | 32 | {'completed': 32} | 2/552 | 40/10394 |
| opng_strcasecmp | ok | 90 | {'completed': 90} | 1/552 | 9/10394 |
| opng_strltrim | ok | 21 | {'completed': 21} | 1/552 | 3/10394 |
| opng_strparse_rangeset_to_bitset | ok | 96 | {'completed': 96} | 1/552 | 53/10394 |
| opng_strpbrk_digit | ok | 20 | {'completed': 20} | 1/552 | 9/10394 |
| opng_strtail | ok | 15 | {'completed': 15} | 1/552 | 3/10394 |
| optimize_cmf | ok | 18 | {'completed': 18} | 1/552 | 13/10394 |
| panic | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| parse_args | ok | 107 | {'completed': 107} | 2/552 | 113/10394 |
| png_access_version_number | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| png_do_write_interlace | ok | 26 | {'completed': 26} | 1/552 | 9/10394 |
| png_gt | ok | 6 | {'completed': 6} | 1/552 | 1/10394 |
| png_read_filter_row_avg | ok | 7 | {'completed': 6, 'crash': 1} | 1/552 | 5/10394 |
| png_read_filter_row_paeth_1byte_pixel | ok | 7 | {'completed': 6, 'crash': 1} | 1/552 | 2/10394 |
| png_read_filter_row_paeth_multibyte_pixel | ok | 8 | {'completed': 7, 'crash': 1} | 1/552 | 12/10394 |
| png_read_filter_row_sub | ok | 17 | {'completed': 16, 'crash': 1} | 1/552 | 4/10394 |
| png_read_filter_row_up | ok | 7 | {'completed': 6, 'crash': 1} | 1/552 | 4/10394 |
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
| pnm_raw_sample_size | ok | 21 | {'completed': 21} | 1/552 | 16/10394 |
| uncompress | ok | 97 | {'completed': 97} | 12/552 | 150/10394 |
| zError | ok | 6 | {'crash': 5, 'completed': 1} | 1/552 | 1/10394 |
| zcalloc | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| zcfree | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| zlibCompileFlags | ok | 1 | {'completed': 1} | 1/552 | 9/10394 |
| zlibVersion | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |

## Procedure, deviations, and what is not established

<!-- prose -->
