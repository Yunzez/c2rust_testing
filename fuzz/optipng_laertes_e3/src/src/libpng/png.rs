
extern "C" {
    
    
    
    
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn free(__ptr: * mut core::ffi::c_void);
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_safecat;
pub use crate::src::libpng::pngerror::png_set_error_fn;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc_base;
pub use crate::src::libpng::pngmem::png_malloc_warn;
pub use crate::src::libpng::pngwutil::png_save_uint_32;
pub use crate::src::zlib::crc32::crc32;
pub use crate::src::zlib::inflate::inflateReset;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
pub type png_byte = std::os::raw::c_uchar;
pub type png_uint_16 = std::os::raw::c_ushort;
pub type png_int_32 = std::os::raw::c_int;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_size_t = std::os::raw::c_ulong;
pub type png_alloc_size_t = std::os::raw::c_ulong;
pub type png_fixed_point = std::os::raw::c_int;
pub type png_voidp = * mut core::ffi::c_void;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_bytep = * const std::os::raw::c_uchar;
pub type png_uint_16p = * mut std::os::raw::c_ushort;
pub type png_charp = * mut std::os::raw::c_char;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_FILE_p = * mut crate::src::gifread::gifread::_IO_FILE;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_struct_def {
    pub error_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
    pub warning_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
    pub error_ptr: * mut core::ffi::c_void,
    pub write_data_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut std::os::raw::c_uchar,_: std::os::raw::c_ulong,) -> ()>,
    pub read_data_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut std::os::raw::c_uchar,_: std::os::raw::c_ulong,) -> ()>,
    pub io_ptr: * mut core::ffi::c_void,
    pub mode: std::os::raw::c_uint,
    pub flags: std::os::raw::c_uint,
    pub transformations: std::os::raw::c_uint,
    pub zowner: std::os::raw::c_uint,
    pub zstream: crate::src::libpng::png::z_stream_s,
    pub zbuffer_list: * mut crate::src::libpng::png::png_compression_buffer,
    pub zbuffer_size: std::os::raw::c_uint,
    pub zlib_level: std::os::raw::c_int,
    pub zlib_method: std::os::raw::c_int,
    pub zlib_window_bits: std::os::raw::c_int,
    pub zlib_mem_level: std::os::raw::c_int,
    pub zlib_strategy: std::os::raw::c_int,
    pub zlib_set_level: std::os::raw::c_int,
    pub zlib_set_method: std::os::raw::c_int,
    pub zlib_set_window_bits: std::os::raw::c_int,
    pub zlib_set_mem_level: std::os::raw::c_int,
    pub zlib_set_strategy: std::os::raw::c_int,
    pub width: std::os::raw::c_uint,
    pub height: std::os::raw::c_uint,
    pub num_rows: std::os::raw::c_uint,
    pub usr_width: std::os::raw::c_uint,
    pub rowbytes: std::os::raw::c_ulong,
    pub iwidth: std::os::raw::c_uint,
    pub row_number: std::os::raw::c_uint,
    pub chunk_name: std::os::raw::c_uint,
    pub prev_row: * mut std::os::raw::c_uchar,
    pub row_buf: * mut std::os::raw::c_uchar,
    pub try_row: * mut std::os::raw::c_uchar,
    pub tst_row: * mut std::os::raw::c_uchar,
    pub info_rowbytes: std::os::raw::c_ulong,
    pub idat_size: std::os::raw::c_uint,
    pub crc: std::os::raw::c_uint,
    pub palette: * mut crate::src::libpng::png::png_color_struct,
    pub num_palette: std::os::raw::c_ushort,
    pub num_palette_max: std::os::raw::c_int,
    pub num_trans: std::os::raw::c_ushort,
    pub compression: std::os::raw::c_uchar,
    pub filter: std::os::raw::c_uchar,
    pub interlaced: std::os::raw::c_uchar,
    pub pass: std::os::raw::c_uchar,
    pub do_filter: std::os::raw::c_uchar,
    pub color_type: std::os::raw::c_uchar,
    pub bit_depth: std::os::raw::c_uchar,
    pub usr_bit_depth: std::os::raw::c_uchar,
    pub pixel_depth: std::os::raw::c_uchar,
    pub channels: std::os::raw::c_uchar,
    pub usr_channels: std::os::raw::c_uchar,
    pub sig_bytes: std::os::raw::c_uchar,
    pub maximum_pixel_depth: std::os::raw::c_uchar,
    pub transformed_pixel_depth: std::os::raw::c_uchar,
    pub zstream_start: std::os::raw::c_uchar,
    pub background_gamma_type: std::os::raw::c_uchar,
    pub background_gamma: std::os::raw::c_int,
    pub background: crate::src::libpng::png::png_color_16_struct,
    pub output_flush_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,) -> ()>,
    pub flush_dist: std::os::raw::c_uint,
    pub flush_rows: std::os::raw::c_uint,
    pub sig_bit: crate::src::libpng::png::png_color_8_struct,
    pub trans_alpha: * mut std::os::raw::c_uchar,
    pub trans_color: crate::src::libpng::png::png_color_16_struct,
    pub read_row_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>,
    pub write_row_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>,
    pub free_me: std::os::raw::c_uint,
    pub unknown_default: std::os::raw::c_int,
    pub num_chunk_list: std::os::raw::c_uint,
    pub chunk_list: * mut std::os::raw::c_uchar,
    pub big_row_buf: * mut std::os::raw::c_uchar,
    pub compression_type: std::os::raw::c_uchar,
    pub user_width_max: std::os::raw::c_uint,
    pub user_height_max: std::os::raw::c_uint,
    pub user_chunk_cache_max: std::os::raw::c_uint,
    pub user_chunk_malloc_max: std::os::raw::c_ulong,
    pub unknown_chunk: crate::src::libpng::png::png_unknown_chunk_t,
    pub old_big_row_buf_size: std::os::raw::c_ulong,
    pub read_buffer: * mut std::os::raw::c_uchar,
    pub read_buffer_size: std::os::raw::c_ulong,
    pub IDAT_read_size: std::os::raw::c_uint,
    pub io_state: std::os::raw::c_uint,
    pub big_prev_row: * mut std::os::raw::c_uchar,
    pub read_filter: [Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_row_info_struct,_: * mut std::os::raw::c_uchar,_: * const std::os::raw::c_uchar,) -> ()>; 4],
}
impl png_struct_def {
    pub const fn new() -> Self {
        png_struct_def {
        error_fn: None,
        warning_fn: None,
        error_ptr: (0 as * mut core::ffi::c_void),
        write_data_fn: None,
        read_data_fn: None,
        io_ptr: (0 as * mut core::ffi::c_void),
        mode: 0,
        flags: 0,
        transformations: 0,
        zowner: 0,
        zstream: crate::src::libpng::png::z_stream_s::new(),
        zbuffer_list: (0 as * mut crate::src::libpng::png::png_compression_buffer),
        zbuffer_size: 0,
        zlib_level: 0,
        zlib_method: 0,
        zlib_window_bits: 0,
        zlib_mem_level: 0,
        zlib_strategy: 0,
        zlib_set_level: 0,
        zlib_set_method: 0,
        zlib_set_window_bits: 0,
        zlib_set_mem_level: 0,
        zlib_set_strategy: 0,
        width: 0,
        height: 0,
        num_rows: 0,
        usr_width: 0,
        rowbytes: 0,
        iwidth: 0,
        row_number: 0,
        chunk_name: 0,
        prev_row: (0 as * mut std::os::raw::c_uchar),
        row_buf: (0 as * mut std::os::raw::c_uchar),
        try_row: (0 as * mut std::os::raw::c_uchar),
        tst_row: (0 as * mut std::os::raw::c_uchar),
        info_rowbytes: 0,
        idat_size: 0,
        crc: 0,
        palette: (0 as * mut crate::src::libpng::png::png_color_struct),
        num_palette: 0,
        num_palette_max: 0,
        num_trans: 0,
        compression: 0,
        filter: 0,
        interlaced: 0,
        pass: 0,
        do_filter: 0,
        color_type: 0,
        bit_depth: 0,
        usr_bit_depth: 0,
        pixel_depth: 0,
        channels: 0,
        usr_channels: 0,
        sig_bytes: 0,
        maximum_pixel_depth: 0,
        transformed_pixel_depth: 0,
        zstream_start: 0,
        background_gamma_type: 0,
        background_gamma: 0,
        background: crate::src::libpng::png::png_color_16_struct::new(),
        output_flush_fn: None,
        flush_dist: 0,
        flush_rows: 0,
        sig_bit: crate::src::libpng::png::png_color_8_struct::new(),
        trans_alpha: (0 as * mut std::os::raw::c_uchar),
        trans_color: crate::src::libpng::png::png_color_16_struct::new(),
        read_row_fn: None,
        write_row_fn: None,
        free_me: 0,
        unknown_default: 0,
        num_chunk_list: 0,
        chunk_list: (0 as * mut std::os::raw::c_uchar),
        big_row_buf: (0 as * mut std::os::raw::c_uchar),
        compression_type: 0,
        user_width_max: 0,
        user_height_max: 0,
        user_chunk_cache_max: 0,
        user_chunk_malloc_max: 0,
        unknown_chunk: crate::src::libpng::png::png_unknown_chunk_t::new(),
        old_big_row_buf_size: 0,
        read_buffer: (0 as * mut std::os::raw::c_uchar),
        read_buffer_size: 0,
        IDAT_read_size: 0,
        io_state: 0,
        big_prev_row: (0 as * mut std::os::raw::c_uchar),
        read_filter: [None,None,None,None,]
        }
    }
}

