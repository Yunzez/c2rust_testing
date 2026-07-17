
extern "C" {
    
    
    pub type _IO_marker;
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn getc(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn fseek(__stream: * mut crate::src::gifread::gifread::_IO_FILE, __off: std::os::raw::c_long, __whence: std::os::raw::c_int)
     -> std::os::raw::c_int;
    #[no_mangle]
    fn fgetpos(__stream: * mut crate::src::gifread::gifread::_IO_FILE, __pos: * mut crate::src::optipng::ioutil::_G_fpos_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn fsetpos(__stream: * mut crate::src::gifread::gifread::_IO_FILE, __pos: * const crate::src::optipng::ioutil::_G_fpos_t) -> std::os::raw::c_int;
    #[no_mangle]
    fn memcmp(_: * const core::ffi::c_void, _: * const core::ffi::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::png::png_get_io_ptr;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngread::png_read_png;
pub use crate::src::pngxtern::pngxrbmp::pngx_read_bmp;
pub use crate::src::pngxtern::pngxrbmp::pngx_sig_is_bmp;
pub use crate::src::pngxtern::pngxrgif::pngx_read_gif;
pub use crate::src::pngxtern::pngxrgif::pngx_sig_is_gif;
pub use crate::src::pngxtern::pngxrjpg::pngx_read_jpeg;
pub use crate::src::pngxtern::pngxrjpg::pngx_sig_is_jpeg;
pub use crate::src::pngxtern::pngxrpnm::pngx_read_pnm;
pub use crate::src::pngxtern::pngxrpnm::pngx_sig_is_pnm;
pub use crate::src::pngxtern::pngxrtif::pngx_read_tiff;
pub use crate::src::pngxtern::pngxrtif::pngx_sig_is_tiff;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type __mbstate_t = crate::src::optipng::ioutil::__mbstate_t;
// #[derive(Copy, Clone)]

pub type C2RustUnnamed = crate::src::optipng::ioutil::C2RustUnnamed;
// #[derive(Copy, Clone)]

pub type _G_fpos_t = crate::src::optipng::ioutil::_G_fpos_t;
pub type __fpos_t = crate::src::optipng::ioutil::_G_fpos_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
pub type fpos_t = crate::src::optipng::ioutil::_G_fpos_t;
pub type png_byte = std::os::raw::c_uchar;
pub type png_voidp = * mut core::ffi::c_void;
pub type png_bytep = * mut std::os::raw::c_uchar;
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_const_charpp = * mut * const std::os::raw::c_char;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
/*
 * pngxread.c - libpng external I/O: read utility functions.
 * Copyright (C) 2003-2011 Cosmin Truta.
 */
unsafe extern "C" fn pngx_sig_is_png<'a1, 'a2>(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                     mut sig: * mut std::os::raw::c_uchar, mut sig_size: std::os::raw::c_ulong,
                                     mut fmt_name_ptr: Option<&'a1 mut * const std::os::raw::c_char>,
                                     mut fmt_long_name_ptr: Option<&'a2 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_int {
    /* The signature of this function differs from the other pngx_sig_is_X()
    * functions, to allow extra functionality (e.g. customized error messages)
    * without requiring a full pngx_read_png().
    */
    static mut pngx_png_standalone_fmt_name: [std::os::raw::c_char; 4] =
        [0,0,0,0,]; unsafe fn laertes_init_pngx_png_standalone_fmt_name() {
pngx_png_standalone_fmt_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 4], &'_ [i8; 4]>(b"PNG\x00")
        };}//;
    static mut pngx_png_datastream_fmt_name: [std::os::raw::c_char; 15] =
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_pngx_png_datastream_fmt_name() {
pngx_png_datastream_fmt_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 15], &'_ [i8; 15]>(b"PNG datastream\x00")
        };}//;
    static mut pngx_png_standalone_fmt_long_name: [std::os::raw::c_char; 26] =
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_pngx_png_standalone_fmt_long_name() {
pngx_png_standalone_fmt_long_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 26], &'_ [i8; 26]>(b"Portable Network Graphics\x00")
        };}//;
    static mut pngx_png_datastream_fmt_long_name: [std::os::raw::c_char; 46] =
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_pngx_png_datastream_fmt_long_name() {
pngx_png_datastream_fmt_long_name = unsafe {
            *core::intrinsics::transmute::<&'_ [u8; 46], &'_ [i8; 46]>(b"Portable Network Graphics embedded datastream\x00")
        };}//;
    static mut png_file_sig: [std::os::raw::c_uchar; 8] =
        [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_png_file_sig() {
png_file_sig = [137 as std::os::raw::c_int as png_byte, 80 as std::os::raw::c_int as png_byte,
         78 as std::os::raw::c_int as png_byte, 71 as std::os::raw::c_int as png_byte,
         13 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte,
         26 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte];}//;
    static mut mng_file_sig: [std::os::raw::c_uchar; 8] =
        [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_mng_file_sig() {
mng_file_sig = [138 as std::os::raw::c_int as png_byte, 77 as std::os::raw::c_int as png_byte,
         78 as std::os::raw::c_int as png_byte, 71 as std::os::raw::c_int as png_byte,
         13 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte,
         26 as std::os::raw::c_int as png_byte, 10 as std::os::raw::c_int as png_byte];}//;
    static mut png_ihdr_sig: [std::os::raw::c_uchar; 8] =
        [0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_png_ihdr_sig() {
png_ihdr_sig = [0 as std::os::raw::c_int as png_byte, 0 as std::os::raw::c_int as png_byte,
         0 as std::os::raw::c_int as png_byte, 13 as std::os::raw::c_int as png_byte,
         73 as std::os::raw::c_int as png_byte, 72 as std::os::raw::c_int as png_byte,
         68 as std::os::raw::c_int as png_byte, 82 as std::os::raw::c_int as png_byte];}//;
    let mut has_png_sig: i32 = 0;
    /* Since png_read_png() fails rather abruptly with png_error(),
    * spend a little more effort to ensure that the format is indeed PNG.
    * Among other things, look for the presence of IHDR.
    */
    if sig_size <= (25 as std::os::raw::c_int + 18 as std::os::raw::c_int) as std::os::raw::c_ulong {
        /* size of (IHDR + IDAT) > (12+13) + (12+6) */
        return -(1 as std::os::raw::c_int)
    }
    has_png_sig =
        (memcmp(sig as *const std::os::raw::c_void,
                png_file_sig.as_ptr() as *const std::os::raw::c_void,
                8 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int) as
            std::os::raw::c_int;
    if memcmp(sig.offset((if has_png_sig != 0 {
                              8 as std::os::raw::c_int
                          } else { 0 as std::os::raw::c_int }) as isize) as
                  *const std::os::raw::c_void,
              png_ihdr_sig.as_ptr() as *const std::os::raw::c_void,
              8 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 as std::os::raw::c_int {
        /* This is not valid PNG: get as much information as possible. */
        if memcmp(sig as *const std::os::raw::c_void,
                  png_file_sig.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) == 0 as std::os::raw::c_int &&
               (*sig.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                    10 as std::os::raw::c_int ||
                    *sig.offset(4 as std::os::raw::c_int as isize) as std::os::raw::c_int ==
                        13 as std::os::raw::c_int) {
            png_error(png_ptr as *const png_struct,
                      b"PNG file appears to be corrupted by text file conversions\x00"
                          as *const u8 as *const std::os::raw::c_char);
        } else if memcmp(sig as *const std::os::raw::c_void,
                         mng_file_sig.as_ptr() as *const std::os::raw::c_void,
                         8 as std::os::raw::c_int as std::os::raw::c_ulong) ==
                      0 as std::os::raw::c_int {
            png_error(png_ptr as *const png_struct,
                      b"MNG decoding is not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
        /* not PNG */
        return 0 as std::os::raw::c_int
    }
    /* JNG is handled by the pngxrjpg module. */
    /* Store the format name. */
    if !borrow(& fmt_name_ptr).is_none() {
        *(borrow_mut(&mut fmt_name_ptr)).unwrap() =
            if has_png_sig != 0 {
                pngx_png_standalone_fmt_name.as_ptr()
            } else { pngx_png_datastream_fmt_name.as_ptr() }
    }
    if !borrow(& fmt_long_name_ptr).is_none() {
        *(borrow_mut(&mut fmt_long_name_ptr)).unwrap() =
            if has_png_sig != 0 {
                pngx_png_standalone_fmt_long_name.as_ptr()
            } else { pngx_png_datastream_fmt_long_name.as_ptr() }
    }
    return 1 as std::os::raw::c_int;
    /* PNG, really! */
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_image<'a1, 'a2>(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                         mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                         mut fmt_name_ptr: Option<&'a1 mut * const std::os::raw::c_char>,
                                         mut fmt_long_name_ptr:
                                             Option<&'a2 mut * const std::os::raw::c_char>)
 -> std::os::raw::c_int {
    let mut sig: [u8; 128] = [0; 128];
    let mut num: u64 = 0;
    let mut read_fn:
            Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut crate::src::libpng::png::png_info_def,_: * mut crate::src::gifread::gifread::_IO_FILE,) -> i32> = None;
    let mut stream: * mut crate::src::gifread::gifread::_IO_FILE = 0 as *mut FILE;
    let mut fpos: crate::src::optipng::ioutil::_G_fpos_t =
        fpos_t{__pos: 0,
               __state:
                   __mbstate_t{__count: 0,
                               __value: C2RustUnnamed{__wch: 0,},},};
    let mut result: i32 = 0;
    /* Precondition. */
    /* Read the signature bytes. */
    stream = png_get_io_ptr(png_ptr as *const png_struct) as *mut FILE;
    if fgetpos(stream, &mut fpos) != 0 as std::os::raw::c_int {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t ftell in input file stream\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    num =
        fread(sig.as_mut_ptr() as *mut std::os::raw::c_void,
              1 as std::os::raw::c_int as std::os::raw::c_ulong,
              ::std::mem::size_of::<[png_byte; 128]>() as std::os::raw::c_ulong,
              stream);
    if fsetpos(stream, &mut fpos) != 0 as std::os::raw::c_int {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t fseek in input file stream\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Try the PNG format first. */
    if pngx_sig_is_png(png_ptr, sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr),
                       borrow_mut(&mut fmt_long_name_ptr)) > 0 as std::os::raw::c_int {
        png_read_png(png_ptr, info_ptr, 0 as std::os::raw::c_int,
                     (0 as * mut core::ffi::c_void));
        if getc(stream) != -(1 as std::os::raw::c_int) {
            png_warning(png_ptr as *const png_struct,
                        b"Extraneous data found after IEND\x00" as *const u8
                            as *const std::os::raw::c_char);
            fseek(stream, 0 as std::os::raw::c_int as std::os::raw::c_long, 2 as std::os::raw::c_int);
        }
        return 1 as std::os::raw::c_int
    }
    /* Check the signature bytes against other known image formats. */
    if pngx_sig_is_bmp(sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr), borrow_mut(&mut fmt_long_name_ptr))
           > 0 as std::os::raw::c_int {
        read_fn =
            Some(pngx_read_bmp)
    } else if pngx_sig_is_gif(sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr),
                              borrow_mut(&mut fmt_long_name_ptr)) > 0 as std::os::raw::c_int {
        read_fn =
            Some(pngx_read_gif)
    } else if pngx_sig_is_jpeg(sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr),
                               borrow_mut(&mut fmt_long_name_ptr)) > 0 as std::os::raw::c_int {
        read_fn =
            Some(pngx_read_jpeg)
    } else if pngx_sig_is_pnm(sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr),
                              borrow_mut(&mut fmt_long_name_ptr)) > 0 as std::os::raw::c_int {
        read_fn =
            Some(pngx_read_pnm)
    } else if pngx_sig_is_tiff(sig.as_mut_ptr(), num, borrow_mut(&mut fmt_name_ptr),
                               borrow_mut(&mut fmt_long_name_ptr)) > 0 as std::os::raw::c_int {
        read_fn =
            Some(pngx_read_tiff)
    } else { return 0 as std::os::raw::c_int } /* not a known image format */
    /* Read the image. */
    result =
        read_fn.expect("non-null function pointer")(png_ptr, info_ptr,
                                                    stream);
    /* Signature checking may give false positives; reading can still fail. */
    if result <= 0 as std::os::raw::c_int {
        /* this isn't the format we thought it was */
        if fsetpos(stream, &mut fpos) != 0 as std::os::raw::c_int {
            png_error(png_ptr as *const png_struct,
                      b"Can\'t fseek in input file stream\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
    }
    return result;
}
use crate::laertes_rt::*;
