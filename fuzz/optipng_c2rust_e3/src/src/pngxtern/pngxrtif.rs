
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    static minitiff_sig_m: [std::os::raw::c_char; 4];
    #[no_mangle]
    static minitiff_sig_i: [std::os::raw::c_char; 4];
    #[no_mangle]
    fn memcmp(_: *const std::os::raw::c_void, _: *const std::os::raw::c_void,
              _: std::os::raw::c_ulong) -> std::os::raw::c_int;
    #[no_mangle]
    fn strstr(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::minitiff::tiffread::minitiff_read_info;
pub use crate::src::minitiff::tiffread::minitiff_read_row;
pub use crate::src::minitiff::tiffutil::minitiff_destroy_info;
pub use crate::src::minitiff::tiffutil::minitiff_init_info;
pub use crate::src::minitiff::tiffutil::minitiff_validate_info;
pub use crate::src::pngxtern::pngxmem::pngx_malloc_rows;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
pub type png_const_charpp = crate::src::optipng::optim::png_const_charpp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infop = crate::src::libpng::png::png_infop;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
// #[derive(Copy, Clone)]

pub type minitiff_info = crate::src::minitiff::tiffread::minitiff_info;
/*
 * pngxrtif.c - libpng external I/O: TIFF reader.
 * Copyright (C) 2006-2012 Cosmin Truta.
 */
/*
 * The TIFF format name.
 */
static mut tiff_fmt_name: [std::os::raw::c_char; 5] =
    unsafe {
        *::std::mem::transmute::<&[u8; 5], &[std::os::raw::c_char; 5]>(b"TIFF\x00")
    };
static mut tiff_fmt_long_name: [std::os::raw::c_char; 25] =
    unsafe {
        *::std::mem::transmute::<&[u8; 25],
                                 &[std::os::raw::c_char; 25]>(b"Tagged Image File Format\x00")
    };
#[no_mangle]
pub unsafe extern "C" fn pngx_sig_is_tiff(mut sig: png_bytep,
                                          mut sig_size: size_t,
                                          mut fmt_name_ptr: png_const_charpp,
                                          mut fmt_long_name_ptr:
                                              png_const_charpp)
 -> std::os::raw::c_int {
    /* Require at least the TIFF signature. */
    if sig_size < 8 as std::os::raw::c_int as std::os::raw::c_ulong {
        return -(1 as std::os::raw::c_int)
    } /* insufficient data */
    if memcmp(sig as *const std::os::raw::c_void,
              minitiff_sig_m.as_ptr() as *const std::os::raw::c_void,
              4 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 as std::os::raw::c_int &&
           memcmp(sig as *const std::os::raw::c_void,
                  minitiff_sig_i.as_ptr() as *const std::os::raw::c_void,
                  4 as std::os::raw::c_int as std::os::raw::c_ulong) != 0 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int
    } /* not TIFF */
    /* Store the format name. */
    if !fmt_name_ptr.is_null() { *fmt_name_ptr = tiff_fmt_name.as_ptr() }
    if !fmt_long_name_ptr.is_null() {
        *fmt_long_name_ptr = tiff_fmt_long_name.as_ptr()
    }
    return 1 as std::os::raw::c_int;
    /* TIFF */
}
/* FIXME: Not thread-safe. */
static mut err_png_ptr: png_structp = 0 as *const png_struct as png_structp;
static mut num_extra_images: std::os::raw::c_uint = 0;
unsafe extern "C" fn pngx_tiff_error(mut msg: *const std::os::raw::c_char) {
    png_error(err_png_ptr as *const png_struct, msg);
}
unsafe extern "C" fn pngx_tiff_warning(mut msg: *const std::os::raw::c_char) {
    /* FIXME:
    * Inspection of warning messages is fragile, but is
    * required by the limitations of minitiff version 0.1.
    */
    if !strstr(msg,
               b"multi-image\x00" as *const u8 as
                   *const std::os::raw::c_char).is_null() {
        num_extra_images = num_extra_images.wrapping_add(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn pngx_read_tiff(mut png_ptr: png_structp,
                                        mut info_ptr: png_infop,
                                        mut stream: *mut FILE)
 -> std::os::raw::c_int {
    let mut tiff_info: minitiff_info =
        minitiff_info{error_handler: None,
                      warning_handler: None,
                      byte_order: 0,
                      width: 0,
                      height: 0,
                      bits_per_sample: 0,
                      compression: 0,
                      photometric: 0,
                      strip_offsets_count: 0,
                      strip_offsets: 0 as *mut std::os::raw::c_ulong,
                      orientation: 0,
                      samples_per_pixel: 0,
                      rows_per_strip: 0,};
    let mut width: std::os::raw::c_uint = 0;
    let mut height: std::os::raw::c_uint = 0;
    let mut pixel_size: std::os::raw::c_uint = 0;
    let mut sample_depth: std::os::raw::c_uint = 0;
    let mut sample_max: std::os::raw::c_uint = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut sample_overflow: std::os::raw::c_int = 0;
    let mut row_pointers: png_bytepp = 0 as *mut *mut png_byte;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut i: std::os::raw::c_uint = 0;
    let mut j: std::os::raw::c_uint = 0;
    let mut k: std::os::raw::c_uint = 0;
    err_png_ptr = png_ptr;
    num_extra_images = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    minitiff_init_info(&mut tiff_info);
    tiff_info.error_handler =
        Some(pngx_tiff_error as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ());
    tiff_info.warning_handler =
        Some(pngx_tiff_warning as
                 unsafe extern "C" fn(_: *const std::os::raw::c_char) -> ());
    minitiff_read_info(&mut tiff_info, stream);
    minitiff_validate_info(&mut tiff_info);
    width = tiff_info.width as std::os::raw::c_uint;
    height = tiff_info.height as std::os::raw::c_uint;
    pixel_size = tiff_info.samples_per_pixel;
    sample_depth = tiff_info.bits_per_sample;
    match pixel_size {
        1 => {
            color_type = 0 as std::os::raw::c_int
            /* avoid "uninitialized color_type" warning */
        }
        2 => { color_type = 4 as std::os::raw::c_int }
        3 => { color_type = 2 as std::os::raw::c_int }
        4 => { color_type = 2 as std::os::raw::c_int | 4 as std::os::raw::c_int }
        _ => {
            png_error(png_ptr as *const png_struct,
                      b"Unsupported TIFF color space\x00" as *const u8 as
                          *const std::os::raw::c_char);
            /* NOTREACHED */
            return 0 as std::os::raw::c_int
        }
    }
    if sample_depth > 16 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Unsupported TIFF sample depth\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    sample_max =
        (((1 as std::os::raw::c_int) << sample_depth) - 1 as std::os::raw::c_int) as
            std::os::raw::c_uint;
    sample_overflow = 0 as std::os::raw::c_int;
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 if sample_depth <= 8 as std::os::raw::c_int as std::os::raw::c_uint {
                     8 as std::os::raw::c_int
                 } else { 16 as std::os::raw::c_int }, color_type, 0 as std::os::raw::c_int,
                 0 as std::os::raw::c_int, 0 as std::os::raw::c_int);
    row_pointers = pngx_malloc_rows(png_ptr, info_ptr, 0 as std::os::raw::c_int);
    if sample_depth <= 8 as std::os::raw::c_int as std::os::raw::c_uint {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize);
            minitiff_read_row(&mut tiff_info, row, i as size_t, stream);
            if sample_depth < 8 as std::os::raw::c_int as std::os::raw::c_uint {
                j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b: std::os::raw::c_uint =
                        *row.offset(j as isize) as std::os::raw::c_uint;
                    if b > sample_max {
                        b = sample_max;
                        sample_overflow = 1 as std::os::raw::c_int
                    }
                    *row.offset(j as isize) =
                        b.wrapping_mul(255 as std::os::raw::c_int as
                                           std::os::raw::c_uint).wrapping_add(sample_max.wrapping_div(2
                                                                                                  as
                                                                                                  std::os::raw::c_int
                                                                                                  as
                                                                                                  std::os::raw::c_uint)).wrapping_div(sample_max)
                            as png_byte;
                    j = j.wrapping_add(1)
                }
            }
            if tiff_info.photometric == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                j = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                while j < pixel_size.wrapping_mul(width) {
                    *row.offset(j as isize) =
                        (255 as std::os::raw::c_int -
                             *row.offset(j as isize) as std::os::raw::c_int) as
                            png_byte;
                    j = j.wrapping_add(1)
                }
            }
            i = i.wrapping_add(1)
        }
    } else {
        i = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        while i < height {
            row = *row_pointers.offset(i as isize);
            minitiff_read_row(&mut tiff_info, row, i as size_t, stream);
            if tiff_info.byte_order == 'I' as i32 {
                /* "Intel" byte order => swap row bytes */
                k = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                j = k;
                while j < pixel_size.wrapping_mul(width) {
                    let mut b_0: png_byte = *row.offset(k as isize);
                    *row.offset(k as isize) =
                        *row.offset(k.wrapping_add(1 as std::os::raw::c_int as
                                                       std::os::raw::c_uint) as
                                        isize);
                    *row.offset(k.wrapping_add(1 as std::os::raw::c_int as
                                                   std::os::raw::c_uint) as isize) =
                        b_0;
                    j = j.wrapping_add(1);
                    k = k.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                }
            }
            if sample_depth < 16 as std::os::raw::c_int as std::os::raw::c_uint {
                k = 0 as std::os::raw::c_int as std::os::raw::c_uint;
                j = k;
                while k < pixel_size.wrapping_mul(width) {
                    let mut b_1: std::os::raw::c_uint =
                        (((*row.offset(k as isize) as std::os::raw::c_int) <<
                              8 as std::os::raw::c_int) +
                             *row.offset(k.wrapping_add(1 as std::os::raw::c_int as
                                                            std::os::raw::c_uint) as
                                             isize) as std::os::raw::c_int) as
                            std::os::raw::c_uint;
                    if b_1 > sample_max {
                        b_1 = sample_max;
                        sample_overflow = 1 as std::os::raw::c_int
                    }
                    b_1 =
                        b_1.wrapping_mul(65535 as
                                             std::os::raw::c_uint).wrapping_add(sample_max.wrapping_div(2
                                                                                                    as
                                                                                                    std::os::raw::c_int
                                                                                                    as
                                                                                                    std::os::raw::c_uint)).wrapping_div(sample_max);
                    *row.offset(k as isize) =
                        (b_1 >> 8 as std::os::raw::c_int) as png_byte;
                    *row.offset(k.wrapping_add(1 as std::os::raw::c_int as
                                                   std::os::raw::c_uint) as isize) =
                        (b_1 & 255 as std::os::raw::c_int as std::os::raw::c_uint) as
                            png_byte;
                    j = j.wrapping_add(1);
                    k = k.wrapping_add(2 as std::os::raw::c_int as std::os::raw::c_uint)
                }
            }
            i = i.wrapping_add(1)
        }
    }
    if sample_overflow != 0 {
        png_warning(png_ptr as *const png_struct,
                    b"Overflow in TIFF samples\x00" as *const u8 as
                        *const std::os::raw::c_char);
    }
    minitiff_destroy_info(&mut tiff_info);
    return (1 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_add(num_extra_images)
               as std::os::raw::c_int;
}
