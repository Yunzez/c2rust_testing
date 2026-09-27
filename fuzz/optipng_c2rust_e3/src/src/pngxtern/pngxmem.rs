
extern "C" {
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn memset(_: *mut std::os::raw::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
}
pub use crate::src::libpng::png::png_free_data;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngget::png_get_image_height;
pub use crate::src::libpng::pngget::png_get_rowbytes;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngset::png_set_rows;
pub use crate::src::libpng::png::png_info_def;
pub use crate::src::libpng::png::png_struct_def;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type png_byte = crate::src::libpng::png::png_byte;
pub type png_uint_32 = crate::src::libpng::png::png_uint_32;
pub type png_size_t = crate::src::libpng::png::png_size_t;
pub type png_alloc_size_t = crate::src::libpng::png::png_alloc_size_t;
pub type png_voidp = crate::src::libpng::png::png_voidp;
pub type png_bytep = crate::src::libpng::png::png_bytep;
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
pub type png_bytepp = crate::src::libpng::png::png_bytepp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infop = crate::src::libpng::png::png_infop;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_inforp = crate::src::libpng::pngget::png_const_inforp;
pub type pngx_alloc_size_t = png_alloc_size_t;
/*
 * pngxmem.c - libpng extension: memory allocation utilities.
 *
 * Copyright (C) 2003-2017 Cosmin Truta.
 * This software is distributed under the same licensing and warranty terms
 * as libpng.
 */
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows(mut png_ptr: png_structp,
                                          mut info_ptr: png_infop,
                                          mut filler: std::os::raw::c_int)
 -> png_bytepp {
    return pngx_malloc_rows_extended(png_ptr, info_ptr,
                                     0 as std::os::raw::c_int as pngx_alloc_size_t,
                                     filler);
}
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows_extended(mut png_ptr: png_structp,
                                                   mut info_ptr: png_infop,
                                                   mut min_row_size:
                                                       pngx_alloc_size_t,
                                                   mut filler: std::os::raw::c_int)
 -> png_bytepp {
    let mut row_size: pngx_alloc_size_t = 0;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut rows: png_bytepp = 0 as *mut *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    /* Check the image dimensions and calculate the row size. */
    height =
        png_get_image_height(png_ptr as *const png_struct,
                             info_ptr as *const png_info);
    if height == 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr as *const png_struct,
                  b"Missing IHDR\x00" as *const u8 as *const std::os::raw::c_char);
    }
    row_size =
        png_get_rowbytes(png_ptr as *const png_struct,
                         info_ptr as *const png_info);
    /* libpng sets row_size to 0 when the width is too large to process. */
    if row_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           height as pngx_alloc_size_t >
               (-(1 as std::os::raw::c_int) as
                    pngx_alloc_size_t).wrapping_div(::std::mem::size_of::<png_bytep>()
                                                        as std::os::raw::c_ulong) {
        png_error(png_ptr as *const png_struct,
                  b"Can\'t handle exceedingly large image dimensions\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    if row_size < min_row_size { row_size = min_row_size }
    /* Deallocate the currently-existing rows. */
    png_free_data(png_ptr as *const png_struct, info_ptr,
                  0x40 as std::os::raw::c_uint, 0 as std::os::raw::c_int);
    /* Allocate memory for the row index. */
    rows =
        png_malloc(png_ptr as *const png_struct,
                   (height as
                        std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_bytep>()
                                                        as std::os::raw::c_ulong)) as
            png_bytepp;
    if rows.is_null() { return 0 as png_bytepp }
    /* Allocate memory for each row. */
    i = 0 as std::os::raw::c_int as png_uint_32;
    while i < height {
        row = png_malloc(png_ptr as *const png_struct, row_size) as png_bytep;
        if row.is_null() {
            /* Release the memory allocated up to the point of failure. */
            while i > 0 as std::os::raw::c_int as std::os::raw::c_uint {
                i = i.wrapping_sub(1);
                png_free(png_ptr as *const png_struct,
                         *rows.offset(i as isize) as png_voidp);
            }
            png_free(png_ptr as *const png_struct, rows as png_voidp);
            return 0 as png_bytepp
        }
        if filler >= 0 as std::os::raw::c_int {
            memset(row as *mut std::os::raw::c_void, filler, row_size);
        }
        let ref mut fresh0 = *rows.offset(i as isize);
        *fresh0 = row;
        i = i.wrapping_add(1)
    }
    /* Set the row pointers. */
    png_set_rows(png_ptr as *const png_struct, info_ptr, rows);
    return rows;
}
/* PNG_INFO_IMAGE_SUPPORTED */
/* not necessary */
