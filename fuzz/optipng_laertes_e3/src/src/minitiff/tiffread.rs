
extern "C" {
    
    
    
    #[no_mangle]
    static minitiff_sig_m: [std::os::raw::c_char; 4];
    #[no_mangle]
    static minitiff_sig_i: [std::os::raw::c_char; 4];
    #[no_mangle]
    static minitiff_sig_bigm: [std::os::raw::c_char; 4];
    #[no_mangle]
    static minitiff_sig_bigi: [std::os::raw::c_char; 4];
    
    
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fseek(__stream: * mut crate::src::gifread::gifread::_IO_FILE, __off: std::os::raw::c_long, __whence: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn ftell(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_long;
    
    
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
}
pub use crate::src::minitiff::tiffutil::minitiff_error;
pub use crate::src::minitiff::tiffutil::minitiff_warning;
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
pub struct minitiff_info {
    pub error_handler: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()>,
    pub warning_handler: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_char,) -> ()>,
    pub byte_order: std::os::raw::c_int,
    pub width: std::os::raw::c_ulong,
    pub height: std::os::raw::c_ulong,
    pub bits_per_sample: std::os::raw::c_uint,
    pub compression: std::os::raw::c_uint,
    pub photometric: std::os::raw::c_uint,
    pub strip_offsets_count: std::os::raw::c_ulong,
    pub strip_offsets: * mut std::os::raw::c_ulong,
    pub orientation: std::os::raw::c_uint,
    pub samples_per_pixel: std::os::raw::c_uint,
    pub rows_per_strip: std::os::raw::c_ulong,
}
impl minitiff_info {
    pub const fn new() -> Self {
        minitiff_info {
        error_handler: None,
        warning_handler: None,
        byte_order: 0,
        width: 0,
        height: 0,
        bits_per_sample: 0,
        compression: 0,
        photometric: 0,
        strip_offsets_count: 0,
        strip_offsets: (0 as * mut std::os::raw::c_ulong),
        orientation: 0,
        samples_per_pixel: 0,
        rows_per_strip: 0
        }
    }
}

impl std::default::Default for minitiff_info {
    fn default() -> Self { minitiff_info::new() }
}

/*
 * Memory reader structure.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minitiff_getter {
    pub get_ushort: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_uchar,) -> std::os::raw::c_uint>,
    pub get_ulong: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_uchar,) -> std::os::raw::c_ulong>,
}
impl minitiff_getter {
    pub const fn new() -> Self {
        minitiff_getter {
        get_ushort: None,
        get_ulong: None
        }
    }
}

impl std::default::Default for minitiff_getter {
    fn default() -> Self { minitiff_getter::new() }
}

pub const MINITIFF_TYPE_LONG: C2RustUnnamed = 4;
pub const MINITIFF_TYPE_SHORT: C2RustUnnamed = 3;
pub const MINITIFF_TYPE_BYTE: C2RustUnnamed = 1;
pub const MINITIFF_TAG_PRINT_IM: C2RustUnnamed_0 = 50341;
pub const MINITIFF_TAG_INTEROPERABILITY_IFD: C2RustUnnamed_0 = 40965;
pub const MINITIFF_TAG_GPS_IFD: C2RustUnnamed_0 = 34853;
pub const MINITIFF_TAG_ICC_PROFILE: C2RustUnnamed_0 = 34675;
pub const MINITIFF_TAG_EXIF_IFD: C2RustUnnamed_0 = 34665;
pub const MINITIFF_TAG_IPTC: C2RustUnnamed_0 = 33723;
pub const MINITIFF_TAG_XMP: C2RustUnnamed_0 = 700;
pub const MINITIFF_TAG_PREDICTOR: C2RustUnnamed_0 = 317;
pub const MINITIFF_TAG_PLANAR_CONFIGURATION: C2RustUnnamed_0 = 284;
pub const MINITIFF_TAG_STRIP_BYTE_COUNTS: C2RustUnnamed_0 = 279;
pub const MINITIFF_TAG_ROWS_PER_STRIP: C2RustUnnamed_0 = 278;
pub const MINITIFF_TAG_SAMPLES_PER_PIXEL: C2RustUnnamed_0 = 277;
pub const MINITIFF_TAG_ORIENTATION: C2RustUnnamed_0 = 274;
pub const MINITIFF_TAG_STRIP_OFFSETS: C2RustUnnamed_0 = 273;
pub const MINITIFF_TAG_PHOTOMETRIC: C2RustUnnamed_0 = 262;
pub const MINITIFF_TAG_COMPRESSION: C2RustUnnamed_0 = 259;
pub const MINITIFF_TAG_BITS_PER_SAMPLE: C2RustUnnamed_0 = 258;
pub const MINITIFF_TAG_HEIGHT: C2RustUnnamed_0 = 257;
pub const MINITIFF_TAG_WIDTH: C2RustUnnamed_0 = 256;
pub const MINITIFF_TAG_SUBFILE_TYPE: C2RustUnnamed_0 = 254;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const MINITIFF_TYPE_IFD64: C2RustUnnamed = 18;
pub const MINITIFF_TYPE_SLONG64: C2RustUnnamed = 17;
pub const MINITIFF_TYPE_LONG64: C2RustUnnamed = 16;
pub const MINITIFF_TYPE_IFD: C2RustUnnamed = 13;
pub const MINITIFF_TYPE_DOUBLE: C2RustUnnamed = 12;
pub const MINITIFF_TYPE_FLOAT: C2RustUnnamed = 11;
pub const MINITIFF_TYPE_SRATIONAL: C2RustUnnamed = 10;
pub const MINITIFF_TYPE_SLONG: C2RustUnnamed = 9;
pub const MINITIFF_TYPE_SSHORT: C2RustUnnamed = 8;
pub const MINITIFF_TYPE_UNDEFINED: C2RustUnnamed = 7;
pub const MINITIFF_TYPE_SBYTE: C2RustUnnamed = 6;
pub const MINITIFF_TYPE_RATIONAL: C2RustUnnamed = 5;
pub const MINITIFF_TYPE_ASCII: C2RustUnnamed = 2;
pub const MINITIFF_TYPE_NONE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = std::os::raw::c_uint;
pub const MINITIFF_TAG_COPYRIGHT: C2RustUnnamed_0 = 33432;
pub const MINITIFF_TAG_BYTE_COUNTS: C2RustUnnamed_0 = 325;
pub const MINITIFF_TAG_TILE_OFFSETS: C2RustUnnamed_0 = 324;
pub const MINITIFF_TAG_TILE_LENGTH: C2RustUnnamed_0 = 323;
pub const MINITIFF_TAG_TILE_WIDTH: C2RustUnnamed_0 = 322;
pub const MINITIFF_TAG_HALFTONE_HINTS: C2RustUnnamed_0 = 321;
pub const MINITIFF_TAG_COLOR_MAP: C2RustUnnamed_0 = 320;
pub const MINITIFF_TAG_PRIMARY_CHROMATICITIES: C2RustUnnamed_0 = 319;
pub const MINITIFF_TAG_WHITE_POINT: C2RustUnnamed_0 = 318;
pub const MINITIFF_TAG_HOST_COMPUTER: C2RustUnnamed_0 = 316;
pub const MINITIFF_TAG_ARTIST: C2RustUnnamed_0 = 315;
pub const MINITIFF_TAG_DATE_TIME: C2RustUnnamed_0 = 306;
pub const MINITIFF_TAG_SOFTWARE: C2RustUnnamed_0 = 305;
pub const MINITIFF_TAG_TRANSFER_FUNCTION: C2RustUnnamed_0 = 301;
pub const MINITIFF_TAG_PAGE_NUMBER: C2RustUnnamed_0 = 297;
pub const MINITIFF_TAG_RESOLUTION_UNIT: C2RustUnnamed_0 = 296;
pub const MINITIFF_TAG_Y_POSITION: C2RustUnnamed_0 = 287;
pub const MINITIFF_TAG_X_POSITION: C2RustUnnamed_0 = 286;
pub const MINITIFF_TAG_PAGE_NAME: C2RustUnnamed_0 = 285;
pub const MINITIFF_TAG_Y_RESOLUTION: C2RustUnnamed_0 = 283;
pub const MINITIFF_TAG_X_RESOLUTION: C2RustUnnamed_0 = 282;
pub const MINITIFF_TAG_MAX_SAMPLE_VALUE: C2RustUnnamed_0 = 281;
pub const MINITIFF_TAG_MIN_SAMPLE_VALUE: C2RustUnnamed_0 = 280;
pub const MINITIFF_TAG_MODEL: C2RustUnnamed_0 = 272;
pub const MINITIFF_TAG_MAKE: C2RustUnnamed_0 = 271;
pub const MINITIFF_TAG_IMAGE_DESCRIPTION: C2RustUnnamed_0 = 270;
pub const MINITIFF_TAG_DOCUMENT_NAME: C2RustUnnamed_0 = 269;
pub const MINITIFF_TAG_FILL_ORDER: C2RustUnnamed_0 = 266;
pub const MINITIFF_TAG_CELL_LENGTH: C2RustUnnamed_0 = 265;
pub const MINITIFF_TAG_CELL_WIDTH: C2RustUnnamed_0 = 264;
pub const MINITIFF_TAG_THRESHOLDING: C2RustUnnamed_0 = 263;
pub const MINITIFF_TAG_OLD_SUBFILE_TYPE: C2RustUnnamed_0 = 255;
/*
 * tiffread.c
 * File input routines for minitiff.
 *
 * Copyright (C) 2006-2017 Cosmin Truta.
 *
 * minitiff is open-source software, distributed under the zlib license.
 * For conditions of distribution and use, see copyright notice in minitiff.h.
 */
/*
 * Error messages.
 */
static mut msg_err_alloc: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_alloc() {
msg_err_alloc = b"Error allocating memory for TIFF file\x00" as *const u8 as
        *const std::os::raw::c_char;}//;
static mut msg_err_read: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_read() {
msg_err_read = b"Error reading TIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_err_seek: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_seek() {
msg_err_seek = b"Error repositioning TIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_err_notiff: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_notiff() {
msg_err_notiff = b"Not a TIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_err_bigtiff: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_bigtiff() {
msg_err_bigtiff = b"Unsupported BigTIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_err_invalid: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_invalid() {
msg_err_invalid = b"Invalid TIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_err_range: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_range() {
msg_err_range = b"Value out of the supported range in TIFF file\x00" as *const u8 as
        *const std::os::raw::c_char;}//;
static mut msg_err_unsupported: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_err_unsupported() {
msg_err_unsupported = b"Unsupported data in TIFF file\x00" as *const u8 as *const std::os::raw::c_char;}//;
static mut msg_warn_metadata: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_warn_metadata() {
msg_warn_metadata = b"Unrecognized metadata in TIFF file\x00" as *const u8 as
        *const std::os::raw::c_char;}//;
static mut msg_warn_multiple: * const std::os::raw::c_char =
    (0 as * const std::os::raw::c_char); unsafe fn laertes_init_msg_warn_multiple() {
msg_warn_multiple = b"Selected first image from multi-image TIFF file\x00" as *const u8 as
        *const std::os::raw::c_char;}//;
/*
 * Memory reader.
 */
unsafe extern "C" fn get_ushort_m(mut buf_ptr: * const std::os::raw::c_uchar)
 -> std::os::raw::c_uint {
    return ((*buf_ptr.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_uint) <<
                8 as
                    std::os::raw::c_int).wrapping_add(*buf_ptr.offset(1 as std::os::raw::c_int
                                                                  as isize) as
                                                  std::os::raw::c_uint);
}
/*
 * Memory reader.
 */
unsafe extern "C" fn get_ushort_i(mut buf_ptr: * const std::os::raw::c_uchar)
 -> std::os::raw::c_uint {
    return (*buf_ptr.offset(0 as std::os::raw::c_int as isize) as
                std::os::raw::c_uint).wrapping_add((*buf_ptr.offset(1 as std::os::raw::c_int
                                                                as isize) as
                                                std::os::raw::c_uint) <<
                                               8 as std::os::raw::c_int);
}
/*
 * Memory reader.
 */
unsafe extern "C" fn get_ulong_m(mut buf_ptr: * const std::os::raw::c_uchar)
 -> std::os::raw::c_ulong {
    return ((*buf_ptr.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_ulong) <<
                24 as
                    std::os::raw::c_int).wrapping_add((*buf_ptr.offset(1 as
                                                                   std::os::raw::c_int
                                                                   as isize)
                                                   as std::os::raw::c_ulong) <<
                                                  16 as
                                                      std::os::raw::c_int).wrapping_add((*buf_ptr.offset(2
                                                                                                     as
                                                                                                     std::os::raw::c_int
                                                                                                     as
                                                                                                     isize)
                                                                                     as
                                                                                     std::os::raw::c_ulong)
                                                                                    <<
                                                                                    8
                                                                                        as
                                                                                        std::os::raw::c_int).wrapping_add(*buf_ptr.offset(3
                                                                                                                                      as
                                                                                                                                      std::os::raw::c_int
                                                                                                                                      as
                                                                                                                                      isize)
                                                                                                                      as
                                                                                                                      std::os::raw::c_ulong);
}
/*
 * Memory reader.
 */
unsafe extern "C" fn get_ulong_i(mut buf_ptr: * const std::os::raw::c_uchar)
 -> std::os::raw::c_ulong {
    return (*buf_ptr.offset(0 as std::os::raw::c_int as isize) as
                std::os::raw::c_ulong).wrapping_add((*buf_ptr.offset(1 as std::os::raw::c_int
                                                                 as isize) as
                                                 std::os::raw::c_ulong) <<
                                                8 as
                                                    std::os::raw::c_int).wrapping_add((*buf_ptr.offset(2
                                                                                                   as
                                                                                                   std::os::raw::c_int
                                                                                                   as
                                                                                                   isize)
                                                                                   as
                                                                                   std::os::raw::c_ulong)
                                                                                  <<
                                                                                  16
                                                                                      as
                                                                                      std::os::raw::c_int).wrapping_add((*buf_ptr.offset(3
                                                                                                                                     as
                                                                                                                                     std::os::raw::c_int
                                                                                                                                     as
                                                                                                                                     isize)
                                                                                                                     as
                                                                                                                     std::os::raw::c_ulong)
                                                                                                                    <<
                                                                                                                    24
                                                                                                                        as
                                                                                                                        std::os::raw::c_int);
}
/*
 * Memory reader.
 */
unsafe extern "C" fn get_ulong_value<'a1>(mut getter_ptr: Option<&'a1 crate::src::minitiff::tiffread::minitiff_getter>,
                                     mut tag_type: std::os::raw::c_int,
                                     mut buf_ptr: * const std::os::raw::c_uchar)
 -> std::os::raw::c_ulong {
    match tag_type {
        1 => {
            return *buf_ptr.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_ulong
            /* error */
        }
        3 => {
            return (*((getter_ptr).clone()).unwrap()).get_ushort.expect("non-null function pointer")(buf_ptr)
                       as std::os::raw::c_ulong
        }
        4 => {
            return (*((getter_ptr).clone()).unwrap()).get_ulong.expect("non-null function pointer")(buf_ptr)
        }
        _ => { return -(1 as std::os::raw::c_long) as std::os::raw::c_ulong }
    };
}
/*
 * Memory allocator.
 */
