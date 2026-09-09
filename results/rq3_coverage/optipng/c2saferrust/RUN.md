# optipng × c2saferrust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 552 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 121 |
| built | 96 |
| executed (corpus > 0) | 96 |
| coverage exported | 79 |

Plan failures, by the generator's own reason:

- **178** × signature: struct-invariant param png_ptr: png_struct_def has pointer field 'error_fn' (ne
- **39** × signature: struct-invariant param strm: z_stream_s has pointer field 'next_in' (needs inva
- **32** × it flows into gz_open(), whose effect the harness cannot undo. What the boundary consumes 
- **30** × signature: struct-invariant param png_ptr: png_struct_def opaque/incomplete (no visible fi
- **28** × signature: struct-invariant param s: deflate_state has pointer field 'strm' (needs invaria
- **24** × signature: struct-invariant param stream: FILE has pointer field '_IO_read_ptr' (needs inv
- **17** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is
- **13** × signature: struct-invariant param state: gz_state has nested struct field 'x' (needs invar
- **10** × signature: struct-invariant param info_ptr: minitiff_info has pointer field 'error_handler
- **4** × signature: struct-invariant param ext: GIFExtension has pointer field 'Screen' (needs inva
- **4** × signature: struct-invariant param image: GIFImage has pointer field 'Screen' (needs invari
- **4** × input buffer of i8 has Rust type &str, which is not a raw pointer, a slice, a Vec or a Box
- **4** × signature: callback parameter error_fn deferred: function pointers (callback binding) not 
- **2** × Rust signature has 2 parameters, C has 1: reshaped API, no positional bridge
- **2** × signature: struct-invariant param dest: z_stream_s has pointer field 'next_in' (needs inva
- **2** × signature: struct-invariant param getter_ptr: minitiff_getter has pointer field 'get_ushor
- **2** × signature: struct-invariant param infile: FILE has pointer field '_IO_read_ptr' (needs inv
- **2** × Rust signature has 2 parameters, C has 3: reshaped API, no positional bridge
- **2** × signature: unsupported: pointer-to-pointer-to-struct param png_ptr_ptr
- **2** × signature: struct-invariant param pp: png_struct_def has pointer field 'error_fn' (needs i
- **1** × string pointer table of u8 has Rust type png_bytepp, which is not a pointer to a pointer t
- **1** × signature: struct-invariant param state: inflate_state has pointer field 'strm' (needs inv
- **1** × signature: struct-invariant param tree: ct_data has nested struct field 'fc' (needs invari
- **1** × it flows into open(), whose effect the harness cannot undo. What the boundary consumes is 
- **1** × signature: callback parameter in_ deferred: function pointers (callback binding) not yet s
- **1** × signature: unsupported: pointer-to-pointer-to-struct param table
- **1** × it flows into process_files(), whose effect the harness cannot undo. What the boundary con
- **1** × signature: struct-invariant param init_options: opng_options has pointer field 'out_name' 
- **1** × it flows into opng_optimize_impl(), whose effect the harness cannot undo. What the boundar
- **1** × it flows into chmod(), whose effect the harness cannot undo. What the boundary consumes is
- **1** × it flows into mkdir(), whose effect the harness cannot undo. What the boundary consumes is
- **1** × it flows into rename(), whose effect the harness cannot undo. What the boundary consumes i
- **1** × it flows into unlink(), whose effect the harness cannot undo. What the boundary consumes i
- **1** × Rust signature has 4 parameters, C has 3: reshaped API, no positional bridge
- **1** × signature: struct-invariant param outfile: FILE has pointer field '_IO_read_ptr' (needs in
- **1** × signature: unsupported: pointer-to-pointer-to-struct param info_ptr_ptr
- **1** × signature: unsupported: pointer-to-pointer-to-struct param listp
- **1** × signature: unsupported: pointer-to-pointer-to-struct param palette
- **1** × signature: unsupported: pointer-to-pointer-to-struct param background
- **1** × signature: unsupported: pointer-to-pointer-to-struct param sig_bit
- **1** × signature: unsupported: pointer-to-pointer-to-struct param trans_color
- **1** × signature: unsupported: pointer-to-pointer-to-struct param unknowns
- **1** × signature: unsupported: pointer-to-pointer-to-struct param ptr_ptr
- **1** × signature: png_rtran_ok is not present in the Rust translation (no boundary)
- **1** × signature: callback parameter read_data_fn deferred: function pointers (callback binding) 
- **1** × signature: callback parameter read_row_fn deferred: function pointers (callback binding) n
- **1** × signature: callback parameter write_data_fn deferred: function pointers (callback binding)
- **1** × signature: callback parameter write_row_fn deferred: function pointers (callback binding) 
- **1** × string pointer table of i8 has Rust type Vec<String>, which is not a pointer to a pointer 
- **1** × output_buffer is written by the callee but the Rust parameter is *const Bytef (const)

Planned but not built:

- `adler32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe
- `adler32_z`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `bi_reverse`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe
- `crc32_big`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `crc32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe
- `crc32_little`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `crc32_z`: error[E0061]: this function takes 3 arguments but 2 arguments were supplied
error[E0061]: this function takes 3 argument
- `gf2_matrix_square`: error[E0308]: arguments to this function are incorrect
error[E0308]: arguments to this function are incorrect
error: cou
- `gf2_matrix_times`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe
- `opng_insert_palette_entry`: error[E0425]: cannot find value `num_palette_c` in this scope
error[E0425]: cannot find value `num_palette_r` in this sc
- `opng_sprint_uratio_impl`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_ullratio_to_factor_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_ulratio_to_factor_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `png_do_read_interlace`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `png_format_number`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `png_get_int_32`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `png_get_uint_16`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe
- `png_get_uint_32`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `png_sig_cmp`: error[E0308]: arguments to this function are incorrect
error[E0308]: arguments to this function are incorrect
error: cou
- `png_warning_parameter`: error[E0308]: arguments to this function are incorrect
error[E0308]: arguments to this function are incorrect
error: cou
- `png_warning_parameter_signed`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `png_warning_parameter_unsigned`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "
- `pnm_mem_size`: error[E0308]: mismatched types
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2saferrust_ft") due to
- `syncsearch`: error[E0061]: this function takes 3 arguments but 2 arguments were supplied
error[E0061]: this function takes 3 argument
- `tr_static_init`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2saferrust-fuzz` (bin "optipng_c2safe

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `DefaultError` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `DefaultWarning` | yes | 16 | 0 | normal 16 | batch |
| `ErrorAlloc` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `add_one_chunk` | yes | 5 | 16 | normal 4, signal 1 | per-input (4/5 completed) |
| `adler32` | no | 1 | 1 | divergence 1 | batch |
| `adler32_combine` | no | 10 | 0 | normal 10 | batch |
| `adler32_combine64` | no | 10 | 0 | normal 10 | batch |
| `app_finish` | yes | 1 | 0 | normal 1 | batch |
| `app_init` | yes | 1 | 0 | normal 1 | batch |
| `app_print_cntrl` | yes | 6 | 0 | normal 6 | batch |
| `app_progress` | yes | 6 | 0 | normal 6 | batch |
| `bmp_get_dword` | yes | 5 | 0 | normal 5 | batch |
| `bmp_get_word` | yes | 3 | 0 | normal 3 | batch |
| `bmp_memset_bytes` | yes | 10 | 118 | normal 9, signal 1 | per-input (9/10 completed) |
| `bmp_memset_halfbytes` | yes | 15 | 602 | normal 14, signal 1 | per-input (14/15 completed) |
| `bmp_process_mask` | yes | 13 | 0 | normal 13 | batch |
| `check_num_option` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `check_obj_option` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `check_power2_option` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `check_rangeset_option` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `compress` | no | 192 | 4925 | divergence 1, normal 190, panic 1 | per-input (191/192 completed) |
| `compress2` | no | 293 | 9769 | divergence 99, normal 194 | batch |
| `compressBound` | no | 8 | 0 | normal 8 | batch |
| `crc32` | no | 29 | 0 | divergence 26, normal 3 | batch |
| `crc32_combine` | no | 49 | 4655 | divergence 23, normal 12, signal 14 | per-input (48/49 completed) |
| `crc32_combine64` | no | 49 | 4657 | divergence 23, normal 12, signal 14 | per-input (48/49 completed) |
| `err_option_arg` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `get_crc_table` | no | 1 | 0 | normal 1 | batch |
| `get_ulong_i` | yes | 5 | 0 | normal 5 | batch |
| `get_ulong_m` | yes | 5 | 0 | normal 5 | batch |
| `get_ushort_i` | yes | 3 | 0 | normal 3 | batch |
| `get_ushort_m` | yes | 3 | 0 | normal 3 | batch |
| `opng_allow_chunk` | yes | 1 | 1 | normal 1 | batch |
| `opng_bitset_count` | no | 8 | 0 | normal 8 | batch |
| `opng_bitset_find_first` | no | 9 | 0 | normal 9 | batch |
| `opng_bitset_find_last` | no | 33 | 0 | normal 33 | batch |
| `opng_bitset_find_next` | no | 12 | 0 | normal 12 | batch |
| `opng_bitset_find_prev` | no | 11 | 0 | normal 11 | batch |
| `opng_check_idat_size` | yes | 4 | 8 | normal 3, signal 1 | per-input (3/4 completed) |
| `opng_clear_image_info` | yes | 1 | 0 | normal 1 | batch |
| `opng_destroy_image_info` | yes | 1 | 0 | normal 1 | batch |
| `opng_finalize` | no | 1 | 0 | normal 1 | batch |
| `opng_finish_iterations` | yes | 1 | 0 | normal 1 | batch |
| `opng_free` | yes | 1 | 2 | panic 1 | failed rc=1 |
| `opng_get_alpha_row` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_init_iteration` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_init_iterations` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_init_read_data` | yes | 1 | 0 | normal 1 | batch |
| `opng_init_write_data` | yes | 1 | 0 | normal 1 | batch |
| `opng_is_apng_chunk` | yes | 1 | 0 | normal 1 | batch |
| `opng_is_image_chunk` | yes | 1 | 1 | normal 1 | batch |
| `opng_iterate` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_optimize_impl` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_os_test` | no | 43 | 0 | normal 43 | batch |
| `opng_os_test_eq` | no | 42 | 0 | normal 42 | batch |
| `opng_path_make_backup` | no | 20 | 0 | normal 20 | batch |
| `opng_path_replace_dir` | no | 57 | 0 | normal 57 | batch |
| `opng_path_replace_ext` | no | 67 | 0 | normal 67 | batch |
| `opng_print_fsize_difference` | yes | 11 | 0 | signal 11 | batch |
| `opng_print_fsize_ratio` | yes | 9 | 0 | signal 9 | batch |
| `opng_print_image_info` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_str2ulong` | yes | 32 | 0 | normal 32 | batch |
| `opng_strcasecmp` | yes | 90 | 0 | divergence 55, normal 35 | batch |
| `opng_strltrim` | yes | 21 | 0 | normal 21 | batch |
| `opng_strparse_rangeset_to_bitset` | no | 96 | 0 | normal 96 | batch |
| `opng_strpbrk_digit` | yes | 20 | 0 | normal 20 | batch |
| `opng_strtail` | yes | 15 | 0 | normal 15 | batch |
| `optimize_cmf` | yes | 18 | 1259 | normal 18 | batch |
| `panic` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `parse_args` | yes | 107 | 9468 | normal 107 | batch |
| `png_access_version_number` | no | 1 | 0 | normal 1 | batch |
| `png_do_write_interlace` | no | 26 | 76 | normal 26 | batch |
| `png_gt` | yes | 6 | 0 | normal 6 | batch |
| `png_read_filter_row_avg` | yes | 7 | 25 | normal 6, signal 1 | per-input (6/7 completed) |
| `png_read_filter_row_paeth_1byte_pixel` | yes | 7 | 25 | normal 6, signal 1 | per-input (6/7 completed) |
| `png_read_filter_row_paeth_multibyte_pixel` | yes | 8 | 52 | normal 7, signal 1 | per-input (7/8 completed) |
| `png_read_filter_row_sub` | yes | 17 | 2987 | normal 16, signal 1 | per-input (16/17 completed) |
| `png_read_filter_row_up` | yes | 7 | 25 | normal 6, signal 1 | per-input (6/7 completed) |
| `png_safecat` | no | 11 | 148 | normal 10, signal 1 | per-input (10/11 completed) |
| `png_save_int_32` | no | 5 | 0 | normal 5 | batch |
| `png_save_uint_16` | no | 5 | 0 | normal 5 | batch |
| `png_save_uint_32` | no | 5 | 0 | normal 5 | batch |
| `png_zalloc` | no | 8 | 0 | normal 8 | batch |
| `png_zfree` | no | 1 | 0 | normal 1 | batch |
| `pngx_gif_error` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `pngx_gif_warning` | yes | 16 | 0 | normal 16 | batch |
| `pngx_tiff_error` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `pngx_tiff_warning` | yes | 15 | 0 | normal 15 | batch |
| `pnm_is_valid` | no | 29 | 0 | normal 29 | batch |
| `pnm_raw_sample_size` | no | 21 | 0 | normal 21 | batch |
| `uncompress` | no | 97 | 10649 | divergence 81, normal 15, panic 1 | per-input (97/97 completed) |
| `zError` | no | 6 | 20 | normal 1, signal 5 | batch |
| `zcalloc` | no | 1 | 1 | normal 1 | batch |
| `zcfree` | no | 1 | 0 | normal 1 | batch |
| `zlibCompileFlags` | no | 1 | 0 | normal 1 | batch |
| `zlibVersion` | no | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no transpiled test target; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 96 harnesses, 17 crash-all (`DefaultError` accepted, `ErrorAlloc` accepted, `check_num_option` accepted, `check_obj_option` accepted, `check_power2_option` accepted, `check_rangeset_option` accepted, `err_option_arg` accepted, `opng_free` accepted, `opng_get_alpha_row` accepted, `opng_init_iterations` accepted, `opng_iterate` accepted, `opng_optimize_impl` accepted, `opng_print_fsize_difference` accepted, `opng_print_fsize_ratio` accepted, `panic` accepted, `pngx_gif_error` accepted, `pngx_tiff_error` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`, `3d43f75aea807da1 (bin reused from the killed run)` — more than one: see deviations.

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 564 | 0 | 144 | 0 | 0 | 144 | 420 | 0.000 | 0.255 |
| regions | 37297 | 0 | 9815 | 0 | 0 | 9815 | 27482 | 0.000 | 0.263 |

Sanity checks: function pass, region pass. Harnesses unioned: 79. Identities outside the universe (excluded, never added): 0 fn / 24 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `DefaultWarning` | 16 | 16 | 16 | 16 | 16 |
| `add_one_chunk` | 5 | 5 | 5 | 5 | 5 |
| `adler32_combine` | 10 | 10 | 10 | 10 | 10 |
| `adler32_combine64` | 10 | 10 | 10 | 10 | 10 |
| `app_print_cntrl` | 6 | 6 | 6 | 6 | 6 |
| `app_progress` | 6 | 6 | 6 | 6 | 6 |
| `bmp_get_dword` | 5 | 5 | 5 | 5 | 5 |
| `bmp_get_word` | 3 | 3 | 3 | 3 | 3 |
| `bmp_memset_bytes` | 10 | 10 | 10 | 10 | 10 |
| `bmp_memset_halfbytes` | 15 | 15 | 15 | 15 | 15 |
| `bmp_process_mask` | 13 | 13 | 13 | 13 | 13 |
| `compress` | 131 | 178 | 187 | 191 | 192 |
| `compress2` | 150 | 168 | 207 | 266 | 293 |
| `compressBound` | 8 | 8 | 8 | 8 | 8 |
| `crc32` | 29 | 29 | 29 | 29 | 29 |
| `crc32_combine` | 42 | 42 | 44 | 47 | 49 |
| `crc32_combine64` | 42 | 42 | 44 | 47 | 49 |
| `get_ulong_i` | 5 | 5 | 5 | 5 | 5 |
| `get_ulong_m` | 5 | 5 | 5 | 5 | 5 |
| `get_ushort_i` | 3 | 3 | 3 | 3 | 3 |
| `get_ushort_m` | 3 | 3 | 3 | 3 | 3 |
| `opng_bitset_count` | 8 | 8 | 8 | 8 | 8 |
| `opng_bitset_find_first` | 9 | 9 | 9 | 9 | 9 |
| `opng_bitset_find_last` | 33 | 33 | 33 | 33 | 33 |
| `opng_bitset_find_next` | 12 | 12 | 12 | 12 | 12 |
| `opng_bitset_find_prev` | 11 | 11 | 11 | 11 | 11 |
| `opng_check_idat_size` | 4 | 4 | 4 | 4 | 4 |
| `opng_os_test` | 41 | 42 | 43 | 43 | 43 |
| `opng_os_test_eq` | 40 | 41 | 42 | 42 | 42 |
| `opng_path_make_backup` | 20 | 20 | 20 | 20 | 20 |
| `opng_path_replace_dir` | 53 | 55 | 55 | 55 | 57 |
| `opng_path_replace_ext` | 64 | 67 | 67 | 67 | 67 |
| `opng_print_fsize_difference` | 11 | 11 | 11 | 11 | 11 |
| `opng_print_fsize_ratio` | 9 | 9 | 9 | 9 | 9 |
| `opng_str2ulong` | 32 | 32 | 32 | 32 | 32 |
| `opng_strcasecmp` | 82 | 84 | 85 | 89 | 90 |
| `opng_strltrim` | 21 | 21 | 21 | 21 | 21 |
| `opng_strparse_rangeset_to_bitset` | 94 | 95 | 96 | 96 | 96 |
| `opng_strpbrk_digit` | 20 | 20 | 20 | 20 | 20 |
| `opng_strtail` | 15 | 15 | 15 | 15 | 15 |
| `optimize_cmf` | 18 | 18 | 18 | 18 | 18 |
| `parse_args` | 59 | 82 | 93 | 106 | 107 |
| `png_do_write_interlace` | 10 | 21 | 21 | 23 | 26 |
| `png_gt` | 6 | 6 | 6 | 6 | 6 |
| `png_read_filter_row_avg` | 6 | 7 | 7 | 7 | 7 |
| `png_read_filter_row_paeth_1byte_pixel` | 6 | 7 | 7 | 7 | 7 |
| `png_read_filter_row_paeth_multibyte_pixel` | 8 | 8 | 8 | 8 | 8 |
| `png_read_filter_row_sub` | 17 | 17 | 17 | 17 | 17 |
| `png_read_filter_row_up` | 6 | 7 | 7 | 7 | 7 |
| `png_safecat` | 11 | 11 | 11 | 11 | 11 |
| `png_save_int_32` | 5 | 5 | 5 | 5 | 5 |
| `png_save_uint_16` | 5 | 5 | 5 | 5 | 5 |
| `png_save_uint_32` | 5 | 5 | 5 | 5 | 5 |
| `png_zalloc` | 8 | 8 | 8 | 8 | 8 |
| `pngx_gif_warning` | 16 | 16 | 16 | 16 | 16 |
| `pngx_tiff_warning` | 15 | 15 | 15 | 15 | 15 |
| `pnm_is_valid` | 29 | 29 | 29 | 29 | 29 |
| `pnm_raw_sample_size` | 21 | 21 | 21 | 21 | 21 |
| `uncompress` | 73 | 81 | 84 | 92 | 97 |
| `zError` | 6 | 6 | 6 | 6 | 6 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1401 |
| divergence | 309 |
| signal | 79 |
| panic | 3 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `DefaultError` | 3 / 3 | ub_associated 3 | 1 |
| `ErrorAlloc` | 3 / 3 | ub_associated 3 | 1 |
| `add_one_chunk` | 17 / 17 | ub_associated 17 | 2 |
| `adler32` | 2 / 2 | confirmed_divergence 1, confirmed_termination 1 | 2 |
| `bmp_memset_bytes` | 119 / 119 | confirmed_termination 117, ub_associated 2 | 2 |
| `bmp_memset_halfbytes` | 201 / 501 | ub_associated 201 | 9 |
| `check_num_option` | 3 / 3 | ub_associated 3 | 1 |
| `check_obj_option` | 3 / 3 | ub_associated 3 | 1 |
| `check_power2_option` | 3 / 3 | ub_associated 3 | 1 |
| `check_rangeset_option` | 3 / 3 | ub_associated 3 | 1 |
| `compress` | 202 / 502 | confirmed_divergence 1, confirmed_termination 201 | 2 |
| `compress2` | 299 / 599 | confirmed_divergence 99, confirmed_termination 197, ub_associated_termination 3 | 3 |
| `crc32` | 26 / 26 | confirmed_divergence 26 | 1 |
| `crc32_combine` | 237 / 537 | confirmed_divergence 23, inconclusive 13, ub_associated 201 | 3 |
| `crc32_combine64` | 237 / 537 | confirmed_divergence 23, inconclusive 13, ub_associated 201 | 3 |
| `err_option_arg` | 3 / 3 | ub_associated 3 | 1 |
| `opng_allow_chunk` | 1 / 1 | ub_associated 1 | 1 |
| `opng_check_idat_size` | 9 / 9 | ub_associated_termination 9 | 1 |
| `opng_free` | 3 / 3 | confirmed_termination 3 | 1 |
| `opng_get_alpha_row` | 3 / 3 | ub_associated 3 | 1 |
| `opng_init_iteration` | 3 / 3 | ub_associated_termination 3 | 2 |
| `opng_init_iterations` | 3 / 3 | ub_associated_termination 3 | 1 |
| `opng_is_image_chunk` | 1 / 1 | ub_associated 1 | 1 |
| `opng_iterate` | 3 / 3 | ub_associated_termination 3 | 1 |
| `opng_optimize_impl` | 3 / 3 | ub_associated_termination 3 | 1 |
| `opng_print_fsize_difference` | 11 / 11 | ub_associated 11 | 1 |
| `opng_print_fsize_ratio` | 9 / 9 | ub_associated 9 | 1 |
| `opng_print_image_info` | 3 / 3 | ub_associated_termination 3 | 1 |
| `opng_strcasecmp` | 55 / 55 | confirmed_divergence 55 | 1 |
| `optimize_cmf` | 200 / 500 | confirmed_termination 200 | 2 |
| `panic` | 3 / 3 | ub_associated 3 | 1 |
| `parse_args` | 200 / 500 | ub_associated 200 | 1 |
| `png_do_write_interlace` | 76 / 76 | inconclusive 2, not_reproducible 64, ub_associated 10 | 5 |
| `png_read_filter_row_avg` | 26 / 26 | ub_associated 26 | 1 |
| `png_read_filter_row_paeth_1byte_pixel` | 26 / 26 | ub_associated 26 | 1 |
| `png_read_filter_row_paeth_multibyte_pixel` | 53 / 53 | ub_associated 53 | 2 |
| `png_read_filter_row_sub` | 201 / 501 | ub_associated 201 | 1 |
| `png_read_filter_row_up` | 26 / 26 | ub_associated 26 | 1 |
| `png_safecat` | 149 / 149 | ub_associated 149 | 1 |
| `pngx_gif_error` | 3 / 3 | ub_associated 3 | 1 |
| `pngx_tiff_error` | 3 / 3 | ub_associated 3 | 1 |
| `uncompress` | 282 / 582 | confirmed_divergence 169, confirmed_termination 52, instrument_only 31, not_reproducible 30 | 3 |
| `zError` | 25 / 25 | ub_associated 5, ub_associated_termination 20 | 6 |
| `zcalloc` | 1 / 1 | ub_associated 1 | 1 |

Total: confirmed_divergence 397, confirmed_termination 771, inconclusive 28, instrument_only 31, not_reproducible 94, ub_associated 1374, ub_associated_termination 47  *(re-classified offline 2026-09-09: 31 rows `confirmed_termination` → `instrument_only`, no-sanitizer replay normal; see §7)*

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** §4–§6 regenerated from the archive on 2026-09-09; the campaign, replay and sample
are the original run of 2026-09-08 (the cell was re-run once with `--reuse-bins` after a first pass
left an empty corpus; §3a records the run that produced the data).

**What the cell says.** 552 matched, 121 planned (431 struct-invariant signatures, same classes as
c2rust), **96 built** — this translation reshapes fewer of the duplicate-symbol units, so 25 rather
than 74 fail to build — 79 exported, corpus 1 792; reach 144/564 functions (0.255),
9 815/37 297 regions (0.263). Fifteen boundaries were Rust-side crash-all in preflight with C clean
(`check_*_option` ×4, `err_option_arg`, `opng_init_iterations`, `opng_iterate`,
`opng_optimize_impl`, `panic`, `opng_print_fsize_*`, `opng_free`, plus the five contract-terminators
shared with c2rust): pre-accepted so the campaign could run, each adjudicated individually below.

The sample confirms **771 `confirmed_termination` + 397 `confirmed_divergence` on 11 boundaries** (31
further rows panicked only in the ASan build and were re-classified `instrument_only` on 2026-09-09,
when the classifier was fixed to require the no-sanitizer replay to trap). Every row was read to its
root cause — `TRIAGE.md` in this directory carries the table. Six defects and two non-defects:
* **S1 re-found** — `crc32` 26/26 divergence through `crc32_z`'s `is_null → is_empty`.
* **S2 re-found** — `adler32_z`'s rewritten block loop: index-out-of-bounds when fewer than 8 bytes
  remain after a 16-byte step, wrong sums otherwise (`compress2` 169 + 99, `compress` 1 + 1, `adler32`
  1 + 1).
* **S19 (new)** — `send_bits`' flush shifts a `u16` by `bi_valid == 16` (C promotes to int):
  `compress` 200, `compress2` 28 panics under overflow checks; a corrupted stream in release.
* **S20 (new)** — inflate's `if (state->offset > copy)` became a comparison of the state POINTER with
  the byte count: `uncompress` 169 divergence on `destLen` (false `invalid distance too far back`)
  + 4 NULL dereferences.
* **C15 (new)** — `inflate_fast` lost `from = out - dist`; the direct-copy branch builds a slice over
  NULL: `uncompress` 48 no-sanitizer panics.
* **S21 (new)** — `crc32_combine_`: `gf2_matrix_square` squares the wrong matrix and the loop swaps
  `even`/`odd`: 23 + 23 divergence on the return value.
* **C13**, **S16** — `opng_free` 3/3, `opng_strcasecmp` 55/55, as promoted on 2026-09-09.
* **Not defects**: `optimize_cmf` 200 (`--z_cinfo` on unsigned 0 is a defined wrap in C and identical in
  a Rust release build; only the overflow check differs) and `bmp_memset_bytes` 117 (harness
  input-model gap: `memset(ptr + offset, …)` with an unbounded `offset`; both sides compute an
  out-of-bounds pointer).

**Not established.** Whether the allocator-mismatch half of `opng_free` (Rust allocator releasing
malloc memory) is observable; S19's release-build corruption is read from the source, the sample only
shows the debug panic.
