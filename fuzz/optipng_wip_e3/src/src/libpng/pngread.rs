










use std::ptr;
use std::vec::Vec;

extern "C" {
    
    
    
    
    
    
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::png::png_chunk_unknown_handling;
pub use crate::src::libpng::png::png_create_png_struct;
pub use crate::src::libpng::png::png_destroy_info_struct;
pub use crate::src::libpng::png::png_free_data;
pub use crate::src::libpng::png::png_zfree;
pub use crate::src::libpng::pngerror::png_app_error;
pub use crate::src::libpng::pngerror::png_benign_error;
pub use crate::src::libpng::pngerror::png_chunk_benign_error;
pub use crate::src::libpng::pngerror::png_chunk_error;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngmem::png_destroy_png_struct;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngrio::png_set_read_fn;
pub use crate::src::libpng::pngrtran::png_do_read_transformations;
pub use crate::src::libpng::pngrtran::png_read_transform_info;
pub use crate::src::libpng::pngrutil::png_combine_row;
pub use crate::src::libpng::pngrutil::png_crc_finish;
pub use crate::src::libpng::pngrutil::png_do_read_interlace;
pub use crate::src::libpng::pngrutil::png_handle_IEND;
pub use crate::src::libpng::pngrutil::png_handle_IHDR;
pub use crate::src::libpng::pngrutil::png_handle_PLTE;
pub use crate::src::libpng::pngrutil::png_handle_bKGD;
pub use crate::src::libpng::pngrutil::png_handle_hIST;
pub use crate::src::libpng::pngrutil::png_handle_sBIT;
pub use crate::src::libpng::pngrutil::png_handle_tRNS;
pub use crate::src::libpng::pngrutil::png_handle_unknown;
pub use crate::src::libpng::pngrutil::png_read_IDAT_data;
pub use crate::src::libpng::pngrutil::png_read_chunk_header;
pub use crate::src::libpng::pngrutil::png_read_filter_row;
pub use crate::src::libpng::pngrutil::png_read_finish_IDAT;
pub use crate::src::libpng::pngrutil::png_read_finish_row;
pub use crate::src::libpng::pngrutil::png_read_sig;
pub use crate::src::libpng::pngrutil::png_read_start_row;
pub use crate::src::libpng::pngtrans::png_set_interlace_handling;
pub use crate::src::zlib::inflate::inflateEnd;
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
pub type png_structpp = *mut *mut png_struct;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::pngget::png_info_def;
pub type png_unknown_chunkp = crate::src::libpng::png::png_unknown_chunkp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infopp = crate::src::libpng::png::png_infopp;
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_malloc_ptr = crate::src::libpng::png::png_malloc_ptr;
pub type png_free_ptr = crate::src::libpng::png::png_free_ptr;
pub type z_streamp = crate::src::libpng::png::z_streamp;
pub type voidp = *mut std::os::raw::c_void;
/* pngread.c - read a PNG file
 *
 * Last changed in libpng 1.6.33 [September 28, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file contains routines that an application calls directly to
 * read a PNG file or stream.
 */
/* Create a PNG structure for reading, and allocate any memory needed. */

pub fn png_create_read_struct(user_png_ver: &str, error_ptr: *mut std::ffi::c_void, error_fn: Option<unsafe extern "C" fn(png_structp, png_const_charp)>, warn_fn: Option<unsafe extern "C" fn(png_structp, png_const_charp)>) -> *mut png_struct {
    let png_ptr = unsafe {
        png_create_png_struct(
            user_png_ver.as_ptr() as *const i8,
            error_ptr,
            error_fn,
            warn_fn,
            std::ptr::null_mut(),
            None,
            None,
        )
    };

    if !png_ptr.is_null() {
        unsafe {
            (*png_ptr).mode = 0x8000;
            (*png_ptr).IDAT_read_size = 8192;
            (*png_ptr).flags |= 0x100000;
            (*png_ptr).flags |= 0x200000;
            png_set_read_fn(png_ptr, std::ptr::null_mut(), None);
        }
    }

    png_ptr
}

/* Read the information before the actual image data.  This has been
 * changed in v0.90 to allow reading a file that already has the magic
 * bytes read from the stream.  You can tell libpng how many bytes have
 * been read from the beginning of the stream (up to the maximum of 8)
 * via png_set_sig_bytes(), and we will only check the remaining bytes
 * here.  The application can then have access to the signature bytes we
 * read if it is determined that this isn't a valid PNG file.
 */

pub unsafe extern "C" fn png_read_info(mut png_ptr: png_structrp,
                                       mut info_ptr: png_inforp) {
    let mut keep: std::os::raw::c_int = 0;
    if png_ptr.is_null() || info_ptr.is_null() { return }
    /* Read and check the PNG file signature. */
    png_read_sig(png_ptr, info_ptr);
    loop  {
        let length = png_read_chunk_header(png_ptr);
let chunk_name = unsafe { (*png_ptr).chunk_name };

if chunk_name == (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32) {
    if unsafe { (*png_ptr).mode & 0x1 } == 0 {
        png_chunk_error(png_ptr, b"Missing IHDR before IDAT\x00" as *const u8 as *const i8);
    } else if (unsafe { (*png_ptr).color_type } == 2 || unsafe { (*png_ptr).color_type } == 1) && (unsafe { (*png_ptr).mode & 0x2 } == 0) {
        png_chunk_error(png_ptr, b"Missing PLTE before IDAT\x00" as *const u8 as *const i8);
    } else if unsafe { (*png_ptr).mode & 0x8 } != 0 {
        png_chunk_benign_error(png_ptr, b"Too many IDATs found\x00" as *const u8 as *const i8);
    }
    unsafe { (*png_ptr).mode |= 0x4 };
} else if unsafe { (*png_ptr).mode & 0x4 } != 0 {
    unsafe { (*png_ptr).mode |= 0x2000 };
    unsafe { (*png_ptr).mode |= 0x8 };
}

if chunk_name == (73u32 << 24) | (72u32 << 16) | (68u32 << 8) | (82u32) {
    png_handle_IHDR(png_ptr, info_ptr, length);
} else if chunk_name == (73u32 << 24) | (69u32 << 16) | (78u32 << 8) | (68u32) {
    png_handle_IEND(png_ptr, info_ptr, length);
} else {
    keep = png_chunk_unknown_handling(png_ptr, chunk_name);
    if keep != 0 {
        png_handle_unknown(png_ptr, info_ptr, length, keep);
        if chunk_name == (80u32 << 24) | (76u32 << 16) | (84u32 << 8) | (69u32) {
            unsafe { (*png_ptr).mode |= 0x2 };
        } else {
            if !(chunk_name == (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32)) {
                continue;
            }
            unsafe { (*png_ptr).idat_size = 0 };
            break;
        }
    } else if chunk_name == (80u32 << 24) | (76u32 << 16) | (84u32 << 8) | (69u32) {
        png_handle_PLTE(png_ptr, info_ptr, length);
    } else if chunk_name == (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32) {
        unsafe { (*png_ptr).idat_size = length };
        break;
    } else if chunk_name == (98u32 << 24) | (75u32 << 16) | (71u32 << 8) | (68u32) {
        png_handle_bKGD(png_ptr, info_ptr, length);
    } else if chunk_name == (104u32 << 24) | (73u32 << 16) | (83u32 << 8) | (84u32) {
        png_handle_hIST(png_ptr, info_ptr, length);
    } else if chunk_name == (115u32 << 24) | (66u32 << 16) | (73u32 << 8) | (84u32) {
        png_handle_sBIT(png_ptr, info_ptr, length);
    } else if chunk_name == (116u32 << 24) | (82u32 << 16) | (78u32 << 8) | (83u32) {
        png_handle_tRNS(png_ptr, info_ptr, length);
    } else {
        png_handle_unknown(png_ptr, info_ptr, length, 0);
    }
}

    };
}
/* SEQUENTIAL_READ */
/* Optional call to update the users info_ptr structure */

pub fn png_read_update_info(mut png_ptr: *mut png_struct, mut info_ptr: *mut png_info) {
    if !png_ptr.is_null() {
        let png = unsafe { &mut *png_ptr };
        if png.flags & 0x40 as u32 == 0 {
            unsafe {
                png_read_start_row(png_ptr);
                png_read_transform_info(&mut png_ptr, &mut info_ptr);
            }
        } else {
            unsafe {
                png_app_error(png_ptr, b"png_read_update_info/png_start_read_image: duplicate call\x00" as *const u8 as *const i8);
            }
        }
    }
}

/* Initialize palette, background, etc, after transformations
 * are set, but before any reading takes place.  This allows
 * the user to obtain a gamma-corrected palette, for example.
 * If the user doesn't call this, we will do it ourselves.
 */

pub fn png_start_read_image(png_ptr: png_structrp) {
    if !png_ptr.is_null() {
        if unsafe { (*png_ptr).flags & 0x40 as u32 } == 0 {
            unsafe { png_read_start_row(png_ptr) };
        } else {
            unsafe {
                png_app_error(png_ptr, b"png_start_read_image/png_read_update_info: duplicate call\x00" as *const u8 as *const i8);
            }
        }
    }
}

/* SEQUENTIAL_READ */
/* MNG_FEATURES */

pub unsafe extern "C" fn png_read_row(mut png_ptr: png_structrp,
                                      mut row: png_bytep,
                                      mut dsp_row: png_bytep) {
    let mut row_info = png_row_info {
    width: 0,
    rowbytes: 0,
    color_type: 0,
    bit_depth: 0,
    channels: 0,
    pixel_depth: 0,
};

if png_ptr.is_null() {
    return;
}

// png_read_start_row sets the information (in particular iwidth) for this interlace pass.
if (*png_ptr).flags & 0x40 == 0 {
    png_read_start_row(png_ptr);
}

// 1.5.6: row_info moved out of png_struct to a local here.
row_info.width = (*png_ptr).iwidth; // NOTE: width of current interlaced row
row_info.color_type = (*png_ptr).color_type;
row_info.bit_depth = (*png_ptr).bit_depth;
row_info.channels = (*png_ptr).channels;
row_info.pixel_depth = (*png_ptr).pixel_depth;

row_info.rowbytes = if row_info.pixel_depth >= 8 {
    (row_info.width as png_size_t).wrapping_mul(row_info.pixel_depth as png_size_t >> 3)
} else {
    ((row_info.width as png_size_t).wrapping_mul(row_info.pixel_depth as png_size_t).wrapping_add(7)) >> 3
};

if (*png_ptr).row_number == 0 && (*png_ptr).pass == 0 {
    // WARNINGS
    // If interlaced and we do not need a new row, combine row and return.
    // Notice that the pixels we have from previous rows have been transformed
    // already; we can only combine like with like (transformed or
    // untransformed) and, because of the libpng API for interlaced images, this
    // means we must transform before de-interlacing.
    if (*png_ptr).interlaced != 0 && (*png_ptr).transformations & 0x2 != 0 {
        match (*png_ptr).pass {
            0 => {
                if (*png_ptr).row_number & 0x7 != 0 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        png_combine_row(png_ptr, dsp_row, 1);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            1 => {
                if (*png_ptr).row_number & 0x7 != 0 || (*png_ptr).width < 5 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        png_combine_row(png_ptr, dsp_row, 1);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            2 => {
                if (*png_ptr).row_number & 0x7 != 4 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        if (*png_ptr).row_number & 4 != 0 {
                            png_combine_row(png_ptr, dsp_row, 1);
                        }
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            3 => {
                if (*png_ptr).row_number & 3 != 0 || (*png_ptr).width < 3 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        png_combine_row(png_ptr, dsp_row, 1);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            4 => {
                if (*png_ptr).row_number & 3 != 2 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        if (*png_ptr).row_number & 2 != 0 {
                            png_combine_row(png_ptr, dsp_row, 1);
                        }
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            5 => {
                if (*png_ptr).row_number & 1 != 0 || (*png_ptr).width < 2 {
                    if let Some(dsp_row) = dsp_row.as_mut() {
                        png_combine_row(png_ptr, dsp_row, 1);
                    }
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
            6 | _ => {
                if (*png_ptr).row_number & 1 == 0 {
                    png_read_finish_row(png_ptr);
                    return;
                }
            }
        }
    }

    if (*png_ptr).mode & 0x4 == 0 {
        png_error(png_ptr, b"Invalid attempt to read row data\x00" as *const u8 as *const i8);
    }

    // Fill the row with IDAT data:
    let row_buf = &mut *(*png_ptr).row_buf; // Dereference to get the mutable pointer
    unsafe {
        *row_buf = 255; // to force error if no data was found
    }
    png_read_IDAT_data(png_ptr, row_buf, row_info.rowbytes.wrapping_add(1));
}

    if unsafe { *(*png_ptr).row_buf.offset(0) } > 0 {
    if unsafe { *(*png_ptr).row_buf.offset(0) } < 5 {
        png_read_filter_row(
            png_ptr,
            &mut row_info,
            unsafe { (*png_ptr).row_buf.offset(1) },
            unsafe { (*png_ptr).prev_row.offset(1) } as png_const_bytep,
            unsafe { *(*png_ptr).row_buf.offset(0) } as i32,
        );
    } else {
        png_error(png_ptr, b"bad adaptive filter value\x00" as *const u8 as *const i8);
    }
}

unsafe {
    memcpy(
        (*png_ptr).prev_row as *mut std::ffi::c_void,
        (*png_ptr).row_buf as *const std::ffi::c_void,
        row_info.rowbytes.wrapping_add(1) as u64,
    );
}

if unsafe { (*png_ptr).transformations } != 0 {
    png_do_read_transformations(png_ptr, &mut row_info);
}

if unsafe { (*png_ptr).transformed_pixel_depth } == 0 {
    unsafe { (*png_ptr).transformed_pixel_depth = row_info.pixel_depth };
    if row_info.pixel_depth > unsafe { (*png_ptr).maximum_pixel_depth } {
        png_error(png_ptr, b"sequential row overflow\x00" as *const u8 as *const i8);
    }
} else if unsafe { (*png_ptr).transformed_pixel_depth } != row_info.pixel_depth {
    png_error(png_ptr, b"internal sequential row size calculation error\x00" as *const u8 as *const i8);
}

if unsafe { (*png_ptr).interlaced } != 0 && (unsafe { (*png_ptr).transformations } & 0x2) != 0 {
    if unsafe { (*png_ptr).pass } < 6 {
        png_do_read_interlace(
            &mut row_info,
            unsafe { (*png_ptr).row_buf.offset(1) },
            unsafe { (*png_ptr).pass as i32 },
            unsafe { (*png_ptr).transformations },
        );
    }
    if !dsp_row.is_null() {
        png_combine_row(png_ptr, dsp_row, 1);
    }
    if !row.is_null() {
        png_combine_row(png_ptr, row, 0);
    }
} else {
    if !row.is_null() {
        png_combine_row(png_ptr, row, -1);
    }
    if !dsp_row.is_null() {
        png_combine_row(png_ptr, dsp_row, -1);
    }
}

png_read_finish_row(png_ptr);
if let Some(read_row_fn) = unsafe { (*png_ptr).read_row_fn } {
    unsafe {
        read_row_fn(png_ptr, (*png_ptr).row_number, (*png_ptr).pass as i32);
    }
}

}
/* SEQUENTIAL_READ */
/* Read one or more rows of image data.  If the image is interlaced,
 * and png_set_interlace_handling() has been called, the rows need to
 * contain the contents of the rows from the previous pass.  If the
 * image has alpha or transparency, and png_handle_alpha()[*] has been
 * called, the rows contents must be initialized to the contents of the
 * screen.
 *
 * "row" holds the actual image, and pixels are placed in it
 * as they arrive.  If the image is displayed after each pass, it will
 * appear to "sparkle" in.  "display_row" can be used to display a
 * "chunky" progressive image, with finer detail added as it becomes
 * available.  If you do not want this "chunky" display, you may pass
 * NULL for display_row.  If you do not want the sparkle display, and
 * you have not called png_handle_alpha(), you may pass NULL for rows.
 * If you have called png_handle_alpha(), and the image has either an
 * alpha channel or a transparency chunk, you must provide a buffer for
 * rows.  In this case, you do not have to provide a display_row buffer
 * also, but you may.  If the image is not interlaced, or if you have
 * not called png_set_interlace_handling(), the display_row buffer will
 * be ignored, so pass NULL to it.
 *
 * [*] png_handle_alpha() does not exist yet, as of this version of libpng
 */

pub unsafe extern "C" fn png_read_rows(mut png_ptr: png_structrp,
                                       mut row: png_bytepp,
                                       mut display_row: png_bytepp,
                                       mut num_rows: png_uint_32) {
    let mut i: png_uint_32 = 0;
    let mut rp: png_bytepp = 0 as *mut *mut png_byte;
    let mut dp: png_bytepp = 0 as *mut *mut png_byte;
    if png_ptr.is_null() { return }
    rp = row;
    dp = display_row;
    if !rp.is_null() && !dp.is_null() {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < num_rows {
            let fresh0 = rp;
            rp = rp.offset(1);
            let mut rptr: png_bytep = *fresh0;
            let fresh1 = dp;
            dp = dp.offset(1);
            let mut dptr: png_bytep = *fresh1;
            png_read_row(png_ptr, rptr, dptr);
            i = i.wrapping_add(1)
        }
    } else if !rp.is_null() {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < num_rows {
            let mut rptr_0: png_bytep = *rp;
            png_read_row(png_ptr, rptr_0, 0 as png_bytep);
            rp = rp.offset(1);
            i = i.wrapping_add(1)
        }
    } else if !dp.is_null() {
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < num_rows {
            let mut dptr_0: png_bytep = *dp;
            png_read_row(png_ptr, 0 as png_bytep, dptr_0);
            dp = dp.offset(1);
            i = i.wrapping_add(1)
        }
    };
}
/* SEQUENTIAL_READ */
/* Read the entire image.  If the image has an alpha channel or a tRNS
 * chunk, and you have called png_handle_alpha()[*], you will need to
 * initialize the image to the current image that PNG will be overlaying.
 * We set the num_rows again here, in case it was incorrectly set in
 * png_read_start_row() by a call to png_read_update_info() or
 * png_start_read_image() if png_set_interlace_handling() wasn't called
 * prior to either of these functions like it should have been.  You can
 * only call this function once.  If you desire to have an image for
 * each pass of a interlaced image, use png_read_rows() instead.
 *
 * [*] png_handle_alpha() does not exist yet, as of this version of libpng
 */

pub unsafe extern "C" fn png_read_image(mut png_ptr: png_structrp,
                                        mut image: png_bytepp) {
    let mut i: png_uint_32 = 0;
    let mut image_height: png_uint_32 = 0;
    let mut pass: std::os::raw::c_int = 0;
    let mut j: std::os::raw::c_int = 0;
    let mut rp: png_bytepp = 0 as *mut *mut png_byte;
    if png_ptr.is_null() { return }
    if (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        let pass = png_set_interlace_handling(png_ptr);
        /* And make sure transforms are initialized. */
        png_start_read_image(png_ptr);
    } else if (*png_ptr).interlaced != 0 && (*png_ptr).transformations & 0x2 == 0 {
    png_warning(png_ptr, b"Interlace handling should be turned on when using png_read_image\x00" as *const u8 as *const i8);
    (*png_ptr).num_rows = (*png_ptr).height;
}
let pass = png_set_interlace_handling(png_ptr);
    image_height = (*png_ptr).height;
    j = 0 as std::os::raw::c_int;
    while j < pass {
        rp = image;
        i = 0 as std::os::raw::c_int as png_uint_32;
        while i < image_height {
            png_read_row(png_ptr, *rp, 0 as png_bytep);
            rp = rp.offset(1);
            i = i.wrapping_add(1)
        }
        j += 1
    };
}
/* SEQUENTIAL_READ */
/* Read the end of the PNG file.  Will not read past the end of the
 * file, will verify the end is accurate, and will read any comments
 * or time information at the end of the file, if info is not NULL.
 */

pub unsafe extern "C" fn png_read_end(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp) {
    let mut keep: i32 = 0;
if png_ptr.is_null() { return; }

/* If png_read_end is called in the middle of reading the rows there may
* still be pending IDAT data and an owned zstream.  Deal with this here.
*/
let chunk_type = (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32);
if png_chunk_unknown_handling(png_ptr, chunk_type) == 0 {
    png_read_finish_IDAT(png_ptr);
}

/* Report invalid palette index; added at libng-1.5.10 */
if (*png_ptr).color_type as i32 == (2 | 1) && (*png_ptr).num_palette_max > (*png_ptr).num_palette as i32 {
    png_benign_error(png_ptr, "Read palette index exceeding num_palette");
}

    loop {
    let length: png_uint_32 = png_read_chunk_header(png_ptr);
    let chunk_name = unsafe { (*png_ptr).chunk_name };

    if chunk_name != (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32) {
        unsafe { (*png_ptr).mode |= 0x2000; }
    }

    if chunk_name == (73u32 << 24) | (69u32 << 16) | (78u32 << 8) | (68u32) {
        png_handle_IEND(png_ptr, info_ptr, length);
    } else if chunk_name == (73u32 << 24) | (72u32 << 16) | (68u32 << 8) | (82u32) {
        png_handle_IHDR(png_ptr, info_ptr, length);
    } else if info_ptr.is_null() {
        png_crc_finish(png_ptr, length);
    } else {
        keep = png_chunk_unknown_handling(png_ptr, chunk_name);
        if keep != 0 {
            if chunk_name == (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32) {
                if length > 0 && (unsafe { (*png_ptr).flags } & 0x8) == 0 || (unsafe { (*png_ptr).mode } & 0x2000) != 0 {
                    png_benign_error(png_ptr, ".Too many IDATs found");
                }
            }
            png_handle_unknown(png_ptr, info_ptr, length, keep);
            if chunk_name == (80u32 << 24) | (76u32 << 16) | (84u32 << 8) | (69u32) {
                unsafe { (*png_ptr).mode |= 0x2; }
            }
        } else if chunk_name == (73u32 << 24) | (68u32 << 16) | (65u32 << 8) | (84u32) {
            if length > 0 && (unsafe { (*png_ptr).flags } & 0x8) == 0 || (unsafe { (*png_ptr).mode } & 0x2000) != 0 {
                png_benign_error(png_ptr, "..Too many IDATs found");
            }
            png_crc_finish(png_ptr, length);
        } else if chunk_name == (80u32 << 24) | (76u32 << 16) | (84u32 << 8) | (69u32) {
            png_handle_PLTE(png_ptr, info_ptr, length);
        } else if chunk_name == (98u32 << 24) | (75u32 << 16) | (71u32 << 8) | (68u32) {
            png_handle_bKGD(png_ptr, info_ptr, length);
        } else if chunk_name == (104u32 << 24) | (73u32 << 16) | (83u32 << 8) | (84u32) {
            png_handle_hIST(png_ptr, info_ptr, length);
        } else if chunk_name == (115u32 << 24) | (66u32 << 16) | (73u32 << 8) | (84u32) {
            png_handle_sBIT(png_ptr, info_ptr, length);
        } else if chunk_name == (116u32 << 24) | (82u32 << 16) | (78u32 << 8) | (83u32) {
            png_handle_tRNS(png_ptr, info_ptr, length);
        } else {
            png_handle_unknown(png_ptr, info_ptr, length, 0);
        }
    }

    if unsafe { (*png_ptr).mode & 0x10 } == 0 {
        break;
    }
}

}
/* SEQUENTIAL_READ */
/* Free all memory used in the read struct */
unsafe extern "C" fn png_read_destroy(mut png_ptr: png_structrp) {
    png_free(png_ptr, (*png_ptr).big_row_buf as png_voidp);
    (*png_ptr).big_row_buf = 0 as png_bytep;
    png_free(png_ptr, (*png_ptr).big_prev_row as png_voidp);
    (*png_ptr).big_prev_row = 0 as png_bytep;
    png_free(png_ptr, (*png_ptr).read_buffer as png_voidp);
    (*png_ptr).read_buffer = 0 as png_bytep;
    if (*png_ptr).free_me & 0x1000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_zfree(png_ptr as voidpf, (*png_ptr).palette as voidpf);
        (*png_ptr).palette = 0 as png_colorp
    }
    (*png_ptr).free_me &= !(0x1000 as std::os::raw::c_uint);
    if (*png_ptr).free_me & 0x2000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_free(png_ptr, (*png_ptr).trans_alpha as png_voidp);
        (*png_ptr).trans_alpha = 0 as png_bytep
    }
    (*png_ptr).free_me &= !(0x2000 as std::os::raw::c_uint);
    inflateEnd(&mut (*png_ptr).zstream);
    png_free(png_ptr, (*png_ptr).unknown_chunk.data as png_voidp);
    (*png_ptr).unknown_chunk.data = 0 as *mut png_byte;
    png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
    (*png_ptr).chunk_list = 0 as png_bytep;
    /* NOTE: the 'setjmp' buffer may still be allocated and the memory and error
    * callbacks are still set at this point.  They are required to complete the
    * destruction of the png_struct itself.
    */
}
/* Free all memory used by the read */

pub fn png_destroy_read_struct(
    png_ptr_ptr: &mut *mut png_struct,
    info_ptr_ptr: &mut *mut png_info,
    end_info_ptr_ptr: &mut *mut png_info,
) {
    let png_ptr = unsafe { *png_ptr_ptr };
    if png_ptr.is_null() { return; }

    // Use the API to destroy info structs to ensure consistent behavior.
    if !end_info_ptr_ptr.is_null() {
        unsafe { png_destroy_info_struct(png_ptr, end_info_ptr_ptr); }
    }
    if !info_ptr_ptr.is_null() {
        unsafe { png_destroy_info_struct(png_ptr, info_ptr_ptr); }
    }

    unsafe {
        *png_ptr_ptr = std::ptr::null_mut();
        png_read_destroy(png_ptr);
        png_destroy_png_struct(png_ptr);
    }
}


pub unsafe extern "C" fn png_set_read_status_fn(mut png_ptr: png_structrp,
                                                mut read_row_fn:
                                                    png_read_status_ptr) {
    if png_ptr.is_null() { return }
    (*png_ptr).read_row_fn = read_row_fn;
}

pub unsafe extern "C" fn png_read_png(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp,
                                      mut transforms: std::os::raw::c_int,
                                      mut params: voidp) {
    if png_ptr.is_null() || info_ptr.is_null() { return; }

let info = unsafe { &*info_ptr };
png_read_info(png_ptr, info_ptr);

if info.height as usize > (std::i32::MIN as u32 as usize) / std::mem::size_of::<png_bytep>() {
    png_error(png_ptr, b"Image is too high to process with png_read_png()\0".as_ptr() as *const i8);
}

if transforms & 0x8000 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_SCALE_16 not supported\0".as_ptr() as *const i8);
}

if transforms & 0x1 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_STRIP_16 not supported\0".as_ptr() as *const i8);
}

if transforms & 0x2 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_STRIP_ALPHA not supported\0".as_ptr() as *const i8);
}

if transforms & 0x4 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_PACKING not supported\0".as_ptr() as *const i8);
}

if transforms & 0x8 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_PACKSWAP not supported\0".as_ptr() as *const i8);
}

if transforms & 0x10 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_EXPAND not supported\0".as_ptr() as *const i8);
}

if transforms & 0x20 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_INVERT_MONO not supported\0".as_ptr() as *const i8);
}

