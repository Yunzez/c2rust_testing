








use crate::src::libpng::png::png_const_structrp as other_png_const_structrp;

extern "C" {
    
    
    
}
pub use crate::src::libpng::png::png_check_IHDR;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_16 = crate::src::libpng::png::png_uint_16;
pub type png_int_32 = crate::src::libpng::png::png_int_32;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_size_t = crate::src::libpng::png::png_size_t;
pub type png_alloc_size_t = crate::src::libpng::png::png_alloc_size_t;
pub type png_fixed_point = crate::src::libpng::png::png_fixed_point;
pub type png_voidp = crate::src::libpng::png::png_voidp;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_bytep = crate::src::libpng::png::png_const_bytep;
pub type png_uint_16p = crate::src::libpng::png::png_uint_16p;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
// #[derive(Copy, Clone)]

pub type png_struct_def = crate::src::libpng::pngerror::png_struct_def;
pub type png_row_infop = crate::src::libpng::png::png_row_infop;
pub type png_row_info = crate::src::libpng::png::png_row_info;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::pngerror::png_row_info_struct;
pub type uInt = crate::src::libpng::png::uInt;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk;
// #[derive(Copy, Clone)]

pub type png_unknown_chunk_t = crate::src::libpng::pngerror::png_unknown_chunk_t;
pub type png_write_status_ptr = crate::src::libpng::png::png_write_status_ptr;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_read_status_ptr = crate::src::libpng::png::png_read_status_ptr;
pub type png_color_16 = crate::src::libpng::png::png_color_16;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::pngerror::png_color_16_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::pngerror::png_color_8_struct;
pub type png_flush_ptr = crate::src::libpng::png::png_flush_ptr;
pub type png_colorp = crate::src::libpng::png::png_colorp;
pub type png_color = crate::src::libpng::png::png_color;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::pngerror::png_color_struct;
pub type png_compression_bufferp = crate::src::libpng::png::png_compression_bufferp;
// #[derive(Copy, Clone)]

pub type png_compression_buffer = crate::src::libpng::pngerror::png_compression_buffer;
pub type z_stream = crate::src::libpng::png::z_stream;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::pngerror::z_stream_s;
pub type uLong = crate::src::libpng::png::uLong;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type free_func = crate::src::libpng::png::free_func;
pub type alloc_func = crate::src::libpng::png::alloc_func;
pub type Bytef = crate::src::libpng::png::Bytef;
pub type Byte = crate::src::libpng::png::Byte;
pub type png_rw_ptr = crate::src::libpng::png::png_rw_ptr;
pub type png_error_ptr = crate::src::libpng::png::png_error_ptr;
pub type png_const_structp = *const png_struct;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::png::png_info_def;
pub type png_unknown_chunkp = crate::src::libpng::png::png_unknown_chunkp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_const_infop = *const png_info;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_inforp = *const png_info;
pub type png_color_16p = *mut png_color_16;
pub type png_color_8p = *mut png_color_8;
pub type png_unknown_chunkpp = *mut *mut png_unknown_chunk;
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

pub unsafe extern "C" fn png_get_valid(mut png_ptr: png_const_structrp,
                                       mut info_ptr: png_const_inforp,
                                       mut flag: png_uint_32) -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).valid & flag
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub fn png_get_rowbytes(png_ptr: Option<&png_struct>, info_ptr: Option<&png_info>) -> png_size_t {
    match (png_ptr, info_ptr) {
        (Some(info), Some(png)) => png.rowbytes,
        _ => 0,
    }
}


pub fn png_get_rows(png_ptr: *const png_struct, info_ptr: *const png_info) -> *mut *mut u8 {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        unsafe {
            return (*info_ptr).row_pointers;
        }
    }
    std::ptr::null_mut()
}

/* Easy access to info, added in libpng-0.99 */

pub fn png_get_image_width(png_ptr: Option<&png_struct>, info_ptr: Option<&png_info>) -> png_uint_32 {
    match (png_ptr, info_ptr) {
        (Some(info), Some(png)) => png.width,
        _ => 0,
    }
}


pub unsafe extern "C" fn png_get_image_height(mut png_ptr: png_const_structrp,
                                              mut info_ptr: png_const_inforp)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() { return (*info_ptr).height }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub fn png_get_bit_depth(png_ptr: *const png_struct, info_ptr: *const png_info) -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        unsafe {
            return (*info_ptr).bit_depth;
        }
    }
    0
}


pub unsafe extern "C" fn png_get_color_type(mut png_ptr: png_const_structrp,
                                            mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).color_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}

pub unsafe extern "C" fn png_get_filter_type(mut png_ptr: png_const_structrp,
                                             mut info_ptr: png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).filter_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}

pub unsafe extern "C" fn png_get_interlace_type(mut png_ptr:
                                                    png_const_structrp,
                                                mut info_ptr:
                                                    png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).interlace_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}

pub unsafe extern "C" fn png_get_compression_type(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_byte {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).compression_type
    }
    return 0 as std::os::raw::c_int as png_byte;
}

