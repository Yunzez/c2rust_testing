# Same-corpus C reach — optipng_laertes

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `optipng_multi.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 101 / 552 (0.183) | 1267 / 10394 (0.122) |
| Rust (archived campaign) | 71 / 820 (0.087) | 6611 / 49009 (0.135) |

Inputs replayed on the C side: {'crash': 62, 'completed': 937} over 55 / 55 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/optipng__laertes/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 392 (out of scope: 4).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 47 | 28 (1) | 0 (0) | 317 | 156 | 4 (4) | 166 (1) |

`c_only`: png_format_number→png_format_number, deflateStateCheck→deflateStateCheck, deflateResetKeep→deflateResetKeep, deflateReset→deflateReset, putShortMSB→putShortMSB, flush_pending→flush_pending, deflate→deflate, deflateEnd→deflateEnd, read_buf→read_buf, lm_init→lm_init, fill_window→fill_window, deflate_stored→deflate_stored, tr_static_init→tr_static_init, _tr_init→_tr_init, init_block→init_block, pqdownheap→pqdownheap, gen_codes→gen_codes, build_tree→build_tree, scan_tree→scan_tree, build_bl_tree→build_bl_tree, _tr_stored_block→_tr_stored_block, _tr_flush_bits→_tr_flush_bits, _tr_flush_block→_tr_flush_block, compress_block→compress_block, detect_data_type→detect_data_type, bi_reverse→bi_reverse, bi_flush→bi_flush, bi_windup→bi_windup

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
| adler32_z | ok | 32 | {'completed': 32} | 1/552 | 30/10394 |
| compress | ok | 13 | {'completed': 13} | 36/552 | 524/10394 |
| compress2 | ok | 16 | {'completed': 16} | 23/552 | 249/10394 |
| compressBound | ok | 8 | {'completed': 8} | 1/552 | 1/10394 |
| crc32 | ok | 30 | {'completed': 30} | 3/552 | 22/10394 |
| crc32_combine | ok | 68 | {'crash': 28, 'completed': 40} | 4/552 | 27/10394 |
| crc32_combine64 | ok | 68 | {'crash': 28, 'completed': 40} | 4/552 | 27/10394 |
| crc32_z | ok | 30 | {'completed': 30} | 2/552 | 21/10394 |
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
| opng_os_test | ok | 43 | {'completed': 43} | 1/552 | 34/10394 |
| opng_os_test_eq | ok | 42 | {'completed': 42} | 1/552 | 13/10394 |
| opng_path_make_backup | ok | 20 | {'completed': 20} | 1/552 | 6/10394 |
| opng_path_replace_dir | ok | 58 | {'completed': 58} | 1/552 | 17/10394 |
| opng_path_replace_ext | ok | 63 | {'completed': 63} | 1/552 | 26/10394 |
| opng_strparse_rangeset_to_bitset | ok | 88 | {'completed': 88} | 1/552 | 53/10394 |
| optimize_cmf | ok | 17 | {'completed': 17} | 1/552 | 13/10394 |
| png_access_version_number | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| png_format_number | ok | 1 | {'completed': 1} | 1/552 | 13/10394 |
| png_gt | ok | 6 | {'completed': 6} | 1/552 | 1/10394 |
| png_safecat | ok | 11 | {'completed': 11} | 1/552 | 13/10394 |
| png_save_int_32 | ok | 5 | {'completed': 5} | 2/552 | 2/10394 |
| png_save_uint_16 | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| png_save_uint_32 | ok | 5 | {'completed': 5} | 1/552 | 1/10394 |
| png_sig_cmp | ok | 28 | {'completed': 28} | 1/552 | 12/10394 |
| png_zalloc | ok | 8 | {'completed': 8} | 1/552 | 5/10394 |
| png_zfree | ok | 1 | {'completed': 1} | 2/552 | 6/10394 |
| pngx_gif_error | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| pngx_gif_warning | ok | 16 | {'completed': 16} | 3/552 | 11/10394 |
| pngx_tiff_error | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| pngx_tiff_warning | ok | 15 | {'completed': 15} | 1/552 | 4/10394 |
| pnm_is_valid | ok | 29 | {'completed': 29} | 1/552 | 25/10394 |
| uncompress | ok | 60 | {'completed': 60} | 17/552 | 339/10394 |
| zError | ok | 1 | {'crash': 1} | 0/552 | 0/10394 |
| zcalloc | ok | 1 | {'completed': 1} | 1/552 | 3/10394 |
| zcfree | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |
| zlibCompileFlags | ok | 1 | {'completed': 1} | 1/552 | 9/10394 |
| zlibVersion | ok | 1 | {'completed': 1} | 1/552 | 1/10394 |

## Procedure, deviations, and what is not established

<!-- prose -->