impl std::default::Default for png_struct_def {
    fn default() -> Self { png_struct_def::new() }
}

pub type png_row_infop = * mut crate::src::libpng::png::png_row_info_struct;
pub type png_row_info = crate::src::libpng::png::png_row_info_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_row_info_struct {
    pub width: std::os::raw::c_uint,
    pub rowbytes: std::os::raw::c_ulong,
    pub color_type: std::os::raw::c_uchar,
    pub bit_depth: std::os::raw::c_uchar,
    pub channels: std::os::raw::c_uchar,
    pub pixel_depth: std::os::raw::c_uchar,
}
impl png_row_info_struct {
    pub const fn new() -> Self {
        png_row_info_struct {
        width: 0,
        rowbytes: 0,
        color_type: 0,
        bit_depth: 0,
        channels: 0,
        pixel_depth: 0
        }
    }
}

impl std::default::Default for png_row_info_struct {
    fn default() -> Self { png_row_info_struct::new() }
}

pub type uInt = std::os::raw::c_uint;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_unknown_chunk_t {
    pub name: [std::os::raw::c_uchar; 5],
    pub data: * mut std::os::raw::c_uchar,
    pub size: std::os::raw::c_ulong,
    pub location: std::os::raw::c_uchar,
}
impl png_unknown_chunk_t {
    pub const fn new() -> Self {
        png_unknown_chunk_t {
        name: [0,0,0,0,0,],
        data: (0 as * mut std::os::raw::c_uchar),
        size: 0,
        location: 0
        }
    }
}

impl std::default::Default for png_unknown_chunk_t {
    fn default() -> Self { png_unknown_chunk_t::new() }
}

pub type png_write_status_ptr<'a1>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_read_status_ptr<'a1>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>;
pub type png_color_16 = crate::src::libpng::png::png_color_16_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_16_struct {
    pub index: std::os::raw::c_uchar,
    pub red: std::os::raw::c_ushort,
    pub green: std::os::raw::c_ushort,
    pub blue: std::os::raw::c_ushort,
    pub gray: std::os::raw::c_ushort,
}
impl png_color_16_struct {
    pub const fn new() -> Self {
        png_color_16_struct {
        index: 0,
        red: 0,
        green: 0,
        blue: 0,
        gray: 0
        }
    }
}

impl std::default::Default for png_color_16_struct {
    fn default() -> Self { png_color_16_struct::new() }
}

pub type png_color_8 = crate::src::libpng::png::png_color_8_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_8_struct {
    pub red: std::os::raw::c_uchar,
    pub green: std::os::raw::c_uchar,
    pub blue: std::os::raw::c_uchar,
    pub gray: std::os::raw::c_uchar,
    pub alpha: std::os::raw::c_uchar,
}
impl png_color_8_struct {
    pub const fn new() -> Self {
        png_color_8_struct {
        red: 0,
        green: 0,
        blue: 0,
        gray: 0,
        alpha: 0
        }
    }
}

impl std::default::Default for png_color_8_struct {
    fn default() -> Self { png_color_8_struct::new() }
}

pub type png_flush_ptr<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,) -> ()>;
pub type png_colorp = * mut crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_color_struct {
    pub red: std::os::raw::c_uchar,
    pub green: std::os::raw::c_uchar,
    pub blue: std::os::raw::c_uchar,
}
impl png_color_struct {
    pub const fn new() -> Self {
        png_color_struct {
        red: 0,
        green: 0,
        blue: 0
        }
    }
}

impl std::default::Default for png_color_struct {
    fn default() -> Self { png_color_struct::new() }
}

pub type png_compression_bufferp = * mut crate::src::libpng::png::png_compression_buffer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_compression_buffer {
    pub next: * mut crate::src::libpng::png::png_compression_buffer,
    pub output: [std::os::raw::c_uchar; 1],
}
impl png_compression_buffer {
    pub const fn new() -> Self {
        png_compression_buffer {
        next: (0 as * mut crate::src::libpng::png::png_compression_buffer),
        output: [0,]
        }
    }
}

impl std::default::Default for png_compression_buffer {
    fn default() -> Self { png_compression_buffer::new() }
}

pub type z_stream = crate::src::libpng::png::z_stream_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: * const std::os::raw::c_uchar,
    pub avail_in: std::os::raw::c_uint,
    pub total_in: std::os::raw::c_ulong,
    pub next_out: * mut std::os::raw::c_uchar,
    pub avail_out: std::os::raw::c_uint,
    pub total_out: std::os::raw::c_ulong,
    pub msg: * const std::os::raw::c_char,
    pub state: * mut crate::src::zlib::deflate::internal_state,
    pub zalloc: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: std::os::raw::c_uint,_: std::os::raw::c_uint,) -> * mut core::ffi::c_void>,
    pub zfree: Option<unsafe extern "C"  fn(_: * mut core::ffi::c_void,_: * mut core::ffi::c_void,) -> ()>,
    pub opaque: * mut core::ffi::c_void,
    pub data_type: std::os::raw::c_int,
    pub adler: std::os::raw::c_ulong,
    pub reserved: std::os::raw::c_ulong,
}
impl z_stream_s {
    pub const fn new() -> Self {
        z_stream_s {
        next_in: (0 as * const std::os::raw::c_uchar),
        avail_in: 0,
        total_in: 0,
        next_out: (0 as * mut std::os::raw::c_uchar),
        avail_out: 0,
        total_out: 0,
        msg: (0 as * const std::os::raw::c_char),
        state: (0 as * mut crate::src::zlib::deflate::internal_state),
        zalloc: None,
        zfree: None,
        opaque: (0 as * mut core::ffi::c_void),
        data_type: 0,
        adler: 0,
        reserved: 0
        }
    }
}