pub extern "C" fn png_get_x_pixels_per_meter(mut png_ptr:
                                                        png_const_structrp,
                                                    mut info_ptr:
                                                        png_const_inforp)
 -> png_uint_32 {
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub extern "C" fn png_get_y_pixels_per_meter(mut png_ptr:
                                                        png_const_structrp,
                                                    mut info_ptr:
                                                        png_const_inforp)
 -> png_uint_32 {
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub fn png_get_pixels_per_meter(png_ptr: png_const_structrp, info_ptr: png_const_inforp) -> u32 {
    0
}


pub extern "C" fn png_get_pixel_aspect_ratio_fixed(mut png_ptr:
                                                              png_const_structrp,
                                                          mut info_ptr:
                                                              png_const_inforp)
 -> png_fixed_point {
    return 0 as std::os::raw::c_int;
}

pub extern "C" fn png_get_x_offset_microns(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}

pub extern "C" fn png_get_y_offset_microns(mut png_ptr:
                                                      png_const_structrp,
                                                  mut info_ptr:
                                                      png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}

pub extern "C" fn png_get_x_offset_pixels(mut png_ptr:
                                                     png_const_structrp,
                                                 mut info_ptr:
                                                     png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}

pub extern "C" fn png_get_y_offset_pixels(mut png_ptr:
                                                     png_const_structrp,
                                                 mut info_ptr:
                                                     png_const_inforp)
 -> png_int_32 {
    return 0 as std::os::raw::c_int;
}
/* INCH_CONVERSIONS */
/* png_get_channels really belongs in here, too, but it's been around longer */
/* EASY_ACCESS */

pub fn png_get_channels(png_ptr: &png_struct, info_ptr: &png_info) -> png_byte {
    if let (Some(png_ptr), Some(info_ptr)) = (Some(png_ptr), Some(info_ptr)) {
        return info_ptr.channels;
    }
    0
}


pub unsafe extern "C" fn png_get_signature(mut png_ptr: png_const_structrp,
                                           mut info_ptr: png_const_inforp)
 -> png_const_bytep {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*info_ptr).signature.as_ptr()
    }
    return 0 as png_const_bytep;
}

