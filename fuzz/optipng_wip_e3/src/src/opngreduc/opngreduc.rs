










use std::slice;

extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    fn __assert_fail(__assertion: *const std::os::raw::c_char,
                     __file: *const std::os::raw::c_char, __line: std::os::raw::c_uint,
                     __function: *const std::os::raw::c_char) -> !;
}
pub use crate::src::libpng::png::png_free_data;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngget::png_get_IHDR;
pub use crate::src::libpng::pngget::png_get_PLTE;
pub use crate::src::libpng::pngget::png_get_bKGD;
pub use crate::src::libpng::pngget::png_get_bit_depth;
pub use crate::src::libpng::pngget::png_get_channels;
pub use crate::src::libpng::pngget::png_get_color_type;
pub use crate::src::libpng::pngget::png_get_hIST;
pub use crate::src::libpng::pngget::png_get_image_height;
pub use crate::src::libpng::pngget::png_get_image_width;
pub use crate::src::libpng::pngget::png_get_rows;
pub use crate::src::libpng::pngget::png_get_sBIT;
pub use crate::src::libpng::pngget::png_get_tRNS;
pub use crate::src::libpng::pngget::png_get_valid;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::pngset::png_set_PLTE;
pub use crate::src::libpng::pngset::png_set_invalid;
pub use crate::src::libpng::pngset::png_set_tRNS;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_16 = crate::src::libpng::png::png_uint_16;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_size_t = crate::src::libpng::png::png_size_t;
pub type png_alloc_size_t = crate::src::libpng::png::png_alloc_size_t;
pub type png_voidp = crate::src::libpng::png::png_voidp;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_bytep = crate::src::libpng::png::png_const_bytep;
pub type png_uint_16p = crate::src::libpng::png::png_uint_16p;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infop = crate::src::libpng::png::png_infop;
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_inforp = crate::src::libpng::pngget::png_const_inforp;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::pngerror::png_color_struct;
pub type png_color = crate::src::libpng::png::png_color;
pub type png_colorp = crate::src::libpng::png::png_colorp;
pub type png_const_colorp = crate::src::libpng::pngrutil::png_const_colorp;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::pngerror::png_color_16_struct;
pub type png_color_16 = crate::src::libpng::png::png_color_16;
pub type png_color_16p = crate::src::libpng::pngget::png_color_16p;
pub type png_const_color_16p = crate::src::libpng::pngrutil::png_const_color_16p;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::pngerror::png_color_8_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
pub type png_color_8p = crate::src::libpng::pngget::png_color_8p;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::pngerror::png_row_info_struct;
pub type png_row_info = crate::src::libpng::png::png_row_info;
pub type png_row_infop = crate::src::libpng::png::png_row_infop;
/*
 * opngreduc.c - libpng extension: lossless image reductions.
 *
 * Copyright (C) 2003-2014 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 */
/* CAUTION:
 * Image reductions do not work well under certain transformations.
 *
 * Transformations like PNG_BGR, PNG_SWAP_BYTES, PNG_FILLER, PNG_INVERT_ALPHA,
 * and possibly others, require special treatment. However, the libpng API
 * does not currently convey the effect of transformations on its internal
 * state or on the layout of pixel data.
 *
 * Transformations which affect pixel depth (e.g. PNG_FILLER) are especially
 * dangerous when used in conjunction with this code, and should be avoided.
 */
/*
 * Check if the image information is valid.
 * The image information is said to be valid if all the required
 * critical chunk data is present in the png structures.
 * The function returns 1 if this information is valid, and 0 otherwise.
 */

pub fn opng_validate_image(png_ptr: *const png_struct_def, info_ptr: *const png_info_def) -> std::os::raw::c_int {
    unsafe {
        /* Validate IHDR. */
        if png_get_bit_depth(png_ptr, info_ptr) == 0 {
            return 0;
        }
        /* Validate PLTE. */
        if png_get_color_type(png_ptr, info_ptr) & 1 != 0 {
            if png_get_valid(png_ptr, info_ptr, 0x8) == 0 {
                return 0;
            }
        }
        /* Validate IDAT. */
        if png_get_valid(png_ptr, info_ptr, 0x8000) == 0 {
            return 0;
        }
    }
    return 1;
}

/*
 * Build a color+alpha palette in which the entries are sorted by
 * (alpha, red, green, blue), in this particular order.
 * Use the insertion sort algorithm.
 * The alpha value is ignored if it is not in the range [0 .. 255].
 * The function returns:
 *   1 if the insertion is successful;  *index = position of new entry.
 *   0 if the insertion is unnecessary; *index = position of crt entry.
 *  -1 if overflow;            *num_palette = *num_trans = *index = -1.
 */
unsafe extern "C" fn opng_insert_palette_entry(mut palette: png_colorp,
                                               mut num_palette:
                                                   *mut std::os::raw::c_int,
                                               mut trans_alpha: png_bytep,
                                               mut num_trans:
                                                   *mut std::os::raw::c_int,
                                               mut max_tuples: std::os::raw::c_int,
                                               mut red: std::os::raw::c_uint,
                                               mut green: std::os::raw::c_uint,
                                               mut blue: std::os::raw::c_uint,
                                               mut alpha: std::os::raw::c_uint,
                                               mut index: *mut std::os::raw::c_int)
 -> std::os::raw::c_int {
    let mut low: i32 = 0;
let mut high: i32 = 0;
let mut mid: i32 = 0;
let mut cmp: i32 = 0;
let mut i: i32 = 0;

let palette_slice = unsafe { std::slice::from_raw_parts_mut(palette, *num_palette as usize) };
let trans_alpha_slice = unsafe { std::slice::from_raw_parts(trans_alpha, *num_trans as usize) };

if *num_palette < 0 || *num_palette > max_tuples {
    panic!("*num_palette >= 0 && *num_palette <= max_tuples");
}

if *num_trans < 0 || *num_trans > *num_palette {
    panic!("*num_trans >= 0 && *num_trans <= *num_palette");
}

if alpha < 255 {
    // Do a binary search among transparent tuples.
    low = 0;
    high = *num_trans - 1;
    while low <= high {
        mid = (low + high) / 2;
        cmp = if alpha != trans_alpha_slice[mid as usize] as u32 {
            alpha as i32 - trans_alpha_slice[mid as usize] as i32
        } else if red != palette_slice[mid as usize].red as u32 {
            red as i32 - palette_slice[mid as usize].red as u32 as i32
        } else if green != palette_slice[mid as usize].green as u32 {
            green as i32 - palette_slice[mid as usize].green as u32 as i32
        } else {
            blue as i32 - palette_slice[mid as usize].blue as u32 as i32
        };

        if cmp < 0 {
            high = mid - 1;
        } else if cmp > 0 {
            low = mid + 1;
        } else {
            *index = mid;
            return 0;
        }
    }
} else {
    // alpha == 255 || alpha not in [0 .. 255]
    // Do a (faster) binary search among opaque tuples.
    low = *num_trans;
    high = *num_palette - 1;
    while low <= high {
        mid = (low + high) / 2;
        cmp = if red != palette_slice[mid as usize].red as u32 {
            red as i32 - palette_slice[mid as usize].red as u32 as i32
        } else if green != palette_slice[mid as usize].green as u32 {
            green as i32 - palette_slice[mid as usize].green as u32 as i32
        } else {
            blue as i32 - palette_slice[mid as usize].blue as u32 as i32
        };

        if cmp < 0 {
            high = mid - 1;
        } else if cmp > 0 {
            low = mid + 1;
        } else {
            *index = mid;
            return 0;
        }
    }
}

if alpha > 255 {
    // The binary search among opaque tuples has failed.
    // Do a linear search among transparent tuples, ignoring alpha.
    for i in 0..*num_trans {
        cmp = if red != palette_slice[i as usize].red as u32 {
            red as i32 - palette_slice[i as usize].red as u32 as i32
        } else if green != palette_slice[i as usize].green as u32 {
            green as i32 - palette_slice[i as usize].green as u32 as i32
        } else {
            blue as i32 - palette_slice[i as usize].blue as u32 as i32
        };

        if cmp == 0 {
            *index = i;
            return 0;
        }
    }
}

// Check for overflow.
if *num_palette >= max_tuples {
    *index = -1;
    *num_trans = *index;
    *num_palette = *num_trans;
    return -1;
}

// Insert new tuple at [low].
if low < 0 || low > *num_palette {
    panic!("low >= 0 && low <= *num_palette");
}

let mut i = *num_palette;
while i > low {
    palette_slice[i as usize] = palette_slice[(i - 1) as usize];
    i -= 1;
}

palette_slice[low as usize].red = red as u8;
palette_slice[low as usize].green = green as u8;
palette_slice[low as usize].blue = blue as u8;
*num_palette += 1;

    if alpha < 255 {
    if low <= *num_trans {
        // Do nothing, condition is satisfied
    } else {
        panic!("low <= *num_trans");
    }
    
    let trans_alpha_slice = unsafe { std::slice::from_raw_parts_mut(trans_alpha, *num_trans as usize + 1) };
    let mut i = *num_trans;
    while i > low {
        trans_alpha_slice[i as usize] = trans_alpha_slice[(i - 1) as usize];
        i -= 1;
    }
    trans_alpha_slice[low as usize] = alpha as u8;
    *num_trans += 1;
}
*index = low;
return 1;

}
/*
 * Change the size of the palette buffer.
 * Changing info_ptr->num_palette directly, avoiding reallocation, should
 * have been sufficient, but can't be done using the current libpng API.
 */