impl std::default::Default for z_stream_s {
    fn default() -> Self { z_stream_s::new() }
}

pub type uLong = std::os::raw::c_ulong;
pub type voidpf = * mut core::ffi::c_void;
pub type free_func<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
pub type alloc_func<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: std::os::raw::c_uint,_: std::os::raw::c_uint,) -> Option<&'a2 mut core::ffi::c_void>>;
pub type Bytef = std::os::raw::c_uchar;
pub type Byte = std::os::raw::c_uchar;
pub type png_rw_ptr<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut std::os::raw::c_uchar>,_: std::os::raw::c_ulong,) -> ()>;
pub type png_error_ptr<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 std::os::raw::c_char>,) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct png_info_def {
    pub width: std::os::raw::c_uint,
    pub height: std::os::raw::c_uint,
    pub valid: std::os::raw::c_uint,
    pub rowbytes: std::os::raw::c_ulong,
    pub palette: * mut crate::src::libpng::png::png_color_struct,
    pub num_palette: std::os::raw::c_ushort,
    pub num_trans: std::os::raw::c_ushort,
    pub bit_depth: std::os::raw::c_uchar,
    pub color_type: std::os::raw::c_uchar,
    pub compression_type: std::os::raw::c_uchar,
    pub filter_type: std::os::raw::c_uchar,
    pub interlace_type: std::os::raw::c_uchar,
    pub channels: std::os::raw::c_uchar,
    pub pixel_depth: std::os::raw::c_uchar,
    pub spare_byte: std::os::raw::c_uchar,
    pub signature: [std::os::raw::c_uchar; 8],
    pub sig_bit: crate::src::libpng::png::png_color_8_struct,
    pub trans_alpha: * mut std::os::raw::c_uchar,
    pub trans_color: crate::src::libpng::png::png_color_16_struct,
    pub background: crate::src::libpng::png::png_color_16_struct,
    pub hist: * mut std::os::raw::c_ushort,
    pub free_me: std::os::raw::c_uint,
    pub unknown_chunks: * mut crate::src::libpng::png::png_unknown_chunk_t,
    pub unknown_chunks_num: std::os::raw::c_int,
    pub row_pointers: * mut * mut std::os::raw::c_uchar,
}
impl png_info_def {
    pub const fn new() -> Self {
        png_info_def {
        width: 0,
        height: 0,
        valid: 0,
        rowbytes: 0,
        palette: (0 as * mut crate::src::libpng::png::png_color_struct),
        num_palette: 0,
        num_trans: 0,
        bit_depth: 0,
        color_type: 0,
        compression_type: 0,
        filter_type: 0,
        interlace_type: 0,
        channels: 0,
        pixel_depth: 0,
        spare_byte: 0,
        signature: [0,0,0,0,0,0,0,0,],
        sig_bit: crate::src::libpng::png::png_color_8_struct::new(),
        trans_alpha: (0 as * mut std::os::raw::c_uchar),
        trans_color: crate::src::libpng::png::png_color_16_struct::new(),
        background: crate::src::libpng::png::png_color_16_struct::new(),
        hist: (0 as * mut std::os::raw::c_ushort),
        free_me: 0,
        unknown_chunks: (0 as * mut crate::src::libpng::png::png_unknown_chunk_t),
        unknown_chunks_num: 0,
        row_pointers: (0 as * mut * mut std::os::raw::c_uchar)
        }
    }
}

impl std::default::Default for png_info_def {
    fn default() -> Self { png_info_def::new() }
}

pub type png_unknown_chunkp = * mut crate::src::libpng::png::png_unknown_chunk_t;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_infopp = * mut * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_malloc_ptr<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_ulong,) -> Option<&'a2 mut core::ffi::c_void>>;
pub type png_free_ptr<'a1, 'a2>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
pub type z_streamp = * mut crate::src::libpng::png::z_stream_s;
/* The version tests may need to be added to, but the problem warning has
 * consistently been fixed in GCC versions which obtain wide-spread release.
 * The problem is that many versions of GCC rearrange comparison expressions in
 * the optimizer in such a way that the results of the comparison will change
 * if signed integer overflow occurs.  Such comparisons are not permitted in
 * ANSI C90, however GCC isn't clever enough to work out that that do not occur
 * below in png_ascii_from_fp and png_muldiv, so it produces a warning with
 * -Wextra.  Unfortunately this is highly dependent on the optimizer and the
 * machine architecture so the warning comes and goes unpredictably and is
 * impossible to "fix", even were that a good idea.
 */
/* GNU 7.1.x */
/* GNU */
/* Tells libpng that we have already handled the first "num_bytes" bytes
 * of the PNG file signature.  If the PNG data is embedded into another
 * stream we can set num_bytes = 8 so that libpng will not attempt to read
 * or write any of the magic bytes before it starts on the IHDR.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_sig_bytes(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                           mut num_bytes: std::os::raw::c_int) {
    let mut nb: u32 = num_bytes as std::os::raw::c_uint;
    if png_ptr.is_null() { return }
    if num_bytes < 0 as std::os::raw::c_int { nb = 0 as std::os::raw::c_int as std::os::raw::c_uint }
    if nb > 8 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr,
                  b"Too many bytes for PNG signature\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    (*png_ptr).sig_bytes = nb as png_byte;
}
/* Checks whether the supplied bytes match the PNG signature.  We allow
 * checking less than the full 8-byte signature so that those apps that
 * already read the first few bytes of a file to determine the file type
 * can simply check the remaining bytes for extra assurance.  Returns
 * an integer less than, equal to, or greater than zero if sig is found,
 * respectively, to be less than, to match, or be greater than the correct
 * PNG signature (this is the same behavior as strcmp, memcmp, etc).
 */
