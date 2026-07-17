
extern "C" {
    
    
    
}
pub use crate::src::libpng::png::png_check_IHDR;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = std::os::raw::c_ulong;
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
pub type png_const_charp = * const std::os::raw::c_char;
pub type png_bytepp = * mut * mut std::os::raw::c_uchar;
// #[derive(Copy, Clone)]

pub type png_struct_def = crate::src::libpng::png::png_struct_def;
pub type png_row_infop = * mut crate::src::libpng::png::png_row_info_struct;
pub type png_row_info = crate::src::libpng::png::png_row_info_struct;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::png::png_row_info_struct;
pub type uInt = std::os::raw::c_uint;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk_t;
// #[derive(Copy, Clone)]

pub type png_unknown_chunk_t = crate::src::libpng::png::png_unknown_chunk_t;
pub type png_write_status_ptr<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_read_status_ptr<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>;
pub type png_color_16 = crate::src::libpng::png::png_color_16_struct;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8_struct;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_flush_ptr<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,) -> ()>;
pub type png_colorp = * mut crate::src::libpng::png::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color_struct;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_compression_bufferp = * mut crate::src::libpng::png::png_compression_buffer;
// #[derive(Copy, Clone)]

pub type png_compression_buffer = crate::src::libpng::png::png_compression_buffer;
pub type z_stream = crate::src::libpng::png::z_stream_s;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::png::z_stream_s;
pub type uLong = std::os::raw::c_ulong;
pub type voidpf = * mut core::ffi::c_void;
pub type free_func<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
pub type alloc_func<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut core::ffi::c_void>,_: std::os::raw::c_uint,_: std::os::raw::c_uint,) -> Option<&'a2 mut core::ffi::c_void>>;
pub type Bytef = std::os::raw::c_uchar;
pub type Byte = std::os::raw::c_uchar;
pub type png_rw_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut std::os::raw::c_uchar>,_: std::os::raw::c_ulong,) -> ()>;
pub type png_error_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 std::os::raw::c_char>,) -> ()>;
pub type png_const_structp = * const crate::src::libpng::png::png_struct_def;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::png::png_info_def;
pub type png_unknown_chunkp = * mut crate::src::libpng::png::png_unknown_chunk_t;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_const_infop = * const crate::src::libpng::png::png_info_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_const_inforp = * const crate::src::libpng::png::png_info_def;
pub type png_color_16p = * mut crate::src::libpng::png::png_color_16_struct;
pub type png_color_8p = * mut crate::src::libpng::png::png_color_8_struct;
pub type png_unknown_chunkpp = * mut * mut crate::src::libpng::png::png_unknown_chunk_t;
/* pngget.c - retrieval of values from info struct
 *
 * Last changed in libpng 1.6.32 [August 24, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 */