unsafe extern "C" fn alloc_ulong_array<'a1>(mut info_ptr: Option<&'a1 mut crate::src::minitiff::tiffread::minitiff_info>,
                                       mut count: std::os::raw::c_ulong)
 -> * mut std::os::raw::c_ulong {
    let mut result: * mut u64 = 0 as *mut std::os::raw::c_ulong;
    if count >
           (-(1 as std::os::raw::c_int) as
                size_t).wrapping_div(::std::mem::size_of::<std::os::raw::c_ulong>() as
                                         std::os::raw::c_ulong) {
        minitiff_error(borrow(& info_ptr), msg_err_range);
    }
    result =
        malloc(count.wrapping_mul(::std::mem::size_of::<std::os::raw::c_ulong>() as
                                      std::os::raw::c_ulong)) as *mut std::os::raw::c_ulong;
    if result.is_null() { minitiff_error(borrow(& info_ptr), msg_err_alloc); }
    return result;
}
/*
 * Type-casting utilities.
 */
unsafe extern "C" fn cast_ulong_to_uint<'a1>(mut info_ptr: Option<&'a1 mut crate::src::minitiff::tiffread::minitiff_info>,
                                        mut value: std::os::raw::c_ulong)
 -> std::os::raw::c_uint {
    let mut result: u32 = value as std::os::raw::c_uint;
    if result as std::os::raw::c_ulong != value {
        minitiff_error(borrow(& info_ptr), msg_err_range);
    }
    return result;
}
/*
 * File reader.
 */