#[no_mangle]
pub unsafe extern "C" fn png_sig_cmp(mut sig: * const std::os::raw::c_uchar,
                                     mut start: std::os::raw::c_ulong,
                                     mut num_to_check: std::os::raw::c_ulong)
 -> std::os::raw::c_int {
    let mut png_signature: [u8; 8] =
        [137 as std::os::raw::c_int as png_byte, 80 as std::os::raw::c_int as png_byte,
         78 as std::os::raw::c_int as png_byte, 71 as std::os::raw::c_int as png_byte,
         13 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte,
         26 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte];
    if num_to_check > 8 as std::os::raw::c_int as std::os::raw::c_ulong {
        num_to_check = 8 as std::os::raw::c_int as png_size_t
    } else if num_to_check < 1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    }
    if start > 7 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    }
    if start.wrapping_add(num_to_check) > 8 as std::os::raw::c_int as std::os::raw::c_ulong {
        num_to_check = (8 as std::os::raw::c_int as std::os::raw::c_ulong).wrapping_sub(start)
    }
    return memcmp(&*sig.offset(start as isize) as *const png_byte as
                      *const std::os::raw::c_void,
                  &mut *png_signature.as_mut_ptr().offset(start as isize) as
                      *mut png_byte as *const std::os::raw::c_void, num_to_check);
}
/* READ */
/* Function to allocate memory for zlib */
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_zalloc(mut png_ptr: * mut core::ffi::c_void, mut items: std::os::raw::c_uint,
                                    mut size: std::os::raw::c_uint) -> * mut core::ffi::c_void {
    let mut num_bytes: u64 = size as png_alloc_size_t;
    if png_ptr.is_null() { return 0 as *mut std::os::raw::c_void }
    if items as std::os::raw::c_ulong >=
           (!(0 as std::os::raw::c_int as
                  png_alloc_size_t)).wrapping_div(size as std::os::raw::c_ulong) {
        png_warning(png_ptr as *const png_struct,
                    b"Potential overflow in png_zalloc()\x00" as *const u8 as
                        *const std::os::raw::c_char);
        return 0 as *mut std::os::raw::c_void
    }
    num_bytes =
        (num_bytes as std::os::raw::c_ulong).wrapping_mul(items as std::os::raw::c_ulong) as
            png_alloc_size_t as png_alloc_size_t;
    return png_malloc_warn(png_ptr as *const png_struct, num_bytes);
}
/* Function to free memory for zlib */
#[no_mangle]
pub unsafe extern "C" fn png_zfree(mut png_ptr: * mut core::ffi::c_void, mut ptr: * mut core::ffi::c_void) {
    png_free(png_ptr as *const png_struct, ptr);
}
/* Reset the CRC variable to 32 bits of 1's.  Care must be taken
 * in case CRC is > 32 bits to leave the top bits 0.
 */
#[no_mangle]
pub unsafe extern "C" fn png_reset_crc(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    /* The cast is safe because the crc is a 32-bit value. */
    (*png_ptr).crc =
        crc32(0 as std::os::raw::c_int as uLong, 0 as *const Bytef,
              0 as std::os::raw::c_int as uInt) as png_uint_32;
}
/* Calculate the CRC over a section of data.  We can only pass as
 * much data to this routine as the largest single buffer size.  We
 * also check that this data will actually be used before going to the
 * trouble of calculating it.
 */
#[no_mangle]
pub unsafe extern "C" fn png_calculate_crc(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                           mut ptr: * const std::os::raw::c_uchar,
                                           mut length: std::os::raw::c_ulong) {
    let mut need_crc: i32 = 1 as std::os::raw::c_int;
    if 1 as std::os::raw::c_int as std::os::raw::c_uint &
           (*png_ptr).chunk_name >> 29 as std::os::raw::c_int !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*png_ptr).flags & (0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint)
               == 0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint {
            need_crc = 0 as std::os::raw::c_int
        }
    } else if (*png_ptr).flags & 0x800 as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        need_crc = 0 as std::os::raw::c_int
    }
    /* critical */
    /* 'uLong' is defined in zlib.h as unsigned long; this means that on some
    * systems it is a 64-bit value.  crc32, however, returns 32 bits so the
    * following cast is safe.  'uInt' may be no more than 16 bits, so it is
    * necessary to perform a loop here.
    */
    if need_crc != 0 as std::os::raw::c_int &&
           length > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        let mut crc: u64 =
            (*png_ptr).crc as uLong; /* Should never issue a warning */
        loop  {
            let mut safe_length: u32 = length as uInt; /* evil, but safe */
            if safe_length == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                safe_length = -(1 as std::os::raw::c_int) as uInt
            }
            crc = crc32(crc, ptr, safe_length);
            /* The following should never issue compiler warnings; if they do the
          * target system has characteristics that will probably violate other
          * assumptions within the libpng code.
          */
            ptr = ptr.offset(safe_length as isize);
            length =
                (length as
                     std::os::raw::c_ulong).wrapping_sub(safe_length as std::os::raw::c_ulong)
                    as png_size_t as png_size_t;
            if !(length > 0 as std::os::raw::c_int as std::os::raw::c_ulong) { break ; }
        }
        /* And the following is always safe because the crc is only 32 bits. */
        (*png_ptr).crc = crc as png_uint_32
    };
}
/* Check a user supplied version number, called from both read and write
 * functions that create a png_struct.
 */