if transforms & 0x40 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_SHIFT not supported\0".as_ptr() as *const i8);
}

if transforms & 0x80 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_BGR not supported\0".as_ptr() as *const i8);
}

if transforms & 0x100 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_SWAP_ALPHA not supported\0".as_ptr() as *const i8);
}

if transforms & 0x200 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_SWAP_ENDIAN not supported\0".as_ptr() as *const i8);
}

if transforms & 0x400 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_INVERT_ALPHA not supported\0".as_ptr() as *const i8);
}

if transforms & 0x2000 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_GRAY_TO_RGB not supported\0".as_ptr() as *const i8);
}

if transforms & 0x4000 != 0 {
    png_app_error(png_ptr, b"PNG_TRANSFORM_EXPAND_16 not supported\0".as_ptr() as *const i8);
}

png_set_interlace_handling(png_ptr);
png_read_update_info(png_ptr, info_ptr);
png_free_data(png_ptr, info_ptr, 0x40, 0);

    if (*info_ptr).row_pointers.is_null() {
    let height = (*info_ptr).height as usize;
    let mut row_pointers: Vec<*mut png_byte> = vec![std::ptr::null_mut(); height];

    for iptr in 0..height {
        row_pointers[iptr] = std::ptr::null_mut();
    }

    (*info_ptr).free_me |= 0x40;

    for iptr in 0..height {
        row_pointers[iptr] = png_malloc(png_ptr, (*info_ptr).rowbytes) as *mut png_byte;
    }

    (*info_ptr).row_pointers = row_pointers.as_mut_ptr();
}

png_read_image(png_ptr, (*info_ptr).row_pointers);
(*info_ptr).valid |= 0x8000;
png_read_end(png_ptr, info_ptr);

}
/* READ */
/* SIMPLIFIED_READ */
/* SEQUENTIAL_READ */
/* INFO_IMAGE */