fn opng_realloc_PLTE(png_ptr: *mut png_struct_def, info_ptr: *mut png_info_def, num_palette: i32) {
    let mut buffer: Vec<png_color> = vec![png_color { red: 0, green: 0, blue: 0 }; 256];
    let mut palette: *mut png_color = std::ptr::null_mut();
    let mut src_num_palette: i32 = 0;

    if num_palette <= 0 {
        panic!("num_palette must be greater than 0");
    }

    unsafe {
        png_get_PLTE(png_ptr, info_ptr, &mut palette, &mut src_num_palette);
    }

    if num_palette == src_num_palette {
        return;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(palette, buffer.as_mut_ptr(), num_palette as usize);
    }

    if num_palette > src_num_palette {
        for i in src_num_palette..num_palette {
            buffer[i as usize] = png_color { red: 0, green: 0, blue: 0 };
        }
    }

    unsafe {
        png_set_PLTE(png_ptr, info_ptr, buffer.as_ptr(), num_palette);
    }
}

/*
 * Change the size of the transparency buffer.
 * Changing info_ptr->num_trans directly, avoiding reallocation, should
 * have been sufficient, but can't be done using the current libpng API.
 */
fn opng_realloc_tRNS(png_ptr: *mut png_struct_def, info_ptr: *mut png_info_def, num_trans: i32) {
    let mut buffer: [u8; 256] = [0; 256]; // tRNS should be invalidated in this case
    let mut trans_alpha: *mut u8 = std::ptr::null_mut();
    let mut src_num_trans: i32 = 0;

    if num_trans <= 0 {
        panic!("num_trans must be greater than 0");
    }

    unsafe {
        png_get_tRNS(png_ptr, info_ptr, &mut trans_alpha, &mut src_num_trans, std::ptr::null_mut());
    }

    if num_trans == src_num_trans {
        return;
    }

    unsafe {
        std::ptr::copy_nonoverlapping(trans_alpha, buffer.as_mut_ptr(), num_trans as usize);
    }

    if num_trans > src_num_trans {
        buffer[src_num_trans as usize..num_trans as usize].fill(0);
    }

    unsafe {
        png_set_tRNS(png_ptr, info_ptr, buffer.as_ptr(), num_trans, std::ptr::null());
    }
}

/*
 * Retrieve the alpha samples from the given image row.
 */