#[no_mangle]
pub unsafe extern "C" fn png_user_version_check(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                mut user_png_ver:
                                                    * const std::os::raw::c_char)
 -> std::os::raw::c_int {
    /* Libpng versions 1.0.0 and later are binary compatible if the version
    * string matches through the second '.'; we must recompile any
    * applications that use any older library version.
    */
    if !user_png_ver.is_null() {
        let mut i: i32 = -(1 as std::os::raw::c_int);
        let mut found_dots: i32 = 0 as std::os::raw::c_int;
        loop  {
            i += 1;
            if *user_png_ver.offset(i as isize) as std::os::raw::c_int !=
                   (*core::intrinsics::transmute::<&'_ [u8; 7], &'_ [i8; 7]>(b"1.6.34\x00"))[i
                                                                                     as
                                                                                     usize]
                       as std::os::raw::c_int {
                (*png_ptr).flags |= 0x20000 as std::os::raw::c_uint
            }
            if *user_png_ver.offset(i as isize) as std::os::raw::c_int == '.' as i32 {
                found_dots += 1
            }
            if !(found_dots < 2 as std::os::raw::c_int &&
                     *user_png_ver.offset(i as isize) as std::os::raw::c_int !=
                         0 as std::os::raw::c_int &&
                     (*core::intrinsics::transmute::<&'_ [u8; 7], &'_ [i8; 7]>(b"1.6.34\x00"))[i
                                                                                       as
                                                                                       usize]
                         as std::os::raw::c_int != 0 as std::os::raw::c_int) {
                break ;
            }
        }
    } else { (*png_ptr).flags |= 0x20000 as std::os::raw::c_uint }
    if (*png_ptr).flags & 0x20000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        let mut pos: u64 = 0 as std::os::raw::c_int as size_t;
        let mut m: [i8; 128] = [0; 128];
        pos =
            png_safecat(m.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                            std::os::raw::c_ulong, pos,
                        b"Application built with libpng-\x00" as *const u8 as
                            *const std::os::raw::c_char);
        pos =
            png_safecat(m.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                            std::os::raw::c_ulong, pos, user_png_ver);
        pos =
            png_safecat(m.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                            std::os::raw::c_ulong, pos,
                        b" but running with \x00" as *const u8 as
                            *const std::os::raw::c_char);
        pos =
            png_safecat(m.as_mut_ptr(),
                        ::std::mem::size_of::<[std::os::raw::c_char; 128]>() as
                            std::os::raw::c_ulong, pos,
                        b"1.6.34\x00" as *const u8 as *const std::os::raw::c_char);
        png_warning(png_ptr, m.as_mut_ptr() as png_const_charp);
        return 0 as std::os::raw::c_int
    }
    /* Success return. */
    return 1 as std::os::raw::c_int;
}
/* Generic function to create a png_struct for either read or write - this
 * contains the common initialization.
 */
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_create_png_struct(mut user_png_ver:
                                                   * const std::os::raw::c_char,
                                               mut error_ptr: * mut core::ffi::c_void,
                                               mut error_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
                                               mut warn_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
                                               mut mem_ptr: * mut core::ffi::c_void,
                                               mut malloc_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: std::os::raw::c_ulong,) -> * mut core::ffi::c_void>,
                                               mut free_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut core::ffi::c_void,) -> ()>)
 -> * mut crate::src::libpng::png::png_struct_def {
    let mut create_struct: crate::src::libpng::png::png_struct_def =
        png_struct{error_fn: None,
                   warning_fn: None,
                   error_ptr: 0 as *mut std::os::raw::c_void,
                   write_data_fn: None,
                   read_data_fn: None,
                   io_ptr: 0 as *mut std::os::raw::c_void,
                   mode: 0,
                   flags: 0,
                   transformations: 0,
                   zowner: 0,
                   zstream:
                       z_stream{next_in: 0 as *const Bytef,
                                avail_in: 0,
                                total_in: 0,
                                next_out: 0 as *mut Bytef,
                                avail_out: 0,
                                total_out: 0,
                                msg: 0 as *const std::os::raw::c_char,
                                state: 0 as *mut internal_state,
                                zalloc: None,
                                zfree: None,
                                opaque: 0 as *mut std::os::raw::c_void,
                                data_type: 0,
                                adler: 0,
                                reserved: 0,},
                   zbuffer_list: 0 as *mut png_compression_buffer,
                   zbuffer_size: 0,
                   zlib_level: 0,
                   zlib_method: 0,
                   zlib_window_bits: 0,
                   zlib_mem_level: 0,
                   zlib_strategy: 0,
                   zlib_set_level: 0,
                   zlib_set_method: 0,
                   zlib_set_window_bits: 0,
                   zlib_set_mem_level: 0,
                   zlib_set_strategy: 0,
                   width: 0,
                   height: 0,
                   num_rows: 0,
                   usr_width: 0,
                   rowbytes: 0,
                   iwidth: 0,
                   row_number: 0,
                   chunk_name: 0,
                   prev_row: 0 as *mut png_byte,
                   row_buf: 0 as *mut png_byte,
                   try_row: 0 as *mut png_byte,
                   tst_row: 0 as *mut png_byte,
                   info_rowbytes: 0,
                   idat_size: 0,
                   crc: 0,
                   palette: 0 as *mut png_color,
                   num_palette: 0,
                   num_palette_max: 0,
                   num_trans: 0,
                   compression: 0,
                   filter: 0,
                   interlaced: 0,
                   pass: 0,
                   do_filter: 0,
                   color_type: 0,
                   bit_depth: 0,
                   usr_bit_depth: 0,
                   pixel_depth: 0,
                   channels: 0,
                   usr_channels: 0,
                   sig_bytes: 0,
                   maximum_pixel_depth: 0,
                   transformed_pixel_depth: 0,
                   zstream_start: 0,
                   background_gamma_type: 0,
                   background_gamma: 0,
                   background:
                       png_color_16{index: 0,
                                    red: 0,
                                    green: 0,
                                    blue: 0,
                                    gray: 0,},
                   output_flush_fn: None,
                   flush_dist: 0,
                   flush_rows: 0,
                   sig_bit:
                       png_color_8{red: 0,
                                   green: 0,
                                   blue: 0,
                                   gray: 0,
                                   alpha: 0,},
                   trans_alpha: 0 as *mut png_byte,
                   trans_color:
                       png_color_16{index: 0,
                                    red: 0,
                                    green: 0,
                                    blue: 0,
                                    gray: 0,},
                   read_row_fn: None,
                   write_row_fn: None,
                   free_me: 0,
                   unknown_default: 0,
                   num_chunk_list: 0,
                   chunk_list: 0 as *mut png_byte,
                   big_row_buf: 0 as *mut png_byte,
                   compression_type: 0,
                   user_width_max: 0,
                   user_height_max: 0,
                   user_chunk_cache_max: 0,
                   user_chunk_malloc_max: 0,
                   unknown_chunk:
                       png_unknown_chunk{name: [0; 5],
                                         data: 0 as *mut png_byte,
                                         size: 0,
                                         location: 0,},
                   old_big_row_buf_size: 0,
                   read_buffer: 0 as *mut png_byte,
                   read_buffer_size: 0,
                   IDAT_read_size: 0,
                   io_state: 0,
                   big_prev_row: 0 as *mut png_byte,
                   read_filter: [None; 4],};
    /* This temporary stack-allocated structure is used to provide a place to
    * build enough context to allow the user provided memory allocator (if any)
    * to be called.
    */
    memset(&mut create_struct as *mut png_struct as *mut std::os::raw::c_void,
           0 as std::os::raw::c_int,
           ::std::mem::size_of::<png_struct>() as std::os::raw::c_ulong);
    /* Added at libpng-1.2.6 */
    create_struct.user_width_max = 1000000 as std::os::raw::c_int as png_uint_32;
    create_struct.user_height_max = 1000000 as std::os::raw::c_int as png_uint_32;
    /* Added at libpng-1.2.43 and 1.4.0 */
    create_struct.user_chunk_cache_max = 1000 as std::os::raw::c_int as png_uint_32;
    /* Added at libpng-1.2.43 and 1.4.1, required only for read but exists
       * in png_struct regardless.
       */
    create_struct.user_chunk_malloc_max =
        8000000 as std::os::raw::c_int as png_alloc_size_t;
    /* The following two API calls simply set fields in png_struct, so it is safe
    * to do them now even though error handling is not yet set up.
    */
    /* (*error_fn) can return control to the caller after the error_ptr is set,
    * this will result in a memory leak unless the error_fn does something
    * extremely sophisticated.  The design lacks merit but is implicit in the
    * API.
    */
    png_set_error_fn(Some(&mut create_struct), error_ptr, error_fn, warn_fn);
    /* Call the general version checker (shared with read and write code):
          */
    if png_user_version_check(&mut create_struct, user_png_ver) !=
           0 as std::os::raw::c_int {
        let mut png_ptr: * mut crate::src::libpng::png::png_struct_def =
            png_malloc_warn(&mut create_struct,
                            ::std::mem::size_of::<png_struct>() as
                                std::os::raw::c_ulong) as *mut png_struct;
        if !png_ptr.is_null() {
            /* png_ptr->zstream holds a back-pointer to the png_struct, so
                * this can only be done now:
                */
            create_struct.zstream.zalloc =
                Some(png_zalloc);
            create_struct.zstream.zfree =
                Some(png_zfree);
            create_struct.zstream.opaque = png_ptr as voidpf;
            *png_ptr = create_struct;
            /* This is the successful return point */
            return png_ptr
        }
    }
    /* A longjmp because of a bug in the application storage allocator or a
    * simple failure to allocate the png_struct.
    */
    return 0 as png_structp;
}
/* Allocate the memory for an info_struct for the application. */
#[no_mangle]
pub unsafe extern "C" fn png_create_info_struct(mut png_ptr:
                                                    * const crate::src::libpng::png::png_struct_def)
 -> * mut crate::src::libpng::png::png_info_def {
    let mut info_ptr: * mut crate::src::libpng::png::png_info_def = 0 as *mut png_info;
    if png_ptr.is_null() { return 0 as png_infop }
    /* Use the internal API that does not (or at least should not) error out, so
    * that this call always returns ok.  The application typically sets up the
    * error handling *after* creating the info_struct because this is the way it
    * has always been done in 'example.c'.
    */
    info_ptr =
        png_malloc_base(png_ptr,
                        ::std::mem::size_of::<png_info>() as std::os::raw::c_ulong) as
            *mut png_info;
    if !info_ptr.is_null() {
        memset(info_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<png_info>() as std::os::raw::c_ulong);
    }
    return info_ptr;
}
/* This function frees the memory associated with a single info struct.
 * Normally, one would use either png_destroy_read_struct() or
 * png_destroy_write_struct() to free an info struct, but this may be
 * useful for some applications.  From libpng 1.6.0 this function is also used
 * internally to implement the png_info release part of the 'struct' destroy
 * APIs.  This ensures that all possible approaches free the same data (all of
 * it).
 */
