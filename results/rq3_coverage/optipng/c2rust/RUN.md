# optipng × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 552 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 128 |
| built | 54 |
| executed (corpus > 0) | 54 |
| coverage exported | 47 |

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
- **4** × signature: callback parameter error_fn deferred: function pointers (callback binding) not 
- **2** × Rust signature has 2 parameters, C has 1: reshaped API, no positional bridge
- **2** × signature: struct-invariant param dest: z_stream_s has pointer field 'next_in' (needs inva
- **2** × signature: struct-invariant param getter_ptr: minitiff_getter has pointer field 'get_ushor
- **2** × signature: struct-invariant param infile: FILE has pointer field '_IO_read_ptr' (needs inv
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
- **1** × output_buffer is written by the callee but the Rust parameter is *const Bytef (const)

Planned but not built:

- `adler32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `adler32_z`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `app_finish`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `app_init`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `app_print_cntrl`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `app_progress`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bi_reverse`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bmp_get_dword`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bmp_get_word`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bmp_memset_bytes`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bmp_memset_halfbytes`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `bmp_process_mask`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `check_num_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `check_obj_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `check_power2_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `check_rangeset_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `crc32_big`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `crc32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `crc32_little`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `crc32_z`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `err_option_arg`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `gf2_matrix_square`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `gf2_matrix_times`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_allow_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_check_idat_size`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_clear_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_destroy_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_finalize`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_finish_iterations`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_free`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_init_iteration`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_init_iterations`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_init_read_data`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_init_write_data`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_insert_palette_entry`: error[E0425]: cannot find value `num_palette_c` in this scope
error[E0425]: cannot find value `num_palette_r` in this sc
- `opng_is_apng_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_is_image_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_iterate`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_optimize_impl`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_print_error`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_print_fsize_difference`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_print_fsize_ratio`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_print_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_print_warning`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_sprint_uratio_impl`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_str2ulong`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_strcasecmp`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_strltrim`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_strpbrk_digit`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_strtail`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `opng_ullratio_to_factor_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_ullratio_to_percent_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_ulratio_to_factor_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_ulratio_to_percent_string`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `optimize_cmf`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `panic`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `parse_args`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_do_read_interlace`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `png_format_number`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `png_get_int_32`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_get_uint_16`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_get_uint_32`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_read_filter_row_avg`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_read_filter_row_paeth_1byte_pixel`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_read_filter_row_paeth_multibyte_pixel`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_read_filter_row_sub`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_read_filter_row_up`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `png_sig_cmp`: error[E0308]: arguments to this function are incorrect
error[E0308]: arguments to this function are incorrect
error: cou
- `png_warning_parameter`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `png_warning_parameter_signed`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `png_warning_parameter_unsigned`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_c2rust-fuzz` (bin "optip
- `process_files`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `syncsearch`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")
- `tr_static_init`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_c2rust-fuzz` (bin "optipng_c2rust_ft")

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `DefaultError` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `DefaultWarning` | yes | 16 | 0 | normal 16 | batch |
| `ErrorAlloc` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `add_one_chunk` | yes | 5 | 16 | normal 4, signal 1 | per-input (4/5 completed) |
| `adler32` | no | 32 | 0 | normal 32 | batch |
| `adler32_combine` | no | 10 | 0 | normal 10 | batch |
| `adler32_combine64` | no | 10 | 0 | normal 10 | batch |
| `compress` | no | 411 | 0 | normal 411 | batch |
| `compress2` | no | 488 | 0 | normal 488 | batch |
| `compressBound` | no | 8 | 0 | normal 8 | batch |
| `crc32` | no | 30 | 0 | normal 30 | batch |
| `crc32_combine` | no | 68 | 6987 | normal 40, signal 28 | per-input (67/68 completed) |
| `crc32_combine64` | no | 68 | 6980 | normal 40, signal 28 | per-input (67/68 completed) |
| `default_error_handler` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `default_warning_handler` | yes | 16 | 0 | normal 16 | batch |
| `get_crc_table` | no | 1 | 0 | normal 1 | batch |
| `get_ulong_i` | yes | 5 | 0 | normal 5 | batch |
| `get_ulong_m` | yes | 5 | 0 | normal 5 | batch |
| `get_ushort_i` | yes | 3 | 0 | normal 3 | batch |
| `get_ushort_m` | yes | 3 | 0 | normal 3 | batch |
| `opng_bitset_count` | no | 8 | 0 | normal 8 | batch |
| `opng_bitset_find_first` | no | 32 | 0 | normal 32 | batch |
| `opng_bitset_find_last` | no | 33 | 0 | normal 33 | batch |
| `opng_bitset_find_next` | no | 12 | 0 | normal 12 | batch |
| `opng_bitset_find_prev` | no | 11 | 0 | normal 11 | batch |
| `opng_get_alpha_row` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `opng_os_test` | no | 44 | 0 | normal 44 | batch |
| `opng_os_test_eq` | no | 42 | 0 | normal 42 | batch |
| `opng_path_make_backup` | no | 20 | 0 | normal 20 | batch |
| `opng_path_replace_dir` | no | 58 | 0 | normal 58 | batch |
| `opng_path_replace_ext` | no | 64 | 0 | normal 64 | batch |
| `opng_strparse_rangeset_to_bitset` | no | 90 | 0 | normal 90 | batch |
| `png_access_version_number` | no | 1 | 0 | normal 1 | batch |
| `png_do_write_interlace` | no | 28 | 62 | normal 28 | batch |
| `png_gt` | yes | 6 | 0 | normal 6 | batch |
| `png_safecat` | no | 11 | 152 | normal 10, signal 1 | per-input (10/11 completed) |
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
| `pnm_mem_size` | no | 32 | 0 | normal 32 | batch |
| `pnm_raw_sample_size` | no | 21 | 0 | normal 21 | batch |
| `uncompress` | no | 347 | 0 | normal 347 | batch |
| `zError` | no | 1 | 2 | signal 1 | failed rc=1 |
| `zcalloc` | no | 1 | 1 | normal 1 | batch |
| `zcfree` | no | 1 | 0 | normal 1 | batch |
| `zlibCompileFlags` | no | 1 | 0 | normal 1 | batch |
| `zlibVersion` | no | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no transpiled test target; denominator only

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 54 harnesses, 6 crash-all (`DefaultError` accepted, `ErrorAlloc` accepted, `default_error_handler` accepted, `opng_get_alpha_row` accepted, `pngx_gif_error` accepted, `pngx_tiff_error` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`, `3d43f75aea807da1 (bin reused from the killed run)` — more than one: see deviations.

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 2069 |
| signal | 65 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `DefaultError` | 3 / 3 | ub_associated 3 | 1 |
| `ErrorAlloc` | 3 / 3 | ub_associated 3 | 1 |
| `add_one_chunk` | 17 / 17 | ub_associated 17 | 2 |
| `crc32_combine` | 228 / 528 | inconclusive 10, instrument_only 104, ub_associated 114 | 3 |
| `crc32_combine64` | 228 / 528 | inconclusive 10, instrument_only 104, ub_associated 114 | 3 |
| `default_error_handler` | 3 / 3 | ub_associated 3 | 1 |
| `opng_get_alpha_row` | 3 / 3 | ub_associated 3 | 1 |
| `png_do_write_interlace` | 62 / 62 | inconclusive 1, not_reproducible 42, ub_associated 19 | 4 |
| `png_safecat` | 153 / 153 | ub_associated 153 | 1 |
| `pngx_gif_error` | 3 / 3 | ub_associated 3 | 1 |
| `pngx_tiff_error` | 3 / 3 | ub_associated 3 | 1 |
| `zError` | 3 / 3 | ub_associated_termination 3 | 1 |
| `zcalloc` | 1 / 1 | ub_associated 1 | 1 |

Total: inconclusive 21, instrument_only 208, not_reproducible 42, ub_associated 436, ub_associated_termination 3

<!-- prose -->

