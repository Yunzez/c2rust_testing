
extern "C" {
    
    
    
    
    
    
    
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
}
pub use crate::src::libpng::pngerror::png_error;
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
pub type png_byte = std::os::raw::c_uchar;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_const_charpp = * mut * const std::os::raw::c_char;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
static mut jpeg_sig_jp2: [std::os::raw::c_uchar; 12] =
    [0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_jpeg_sig_jp2() {
jpeg_sig_jp2 = [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
     0 as std::os::raw::c_int as png_byte, 0xc as std::os::raw::c_int as png_byte,
     0x6a as std::os::raw::c_int as png_byte, 0x50 as std::os::raw::c_int as png_byte,
     0x20 as std::os::raw::c_int as png_byte, 0x20 as std::os::raw::c_int as png_byte,
     0xd as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte,
     0x87 as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte];}//;
static mut jpeg_sig_jpc: [std::os::raw::c_uchar; 4] =
    [0,0,0,0,]; unsafe fn laertes_init_jpeg_sig_jpc() {
jpeg_sig_jpc = [0xff as std::os::raw::c_int as png_byte, 0x4f as std::os::raw::c_int as png_byte,
     0xff as std::os::raw::c_int as png_byte, 0x51 as std::os::raw::c_int as png_byte];}//;
static mut jpeg_sig_jng: [std::os::raw::c_uchar; 8] =
    [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_jpeg_sig_jng() {
jpeg_sig_jng = [0x8b as std::os::raw::c_int as png_byte, 0x4a as std::os::raw::c_int as png_byte,
     0x4e as std::os::raw::c_int as png_byte, 0x47 as std::os::raw::c_int as png_byte,
     0xd as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte,
     0x1a as std::os::raw::c_int as png_byte, 0xa as std::os::raw::c_int as png_byte];}//;
static mut jpeg_sig_jng_jhdr: [std::os::raw::c_uchar; 8] =
    [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_jpeg_sig_jng_jhdr() {
jpeg_sig_jng_jhdr = [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
     0 as std::os::raw::c_int as png_byte, 0x1a as std::os::raw::c_int as png_byte,
     0x4a as std::os::raw::c_int as png_byte, 0x48 as std::os::raw::c_int as png_byte,
     0x44 as std::os::raw::c_int as png_byte, 0x52 as std::os::raw::c_int as png_byte];}//;
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_jpeg<'a1, 'a2>(mut sig: * mut std::os::raw::c_uchar,
                                          mut sig_size: std::os::raw::c_ulong,
                                          mut fmt_name_ptr: Option<&'a1 mut * const std::os::raw::c_char>,
                                          mut fmt_long_name_ptr:
                                              Option<&'a2 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_int {
    let mut fmt: * const i8 =
        0 as *const std::os::raw::c_char; /* not JPEG */
    let mut marker: u32 = 0;
    let mut result: i32 = 0;
    if sig_size < 12 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    }
    if *sig.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
           0xff as std::os::raw::c_int &&
           *sig.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xd8 as std::os::raw::c_int &&
           *sig.offset(2 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
               0xff as std::os::raw::c_int {
        marker =
            0xff00 as std::os::raw::c_uint |
                *sig.offset(3 as std::os::raw::c_int as isize) as std::os::raw::c_uint;
        if marker >= 0xffc0 as std::os::raw::c_uint &&
               marker <= 0xffcf as std::os::raw::c_uint ||
               marker >= 0xffda as std::os::raw::c_uint &&
                   marker <= 0xfffe as std::os::raw::c_uint {
            fmt = b"JPEG\x00" as *const u8 as *const std::os::raw::c_char;
            result = 1 as std::os::raw::c_int
            /* not JPEG */
            /* JFIF, EXIF, JPEG-LS, codestream, etc. */
        } else { return 0 as std::os::raw::c_int }
    } else if memcmp(sig as *const std::os::raw::c_void,
                     jpeg_sig_jp2.as_ptr() as *const std::os::raw::c_void,
                     12 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int
                  ||
                  memcmp(sig as *const std::os::raw::c_void,
                         jpeg_sig_jpc.as_ptr() as *const std::os::raw::c_void,
                         4 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
        fmt = b"JPEG-2000\x00" as *const u8 as *const std::os::raw::c_char;
        result = 2 as std::os::raw::c_int
        /* .jp2 or JPEG-2000 codestream */
    } else if memcmp(sig as *const std::os::raw::c_void,
                     jpeg_sig_jng.as_ptr() as *const std::os::raw::c_void,
                     8 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int ||
                  memcmp(sig as *const std::os::raw::c_void,
                         jpeg_sig_jng_jhdr.as_ptr() as *const std::os::raw::c_void,
                         8 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
        fmt = b"JNG\x00" as *const u8 as *const std::os::raw::c_char;
        result = 3 as std::os::raw::c_int
        /* JNG, standalone or datastream */
    } else { return 0 as std::os::raw::c_int }
    /* Store the format name. */
    if !borrow(& fmt_name_ptr).is_none() { *(borrow_mut(&mut fmt_name_ptr)).unwrap() = fmt } /* not a JPEG file */
    if !borrow(& fmt_long_name_ptr).is_none() { *(borrow_mut(&mut fmt_long_name_ptr)).unwrap() = fmt }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_jpeg(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                        mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                        mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_int {
    let mut buf: [u8; 12] = [0; 12];
    let mut sig_code: i32 = 0;
    if fread(buf.as_mut_ptr() as *mut std::os::raw::c_void,
             12 as std::os::raw::c_int as std::os::raw::c_ulong,
             1 as std::os::raw::c_int as std::os::raw::c_ulong, stream) !=
           1 as std::os::raw::c_int as std::os::raw::c_ulong {
        return 0 as std::os::raw::c_int
    }
    sig_code =
        pngx_sig_is_jpeg(buf.as_mut_ptr(), 12 as std::os::raw::c_int as size_t,
                         Option::<&'_ mut * const i8>::None, Option::<&'_ mut * const i8>::None);
    /* TODO: Use the format names passed by pngx_sig_is_jpeg. */
    match sig_code {
        1 => {
            png_error(png_ptr as *const png_struct,
                      b"JPEG decoding is not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
        2 => {
            png_error(png_ptr as *const png_struct,
                      b"JPEG-2000 decoding is not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
        }
        3 => {
            png_error(png_ptr as *const png_struct,
                      b"JNG (JPEG) decoding is not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
        }
        _ => { }
    }
    if info_ptr.is_null() {
        /* dummy, keep compilers happy */
        return 0 as std::os::raw::c_int
    }
    return 0 as std::os::raw::c_int;
    /* always fail */
}
use crate::laertes_rt::*;