#[no_mangle]
pub unsafe extern "C" fn png_destroy_info_struct<'a1>(mut png_ptr:
                                                     * const crate::src::libpng::png::png_struct_def,
                                                 mut info_ptr_ptr:
                                                     Option<&'a1 mut * mut crate::src::libpng::png::png_info_def>) {
    let mut info_ptr: * mut crate::src::libpng::png::png_info_def = 0 as png_inforp;
    if png_ptr.is_null() { return }
    if !borrow(& info_ptr_ptr).is_none() { info_ptr = *(borrow_mut(&mut info_ptr_ptr)).unwrap() }
    if !info_ptr.is_null() {
        /* Do this first in case of an error below; if the app implements its own
       * memory management this can lead to png_free calling png_error, which
       * will abort this routine and return control to the app error handler.
       * An infinite loop may result if it then tries to free the same info
       * ptr.
       */
        *(borrow_mut(&mut info_ptr_ptr)).unwrap() = 0 as *mut png_info;
        png_free_data(png_ptr, info_ptr, 0xffff as std::os::raw::c_uint,
                      -(1 as std::os::raw::c_int));
        memset(info_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<png_info>() as std::os::raw::c_ulong);
        png_free(png_ptr, info_ptr as png_voidp);
    };
}
/* Initialize the info structure.  This is now an internal function (0.89)
 * and applications using it are urged to use png_create_info_struct()
 * instead.  Use deprecated in 1.6.0, internal use removed (used internally it
 * is just a memset).
 *
 * NOTE: it is almost inconceivable that this API is used because it bypasses
 * the user-memory mechanism and the user error handling/warning mechanisms in
 * those cases where it does anything other than a memset.
 */
#[no_mangle]
pub unsafe extern "C" fn png_info_init_3<'a1>(mut ptr_ptr: Option<&'a1 mut * mut crate::src::libpng::png::png_info_def>,
                                         mut png_info_struct_size:
                                             std::os::raw::c_ulong) {
    let mut info_ptr: * mut crate::src::libpng::png::png_info_def = *(borrow_mut(&mut ptr_ptr)).unwrap();
    if info_ptr.is_null() { return }
    if ::std::mem::size_of::<png_info>() as std::os::raw::c_ulong >
           png_info_struct_size {
        *(borrow_mut(&mut ptr_ptr)).unwrap() = 0 as *mut png_info;
        /* The following line is why this API should not be used: */
        free(info_ptr as *mut std::os::raw::c_void);
        info_ptr =
            png_malloc_base(0 as *const png_struct,
                            ::std::mem::size_of::<png_info>() as
                                std::os::raw::c_ulong) as *mut png_info;
        if info_ptr.is_null() { return }
        *(borrow_mut(&mut ptr_ptr)).unwrap() = info_ptr
    }
    /* Set everything to 0 */
    memset(info_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<png_info>() as std::os::raw::c_ulong);
}
/* The following API is not called internally */
#[no_mangle]
pub unsafe extern "C" fn png_data_freer(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                        mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                        mut freer: std::os::raw::c_int,
                                        mut mask: std::os::raw::c_uint) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if freer == 1 as std::os::raw::c_int {
        (*info_ptr).free_me |= mask
    } else if freer == 2 as std::os::raw::c_int {
        (*info_ptr).free_me &= !mask
    } else {
        png_error(png_ptr,
                  b"Unknown freer parameter in png_data_freer\x00" as
                      *const u8 as *const std::os::raw::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_free_data(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut mask: std::os::raw::c_uint,
                                       mut num: std::os::raw::c_int) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    /* Free any tRNS entry */
    if mask & 0x2000 as std::os::raw::c_uint & (*info_ptr).free_me !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        (*info_ptr).valid &= !(0x10 as std::os::raw::c_uint);
        png_free(png_ptr, (*info_ptr).trans_alpha as png_voidp);
        (*info_ptr).trans_alpha = 0 as png_bytep;
        (*info_ptr).num_trans = 0 as std::os::raw::c_int as png_uint_16
    }
    if !(*info_ptr).unknown_chunks.is_null() &&
           mask & 0x200 as std::os::raw::c_uint & (*info_ptr).free_me !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        if num != -(1 as std::os::raw::c_int) {
            png_free(png_ptr,
                     (*(*info_ptr).unknown_chunks.offset(num as isize)).data
                         as png_voidp);
            let ref mut fresh0 =
                (*(*info_ptr).unknown_chunks.offset(num as isize)).data;
            *fresh0 = 0 as *mut png_byte
        } else {
            let mut i: i32 = 0;
            i = 0 as std::os::raw::c_int;
            while i < (*info_ptr).unknown_chunks_num {
                png_free(png_ptr,
                         (*(*info_ptr).unknown_chunks.offset(i as isize)).data
                             as png_voidp);
                i += 1
            }
            png_free(png_ptr, (*info_ptr).unknown_chunks as png_voidp);
            (*info_ptr).unknown_chunks = 0 as png_unknown_chunkp;
            (*info_ptr).unknown_chunks_num = 0 as std::os::raw::c_int
        }
    }
    /* Free any hIST entry */
    if mask & 0x8 as std::os::raw::c_uint & (*info_ptr).free_me !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_free(png_ptr, (*info_ptr).hist as png_voidp);
        (*info_ptr).hist = 0 as png_uint_16p;
        (*info_ptr).valid &= !(0x40 as std::os::raw::c_uint)
    }
    /* Free any PLTE entry that was internally allocated */
    if mask & 0x1000 as std::os::raw::c_uint & (*info_ptr).free_me !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_free(png_ptr, (*info_ptr).palette as png_voidp);
        (*info_ptr).palette = 0 as png_colorp;
        (*info_ptr).valid &= !(0x8 as std::os::raw::c_uint);
        (*info_ptr).num_palette = 0 as std::os::raw::c_int as png_uint_16
    }
    /* Free any image bits attached to the info structure */
    if mask & 0x40 as std::os::raw::c_uint & (*info_ptr).free_me !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if !(*info_ptr).row_pointers.is_null() {
            let mut row: u32 = 0;
            row = 0 as std::os::raw::c_int as png_uint_32;
            while row < (*info_ptr).height {
                png_free(png_ptr,
                         *(*info_ptr).row_pointers.offset(row as isize) as
                             png_voidp);
                row = row.wrapping_add(1)
            }
            png_free(png_ptr, (*info_ptr).row_pointers as png_voidp);
            (*info_ptr).row_pointers = 0 as png_bytepp
        }
        (*info_ptr).valid &= !(0x8000 as std::os::raw::c_uint)
    }
    if num != -(1 as std::os::raw::c_int) { mask &= !(0x4220 as std::os::raw::c_uint) }
    (*info_ptr).free_me &= !mask;
}
/* READ || WRITE */
/* This function returns a pointer to the io_ptr associated with the user
 * functions.  The application should free any memory associated with this
 * pointer before png_write_destroy() or png_read_destroy() are called.
 */
