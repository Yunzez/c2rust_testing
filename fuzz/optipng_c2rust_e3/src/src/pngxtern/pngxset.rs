
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
#[no_mangle]
pub unsafe extern "C" fn pngx_set_compression_type(mut png_ptr: png_structp,
                                                   mut info_ptr: png_infop,
                                                   mut compression_type:
                                                       std::os::raw::c_int) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut old_compression_type: std::os::raw::c_int = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    &mut width, &mut height, &mut bit_depth, &mut color_type,
                    &mut interlace_type, &mut old_compression_type,
                    &mut filter_type) == 0 {
        return
    }
    if compression_type == old_compression_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_filter_type(mut png_ptr: png_structp,
                                              mut info_ptr: png_infop,
                                              mut filter_type: std::os::raw::c_int) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut interlace_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut old_filter_type: std::os::raw::c_int = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    &mut width, &mut height, &mut bit_depth, &mut color_type,
                    &mut interlace_type, &mut compression_type,
                    &mut old_filter_type) == 0 {
        return
    }
    if filter_type == old_filter_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_interlace_type(mut png_ptr: png_structp,
                                                 mut info_ptr: png_infop,
                                                 mut interlace_type:
                                                     std::os::raw::c_int) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: std::os::raw::c_int = 0;
    let mut color_type: std::os::raw::c_int = 0;
    let mut compression_type: std::os::raw::c_int = 0;
    let mut filter_type: std::os::raw::c_int = 0;
    let mut old_interlace_type: std::os::raw::c_int = 0;
    if png_get_IHDR(png_ptr as *const png_struct, info_ptr as *const png_info,
                    &mut width, &mut height, &mut bit_depth, &mut color_type,
                    &mut old_interlace_type, &mut compression_type,
                    &mut filter_type) == 0 {
        return
    }
    if interlace_type == old_interlace_type { return }
    png_set_IHDR(png_ptr as *const png_struct, info_ptr, width, height,
                 bit_depth, color_type, interlace_type, compression_type,
                 filter_type);
}
