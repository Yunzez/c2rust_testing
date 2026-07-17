
extern "C" {
    
    
    
    
    
    #[no_mangle]
    fn fclose(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fopen(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> * mut crate::src::gifread::gifread::_IO_FILE;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fwrite(_: * const core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn free(__ptr: * mut core::ffi::c_void);
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strcpy(_: * mut std::os::raw::c_char, _: * const std::os::raw::c_char)
     -> * mut std::os::raw::c_char;
    #[no_mangle]
    fn strcmp(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn _setjmp(_: * mut crate::src::optipng::optim::__jmp_buf_tag) -> std::os::raw::c_int;
    #[no_mangle]
    fn longjmp(_: * mut crate::src::optipng::optim::__jmp_buf_tag, _: std::os::raw::c_int) -> !;
}
pub use crate::src::libpng::png::png_create_info_struct;
pub use crate::src::libpng::png::png_data_freer;
pub use crate::src::libpng::png::png_get_io_ptr;
pub use crate::src::libpng::png::png_handle_as_unknown;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngget::png_get_IHDR;
pub use crate::src::libpng::pngget::png_get_PLTE;
pub use crate::src::libpng::pngget::png_get_bKGD;
pub use crate::src::libpng::pngget::png_get_hIST;
pub use crate::src::libpng::pngget::png_get_image_height;
pub use crate::src::libpng::pngget::png_get_io_state;
pub use crate::src::libpng::pngget::png_get_rows;
pub use crate::src::libpng::pngget::png_get_sBIT;
pub use crate::src::libpng::pngget::png_get_tRNS;
pub use crate::src::libpng::pngget::png_get_unknown_chunks;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngread::png_create_read_struct;
pub use crate::src::libpng::pngread::png_destroy_read_struct;
pub use crate::src::libpng::pngrio::png_set_read_fn;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_bKGD;
pub use crate::src::libpng::pngset::png_set_hIST;
pub use crate::src::libpng::pngset::png_set_keep_unknown_chunks;
pub use crate::src::libpng::pngset::png_set_rows;
pub use crate::src::libpng::pngset::png_set_sBIT;
pub use crate::src::libpng::pngset::png_set_tRNS;
pub use crate::src::libpng::pngset::png_set_unknown_chunk_location;
pub use crate::src::libpng::pngset::png_set_unknown_chunks;
pub use crate::src::libpng::pngset::png_set_user_limits;
pub use crate::src::libpng::pngwio::png_set_write_fn;
pub use crate::src::libpng::pngwrite::png_create_write_struct;
pub use crate::src::libpng::pngwrite::png_destroy_write_struct;
pub use crate::src::libpng::pngwrite::png_set_compression_level;
pub use crate::src::libpng::pngwrite::png_set_compression_mem_level;
pub use crate::src::libpng::pngwrite::png_set_compression_strategy;
pub use crate::src::libpng::pngwrite::png_set_compression_window_bits;
pub use crate::src::libpng::pngwrite::png_set_filter;
pub use crate::src::libpng::pngwrite::png_write_png;
pub use crate::src::libpng::pngwutil::png_save_uint_32;
pub use crate::src::libpng::pngwutil::png_write_chunk;
pub use crate::src::libpng::pngwutil::png_write_sig;
pub use crate::src::opngreduc::opngreduc::opng_reduce_image;
pub use crate::src::opngreduc::opngreduc::opng_validate_image;
pub use crate::src::optipng::bitset::opng_bitset_count;
pub use crate::src::optipng::bitset::opng_bitset_find_first;
pub use crate::src::optipng::bitset::opng_strparse_rangeset_to_bitset;
pub use crate::src::optipng::ioutil::opng_fgetsize;
pub use crate::src::optipng::ioutil::opng_fseeko;
pub use crate::src::optipng::ioutil::opng_ftello;
pub use crate::src::optipng::ioutil::opng_fwriteo;
pub use crate::src::optipng::ioutil::opng_os_copy_attr;
pub use crate::src::optipng::ioutil::opng_os_create_dir;
pub use crate::src::optipng::ioutil::opng_os_rename;
pub use crate::src::optipng::ioutil::opng_os_test;
pub use crate::src::optipng::ioutil::opng_os_test_eq;
pub use crate::src::optipng::ioutil::opng_os_unlink;
pub use crate::src::optipng::ioutil::opng_path_make_backup;
pub use crate::src::optipng::ioutil::opng_path_replace_dir;
pub use crate::src::optipng::ioutil::opng_path_replace_ext;
pub use crate::src::optipng::ratio::opng_ulratio_to_factor_string;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows;
pub use crate::src::pngxtern::pngxread::pngx_read_image;
pub use crate::src::zlib::crc32::crc32;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [std::os::raw::c_ulong; 16],
}
impl __sigset_t {
    pub const fn new() -> Self {
        __sigset_t {
        __val: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]
        }
    }
}

impl std::default::Default for __sigset_t {
    fn default() -> Self { __sigset_t::new() }
}

pub type opng_bitset_t = std::os::raw::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_options {
    pub backup: std::os::raw::c_int,
    pub clobber: std::os::raw::c_int,
    pub debug: std::os::raw::c_int,
    pub fix: std::os::raw::c_int,
    pub force: std::os::raw::c_int,
    pub full: std::os::raw::c_int,
    pub preserve: std::os::raw::c_int,
    pub quiet: std::os::raw::c_int,
    pub simulate: std::os::raw::c_int,
    pub verbose: std::os::raw::c_int,
    pub out_name: * const std::os::raw::c_char,
    pub dir_name: * const std::os::raw::c_char,
    pub log_name: * const std::os::raw::c_char,
    pub interlace: std::os::raw::c_int,
    pub nb: std::os::raw::c_int,
    pub nc: std::os::raw::c_int,
    pub np: std::os::raw::c_int,
    pub nz: std::os::raw::c_int,
    pub optim_level: std::os::raw::c_int,
    pub compr_level_set: std::os::raw::c_uint,
    pub mem_level_set: std::os::raw::c_uint,
    pub strategy_set: std::os::raw::c_uint,
    pub filter_set: std::os::raw::c_uint,
    pub window_bits: std::os::raw::c_int,
    pub snip: std::os::raw::c_int,
    pub strip_all: std::os::raw::c_int,
}
impl opng_options {
    pub const fn new() -> Self {
        opng_options {
        backup: 0,
        clobber: 0,
        debug: 0,
        fix: 0,
        force: 0,
        full: 0,
        preserve: 0,
        quiet: 0,
        simulate: 0,
        verbose: 0,
        out_name: (0 as * const std::os::raw::c_char),
        dir_name: (0 as * const std::os::raw::c_char),
        log_name: (0 as * const std::os::raw::c_char),
        interlace: 0,
        nb: 0,
        nc: 0,
        np: 0,
        nz: 0,
        optim_level: 0,
        compr_level_set: 0,
        mem_level_set: 0,
        strategy_set: 0,
        filter_set: 0,
        window_bits: 0,
        snip: 0,
        strip_all: 0
        }
    }
}

impl std::default::Default for opng_options {
    fn default() -> Self { opng_options::new() }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ui {
    pub printf_fn: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,...) -> ()>,
    pub print_cntrl_fn: Option<unsafe extern "C"  fn(_: std::os::raw::c_int,) -> ()>,
    pub progress_fn: Option<unsafe extern "C"  fn(_: std::os::raw::c_ulong,_: std::os::raw::c_ulong,) -> ()>,
    pub panic_fn: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()>,
}
impl opng_ui {
    pub const fn new() -> Self {
        opng_ui {
        printf_fn: None,
        print_cntrl_fn: None,
        progress_fn: None,
        panic_fn: None
        }
    }
}

impl std::default::Default for opng_ui {
    fn default() -> Self { opng_ui::new() }
}

/*
 * The optimization engine.
 * (Since the engine is not thread-safe, there isn't much to put in here...)
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_engine_struct {
    pub started: std::os::raw::c_int,
}
impl opng_engine_struct {
    pub const fn new() -> Self {
        opng_engine_struct {
        started: 0
        }
    }
}

impl std::default::Default for opng_engine_struct {
    fn default() -> Self { opng_engine_struct::new() }
}

/*
 * The optimization process summary.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_summary_struct {
    pub file_count: std::os::raw::c_uint,
    pub err_count: std::os::raw::c_uint,
    pub fix_count: std::os::raw::c_uint,
    pub snip_count: std::os::raw::c_uint,
}
impl opng_summary_struct {
    pub const fn new() -> Self {
        opng_summary_struct {
        file_count: 0,
        err_count: 0,
        fix_count: 0,
        snip_count: 0
        }
    }
}

impl std::default::Default for opng_summary_struct {
    fn default() -> Self { opng_summary_struct::new() }
}

/*
 * The optimized image.
 */
// #[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_image_struct<'a> {
    pub width: std::os::raw::c_uint,
    pub height: std::os::raw::c_uint,
    pub bit_depth: std::os::raw::c_int,
    pub color_type: std::os::raw::c_int,
    pub compression_type: std::os::raw::c_int,
    pub filter_type: std::os::raw::c_int,
    pub interlace_type: std::os::raw::c_int,
    pub row_pointers: * mut * mut std::os::raw::c_uchar,
    pub palette: * mut crate::src::libpng::png::png_color_struct,
    pub num_palette: std::os::raw::c_int,
    pub background_ptr: Option<&'a mut crate::src::libpng::png::png_color_16_struct>,
    pub background: crate::src::libpng::png::png_color_16_struct,
    pub hist: * mut std::os::raw::c_ushort,
    pub sig_bit_ptr: Option<&'a mut crate::src::libpng::png::png_color_8_struct>,
    pub sig_bit: crate::src::libpng::png::png_color_8_struct,
    pub trans_alpha: * mut std::os::raw::c_uchar,
    pub num_trans: std::os::raw::c_int,
    pub trans_color_ptr: Option<&'a mut crate::src::libpng::png::png_color_16_struct>,
    pub trans_color: crate::src::libpng::png::png_color_16_struct,
    pub unknowns: * mut crate::src::libpng::png::png_unknown_chunk_t,
    pub num_unknowns: std::os::raw::c_int,
}
impl<'a> opng_image_struct<'a> {
    pub const fn new() -> Self {
        opng_image_struct {
        width: 0,
        height: 0,
        bit_depth: 0,
        color_type: 0,
        compression_type: 0,
        filter_type: 0,
        interlace_type: 0,
        row_pointers: (0 as * mut * mut std::os::raw::c_uchar),
        palette: (0 as * mut crate::src::libpng::png::png_color_struct),
        num_palette: 0,
        background_ptr: None,
        background: crate::src::libpng::png::png_color_16_struct::new(),
        hist: (0 as * mut std::os::raw::c_ushort),
        sig_bit_ptr: None,
        sig_bit: crate::src::libpng::png::png_color_8_struct::new(),
        trans_alpha: (0 as * mut std::os::raw::c_uchar),
        num_trans: 0,
        trans_color_ptr: None,
        trans_color: crate::src::libpng::png::png_color_16_struct::new(),
        unknowns: (0 as * mut crate::src::libpng::png::png_unknown_chunk_t),
        num_unknowns: 0
        }
    }
}

impl<'a> std::default::Default for opng_image_struct<'a> {
    fn default() -> Self { opng_image_struct::new() }
}

pub type png_unknown_chunkp = * mut crate::src::libpng::png::png_unknown_chunk_t;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk_t;
// #[derive(Copy, Clone)]

pub type png_unknown_chunk_t = crate::src::libpng::png::png_unknown_chunk_t;
pub type png_byte = std::os::raw::c_uchar;
pub type png_size_t = std::os::raw::c_ulong;
pub type png_color_16 = crate::src::libpng::png::png_color_16_struct;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_color_16p = * mut crate::src::libpng::png::png_color_16_struct;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_color_8 = crate::src::libpng::png::png_color_8_struct;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_color_8p = * mut crate::src::libpng::png::png_color_8_struct;
pub type png_uint_16p = * mut std::os::raw::c_ushort;
pub type png_colorp = * mut crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
pub type png_uint_32 = std::os::raw::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub etmp: * const std::os::raw::c_char,
}
impl C2RustUnnamed {
    pub const fn new() -> Self {
        C2RustUnnamed {
        etmp: (0 as * const std::os::raw::c_char)
        }
    }
}

impl std::default::Default for C2RustUnnamed {
    fn default() -> Self { C2RustUnnamed::new() }
}

/*
 * optim.c
 * The main PNG optimization engine.
 *
 * Copyright (C) 2001-2017 Cosmin Truta and the Contributing Authors.
 *
 * This software is distributed under the zlib license.
 * Please see the accompanying LICENSE file.
 */