unsafe extern "C" fn read_ulong_values<'a1>(mut getter_ptr: Option<&'a1 crate::src::minitiff::tiffread::minitiff_getter>,
                                       mut tag_type: std::os::raw::c_int,
                                       mut values: * mut std::os::raw::c_ulong,
                                       mut count: std::os::raw::c_ulong,
                                       mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong {
    let mut buf: [u8; 4] = [0; 4];
    let mut value_size: u64 = 0;
    let mut i: u64 = 0;
    match tag_type {
        1 => {
            value_size = 1 as std::os::raw::c_int as size_t
            /* read nothing */
        }
        3 => { value_size = 2 as std::os::raw::c_int as size_t }
        4 => { value_size = 4 as std::os::raw::c_int as size_t }
        _ => { return 0 as std::os::raw::c_int as size_t }
    }
    i = 0 as std::os::raw::c_int as size_t;
    while i < count {
        if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void, value_size,
                 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
            break ;
        }
        *values.offset(i as isize) =
            get_ulong_value((getter_ptr).clone(), tag_type, buf.as_mut_ptr());
        i = i.wrapping_add(1)
    }
    return i;
}
/*
 * File seeker.
 */
unsafe extern "C" fn seek_to_offset<'a1>(mut info_ptr: Option<&'a1 mut crate::src::minitiff::tiffread::minitiff_info>,
                                    mut offset: std::os::raw::c_long,
                                    mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    if offset < 0 as std::os::raw::c_int as std::os::raw::c_long {
        minitiff_error(borrow(& info_ptr), msg_err_range);
    }
    if ftell(stream) == offset { return }
    if fseek(stream, offset, 0 as std::os::raw::c_int) != 0 as std::os::raw::c_int {
        minitiff_error(borrow(& info_ptr), msg_err_seek);
    };
}
/*
 * TIFF structure reader.
 */
