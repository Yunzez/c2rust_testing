



extern "C" {
    
    
    
    
    
    
}
pub use crate::src::libpng::pngget::png_get_IHDR;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infop = crate::src::libpng::png::png_infop;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_inforp = crate::src::libpng::pngget::png_const_inforp;
/*
 * pngxset.c - libpng extension: additional image info storage.
 *
 * Copyright (C) 2003-2011 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 */
/*
 * NOTE:
 * There is a discrepancy between the parameter names used in
 * this module vs. the PNG specification.
 * The PNG specification uses the terms "compression method",
 * "filter method" and "interlace method", while this module
 * uses the terms, respectively, "compression type", "filter type"
 * and "interlace type", following the libpng naming conventions.
 */

pub fn pngx_set_compression_type(png_ptr: &mut png_struct_def, info_ptr: &mut png_info_def, compression_type: i32) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut old_compression_type: i32 = 0;

    // Use an unsafe block to call the unsafe function
    if unsafe { png_get_IHDR(png_ptr, info_ptr, &mut width, &mut height, &mut bit_depth, &mut color_type, &mut interlace_type, &mut old_compression_type, &mut filter_type) } == 0 {
        return;
    }
    if compression_type == old_compression_type {
        return;
    }
    // Use an unsafe block to call the unsafe function
    unsafe {
        png_set_IHDR(png_ptr, info_ptr, width, height, bit_depth, color_type, interlace_type, compression_type, filter_type);
    }
}


pub fn pngx_set_filter_type(png_ptr: &mut png_struct, info_ptr: &mut png_info, filter_type: i32) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut old_filter_type: i32 = 0;

    unsafe {
        if png_get_IHDR(png_ptr, info_ptr, &mut width, &mut height, &mut bit_depth, &mut color_type, &mut interlace_type, &mut compression_type, &mut old_filter_type) == 0 {
            return;
        }
    }
    
    if filter_type == old_filter_type { return; }
    
    unsafe {
        png_set_IHDR(png_ptr, info_ptr, width, height, bit_depth, color_type, interlace_type, compression_type, filter_type);
    }
}


pub fn pngx_set_interlace_type(png_ptr: &mut png_struct, info_ptr: &mut png_info, interlace_type: i32) {
    let (width, height, bit_depth, color_type, compression_type, filter_type, old_interlace_type) = {
        let mut width = 0;
        let mut height = 0;
        let mut bit_depth = 0;
        let mut color_type = 0;
        let mut compression_type = 0;
        let mut filter_type = 0;
        let mut old_interlace_type = 0;

        unsafe {
            if png_get_IHDR(png_ptr, info_ptr, &mut width, &mut height, &mut bit_depth, &mut color_type, &mut old_interlace_type, &mut compression_type, &mut filter_type) == 0 {
                return;
            }
        }

        (width, height, bit_depth, color_type, compression_type, filter_type, old_interlace_type)
    };

    if interlace_type == old_interlace_type {
        return;
    }

    unsafe {
        png_set_IHDR(png_ptr, info_ptr, width, height, bit_depth, color_type, interlace_type, compression_type, filter_type);
    }
}