/*
 * User exception setup.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exception_context {
    pub penv: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1],
    pub caught: std::os::raw::c_int,
    pub v: crate::src::optipng::optim::C2RustUnnamed,
}
impl exception_context {
    pub const fn new() -> Self {
        exception_context {
        penv: (0 as * mut [crate::src::optipng::optim::__jmp_buf_tag; 1]),
        caught: 0,
        v: crate::src::optipng::optim::C2RustUnnamed::new()
        }
    }
}

impl std::default::Default for exception_context {
    fn default() -> Self { exception_context::new() }
}

pub type jmp_buf = [crate::src::optipng::optim::__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [std::os::raw::c_long; 8],
    pub __mask_was_saved: std::os::raw::c_int,
    pub __saved_mask: crate::src::optipng::optim::__sigset_t,
}
impl __jmp_buf_tag {
    pub const fn new() -> Self {
        __jmp_buf_tag {
        __jmpbuf: [0,0,0,0,0,0,0,0,],
        __mask_was_saved: 0,
        __saved_mask: crate::src::optipng::optim::__sigset_t::new()
        }
    }
}

impl std::default::Default for __jmp_buf_tag {
    fn default() -> Self { __jmp_buf_tag::new() }
}

pub type __jmp_buf = [std::os::raw::c_long; 8];
pub const INPUT_HAS_MULTIPLE_IMAGES: C2RustUnnamed_0 = 16;
/*
 * The optimization process.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_process_struct {
    pub status: std::os::raw::c_uint,
    pub num_iterations: std::os::raw::c_int,
    pub in_datastream_offset: std::os::raw::c_long,
    pub in_file_size: std::os::raw::c_ulong,
    pub out_file_size: std::os::raw::c_ulong,
    pub in_idat_size: std::os::raw::c_ulong,
    pub out_idat_size: std::os::raw::c_ulong,
    pub best_idat_size: std::os::raw::c_ulong,
    pub max_idat_size: std::os::raw::c_ulong,
    pub in_plte_trns_size: std::os::raw::c_uint,
    pub out_plte_trns_size: std::os::raw::c_uint,
    pub reductions: std::os::raw::c_uint,
    pub compr_level_set: std::os::raw::c_uint,
    pub mem_level_set: std::os::raw::c_uint,
    pub strategy_set: std::os::raw::c_uint,
    pub filter_set: std::os::raw::c_uint,
    pub best_compr_level: std::os::raw::c_int,
    pub best_mem_level: std::os::raw::c_int,
    pub best_strategy: std::os::raw::c_int,
    pub best_filter: std::os::raw::c_int,
}
impl opng_process_struct {
    pub const fn new() -> Self {
        opng_process_struct {
        status: 0,
        num_iterations: 0,
        in_datastream_offset: 0,
        in_file_size: 0,
        out_file_size: 0,
        in_idat_size: 0,
        out_idat_size: 0,
        best_idat_size: 0,
        max_idat_size: 0,
        in_plte_trns_size: 0,
        out_plte_trns_size: 0,
        reductions: 0,
        compr_level_set: 0,
        mem_level_set: 0,
        strategy_set: 0,
        filter_set: 0,
        best_compr_level: 0,
        best_mem_level: 0,
        best_strategy: 0,
        best_filter: 0
        }
    }
}

impl std::default::Default for opng_process_struct {
    fn default() -> Self { opng_process_struct::new() }
}

pub type opng_fsize_t = std::os::raw::c_ulong;
pub type opng_foffset_t = std::os::raw::c_long;
pub const INPUT_HAS_ERRORS: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ulratio {
    pub num: std::os::raw::c_ulong,
    pub denom: std::os::raw::c_ulong,
}
impl opng_ulratio {
    pub const fn new() -> Self {
        opng_ulratio {
        num: 0,
        denom: 0
        }
    }
}

impl std::default::Default for opng_ulratio {
    fn default() -> Self { opng_ulratio::new() }
}

pub const INPUT_HAS_PNG_DATASTREAM: C2RustUnnamed_0 = 2;
pub type png_infopp = * mut * mut crate::src::libpng::png::png_info_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structpp = * mut * mut crate::src::libpng::png::png_struct_def;
pub type png_voidp = * mut core::ffi::c_void;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_const_bytep = * const std::os::raw::c_uchar;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_alloc_size_t = std::os::raw::c_ulong;
pub type png_flush_ptr<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,) -> ()>;
pub type png_const_charp = * const std::os::raw::c_char;
pub type uLong = std::os::raw::c_ulong;
pub type uInt = std::os::raw::c_uint;
pub type Bytef = std::os::raw::c_uchar;
pub type Byte = std::os::raw::c_uchar;
pub type png_rw_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut std::os::raw::c_uchar>,_: std::os::raw::c_ulong,) -> ()>;
pub const OUTPUT_NEEDS_NEW_IDAT: C2RustUnnamed_0 = 8192;
pub type png_error_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 std::os::raw::c_char>,) -> ()>;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_const_unknown_chunkp = * const crate::src::libpng::png::png_unknown_chunk_t;
pub type png_const_color_8p = * const crate::src::libpng::png::png_color_8_struct;
pub type png_const_uint_16p = * const std::os::raw::c_ushort;
pub type png_const_color_16p = * const crate::src::libpng::png::png_color_16_struct;
pub type png_const_colorp = * const crate::src::libpng::png::png_color_struct;
pub const OUTPUT_NEEDS_NEW_FILE: C2RustUnnamed_0 = 4096;
/*
 * The optimization level presets.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_preset {
    pub compr_level: * const std::os::raw::c_char,
    pub mem_level: * const std::os::raw::c_char,
    pub strategy: * const std::os::raw::c_char,
    pub filter: * const std::os::raw::c_char,
}
impl opng_preset {
    pub const fn new() -> Self {
        opng_preset {
        compr_level: (0 as * const std::os::raw::c_char),
        mem_level: (0 as * const std::os::raw::c_char),
        strategy: (0 as * const std::os::raw::c_char),
        filter: (0 as * const std::os::raw::c_char)
        }
    }
}

impl std::default::Default for opng_preset {
    fn default() -> Self { opng_preset::new() }
}

pub const INPUT_IS_PNG_FILE: C2RustUnnamed_0 = 1;
pub const INPUT_HAS_STRIPPED_DATA: C2RustUnnamed_0 = 64;
pub const INPUT_HAS_APNG: C2RustUnnamed_0 = 32;
pub const INPUT_HAS_DIGITAL_SIGNATURE: C2RustUnnamed_0 = 8;
pub const INPUT_HAS_PNG_SIGNATURE: C2RustUnnamed_0 = 4;
pub const INPUT_HAS_JUNK: C2RustUnnamed_0 = 128;
pub type png_unknown_chunkpp = * mut * mut crate::src::libpng::png::png_unknown_chunk_t;
pub type png_const_inforp = * const crate::src::libpng::png::png_info_def;
pub type png_const_charpp = * mut * const std::os::raw::c_char;
/*
 * Status flags.
 */
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const OUTPUT_HAS_ERRORS: C2RustUnnamed_0 = 16384;
#[no_mangle]
pub static mut the_exception_context: [crate::src::optipng::optim::exception_context; 1] =
    [crate::src::optipng::optim::exception_context::new(),]; unsafe fn laertes_init_the_exception_context() {
the_exception_context = [exception_context{penv: 0 as *const jmp_buf as *mut jmp_buf,
                       caught: 0,
                       v: C2RustUnnamed{etmp: 0 as *const std::os::raw::c_char,},};
        1];}//;
