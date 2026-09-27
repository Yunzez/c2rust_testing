
extern "C" {
    
    
    
    
    
    
}
pub use crate::src::libpng::pngget::png_get_IHDR;
pub use crate::src::libpng::pngset::png_set_IHDR;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub type png_uint_32 = std::os::raw::c_uint;
pub type png_struct = crate::src::libpng::png::png_struct_def;
pub type png_structp = * mut crate::src::libpng::png::png_struct_def;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infop = * mut crate::src::libpng::png::png_info_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_const_inforp = * const crate::src::libpng::png::png_info_def;
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
#[no_mangle]
pub unsafe extern "C" fn pngx_set_compression_type(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                   mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                                   mut compression_type:
                                                       std::os::raw::c_int) {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut old_compression_type: i32 = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    Some(&mut width), Some(&mut height), Some(&mut bit_depth), Some(&mut color_type),
                    Some(&mut interlace_type), Some(&mut old_compression_type),
                    Some(&mut filter_type)) == 0 {
        return
    }
    if compression_type == old_compression_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_filter_type(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                              mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                              mut filter_type: std::os::raw::c_int) {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut interlace_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut old_filter_type: i32 = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    Some(&mut width), Some(&mut height), Some(&mut bit_depth), Some(&mut color_type),
                    Some(&mut interlace_type), Some(&mut compression_type),
                    Some(&mut old_filter_type)) == 0 {
        return
    }
    if filter_type == old_filter_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_interlace_type(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                 mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                                 mut interlace_type:
                                                     std::os::raw::c_int) {
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    let mut bit_depth: i32 = 0;
    let mut color_type: i32 = 0;
    let mut compression_type: i32 = 0;
    let mut filter_type: i32 = 0;
    let mut old_interlace_type: i32 = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    Some(&mut width), Some(&mut height), Some(&mut bit_depth), Some(&mut color_type),
                    Some(&mut old_interlace_type), Some(&mut compression_type),
                    Some(&mut filter_type)) == 0 {
        return
    }
    if interlace_type == old_interlace_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
use crate::laertes_rt::*;
