# optipng × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 552 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 121 |
| built | 55 |
| executed (corpus > 0) | 55 |
| coverage exported | 48 |

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
- **2** × struct parameter has Rust type Option<&'a1 mut crate::src::libpng::png::png_row_info_struc
- **2** × struct parameter has Rust type Option<&'a1 crate::src::optipng::ratio::opng_ullratio>
- **2** × struct parameter has Rust type Option<&'a1 crate::src::optipng::optim::opng_ulratio>
- **2** × signature: unsupported: pointer-to-pointer-to-struct param png_ptr_ptr
- **2** × signature: struct-invariant param pp: png_struct_def has pointer field 'error_fn' (needs i
- **2** × struct parameter has Rust type Option<&'a1 crate::src::pngxtern::pngxrpnm::pnm_struct>
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
- **1** × output_buffer is written by the callee but the Rust parameter is * const std::os::raw::c_u

Planned but not built:

- `adler32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `app_finish`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `app_init`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `app_print_cntrl`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `app_progress`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bi_reverse`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_get_dword`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_get_word`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_memset_bytes`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_memset_halfbytes`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_process_mask`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `bmp_to_png_rows`: error[E0425]: cannot find value `height` in this scope
error[E0425]: cannot find value `height` in this scope
error[E042
- `check_num_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `check_obj_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `check_power2_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `check_rangeset_option`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `crc32_big`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `crc32_combine_`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `crc32_little`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `err_option_arg`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `gf2_matrix_square`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `gf2_matrix_times`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_allow_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_check_idat_size`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_clear_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_destroy_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_finalize`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_finish_iterations`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_free`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_init_iteration`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_init_iterations`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_init_read_data`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_init_write_data`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_insert_palette_entry`: error[E0425]: cannot find value `num_palette_c` in this scope
error[E0425]: cannot find value `num_palette_r` in this sc
- `opng_is_apng_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_is_image_chunk`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_iterate`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_optimize_impl`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_print_error`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_print_fsize_difference`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_print_fsize_ratio`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_print_image_info`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_print_warning`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_sprint_uratio_impl`: error[E0425]: cannot find value `buffer_size` in this scope
error[E0425]: cannot find value `buffer_size` in this scope

- `opng_str2ulong`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_strcasecmp`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_strltrim`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_strpbrk_digit`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `opng_strtail`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `panic`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `parse_args`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_do_read_interlace`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_laertes-fuzz` (bin "opti
- `png_get_int_32`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_get_uint_16`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_get_uint_32`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_read_filter_row_avg`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_read_filter_row_paeth_1byte_pixel`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_read_filter_row_paeth_multibyte_pixel`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_read_filter_row_sub`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_read_filter_row_up`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `png_warning_parameter`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_laertes-fuzz` (bin "opti
- `png_warning_parameter_signed`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_laertes-fuzz` (bin "opti
- `png_warning_parameter_unsigned`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `optipng_laertes-fuzz` (bin "opti
- `process_files`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `syncsearch`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft
- `tr_static_init`: error: linking with `cc` failed: exit status: 1
error: could not compile `optipng_laertes-fuzz` (bin "optipng_laertes_ft

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `DefaultError` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `DefaultWarning` | yes | 16 | 0 | normal 16 | batch |
| `ErrorAlloc` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `add_one_chunk` | yes | 5 | 17 | normal 4, signal 1 | per-input (4/5 completed) |
| `adler32` | no | 32 | 0 | normal 32 | batch |
| `adler32_combine` | no | 10 | 0 | normal 10 | batch |
| `adler32_combine64` | no | 10 | 0 | normal 10 | batch |
| `adler32_z` | no | 32 | 0 | normal 32 | batch |
| `compress` | no | 13 | 0 | divergence 13 | batch |
| `compress2` | no | 16 | 0 | divergence 16 | batch |
| `compressBound` | no | 8 | 0 | normal 8 | batch |
| `crc32` | no | 30 | 0 | divergence 23, normal 7 | batch |
| `crc32_combine` | no | 68 | 6991 | normal 40, signal 28 | per-input (67/68 completed) |
| `crc32_combine64` | no | 68 | 6998 | normal 40, signal 28 | per-input (67/68 completed) |
| `crc32_z` | no | 30 | 0 | divergence 23, normal 7 | batch |
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
| `opng_os_test` | no | 43 | 0 | normal 43 | batch |
| `opng_os_test_eq` | no | 42 | 0 | normal 42 | batch |
| `opng_path_make_backup` | no | 20 | 0 | divergence 19, normal 1 | batch |
| `opng_path_replace_dir` | no | 58 | 0 | normal 58 | batch |
| `opng_path_replace_ext` | no | 63 | 0 | normal 63 | batch |
| `opng_strparse_rangeset_to_bitset` | no | 88 | 0 | normal 88 | batch |
| `optimize_cmf` | yes | 17 | 53 | normal 17 | batch |
| `png_access_version_number` | no | 1 | 0 | normal 1 | batch |
| `png_format_number` | no | 1 | 2 | signal 1 | failed rc=1 |
| `png_gt` | yes | 6 | 0 | normal 6 | batch |
| `png_safecat` | no | 11 | 151 | normal 10, signal 1 | per-input (10/11 completed) |
| `png_save_int_32` | no | 5 | 0 | normal 5 | batch |
| `png_save_uint_16` | no | 5 | 0 | normal 5 | batch |
| `png_save_uint_32` | no | 5 | 0 | normal 5 | batch |
| `png_sig_cmp` | no | 28 | 0 | normal 28 | batch |
| `png_zalloc` | no | 8 | 0 | normal 8 | batch |
| `png_zfree` | no | 1 | 0 | normal 1 | batch |
| `pngx_gif_error` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `pngx_gif_warning` | yes | 16 | 0 | normal 16 | batch |
| `pngx_tiff_error` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `pngx_tiff_warning` | yes | 15 | 0 | normal 15 | batch |
| `pnm_is_valid` | no | 29 | 0 | normal 29 | batch |
| `uncompress` | no | 60 | 1378 | divergence 4, normal 56 | batch |
| `zError` | no | 1 | 2 | signal 1 | failed rc=1 |
| `zcalloc` | no | 1 | 1 | normal 1 | batch |
| `zcfree` | no | 1 | 0 | normal 1 | batch |
| `zlibCompileFlags` | no | 1 | 0 | normal 1 | batch |
| `zlibVersion` | no | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no transpiled test target; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 55 harnesses, 6 crash-all (`DefaultError` accepted, `ErrorAlloc` accepted, `default_error_handler` accepted, `png_format_number` accepted, `pngx_gif_error` accepted, `pngx_tiff_error` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`, `3d43f75aea807da1 (bin reused from the killed run)` — more than one: see deviations.

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 820 | 0 | 71 | 0 | 0 | 71 | 749 | 0.000 | 0.087 |
| regions | 49009 | 0 | 6611 | 0 | 0 | 6611 | 42398 | 0.000 | 0.135 |

Sanity checks: function pass, region pass. Harnesses unioned: 48. Identities outside the universe (excluded, never added): 0 fn / 5 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `DefaultWarning` | 16 | 16 | 16 | 16 | 16 |
| `add_one_chunk` | 5 | 5 | 5 | 5 | 5 |
| `adler32` | 32 | 32 | 32 | 32 | 32 |
| `adler32_combine` | 10 | 10 | 10 | 10 | 10 |
| `adler32_combine64` | 10 | 10 | 10 | 10 | 10 |
| `adler32_z` | 32 | 32 | 32 | 32 | 32 |
| `compress` | 12 | 13 | 13 | 13 | 13 |
| `compress2` | 15 | 16 | 16 | 16 | 16 |
| `compressBound` | 8 | 8 | 8 | 8 | 8 |
| `crc32` | 30 | 30 | 30 | 30 | 30 |
| `crc32_combine` | 61 | 68 | 68 | 68 | 68 |
| `crc32_combine64` | 62 | 68 | 68 | 68 | 68 |
| `crc32_z` | 30 | 30 | 30 | 30 | 30 |
| `default_warning_handler` | 16 | 16 | 16 | 16 | 16 |
| `get_ulong_i` | 5 | 5 | 5 | 5 | 5 |
| `get_ulong_m` | 5 | 5 | 5 | 5 | 5 |
| `get_ushort_i` | 3 | 3 | 3 | 3 | 3 |
| `get_ushort_m` | 3 | 3 | 3 | 3 | 3 |
| `opng_bitset_count` | 8 | 8 | 8 | 8 | 8 |
| `opng_bitset_find_first` | 32 | 32 | 32 | 32 | 32 |
| `opng_bitset_find_last` | 33 | 33 | 33 | 33 | 33 |
| `opng_bitset_find_next` | 12 | 12 | 12 | 12 | 12 |
| `opng_bitset_find_prev` | 11 | 11 | 11 | 11 | 11 |
| `opng_os_test` | 43 | 43 | 43 | 43 | 43 |
| `opng_os_test_eq` | 40 | 42 | 42 | 42 | 42 |
| `opng_path_make_backup` | 20 | 20 | 20 | 20 | 20 |
| `opng_path_replace_dir` | 55 | 57 | 58 | 58 | 58 |
| `opng_path_replace_ext` | 62 | 63 | 63 | 63 | 63 |
| `opng_strparse_rangeset_to_bitset` | 87 | 88 | 88 | 88 | 88 |
| `optimize_cmf` | 17 | 17 | 17 | 17 | 17 |
| `png_gt` | 6 | 6 | 6 | 6 | 6 |
| `png_safecat` | 11 | 11 | 11 | 11 | 11 |
| `png_save_int_32` | 5 | 5 | 5 | 5 | 5 |
| `png_save_uint_16` | 5 | 5 | 5 | 5 | 5 |
| `png_save_uint_32` | 5 | 5 | 5 | 5 | 5 |
| `png_sig_cmp` | 28 | 28 | 28 | 28 | 28 |
| `png_zalloc` | 8 | 8 | 8 | 8 | 8 |
| `pngx_gif_warning` | 16 | 16 | 16 | 16 | 16 |
| `pngx_tiff_warning` | 15 | 15 | 15 | 15 | 15 |
| `pnm_is_valid` | 29 | 29 | 29 | 29 | 29 |
| `uncompress` | 36 | 37 | 51 | 59 | 60 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 836 |
| divergence | 98 |
| signal | 65 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `DefaultError` | 3 / 3 | ub_associated 3 | 1 |
| `ErrorAlloc` | 3 / 3 | ub_associated_termination 3 | 1 |
| `add_one_chunk` | 18 / 18 | ub_associated 18 | 2 |
| `compress` | 13 / 13 | confirmed_divergence 13 | 1 |
| `compress2` | 16 / 16 | confirmed_divergence 15, ub_associated_value 1 | 2 |
| `crc32` | 23 / 23 | confirmed_divergence 23 | 1 |
| `crc32_combine` | 228 / 528 | inconclusive 10, instrument_only 104, ub_associated 114 | 3 |
| `crc32_combine64` | 228 / 528 | inconclusive 10, instrument_only 106, ub_associated 112 | 3 |
| `crc32_z` | 23 / 23 | confirmed_divergence 23 | 1 |
| `default_error_handler` | 3 / 3 | ub_associated 3 | 1 |
| `opng_path_make_backup` | 19 / 19 | confirmed_divergence 19 | 1 |
| `optimize_cmf` | 53 / 53 | inconclusive 53 | 1 |
| `png_format_number` | 3 / 3 | ub_associated 3 | 1 |
| `png_safecat` | 152 / 152 | ub_associated 152 | 1 |
| `pngx_gif_error` | 3 / 3 | ub_associated 3 | 1 |
| `pngx_tiff_error` | 3 / 3 | ub_associated 3 | 1 |
| `uncompress` | 204 / 504 | confirmed_divergence 4, instrument_only 200 | 2 |
| `zError` | 3 / 3 | ub_associated_termination 3 | 1 |
| `zcalloc` | 1 / 1 | ub_associated 1 | 1 |

Total: confirmed_divergence 97, inconclusive 73, instrument_only 410, ub_associated 412, ub_associated_termination 6, ub_associated_value 1  *(re-classified offline 2026-09-09: 200 rows `confirmed_termination` → `instrument_only`, no-sanitizer replay normal; see §7)*

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** This is the rerun of 2026-09-09 (`--reuse-bins`; the first pass had stopped at
preflight review). §4–§6 from the rerun's archive.

**What the cell says.** 552 matched, 121 planned, 55 built (66 not built: the same duplicate-symbol
units as c2rust), 48 exported, corpus 999; reach 71/820 functions (0.087), 6 611/49 009 regions
(0.135) — the universe includes the Laertes runtime (as for qsort × Laertes, 83 vs 3), so the
fraction is not comparable with the other two optipng cells; the only-ours counts are. Six
pre-accepted C-side crash-alls (the five contract-terminators and `png_format_number`).

Sample: **97 `confirmed_divergence`, every one the severed-init law** (the 200 `uncompress` rows that had read as terminations were re-classified `instrument_only` on 2026-09-09: they panic only in the ASan build, see below)
(178 `laertes_init_*` defined in this crate, 0 called):
* `crc32` 23/23, `crc32_z` 23/23 — `crc_table` zero: **S4 re-found**; `compress` 13/13 and
  `compress2` 15/16 divergence are deflate's zeroed tables, the same law, recorded under S4.
* `opng_path_make_backup` 19/19 divergence — `bak_extname` (".bak") zero, the backup path equals
  the input path. **Manifest S17**.
* `uncompress` 4 divergence (+ 200 sanitizer-only panics) — inflate's `order[19]` zero, `lens[1..18]`
  never written, so `inflate_table` reads whatever the heap holds: ASan's fill pattern 0xBEBE = 48830
  indexes `count[16]` out of bounds (backtrace uncompress → uncompress2 → inflate → inflate_table),
  a zeroed heap gives a silently wrong result — every one of the 200 no-sanitizer replays returned
  normally. Termination is instrument-dependent; the defect is semantic. **Manifest S18** (was C14).
The other verdicts: 412 `ub_associated`, 410 `instrument_only` (210 + the 200 above), 73 `inconclusive`,
6 `ub_associated_termination`, 1 `ub_associated_value`.

**Not established.** Nothing beyond the initialization family was reached: every deeper boundary
dies on a zeroed table first.