static mut presets: [crate::src::optipng::optim::opng_preset; 8] =
    [crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),crate::src::optipng::optim::opng_preset::new(),]; unsafe fn laertes_init_presets() {
presets = [{
         let mut init =
             opng_preset{compr_level:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0,5\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8-9\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0,5\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8-9\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"1-9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     },
     {
         let mut init =
             opng_preset{compr_level:
                             b"1-9\x00" as *const u8 as *const std::os::raw::c_char,
                         mem_level:
                             b"8-9\x00" as *const u8 as *const std::os::raw::c_char,
                         strategy:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,
                         filter:
                             b"0-\x00" as *const u8 as *const std::os::raw::c_char,};
         init
     }];}//;
/*
 * The filter table.
 */
static mut filter_table: [std::os::raw::c_int; 6] =
    [0,0,0,0,0,0,]; unsafe fn laertes_init_filter_table() {
filter_table = [0x8 as std::os::raw::c_int, 0x10 as std::os::raw::c_int, 0x20 as std::os::raw::c_int,
     0x40 as std::os::raw::c_int, 0x80 as std::os::raw::c_int,
     0x8 as std::os::raw::c_int | 0x10 as std::os::raw::c_int | 0x20 as std::os::raw::c_int |
         0x40 as std::os::raw::c_int | 0x80 as std::os::raw::c_int];}//;
/*
 * The chunks handled by OptiPNG.
 */
static mut sig_PLTE: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_PLTE() {
sig_PLTE = [0x50 as std::os::raw::c_int as png_byte, 0x4c as std::os::raw::c_int as png_byte,
     0x54 as std::os::raw::c_int as png_byte, 0x45 as std::os::raw::c_int as png_byte];}//;
static mut sig_tRNS: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_tRNS() {
sig_tRNS = [0x74 as std::os::raw::c_int as png_byte, 0x52 as std::os::raw::c_int as png_byte,
     0x4e as std::os::raw::c_int as png_byte, 0x53 as std::os::raw::c_int as png_byte];}//;
static mut sig_IDAT: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_IDAT() {
sig_IDAT = [0x49 as std::os::raw::c_int as png_byte, 0x44 as std::os::raw::c_int as png_byte,
     0x41 as std::os::raw::c_int as png_byte, 0x54 as std::os::raw::c_int as png_byte];}//;
static mut sig_IEND: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_IEND() {
sig_IEND = [0x49 as std::os::raw::c_int as png_byte, 0x45 as std::os::raw::c_int as png_byte,
     0x4e as std::os::raw::c_int as png_byte, 0x44 as std::os::raw::c_int as png_byte];}//;
static mut sig_bKGD: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_bKGD() {
sig_bKGD = [0x62 as std::os::raw::c_int as png_byte, 0x4b as std::os::raw::c_int as png_byte,
     0x47 as std::os::raw::c_int as png_byte, 0x44 as std::os::raw::c_int as png_byte];}//;
static mut sig_hIST: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_hIST() {
sig_hIST = [0x68 as std::os::raw::c_int as png_byte, 0x49 as std::os::raw::c_int as png_byte,
     0x53 as std::os::raw::c_int as png_byte, 0x54 as std::os::raw::c_int as png_byte];}//;
static mut sig_sBIT: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_sBIT() {
sig_sBIT = [0x73 as std::os::raw::c_int as png_byte, 0x42 as std::os::raw::c_int as png_byte,
     0x49 as std::os::raw::c_int as png_byte, 0x54 as std::os::raw::c_int as png_byte];}//;
static mut sig_dSIG: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_dSIG() {
sig_dSIG = [0x64 as std::os::raw::c_int as png_byte, 0x53 as std::os::raw::c_int as png_byte,
     0x49 as std::os::raw::c_int as png_byte, 0x47 as std::os::raw::c_int as png_byte];}//;
static mut sig_acTL: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_acTL() {
sig_acTL = [0x61 as std::os::raw::c_int as png_byte, 0x63 as std::os::raw::c_int as png_byte,
     0x54 as std::os::raw::c_int as png_byte, 0x4c as std::os::raw::c_int as png_byte];}//;
static mut sig_fcTL: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_fcTL() {
sig_fcTL = [0x66 as std::os::raw::c_int as png_byte, 0x63 as std::os::raw::c_int as png_byte,
     0x54 as std::os::raw::c_int as png_byte, 0x4c as std::os::raw::c_int as png_byte];}//;
static mut sig_fdAT: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_sig_fdAT() {
sig_fdAT = [0x66 as std::os::raw::c_int as png_byte, 0x64 as std::os::raw::c_int as png_byte,
     0x41 as std::os::raw::c_int as png_byte, 0x54 as std::os::raw::c_int as png_byte];}//;
static mut engine: crate::src::optipng::optim::opng_engine_struct = crate::src::optipng::optim::opng_engine_struct::new(); unsafe fn laertes_init_engine() {
engine = opng_engine_struct{started: 0,};}//;
static mut process: crate::src::optipng::optim::opng_process_struct =
    crate::src::optipng::optim::opng_process_struct::new(); unsafe fn laertes_init_process() {
process = opng_process_struct{status: 0,
                        num_iterations: 0,
                        in_datastream_offset: 0,
                        in_file_size: 0,
                        out_file_size: 0,
                        in_idat_size: 0,
                        out_idat_size: 0,
                        best_idat_size: 0,
                        max_idat_size: 0,
                        in_plte_trns_size: 0,
                        out_plte_trns_size: 0,
                        reductions: 0,
                        compr_level_set: 0,
                        mem_level_set: 0,
                        strategy_set: 0,
                        filter_set: 0,
                        best_compr_level: 0,
                        best_mem_level: 0,
                        best_strategy: 0,
                        best_filter: 0,};}//;
/*
 * The optimization process limits.
 */
static mut idat_size_max: std::os::raw::c_ulong =
    0; unsafe fn laertes_init_idat_size_max() {
idat_size_max = 0x7fffffff as std::os::raw::c_long as png_uint_32 as opng_fsize_t;}//;
static mut idat_size_max_string: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_idat_size_max_string() {
idat_size_max_string = b"2GB\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut summary: crate::src::optipng::optim::opng_summary_struct =
    crate::src::optipng::optim::opng_summary_struct::new(); unsafe fn laertes_init_summary() {
summary = opng_summary_struct{file_count: 0,
                        err_count: 0,
                        fix_count: 0,
                        snip_count: 0,};}//;
static mut image: crate::src::optipng::optim::opng_image_struct<'static> =
    crate::src::optipng::optim::opng_image_struct::new(); unsafe fn laertes_init_image() {
image = opng_image_struct{width: 0,
                      height: 0,
                      bit_depth: 0,
                      color_type: 0,
                      compression_type: 0,
                      filter_type: 0,
                      interlace_type: 0,
                      row_pointers:
                          0 as *const *mut png_byte as *mut *mut png_byte,
                      palette: 0 as *const png_color as *mut png_color,
                      num_palette: 0,
                      background_ptr:
                          Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None,
                      background:
                          png_color_16{index: 0,
                                       red: 0,
                                       green: 0,
                                       blue: 0,
                                       gray: 0,},
                      hist: 0 as *const png_uint_16 as *mut png_uint_16,
                      sig_bit_ptr:
                          Option::<&'_ mut crate::src::libpng::png::png_color_8_struct>::None,
                      sig_bit:
                          png_color_8{red: 0,
                                      green: 0,
                                      blue: 0,
                                      gray: 0,
                                      alpha: 0,},
                      trans_alpha: 0 as *const png_byte as *mut png_byte,
                      num_trans: 0,
                      trans_color_ptr:
                          Option::<&'_ mut crate::src::libpng::png::png_color_16_struct>::None,
                      trans_color:
                          png_color_16{index: 0,
                                       red: 0,
                                       green: 0,
                                       blue: 0,
                                       gray: 0,},
                      unknowns:
                          0 as *const png_unknown_chunk as
                              *mut png_unknown_chunk,
                      num_unknowns: 0,};}//;
/*
 * The user options.
 */
static mut options: crate::src::optipng::optim::opng_options =
    crate::src::optipng::optim::opng_options::new(); unsafe fn laertes_init_options() {
options = opng_options{backup: 0,
                 clobber: 0,
                 debug: 0,
                 fix: 0,
                 force: 0,
                 full: 0,
                 preserve: 0,
                 quiet: 0,
                 simulate: 0,
                 verbose: 0,
                 out_name: 0 as *const std::os::raw::c_char,
                 dir_name: 0 as *const std::os::raw::c_char,
                 log_name: 0 as *const std::os::raw::c_char,
                 interlace: 0,
                 nb: 0,
                 nc: 0,
                 np: 0,
                 nz: 0,
                 optim_level: 0,
                 compr_level_set: 0,
                 mem_level_set: 0,
                 strategy_set: 0,
                 filter_set: 0,
                 window_bits: 0,
                 snip: 0,
                 strip_all: 0,};}//;
/*
 * The user interface.
 */
static mut usr_printf:
       Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,...) -> ()> =
    None; unsafe fn laertes_init_usr_printf() {
usr_printf = None;}//;
static mut usr_print_cntrl: Option<unsafe extern "C"  fn(_: std::os::raw::c_int,) -> ()>
       =
    None; unsafe fn laertes_init_usr_print_cntrl() {
usr_print_cntrl = None;}//;
static mut usr_progress:
       Option<unsafe extern "C"  fn(_: std::os::raw::c_ulong,_: std::os::raw::c_ulong,) -> ()>
       =
    None; unsafe fn laertes_init_usr_progress() {
usr_progress = None;}//;
static mut usr_panic:
       Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()> =
    None; unsafe fn laertes_init_usr_panic() {
usr_panic = None;}//;
/*
 * More global variables, for quick access and bonus style points.
 */
static mut read_ptr: * mut crate::src::libpng::png::png_struct_def = (0 as * mut crate::src::libpng::png::png_struct_def); unsafe fn laertes_init_read_ptr() {
read_ptr = 0 as *const png_struct as *mut png_struct;}//;
static mut read_info_ptr: * mut crate::src::libpng::png::png_info_def = (0 as * mut crate::src::libpng::png::png_info_def); unsafe fn laertes_init_read_info_ptr() {
read_info_ptr = 0 as *const png_info as *mut png_info;}//;
static mut write_ptr: * mut crate::src::libpng::png::png_struct_def = (0 as * mut crate::src::libpng::png::png_struct_def); unsafe fn laertes_init_write_ptr() {
write_ptr = 0 as *const png_struct as *mut png_struct;}//;
static mut write_info_ptr: * mut crate::src::libpng::png::png_info_def = (0 as * mut crate::src::libpng::png::png_info_def); unsafe fn laertes_init_write_info_ptr() {
write_info_ptr = 0 as *const png_info as *mut png_info;}//;
/*
 * Internal debugging tool.
 */
/* strong check, no #ifdef's */
/*
 * Size ratio display.
 */
unsafe extern "C" fn opng_print_fsize_ratio(mut num: std::os::raw::c_ulong,
                                            mut denom: std::os::raw::c_ulong) {
    let mut buffer: [i8; 32] = [0; 32];
    let mut ratio: crate::src::optipng::optim::opng_ulratio = opng_ulratio{num: 0, denom: 0,};
    let mut result: i32 = 0;
    ratio.num = num;
    ratio.denom = denom;
    result =
        opng_ulratio_to_factor_string(buffer.as_mut_ptr(),
                                      ::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                          as std::os::raw::c_ulong, Some(&mut ratio));
    usr_printf.expect("non-null function pointer")(b"%s%s\x00" as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   buffer.as_mut_ptr(),
                                                   if result >
                                                          0 as std::os::raw::c_int {
                                                       b"\x00" as *const u8 as
                                                           *const std::os::raw::c_char
                                                   } else {
                                                       b"...\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char
                                                   });
}
/*
 * Size change display.
 */
unsafe extern "C" fn opng_print_fsize_difference(mut init_size: std::os::raw::c_ulong,
                                                 mut final_size: std::os::raw::c_ulong,
                                                 mut show_ratio:
                                                     std::os::raw::c_int) {
    let mut difference: u64 = 0;
    let mut sign: i32 = 0;
    if init_size <= final_size {
        sign = 0 as std::os::raw::c_int;
        difference = final_size.wrapping_sub(init_size)
    } else {
        sign = 1 as std::os::raw::c_int;
        difference = init_size.wrapping_sub(final_size)
    }
    if difference == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        usr_printf.expect("non-null function pointer")(b"no change\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char);
        return
    }
    if difference == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        usr_printf.expect("non-null function pointer")(b"1 byte\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char);
    } else {
        usr_printf.expect("non-null function pointer")(b"%lu bytes\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char,
                                                       difference);
    }
    if show_ratio != 0 && init_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        usr_printf.expect("non-null function pointer")(b" = \x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char);
        opng_print_fsize_ratio(difference, init_size);
    }
    usr_printf.expect("non-null function pointer")(if sign == 0 as std::os::raw::c_int
                                                      {
                                                       b" increase\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char
                                                   } else {
                                                       b" decrease\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char
                                                   });
}
/*
 * Image info display.
 */
unsafe extern "C" fn opng_print_image_info(mut show_dim: std::os::raw::c_int,
                                           mut show_depth: std::os::raw::c_int,
                                           mut show_type: std::os::raw::c_int,
                                           mut show_interlaced: std::os::raw::c_int) {
    static mut type_channels: [std::os::raw::c_int; 8] =
        [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_type_channels() {
type_channels = [1 as std::os::raw::c_int, 0 as std::os::raw::c_int, 3 as std::os::raw::c_int,
         1 as std::os::raw::c_int, 2 as std::os::raw::c_int, 0 as std::os::raw::c_int,
         4 as std::os::raw::c_int, 0 as std::os::raw::c_int];}//;
    let mut channels: i32 = 0;
    let mut printed: i32 = 0;
    printed = 0 as std::os::raw::c_int;
    if show_dim != 0 {
        printed = 1 as std::os::raw::c_int;
        usr_printf.expect("non-null function pointer")(b"%lux%lu pixels\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char,
                                                       image.width as
                                                           std::os::raw::c_ulong,
                                                       image.height as
                                                           std::os::raw::c_ulong);
    }
    if show_depth != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(b", \x00" as
                                                               *const u8 as
                                                               *const std::os::raw::c_char);
        }
        printed = 1 as std::os::raw::c_int;
        channels =
            type_channels[(image.color_type & 7 as std::os::raw::c_int) as usize];
        if channels != 1 as std::os::raw::c_int {
            usr_printf.expect("non-null function pointer")(b"%dx%d bits/pixel\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           channels,
                                                           image.bit_depth);
        } else if image.bit_depth != 1 as std::os::raw::c_int {
            usr_printf.expect("non-null function pointer")(b"%d bits/pixel\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           image.bit_depth);
        } else {
            usr_printf.expect("non-null function pointer")(b"1 bit/pixel\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
        }
    }
    if show_type != 0 {
        if printed != 0 {
            usr_printf.expect("non-null function pointer")(b", \x00" as
                                                               *const u8 as
                                                               *const std::os::raw::c_char);
        }
        printed = 1 as std::os::raw::c_int;
        if image.color_type & 1 as std::os::raw::c_int != 0 {
            if image.num_palette == 1 as std::os::raw::c_int {
                usr_printf.expect("non-null function pointer")(b"1 color\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            } else {
                usr_printf.expect("non-null function pointer")(b"%d colors\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               image.num_palette);
            }
            if image.num_trans > 0 as std::os::raw::c_int {
                usr_printf.expect("non-null function pointer")(b" (%d transparent)\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               image.num_trans);
            }
            usr_printf.expect("non-null function pointer")(b" in palette\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
        } else {
            usr_printf.expect("non-null function pointer")(if image.color_type
                                                                  &
                                                                  2 as
                                                                      std::os::raw::c_int
                                                                  != 0 {
                                                               b"RGB\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char
                                                           } else {
                                                               b"grayscale\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char
                                                           });
            if image.color_type & 4 as std::os::raw::c_int != 0 {
                usr_printf.expect("non-null function pointer")(b"+alpha\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            } else if !borrow(& image.trans_color_ptr).is_none() {
                usr_printf.expect("non-null function pointer")(b"+transparency\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            }
        }
    }
    if show_interlaced != 0 {
        if image.interlace_type != 0 as std::os::raw::c_int {
            if printed != 0 {
                usr_printf.expect("non-null function pointer")(b", \x00" as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            }
            usr_printf.expect("non-null function pointer")(b"interlaced\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
        }
    };
}
/*
 * Warning display.
 */
unsafe extern "C" fn opng_print_warning(mut msg: * const std::os::raw::c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as
                                                            i32); /* VT: new paragraph */
    usr_printf.expect("non-null function pointer")(b"Warning: %s\n\x00" as
                                                       *const u8 as
                                                       *const std::os::raw::c_char,
                                                   msg);
}
/*
 * Error display.
 */
unsafe extern "C" fn opng_print_error(mut msg: * const std::os::raw::c_char) {
    usr_print_cntrl.expect("non-null function pointer")('\u{b}' as
                                                            i32); /* VT: new paragraph */
    usr_printf.expect("non-null function pointer")(b"Error: %s\n\x00" as
                                                       *const u8 as
                                                       *const std::os::raw::c_char,
                                                   msg);
}
/*
 * Warning handler.
 */
unsafe extern "C" fn opng_warning(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                  mut msg: * const std::os::raw::c_char) {
    /* Error in input or output file; processing may continue. */
    /* Recovery requires (re)compression of IDAT. */
    if png_ptr == read_ptr {
        process.status |=
            (INPUT_HAS_ERRORS as std::os::raw::c_int |
                 OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int) as std::os::raw::c_uint
    }
    opng_print_warning(msg);
}
/*
 * Error handler.
 */
unsafe extern "C" fn opng_error(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                mut msg: * const std::os::raw::c_char) {
    /* Error in input or output file; processing must stop. */
    /* Recovery requires (re)compression of IDAT. */
    if png_ptr == read_ptr {
        process.status |=
            (INPUT_HAS_ERRORS as std::os::raw::c_int |
                 OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int) as std::os::raw::c_uint
    }
    loop  {
        let ref mut fresh0 = (*the_exception_context.as_mut_ptr()).v.etmp;
        ::std::ptr::write_volatile(fresh0, msg);
        longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                1 as std::os::raw::c_int);
    };
}
/*
 * Memory deallocator.
 */
unsafe extern "C" fn opng_free(mut ptr: * mut core::ffi::c_void) {
    /* This deallocator must be compatible with libpng's memory allocation
     * routines, png_malloc() and png_free().
     * If those routines change, this one must be changed accordingly.
     */
    free(ptr);
}
/*
 * IDAT size checker.
 */
unsafe extern "C" fn opng_check_idat_size(mut size: std::os::raw::c_ulong) {
    if size > idat_size_max {
        loop  {
            let ref mut fresh1 = (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh1,
                                       b"IDAT sizes larger than the maximum chunk size are currently unsupported\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    };
}
/*
 * Chunk handler.
 */
unsafe extern "C" fn opng_set_keep_unknown_chunk(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                 mut keep: std::os::raw::c_int,
                                                 mut chunk_type: * mut std::os::raw::c_uchar) {
    let mut chunk_name: [u8; 5] = [0; 5];
    /* Call png_set_keep_unknown_chunks() once per each chunk type only. */
    memcpy(chunk_name.as_mut_ptr() as *mut std::os::raw::c_void,
           chunk_type as *const std::os::raw::c_void,
           4 as std::os::raw::c_int as std::os::raw::c_ulong);
    chunk_name[4 as std::os::raw::c_int as usize] = 0 as std::os::raw::c_int as png_byte;
    if png_handle_as_unknown(png_ptr as *const png_struct,
                             chunk_name.as_mut_ptr() as png_const_bytep) == 0
       {
        png_set_keep_unknown_chunks(png_ptr, keep,
                                    chunk_name.as_mut_ptr() as
                                        png_const_bytep, 1 as std::os::raw::c_int);
    };
}
/*
 * Chunk categorization.
 */
unsafe extern "C" fn opng_is_image_chunk(mut chunk_type: * mut std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    if *chunk_type.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int &
           0x20 as std::os::raw::c_int == 0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int
    }
    /* Although tRNS is listed as ancillary in the PNG specification, it stores
     * alpha samples, which is critical information. For example, tRNS cannot
     * be generally ignored when rendering animations.
     * Operations claimed to be lossless must treat tRNS as a critical chunk.
     */
    if memcmp(chunk_type as *const std::os::raw::c_void,
              sig_tRNS.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/*
 * Chunk categorization.
 */
unsafe extern "C" fn opng_is_apng_chunk(mut chunk_type: * mut std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    if memcmp(chunk_type as *const std::os::raw::c_void,
              sig_acTL.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
           memcmp(chunk_type as *const std::os::raw::c_void,
                  sig_fcTL.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
           memcmp(chunk_type as *const std::os::raw::c_void,
                  sig_fdAT.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
        return 1 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
}
/*
 * Chunk filter.
 */
unsafe extern "C" fn opng_allow_chunk(mut chunk_type: * mut std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    /* Always allow critical chunks and tRNS. */
    if opng_is_image_chunk(chunk_type) != 0 { return 1 as std::os::raw::c_int }
    /* Block all the other chunks if requested. */
    if options.strip_all != 0 { return 0 as std::os::raw::c_int }
    /* Always block the digital signature chunks. */
    if memcmp(chunk_type as *const std::os::raw::c_void,
              sig_dSIG.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    }
    /* Block the APNG chunks when snipping. */
    if options.snip != 0 && opng_is_apng_chunk(chunk_type) != 0 {
        return 0 as std::os::raw::c_int
    }
    /* Allow all the other chunks. */
    return 1 as std::os::raw::c_int;
}
/*
 * Chunk handler.
 */
unsafe extern "C" fn opng_handle_chunk(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut chunk_type: * mut std::os::raw::c_uchar) {
    let mut keep: i32 = 0;
    if opng_is_image_chunk(chunk_type) != 0 { return }
    if options.strip_all != 0 {
        process.status |=
            (INPUT_HAS_STRIPPED_DATA as std::os::raw::c_int |
                 INPUT_HAS_JUNK as std::os::raw::c_int) as std::os::raw::c_uint;
        opng_set_keep_unknown_chunk(png_ptr, 1 as std::os::raw::c_int, chunk_type);
        return
    }
    /* Let libpng handle bKGD, hIST and sBIT. */
    if memcmp(chunk_type as *const std::os::raw::c_void,
              sig_bKGD.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
           memcmp(chunk_type as *const std::os::raw::c_void,
                  sig_hIST.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
           memcmp(chunk_type as *const std::os::raw::c_void,
                  sig_sBIT.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
        return
    }
    /* Everything else is handled as unknown by libpng. */
    keep = 3 as std::os::raw::c_int;
    if memcmp(chunk_type as *const std::os::raw::c_void,
              sig_dSIG.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
        /* Recognize dSIG, but let libpng handle it as unknown. */
        process.status |=
            INPUT_HAS_DIGITAL_SIGNATURE as std::os::raw::c_int as std::os::raw::c_uint
    } else if opng_is_apng_chunk(chunk_type) != 0 {
        /* Recognize APNG, but let libpng handle it as unknown. */
        process.status |= INPUT_HAS_APNG as std::os::raw::c_int as std::os::raw::c_uint;
        if memcmp(chunk_type as *const std::os::raw::c_void,
                  sig_fdAT.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            process.status |=
                INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as std::os::raw::c_uint
        }
        if options.snip != 0 {
            process.status |= INPUT_HAS_JUNK as std::os::raw::c_int as std::os::raw::c_uint;
            keep = 1 as std::os::raw::c_int
        }
    }
    opng_set_keep_unknown_chunk(png_ptr, keep, chunk_type);
}
/*
 * Initialization for input handler.
 */
 extern "C" fn opng_init_read_data() {
    /* The relevant process data members are set to zero,
     * and nothing else needs to be done at this moment.
     */
}
/*
 * Initialization for output handler.
 */
unsafe extern "C" fn opng_init_write_data() {
    process.out_file_size = 0 as std::os::raw::c_int as opng_fsize_t;
    process.out_plte_trns_size = 0 as std::os::raw::c_int as png_uint_32;
    process.out_idat_size = 0 as std::os::raw::c_int as opng_fsize_t;
}
/*
 * Input handler.
 */
unsafe extern "C" fn opng_read_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                    mut data: * mut std::os::raw::c_uchar, mut length: std::os::raw::c_ulong) {
    let mut stream: * mut crate::src::gifread::gifread::_IO_FILE =
        png_get_io_ptr(png_ptr as *const png_struct) as *mut FILE;
    let mut io_state: i32 =
        png_get_io_state(png_ptr as *const png_struct) as std::os::raw::c_int;
    let mut io_state_loc: i32 = io_state & 0xf0 as std::os::raw::c_int;
    let mut chunk_sig: * mut u8 = 0 as *mut png_byte;
    /* Read the data. */
    if fread(data as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
             length, stream) != length {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t read the input file or unexpected end of file\x00"
                      as *const u8 as *const std::os::raw::c_char);
    }
    if process.in_file_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        /* first piece of PNG data */
        if !(length == 8 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"PNG I/O must start with the first 8 bytes\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
        process.in_datastream_offset =
            opng_ftello(stream) - 8 as std::os::raw::c_int as std::os::raw::c_long;
        process.status |=
            INPUT_HAS_PNG_DATASTREAM as std::os::raw::c_int as std::os::raw::c_uint;
        if io_state_loc == 0x10 as std::os::raw::c_int {
            process.status |=
                INPUT_HAS_PNG_SIGNATURE as std::os::raw::c_int as std::os::raw::c_uint
        }
        if process.in_datastream_offset == 0 as std::os::raw::c_int as std::os::raw::c_long {
            process.status |= INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint
        } else if process.in_datastream_offset <
                      0 as std::os::raw::c_int as std::os::raw::c_long {
            png_error(png_ptr as *const png_struct,
                      b"Can\'t get the file-position indicator in input file\x00"
                          as *const u8 as *const std::os::raw::c_char);
        }
        process.in_file_size = process.in_datastream_offset as opng_fsize_t
    }
    process.in_file_size =
        (process.in_file_size as std::os::raw::c_ulong).wrapping_add(length) as
            opng_fsize_t as opng_fsize_t;
    /* Handle the OptiPNG-specific events. */
    if !(io_state & 0x1 as std::os::raw::c_int != 0 &&
             io_state_loc != 0 as std::os::raw::c_int) {
        usr_panic.expect("non-null function pointer")(b"Incorrect info in png_ptr->io_state\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    if io_state_loc == 0x20 as std::os::raw::c_int {
        /* In libpng 1.4.x and later, the chunk length and the chunk name
         * are serialized in a single operation. This is also ensured by
         * the opngio add-on for libpng 1.2.x and earlier.
         */
        if !(length == 8 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"Reading chunk header, expecting 8 bytes\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
        chunk_sig = data.offset(4 as std::os::raw::c_int as isize);
        if memcmp(chunk_sig as *const std::os::raw::c_void,
                  sig_IDAT.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            if !(png_ptr == read_ptr) {
                usr_panic.expect("non-null function pointer")(b"Incorrect I/O handler setup\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const std::os::raw::c_char);
            }
            if png_get_rows(read_ptr as *const png_struct,
                            read_info_ptr as *const png_info).is_null() {
                /* 1st IDAT */
                if !(process.in_idat_size ==
                         0 as std::os::raw::c_int as std::os::raw::c_ulong) {
                    usr_panic.expect("non-null function pointer")(b"Found IDAT with no rows\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const std::os::raw::c_char);
                }
                /* Allocate the rows here, bypassing libpng.
                 * This allows to initialize the contents and perform recovery
                 * in case of a premature EOF.
                 */
                if png_get_image_height(read_ptr as *const png_struct,
                                        read_info_ptr as *const png_info) ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    return
                } /* premature IDAT; an error will occur later */
                if pngx_malloc_rows(read_ptr, read_info_ptr,
                                    0 as std::os::raw::c_int).is_null() {
                    usr_panic.expect("non-null function pointer")(b"Failed allocation of image rows; unsafe libpng allocator\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const std::os::raw::c_char);
                }
                png_data_freer(read_ptr as *const png_struct, read_info_ptr,
                               2 as std::os::raw::c_int, 0x40 as std::os::raw::c_uint);
            } else {
                /* There is split IDAT overhead. Join IDATs. */
                process.status |=
                    INPUT_HAS_JUNK as std::os::raw::c_int as std::os::raw::c_uint
            }
            process.in_idat_size =
                (process.in_idat_size as
                     std::os::raw::c_ulong).wrapping_add(((*data as png_uint_32) <<
                                                      24 as
                                                          std::os::raw::c_int).wrapping_add((*data.offset(1
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      isize)
                                                                                         as
                                                                                         png_uint_32)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int).wrapping_add((*data.offset(2
                                                                                                                                        as
                                                                                                                                        std::os::raw::c_int
                                                                                                                                        as
                                                                                                                                        isize)
                                                                                                                           as
                                                                                                                           png_uint_32)
                                                                                                                          <<
                                                                                                                          8
                                                                                                                              as
                                                                                                                              std::os::raw::c_int).wrapping_add(*data.offset(3
                                                                                                                                                                         as
                                                                                                                                                                         std::os::raw::c_int
                                                                                                                                                                         as
                                                                                                                                                                         isize)
                                                                                                                                                            as
                                                                                                                                                            png_uint_32)
                                                     as std::os::raw::c_ulong) as
                    opng_fsize_t as opng_fsize_t
        } else if memcmp(chunk_sig as *const std::os::raw::c_void,
                         sig_PLTE.as_ptr() as *const std::os::raw::c_void,
                         4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int ||
                      memcmp(chunk_sig as *const std::os::raw::c_void,
                             sig_tRNS.as_ptr() as *const std::os::raw::c_void,
                             4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                          0 as std::os::raw::c_int {
            /* Add the chunk overhead (header + CRC) to the data size. */
            process.in_plte_trns_size =
                (process.in_plte_trns_size as
                     std::os::raw::c_uint).wrapping_add(((*data as png_uint_32) <<
                                                     24 as
                                                         std::os::raw::c_int).wrapping_add((*data.offset(1
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                        as
                                                                                        png_uint_32)
                                                                                       <<
                                                                                       16
                                                                                           as
                                                                                           std::os::raw::c_int).wrapping_add((*data.offset(2
                                                                                                                                       as
                                                                                                                                       std::os::raw::c_int
                                                                                                                                       as
                                                                                                                                       isize)
                                                                                                                          as
                                                                                                                          png_uint_32)
                                                                                                                         <<
                                                                                                                         8
                                                                                                                             as
                                                                                                                             std::os::raw::c_int).wrapping_add(*data.offset(3
                                                                                                                                                                        as
                                                                                                                                                                        std::os::raw::c_int
                                                                                                                                                                        as
                                                                                                                                                                        isize)
                                                                                                                                                           as
                                                                                                                                                           png_uint_32).wrapping_add(12
                                                                                                                                                                                         as
                                                                                                                                                                                         std::os::raw::c_int
                                                                                                                                                                                         as
                                                                                                                                                                                         std::os::raw::c_uint))
                    as png_uint_32 as png_uint_32
        } else { opng_handle_chunk(png_ptr, chunk_sig); }
    } else if io_state_loc == 0x80 as std::os::raw::c_int {
        if !(length == 4 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"Reading chunk CRC, expecting 4 bytes\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
    };
}
/*
 * Output handler.
 */
unsafe extern "C" fn opng_write_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                     mut data: * mut std::os::raw::c_uchar,
                                     mut length: std::os::raw::c_ulong) {
    static mut allow_crt_chunk: std::os::raw::c_int = 0; unsafe fn laertes_init_allow_crt_chunk() {
allow_crt_chunk = 0;}//;
    static mut crt_chunk_is_idat: std::os::raw::c_int = 0; unsafe fn laertes_init_crt_chunk_is_idat() {
crt_chunk_is_idat = 0;}//;
    static mut crt_idat_offset: std::os::raw::c_long = 0; unsafe fn laertes_init_crt_idat_offset() {
crt_idat_offset = 0;}//;
    static mut crt_idat_size: std::os::raw::c_ulong = 0; unsafe fn laertes_init_crt_idat_size() {
crt_idat_size = 0;}//;
    static mut crt_idat_crc: std::os::raw::c_uint = 0; unsafe fn laertes_init_crt_idat_crc() {
crt_idat_crc = 0;}//;
    let mut stream: * mut crate::src::gifread::gifread::_IO_FILE =
        png_get_io_ptr(png_ptr as *const png_struct) as *mut FILE;
    let mut io_state: i32 =
        png_get_io_state(png_ptr as *const png_struct) as std::os::raw::c_int;
    let mut io_state_loc: i32 = io_state & 0xf0 as std::os::raw::c_int;
    let mut chunk_sig: * mut u8 = 0 as *mut png_byte;
    let mut buf: [u8; 4] = [0; 4];
    if !(io_state & 0x2 as std::os::raw::c_int != 0 &&
             io_state_loc != 0 as std::os::raw::c_int) {
        usr_panic.expect("non-null function pointer")(b"Incorrect info in png_ptr->io_state\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    /* Handle the OptiPNG-specific events. */
    if io_state_loc == 0x20 as std::os::raw::c_int {
        if !(length == 8 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"Writing chunk header, expecting 8 bytes\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
        chunk_sig = data.offset(4 as std::os::raw::c_int as isize);
        allow_crt_chunk = opng_allow_chunk(chunk_sig);
        if memcmp(chunk_sig as *const std::os::raw::c_void,
                  sig_IDAT.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            crt_chunk_is_idat = 1 as std::os::raw::c_int;
            process.out_idat_size =
                (process.out_idat_size as
                     std::os::raw::c_ulong).wrapping_add(((*data as png_uint_32) <<
                                                      24 as
                                                          std::os::raw::c_int).wrapping_add((*data.offset(1
                                                                                                      as
                                                                                                      std::os::raw::c_int
                                                                                                      as
                                                                                                      isize)
                                                                                         as
                                                                                         png_uint_32)
                                                                                        <<
                                                                                        16
                                                                                            as
                                                                                            std::os::raw::c_int).wrapping_add((*data.offset(2
                                                                                                                                        as
                                                                                                                                        std::os::raw::c_int
                                                                                                                                        as
                                                                                                                                        isize)
                                                                                                                           as
                                                                                                                           png_uint_32)
                                                                                                                          <<
                                                                                                                          8
                                                                                                                              as
                                                                                                                              std::os::raw::c_int).wrapping_add(*data.offset(3
                                                                                                                                                                         as
                                                                                                                                                                         std::os::raw::c_int
                                                                                                                                                                         as
                                                                                                                                                                         isize)
                                                                                                                                                            as
                                                                                                                                                            png_uint_32)
                                                     as std::os::raw::c_ulong) as
                    opng_fsize_t as opng_fsize_t;
            /* Abandon the trial if IDAT is bigger than the maximum allowed. */
            if stream.is_null() {
                if process.out_idat_size > process.max_idat_size {
                    loop  {
                        let ref mut fresh2 =
                            (*the_exception_context.as_mut_ptr()).v.etmp;
                        ::std::ptr::write_volatile(fresh2,
                                                   0 as *const std::os::raw::c_char);
                        longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                1 as std::os::raw::c_int);
                    }
                }
                /* early interruption, not an error */
            }
        } else {
            /* not IDAT */
            crt_chunk_is_idat = 0 as std::os::raw::c_int;
            if memcmp(chunk_sig as *const std::os::raw::c_void,
                      sig_PLTE.as_ptr() as *const std::os::raw::c_void,
                      4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
                   ||
                   memcmp(chunk_sig as *const std::os::raw::c_void,
                          sig_tRNS.as_ptr() as *const std::os::raw::c_void,
                          4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                       0 as std::os::raw::c_int {
                /* Add the chunk overhead (header + CRC) to the data size. */
                process.out_plte_trns_size =
                    (process.out_plte_trns_size as
                         std::os::raw::c_uint).wrapping_add(((*data as png_uint_32) <<
                                                         24 as
                                                             std::os::raw::c_int).wrapping_add((*data.offset(1
                                                                                                         as
                                                                                                         std::os::raw::c_int
                                                                                                         as
                                                                                                         isize)
                                                                                            as
                                                                                            png_uint_32)
                                                                                           <<
                                                                                           16
                                                                                               as
                                                                                               std::os::raw::c_int).wrapping_add((*data.offset(2
                                                                                                                                           as
                                                                                                                                           std::os::raw::c_int
                                                                                                                                           as
                                                                                                                                           isize)
                                                                                                                              as
                                                                                                                              png_uint_32)
                                                                                                                             <<
                                                                                                                             8
                                                                                                                                 as
                                                                                                                                 std::os::raw::c_int).wrapping_add(*data.offset(3
                                                                                                                                                                            as
                                                                                                                                                                            std::os::raw::c_int
                                                                                                                                                                            as
                                                                                                                                                                            isize)
                                                                                                                                                               as
                                                                                                                                                               png_uint_32).wrapping_add(12
                                                                                                                                                                                             as
                                                                                                                                                                                             std::os::raw::c_int
                                                                                                                                                                                             as
                                                                                                                                                                                             std::os::raw::c_uint))
                        as png_uint_32 as png_uint_32
            }
        }
    } else if io_state_loc == 0x80 as std::os::raw::c_int {
        if !(length == 4 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"Writing chunk CRC, expecting 4 bytes\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
    }
    /* Exit early if this is only a trial. */
    if stream.is_null() { return }
    /* Continue only if the current chunk type is allowed. */
    if io_state_loc != 0x10 as std::os::raw::c_int && allow_crt_chunk == 0 { return }
    /* Here comes an elaborate way of writing the data, in which all IDATs
     * are joined into a single chunk.
     * Normally, the user-supplied I/O routines are not so complicated.
     */
    match io_state_loc {
        32 => {
            if crt_chunk_is_idat != 0 {
                if crt_idat_offset == 0 as std::os::raw::c_int as std::os::raw::c_long {
                    /* This is the header of the first IDAT. */
                    crt_idat_offset = opng_ftello(stream);
                    /* Try guessing the size of the final (joined) IDAT. */
                    if process.best_idat_size >
                           0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        /* The guess is expected to be right. */
                        crt_idat_size = process.best_idat_size
                    } else {
                        /* The guess could be wrong.
                     * The size of the final IDAT will be revised.
                     */
                        crt_idat_size = length
                    }
                    png_save_uint_32(data, crt_idat_size as png_uint_32);
                    /* Start computing the CRC of the final IDAT. */
                    crt_idat_crc =
                        crc32(0 as std::os::raw::c_int as uLong, sig_IDAT.as_ptr(),
                              4 as std::os::raw::c_int as uInt) as png_uint_32
                } else {
                    /* This is not the first IDAT. Do not write its header. */
                    return
                }
            } else if crt_idat_offset != 0 as std::os::raw::c_int as std::os::raw::c_long {
                /* This is the header of the first chunk after IDAT.
                 * Finalize IDAT before resuming the normal operation.
                 */
                png_save_uint_32(buf.as_mut_ptr(), crt_idat_crc); /* error */
                if fwrite(buf.as_mut_ptr() as *const std::os::raw::c_void,
                          1 as std::os::raw::c_int as std::os::raw::c_ulong,
                          4 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                       4 as std::os::raw::c_int as std::os::raw::c_ulong {
                    io_state = 0 as std::os::raw::c_int
                }
                process.out_file_size =
                    (process.out_file_size as
                         std::os::raw::c_ulong).wrapping_add(4 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        opng_fsize_t as opng_fsize_t;
                if process.out_idat_size != crt_idat_size {
                    /* The IDAT size has not been guessed correctly.
                     * It must be updated in a non-streamable way.
                     */
                    if !(process.best_idat_size ==
                             0 as std::os::raw::c_int as std::os::raw::c_ulong) {
                        usr_panic.expect("non-null function pointer")(b"Wrong guess of the output IDAT size\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const std::os::raw::c_char);
                    }
                    opng_check_idat_size(process.out_idat_size);
                    png_save_uint_32(buf.as_mut_ptr(),
                                     process.out_idat_size as png_uint_32);
                    if opng_fwriteo(stream, crt_idat_offset, 0 as std::os::raw::c_int,
                                    buf.as_mut_ptr() as *const std::os::raw::c_void,
                                    4 as std::os::raw::c_int as size_t) !=
                           4 as std::os::raw::c_int as std::os::raw::c_ulong {
                        io_state = 0 as std::os::raw::c_int
                    }
                    /* error */
                }
                if io_state == 0 as std::os::raw::c_int {
                    png_error(png_ptr as *const png_struct,
                              b"Can\'t finalize IDAT\x00" as *const u8 as
                                  *const std::os::raw::c_char);
                }
                crt_idat_offset = 0 as std::os::raw::c_int as opng_foffset_t
            }
        }
        64 => {
            if crt_chunk_is_idat != 0 {
                crt_idat_crc =
                    crc32(crt_idat_crc as uLong, data as *const Bytef,
                          length as uInt) as png_uint_32
            }
        }
        128 => {
            if crt_chunk_is_idat != 0 {
                /* Defer writing until the first non-IDAT occurs. */
                return
            }
        }
        _ => { }
    }
    /* Write the data. */
    if fwrite(data as *const std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
              length, stream) != length {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t write the output file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    process.out_file_size =
        (process.out_file_size as std::os::raw::c_ulong).wrapping_add(length) as
            opng_fsize_t as opng_fsize_t;
}
/*
 * Image info initialization.
 */
unsafe extern "C" fn opng_clear_image_info() {
    memset(&mut image as *mut opng_image_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_image_struct>() as std::os::raw::c_ulong);
}
/*
 * Image info transfer.
 */
unsafe extern "C" fn opng_load_image_info(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                          mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                          mut load_meta: std::os::raw::c_int) {
    memset(&mut image as *mut opng_image_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_image_struct>() as std::os::raw::c_ulong);
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 Some(&mut image.width), Some(&mut image.height), Some(&mut image.bit_depth),
                 Some(&mut image.color_type), Some(&mut image.interlace_type),
                 Some(&mut image.compression_type), Some(&mut image.filter_type));
    image.row_pointers =
        png_get_rows(png_ptr as *const png_struct,
                     info_ptr as *const png_info);
    png_get_PLTE(png_ptr as *const png_struct, info_ptr, Some(&mut image.palette),
                 Some(&mut image.num_palette));
    /* Transparency is not considered metadata, although tRNS is ancillary.
     * See the comment in opng_is_image_chunk() above.
     */
    if png_get_tRNS(png_ptr as *const png_struct, info_ptr,
                    Some(&mut image.trans_alpha), Some(&mut image.num_trans),
                    Some(&mut image.trans_color_ptr)) != 0 {
        /* Double copying (pointer + value) is necessary here
         * due to an inconsistency in the libpng design.
         */
        if !borrow(& image.trans_color_ptr).is_none() {
            image.trans_color = *(borrow_mut(&mut image.trans_color_ptr)).unwrap();
            image.trans_color_ptr = Some(&mut image.trans_color)
        }
    }
    if load_meta == 0 { return }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr,
                    Some(&mut image.background_ptr)) != 0 {
        /* Same problem as in tRNS. */
        image.background = *(borrow_mut(&mut image.background_ptr)).unwrap();
        image.background_ptr = Some(&mut image.background)
    }
    png_get_hIST(png_ptr as *const png_struct, info_ptr, Some(&mut image.hist));
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr,
                    Some(&mut image.sig_bit_ptr)) != 0 {
        /* Same problem as in tRNS. */
        image.sig_bit = *(borrow_mut(&mut image.sig_bit_ptr)).unwrap();
        image.sig_bit_ptr = Some(&mut image.sig_bit)
    }
    image.num_unknowns =
        png_get_unknown_chunks(png_ptr as *const png_struct, info_ptr,
                               Some(&mut image.unknowns));
}
/*
 * Image info transfer.
 */
unsafe extern "C" fn opng_store_image_info(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                           mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                           mut store_meta: std::os::raw::c_int) {
    let mut i: i32 = 0;
    if image.row_pointers.is_null() {
        usr_panic.expect("non-null function pointer")(b"No info in image\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, image.width,
                 image.height, image.bit_depth, image.color_type,
                 image.interlace_type, image.compression_type,
                 image.filter_type);
    png_set_rows(write_ptr as *const png_struct, write_info_ptr,
                 image.row_pointers);
    if !image.palette.is_null() {
        png_set_PLTE(png_ptr, info_ptr, image.palette as png_const_colorp,
                     image.num_palette);
    }
    /* Transparency is not considered metadata, although tRNS is ancillary.
     * See the comment in opng_is_image_chunk() above.
     */
    if !image.trans_alpha.is_null() || !borrow(& image.trans_color_ptr).is_none() {
        png_set_tRNS(png_ptr, info_ptr, image.trans_alpha as png_const_bytep,
                     image.num_trans,
                     borrow(& image.trans_color_ptr));
    }
    if store_meta == 0 { return }
    if !borrow(& image.background_ptr).is_none() {
        png_set_bKGD(png_ptr as *const png_struct, info_ptr,
                     borrow(& image.background_ptr));
    }
    if !image.hist.is_null() {
        png_set_hIST(png_ptr as *const png_struct, info_ptr,
                     image.hist as png_const_uint_16p);
    }
    if !borrow(& image.sig_bit_ptr).is_none() {
        png_set_sBIT(png_ptr as *const png_struct, info_ptr,
                     borrow(& image.sig_bit_ptr));
    }
    if image.num_unknowns != 0 as std::os::raw::c_int {
        png_set_unknown_chunks(png_ptr as *const png_struct, info_ptr,
                               image.unknowns as png_const_unknown_chunkp,
                               image.num_unknowns);
        /* This should be handled by libpng. */
        i = 0 as std::os::raw::c_int;
        while i < image.num_unknowns {
            png_set_unknown_chunk_location(png_ptr as *const png_struct,
                                           info_ptr, i,
                                           (*image.unknowns.offset(i as
                                                                       isize)).location
                                               as std::os::raw::c_int);
            i += 1
        }
    };
}
/*
 * Image info destruction.
 */
unsafe extern "C" fn opng_destroy_image_info() {
    let mut i: u32 = 0; /* nothing to clean up */
    let mut j: i32 = 0;
    if image.row_pointers.is_null() { return }
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < image.height {
        opng_free(*image.row_pointers.offset(i as isize) as
                      *mut std::os::raw::c_void);
        i = i.wrapping_add(1)
    }
    opng_free(image.row_pointers as *mut std::os::raw::c_void);
    opng_free(image.palette as *mut std::os::raw::c_void);
    opng_free(image.trans_alpha as *mut std::os::raw::c_void);
    opng_free(image.hist as *mut std::os::raw::c_void);
    j = 0 as std::os::raw::c_int;
    while j < image.num_unknowns {
        opng_free((*image.unknowns.offset(j as isize)).data as
                      *mut std::os::raw::c_void);
        j += 1
    }
    opng_free(image.unknowns as *mut std::os::raw::c_void);
    /* DO NOT deallocate background_ptr, sig_bit_ptr, trans_color_ptr.
     * See the comments regarding double copying inside opng_load_image_info().
     */
    /* Clear the space here and do not worry about double-deallocation issues
     * that might arise later on.
     */
    memset(&mut image as *mut opng_image_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_image_struct>() as std::os::raw::c_ulong);
}
/*
 * Image file reading.
 */
unsafe extern "C" fn opng_read_file(mut infile: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut fmt_name: * const i8 =
        0 as *const std::os::raw::c_char; /* volatile is required by cexcept */
    let mut num_img: i32 = 0;
    let mut reductions: u32 = 0;
    let mut err_msg: * const i8 = 0 as *const std::os::raw::c_char;
    let mut exception__prev: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh3 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh3 = &mut exception__env;
    if _setjmp(exception__env.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            read_ptr =
                png_create_read_struct(b"1.6.34\x00" as *const u8 as
                                           *const std::os::raw::c_char,
                                       0 as *mut std::os::raw::c_void,
                                       Some(opng_error),
                                       Some(opng_warning));
            read_info_ptr =
                png_create_info_struct(read_ptr as *const png_struct);
            if read_info_ptr.is_null() {
                loop  {
                    let ref mut fresh4 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh4,
                                               b"Out of memory\x00" as
                                                   *const u8 as
                                                   *const std::os::raw::c_char);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            /* everything is ok */
            png_set_keep_unknown_chunks(read_ptr, 3 as std::os::raw::c_int,
                                        0 as png_const_bytep,
                                        0 as std::os::raw::c_int);
            png_set_user_limits(read_ptr,
                                0x7fffffff as std::os::raw::c_long as png_uint_32,
                                0x7fffffff as std::os::raw::c_long as png_uint_32);
            opng_init_read_data();
            png_set_read_fn(read_ptr, infile as png_voidp,
                            Some(opng_read_data));
            fmt_name = 0 as *const std::os::raw::c_char;
            num_img =
                pngx_read_image(read_ptr, read_info_ptr, Some(&mut fmt_name),
                                Option::<&'_ mut * const i8>::None);
            if num_img <= 0 as std::os::raw::c_int {
                loop  {
                    let ref mut fresh5 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh5,
                                               b"Unrecognized image file format\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            if num_img > 1 as std::os::raw::c_int {
                process.status |=
                    INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as std::os::raw::c_uint
            }
            if process.status &
                   INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
                   process.status &
                       INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as
                           std::os::raw::c_uint != 0 {
                /* Override the default libpng settings. */
                /* Read the input image file. */
                /* pngxtern can't distinguish between APNG and proper PNG. */
                fmt_name =
                    if process.status &
                           INPUT_HAS_PNG_SIGNATURE as std::os::raw::c_int as
                               std::os::raw::c_uint != 0 {
                        b"APNG\x00" as *const u8 as *const std::os::raw::c_char
                    } else {
                        b"APNG datastream\x00" as *const u8 as
                            *const std::os::raw::c_char
                    }
            }
            if fmt_name.is_null() {
                usr_panic.expect("non-null function pointer")(b"No format name from pngxtern\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const std::os::raw::c_char);
            }
            if process.in_file_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                if opng_fgetsize(infile, Some(&mut process.in_file_size)) <
                       0 as std::os::raw::c_int {
                    opng_print_warning(b"Can\'t get the correct file size\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char);
                    process.in_file_size = 0 as std::os::raw::c_int as opng_fsize_t
                }
            }
            ::std::ptr::write_volatile(&mut err_msg as
                                           *mut *const std::os::raw::c_char,
                                       0 as *const std::os::raw::c_char);
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh6 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh6 = exception__prev;
    if !((*the_exception_context.as_mut_ptr()).caught == 0 ||
             {
                 ::std::ptr::write_volatile(&mut err_msg as
                                                *mut *const std::os::raw::c_char,
                                            (*the_exception_context.as_mut_ptr()).v.etmp);
                 (0 as std::os::raw::c_int) != 0
             }) {
        /* If the critical info has been loaded, treat all errors as warnings.
         * This enables a more advanced data recovery.
         */
        if opng_validate_image(read_ptr, read_info_ptr) != 0 {
            png_warning(read_ptr as *const png_struct, err_msg);
            ::std::ptr::write_volatile(&mut err_msg as
                                           *mut *const std::os::raw::c_char,
                                       0 as *const std::os::raw::c_char)
        }
    }
    let mut exception__prev_0: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env_0: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev_0 as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh7 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh7 = &mut exception__env_0;
    if _setjmp(exception__env_0.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            if !err_msg.is_null() {
                loop  {
                    let ref mut fresh8 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh8, err_msg);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            /* Display format and image information. */
            if strcmp(fmt_name,
                      b"PNG\x00" as *const u8 as *const std::os::raw::c_char) !=
                   0 as std::os::raw::c_int {
                usr_printf.expect("non-null function pointer")(b"Importing %s\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               fmt_name);
                if process.status &
                       INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as
                           std::os::raw::c_uint != 0 {
                    if process.status &
                           INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint ==
                           0 {
                        usr_printf.expect("non-null function pointer")(b" (multi-image or animation)\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
                    }
                    if options.snip != 0 {
                        usr_printf.expect("non-null function pointer")(b"; snipping...\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const std::os::raw::c_char);
                    }
                }
                usr_printf.expect("non-null function pointer")(b"\n\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            }
            opng_load_image_info(read_ptr, read_info_ptr, 1 as std::os::raw::c_int);
            opng_print_image_info(1 as std::os::raw::c_int, 1 as std::os::raw::c_int,
                                  1 as std::os::raw::c_int, 1 as std::os::raw::c_int);
            usr_printf.expect("non-null function pointer")(b"\n\x00" as
                                                               *const u8 as
                                                               *const std::os::raw::c_char);
            /* Choose the applicable image reductions. */
            reductions =
                ((0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int |
                      (0x4 as std::os::raw::c_int | 0x8 as std::os::raw::c_int |
                           0x10 as std::os::raw::c_int | 0x20 as std::os::raw::c_int |
                           0x40 as std::os::raw::c_int | 0x80 as std::os::raw::c_int) |
                      (0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int) |
                      0x1000 as std::os::raw::c_int) & !(0x1000 as std::os::raw::c_int)) as
                    png_uint_32;
            if options.nb != 0 {
                reductions &=
                    !(0x1 as std::os::raw::c_int | 0x2 as std::os::raw::c_int) as std::os::raw::c_uint
            }
            if options.nc != 0 {
                reductions &=
                    !(0x4 as std::os::raw::c_int | 0x8 as std::os::raw::c_int |
                          0x10 as std::os::raw::c_int | 0x20 as std::os::raw::c_int |
                          0x40 as std::os::raw::c_int | 0x80 as std::os::raw::c_int) as
                        std::os::raw::c_uint
            }
            if options.np != 0 {
                reductions &=
                    !(0x100 as std::os::raw::c_int | 0x200 as std::os::raw::c_int) as
                        std::os::raw::c_uint
            }
            if options.nz != 0 &&
                   process.status &
                       INPUT_HAS_PNG_DATASTREAM as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 {
                /* Do not reduce files with PNG datastreams under -nz. */
                reductions = 0 as std::os::raw::c_int as png_uint_32
            }
            if process.status &
                   INPUT_HAS_DIGITAL_SIGNATURE as std::os::raw::c_int as std::os::raw::c_uint
                   != 0 {
                /* Do not reduce signed files. */
                reductions = 0 as std::os::raw::c_int as png_uint_32
            }
            if process.status &
                   INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
                   process.status &
                       INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as
                           std::os::raw::c_uint != 0 &&
                   reductions != 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                   options.snip == 0 {
                usr_printf.expect("non-null function pointer")(b"Can\'t reliably reduce APNG file; disabling reductions.\n(Did you want to -snip and optimize the first frame?)\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
                reductions = 0 as std::os::raw::c_int as png_uint_32
            }
            /* Try to reduce the image. */
            process.reductions =
                opng_reduce_image(read_ptr, read_info_ptr, reductions);
            /* If the image is reduced, enforce full compression. */
            if process.reductions != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                opng_load_image_info(read_ptr, read_info_ptr,
                                     1 as std::os::raw::c_int);
                usr_printf.expect("non-null function pointer")(b"Reducing image to \x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
                opng_print_image_info(0 as std::os::raw::c_int, 1 as std::os::raw::c_int,
                                      1 as std::os::raw::c_int, 0 as std::os::raw::c_int);
                usr_printf.expect("non-null function pointer")(b"\n\x00" as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char);
            }
            /* Change the interlace type if required. */
            if options.interlace >= 0 as std::os::raw::c_int &&
                   image.interlace_type != options.interlace {
                image.interlace_type = options.interlace;
                /* A change in interlacing requires IDAT recoding. */
                process.status |=
                    OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
            }
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh9 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh9 = exception__prev_0;
    if (*the_exception_context.as_mut_ptr()).caught == 0 ||
           {
               ::std::ptr::write_volatile(&mut err_msg as
                                              *mut *const std::os::raw::c_char,
                                          (*the_exception_context.as_mut_ptr()).v.etmp);
               (0 as std::os::raw::c_int) != 0
           } {
    } else {
        /* Do the cleanup, then rethrow the exception. */
        png_data_freer(read_ptr as *const png_struct, read_info_ptr,
                       1 as std::os::raw::c_int, 0xffff as std::os::raw::c_uint);
        png_destroy_read_struct(Some(&mut read_ptr), Some(&mut read_info_ptr),
                                Option::<&'_ mut * mut crate::src::libpng::png::png_info_def>::None);
        loop  {
            let ref mut fresh10 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh10, err_msg);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    /* Destroy the libpng structures, but leave the enclosed data intact
     * to allow further processing.
     */
    png_data_freer(read_ptr as *const png_struct, read_info_ptr,
                   2 as std::os::raw::c_int, 0xffff as std::os::raw::c_uint);
    png_destroy_read_struct(Some(&mut read_ptr), Some(&mut read_info_ptr),
                            Option::<&'_ mut * mut crate::src::libpng::png::png_info_def>::None);
}
/*
 * PNG file writing.
 *
 * If the output file is NULL, PNG encoding is still done,
 * but no file is written.
 */
unsafe extern "C" fn opng_write_file(mut outfile: * mut crate::src::gifread::gifread::_IO_FILE,
                                     mut compression_level: std::os::raw::c_int,
                                     mut memory_level: std::os::raw::c_int,
                                     mut compression_strategy: std::os::raw::c_int,
                                     mut filter: std::os::raw::c_int) {
    let mut err_msg: * const i8 =
        0 as *const std::os::raw::c_char; /* volatile is required by cexcept */
    if !(compression_level >= 1 as std::os::raw::c_int &&
             compression_level <= 9 as std::os::raw::c_int &&
             memory_level >= 1 as std::os::raw::c_int &&
             memory_level <= 9 as std::os::raw::c_int &&
             compression_strategy >= 0 as std::os::raw::c_int &&
             compression_strategy <= 3 as std::os::raw::c_int &&
             filter >= 0 as std::os::raw::c_int && filter <= 5 as std::os::raw::c_int) {
        usr_panic.expect("non-null function pointer")(b"Invalid encoding parameters\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    let mut exception__prev: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh11 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh11 = &mut exception__env;
    if _setjmp(exception__env.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            write_ptr =
                png_create_write_struct(b"1.6.34\x00" as *const u8 as
                                            *const std::os::raw::c_char,
                                        0 as *mut std::os::raw::c_void,
                                        Some(opng_error),
                                        Some(opng_warning));
            write_info_ptr =
                png_create_info_struct(write_ptr as *const png_struct);
            if write_info_ptr.is_null() {
                loop  {
                    let ref mut fresh12 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh12,
                                               b"Out of memory\x00" as
                                                   *const u8 as
                                                   *const std::os::raw::c_char);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            png_set_compression_level(write_ptr, compression_level);
            png_set_compression_mem_level(write_ptr, memory_level);
            png_set_compression_strategy(write_ptr, compression_strategy);
            png_set_filter(write_ptr, 0 as std::os::raw::c_int,
                           filter_table[filter as usize]);
            if compression_strategy != 2 as std::os::raw::c_int &&
                   compression_strategy != 3 as std::os::raw::c_int {
                if options.window_bits > 0 as std::os::raw::c_int {
                    png_set_compression_window_bits(write_ptr,
                                                    options.window_bits);
                }
            } else {
                png_set_compression_window_bits(write_ptr, 9 as std::os::raw::c_int);
            }
            /* everything is ok */
            png_set_keep_unknown_chunks(write_ptr, 3 as std::os::raw::c_int,
                                        0 as png_const_bytep,
                                        0 as std::os::raw::c_int);
            png_set_user_limits(write_ptr,
                                0x7fffffff as std::os::raw::c_long as png_uint_32,
                                0x7fffffff as std::os::raw::c_long as png_uint_32);
            opng_store_image_info(write_ptr, write_info_ptr,
                                  (outfile !=
                                       0 as *mut std::os::raw::c_void as *mut FILE) as
                                      std::os::raw::c_int);
            opng_init_write_data();
            png_set_write_fn(write_ptr, outfile as png_voidp,
                             Some(opng_write_data),
                             None);
            png_write_png(write_ptr, write_info_ptr, 0 as std::os::raw::c_int,
                          (0 as * mut core::ffi::c_void));
            ::std::ptr::write_volatile(&mut err_msg as
                                           *mut *const std::os::raw::c_char,
                                       0 as *const std::os::raw::c_char);
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh13 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh13 = exception__prev;
    if !((*the_exception_context.as_mut_ptr()).caught == 0 ||
             {
                 ::std::ptr::write_volatile(&mut err_msg as
                                                *mut *const std::os::raw::c_char,
                                            (*the_exception_context.as_mut_ptr()).v.etmp);
                 (0 as std::os::raw::c_int) != 0
             }) {
        /* Override the default libpng settings. */
        /* Write the PNG stream. */
        /* Set IDAT size to invalid. */
        process.out_idat_size =
            idat_size_max.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    /* Destroy the libpng structures. */
    png_destroy_write_struct(Some(&mut write_ptr), Some(&mut write_info_ptr));
    if !err_msg.is_null() {
        loop  {
            let ref mut fresh14 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh14, err_msg);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    };
}
/*
 * PNG file copying.
 */
unsafe extern "C" fn opng_copy_file(mut infile: * mut crate::src::gifread::gifread::_IO_FILE,
                                    mut outfile: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut buf: * mut u8 =
        0 as *mut png_byte; /* volatile is required by cexcept */
    let buf_size_incr: u32 = 0x1000 as std::os::raw::c_int as png_uint_32;
    let mut buf_size: u32 = 0;
    let mut length: u32 = 0;
    let mut chunk_hdr: [u8; 8] = [0; 8];
    let mut err_msg: * const i8 = 0 as *const std::os::raw::c_char;
    write_ptr =
        png_create_write_struct(b"1.6.34\x00" as *const u8 as
                                    *const std::os::raw::c_char,
                                0 as *mut std::os::raw::c_void,
                                Some(opng_error),
                                Some(opng_warning));
    if write_ptr.is_null() {
        loop  {
            let ref mut fresh15 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh15,
                                       b"Out of memory\x00" as *const u8 as
                                           *const std::os::raw::c_char);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    opng_init_write_data();
    png_set_write_fn(write_ptr, outfile as png_voidp,
                     Some(opng_write_data), None);
    let mut exception__prev: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh16 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh16 = &mut exception__env;
    if _setjmp(exception__env.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            ::std::ptr::write_volatile(&mut buf as *mut png_bytep,
                                       0 as png_bytep);
            buf_size = 0 as std::os::raw::c_int as png_uint_32;
            /* everything is ok */
            png_write_sig(write_ptr);
            loop 
                 /* Write the signature in the output file. */
                 /* Copy all chunks until IEND. */
        /* Error checking is done only at a very basic level. */
                 {
                if fread(chunk_hdr.as_mut_ptr() as *mut std::os::raw::c_void,
                         8 as std::os::raw::c_int as std::os::raw::c_ulong,
                         1 as std::os::raw::c_int as std::os::raw::c_ulong, infile) !=
                       1 as std::os::raw::c_int as std::os::raw::c_ulong {
                    loop 
                         /* length + name */
                         {
                        let ref mut fresh17 =
                            (*the_exception_context.as_mut_ptr()).v.etmp;
                        ::std::ptr::write_volatile(fresh17,
                                                   b"Read error\x00" as
                                                       *const u8 as
                                                       *const std::os::raw::c_char);
                        longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                1 as std::os::raw::c_int);
                    }
                }
                length =
                    ((*chunk_hdr.as_mut_ptr() as png_uint_32) <<
                         24 as
                             std::os::raw::c_int).wrapping_add((*chunk_hdr.as_mut_ptr().offset(1
                                                                                           as
                                                                                           std::os::raw::c_int
                                                                                           as
                                                                                           isize)
                                                            as png_uint_32) <<
                                                           16 as
                                                               std::os::raw::c_int).wrapping_add((*chunk_hdr.as_mut_ptr().offset(2
                                                                                                                             as
                                                                                                                             std::os::raw::c_int
                                                                                                                             as
                                                                                                                             isize)
                                                                                              as
                                                                                              png_uint_32)
                                                                                             <<
                                                                                             8
                                                                                                 as
                                                                                                 std::os::raw::c_int).wrapping_add(*chunk_hdr.as_mut_ptr().offset(3
                                                                                                                                                              as
                                                                                                                                                              std::os::raw::c_int
                                                                                                                                                              as
                                                                                                                                                              isize)
                                                                                                                               as
                                                                                                                               png_uint_32);
                if length > 0x7fffffff as std::os::raw::c_long as png_uint_32 {
                    if !(buf.is_null() &&
                             length as std::os::raw::c_ulong ==
                                 0x89504e47 as std::os::raw::c_ulong) {
                        loop  {
                            let ref mut fresh18 =
                                (*the_exception_context.as_mut_ptr()).v.etmp;
                            ::std::ptr::write_volatile(fresh18,
                                                       b"Data error\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char);
                            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                    1 as std::os::raw::c_int);
                        }
                    }
                } else {
                    if length.wrapping_add(4 as std::os::raw::c_int as std::os::raw::c_uint) >
                           buf_size {
                        png_free(write_ptr as *const png_struct,
                                 buf as png_voidp);
                        buf_size =
                            length.wrapping_add(4 as std::os::raw::c_int as
                                                    std::os::raw::c_uint).wrapping_add(buf_size_incr.wrapping_sub(1
                                                                                                              as
                                                                                                              std::os::raw::c_int
                                                                                                              as
                                                                                                              std::os::raw::c_uint)).wrapping_div(buf_size_incr).wrapping_mul(buf_size_incr);
                        ::std::ptr::write_volatile(&mut buf as *mut png_bytep,
                                                   png_malloc(write_ptr as
                                                                  *const png_struct,
                                                              buf_size as
                                                                  png_alloc_size_t)
                                                       as png_bytep)
                        /* Do not use realloc() here, it's slower. */
                    }
                    if fread(buf as *mut std::os::raw::c_void,
                             length.wrapping_add(4 as std::os::raw::c_int as
                                                     std::os::raw::c_uint) as
                                 std::os::raw::c_ulong,
                             1 as std::os::raw::c_int as std::os::raw::c_ulong, infile) !=
                           1 as std::os::raw::c_int as std::os::raw::c_ulong {
                        loop 
                             /* data + crc */
                             {
                            let ref mut fresh19 =
                                (*the_exception_context.as_mut_ptr()).v.etmp;
                            ::std::ptr::write_volatile(fresh19,
                                                       b"Read error\x00" as
                                                           *const u8 as
                                                           *const std::os::raw::c_char);
                            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                    1 as std::os::raw::c_int);
                        }
                    }
                    png_write_chunk(write_ptr,
                                    chunk_hdr.as_mut_ptr().offset(4 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      isize)
                                        as png_const_bytep,
                                    buf as png_const_bytep,
                                    length as png_size_t);
                }
                /* Skip the signature. */
                if !(memcmp(chunk_hdr.as_mut_ptr().offset(4 as std::os::raw::c_int as
                                                              isize) as
                                *const std::os::raw::c_void,
                            sig_IEND.as_ptr() as *const std::os::raw::c_void,
                            4 as std::os::raw::c_int as std::os::raw::c_ulong) !=
                         0 as std::os::raw::c_int) {
                    break ;
                }
            }
            ::std::ptr::write_volatile(&mut err_msg as
                                           *mut *const std::os::raw::c_char,
                                       0 as *const std::os::raw::c_char);
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh20 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh20 = exception__prev;
    ((*the_exception_context.as_mut_ptr()).caught == 0) ||
        {
            ::std::ptr::write_volatile(&mut err_msg as
                                           *mut *const std::os::raw::c_char,
                                       (*the_exception_context.as_mut_ptr()).v.etmp);
            (0 as std::os::raw::c_int) != 0
        };
    png_free(write_ptr as *const png_struct, buf as png_voidp);
    png_destroy_write_struct(Some(&mut write_ptr), Option::<&'_ mut * mut crate::src::libpng::png::png_info_def>::None);
    if !err_msg.is_null() {
        loop  {
            let ref mut fresh21 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh21, err_msg);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    };
}
/*
 * Iteration initialization.
 */
unsafe extern "C" fn opng_init_iteration<'a1>(mut cmdline_set: std::os::raw::c_uint,
                                         mut mask_set: std::os::raw::c_uint,
                                         mut preset: * const std::os::raw::c_char,
                                         mut output_set: Option<&'a1 mut std::os::raw::c_uint>) {
    let mut preset_set: u32 = 0;
    let mut check: i32 = 0;
    *(borrow_mut(&mut output_set)).unwrap() = cmdline_set & mask_set;
    if *(borrow(& output_set)).unwrap() == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           cmdline_set != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        loop  {
            let ref mut fresh22 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh22,
                                       b"Iteration parameter(s) out of range\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    if *(borrow(& output_set)).unwrap() == 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           options.optim_level >= 0 as std::os::raw::c_int {
        check =
            opng_strparse_rangeset_to_bitset(Some(&mut preset_set), preset,
                                             mask_set);
        if !(check == 0 as std::os::raw::c_int) {
            usr_panic.expect("non-null function pointer")(b"[internal] Invalid preset\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
        *(borrow_mut(&mut output_set)).unwrap() |= preset_set & mask_set
    };
}
/*
 * Iteration initialization.
 */
unsafe extern "C" fn opng_init_iterations() {
    let mut compr_level_set: u32 = 0;
    let mut mem_level_set: u32 = 0;
    let mut strategy_set: u32 = 0;
    let mut filter_set: u32 = 0;
    let mut strategy_singles_set: u32 = 0;
    let mut preset_index: i32 = 0;
    let mut t1: i32 = 0;
    let mut t2: i32 = 0;
    /* Set the IDAT size limit. The trials that pass this limit will be
     * abandoned, as there will be no need to wait until their completion.
     * This limit may further decrease as iterations go on.
     */
    if process.status & OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
           != 0 || options.full != 0 {
        process.max_idat_size = idat_size_max
    } else {
        if !(process.in_idat_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong) {
            usr_panic.expect("non-null function pointer")(b"No IDAT in input\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
        /* Add the input PLTE and tRNS sizes to the initial max IDAT size,
         * to account for the changes that may occur during reduction.
         * This incurs a negligible overhead on processing only: the final
         * IDAT size will not be affected, because a precise check will be
         * performed at the end, inside opng_finish_iterations().
         */
        process.max_idat_size =
            process.in_idat_size.wrapping_add(process.in_plte_trns_size as
                                                  std::os::raw::c_ulong)
    }
    /* Get preset_index from options.optim_level, but leave the latter intact,
     * because the effect of "optipng -o2 -z... -f..." is slightly different
     * from the effect of "optipng -z... -f..." (without "-o").
     */
    preset_index = options.optim_level;
    if preset_index < 0 as std::os::raw::c_int {
        preset_index = 2 as std::os::raw::c_int
    } else if preset_index > 7 as std::os::raw::c_int {
        preset_index = 7 as std::os::raw::c_int
    }
    /* Initialize the iteration sets.
     * Combine the user-defined values with the optimization presets.
     */
    opng_init_iteration(options.compr_level_set,
                        (((1 as std::os::raw::c_int) <<
                              9 as std::os::raw::c_int + 1 as std::os::raw::c_int) -
                             ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)) as
                            opng_bitset_t,
                        presets[preset_index as usize].compr_level,
                        Some(&mut compr_level_set));
    opng_init_iteration(options.mem_level_set,
                        (((1 as std::os::raw::c_int) <<
                              9 as std::os::raw::c_int + 1 as std::os::raw::c_int) -
                             ((1 as std::os::raw::c_int) << 1 as std::os::raw::c_int)) as
                            opng_bitset_t,
                        presets[preset_index as usize].mem_level,
                        Some(&mut mem_level_set));
    opng_init_iteration(options.strategy_set,
                        (((1 as std::os::raw::c_int) <<
                              3 as std::os::raw::c_int + 1 as std::os::raw::c_int) -
                             ((1 as std::os::raw::c_int) << 0 as std::os::raw::c_int)) as
                            opng_bitset_t,
                        presets[preset_index as usize].strategy,
                        Some(&mut strategy_set));
    opng_init_iteration(options.filter_set,
                        (((1 as std::os::raw::c_int) <<
                              5 as std::os::raw::c_int + 1 as std::os::raw::c_int) -
                             ((1 as std::os::raw::c_int) << 0 as std::os::raw::c_int)) as
                            opng_bitset_t,
                        presets[preset_index as usize].filter,
                        Some(&mut filter_set));
    /* Replace the empty sets with the libpng's "best guess" heuristics. */
    if compr_level_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        compr_level_set |= (1 as std::os::raw::c_uint) << 9 as std::os::raw::c_int
    } /* -zc9 */
    if mem_level_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        mem_level_set |= (1 as std::os::raw::c_uint) << 8 as std::os::raw::c_int
    } /* -zs0 */
    if image.bit_depth < 8 as std::os::raw::c_int || !image.palette.is_null() {
        if strategy_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            strategy_set |= (1 as std::os::raw::c_uint) << 0 as std::os::raw::c_int
        }
        if filter_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            filter_set |= (1 as std::os::raw::c_uint) << 0 as std::os::raw::c_int
        }
        /* -f0 */
    } else {
        if strategy_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            strategy_set |= (1 as std::os::raw::c_uint) << 1 as std::os::raw::c_int
        }
        if filter_set == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            filter_set |= (1 as std::os::raw::c_uint) << 5 as std::os::raw::c_int
        } /* -zs1 */
        /* -f0 */
    }
    /* Store the results into process. */
    process.compr_level_set = compr_level_set;
    process.mem_level_set = mem_level_set;
    process.strategy_set = strategy_set;
    process.filter_set = filter_set;
    strategy_singles_set =
        ((1 as std::os::raw::c_int) << 2 as std::os::raw::c_int |
             (1 as std::os::raw::c_int) << 3 as std::os::raw::c_int) as opng_bitset_t;
    t1 =
        opng_bitset_count(compr_level_set).wrapping_mul(opng_bitset_count(strategy_set
                                                                              &
                                                                              !strategy_singles_set))
            as std::os::raw::c_int;
    t2 =
        opng_bitset_count(strategy_set & strategy_singles_set) as std::os::raw::c_int;
    process.num_iterations =
        ((t1 + t2) as
             std::os::raw::c_uint).wrapping_mul(opng_bitset_count(mem_level_set)).wrapping_mul(opng_bitset_count(filter_set))
            as std::os::raw::c_int;
    if !(process.num_iterations > 0 as std::os::raw::c_int) {
        usr_panic.expect("non-null function pointer")(b"Invalid iteration parameters\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    };
}
/*
 * Iteration.
 */
unsafe extern "C" fn opng_iterate() {
    let mut compr_level_set: u32 = 0;
    let mut mem_level_set: u32 = 0;
    let mut strategy_set: u32 = 0;
    let mut filter_set: u32 = 0;
    let mut compr_level: i32 = 0;
    let mut mem_level: i32 = 0;
    let mut strategy: i32 = 0;
    let mut filter: i32 = 0;
    let mut counter: i32 = 0;
    let mut line_reused: i32 = 0;
    if !(process.num_iterations > 0 as std::os::raw::c_int) {
        usr_panic.expect("non-null function pointer")(b"Iterations not initialized\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    compr_level_set = process.compr_level_set;
    mem_level_set = process.mem_level_set;
    strategy_set = process.strategy_set;
    filter_set = process.filter_set;
    if process.num_iterations == 1 as std::os::raw::c_int &&
           process.status &
               OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        /* There is only one combination. Select it and return. */
        process.best_idat_size =
            0 as std::os::raw::c_int as opng_fsize_t; /* unknown */
        process.best_compr_level = opng_bitset_find_first(compr_level_set);
        process.best_mem_level = opng_bitset_find_first(mem_level_set);
        process.best_strategy = opng_bitset_find_first(strategy_set);
        process.best_filter = opng_bitset_find_first(filter_set);
        return
    }
    /* Prepare for the big iteration. */
    process.best_idat_size =
        idat_size_max.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
    process.best_compr_level = -(1 as std::os::raw::c_int);
    process.best_mem_level = -(1 as std::os::raw::c_int);
    process.best_strategy = -(1 as std::os::raw::c_int);
    process.best_filter = -(1 as std::os::raw::c_int);
    /* Iterate through the "hyper-rectangle" (zc, zm, zs, f). */
    usr_printf.expect("non-null function pointer")(b"\nTrying:\n\x00" as
                                                       *const u8 as
                                                       *const std::os::raw::c_char);
    line_reused = 0 as std::os::raw::c_int;
    counter = 0 as std::os::raw::c_int;
    filter = 0 as std::os::raw::c_int;
    while filter <= 5 as std::os::raw::c_int {
        if filter_set & (1 as std::os::raw::c_uint) << filter !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            strategy = 0 as std::os::raw::c_int;
            while strategy <= 3 as std::os::raw::c_int {
                if strategy_set & (1 as std::os::raw::c_uint) << strategy !=
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    if strategy == 2 as std::os::raw::c_int {
                        /* Under Z_HUFFMAN_ONLY, all compression levels
                 * (deflate_fast and deflate_slow combined)
                 * produce the same output. Pick level 1.
                 */
                        compr_level_set = 0 as std::os::raw::c_int as opng_bitset_t;
                        compr_level_set |=
                            (1 as std::os::raw::c_uint) << 1 as std::os::raw::c_int
                    } else if strategy == 3 as std::os::raw::c_int {
                        /* Under Z_RLE, all deflate_fast compression levels produce
                 * the same output. Ditto about the deflate_slow levels.
                 * Pick level 9, in preference for deflate_slow.
                 */
                        compr_level_set = 0 as std::os::raw::c_int as opng_bitset_t;
                        compr_level_set |=
                            (1 as std::os::raw::c_uint) << 9 as std::os::raw::c_int
                    } else {
                        /* Restore compr_level_set. */
                        compr_level_set = process.compr_level_set
                    } /* CR: reset line */
                    compr_level = 9 as std::os::raw::c_int;
                    while compr_level >= 1 as std::os::raw::c_int {
                        if compr_level_set &
                               (1 as std::os::raw::c_uint) << compr_level !=
                               0 as std::os::raw::c_int as std::os::raw::c_uint {
                            mem_level = 9 as std::os::raw::c_int;
                            while mem_level >= 1 as std::os::raw::c_int {
                                if mem_level_set &
                                       (1 as std::os::raw::c_uint) << mem_level !=
                                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                                    usr_printf.expect("non-null function pointer")(b"  zc = %d  zm = %d  zs = %d  f = %d\x00"
                                                                                       as
                                                                                       *const u8
                                                                                       as
                                                                                       *const std::os::raw::c_char,
                                                                                   compr_level,
                                                                                   mem_level,
                                                                                   strategy,
                                                                                   filter);
                                    usr_progress.expect("non-null function pointer")(counter
                                                                                         as
                                                                                         std::os::raw::c_ulong,
                                                                                     process.num_iterations
                                                                                         as
                                                                                         std::os::raw::c_ulong);
                                    counter += 1;
                                    opng_write_file(0 as *mut FILE,
                                                    compr_level, mem_level,
                                                    strategy, filter);
                                    if process.out_idat_size > idat_size_max {
                                        if options.verbose != 0 {
                                            usr_printf.expect("non-null function pointer")(b"\t\tIDAT too big\n\x00"
                                                                                               as
                                                                                               *const u8
                                                                                               as
                                                                                               *const std::os::raw::c_char);
                                            line_reused = 0 as std::os::raw::c_int
                                        } else {
                                            usr_print_cntrl.expect("non-null function pointer")('\r'
                                                                                                    as
                                                                                                    i32);
                                            line_reused = 1 as std::os::raw::c_int
                                        }
                                    } else {
                                        usr_printf.expect("non-null function pointer")(b"\t\tIDAT size = %lu\n\x00"
                                                                                           as
                                                                                           *const u8
                                                                                           as
                                                                                           *const std::os::raw::c_char,
                                                                                       process.out_idat_size);
                                        line_reused = 0 as std::os::raw::c_int;
                                        if !(process.best_idat_size <
                                                 process.out_idat_size) {
                                            if !(process.best_idat_size ==
                                                     process.out_idat_size &&
                                                     (process.best_strategy ==
                                                          2 as std::os::raw::c_int ||
                                                          process.best_strategy
                                                              ==
                                                              3 as
                                                                  std::os::raw::c_int))
                                               {
                                                process.best_compr_level =
                                                    compr_level;
                                                process.best_mem_level =
                                                    mem_level;
                                                process.best_strategy =
                                                    strategy;
                                                process.best_filter = filter;
                                                process.best_idat_size =
                                                    process.out_idat_size;
                                                if options.full == 0 {
                                                    process.max_idat_size =
                                                        process.out_idat_size
                                                }
                                            }
                                        }
                                    }
                                }
                                /* The current best size is equal to the last size;
                         * the current best strategy is already the fastest.
                         * Discard the last iteration.
                         */
                                mem_level -= 1
                            }
                        } /* minus N: erase N chars from start of line */
                        compr_level -= 1
                    }
                }
                strategy += 1
            }
        }
        filter += 1
    }
    if line_reused != 0 {
        usr_print_cntrl.expect("non-null function pointer")(-(31 as
                                                                  std::os::raw::c_int));
    }
    if !(counter == process.num_iterations) {
        usr_panic.expect("non-null function pointer")(b"Inconsistent iteration counter\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    usr_progress.expect("non-null function pointer")(counter as std::os::raw::c_ulong,
                                                     process.num_iterations as
                                                         std::os::raw::c_ulong);
}
/*
 * Iteration finalization.
 */
unsafe extern "C" fn opng_finish_iterations() {
    if process.best_idat_size.wrapping_add(process.out_plte_trns_size as
                                               std::os::raw::c_ulong) <
           process.in_idat_size.wrapping_add(process.in_plte_trns_size as
                                                 std::os::raw::c_ulong) {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
           != 0 {
        if process.best_idat_size <= idat_size_max {
            usr_printf.expect("non-null function pointer")(b"\nSelecting parameters:\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            usr_printf.expect("non-null function pointer")(b"  zc = %d  zm = %d  zs = %d  f = %d\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           process.best_compr_level,
                                                           process.best_mem_level,
                                                           process.best_strategy,
                                                           process.best_filter);
            if process.best_idat_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* At least one trial has been run. */
                usr_printf.expect("non-null function pointer")(b"\t\tIDAT size = %lu\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               process.best_idat_size);
            }
            usr_printf.expect("non-null function pointer")(b"\n\x00" as
                                                               *const u8 as
                                                               *const std::os::raw::c_char);
        } else {
            /* The compressed image data is larger than the maximum allowed. */
            usr_printf.expect("non-null function pointer")(b"  zc = *  zm = *  zs = *  f = *\t\tIDAT size > %s\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           idat_size_max_string);
        }
    };
}
/*
 * Image file optimization.
 */
unsafe extern "C" fn opng_optimize_impl(mut infile_name:
                                            * const std::os::raw::c_char) {
    static mut infile: * mut crate::src::gifread::gifread::_IO_FILE =
        (0 as * mut crate::src::gifread::gifread::_IO_FILE); unsafe fn laertes_init_infile() {
infile = 0 as *const FILE as *mut FILE;}//; /* static or volatile is required */
    static mut outfile: * mut crate::src::gifread::gifread::_IO_FILE =
        (0 as * mut crate::src::gifread::gifread::_IO_FILE); unsafe fn laertes_init_outfile() {
outfile = 0 as *const FILE as *mut FILE;}//; /* by cexcept */
    static mut infile_name_local: * const std::os::raw::c_char =
        (0 as * const std::os::raw::c_char); unsafe fn laertes_init_infile_name_local() {
infile_name_local = 0 as *const std::os::raw::c_char;}//; /* prepare for error handling */
    static mut outfile_name: * const std::os::raw::c_char =
        (0 as * const std::os::raw::c_char); unsafe fn laertes_init_outfile_name() {
outfile_name = 0 as *const std::os::raw::c_char;}//; /* finally */
    static mut bakfile_name: * const std::os::raw::c_char =
        (0 as * const std::os::raw::c_char); unsafe fn laertes_init_bakfile_name() {
bakfile_name = 0 as *const std::os::raw::c_char;}//; /* rethrow */
    static mut new_outfile: std::os::raw::c_int = 0; unsafe fn laertes_init_new_outfile() {
new_outfile = 0;}//;
    static mut has_backup: std::os::raw::c_int = 0; unsafe fn laertes_init_has_backup() {
has_backup = 0;}//;
    let mut name_buf: [i8; 4096] = [0; 4096];
    let mut tmp_buf: [i8; 4096] = [0; 4096];
    let mut err_msg: * const i8 = 0 as *const std::os::raw::c_char;
    memset(&mut process as *mut opng_process_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_process_struct>() as std::os::raw::c_ulong);
    if options.force != 0 {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
    }
    ::std::ptr::write_volatile(&mut err_msg as *mut *const std::os::raw::c_char,
                               0 as *const std::os::raw::c_char);
    infile_name_local = infile_name;
    infile =
        fopen(infile_name_local,
              b"rb\x00" as *const u8 as *const std::os::raw::c_char);
    if infile.is_null() {
        loop  {
            let ref mut fresh23 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh23,
                                       b"Can\'t open the input file\x00" as
                                           *const u8 as *const std::os::raw::c_char);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    let mut exception__prev: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh24 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh24 = &mut exception__env;
    if _setjmp(exception__env.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            opng_read_file(infile);
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh25 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh25 = exception__prev;
    if !((*the_exception_context.as_mut_ptr()).caught == 0 ||
             {
                 ::std::ptr::write_volatile(&mut err_msg as
                                                *mut *const std::os::raw::c_char,
                                            (*the_exception_context.as_mut_ptr()).v.etmp);
                 (0 as std::os::raw::c_int) != 0
             }) {
        if err_msg.is_null() {
            usr_panic.expect("non-null function pointer")(b"Mysterious error in opng_read_file\x00"
                                                              as *const u8 as
                                                              *const std::os::raw::c_char);
        }
    }
    fclose(infile);
    if !err_msg.is_null() {
        loop  {
            let ref mut fresh26 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh26, err_msg);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    /* Check the error flag. This must be the first check. */
    if process.status & INPUT_HAS_ERRORS as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(b"Recoverable errors found in input.\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        if options.fix != 0 {
            usr_printf.expect("non-null function pointer")(b" Fixing...\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            process.status |=
                OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
        } else {
            usr_printf.expect("non-null function pointer")(b" Rerun OptiPNG with -fix enabled.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            loop  {
                let ref mut fresh27 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh27,
                                           b"Previous error(s) not fixed\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
    }
    /* Check the junk flag. */
    if process.status & INPUT_HAS_JUNK as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Check the PNG signature and datastream flags. */
    if process.status & INPUT_HAS_PNG_SIGNATURE as std::os::raw::c_int as std::os::raw::c_uint
           == 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
    }
    if process.status &
           INPUT_HAS_PNG_DATASTREAM as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if options.nz != 0 &&
               process.status &
                   OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            usr_printf.expect("non-null function pointer")(b"IDAT recoding is necessary, but is disabled by the user.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            loop  {
                let ref mut fresh28 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh28,
                                           b"Can\'t continue\x00" as *const u8
                                               as *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
    } else {
        process.status |= OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Check the digital signature flag. */
    if process.status &
           INPUT_HAS_DIGITAL_SIGNATURE as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(b"Digital signature found in input.\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        if options.force != 0 {
            usr_printf.expect("non-null function pointer")(b" Erasing...\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            process.status |=
                OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
        } else {
            usr_printf.expect("non-null function pointer")(b" Rerun OptiPNG with -force enabled.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            loop  {
                let ref mut fresh29 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh29,
                                           b"Can\'t optimize digitally-signed files\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
    }
    /* Check the multi-image flag. */
    if process.status &
           INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if options.snip == 0 &&
               process.status &
                   INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint == 0 {
            usr_printf.expect("non-null function pointer")(b"Conversion to PNG requires snipping. Rerun OptiPNG with -snip enabled.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            loop  {
                let ref mut fresh30 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh30,
                                           b"Incompatible input format\x00" as
                                               *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
    }
    if process.status & INPUT_HAS_APNG as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
           options.snip != 0 {
        process.status |= OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Check the stripped-data flag. */
    if process.status & INPUT_HAS_STRIPPED_DATA as std::os::raw::c_int as std::os::raw::c_uint
           != 0 {
        usr_printf.expect("non-null function pointer")(b"Stripping metadata...\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
    }
    /* Initialize the output file name. */
    outfile_name = 0 as *const std::os::raw::c_char; /* override the old name */
    if process.status & INPUT_IS_PNG_FILE as std::os::raw::c_int as std::os::raw::c_uint == 0
       {
        if opng_path_replace_ext(name_buf.as_mut_ptr(),
                                 ::std::mem::size_of::<[std::os::raw::c_char; 4096]>()
                                     as std::os::raw::c_ulong, infile_name_local,
                                 b".png\x00" as *const u8 as
                                     *const std::os::raw::c_char).is_null() {
            loop  {
                let ref mut fresh31 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh31,
                                           b"Can\'t create the output file (name too long)\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
        outfile_name = name_buf.as_mut_ptr()
    }
    if !options.out_name.is_null() { outfile_name = options.out_name }
    if !options.dir_name.is_null() {
        let mut tmp_name: * const i8 = 0 as *const std::os::raw::c_char;
        if !outfile_name.is_null() {
            strcpy(tmp_buf.as_mut_ptr(), outfile_name);
            tmp_name = tmp_buf.as_mut_ptr()
        } else { tmp_name = infile_name_local }
        if opng_path_replace_dir(name_buf.as_mut_ptr(),
                                 ::std::mem::size_of::<[std::os::raw::c_char; 4096]>()
                                     as std::os::raw::c_ulong, tmp_name,
                                 options.dir_name).is_null() {
            loop  {
                let ref mut fresh32 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh32,
                                           b"Can\'t create the output file (name too long)\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
        outfile_name = name_buf.as_mut_ptr()
    }
    if outfile_name.is_null() {
        outfile_name = infile_name_local;
        new_outfile = 0 as std::os::raw::c_int
    } else {
        let mut test_eq: i32 =
            opng_os_test_eq(infile_name_local, outfile_name);
        if test_eq >= 0 as std::os::raw::c_int {
            new_outfile = (test_eq == 0 as std::os::raw::c_int) as std::os::raw::c_int
        } else {
            /* We don't know if the two paths point to the same file.
             * Use a crude path name comparison.
             */
            new_outfile =
                (strcmp(infile_name_local, outfile_name) != 0 as std::os::raw::c_int)
                    as std::os::raw::c_int
        }
    }
    /* Initialize the backup file name. */
    bakfile_name = tmp_buf.as_mut_ptr();
    if new_outfile != 0 {
        if opng_path_make_backup(tmp_buf.as_mut_ptr(),
                                 ::std::mem::size_of::<[std::os::raw::c_char; 4096]>()
                                     as std::os::raw::c_ulong, outfile_name).is_null()
           {
            bakfile_name = 0 as *const std::os::raw::c_char
        }
    } else if opng_path_make_backup(tmp_buf.as_mut_ptr(),
                                    ::std::mem::size_of::<[std::os::raw::c_char; 4096]>()
                                        as std::os::raw::c_ulong,
                                    infile_name_local).is_null() {
        bakfile_name = 0 as *const std::os::raw::c_char
    }
    /* Check the name even in simulation mode, to ensure a uniform behavior. */
    if bakfile_name.is_null() {
        loop  {
            let ref mut fresh33 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh33,
                                       b"Can\'t create backup file (name too long)\x00"
                                           as *const u8 as
                                           *const std::os::raw::c_char);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    /* Check the backup file before engaging in lengthy trials. */
    if options.simulate == 0 &&
           opng_os_test(outfile_name,
                        b"e\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
        if new_outfile != 0 && options.backup == 0 && options.clobber == 0 {
            usr_printf.expect("non-null function pointer")(b"The output file exists. Rerun OptiPNG with -backup enabled.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
            loop  {
                let ref mut fresh34 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh34,
                                           b"Can\'t overwrite the output file\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
        if opng_os_test(outfile_name,
                        b"fw\x00" as *const u8 as *const std::os::raw::c_char) !=
               0 as std::os::raw::c_int ||
               options.clobber == 0 &&
                   opng_os_test(bakfile_name,
                                b"e\x00" as *const u8 as *const std::os::raw::c_char)
                       == 0 as std::os::raw::c_int {
            loop  {
                let ref mut fresh35 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh35,
                                           b"Can\'t back up the existing output file\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
    }
    /* Display the input IDAT/file sizes. */
    if process.status &
           INPUT_HAS_PNG_DATASTREAM as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(b"Input IDAT size = %lu bytes\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char,
                                                       process.in_idat_size);
    }
    usr_printf.expect("non-null function pointer")(b"Input file size = %lu bytes\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   process.in_file_size);
    /* Find the best parameters and see if it's worth recompressing. */
    if options.nz == 0 ||
           process.status &
               OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        opng_init_iterations();
        opng_iterate();
        opng_finish_iterations();
    }
    if process.status & OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint
           != 0 {
        process.status |=
            OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint;
        opng_check_idat_size(process.best_idat_size);
    }
    /* Stop here? */
    if process.status & OUTPUT_NEEDS_NEW_FILE as std::os::raw::c_int as std::os::raw::c_uint
           == 0 {
        usr_printf.expect("non-null function pointer")(b"\n%s is already optimized.\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char,
                                                       infile_name_local);
        if new_outfile == 0 { return }
    }
    if options.simulate != 0 {
        usr_printf.expect("non-null function pointer")(b"\nNo output: simulation mode.\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        return
    }
    /* Make room for the output file. */
    if new_outfile != 0 {
        usr_printf.expect("non-null function pointer")(b"\nOutput file: %s\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char,
                                                       outfile_name);
        if !options.dir_name.is_null() {
            opng_os_create_dir(options.dir_name);
        }
        has_backup = 0 as std::os::raw::c_int;
        if opng_os_test(outfile_name,
                        b"e\x00" as *const u8 as *const std::os::raw::c_char) ==
               0 as std::os::raw::c_int {
            if opng_os_rename(outfile_name, bakfile_name, options.clobber) !=
                   0 as std::os::raw::c_int {
                loop  {
                    let ref mut fresh36 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh36,
                                               b"Can\'t back up the output file\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            has_backup = 1 as std::os::raw::c_int
        }
    } else {
        if opng_os_rename(infile_name_local, bakfile_name, options.clobber) !=
               0 as std::os::raw::c_int {
            loop  {
                let ref mut fresh37 =
                    (*the_exception_context.as_mut_ptr()).v.etmp;
                ::std::ptr::write_volatile(fresh37,
                                           b"Can\'t back up the input file\x00"
                                               as *const u8 as
                                               *const std::os::raw::c_char);
                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                        1 as std::os::raw::c_int);
            }
        }
        has_backup = 1 as std::os::raw::c_int
    }
    outfile =
        fopen(outfile_name, b"wb\x00" as *const u8 as *const std::os::raw::c_char);
    let mut exception__prev_0: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env_0: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev_0 as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh38 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh38 = &mut exception__env_0;
    if _setjmp(exception__env_0.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            if outfile.is_null() {
                loop  {
                    let ref mut fresh39 =
                        (*the_exception_context.as_mut_ptr()).v.etmp;
                    ::std::ptr::write_volatile(fresh39,
                                               b"Can\'t open the output file\x00"
                                                   as *const u8 as
                                                   *const std::os::raw::c_char);
                    longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                            1 as std::os::raw::c_int);
                }
            }
            if process.status &
                   OUTPUT_NEEDS_NEW_IDAT as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                /* Write a brand new PNG datastream to the output. */
                opng_write_file(outfile, process.best_compr_level,
                                process.best_mem_level, process.best_strategy,
                                process.best_filter);
            } else {
                /* Copy the input PNG datastream to the output. */
                infile =
                    fopen(if new_outfile != 0 {
                              infile_name_local
                          } else { bakfile_name },
                          b"rb\x00" as *const u8 as *const std::os::raw::c_char);
                if infile.is_null() {
                    loop  {
                        let ref mut fresh40 =
                            (*the_exception_context.as_mut_ptr()).v.etmp;
                        ::std::ptr::write_volatile(fresh40,
                                                   b"Can\'t reopen the input file\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char);
                        longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                1 as std::os::raw::c_int);
                    }
                }
                let mut exception__prev_1: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
                let mut exception__env_1: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
                    [__jmp_buf_tag{__jmpbuf: [0; 8],
                                   __mask_was_saved: 0,
                                   __saved_mask:
                                       __sigset_t{__val: [0; 16],},}; 1];
                ::std::ptr::write_volatile(&mut exception__prev_1 as
                                               *mut *mut jmp_buf,
                                           (*the_exception_context.as_mut_ptr()).penv);
                let ref mut fresh41 =
                    (*the_exception_context.as_mut_ptr()).penv;
                *fresh41 = &mut exception__env_1;
                if _setjmp(exception__env_1.as_mut_ptr()) == 0 as std::os::raw::c_int
                   {
                    loop  {
                        if process.in_datastream_offset >
                               0 as std::os::raw::c_int as std::os::raw::c_long &&
                               opng_fseeko(infile,
                                           process.in_datastream_offset,
                                           0 as std::os::raw::c_int) !=
                                   0 as std::os::raw::c_int {
                            loop  {
                                let ref mut fresh42 =
                                    (*the_exception_context.as_mut_ptr()).v.etmp;
                                ::std::ptr::write_volatile(fresh42,
                                                           b"Can\'t reposition the input file\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char);
                                longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                        1 as std::os::raw::c_int);
                            }
                        }
                        process.best_idat_size = process.in_idat_size;
                        opng_copy_file(infile, outfile);
                        (*the_exception_context.as_mut_ptr()).caught =
                            0 as std::os::raw::c_int;
                        if !((*the_exception_context.as_mut_ptr()).caught !=
                                 0) {
                            break ;
                        }
                    }
                } else {
                    (*the_exception_context.as_mut_ptr()).caught =
                        1 as std::os::raw::c_int
                }
                let ref mut fresh43 =
                    (*the_exception_context.as_mut_ptr()).penv;
                *fresh43 = exception__prev_1;
                if !((*the_exception_context.as_mut_ptr()).caught == 0 ||
                         {
                             ::std::ptr::write_volatile(&mut err_msg as
                                                            *mut *const std::os::raw::c_char,
                                                        (*the_exception_context.as_mut_ptr()).v.etmp);
                             (0 as std::os::raw::c_int) != 0
                         }) {
                    if err_msg.is_null() {
                        usr_panic.expect("non-null function pointer")(b"Mysterious error in opng_copy_file\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const std::os::raw::c_char);
                    }
                }
                /* rethrow */
                fclose(infile); /* finally */
                if !err_msg.is_null() {
                    loop  {
                        let ref mut fresh44 =
                            (*the_exception_context.as_mut_ptr()).v.etmp;
                        ::std::ptr::write_volatile(fresh44, err_msg);
                        longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                                1 as std::os::raw::c_int);
                    }
                }
            }
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh45 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh45 = exception__prev_0;
    if (*the_exception_context.as_mut_ptr()).caught == 0 ||
           {
               ::std::ptr::write_volatile(&mut err_msg as
                                              *mut *const std::os::raw::c_char,
                                          (*the_exception_context.as_mut_ptr()).v.etmp);
               (0 as std::os::raw::c_int) != 0
           } {
    } else {
        if !outfile.is_null() { fclose(outfile); }
        /* rethrow */
        if has_backup != 0 {
            if opng_os_rename(bakfile_name,
                              (if new_outfile != 0 {
                                   outfile_name
                               } else { infile_name_local }),
                              1 as std::os::raw::c_int) != 0 as std::os::raw::c_int {
                opng_print_warning(b"Can\'t recover the original file from backup\x00"
                                       as *const u8 as *const std::os::raw::c_char);
            }
        } else {
            if new_outfile == 0 {
                usr_panic.expect("non-null function pointer")(b"Overwrote input with no temporary backup\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const std::os::raw::c_char);
            }
            if opng_os_unlink(outfile_name) != 0 as std::os::raw::c_int {
                opng_print_warning(b"Can\'t remove the broken output file\x00"
                                       as *const u8 as *const std::os::raw::c_char);
            }
        }
        loop  {
            let ref mut fresh46 =
                (*the_exception_context.as_mut_ptr()).v.etmp;
            ::std::ptr::write_volatile(fresh46, err_msg);
            longjmp((*(*the_exception_context.as_mut_ptr()).penv).as_mut_ptr(),
                    1 as std::os::raw::c_int);
        }
    }
    /* Restore the original input file and rethrow the exception. */
    /* assert(err_msg == NULL); */
    fclose(outfile);
    /* Preserve file attributes (e.g. ownership, access rights, time stamps)
     * on request, if possible.
     */
    if options.preserve != 0 {
        opng_os_copy_attr(if new_outfile != 0 {
                              infile_name_local
                          } else { bakfile_name }, outfile_name);
    }
    /* Remove the backup file if it is not needed. */
    if new_outfile == 0 && options.backup == 0 {
        if opng_os_unlink(bakfile_name) != 0 as std::os::raw::c_int {
            opng_print_warning(b"Can\'t remove the backup file\x00" as
                                   *const u8 as *const std::os::raw::c_char);
        }
    }
    /* Display the output IDAT/file sizes. */
    usr_printf.expect("non-null function pointer")(b"\nOutput IDAT size = %lu bytes\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   process.out_idat_size);
    if process.status &
           INPUT_HAS_PNG_DATASTREAM as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        usr_printf.expect("non-null function pointer")(b" (\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char);
        opng_print_fsize_difference(process.in_idat_size,
                                    process.out_idat_size, 0 as std::os::raw::c_int);
        usr_printf.expect("non-null function pointer")(b")\x00" as *const u8
                                                           as
                                                           *const std::os::raw::c_char);
    }
    usr_printf.expect("non-null function pointer")(b"\nOutput file size = %lu bytes (\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   process.out_file_size);
    opng_print_fsize_difference(process.in_file_size, process.out_file_size,
                                1 as std::os::raw::c_int);
    usr_printf.expect("non-null function pointer")(b")\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
}
/*
 * Engine initialization.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_initialize<'a1, 'a2>(mut init_options:
                                             Option<&'a1 crate::src::optipng::optim::opng_options>,
                                         mut init_ui: Option<&'a2 crate::src::optipng::optim::opng_ui>)
 -> std::os::raw::c_int {
    /* Initialize and check the validity of the user interface. */
    usr_printf = (*(init_ui).unwrap()).printf_fn;
    usr_print_cntrl = (*(init_ui).unwrap()).print_cntrl_fn;
    usr_progress = (*(init_ui).unwrap()).progress_fn;
    usr_panic = (*(init_ui).unwrap()).panic_fn;
    if usr_printf.is_none() || usr_print_cntrl.is_none() ||
           usr_progress.is_none() || usr_panic.is_none() {
        return -(1 as std::os::raw::c_int)
    }
    /* Initialize and adjust the user options. */
    options = *(init_options).unwrap();
    if options.optim_level == 0 as std::os::raw::c_int {
        options.np = 1 as std::os::raw::c_int;
        options.nc = options.np;
        options.nb = options.nc;
        options.nz = 1 as std::os::raw::c_int
    }
    /* Start the engine. */
    memset(&mut summary as *mut opng_summary_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<opng_summary_struct>() as std::os::raw::c_ulong);
    engine.started = 1 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
/*
 * Engine execution.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_optimize(mut infile_name: * const std::os::raw::c_char)
 -> std::os::raw::c_int {
    let mut err_msg: * const i8 =
        0 as
            *const std::os::raw::c_char; /* volatile not needed, but keeps compilers happy */
    let mut result: i32 = 0;
    if engine.started == 0 {
        usr_panic.expect("non-null function pointer")(b"The OptiPNG engine is not running\x00"
                                                          as *const u8 as
                                                          *const std::os::raw::c_char);
    }
    usr_printf.expect("non-null function pointer")(b"** Processing: %s\n\x00"
                                                       as *const u8 as
                                                       *const std::os::raw::c_char,
                                                   infile_name);
    summary.file_count = summary.file_count.wrapping_add(1);
    opng_clear_image_info();
    let mut exception__prev: * mut [crate::src::optipng::optim::__jmp_buf_tag; 1] = 0 as *mut jmp_buf;
    let mut exception__env: [crate::src::optipng::optim::__jmp_buf_tag; 1] =
        [__jmp_buf_tag{__jmpbuf: [0; 8],
                       __mask_was_saved: 0,
                       __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
    ::std::ptr::write_volatile(&mut exception__prev as *mut *mut jmp_buf,
                               (*the_exception_context.as_mut_ptr()).penv);
    let ref mut fresh47 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh47 = &mut exception__env;
    if _setjmp(exception__env.as_mut_ptr()) == 0 as std::os::raw::c_int {
        loop  {
            opng_optimize_impl(infile_name);
            if process.status &
                   INPUT_HAS_ERRORS as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                summary.err_count = summary.err_count.wrapping_add(1);
                summary.fix_count = summary.fix_count.wrapping_add(1)
            }
            if process.status &
                   INPUT_HAS_MULTIPLE_IMAGES as std::os::raw::c_int as std::os::raw::c_uint !=
                   0 {
                if options.snip != 0 {
                    summary.snip_count = summary.snip_count.wrapping_add(1)
                }
            }
            ::std::ptr::write_volatile(&mut result as *mut std::os::raw::c_int,
                                       0 as std::os::raw::c_int);
            (*the_exception_context.as_mut_ptr()).caught = 0 as std::os::raw::c_int;
            if !((*the_exception_context.as_mut_ptr()).caught != 0) {
                break ;
            }
        }
    } else { (*the_exception_context.as_mut_ptr()).caught = 1 as std::os::raw::c_int }
    let ref mut fresh48 = (*the_exception_context.as_mut_ptr()).penv;
    *fresh48 = exception__prev;
    if !((*the_exception_context.as_mut_ptr()).caught == 0 ||
             {
                 err_msg = (*the_exception_context.as_mut_ptr()).v.etmp;
                 (0 as std::os::raw::c_int) != 0
             }) {
        summary.err_count = summary.err_count.wrapping_add(1);
        opng_print_error(err_msg);
        ::std::ptr::write_volatile(&mut result as *mut std::os::raw::c_int,
                                   -(1 as std::os::raw::c_int))
    }
    opng_destroy_image_info();
    usr_printf.expect("non-null function pointer")(b"\n\x00" as *const u8 as
                                                       *const std::os::raw::c_char);
    return result;
}
/*
 * Engine finalization.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_finalize() -> std::os::raw::c_int {
    /* Print the status report. */
    if options.verbose != 0 ||
           summary.snip_count > 0 as std::os::raw::c_int as std::os::raw::c_uint ||
           summary.err_count > 0 as std::os::raw::c_int as std::os::raw::c_uint {
        usr_printf.expect("non-null function pointer")(b"** Status report\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char);
        usr_printf.expect("non-null function pointer")(b"%u file(s) have been processed.\n\x00"
                                                           as *const u8 as
                                                           *const std::os::raw::c_char,
                                                       summary.file_count);
        if summary.snip_count > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            usr_printf.expect("non-null function pointer")(b"%u multi-image file(s) have been snipped.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           summary.snip_count);
        }
        if summary.err_count > 0 as std::os::raw::c_int as std::os::raw::c_uint {
            usr_printf.expect("non-null function pointer")(b"%u error(s) have been encountered.\n\x00"
                                                               as *const u8 as
                                                               *const std::os::raw::c_char,
                                                           summary.err_count);
            if summary.fix_count > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                usr_printf.expect("non-null function pointer")(b"%u erroneous file(s) have been fixed.\n\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const std::os::raw::c_char,
                                                               summary.fix_count);
            }
        }
    }
    /* Stop the engine. */
    engine.started = 0 as std::os::raw::c_int;
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