#[no_mangle]
pub unsafe extern "C" fn png_get_valid(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * const crate::src::libpng::png::png_info_def,
                                       mut flag: std::os::raw::c_uint) -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).valid & flag
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_rowbytes(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                          mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_ulong {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).rowbytes
    }
    return 0 as std::os::raw::c_int as png_size_t;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_rows(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> * mut * mut std::os::raw::c_uchar {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).row_pointers
    }
    return 0 as png_bytepp;
}
/* Easy access to info, added in libpng-0.99 */
#[no_mangle]
pub unsafe extern "C" fn png_get_image_width(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                             mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() { return (*info_ptr).width }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_image_height(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                              mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() { return (*info_ptr).height }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_bit_depth(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                           mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_uchar {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).bit_depth
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_color_type(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                            mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_uchar {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).color_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_filter_type<'a1, 'a2>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                             mut info_ptr: Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uchar {
    if !(png_ptr).clone().is_none() && !(info_ptr).clone().is_none() {
        return (*((info_ptr).clone()).unwrap()).filter_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_interlace_type<'a1, 'a2>(mut png_ptr:
                                                    Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                mut info_ptr:
                                                    Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uchar {
    if !(png_ptr).clone().is_none() && !(info_ptr).clone().is_none() {
        return (*((info_ptr).clone()).unwrap()).interlace_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_compression_type<'a1, 'a2>(mut png_ptr:
                                                      Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                  mut info_ptr:
                                                      Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uchar {
    if !(png_ptr).clone().is_none() && !(info_ptr).clone().is_none() {
        return (*((info_ptr).clone()).unwrap()).compression_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub extern "C" fn png_get_x_pixels_per_meter<'a1, 'a2>(mut png_ptr:
                                                        Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                    mut info_ptr:
                                                        Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uint {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub extern "C" fn png_get_y_pixels_per_meter<'a1, 'a2>(mut png_ptr:
                                                        Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                    mut info_ptr:
                                                        Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uint {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub extern "C" fn png_get_pixels_per_meter<'a1, 'a2>(mut png_ptr:
                                                      Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                  mut info_ptr:
                                                      Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_uint {
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub extern "C" fn png_get_pixel_aspect_ratio_fixed<'a1, 'a2>(mut png_ptr:
                                                              Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                          mut info_ptr:
                                                              Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn png_get_x_offset_microns<'a1, 'a2>(mut png_ptr:
                                                      Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                  mut info_ptr:
                                                      Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn png_get_y_offset_microns<'a1, 'a2>(mut png_ptr:
                                                      Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                  mut info_ptr:
                                                      Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn png_get_x_offset_pixels<'a1, 'a2>(mut png_ptr:
                                                     Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                 mut info_ptr:
                                                     Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub extern "C" fn png_get_y_offset_pixels<'a1, 'a2>(mut png_ptr:
                                                     Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                                 mut info_ptr:
                                                     Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
/* INCH_CONVERSIONS */
/* png_get_channels really belongs in here, too, but it's been around longer */
/* EASY_ACCESS */
#[no_mangle]
pub unsafe extern "C" fn png_get_channels(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                          mut info_ptr: * const crate::src::libpng::png::png_info_def)
 -> std::os::raw::c_uchar {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).channels
    }
    return 0 as std::os::raw::c_int as png_byte;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_signature<'a1, 'a2>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                           mut info_ptr: Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> * const std::os::raw::c_uchar {
    if !(png_ptr).clone().is_none() && !(info_ptr).clone().is_none() {
        return (*((info_ptr).clone()).unwrap()).signature.as_ptr()
    }
    return 0 as png_const_bytep;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_bKGD<'a1, 'a2>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut background: Option<&'a1 mut Option<&'a2 mut crate::src::libpng::png::png_color_16_struct>>)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x20 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !borrow(& background).is_none() {
        *(borrow_mut(&mut background)).unwrap() = Some(&mut (*info_ptr).background);
        return 0x20 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_hIST<'a1>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut hist: Option<&'a1 mut * mut std::os::raw::c_ushort>)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x40 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !borrow(& hist).is_none() {
        *(borrow_mut(&mut hist)).unwrap() = (*info_ptr).hist;
        return 0x40 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_IHDR<'a1, 'a2, 'a3, 'a4, 'a5, 'a6, 'a7>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * const crate::src::libpng::png::png_info_def,
                                      mut width: Option<&'a1 mut std::os::raw::c_uint>,
                                      mut height: Option<&'a2 mut std::os::raw::c_uint>,
                                      mut bit_depth: Option<&'a3 mut std::os::raw::c_int>,
                                      mut color_type: Option<&'a4 mut std::os::raw::c_int>,
                                      mut interlace_type: Option<&'a5 mut std::os::raw::c_int>,
                                      mut compression_type: Option<&'a6 mut std::os::raw::c_int>,
                                      mut filter_type: Option<&'a7 mut std::os::raw::c_int>)
 -> std::os::raw::c_uint {
    if png_ptr.is_null() || info_ptr.is_null() {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if !borrow(& width).is_none() { *(borrow_mut(&mut width)).unwrap() = (*info_ptr).width }
    if !borrow(& height).is_none() { *(borrow_mut(&mut height)).unwrap() = (*info_ptr).height }
    if !borrow(& bit_depth).is_none() {
        *(borrow_mut(&mut bit_depth)).unwrap() = (*info_ptr).bit_depth as std::os::raw::c_int
    }
    if !borrow(& color_type).is_none() {
        *(borrow_mut(&mut color_type)).unwrap() = (*info_ptr).color_type as std::os::raw::c_int
    }
    if !borrow(& compression_type).is_none() {
        *(borrow_mut(&mut compression_type)).unwrap() = (*info_ptr).compression_type as std::os::raw::c_int
    }
    if !borrow(& filter_type).is_none() {
        *(borrow_mut(&mut filter_type)).unwrap() = (*info_ptr).filter_type as std::os::raw::c_int
    }
    if !borrow(& interlace_type).is_none() {
        *(borrow_mut(&mut interlace_type)).unwrap() = (*info_ptr).interlace_type as std::os::raw::c_int
    }
    /* This is redundant if we can be sure that the info_ptr values were all
    * assigned in png_set_IHDR().  We do the check anyhow in case an
    * application has ignored our advice not to mess with the members
    * of info_ptr directly.
    */
    png_check_IHDR(png_ptr, (*info_ptr).width, (*info_ptr).height,
                   (*info_ptr).bit_depth as std::os::raw::c_int,
                   (*info_ptr).color_type as std::os::raw::c_int,
                   (*info_ptr).interlace_type as std::os::raw::c_int,
                   (*info_ptr).compression_type as std::os::raw::c_int,
                   (*info_ptr).filter_type as std::os::raw::c_int);
    return 1 as std::os::raw::c_int as png_uint_32;
}
/* sCAL */
/* pHYs */
#[no_mangle]
pub unsafe extern "C" fn png_get_PLTE<'a1, 'a2>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut palette: Option<&'a1 mut * mut crate::src::libpng::png::png_color_struct>,
                                      mut num_palette: Option<&'a2 mut std::os::raw::c_int>)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x8 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !borrow(& palette).is_none() {
        *(borrow_mut(&mut palette)).unwrap() = (*info_ptr).palette;
        *(borrow_mut(&mut num_palette)).unwrap() = (*info_ptr).num_palette as std::os::raw::c_int;
        return 0x8 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_sBIT<'a1, 'a2>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut sig_bit: Option<&'a1 mut Option<&'a2 mut crate::src::libpng::png::png_color_8_struct>>)
 -> std::os::raw::c_uint {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !borrow(& sig_bit).is_none() {
        *(borrow_mut(&mut sig_bit)).unwrap() = Some(&mut (*info_ptr).sig_bit);
        return 0x2 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_tRNS<'a1, 'a2, 'a3, 'a4>(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                      mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                      mut trans_alpha: Option<&'a1 mut * mut std::os::raw::c_uchar>,
                                      mut num_trans: Option<&'a2 mut std::os::raw::c_int>,
                                      mut trans_color: Option<&'a3 mut Option<&'a4 mut crate::src::libpng::png::png_color_16_struct>>)
 -> std::os::raw::c_uint {
    let mut retval: u32 = 0 as std::os::raw::c_int as png_uint_32;
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x10 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*info_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            if !borrow(& trans_alpha).is_none() {
                *(borrow_mut(&mut trans_alpha)).unwrap() = (*info_ptr).trans_alpha;
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !borrow(& trans_color).is_none() {
                *(borrow_mut(&mut trans_color)).unwrap() = Some(&mut (*info_ptr).trans_color)
            }
        } else {
            /* if (info_ptr->color_type != PNG_COLOR_TYPE_PALETTE) */
            if !borrow(& trans_color).is_none() {
                *(borrow_mut(&mut trans_color)).unwrap() = Some(&mut (*info_ptr).trans_color);
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !borrow(& trans_alpha).is_none() { *(borrow_mut(&mut trans_alpha)).unwrap() = 0 as png_bytep }
        }
        if !borrow(& num_trans).is_none() {
            *(borrow_mut(&mut num_trans)).unwrap() = (*info_ptr).num_trans as std::os::raw::c_int;
            retval |= 0x10 as std::os::raw::c_uint
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_unknown_chunks<'a1>(mut png_ptr:
                                                    * const crate::src::libpng::png::png_struct_def,
                                                mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                                mut unknowns:
                                                    Option<&'a1 mut * mut crate::src::libpng::png::png_unknown_chunk_t>)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() && !info_ptr.is_null() && !borrow(& unknowns).is_none() {
        *(borrow_mut(&mut unknowns)).unwrap() = (*info_ptr).unknown_chunks;
        return (*info_ptr).unknown_chunks_num
    }
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_compression_buffer_size<'a1>(mut png_ptr:
                                                             Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_ulong {
    if (png_ptr).clone().is_none() { return 0 as std::os::raw::c_int as png_size_t }
    if (*((png_ptr).clone()).unwrap()).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        return (*((png_ptr).clone()).unwrap()).IDAT_read_size as png_size_t
    } else { return (*((png_ptr).clone()).unwrap()).zbuffer_size as png_size_t };
}
/* These functions were added to libpng 1.2.6 and were enabled
 * by default in libpng-1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_user_width_max<'a1>(mut png_ptr:
                                                    Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_uint {
    return if !(png_ptr).clone().is_none() {
               (*((png_ptr).clone()).unwrap()).user_width_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
#[no_mangle]
pub unsafe extern "C" fn png_get_user_height_max<'a1>(mut png_ptr:
                                                     Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_uint {
    return if !(png_ptr).clone().is_none() {
               (*((png_ptr).clone()).unwrap()).user_height_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
/* This function was added to libpng 1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_chunk_cache_max<'a1>(mut png_ptr:
                                                     Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_uint {
    return if !(png_ptr).clone().is_none() {
               (*((png_ptr).clone()).unwrap()).user_chunk_cache_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
/* This function was added to libpng 1.4.1 */
#[no_mangle]
pub unsafe extern "C" fn png_get_chunk_malloc_max<'a1>(mut png_ptr:
                                                      Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_ulong {
    return if !(png_ptr).clone().is_none() {
               (*((png_ptr).clone()).unwrap()).user_chunk_malloc_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
}
/* SET_USER_LIMITS */
/* These functions were added to libpng 1.4.0 */
#[no_mangle]
pub unsafe extern "C" fn png_get_io_state(mut png_ptr: * const crate::src::libpng::png::png_struct_def)
 -> std::os::raw::c_uint {
    return (*png_ptr).io_state;
}
#[no_mangle]
pub unsafe extern "C" fn png_get_io_chunk_type<'a1>(mut png_ptr:
                                                   Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> std::os::raw::c_uint {
    return (*((png_ptr).clone()).unwrap()).chunk_name;
}
/* IO_STATE */
#[no_mangle]
pub unsafe extern "C" fn png_get_palette_max<'a1, 'a2>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>,
                                             mut info_ptr: Option<&'a2 crate::src::libpng::png::png_info_def>)
 -> std::os::raw::c_int {
    if !(png_ptr).clone().is_none() && !(info_ptr).clone().is_none() {
        return (*((png_ptr).clone()).unwrap()).num_palette_max
    }
    return -(1 as std::os::raw::c_int);
}
use crate::laertes_rt::*;
/* READ || WRITE */