pub unsafe extern "C" fn png_get_bKGD(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut background: *mut png_color_16p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x20 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !background.is_null() {
        *background = &mut (*info_ptr).background;
        return 0x20 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub unsafe extern "C" fn png_get_hIST(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut hist: *mut png_uint_16p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x40 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !hist.is_null() {
        *hist = (*info_ptr).hist;
        return 0x40 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub unsafe extern "C" fn png_get_IHDR(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_const_inforp,
                                      mut width: *mut png_uint_32,
                                      mut height: *mut png_uint_32,
                                      mut bit_depth: *mut std::os::raw::c_int,
                                      mut color_type: *mut std::os::raw::c_int,
                                      mut interlace_type: *mut std::os::raw::c_int,
                                      mut compression_type: *mut std::os::raw::c_int,
                                      mut filter_type: *mut std::os::raw::c_int)
 -> png_uint_32 {
    if png_ptr.is_null() || info_ptr.is_null() {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if !width.is_null() { *width = (*info_ptr).width }
    if !height.is_null() { *height = (*info_ptr).height }
    if !bit_depth.is_null() {
        *bit_depth = (*info_ptr).bit_depth as std::os::raw::c_int
    }
    if !color_type.is_null() {
        *color_type = (*info_ptr).color_type as std::os::raw::c_int
    }
    if !compression_type.is_null() {
        *compression_type = (*info_ptr).compression_type as std::os::raw::c_int
    }
    if !filter_type.is_null() {
        *filter_type = (*info_ptr).filter_type as std::os::raw::c_int
    }
    if !interlace_type.is_null() {
        *interlace_type = (*info_ptr).interlace_type as std::os::raw::c_int
    }
    /* This is redundant if we can be sure that the info_ptr values were all
    * assigned in png_set_IHDR().  We do the check anyhow in case an
    * application has ignored our advice not to mess with the members
    * of info_ptr directly.
    */
    let width = (*info_ptr).width;
let height = (*info_ptr).height;
let bit_depth = (*info_ptr).bit_depth as i32;
let color_type = (*info_ptr).color_type as i32;
let interlace_type = (*info_ptr).interlace_type as i32;
let compression_type = (*info_ptr).compression_type as i32;
let filter_type = (*info_ptr).filter_type as i32;
png_check_IHDR(png_ptr as *mut png_struct_def, width, height, bit_depth, color_type, interlace_type, compression_type, filter_type);
    return 1 as std::os::raw::c_int as png_uint_32;
}
/* sCAL */
/* pHYs */

pub unsafe extern "C" fn png_get_PLTE(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut palette: *mut png_colorp,
                                      mut num_palette: *mut std::os::raw::c_int)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x8 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !palette.is_null() {
        *palette = (*info_ptr).palette;
        *num_palette = (*info_ptr).num_palette as std::os::raw::c_int;
        return 0x8 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub unsafe extern "C" fn png_get_sBIT(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut sig_bit: *mut png_color_8p)
 -> png_uint_32 {
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint && !sig_bit.is_null() {
        *sig_bit = &mut (*info_ptr).sig_bit;
        return 0x2 as std::os::raw::c_uint
    }
    return 0 as std::os::raw::c_int as png_uint_32;
}

pub unsafe extern "C" fn png_get_tRNS(mut png_ptr: png_const_structrp,
                                      mut info_ptr: png_inforp,
                                      mut trans_alpha: *mut png_bytep,
                                      mut num_trans: *mut std::os::raw::c_int,
                                      mut trans_color: *mut png_color_16p)
 -> png_uint_32 {
    let mut retval: png_uint_32 = 0 as std::os::raw::c_int as png_uint_32;
    if !png_ptr.is_null() && !info_ptr.is_null() &&
           (*info_ptr).valid & 0x10 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*info_ptr).color_type as std::os::raw::c_int ==
               2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
            if !trans_alpha.is_null() {
                *trans_alpha = (*info_ptr).trans_alpha;
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !trans_color.is_null() {
                *trans_color = &mut (*info_ptr).trans_color
            }
        } else {
            /* if (info_ptr->color_type != PNG_COLOR_TYPE_PALETTE) */
            if !trans_color.is_null() {
                *trans_color = &mut (*info_ptr).trans_color;
                retval |= 0x10 as std::os::raw::c_uint
            }
            if !trans_alpha.is_null() { *trans_alpha = 0 as png_bytep }
        }
        if !num_trans.is_null() {
            *num_trans = (*info_ptr).num_trans as std::os::raw::c_int;
            retval |= 0x10 as std::os::raw::c_uint
        }
    }
    return retval;
}

pub fn png_get_unknown_chunks(png_ptr: &png_struct, info_ptr: &png_info) -> (Vec<png_unknown_chunk>, i32) {
    // Assuming png_ptr and info_ptr are valid references, we don't need to check for null
    let unknowns = unsafe { (*info_ptr).unknown_chunks };
    let unknowns_num = unsafe { (*info_ptr).unknown_chunks_num };

    // Convert the raw pointer to a Vec
    let unknowns_vec = unsafe {
        std::slice::from_raw_parts(unknowns, unknowns_num as usize).to_vec()
    };

    (unknowns_vec, unknowns_num)
}


pub unsafe extern "C" fn png_get_compression_buffer_size(mut png_ptr:
                                                             png_const_structrp)
 -> png_size_t {
    if png_ptr.is_null() { return 0 as std::os::raw::c_int as png_size_t }
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        return (*png_ptr).IDAT_read_size as png_size_t
    } else { return (*png_ptr).zbuffer_size as png_size_t };
}
/* These functions were added to libpng 1.2.6 and were enabled
 * by default in libpng-1.4.0 */

pub fn png_get_user_width_max(png_ptr: Option<&png_const_structrp>) -> u32 {
    match png_ptr {
        Some(ptr) => unsafe { (*ptr).as_ref().map_or(0, |p| p.user_width_max) },
        None => 0,
    }
}


pub unsafe extern "C" fn png_get_user_height_max(mut png_ptr:
                                                     png_const_structrp)
 -> png_uint_32 {
    return if !png_ptr.is_null() {
               (*png_ptr).user_height_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_uint };
}
/* This function was added to libpng 1.4.0 */

pub fn png_get_chunk_cache_max(png_ptr: Option<&png_const_structrp>) -> u32 {
    match png_ptr {
        Some(ptr) => {
            let deref_ptr = unsafe { &**ptr };
            deref_ptr.user_chunk_cache_max
        },
        None => 0,
    }
}

/* This function was added to libpng 1.4.1 */

pub unsafe extern "C" fn png_get_chunk_malloc_max(mut png_ptr:
                                                      png_const_structrp)
 -> png_alloc_size_t {
    return if !png_ptr.is_null() {
               (*png_ptr).user_chunk_malloc_max
           } else { 0 as std::os::raw::c_int as std::os::raw::c_ulong };
}
/* SET_USER_LIMITS */
/* These functions were added to libpng 1.4.0 */

pub unsafe extern "C" fn png_get_io_state(mut png_ptr: png_const_structrp)
 -> png_uint_32 {
    return (*png_ptr).io_state;
}

pub unsafe extern "C" fn png_get_io_chunk_type(mut png_ptr:
                                                   png_const_structrp)
 -> png_uint_32 {
    return (*png_ptr).chunk_name;
}
/* IO_STATE */

pub unsafe extern "C" fn png_get_palette_max(mut png_ptr: png_const_structp,
                                             mut info_ptr: png_const_infop)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() && !info_ptr.is_null() {
        return (*png_ptr).num_palette_max
    }
    return -(1 as std::os::raw::c_int);
}
/* READ || WRITE */