#[no_mangle]
pub unsafe extern "C" fn minitiff_read_info<'a1>(mut info_ptr: Option<&'a1 mut crate::src::minitiff::tiffread::minitiff_info>,
                                            mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut current_block: u64;
    let mut getter: crate::src::minitiff::tiffread::minitiff_getter =
        minitiff_getter{get_ushort: None, get_ulong: None,};
    let mut buf: [u8; 12] = [0; 12];
    let mut vbuf: * mut u8 =
        buf.as_mut_ptr().offset(8 as std::os::raw::c_int as isize);
    let mut ulvals: [u64; 4] = [0; 4];
    let mut ulval: u64 = 0;
    let mut dir_offset: i64 = 0;
    let mut dir_size: u32 = 0;
    let mut i: u32 = 0;
    let mut tag_id: u32 = 0;
    let mut tag_type: u32 = 0;
    let mut count: u64 = 0;
    let mut bits_per_sample_count: u64 = 0;
    let mut bits_per_sample_tag_type: u32 = 0;
    let mut strip_offsets_tag_type: u32 = 0;
    let mut bits_per_sample_offset: i64 = 0;
    let mut strip_offsets_offset: i64 = 0;
    let mut unknown_metadata_found: i32 = 0;
    /* Read the TIFF header. */
    if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
             8 as std::os::raw::c_int as std::os::raw::c_ulong,
             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        current_block = 4819465328755230522;
    } else {
        if memcmp(buf.as_mut_ptr() as *const std::os::raw::c_void,
                  minitiff_sig_m.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int {
            (*(borrow_mut(&mut info_ptr)).unwrap()).byte_order = 'M' as i32;
            getter.get_ushort =
                Some(get_ushort_m);
            getter.get_ulong =
                Some(get_ulong_m)
        } else if memcmp(buf.as_mut_ptr() as *const std::os::raw::c_void,
                         minitiff_sig_i.as_ptr() as *const std::os::raw::c_void,
                         4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
            (*(borrow_mut(&mut info_ptr)).unwrap()).byte_order = 'I' as i32;
            getter.get_ushort =
                Some(get_ushort_i);
            getter.get_ulong =
                Some(get_ulong_i)
        } else if memcmp(buf.as_mut_ptr() as *const std::os::raw::c_void,
                         minitiff_sig_bigm.as_ptr() as *const std::os::raw::c_void,
                         4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int ||
                      memcmp(buf.as_mut_ptr() as *const std::os::raw::c_void,
                             minitiff_sig_bigi.as_ptr() as
                                 *const std::os::raw::c_void,
                             4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                          0 as std::os::raw::c_int {
            minitiff_error(borrow(& info_ptr), msg_err_bigtiff);
            return
        } else { minitiff_error(borrow(& info_ptr), msg_err_notiff); return }
        bits_per_sample_count = 0 as std::os::raw::c_int as size_t;
        strip_offsets_tag_type = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        bits_per_sample_tag_type = strip_offsets_tag_type;
        strip_offsets_offset = 0 as std::os::raw::c_int as std::os::raw::c_long;
        bits_per_sample_offset = strip_offsets_offset;
        dir_offset =
            getter.get_ulong.expect("non-null function pointer")(buf.as_mut_ptr().offset(4
                                                                                             as
                                                                                             std::os::raw::c_int
                                                                                             as
                                                                                             isize))
                as std::os::raw::c_long;
        if dir_offset >= 0 as std::os::raw::c_int as std::os::raw::c_long &&
               dir_offset < 8 as std::os::raw::c_int as std::os::raw::c_long {
            current_block = 11132740122879230165;
        } else {
            seek_to_offset(borrow_mut(&mut info_ptr), dir_offset, stream);
            /* Read the TIFF directory. */
            if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
                     2 as std::os::raw::c_int as std::os::raw::c_ulong,
                     1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                   1 as std::os::raw::c_int as std::os::raw::c_ulong {
                current_block = 4819465328755230522;
            } else {
                dir_size =
                    getter.get_ushort.expect("non-null function pointer")(buf.as_mut_ptr());
                unknown_metadata_found = 0 as std::os::raw::c_int;
                i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                loop  {
                    if !(i < dir_size) {
                        current_block = 18137396335907573669;
                        break ;
                    }
                    if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
                             12 as std::os::raw::c_int as std::os::raw::c_ulong,
                             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                           1 as std::os::raw::c_int as std::os::raw::c_ulong {
                        current_block = 4819465328755230522;
                        break ;
                    }
                    tag_id =
                        getter.get_ushort.expect("non-null function pointer")(buf.as_mut_ptr());
                    tag_type =
                        getter.get_ushort.expect("non-null function pointer")(buf.as_mut_ptr().offset(2
                                                                                                          as
                                                                                                          std::os::raw::c_int
                                                                                                          as
                                                                                                          isize));
                    count =
                        cast_ulong_to_uint(borrow_mut(&mut info_ptr),
                                           getter.get_ulong.expect("non-null function pointer")(buf.as_mut_ptr().offset(4
                                                                                                                            as
                                                                                                                            std::os::raw::c_int
                                                                                                                            as
                                                                                                                            isize)))
                            as size_t;
                    if count == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        current_block = 3632591010228416394;
                        break ;
                    }
                    match tag_id {
                        254 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            if ulval != 0 as std::os::raw::c_int as std::os::raw::c_ulong &&
                                   ulval != 1 as std::os::raw::c_int as std::os::raw::c_ulong
                               {
                                current_block = 3632591010228416394;
                                break ;
                            }
                        }
                        256 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).width =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval) as size_t
                        }
                        257 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).height =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval) as size_t
                        }
                        258 => {
                            if count == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                ulval =
                                    get_ulong_value(Some(&mut getter),
                                                    tag_type as std::os::raw::c_int,
                                                    vbuf);
                                (*(borrow_mut(&mut info_ptr)).unwrap()).bits_per_sample =
                                    cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval)
                            } else {
                                bits_per_sample_count = count;
                                bits_per_sample_tag_type = tag_type;
                                bits_per_sample_offset =
                                    getter.get_ulong.expect("non-null function pointer")(vbuf)
                                        as std::os::raw::c_long
                            }
                        }
                        259 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).compression =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval)
                        }
                        262 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).photometric =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval)
                        }
                        273 => {
                            (*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets_count = count;
                            if count == 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                if !(*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets.is_null() {
                                    current_block = 11132740122879230165;
                                    break ;
                                }
                                (*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets =
                                    alloc_ulong_array(borrow_mut(&mut info_ptr),
                                                      1 as std::os::raw::c_int as
                                                          size_t);
                                *(*(borrow(& info_ptr)).unwrap()).strip_offsets.offset(0 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      isize) =
                                    get_ulong_value(Some(&mut getter),
                                                    tag_type as std::os::raw::c_int,
                                                    vbuf)
                            } else {
                                strip_offsets_tag_type = tag_type;
                                strip_offsets_offset =
                                    getter.get_ulong.expect("non-null function pointer")(vbuf)
                                        as std::os::raw::c_long
                            }
                        }
                        274 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).orientation =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval)
                        }
                        277 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).samples_per_pixel =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval)
                        }
                        278 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                            ulval =
                                get_ulong_value(Some(&mut getter),
                                                tag_type as std::os::raw::c_int,
                                                vbuf);
                            (*(borrow_mut(&mut info_ptr)).unwrap()).rows_per_strip =
                                cast_ulong_to_uint(borrow_mut(&mut info_ptr), ulval) as size_t
                        }
                        284 | 317 => {
                            if count != 1 as std::os::raw::c_int as std::os::raw::c_ulong ||
                                   get_ulong_value(Some(&mut getter),
                                                   tag_type as std::os::raw::c_int,
                                                   vbuf) !=
                                       1 as std::os::raw::c_int as std::os::raw::c_ulong {
                                current_block = 3632591010228416394;
                                break ;
                            }
                        }
                        700 | 33723 | 34665 | 34675 | 34853 | 40965 | 50341 =>
                        {
                            if unknown_metadata_found == 0 {
                                unknown_metadata_found = 1 as std::os::raw::c_int;
                                minitiff_warning(borrow(& info_ptr), msg_warn_metadata);
                            }
                        }
                        279 | _ => { }
                    }
                    /* ignored for uncompressed images */
                    i = i.wrapping_add(1)
                }
                match current_block {
                    4819465328755230522 => { }
                    3632591010228416394 => { }
                    11132740122879230165 => { }
                    _ =>
                    /* Is this the last TIFF directory? */
                    {
                        if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
                                 4 as std::os::raw::c_int as std::os::raw::c_ulong,
                                 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
                               1 as std::os::raw::c_int as std::os::raw::c_ulong {
                            current_block = 4819465328755230522;
                        } else {
                            if getter.get_ulong.expect("non-null function pointer")(buf.as_mut_ptr())
                                   != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                                minitiff_warning(borrow(& info_ptr), msg_warn_multiple);
                            }
                            /* Finish up the incomplete readings. */
                            if bits_per_sample_offset !=
                                   0 as std::os::raw::c_int as std::os::raw::c_long {
                                count = bits_per_sample_count;
                                if count !=
                                       (*(borrow(& info_ptr)).unwrap()).samples_per_pixel as
                                           std::os::raw::c_ulong {
                                    current_block = 11132740122879230165;
                                } else if count >
                                              4 as std::os::raw::c_int as
                                                  std::os::raw::c_ulong {
                                    current_block = 3632591010228416394;
                                } else {
                                    seek_to_offset(borrow_mut(&mut info_ptr),
                                                   bits_per_sample_offset,
                                                   stream);
                                    if read_ulong_values(Some(&mut getter),
                                                         bits_per_sample_tag_type
                                                             as std::os::raw::c_int,
                                                         ulvals.as_mut_ptr(),
                                                         count, stream) !=
                                           count {
                                        current_block = 4819465328755230522;
                                    } else {
                                        loop  {
                                            count = count.wrapping_sub(1);
                                            if !(count >
                                                     0 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) {
                                                current_block =
                                                    8140372313878014523;
                                                break ;
                                            }
                                            if ulvals[0 as std::os::raw::c_int as
                                                          usize] !=
                                                   ulvals[count as usize] {
                                                current_block =
                                                    3632591010228416394;
                                                break ;
                                            }
                                        }
                                        match current_block {
                                            3632591010228416394 => { }
                                            _ => {
                                                (*(borrow_mut(&mut info_ptr)).unwrap()).bits_per_sample =
                                                    cast_ulong_to_uint(borrow_mut(&mut info_ptr),
                                                                       ulvals[0
                                                                                  as
                                                                                  std::os::raw::c_int
                                                                                  as
                                                                                  usize]);
                                                current_block =
                                                    11064061988481400464;
                                            }
                                        }
                                    }
                                }
                            } else { current_block = 11064061988481400464; }
                            match current_block {
                                3632591010228416394 => { }
                                4819465328755230522 => { }
                                11132740122879230165 => { }
                                _ => {
                                    if strip_offsets_offset !=
                                           0 as std::os::raw::c_int as std::os::raw::c_long {
                                        count =
                                            (*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets_count;
                                        if count ==
                                               0 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong ||
                                               count > (*(borrow(& info_ptr)).unwrap()).height {
                                            current_block =
                                                11132740122879230165;
                                        } else if !(*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets.is_null()
                                         {
                                            current_block =
                                                11132740122879230165;
                                        } else {
                                            (*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets =
                                                alloc_ulong_array(borrow_mut(&mut info_ptr),
                                                                  count);
                                            seek_to_offset(borrow_mut(&mut info_ptr),
                                                           strip_offsets_offset,
                                                           stream);
                                            if read_ulong_values(Some(&mut getter),
                                                                 strip_offsets_tag_type
                                                                     as
                                                                     std::os::raw::c_int,
                                                                 (*(borrow_mut(&mut info_ptr)).unwrap()).strip_offsets,
                                                                 count,
                                                                 stream) !=
                                                   count {
                                                current_block =
                                                    4819465328755230522;
                                            } else {
                                                current_block =
                                                    16778110326724371720;
                                            }
                                        }
                                    } else {
                                        current_block = 16778110326724371720;
                                    }
                                    match current_block {
                                        4819465328755230522 => { }
                                        11132740122879230165 => { }
                                        _ => {
                                            /* Return successfully. */
                                            return
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        4819465328755230522 =>
        /* Quick and dirty goto labels. */
        {
            minitiff_error(borrow(& info_ptr), msg_err_read);
            current_block = 11132740122879230165;
        }
        _ => { }
    }
    match current_block {
        11132740122879230165 => { minitiff_error(borrow(& info_ptr), msg_err_invalid); }
        _ => { }
    }
    minitiff_error(borrow(& info_ptr), msg_err_unsupported);
}
/*
 * TIFF row reader.
 */
#[no_mangle]
pub unsafe extern "C" fn minitiff_read_row<'a1>(mut info_ptr: Option<&'a1 mut crate::src::minitiff::tiffread::minitiff_info>,
                                           mut row_ptr: * mut std::os::raw::c_uchar,
                                           mut row_index: std::os::raw::c_ulong,
                                           mut stream: * mut crate::src::gifread::gifread::_IO_FILE) {
    let mut current_block: u64;
    let mut row_size: u64 = 0;
    let mut strip_index: u64 = 0;
    let mut bytes_per_sample: u32 = 0;
    let mut sample_max: u32 = 0;
    let mut offset: i64 = 0;
    let mut i: u64 = 0;
    /* Do not do validation here. */
    /* Call minitiff_validate_info() before calling this function. */
    bytes_per_sample =
        (*(borrow(& info_ptr)).unwrap()).bits_per_sample.wrapping_add(7 as std::os::raw::c_int as
                                                     std::os::raw::c_uint).wrapping_div(8
                                                                                    as
                                                                                    std::os::raw::c_int
                                                                                    as
                                                                                    std::os::raw::c_uint);
    row_size =
        (*(borrow(& info_ptr)).unwrap()).width.wrapping_mul((*(borrow(& info_ptr)).unwrap()).samples_per_pixel as
                                           std::os::raw::c_ulong).wrapping_mul(bytes_per_sample
                                                                           as
                                                                           std::os::raw::c_ulong);
    /* Position the file pointer to the beginning of the row,
     * if that has not been done already.
     */
    strip_index = row_index.wrapping_div((*(borrow(& info_ptr)).unwrap()).rows_per_strip);
    if strip_index >= (*(borrow(& info_ptr)).unwrap()).strip_offsets_count {
        current_block = 2230613979331437184;
    } else if (*(*(borrow(& info_ptr)).unwrap()).strip_offsets.offset(strip_index as isize) as
                   std::os::raw::c_long) < 0 as std::os::raw::c_int as std::os::raw::c_long {
        current_block = 4588818381929182025;
    } else {
        offset =
            (*(*(borrow(& info_ptr)).unwrap()).strip_offsets.offset(strip_index as
                                                   isize)).wrapping_add(row_size.wrapping_mul(row_index.wrapping_rem((*(borrow(& info_ptr)).unwrap()).rows_per_strip)))
                as std::os::raw::c_long;
        seek_to_offset(borrow_mut(&mut info_ptr), offset, stream);
        /* Read the row, and do all the necessary adjustments. */
        if fread(row_ptr as *mut std::os::raw::c_void, row_size,
                 1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
               1 as std::os::raw::c_int as std::os::raw::c_ulong {
            /* Quick and dirty goto labels. */
            minitiff_error(borrow(& info_ptr), msg_err_read);
            current_block = 2230613979331437184;
        } else {
            if (*(borrow(& info_ptr)).unwrap()).photometric == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* White is zero. */
                if bytes_per_sample > 1 as std::os::raw::c_int as std::os::raw::c_uint {
                    current_block = 14062777679669091991;
                } else {
                    sample_max =
                        (((1 as std::os::raw::c_int) << (*(borrow(& info_ptr)).unwrap()).bits_per_sample) -
                             1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    i = 0 as std::os::raw::c_int as size_t;
                    while i < row_size {
                        *row_ptr.offset(i as isize) =
                            sample_max.wrapping_sub(*row_ptr.offset(i as
                                                                        isize)
                                                        as std::os::raw::c_uint) as
                                std::os::raw::c_uchar;
                        i = i.wrapping_add(1)
                    }
                    current_block = 12039483399334584727;
                }
            } else { current_block = 12039483399334584727; }
            match current_block {
                14062777679669091991 => { }
                _ => {
                    /* Return successfully. */
                    return
                }
            }
        }
    }
    match current_block {
        2230613979331437184 => {
            minitiff_error(borrow(& info_ptr), msg_err_invalid);
            current_block = 4588818381929182025;
        }
        _ => { }
    }
    match current_block {
        4588818381929182025 => { minitiff_error(borrow(& info_ptr), msg_err_range); }
        _ => { }
    }
    minitiff_error(borrow(& info_ptr), msg_err_unsupported);
}
use crate::laertes_rt::*;