#[no_mangle]
pub unsafe extern "C" fn png_get_io_ptr(mut png_ptr: * const crate::src::libpng::png::png_struct_def)
 -> * mut core::ffi::c_void {
    if png_ptr.is_null() { return 0 as *mut std::os::raw::c_void }
    return (*png_ptr).io_ptr;
}
/* Initialize the default input/output functions for the PNG file.  If you
 * use your own read or write routines, you can call either png_set_read_fn()
 * or png_set_write_fn() instead of png_init_io().  If you have defined
 * PNG_NO_STDIO or otherwise disabled PNG_STDIO_SUPPORTED, you must use a
 * function of your own because "FILE *" isn't necessarily available.
 */
#[no_mangle]
pub unsafe extern "C" fn png_init_io<'a1>(mut png_ptr: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,
                                     mut fp: * mut crate::src::gifread::gifread::_IO_FILE) {
    if borrow(& png_ptr).is_none() { return }
    (*(borrow_mut(&mut png_ptr)).unwrap()).io_ptr = fp as png_voidp;
}
/* PNG signed integers are saved in 32-bit 2's complement format.  ANSI C-90
 * defines a cast of a signed integer to an unsigned integer either to preserve
 * the value, if it is positive, or to calculate:
 *
 *     (UNSIGNED_MAX+1) + integer
 *
 * Where UNSIGNED_MAX is the appropriate maximum unsigned value, so when the
 * negative integral value is added the result will be an unsigned value
 * correspnding to the 2's complement representation.
 */
#[no_mangle]
pub unsafe extern "C" fn png_save_int_32(mut buf: * mut std::os::raw::c_uchar,
                                         mut i: std::os::raw::c_int) {
    png_save_uint_32(buf, i as png_uint_32);
}
/* TIME_RFC1123 */
/* READ || WRITE */
#[no_mangle]
pub extern "C" fn png_get_copyright<'a1>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> * const std::os::raw::c_char {
    /* Silence compiler warning about unused png_ptr */
    return b"\nlibpng version 1.6.34 - September 29, 2017\nCopyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson\nCopyright (c) 1996-1997 Andreas Dilger\nCopyright (c) 1995-1996 Guy Eric Schalnat, Group 42, Inc.\n\x00"
               as *const u8 as *const std::os::raw::c_char;
}
/* The following return the library version as a short string in the
 * format 1.0.0 through 99.99.99zz.  To get the version of *.h files
 * used with your application, print out PNG_LIBPNG_VER_STRING, which
 * is defined in png.h.
 * Note: now there is no difference between png_get_libpng_ver() and
 * png_get_header_ver().  Due to the version_nn_nn_nn typedef guard,
 * it is guaranteed that png.c uses the correct version of png.h.
 */