unsafe extern "C" fn opng_get_alpha_row(mut row_info_ptr: png_row_infop,
                                        mut trans_color: png_color_16p,
                                        mut row: png_bytep,
                                        mut alpha_row: png_bytep) {
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut channels: std::os::raw::c_int = 0;
    let mut trans_red: png_byte = 0;
    let mut trans_green: png_byte = 0;
    let mut trans_blue: png_byte = 0;
    let mut trans_gray: png_byte = 0;
    let mut i: png_uint_32 = 0;
    width = (*row_info_ptr).width;
    color_type = (*row_info_ptr).color_type as std::os::raw::c_int;
    bit_depth = (*row_info_ptr).bit_depth as std::os::raw::c_int;
    channels = (*row_info_ptr).channels as std::os::raw::c_int;
    if color_type & 1 as std::os::raw::c_int == 0 {
    } else {
        __assert_fail(b"!(color_type & 1)\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      267 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    if bit_depth == 8 as std::os::raw::c_int {
    } else {
        __assert_fail(b"bit_depth == 8\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      268 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    if color_type & 4 as std::os::raw::c_int == 0 {
        if trans_color.is_null() {
            /* All pixels are fully opaque. */
            memset(alpha_row as *mut std::os::raw::c_void, 255 as std::os::raw::c_int,
                   width as size_t);
            return
        }
        if color_type == 2 as std::os::raw::c_int {
            if channels == 3 as std::os::raw::c_int {
            } else {
                __assert_fail(b"channels == 3\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              280 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*::std::mem::transmute::<&[u8; 76],
                                                        &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            trans_red = (*trans_color).red as png_byte;
            trans_green = (*trans_color).green as png_byte;
            trans_blue = (*trans_color).blue as png_byte;
            sample_ptr = row;
            i = 0 as std::os::raw::c_int as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) =
                    if *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int == trans_red as std::os::raw::c_int &&
                           *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == trans_green as std::os::raw::c_int &&
                           *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int == trans_blue as std::os::raw::c_int {
                        0 as std::os::raw::c_int
                    } else { 255 as std::os::raw::c_int } as png_byte;
                i = i.wrapping_add(1);
                sample_ptr = sample_ptr.offset(3 as std::os::raw::c_int as isize)
            }
        } else {
            if color_type == 0 as std::os::raw::c_int {
            } else {
                __assert_fail(b"color_type == 0\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              293 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*::std::mem::transmute::<&[u8; 76],
                                                        &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            if channels == 1 as std::os::raw::c_int {
            } else {
                __assert_fail(b"channels == 1\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              294 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*::std::mem::transmute::<&[u8; 76],
                                                        &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
            }
            trans_gray = (*trans_color).gray as png_byte;
            i = 0 as std::os::raw::c_int as png_uint_32;
            while i < width {
                *alpha_row.offset(i as isize) =
                    if *row.offset(i as isize) as std::os::raw::c_int ==
                           trans_gray as std::os::raw::c_int {
                        0 as std::os::raw::c_int
                    } else { 255 as std::os::raw::c_int } as png_byte;
                i = i.wrapping_add(1)
            }
        }
        return
    }
    /* There is a real alpha channel. The alpha sample is last in RGBA tuple. */
    if channels > 1 as std::os::raw::c_int {
    } else {
        __assert_fail(b"channels > 1\x00" as *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      303 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 76],
                                                &[std::os::raw::c_char; 76]>(b"void opng_get_alpha_row(png_row_infop, png_color_16p, png_bytep, png_bytep)\x00")).as_ptr());
    }
    sample_ptr = row.offset((channels - 1 as std::os::raw::c_int) as isize);
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < width {
        *alpha_row = *sample_ptr;
        i = i.wrapping_add(1);
        sample_ptr = sample_ptr.offset(channels as isize);
        alpha_row = alpha_row.offset(1)
    };
}
/*
 * Analyze the redundancy of bits inside the image.
 * The parameter reductions indicates the intended reductions.
 * The function returns the possible reductions.
 */
fn opng_analyze_bits(png_ptr: *mut png_struct_def, 
                     info_ptr: *mut png_info_def, 
                     mut reductions: u32) -> u32 {
    unsafe {
         let mut row_ptr: png_bytepp =
        0 as *mut *mut png_byte; /* not applicable */
    let mut component_ptr: png_bytep =
        0 as *mut png_byte; /* let opng_reduce_palette() handle it */
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut byte_depth: std::os::raw::c_int = 0;
    let mut channels: std::os::raw::c_int = 0;
    let mut sample_size: std::os::raw::c_int = 0;
    let mut offset_alpha: std::os::raw::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 &mut width, &mut height, &mut bit_depth, &mut color_type,
                 0 as *mut std::os::raw::c_int, 0 as *mut std::os::raw::c_int,
                 0 as *mut std::os::raw::c_int);
    if bit_depth < 8 as std::os::raw::c_int { return 0 as std::os::raw::c_int as png_uint_32 }
    if color_type & 1 as std::os::raw::c_int != 0 {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    byte_depth = bit_depth / 8 as std::os::raw::c_int;
    let channels = png_get_channels(&*png_ptr, &*info_ptr) as std::os::raw::c_int;
    sample_size = channels * byte_depth;
    offset_alpha = (channels - 1 as std::os::raw::c_int) * byte_depth;
    /* Select the applicable reductions. */
    reductions &=
        (0x1 as std::os::raw::c_int | 0x4 as std::os::raw::c_int | 0x8 as std::os::raw::c_int) as
            std::os::raw::c_uint;
    if bit_depth <= 8 as std::os::raw::c_int {
        reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    if color_type & 2 as std::os::raw::c_int == 0 {
        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    if color_type & 4 as std::os::raw::c_int == 0 {
        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint
    }
    /* Check if the ancillary information allows these reductions. */
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background)
           != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*background).red as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                   0 as std::os::raw::c_int ||
                   (*background).green as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int ||
                   (*background).blue as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int ||
                   (*background).gray as std::os::raw::c_int % 257 as std::os::raw::c_int !=
                       0 as std::os::raw::c_int {
                reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*background).red as std::os::raw::c_int !=
                   (*background).green as std::os::raw::c_int ||
                   (*background).red as std::os::raw::c_int !=
                       (*background).blue as std::os::raw::c_int {
                reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint
            }
        }
    }
    /* Check for each possible reduction, row by row. */
    let mut row_ptr = png_get_rows(png_ptr, info_ptr); /* no need to go any further */
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        if reductions == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            return 0 as std::os::raw::c_int as png_uint_32
        }
        /* Check if it is possible to reduce the bit depth to 8. */
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            component_ptr = *row_ptr;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < (channels as std::os::raw::c_uint).wrapping_mul(width) {
                if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                       std::os::raw::c_int !=
                       *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int {
                    reductions &= !(0x1 as std::os::raw::c_int) as std::os::raw::c_uint;
                    break ;
                } else {
                    j = j.wrapping_add(1);
                    component_ptr =
                        component_ptr.offset(2 as std::os::raw::c_int as isize)
                }
            }
        }
        if bit_depth == 8 as std::os::raw::c_int {
            /* Check if it is possible to reduce rgb --> gray. */
            if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = *row_ptr;
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int !=
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int ||
                           *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(2 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                       {
                        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
            /* Check if it is possible to strip the alpha channel. */
            if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int != 255 as std::os::raw::c_int {
                        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
        } else {
            /* bit_depth == 16 */
            /* Check if it is possible to reduce rgb --> gray. */
            if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = *row_ptr;
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int !=
                           *component_ptr.offset(2 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int ||
                           *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(4 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                           ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(3 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                           ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int !=
                               *component_ptr.offset(5 as std::os::raw::c_int as
                                                         isize) as std::os::raw::c_int
                       {
                        reductions &= !(0x4 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
            if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                component_ptr = (*row_ptr).offset(offset_alpha as isize);
                j = 0 as std::os::raw::c_int as png_uint_32;
                while j < width {
                    if *component_ptr.offset(0 as std::os::raw::c_int as isize) as
                           std::os::raw::c_int != 255 as std::os::raw::c_int ||
                           *component_ptr.offset(1 as std::os::raw::c_int as isize) as
                               std::os::raw::c_int != 255 as std::os::raw::c_int {
                        reductions &= !(0x8 as std::os::raw::c_int) as std::os::raw::c_uint;
                        break ;
                    } else {
                        j = j.wrapping_add(1);
                        component_ptr =
                            component_ptr.offset(sample_size as isize)
                    }
                }
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    return reductions;

    }
}

/* Check if it is possible to strip the alpha channel. */
/*
 * Reduce the image type to a lower bit depth and color type,
 * by removing redundant bits.
 * Possible reductions: 16bpp to 8bpp; RGB to gray; strip alpha.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 * All reductions are performed in a single step.
 */
fn opng_reduce_bits(png_ptr: *mut png_struct_def, info_ptr: *mut png_info_def, reductions: u32) -> u32 {
    unsafe {
         let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut src_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut src_bit_depth: std::os::raw::c_int = 0;
    let mut dest_bit_depth: std::os::raw::c_int = 0;
    let mut src_byte_depth: std::os::raw::c_int = 0;
    let mut dest_byte_depth: std::os::raw::c_int = 0;
    let mut src_color_type: std::os::raw::c_int = 0;
    let mut dest_color_type: std::os::raw::c_int = 0;
    let mut src_channels: std::os::raw::c_int = 0;
    let mut dest_channels: std::os::raw::c_int = 0;
    let mut src_sample_size: std::os::raw::c_int = 0;
    let mut dest_sample_size: std::os::raw::c_int = 0;
    let mut tran_tbl: [std::os::raw::c_int; 8] = [0; 8];
    let mut trans_color: png_color_16p = 0 as *mut png_color_16;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut sig_bits: png_color_8p = 0 as *mut png_color_8;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: std::os::raw::c_int = 0;
    /* See which reductions may be performed. */
    let reductions = opng_analyze_bits(png_ptr, info_ptr, reductions); /* exit early */
    if reductions == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 &mut width, &mut height, &mut src_bit_depth,
                 &mut src_color_type, &mut interlace_type,
                 &mut compression_type, &mut filter_type);
    /* Compute the new image parameters bit_depth, color_type, etc. */
    if src_bit_depth >= 8 as std::os::raw::c_int {
    } else {
        __assert_fail(b"src_bit_depth >= 8\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      506 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[std::os::raw::c_char; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_bit_depth == 16 as std::os::raw::c_int {
        } else {
            __assert_fail(b"src_bit_depth == 16\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          509 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[std::os::raw::c_char; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_bit_depth = 8 as std::os::raw::c_int
    } else { dest_bit_depth = src_bit_depth }
    src_byte_depth = src_bit_depth / 8 as std::os::raw::c_int;
    dest_byte_depth = dest_bit_depth / 8 as std::os::raw::c_int;
    dest_color_type = src_color_type;
    if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_color_type & 2 as std::os::raw::c_int != 0 {
        } else {
            __assert_fail(b"src_color_type & 2\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          521 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[std::os::raw::c_char; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_color_type &= !(2 as std::os::raw::c_int)
    }
    if reductions & 0x8 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
        if src_color_type & 4 as std::os::raw::c_int != 0 {
        } else {
            __assert_fail(b"src_color_type & 4\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          526 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[std::os::raw::c_char; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        dest_color_type &= !(4 as std::os::raw::c_int)
    }
    let src_channels = png_get_channels(&*png_ptr, &*info_ptr) as std::os::raw::c_int;
    dest_channels =
        (if dest_color_type & 2 as std::os::raw::c_int != 0 {
             3 as std::os::raw::c_int
         } else { 1 as std::os::raw::c_int }) +
            (if dest_color_type & 4 as std::os::raw::c_int != 0 {
                 1 as std::os::raw::c_int
             } else { 0 as std::os::raw::c_int });
    src_sample_size = src_channels * src_byte_depth;
    dest_sample_size = dest_channels * dest_byte_depth;
    /* Pre-compute the intra-sample translation table. */
    k = 0 as std::os::raw::c_int;
    while k < 4 as std::os::raw::c_int * dest_byte_depth {
        tran_tbl[k as usize] = k * src_bit_depth / dest_bit_depth;
        k += 1
    }
    /* If rgb --> gray, shift the alpha component two positions to the left. */
    if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
           dest_color_type & 4 as std::os::raw::c_int != 0 {
        tran_tbl[dest_byte_depth as usize] =
            tran_tbl[(3 as std::os::raw::c_int * dest_byte_depth) as usize];
        if dest_byte_depth == 2 as std::os::raw::c_int {
            tran_tbl[(dest_byte_depth + 1 as std::os::raw::c_int) as usize] =
                tran_tbl[(3 as std::os::raw::c_int * dest_byte_depth +
                              1 as std::os::raw::c_int) as usize]
        }
    }
    /* Translate the samples to the new image type. */
    if src_sample_size > dest_sample_size {
    } else {
        __assert_fail(b"src_sample_size > dest_sample_size\x00" as *const u8
                          as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      551 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 66],
                                                &[std::os::raw::c_char; 66]>(b"png_uint_32 opng_reduce_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        dest_ptr = *row_ptr;
        src_ptr = dest_ptr;
        j = 0 as std::os::raw::c_int as png_uint_32;
        while j < width {
            k = 0 as std::os::raw::c_int;
            while k < dest_sample_size {
                *dest_ptr.offset(k as isize) =
                    *src_ptr.offset(tran_tbl[k as usize] as isize);
                k += 1
            }
            src_ptr = src_ptr.offset(src_sample_size as isize);
            dest_ptr = dest_ptr.offset(dest_sample_size as isize);
            j = j.wrapping_add(1)
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    /* Update the ancillary information. */
         if png_get_tRNS(png_ptr as *const png_struct, info_ptr,
                    0 as *mut png_bytep, 0 as *mut std::os::raw::c_int,
                    &mut trans_color) != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*trans_color).red as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                   0 as std::os::raw::c_int &&
                   (*trans_color).green as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int &&
                   (*trans_color).blue as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int &&
                   (*trans_color).gray as std::os::raw::c_int % 257 as std::os::raw::c_int ==
                       0 as std::os::raw::c_int {
                (*trans_color).red =
                    ((*trans_color).red as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*trans_color).green =
                    ((*trans_color).green as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*trans_color).blue =
                    ((*trans_color).blue as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16;
                (*trans_color).gray =
                    ((*trans_color).gray as std::os::raw::c_int & 255 as std::os::raw::c_int)
                        as png_uint_16
            } else {
                /* 16-bit tRNS in 8-bit samples: all pixels are 100% opaque. */
                png_free_data(png_ptr as *const png_struct, info_ptr,
                              0x2000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
                png_set_invalid(png_ptr as *mut png_struct, info_ptr, 0x10);
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*trans_color).red as std::os::raw::c_int ==
                   (*trans_color).green as std::os::raw::c_int ||
                   (*trans_color).red as std::os::raw::c_int ==
                       (*trans_color).blue as std::os::raw::c_int {
                (*trans_color).gray = (*trans_color).red
            } else {
                /* Non-gray tRNS in grayscale image: all pixels are 100% opaque. */
                png_free_data(png_ptr as *const png_struct, info_ptr,
                              0x2000 as std::os::raw::c_uint, -(1 as std::os::raw::c_int));
                png_set_invalid(png_ptr as *mut png_struct, info_ptr, 0x10);
            }
        }
    }
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background)
           != 0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            (*background).red =
                ((*background).red as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*background).green =
                ((*background).green as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*background).blue =
                ((*background).blue as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16;
            (*background).gray =
                ((*background).gray as std::os::raw::c_int & 255 as std::os::raw::c_int) as
                    png_uint_16
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            (*background).gray = (*background).red
        }
    }
    if png_get_sBIT(png_ptr as *const png_struct, info_ptr, &mut sig_bits) !=
           0 {
        if reductions & 0x1 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            if (*sig_bits).red as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*sig_bits).red = 8 as std::os::raw::c_int as png_byte
            }
            if (*sig_bits).green as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*sig_bits).green = 8 as std::os::raw::c_int as png_byte
            }
            if (*sig_bits).blue as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*sig_bits).blue = 8 as std::os::raw::c_int as png_byte
            }
            if (*sig_bits).gray as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*sig_bits).gray = 8 as std::os::raw::c_int as png_byte
            }
            if (*sig_bits).alpha as std::os::raw::c_int > 8 as std::os::raw::c_int {
                (*sig_bits).alpha = 8 as std::os::raw::c_int as png_byte
            }
        }
        if reductions & 0x4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
            let mut max_sig_bits: png_byte = (*sig_bits).red;
            if (max_sig_bits as std::os::raw::c_int) <
                   (*sig_bits).green as std::os::raw::c_int {
                max_sig_bits = (*sig_bits).green
            }
            if (max_sig_bits as std::os::raw::c_int) < (*sig_bits).blue as std::os::raw::c_int
               {
                max_sig_bits = (*sig_bits).blue
            }
            (*sig_bits).gray = max_sig_bits
        }
    }
    /* Update the image information. */
    png_set_IHDR(&mut *png_ptr, &mut *info_ptr, width, height, dest_bit_depth, dest_color_type, interlace_type, compression_type, filter_type);
    return reductions;

    }
}

/*
 * Reduce the bit depth of a palette image to the lowest possible value.
 * The parameter reductions should contain OPNG_REDUCE_8_TO_4_2_1.
 * The function returns OPNG_REDUCE_8_TO_4_2_1 if successful.
 */
fn opng_reduce_palette_bits(png_ptr: *mut png_struct_def, 
                            info_ptr: *mut png_info_def, 
                            reductions: u32) -> u32 {
    unsafe {
         let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut src_sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut dest_sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut src_bit_depth: std::os::raw::c_int = 0;
    let mut dest_bit_depth: std::os::raw::c_int = 0;
    let mut src_mask_init: std::os::raw::c_uint = 0;
    let mut src_mask: std::os::raw::c_uint = 0;
    let mut src_shift: std::os::raw::c_uint = 0;
    let mut dest_shift: std::os::raw::c_uint = 0;
    let mut sample: std::os::raw::c_uint = 0;
    let mut dest_buf: std::os::raw::c_uint = 0;
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut num_palette: std::os::raw::c_int = 0;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    /* Check if the reduction applies. */
    if reductions & 0x2 as std::os::raw::c_int as std::os::raw::c_uint == 0 {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 &mut width, &mut height, &mut src_bit_depth, &mut color_type,
                 &mut interlace_type, &mut compression_type,
                 &mut filter_type);
    if color_type != 2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if png_get_PLTE(png_ptr as *const png_struct, info_ptr, &mut palette,
                    &mut num_palette) == 0 {
        num_palette = 0 as std::os::raw::c_int
    }
    /* Find the smallest possible bit depth. */
    if num_palette > 16 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    } else {
        if num_palette > 4 as std::os::raw::c_int {
            /* 5 .. 16 entries */
            dest_bit_depth = 4 as std::os::raw::c_int
        } else if num_palette > 2 as std::os::raw::c_int {
            /* 3 or 4 entries */
            dest_bit_depth = 2 as std::os::raw::c_int
        } else {
            /* 1 or 2 entries */
            if num_palette > 0 as std::os::raw::c_int {
            } else {
                __assert_fail(b"num_palette > 0\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              b"opngreduc.c\x00" as *const u8 as
                                  *const std::os::raw::c_char,
                              691 as std::os::raw::c_int as std::os::raw::c_uint,
                              (*::std::mem::transmute::<&[u8; 74],
                                                        &[std::os::raw::c_char; 74]>(b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
            }
            dest_bit_depth = 1 as std::os::raw::c_int
        }
    }
    if src_bit_depth <= dest_bit_depth {
        if src_bit_depth == dest_bit_depth {
        } else {
            __assert_fail(b"src_bit_depth == dest_bit_depth\x00" as *const u8
                              as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          697 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 74],
                                                    &[std::os::raw::c_char; 74]>(b"png_uint_32 opng_reduce_palette_bits(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        return 0 as std::os::raw::c_int as png_uint_32
    }
    /* Iterate through all sample values. */
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    if src_bit_depth == 8 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
            dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                dest_shift =
                    dest_shift.wrapping_sub(dest_bit_depth as std::os::raw::c_uint);
                if dest_shift > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    dest_buf |=
                        ((*src_sample_ptr as std::os::raw::c_int) << dest_shift) as
                            std::os::raw::c_uint
                } else {
                    let fresh0 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh0 =
                        (dest_buf | *src_sample_ptr as std::os::raw::c_uint) as
                            png_byte;
                    dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint
                }
                src_sample_ptr = src_sample_ptr.offset(1);
                j = j.wrapping_add(1)
            }
            if dest_shift != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                *dest_sample_ptr = dest_buf as png_byte
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    } else {
        /* src_bit_depth < 8 */
        src_mask_init =
            (((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int + src_bit_depth) -
                 ((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int)) as std::os::raw::c_uint;
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            dest_sample_ptr = *row_ptr;
            src_sample_ptr = dest_sample_ptr;
            dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
            src_shift = dest_shift;
            src_mask = src_mask_init;
            dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint;
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                src_shift =
                    src_shift.wrapping_sub(src_bit_depth as std::os::raw::c_uint);
                src_mask >>= src_bit_depth;
                sample =
                    (*src_sample_ptr as std::os::raw::c_uint & src_mask) >> src_shift;
                dest_shift =
                    dest_shift.wrapping_sub(dest_bit_depth as std::os::raw::c_uint);
                if dest_shift > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    dest_buf |= sample << dest_shift
                } else {
                    let fresh1 = dest_sample_ptr;
                    dest_sample_ptr = dest_sample_ptr.offset(1);
                    *fresh1 = (dest_buf | sample) as png_byte;
                    dest_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    dest_buf = 0 as std::os::raw::c_int as std::os::raw::c_uint
                }
                if src_shift == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    src_shift = 8 as std::os::raw::c_int as std::os::raw::c_uint;
                    src_mask = src_mask_init;
                    src_sample_ptr = src_sample_ptr.offset(1)
                }
                j = j.wrapping_add(1)
            }
            if dest_shift != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                *dest_sample_ptr = dest_buf as png_byte
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    }
    /* Update the image information. */
    png_set_IHDR(&mut *png_ptr, &mut *info_ptr, width, height, dest_bit_depth, color_type, interlace_type, compression_type, filter_type);
    return 0x2 as std::os::raw::c_int as png_uint_32;

    }
}

/*
 * Reduce the image type from grayscale(+alpha) or RGB(+alpha) to palette,
 * if possible.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */
unsafe extern "C" fn opng_reduce_to_palette(mut png_ptr: png_structp,
                                            mut info_ptr: png_infop,
                                            mut reductions: png_uint_32)
 -> png_uint_32 {
    let mut result: png_uint_32 = 0; /* nothing is done in this case */
    let mut row_info: png_row_info =
        png_row_info{width: 0,
                     rowbytes: 0,
                     color_type: 0,
                     bit_depth: 0,
                     channels: 0,
                     pixel_depth: 0,}; /* not used */
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte; /* not used */
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut alpha_row: png_bytep = 0 as *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut width: png_uint_32 = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut src_bit_depth: std::os::raw::c_int = 0;
    let mut dest_bit_depth: std::os::raw::c_int = 0;
    let mut channels: std::os::raw::c_int = 0;
    let mut palette: [png_color; 256] =
        [png_color{red: 0, green: 0, blue: 0,}; 256];
    let mut trans_alpha: [png_byte; 256] = [0; 256];
    let mut trans_color: png_color_16p = 0 as *mut png_color_16;
    let mut num_palette: std::os::raw::c_int = 0;
    let mut num_trans: std::os::raw::c_int = 0;
    let mut index: std::os::raw::c_int = 0;
    let mut gray: std::os::raw::c_uint = 0;
    let mut red: std::os::raw::c_uint = 0;
    let mut green: std::os::raw::c_uint = 0;
    let mut blue: std::os::raw::c_uint = 0;
    let mut alpha: std::os::raw::c_uint = 0;
    let mut prev_gray: std::os::raw::c_uint = 0;
    let mut prev_red: std::os::raw::c_uint = 0;
    let mut prev_green: std::os::raw::c_uint = 0;
    let mut prev_blue: std::os::raw::c_uint = 0;
    let mut prev_alpha: std::os::raw::c_uint = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 &mut width, &mut height, &mut src_bit_depth, &mut color_type,
                 &mut interlace_type, &mut compression_type,
                 &mut filter_type);
    if src_bit_depth != 8 as std::os::raw::c_int {
        return 0 as std::os::raw::c_int as png_uint_32
    }
    if color_type & 1 as std::os::raw::c_int == 0 {
    } else {
        __assert_fail(b"!(color_type & 1)\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      802 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 72],
                                                &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    let channels = png_get_channels(&*png_ptr, &*info_ptr) as std::os::raw::c_int;
    alpha_row = png_malloc(png_ptr as *const png_struct, width as png_alloc_size_t) as *mut png_byte;
    row_info.width = width;
    row_info.rowbytes = 0 as std::os::raw::c_int as png_size_t;
    row_info.color_type = color_type as png_byte;
    row_info.bit_depth = src_bit_depth as png_byte;
    row_info.channels = channels as png_byte;
    row_info.pixel_depth = 0 as std::os::raw::c_int as png_byte;
    /* Analyze the possibility of this reduction. */
    num_trans = 0 as std::os::raw::c_int;
    num_palette = num_trans;
    trans_color = 0 as png_color_16p;
    png_get_tRNS(png_ptr as *const png_struct, info_ptr, 0 as *mut png_bytep,
                 0 as *mut std::os::raw::c_int, &mut trans_color);
    prev_alpha = 256 as std::os::raw::c_int as std::os::raw::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    prev_gray = prev_red;
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(&mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & 2 as std::os::raw::c_int != 0 {
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                red =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                green =
                    *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                blue =
                    *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if red != prev_red || green != prev_green || blue != prev_blue
                       || alpha != prev_alpha {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 &mut num_palette,
                                                 trans_alpha.as_mut_ptr(),
                                                 &mut num_trans,
                                                 256 as std::os::raw::c_int, red,
                                                 green, blue, alpha,
                                                 &mut index) <
                           0 as std::os::raw::c_int {
                        /* overflow */
                        if num_palette < 0 as std::os::raw::c_int {
                        } else {
                            __assert_fail(b"num_palette < 0\x00" as *const u8
                                              as *const std::os::raw::c_char,
                                          b"opngreduc.c\x00" as *const u8 as
                                              *const std::os::raw::c_char,
                                          844 as std::os::raw::c_int as std::os::raw::c_uint,
                                          (*::std::mem::transmute::<&[u8; 72],
                                                                    &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr()); /* forced exit from outer loop */
                        }
                        i = height;
                        break ;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        } else {
            /* grayscale */
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                gray =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 &mut num_palette,
                                                 trans_alpha.as_mut_ptr(),
                                                 &mut num_trans,
                                                 256 as std::os::raw::c_int, gray,
                                                 gray, gray, alpha,
                                                 &mut index) <
                           0 as std::os::raw::c_int {
                        /* overflow */
                        if num_palette < 0 as std::os::raw::c_int {
                        } else {
                            __assert_fail(b"num_palette < 0\x00" as *const u8
                                              as *const std::os::raw::c_char,
                                          b"opngreduc.c\x00" as *const u8 as
                                              *const std::os::raw::c_char,
                                          866 as std::os::raw::c_int as std::os::raw::c_uint,
                                          (*::std::mem::transmute::<&[u8; 72],
                                                                    &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr()); /* forced exit from outer loop */
                        }
                        i = height;
                        break ;
                    }
                }
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    if num_palette >= 0 as std::os::raw::c_int &&
           png_get_bKGD(png_ptr as *const png_struct, info_ptr,
                        &mut background) != 0 {
        /* bKGD has an alpha-agnostic palette entry. */
        if color_type & 2 as std::os::raw::c_int != 0 {
            red = (*background).red as std::os::raw::c_uint;
            green = (*background).green as std::os::raw::c_uint;
            blue = (*background).blue as std::os::raw::c_uint
        } else {
            blue = (*background).gray as std::os::raw::c_uint;
            green = blue;
            red = green
        }
        opng_insert_palette_entry(palette.as_mut_ptr(), &mut num_palette,
                                  trans_alpha.as_mut_ptr(), &mut num_trans,
                                  256 as std::os::raw::c_int, red, green, blue,
                                  256 as std::os::raw::c_int as std::os::raw::c_uint,
                                  &mut index);
        if index >= 0 as std::os::raw::c_int {
            (*background).index = index as png_byte
        }
    }
    /* Continue only if the uncompressed indexed image (pixels + PLTE + tRNS)
    * is smaller than the uncompressed RGB(A) image.
    * Casual overhead (headers, CRCs, etc.) is ignored.
    *
    * Compare:
    * num_pixels * (src_bit_depth * channels - dest_bit_depth) / 8
    * vs.
    * sizeof(PLTE) + sizeof(tRNS)
    */
    if num_palette >= 0 as std::os::raw::c_int {
        if num_palette > 0 as std::os::raw::c_int && num_palette <= 256 as std::os::raw::c_int
           {
        } else {
            __assert_fail(b"num_palette > 0 && num_palette <= 256\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          905 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 72],
                                                    &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        if num_trans >= 0 as std::os::raw::c_int && num_trans <= num_palette {
        } else {
            __assert_fail(b"num_trans >= 0 && num_trans <= num_palette\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          906 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 72],
                                                    &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        if num_palette <= 2 as std::os::raw::c_int {
            dest_bit_depth = 1 as std::os::raw::c_int
        } else if num_palette <= 4 as std::os::raw::c_int {
            dest_bit_depth = 2 as std::os::raw::c_int
        } else if num_palette <= 16 as std::os::raw::c_int {
            dest_bit_depth = 4 as std::os::raw::c_int
        } else { dest_bit_depth = 8 as std::os::raw::c_int }
        /* Do the comparison in a way that does not cause overflow. */
        if channels * 8 as std::os::raw::c_int == dest_bit_depth ||
               (((3 as std::os::raw::c_int * num_palette + num_trans) *
                     8 as std::os::raw::c_int /
                     (channels * 8 as std::os::raw::c_int - dest_bit_depth)) as
                    std::os::raw::c_uint).wrapping_div(width).wrapping_div(height) >=
                   1 as std::os::raw::c_int as std::os::raw::c_uint {
            num_palette = -(1 as std::os::raw::c_int)
        }
    }
    if num_palette < 0 as std::os::raw::c_int {
        /* can't reduce */
        png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
        return 0 as std::os::raw::c_int as png_uint_32
    }
    /* Reduce. */
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    index = -(1 as std::os::raw::c_int);
    prev_alpha = -(1 as std::os::raw::c_int) as std::os::raw::c_uint;
    prev_blue = prev_alpha;
    prev_green = prev_blue;
    prev_red = prev_green;
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        sample_ptr = *row_ptr;
        opng_get_alpha_row(&mut row_info, trans_color, *row_ptr, alpha_row);
        if color_type & 2 as std::os::raw::c_int != 0 {
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                red =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                green =
                    *sample_ptr.offset(1 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                blue =
                    *sample_ptr.offset(2 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if red != prev_red || green != prev_green || blue != prev_blue
                       || alpha != prev_alpha {
                    prev_red = red;
                    prev_green = green;
                    prev_blue = blue;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 &mut num_palette,
                                                 trans_alpha.as_mut_ptr(),
                                                 &mut num_trans,
                                                 256 as std::os::raw::c_int, red,
                                                 green, blue, alpha,
                                                 &mut index) !=
                           0 as std::os::raw::c_int {
                        index = -(1 as std::os::raw::c_int)
                    }
                    /* this should not happen */
                }
                if index >= 0 as std::os::raw::c_int {
                } else {
                    __assert_fail(b"index >= 0\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"opngreduc.c\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  957 as std::os::raw::c_int as std::os::raw::c_uint,
                                  (*::std::mem::transmute::<&[u8; 72],
                                                            &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
                }
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        } else {
            /* grayscale */
            j = 0 as std::os::raw::c_int as png_uint_32;
            while j < width {
                gray =
                    *sample_ptr.offset(0 as std::os::raw::c_int as isize) as
                        std::os::raw::c_uint;
                alpha = *alpha_row.offset(j as isize) as std::os::raw::c_uint;
                /* Check the cache first. */
                if gray != prev_gray || alpha != prev_alpha {
                    prev_gray = gray;
                    prev_alpha = alpha;
                    if opng_insert_palette_entry(palette.as_mut_ptr(),
                                                 &mut num_palette,
                                                 trans_alpha.as_mut_ptr(),
                                                 &mut num_trans,
                                                 256 as std::os::raw::c_int, gray,
                                                 gray, gray, alpha,
                                                 &mut index) !=
                           0 as std::os::raw::c_int {
                        index = -(1 as std::os::raw::c_int)
                    }
                    /* this should not happen */
                }
                if index >= 0 as std::os::raw::c_int {
                } else {
                    __assert_fail(b"index >= 0\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  b"opngreduc.c\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  977 as std::os::raw::c_int as std::os::raw::c_uint,
                                  (*::std::mem::transmute::<&[u8; 72],
                                                            &[std::os::raw::c_char; 72]>(b"png_uint_32 opng_reduce_to_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
                }
                *(*row_ptr).offset(j as isize) = index as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(channels as isize)
            }
        }
        i = i.wrapping_add(1);
        row_ptr = row_ptr.offset(1)
    }
    /* Update the image information. */
    png_set_IHDR(&mut *png_ptr, &mut *info_ptr, width, height, 8, 2 | 1, interlace_type, compression_type, filter_type);
    let palette_slice = &mut palette[..num_palette as usize];
png_set_PLTE(png_ptr, info_ptr, palette_slice.as_mut_ptr() as png_const_colorp, num_palette);

if num_trans > 0 {
    let trans_alpha_slice = &mut trans_alpha[..num_trans as usize];
    png_set_tRNS(png_ptr, info_ptr, trans_alpha_slice.as_mut_ptr() as png_const_bytep, num_trans, 0 as png_const_color_16p);
}

/* bKGD (if present) is automatically updated. */
png_free(png_ptr as *const png_struct, alpha_row as png_voidp);
result = 0x10;

if reductions & 0x2 != 0 {
    result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions);
}

result

}
/*
 * Analyze the usage of samples.
 * The output value usage_map[n] indicates whether the sample n
 * is used. The usage_map[] array must have 256 entries.
 * The function requires a valid bit depth between 1 and 8.
 */
unsafe extern "C" fn opng_analyze_sample_usage(mut png_ptr: png_structp,
                                               mut info_ptr: png_infop,
                                               mut usage_map: png_bytep) {
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut sample_ptr: png_bytep = 0 as *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut init_shift: std::os::raw::c_int = 0;
    let mut init_mask: std::os::raw::c_int = 0;
    let mut shift: std::os::raw::c_int = 0;
    let mut mask: std::os::raw::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    height =
        png_get_image_height(png_ptr as *const png_struct,
                             info_ptr as *const png_info);
    let width = png_get_image_width(Some(&*png_ptr), Some(&*info_ptr));
    bit_depth = png_get_bit_depth(png_ptr as *const png_struct, info_ptr as *const png_info) as std::os::raw::c_int;
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    /* Initialize the output entries with 0. */
    memset(usage_map as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           256 as std::os::raw::c_int as std::os::raw::c_ulong);
    /* Iterate through all sample values. */
    if bit_depth == 8 as std::os::raw::c_int {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            j = 0 as std::os::raw::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                *usage_map.offset(*sample_ptr as isize) =
                    1 as std::os::raw::c_int as png_byte;
                j = j.wrapping_add(1);
                sample_ptr = sample_ptr.offset(1)
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    } else {
        if bit_depth < 8 as std::os::raw::c_int {
        } else {
            __assert_fail(b"bit_depth < 8\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          1039 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 66],
                                                    &[std::os::raw::c_char; 66]>(b"void opng_analyze_sample_usage(png_structp, png_infop, png_bytep)\x00")).as_ptr());
        }
        init_shift = 8 as std::os::raw::c_int - bit_depth;
        init_mask =
            ((1 as std::os::raw::c_int) << 8 as std::os::raw::c_int) -
                ((1 as std::os::raw::c_int) << init_shift);
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < height {
            j = 0 as std::os::raw::c_int as png_uint_32;
            sample_ptr = *row_ptr;
            while j < width {
                mask = init_mask;
                shift = init_shift;
                loop  {
                    *usage_map.offset(((*sample_ptr as std::os::raw::c_int & mask) >>
                                           shift) as isize) =
                        1 as std::os::raw::c_int as png_byte;
                    mask >>= bit_depth;
                    shift -= bit_depth;
                    j = j.wrapping_add(1);
                    if !(mask > 0 as std::os::raw::c_int && j < width) { break ; }
                }
                sample_ptr = sample_ptr.offset(1)
            }
            i = i.wrapping_add(1);
            row_ptr = row_ptr.offset(1)
        }
    }
    /* bKGD also counts as a used sample. */
    if png_get_bKGD(png_ptr as *const png_struct, info_ptr, &mut background)
           != 0 {
        *usage_map.offset((*background).index as isize) =
            1 as std::os::raw::c_int as png_byte
    };
}
/*
 * Reduce the palette. (Only the fast method is implemented.)
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */
unsafe extern "C" fn opng_reduce_palette(mut png_ptr: png_structp,
                                         mut info_ptr: png_infop,
                                         mut reductions: png_uint_32)
 -> png_uint_32 {
    let mut result: png_uint_32 = 0;
    let mut palette: png_colorp = 0 as *mut png_color;
    let mut trans_alpha: png_bytep = 0 as *mut png_byte;
    let mut row_ptr: png_bytepp = 0 as *mut *mut png_byte;
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut num_palette: std::os::raw::c_int = 0;
    let mut num_trans: std::os::raw::c_int = 0;
    let mut last_color_index: std::os::raw::c_int = 0;
    let mut last_trans_index: std::os::raw::c_int = 0;
    let mut crt_trans_value: png_byte = 0;
    let mut last_trans_value: png_byte = 0;
    let mut is_used: [png_byte; 256] = [0; 256];
    let mut gray_trans: png_color_16 =
        png_color_16{index: 0, red: 0, green: 0, blue: 0, gray: 0,};
    let mut is_gray: std::os::raw::c_int = 0;
    let mut background: png_color_16p = 0 as *mut png_color_16;
    let mut hist: png_uint_16p = 0 as *mut png_uint_16;
    let mut sig_bits: png_color_8p = 0 as *mut png_color_8;
    let mut i: png_uint_32 = 0;
    let mut j: png_uint_32 = 0;
    let mut k: std::os::raw::c_int = 0;
    result = 0 as std::os::raw::c_int as png_uint_32;
    png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                 &mut width, &mut height, &mut bit_depth, &mut color_type,
                 &mut interlace_type, &mut compression_type,
                 &mut filter_type);
    let mut row_ptr = png_get_rows(png_ptr, info_ptr);
    if png_get_PLTE(png_ptr as *const png_struct, info_ptr, &mut palette,
                    &mut num_palette) == 0 {
        palette = 0 as png_colorp;
        num_palette = 0 as std::os::raw::c_int
    }
    if png_get_tRNS(png_ptr as *const png_struct, info_ptr, &mut trans_alpha,
                    &mut num_trans, 0 as *mut png_color_16p) == 0 {
        trans_alpha = 0 as png_bytep;
        num_trans = 0 as std::os::raw::c_int
    } else if !trans_alpha.is_null() && num_trans > 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"trans_alpha != ((void*)0) && num_trans > 0\x00" as
                          *const u8 as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1117 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[std::os::raw::c_char; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    opng_analyze_sample_usage(png_ptr, info_ptr, is_used.as_mut_ptr());
    /* Palette-to-gray does not work (yet) if the bit depth is below 8. */
    is_gray =
        (reductions & 0x80 as std::os::raw::c_int as std::os::raw::c_uint != 0 &&
             bit_depth == 8 as std::os::raw::c_int) as std::os::raw::c_int;
    last_trans_index = -(1 as std::os::raw::c_int);
    last_color_index = last_trans_index;
    k = 0 as std::os::raw::c_int;
    while k < 256 as std::os::raw::c_int {
        if !(is_used[k as usize] == 0) {
            last_color_index = k;
            if k < num_trans &&
                   (*trans_alpha.offset(k as isize) as std::os::raw::c_int) <
                       255 as std::os::raw::c_int {
                last_trans_index = k
            }
            if is_gray != 0 {
                if (*palette.offset(k as isize)).red as std::os::raw::c_int !=
                       (*palette.offset(k as isize)).green as std::os::raw::c_int ||
                       (*palette.offset(k as isize)).red as std::os::raw::c_int !=
                           (*palette.offset(k as isize)).blue as std::os::raw::c_int {
                    is_gray = 0 as std::os::raw::c_int
                }
            }
        }
        k += 1
    }
    if last_color_index >= 0 as std::os::raw::c_int {
    } else {
        __assert_fail(b"last_color_index >= 0\x00" as *const u8 as
                          *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1135 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[std::os::raw::c_char; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    if last_color_index >= last_trans_index {
    } else {
        __assert_fail(b"last_color_index >= last_trans_index\x00" as *const u8
                          as *const std::os::raw::c_char,
                      b"opngreduc.c\x00" as *const u8 as *const std::os::raw::c_char,
                      1136 as std::os::raw::c_int as std::os::raw::c_uint,
                      (*::std::mem::transmute::<&[u8; 69],
                                                &[std::os::raw::c_char; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
    }
    /* Check the integrity of PLTE and tRNS. */
    if last_color_index >= num_palette {
        png_warning(png_ptr as *const png_struct,
                    b"Too few colors in PLTE\x00" as *const u8 as
                        *const std::os::raw::c_char);
        /* Fix the palette by adding blank entries at the end. */
        let new_num_palette = last_color_index + 1;
opng_realloc_PLTE(png_ptr, info_ptr, new_num_palette);
        png_get_PLTE(png_ptr as *const png_struct, info_ptr, &mut palette,
                     &mut num_palette);
        if num_palette == last_color_index + 1 as std::os::raw::c_int {
        } else {
            __assert_fail(b"num_palette == last_color_index + 1\x00" as
                              *const u8 as *const std::os::raw::c_char,
                          b"opngreduc.c\x00" as *const u8 as
                              *const std::os::raw::c_char,
                          1145 as std::os::raw::c_int as std::os::raw::c_uint,
                          (*::std::mem::transmute::<&[u8; 69],
                                                    &[std::os::raw::c_char; 69]>(b"png_uint_32 opng_reduce_palette(png_structp, png_infop, png_uint_32)\x00")).as_ptr());
        }
        result |= 0x2000 as std::os::raw::c_int as std::os::raw::c_uint
    }
    if num_trans > num_palette {
        png_warning(png_ptr as *const png_struct,
                    b"Too many alpha values in tRNS\x00" as *const u8 as
                        *const std::os::raw::c_char);
        /* Transparency will be fixed further below. */
        result |= 0x2000 as std::os::raw::c_int as std::os::raw::c_uint
    }
    /* Check if tRNS can be reduced to grayscale. */
    if is_gray != 0 && last_trans_index >= 0 {
    gray_trans.gray = unsafe { (*palette.offset(last_trans_index as isize)).red } as png_uint_16;
    last_trans_value = unsafe { *trans_alpha.offset(last_trans_index as isize) };
    let mut k = 0;
    while k <= last_color_index {
        if is_used[k as usize] != 0 {
            let crt_trans_value = if k <= last_trans_index {
                unsafe { *trans_alpha.offset(k as isize) }
            } else {
                255
            };
            if (crt_trans_value as i32) < 255 && unsafe { (*palette.offset(k as isize)).red } as i32 != gray_trans.gray as i32 {
                is_gray = 0;
                break;
            }
            if unsafe { (*palette.offset(k as isize)).red } as i32 == gray_trans.gray as i32 && crt_trans_value as i32 != last_trans_value as i32 {
                is_gray = 0;
                break;
            }
        }
        k += 1;
    }
}

/* Remove tRNS if it is entirely sterile. */
if num_trans > 0 && last_trans_index < 0 {
    num_trans = 0;
    png_free_data(png_ptr, info_ptr, 0x2000, -1);
    png_set_invalid(png_ptr, info_ptr, 0x10);
    result |= 0x200;
}

if reductions & 0x200 != 0 {
    if num_palette != last_color_index + 1 {
        /* Reduce PLTE. */
        opng_realloc_PLTE(png_ptr, info_ptr, last_color_index + 1);
        png_get_PLTE(png_ptr, info_ptr, &mut palette, &mut num_palette);
        assert!(num_palette == last_color_index + 1, "num_palette == last_color_index + 1");
        result |= 0x200;
    }
    if num_trans > 0 && num_trans != last_trans_index + 1 {
        /* Reduce tRNS. */
        opng_realloc_tRNS(png_ptr, info_ptr, last_trans_index + 1);
        png_get_tRNS(png_ptr, info_ptr, &mut trans_alpha, &mut num_trans, std::ptr::null_mut());
        assert!(num_trans == last_trans_index + 1, "num_trans == last_trans_index + 1");
        result |= 0x200;
    }
}

if reductions & 0x2 != 0 {
    result |= opng_reduce_palette_bits(png_ptr, info_ptr, reductions);
    /* Refresh the image information. */
    bit_depth = png_get_bit_depth(png_ptr, info_ptr) as i32;
}

if bit_depth < 8 || is_gray == 0 {
    return result;
}

/* Reduce palette --> grayscale. */
for i in 0..height {
    for j in 0..width {
        let index = unsafe { *(*row_ptr.offset(i as isize)).offset(j as isize) };
        unsafe { *(*row_ptr.offset(i as isize)).offset(j as isize) = (*palette.offset(index as isize)).red };
    }
}

/* Update the ancillary information. */
if num_trans > 0 {
    png_set_tRNS(png_ptr, info_ptr, std::ptr::null(), 0, &mut gray_trans as *mut png_color_16 as png_const_color_16p);
}

if png_get_bKGD(png_ptr, info_ptr, &mut background) != 0 {
    unsafe { (*background).gray = (*palette.offset((*background).index as isize)).red as png_uint_16 };
}

if png_get_hIST(png_ptr, info_ptr, &mut hist) != 0 {
    png_free_data(png_ptr, info_ptr, 0x8, -1);
    png_set_invalid(png_ptr, info_ptr, 0x40);
}

if png_get_sBIT(png_ptr, info_ptr, &mut sig_bits) != 0 {
    let mut max_sig_bits: png_byte = unsafe { (*sig_bits).red };
    max_sig_bits = max_sig_bits.max(unsafe { (*sig_bits).green }).max(unsafe { (*sig_bits).blue });
    unsafe { (*sig_bits).gray = max_sig_bits };
}

/* Update the image information. */
png_set_IHDR(unsafe { &mut *png_ptr }, unsafe { &mut *info_ptr }, width, height, bit_depth, 0, interlace_type, compression_type, filter_type);
png_free_data(png_ptr, info_ptr, 0x1000, -1);
png_set_invalid(png_ptr, info_ptr, 0x8);
return 0x80;

    /* ignore the former result */
}
/*
 * Reduce the image (bit depth + color type + palette) without
 * losing any information. The palette (if applicable) and the
 * image data must be present, e.g., by calling png_set_rows(),
 * or by loading IDAT.
 * The parameter reductions indicates the intended reductions.
 * The function returns the successful reductions.
 */

pub fn opng_reduce_image(png_ptr: &mut png_struct_def, info_ptr: &mut png_info_def, reductions: u32) -> u32 {
    let mut result: u32 = 0;
    let color_type: u8;

    unsafe {
        if opng_validate_image(png_ptr, info_ptr) == 0 {
            png_warning(png_ptr, b"Image reduction requires the presence of all critical information\x00" as *const u8 as *const i8);
            return 0;
        }

        color_type = png_get_color_type(png_ptr, info_ptr);

        // The reductions below must be applied in this particular order.
        // Try to reduce the high bits and color/alpha channels.
        result = opng_reduce_bits(png_ptr, info_ptr, reductions);

        // Try to reduce the palette image.
        if (color_type == (2 | 4) && // Assuming 2 is PNG_COLOR_TYPE_RGB and 4 is PNG_COLOR_TYPE_PALETTE
            (reductions & (0x80 | 0x200 | 0x2)) != 0) {
            result |= opng_reduce_palette(png_ptr, info_ptr, reductions);
        }

        // Try to reduce RGB to palette or grayscale to palette.
        if ((color_type & !4) != 0 && // Assuming 4 is PNG_COLOR_TYPE_ALPHA
            (reductions & 0x40) != 0 || 
            (color_type & !4) == 2 && // Assuming 2 is PNG_COLOR_TYPE_RGB
            (reductions & 0x10) != 0) {
            if (result & 0x80 == 0) {
                result |= opng_reduce_to_palette(png_ptr, info_ptr, reductions);
            }
        }
    }

    result
}

/* OPNG_IMAGE_REDUCTIONS_SUPPORTED */