#[no_mangle]
pub extern "C" fn png_get_libpng_ver<'a1>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> * const std::os::raw::c_char {
    /* Version of *.c files used when building libpng */
    return png_get_header_ver((png_ptr).clone());
}
#[no_mangle]
pub extern "C" fn png_get_header_ver<'a1>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> * const std::os::raw::c_char {
    /* Version of *.h files used when building libpng */
    /* Silence compiler warning about unused png_ptr */
    return b"1.6.34\x00" as *const u8 as *const std::os::raw::c_char;
}
#[no_mangle]
pub extern "C" fn png_get_header_version<'a1>(mut png_ptr:
                                                    Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> * const std::os::raw::c_char {
    /* Returns longer string containing both version and date */
    /* Silence compiler warning about unused png_ptr */
    return b" libpng version 1.6.34 - September 29, 2017\n\n\x00" as *const u8
               as *const std::os::raw::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn png_handle_as_unknown(mut png_ptr:
                                                   * const crate::src::libpng::png::png_struct_def,
                                               mut chunk_name:
                                                   * const std::os::raw::c_uchar)
 -> std::os::raw::c_int {
    /* Check chunk_name and return "keep" value if it's on the list, else 0 */
    let mut p: * const u8 = 0 as *const png_byte; /* beyond end */
    let mut p_end: * const u8 = 0 as *const png_byte;
    if png_ptr.is_null() || chunk_name.is_null() ||
           (*png_ptr).num_chunk_list == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int
    }
    p_end = (*png_ptr).chunk_list as png_const_bytep;
    p =
        p_end.offset((*png_ptr).num_chunk_list.wrapping_mul(5 as std::os::raw::c_int
                                                                as
                                                                std::os::raw::c_uint)
                         as isize);
    loop 
         /* The code is the fifth byte after each four byte string.  Historically this
    * code was always searched from the end of the list, this is no longer
    * necessary because the 'set' routine handles duplicate entries correcty.
    */
         /* num_chunk_list > 0, so at least one */
         {
        p = p.offset(-(5 as std::os::raw::c_int as isize));
        if memcmp(chunk_name as *const std::os::raw::c_void, p as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            return *p.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int
        }
        if !(p > p_end) { break ; }
    }
    /* This means that known chunks should be processed and unknown chunks should
    * be handled according to the value of png_ptr->unknown_default; this can be
    * confusing because, as a result, there are two levels of defaulting for
    * unknown chunks.
    */
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_chunk_unknown_handling(mut png_ptr:
                                                        * const crate::src::libpng::png::png_struct_def,
                                                    mut chunk_name:
                                                        std::os::raw::c_uint)
 -> std::os::raw::c_int {
    let mut chunk_string: [u8; 5] = [0; 5];
    *(chunk_string.as_mut_ptr() as
          *mut std::os::raw::c_char).offset(0 as std::os::raw::c_int as isize) =
        (chunk_name >> 24 as std::os::raw::c_int &
             0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
    *(chunk_string.as_mut_ptr() as
          *mut std::os::raw::c_char).offset(1 as std::os::raw::c_int as isize) =
        (chunk_name >> 16 as std::os::raw::c_int &
             0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
    *(chunk_string.as_mut_ptr() as
          *mut std::os::raw::c_char).offset(2 as std::os::raw::c_int as isize) =
        (chunk_name >> 8 as std::os::raw::c_int & 0xff as std::os::raw::c_int as std::os::raw::c_uint)
            as std::os::raw::c_char;
    *(chunk_string.as_mut_ptr() as
          *mut std::os::raw::c_char).offset(3 as std::os::raw::c_int as isize) =
        (chunk_name & 0xff as std::os::raw::c_int as std::os::raw::c_uint) as std::os::raw::c_char;
    *(chunk_string.as_mut_ptr() as
          *mut std::os::raw::c_char).offset(4 as std::os::raw::c_int as isize) =
        0 as std::os::raw::c_int as std::os::raw::c_char;
    return png_handle_as_unknown(png_ptr,
                                 chunk_string.as_mut_ptr() as
                                     png_const_bytep);
}
/* READ_UNKNOWN_CHUNKS || HANDLE_AS_UNKNOWN */
/* SET_UNKNOWN_CHUNKS */
/* This function, added to libpng-1.0.6g, is untested. */
#[no_mangle]
pub unsafe extern "C" fn png_reset_zstream<'a1>(mut png_ptr: Option<&'a1 mut crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_int {
    if borrow(& png_ptr).is_none() { return -(2 as std::os::raw::c_int) }
    /* WARNING: this resets the window bits to the maximum! */
    return inflateReset(&mut (*(borrow_mut(&mut png_ptr)).unwrap()).zstream);
}
/* READ */
/* This function was added to libpng-1.0.7 */
#[no_mangle]
pub extern "C" fn png_access_version_number() -> std::os::raw::c_uint {
    /* Version of *.c files used when building libpng */
    return 10634 as std::os::raw::c_int as png_uint_32;
}
/* Ensure that png_ptr->zstream.msg holds some appropriate error message string.
 * If it doesn't 'ret' is used to set it to something appropriate, even in cases
 * like Z_OK or Z_STREAM_END where the error code is apparently a success code.
 */
#[no_mangle]
pub unsafe extern "C" fn png_zstream_error(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                           mut ret: std::os::raw::c_int) {
    /* Translate 'ret' into an appropriate error string, priority is given to the
    * one in zstream if set.  This always returns a string, even in cases like
    * Z_OK or Z_STREAM_END where the error code is a success code.
    */
    if (*png_ptr).zstream.msg.is_null() {
        match ret {
            1 => {
                /* Normal exit */
                (*png_ptr).zstream.msg =
                    b"unexpected end of LZ stream\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            2 => {
                /* This means the deflate stream did not have a dictionary; this
          * indicates a bogus PNG.
          */
                (*png_ptr).zstream.msg =
                    b"missing LZ dictionary\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            -1 => {
                /* gz APIs only: should not happen */
                (*png_ptr).zstream.msg =
                    b"zlib IO error\x00" as *const u8 as *const std::os::raw::c_char
            }
            -2 => {
                /* internal libpng error */
                (*png_ptr).zstream.msg =
                    b"bad parameters to zlib\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            -3 => {
                (*png_ptr).zstream.msg =
                    b"damaged LZ stream\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            -4 => {
                (*png_ptr).zstream.msg =
                    b"insufficient memory\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            -5 => {
                /* End of input or output; not a problem if the caller is doing
          * incremental read or write.
          */
                (*png_ptr).zstream.msg =
                    b"truncated\x00" as *const u8 as *const std::os::raw::c_char
            }
            -6 => {
                (*png_ptr).zstream.msg =
                    b"unsupported zlib version\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            -7 => {
                /* Compile errors here mean that zlib now uses the value co-opted in
          * pngpriv.h for PNG_UNEXPECTED_ZLIB_RETURN; update the switch above
          * and change pngpriv.h.  Note that this message is "... return",
          * whereas the default/Z_OK one is "... return code".
          */
                (*png_ptr).zstream.msg =
                    b"unexpected zlib return\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
            0 | _ => {
                (*png_ptr).zstream.msg =
                    b"unexpected zlib return code\x00" as *const u8 as
                        *const std::os::raw::c_char
            }
        }
    };
}
/* png_convert_size: a PNGAPI but no longer in png.h, so deleted
 * at libpng 1.5.5!
 */
/* Added at libpng version 1.2.34 and 1.4.0 (moved from pngset.c) */
/* always set if COLORSPACE */
/* GAMMA */
/* COLORSPACE */
/* This exists solely to work round a warning from GNU C. */
 extern "C" fn png_gt(mut a: std::os::raw::c_ulong, mut b: std::os::raw::c_ulong) -> std::os::raw::c_int {
    return (a > b) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_check_IHDR(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                        mut width: std::os::raw::c_uint,
                                        mut height: std::os::raw::c_uint,
                                        mut bit_depth: std::os::raw::c_int,
                                        mut color_type: std::os::raw::c_int,
                                        mut interlace_type: std::os::raw::c_int,
                                        mut compression_type: std::os::raw::c_int,
                                        mut filter_type: std::os::raw::c_int) {
    let mut error: i32 = 0 as std::os::raw::c_int;
    /* Check for width and height valid values */
    if width == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr,
                    b"Image width is zero in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if width > 0x7fffffff as std::os::raw::c_long as png_uint_32 {
        png_warning(png_ptr,
                    b"Invalid image width in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if png_gt((width.wrapping_add(7 as std::os::raw::c_int as std::os::raw::c_uint) &
                   !(7 as std::os::raw::c_uint)) as size_t,
              (-(1 as std::os::raw::c_int) as
                   png_size_t).wrapping_sub(48 as std::os::raw::c_int as
                                                std::os::raw::c_ulong).wrapping_sub(1
                                                                                as
                                                                                std::os::raw::c_int
                                                                                as
                                                                                std::os::raw::c_ulong).wrapping_div(8
                                                                                                                as
                                                                                                                std::os::raw::c_int
                                                                                                                as
                                                                                                                std::os::raw::c_ulong).wrapping_sub(1
                                                                                                                                                as
                                                                                                                                                std::os::raw::c_int
                                                                                                                                                as
                                                                                                                                                std::os::raw::c_ulong))
           != 0 {
        /* extra max_pixel_depth pad */
        /* The size of the row must be within the limits of this architecture.
       * Because the read code can perform arbitrary transformations the
       * maximum size is checked here.  Because the code in png_read_start_row
       * adds extra space "for safety's sake" in several places a conservative
       * limit is used here.
       *
       * NOTE: it would be far better to check the size that is actually used,
       * but the effect in the real world is minor and the changes are more
       * extensive, therefore much more dangerous and much more difficult to
       * write in a way that avoids compiler warnings.
       */
        png_warning(png_ptr,
                    b"Image width is too large for this architecture\x00" as
                        *const u8 as *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if width > (*png_ptr).user_width_max {
        png_warning(png_ptr,
                    b"Image width exceeds user limit in IHDR\x00" as *const u8
                        as *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if height == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr,
                    b"Image height is zero in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if height > 0x7fffffff as std::os::raw::c_long as png_uint_32 {
        png_warning(png_ptr,
                    b"Invalid image height in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if height > (*png_ptr).user_height_max {
        png_warning(png_ptr,
                    b"Image height exceeds user limit in IHDR\x00" as
                        *const u8 as *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    /* Check other values */
    if bit_depth != 1 as std::os::raw::c_int && bit_depth != 2 as std::os::raw::c_int &&
           bit_depth != 4 as std::os::raw::c_int && bit_depth != 8 as std::os::raw::c_int &&
           bit_depth != 16 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid bit depth in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if color_type < 0 as std::os::raw::c_int || color_type == 1 as std::os::raw::c_int ||
           color_type == 5 as std::os::raw::c_int || color_type > 6 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid color type in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if color_type == 2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           bit_depth > 8 as std::os::raw::c_int ||
           (color_type == 2 as std::os::raw::c_int || color_type == 4 as std::os::raw::c_int
                || color_type == 2 as std::os::raw::c_int | 4 as std::os::raw::c_int) &&
               bit_depth < 8 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Invalid color type/bit depth combination in IHDR\x00" as
                        *const u8 as *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if interlace_type >= 2 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Unknown interlace method in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if compression_type != 0 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Unknown compression method in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if filter_type != 0 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Unknown filter method in IHDR\x00" as *const u8 as
                        *const std::os::raw::c_char);
        error = 1 as std::os::raw::c_int
    }
    if error == 1 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"Invalid IHDR data\x00" as *const u8 as
                      *const std::os::raw::c_char);
    };
}
use crate::laertes_rt::*;
/* READ || WRITE */
/* SIMPLIFIED READ/WRITE */
/* SIMPLIFIED READ/WRITE SUPPORT */
/* SIMPLIFIED READ/WRITE sRGB support */
/* sRGB support */
/* HARDWARE OR SOFTWARE OPTION SUPPORT */
/* READ_GAMMA */
/* gamma table code */
/* READ_GAMMA */
/* more fixed point functions for gamma */
/* READ_GAMMA || INCH_CONVERSIONS */
/* SCAL */
/* pCAL || sCAL */
