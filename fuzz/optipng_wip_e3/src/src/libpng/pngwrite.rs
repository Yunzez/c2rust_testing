









use std::ffi::CStr;
use std::ptr;

extern "C" {
    
    
    
    
    
    
    fn memcpy(_: *mut std::os::raw::c_void, _: *const std::os::raw::c_void, _: std::os::raw::c_ulong)
     -> *mut std::os::raw::c_void;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::src::libpng::png::png_create_png_struct;
pub use crate::src::libpng::png::png_destroy_info_struct;
pub use crate::src::libpng::png::png_handle_as_unknown;
pub use crate::src::libpng::pngerror::png_app_error;
pub use crate::src::libpng::pngerror::png_app_warning;
pub use crate::src::libpng::pngerror::png_benign_error;
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngmem::png_destroy_png_struct;
pub use crate::src::libpng::pngmem::png_free;
pub use crate::src::libpng::pngmem::png_malloc;
pub use crate::src::libpng::pngtrans::png_do_check_palette_indexes;
pub use crate::src::libpng::pngtrans::png_set_interlace_handling;
pub use crate::src::libpng::pngwio::png_flush;
pub use crate::src::libpng::pngwio::png_set_write_fn;
pub use crate::src::libpng::pngwutil::png_compress_IDAT;
pub use crate::src::libpng::pngwutil::png_do_write_interlace;
pub use crate::src::libpng::pngwutil::png_free_buffer_list;
pub use crate::src::libpng::pngwutil::png_write_IEND;
pub use crate::src::libpng::pngwutil::png_write_IHDR;
pub use crate::src::libpng::pngwutil::png_write_PLTE;
pub use crate::src::libpng::pngwutil::png_write_bKGD;
pub use crate::src::libpng::pngwutil::png_write_chunk;
pub use crate::src::libpng::pngwutil::png_write_find_filter;
pub use crate::src::libpng::pngwutil::png_write_finish_row;
pub use crate::src::libpng::pngwutil::png_write_hIST;
pub use crate::src::libpng::pngwutil::png_write_sBIT;
pub use crate::src::libpng::pngwutil::png_write_sig;
pub use crate::src::libpng::pngwutil::png_write_start_row;
pub use crate::src::libpng::pngwutil::png_write_tRNS;
pub use crate::src::zlib::deflate::deflateEnd;
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
pub type png_const_uint_16p = crate::src::libpng::pngrutil::png_const_uint_16p;
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
pub type png_structpp = crate::src::libpng::pngread::png_structpp;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::pngget::png_info_def;
pub type png_unknown_chunkp = crate::src::libpng::png::png_unknown_chunkp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_infopp = crate::src::libpng::png::png_infopp;
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
pub type png_const_inforp = crate::src::libpng::pngget::png_const_inforp;
pub type png_const_colorp = crate::src::libpng::pngrutil::png_const_colorp;
pub type png_const_color_16p = crate::src::libpng::pngrutil::png_const_color_16p;
pub type png_const_color_8p = crate::src::libpng::pngrutil::png_const_color_8p;
pub type png_const_unknown_chunkp = crate::src::libpng::pngrutil::png_const_unknown_chunkp;
pub type png_malloc_ptr = crate::src::libpng::png::png_malloc_ptr;
pub type png_free_ptr = crate::src::libpng::png::png_free_ptr;
pub type z_streamp = crate::src::libpng::png::z_streamp;
pub type voidp = crate::src::libpng::pngread::voidp;
/* pngwrite.c - general routines to write a PNG file
 *
 * Last changed in libpng 1.6.32 [August 24, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */
/* SIMPLIFIED_WRITE_STDIO */
/* Write out all the unknown chunks for the current given location */
fn write_unknown_chunks(png_ptr: *mut png_struct_def, info_ptr: *const png_info_def, where_0: u32) {
    let info = unsafe { &*info_ptr };
    if info.unknown_chunks_num != 0 {
        let unknown_chunks = unsafe { std::slice::from_raw_parts(info.unknown_chunks, info.unknown_chunks_num as usize) };
        for up in unknown_chunks.iter() {
            if (up.location as u32 & where_0) != 0 {
                continue;
            }
            let keep = unsafe { png_handle_as_unknown(png_ptr, up.name.as_ptr()) };
            if keep != 1 && (up.name[3] & 0x20 != 0 || keep == 3 || (keep == 0 && unsafe { (*png_ptr).unknown_default } == 3)) {
                if up.size == 0 {
                    unsafe { png_warning(png_ptr, b"Writing zero-length unknown chunk\x00" as *const u8 as *const std::os::raw::c_char) };
                }
                unsafe { png_write_chunk(png_ptr, up.name.as_ptr(), up.data, up.size) };
            }
        }
    }
}

/* WRITE_UNKNOWN_CHUNKS */
/* Writes all the PNG information.  This is the suggested way to use the
 * library.  If you have a new chunk to add, make a function to write it,
 * and put it in the correct location here.  If you want the chunk written
 * after the image data, put it in png_write_end().  I strongly encourage
 * you to supply a PNG_INFO_ flag, and check info_ptr->valid before writing
 * the chunk, as that will keep the code from breaking if you want to just
 * write a plain PNG file.  If you have long comments, I suggest writing
 * them in png_write_end(), and compressing them.
 */

pub unsafe extern "C" fn png_write_info_before_PLTE(mut png_ptr: png_structrp,
                                                    mut info_ptr:
                                                        png_const_inforp) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if (*png_ptr).mode & 0x400 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Write PNG signature */
        png_write_sig(png_ptr);
        /* Write IHDR information. */
        png_write_IHDR(png_ptr, (*info_ptr).width, (*info_ptr).height,
                       (*info_ptr).bit_depth as std::os::raw::c_int,
                       (*info_ptr).color_type as std::os::raw::c_int,
                       (*info_ptr).compression_type as std::os::raw::c_int,
                       (*info_ptr).filter_type as std::os::raw::c_int,
                       (*info_ptr).interlace_type as std::os::raw::c_int);
        /* The rest of these check to see if the valid field has the appropriate
       * flag set, and if it does, writes the chunk.
       *
       * 1.6.0: COLORSPACE support controls the writing of these chunks too, and
       * the chunks will be written if the WRITE routine is there and
       * information * is available in the COLORSPACE. (See
       * png_colorspace_sync_info in png.c for where the valid flags get set.)
       *
       * Under certain circumstances the colorspace can be invalidated without
       * syncing the info_struct 'valid' flags; this happens if libpng detects
       * an error and calls png_error while the color space is being set, yet
       * the application continues writing the PNG.  So check the 'invalid'
       * flag here too.
       */
        /* COLORSPACE */
        if (*info_ptr).valid & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_write_sBIT(png_ptr, &(*info_ptr).sig_bit,
                           (*info_ptr).color_type as std::os::raw::c_int);
        }
        write_unknown_chunks(png_ptr, info_ptr, 0x1);
        (*png_ptr).mode |= 0x400 as std::os::raw::c_uint
    };
}

pub unsafe extern "C" fn png_write_info(mut png_ptr: png_structrp,
                                        mut info_ptr: png_const_inforp) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    png_write_info_before_PLTE(png_ptr, info_ptr);
    if (*info_ptr).valid & 0x8 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_write_PLTE(png_ptr, (*info_ptr).palette as png_const_colorp,
                       (*info_ptr).num_palette as png_uint_32);
    } else if (*info_ptr).color_type as std::os::raw::c_int ==
                  2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"Valid palette required for paletted images\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    if (*info_ptr).valid & 0x10 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_write_tRNS(png_ptr, (*info_ptr).trans_alpha as png_const_bytep,
                       &(*info_ptr).trans_color,
                       (*info_ptr).num_trans as std::os::raw::c_int,
                       (*info_ptr).color_type as std::os::raw::c_int);
    }
    if (*info_ptr).valid & 0x20 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_write_bKGD(png_ptr, &(*info_ptr).background,
                       (*info_ptr).color_type as std::os::raw::c_int);
    }
    if (*info_ptr).valid & 0x40 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        let hist_slice: &[u16] = unsafe { std::slice::from_raw_parts((*info_ptr).hist, (*info_ptr).num_palette as usize) };
png_write_hIST(&mut png_ptr, hist_slice, (*info_ptr).num_palette as i32);
    }
    /* sCAL */
    /* pHYs */
    /* tIME */
    /* sPLT */
    /* tEXt */
    write_unknown_chunks(png_ptr, info_ptr, 0x2);
}
/* Writes the end of the PNG file.  If you don't want to write comments or
 * time information, you can pass NULL for info.  If you already wrote these
 * in png_write_info(), do not write them again here.  If you have long
 * comments, I suggest writing them here, and compressing them.
 */

pub unsafe extern "C" fn png_write_end(mut png_ptr: png_structrp,
                                       mut info_ptr: png_inforp) {
    if png_ptr.is_null() { return }
    if (*png_ptr).mode & 0x4 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr,
                  b"No IDATs written into file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    if (*png_ptr).num_palette_max > (*png_ptr).num_palette as std::os::raw::c_int {
        let error_message = "Wrote palette index exceeding num_palette";
png_benign_error(png_ptr, error_message);
    }
    /* See if user wants us to write information chunks */
    if !info_ptr.is_null() {
        write_unknown_chunks(png_ptr, info_ptr, 0x8);
    }
    (*png_ptr).mode |= 0x8 as std::os::raw::c_int as std::os::raw::c_uint;
    /* Write end of PNG file */
    png_write_IEND(&mut png_ptr);
    /* This flush, added in libpng-1.0.8, removed from libpng-1.0.9beta03,
    * and restored again in libpng-1.2.30, may cause some applications that
    * do not set png_ptr->output_flush_fn to crash.  If your application
    * experiences a problem, please try building libpng with
    * PNG_WRITE_FLUSH_AFTER_IEND_SUPPORTED defined, and report the event to
    * png-mng-implement at lists.sf.net .
    */
}
/* Initialize png_ptr structure, and allocate any memory needed */

pub fn png_create_write_struct(user_png_ver: &str, error_ptr: *mut std::ffi::c_void, error_fn: Option<unsafe extern "C" fn(png_structp, png_const_charp)>, warn_fn: Option<unsafe extern "C" fn(png_structp, png_const_charp)>) -> *mut png_struct {
    unsafe {
        let png_ptr = png_create_png_struct(user_png_ver.as_ptr() as *const i8, error_ptr, error_fn, warn_fn, std::ptr::null_mut(), None, None);
        
        if !png_ptr.is_null() {
            (*png_ptr).zbuffer_size = 8192;
            (*png_ptr).zlib_strategy = 1;
            (*png_ptr).zlib_level = -1;
            (*png_ptr).zlib_mem_level = 8;
            (*png_ptr).zlib_window_bits = 15;
            (*png_ptr).zlib_method = 8;
            (*png_ptr).flags |= 0x200000;
            
            png_set_write_fn(png_ptr, std::ptr::null_mut(), None, None);
        }
        
        png_ptr
    }
}

/* Write a few rows of image data.  If the image is interlaced,
 * either you will have to write the 7 sub images, or, if you
 * have called png_set_interlace_handling(), you will have to
 * "write" the image seven times.
 */

pub unsafe extern "C" fn png_write_rows(mut png_ptr: png_structrp,
                                        mut row: png_bytepp,
                                        mut num_rows: png_uint_32) {
    let mut i: png_uint_32 = 0; /* row counter */
    let mut rp: png_bytepp = 0 as *mut *mut png_byte; /* row pointer */
    if png_ptr.is_null() { return }
    /* Loop through the rows */
    i = 0 as std::os::raw::c_int as png_uint_32;
    rp = row;
    while i < num_rows {
        png_write_row(png_ptr, *rp as png_const_bytep);
        i = i.wrapping_add(1);
        rp = rp.offset(1)
    };
}
/* Write the image.  You only need to call this function once, even
 * if you are writing an interlaced image.
 */

pub unsafe extern "C" fn png_write_image(mut png_ptr: png_structrp,
                                         mut image: png_bytepp) {
    let mut i: png_uint_32 = 0; /* row index */
    let mut pass: std::os::raw::c_int = 0; /* pass variables */
    let mut num_pass: std::os::raw::c_int = 0; /* points to current row */
    let mut rp: png_bytepp = 0 as *mut *mut png_byte;
    if png_ptr.is_null() { return }
    /* Initialize interlace handling.  If image is not interlaced,
    * this will set pass to 1
    */
    let num_pass = png_set_interlace_handling(png_ptr);
    /* Loop through passes */
    pass = 0 as std::os::raw::c_int;
    while pass < num_pass {
        /* Loop through image */
        i = 0 as std::os::raw::c_int as png_uint_32;
        rp = image;
        while i < (*png_ptr).height {
            png_write_row(png_ptr, *rp as png_const_bytep);
            i = i.wrapping_add(1);
            rp = rp.offset(1)
        }
        pass += 1
    };
}
/* MNG_FEATURES */
/* Called by user to write a row of image data */

pub unsafe extern "C" fn png_write_row(mut png_ptr: png_structrp,
                                       mut row: png_const_bytep) {
    /* 1.5.6: moved from png_struct to be a local structure: */
    let mut row_info: png_row_info =
        png_row_info{width: 0,
                     rowbytes: 0,
                     color_type: 0,
                     bit_depth: 0,
                     channels: 0,
                     pixel_depth: 0,};
    if png_ptr.is_null() { return }
    /* Initialize transformations and other stuff if first time */
    if (*png_ptr).row_number == 0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*png_ptr).pass as std::os::raw::c_int == 0 as std::os::raw::c_int {
        /* Make sure we wrote the header info */
        if (*png_ptr).mode & 0x400 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_error(png_ptr,
                      b"png_write_info was never called before png_write_row\x00"
                          as *const u8 as *const std::os::raw::c_char);
        }
        /* Check for transforms that have been set but were defined out */
        png_write_start_row(png_ptr);
    }
    /* If interlaced and not interested in row, return */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int &&
           (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        match (*png_ptr).pass as std::os::raw::c_int {
            0 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            1 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 5 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            2 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 4 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            3 => {
                if (*png_ptr).row_number & 0x3 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            4 => {
                if (*png_ptr).row_number & 0x3 as std::os::raw::c_int as std::os::raw::c_uint
                       != 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            5 => {
                if (*png_ptr).row_number & 0x1 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            6 => {
                if (*png_ptr).row_number & 0x1 as std::os::raw::c_int as std::os::raw::c_uint
                       == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(&mut png_ptr);
                    return
                }
            }
            _ => { }
        }
    }
    /* Set up row info for transformations */
    row_info.color_type = (*png_ptr).color_type;
    row_info.width = (*png_ptr).usr_width;
    row_info.channels = (*png_ptr).usr_channels;
    row_info.bit_depth = (*png_ptr).usr_bit_depth;
    row_info.pixel_depth =
        (row_info.bit_depth as std::os::raw::c_int * row_info.channels as std::os::raw::c_int)
            as png_byte;
    row_info.rowbytes =
        if row_info.pixel_depth as std::os::raw::c_int >= 8 as std::os::raw::c_int {
            (row_info.width as
                 png_size_t).wrapping_mul(row_info.pixel_depth as png_size_t
                                              >> 3 as std::os::raw::c_int)
        } else {
            ((row_info.width as
                  png_size_t).wrapping_mul(row_info.pixel_depth as
                                               png_size_t).wrapping_add(7 as
                                                                            std::os::raw::c_int
                                                                            as
                                                                            std::os::raw::c_ulong))
                >> 3 as std::os::raw::c_int
        };
    /* Copy user's row into buffer, leaving room for filter byte. */
    memcpy((*png_ptr).row_buf.offset(1 as std::os::raw::c_int as isize) as
               *mut std::os::raw::c_void, row as *const std::os::raw::c_void,
           row_info.rowbytes);
    /* Handle interlacing */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 &&
           ((*png_ptr).pass as std::os::raw::c_int) < 6 as std::os::raw::c_int &&
           (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_do_write_interlace(&mut row_info,
                               (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as
                                                             isize),
                               (*png_ptr).pass as std::os::raw::c_int);
        /* This should always get caught above, but still ... */
        if row_info.width == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_write_finish_row(&mut png_ptr);
            return
        }
    }
    /* At this point the row_info pixel depth must match the 'transformed' depth,
    * which is also the output depth.
    */
    if row_info.pixel_depth as std::os::raw::c_int !=
           (*png_ptr).pixel_depth as std::os::raw::c_int ||
           row_info.pixel_depth as std::os::raw::c_int !=
               (*png_ptr).transformed_pixel_depth as std::os::raw::c_int {
        png_error(png_ptr,
                  b"internal write transform logic error\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Added at libpng-1.5.10 */
    /* Check for out-of-range palette index */
    if row_info.color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           (*png_ptr).num_palette_max >= 0 as std::os::raw::c_int {
        png_do_check_palette_indexes(png_ptr, &mut row_info);
    }
    /* Find a filter if necessary, filter the row and write it out. */
    png_write_find_filter(png_ptr, &mut row_info);
    if (*png_ptr).write_row_fn.is_some() {
        Some((*png_ptr).write_row_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                              (*png_ptr).row_number,
                                                                                                              (*png_ptr).pass
                                                                                                                  as
                                                                                                                  std::os::raw::c_int);
    };
}
/* Set the automatic flush interval or 0 to turn flushing off */

pub unsafe extern "C" fn png_set_flush(mut png_ptr: png_structrp,
                                       mut nrows: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    (*png_ptr).flush_dist =
        if nrows < 0 as std::os::raw::c_int {
            0 as std::os::raw::c_int as std::os::raw::c_uint
        } else { nrows as png_uint_32 };
}
/* Flush the current output buffers now */

pub unsafe extern "C" fn png_write_flush(mut png_ptr: png_structrp) {
    if png_ptr.is_null() { return }
    /* We have already written out all of the data */
    if (*png_ptr).row_number >= (*png_ptr).num_rows { return }
    png_compress_IDAT(png_ptr, 0 as png_const_bytep,
                      0 as std::os::raw::c_int as png_alloc_size_t, 2 as std::os::raw::c_int);
    (*png_ptr).flush_rows = 0 as std::os::raw::c_int as png_uint_32;
    png_flush(&mut png_ptr);
}
/* WRITE_FLUSH */
/* Free any memory used in png_ptr struct without freeing the struct itself. */
unsafe extern "C" fn png_write_destroy(mut png_ptr: png_structrp) {
    /* Free any memory zlib uses */
    if (*png_ptr).flags & 0x2 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        deflateEnd(&mut (*png_ptr).zstream);
    }
    /* Free our memory.  png_free checks NULL for us. */
    png_free_buffer_list(png_ptr, &mut (*png_ptr).zbuffer_list);
    png_free(png_ptr, (*png_ptr).row_buf as png_voidp);
    (*png_ptr).row_buf = 0 as png_bytep;
    png_free(png_ptr, (*png_ptr).prev_row as png_voidp);
    png_free(png_ptr, (*png_ptr).try_row as png_voidp);
    png_free(png_ptr, (*png_ptr).tst_row as png_voidp);
    (*png_ptr).prev_row = 0 as png_bytep;
    (*png_ptr).try_row = 0 as png_bytep;
    (*png_ptr).tst_row = 0 as png_bytep;
    png_free(png_ptr, (*png_ptr).chunk_list as png_voidp);
    (*png_ptr).chunk_list = 0 as png_bytep;
    /* The error handling and memory handling information is left intact at this
    * point: the jmp_buf may still have to be freed.  See png_destroy_png_struct
    * for how this happens.
    */
}
/* Free all memory used by the write.
 * In libpng 1.6.0 this API changed quietly to no longer accept a NULL value for
 * *png_ptr_ptr.  Prior to 1.6.0 it would accept such a value and it would free
 * the passed in info_structs but it would quietly fail to free any of the data
 * inside them.  In 1.6.0 it quietly does nothing (it has to be quiet because it
 * has no png_ptr.)
 */

pub fn png_destroy_write_struct(png_ptr_ptr: &mut *mut png_struct, info_ptr_ptr: &mut *mut png_info) {
    if !(*png_ptr_ptr).is_null() {
        let png_ptr: png_structrp = *png_ptr_ptr;
        if !png_ptr.is_null() {
            unsafe {
                png_destroy_info_struct(png_ptr, info_ptr_ptr);
                *png_ptr_ptr = std::ptr::null_mut();
                png_write_destroy(png_ptr);
                png_destroy_png_struct(png_ptr);
            }
        }
    }
}

/* Allow the application to select one or more row filters to use. */

pub fn png_set_filter(png_ptr: *mut png_struct_def, method: i32, mut filters: i32) {
    if png_ptr.is_null() { return; }
    if method == 0 {
        let mut current_block: u64;
        match filters & (0x8 | 0x10 | 0x20 | 0x40 | 0x80 | 0x7) {
            5 | 6 | 7 => {
                unsafe {
                    png_app_error(png_ptr, b"Unknown row filter for method 0\0".as_ptr() as *const i8);
                }
                current_block = 12876746043691347196;
            }
            0 => { current_block = 12876746043691347196; }
            1 => {
                unsafe { (*png_ptr).do_filter = 0x10 as u8; }
                current_block = 1054647088692577877;
            }
            2 => {
                unsafe { (*png_ptr).do_filter = 0x20 as u8; }
                current_block = 1054647088692577877;
            }
            3 => {
                unsafe { (*png_ptr).do_filter = 0x40 as u8; }
                current_block = 1054647088692577877;
            }
            4 => {
                unsafe { (*png_ptr).do_filter = 0x80 as u8; }
                current_block = 1054647088692577877;
            }
            _ => {
                unsafe { (*png_ptr).do_filter = filters as u8; }
                current_block = 1054647088692577877;
            }
        }
        match current_block {
            12876746043691347196 => {
                unsafe { (*png_ptr).do_filter = 0x8 as u8; }
            }
            _ => {}
        }
        if !unsafe { (*png_ptr).row_buf.is_null() } {
            let mut num_filters: i32 = 0;
            let mut buf_size: png_alloc_size_t = 0;
            if unsafe { (*png_ptr).height } == 1 {
                filters &= !(0x20 | 0x40 | 0x80);
            }
            if unsafe { (*png_ptr).width } == 1 {
                filters &= !(0x10 | 0x40 | 0x80);
            }
            if filters & (0x20 | 0x40 | 0x80) != 0 && unsafe { (*png_ptr).prev_row.is_null() } {
                unsafe {
                    png_app_warning(png_ptr, b"png_set_filter: UP/AVG/PAETH cannot be added after start\0".as_ptr() as *const i8);
                }
                filters &= !(0x20 | 0x40 | 0x80);
            }
            num_filters = 0;
            if filters & 0x10 != 0 { num_filters += 1; }
            if filters & 0x20 != 0 { num_filters += 1; }
            if filters & 0x40 != 0 { num_filters += 1; }
            if filters & 0x80 != 0 { num_filters += 1; }
            buf_size = if unsafe { (*png_ptr).usr_channels * (*png_ptr).usr_bit_depth } >= 8 {
                (unsafe { (*png_ptr).width } as png_size_t).wrapping_mul((unsafe { (*png_ptr).usr_channels * (*png_ptr).usr_bit_depth }) as png_size_t >> 3)
            } else {
                (((unsafe { (*png_ptr).width } as png_size_t).wrapping_mul((unsafe { (*png_ptr).usr_channels * (*png_ptr).usr_bit_depth }) as png_size_t).wrapping_add(7)) >> 3)
            }.wrapping_add(1);
            if unsafe { (*png_ptr).try_row.is_null() } {
                unsafe { (*png_ptr).try_row = png_malloc(png_ptr, buf_size) as png_bytep; }
            }
            if num_filters > 1 {
                if unsafe { (*png_ptr).tst_row.is_null() } {
                    unsafe { (*png_ptr).tst_row = png_malloc(png_ptr, buf_size) as png_bytep; }
                }
            }
        }
        unsafe { (*png_ptr).do_filter = filters as u8; }
    } else {
        unsafe {
            png_error(png_ptr, b"Unknown custom filter method\0".as_ptr() as *const i8);
        }
    }
}

/* DEPRECATED */
/* WRITE_WEIGHTED_FILTER */

pub fn png_set_compression_level(png_ptr: *mut png_struct, level: i32) {
    if png_ptr.is_null() { return; }
    unsafe {
        (*png_ptr).zlib_level = level;
    }
}


pub fn png_set_compression_mem_level(png_ptr: &mut png_struct, mem_level: i32) {
    // Since png_ptr is a mutable reference, we don't need to check for null.
    png_ptr.zlib_mem_level = mem_level;
}


pub fn png_set_compression_strategy(png_ptr: *mut png_struct, strategy: i32) {
    if png_ptr.is_null() { return; }
    // The flag setting here prevents the libpng dynamic selection of strategy.
    unsafe {
        (*png_ptr).flags |= 0x1;
        (*png_ptr).zlib_strategy = strategy;
    }
}

/* If PNG_WRITE_OPTIMIZE_CMF_SUPPORTED is defined, libpng will use a
 * smaller value of window_bits if it can do so safely.
 */

pub unsafe extern "C" fn png_set_compression_window_bits(mut png_ptr:
                                                             png_structrp,
                                                         mut window_bits:
                                                             std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    /* Prior to 1.6.0 this would warn but then set the window_bits value. This
    * meant that negative window bits values could be selected that would cause
    * libpng to write a non-standard PNG file with raw deflate or gzip
    * compressed IDAT or ancillary chunks.  Such files can be read and there is
    * no warning on read, so this seems like a very bad idea.
    */
    if window_bits > 15 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Only compression windows <= 32k supported by PNG\x00" as
                        *const u8 as *const std::os::raw::c_char);
        window_bits = 15 as std::os::raw::c_int
    } else if window_bits < 8 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Only compression windows >= 256 supported by PNG\x00" as
                        *const u8 as *const std::os::raw::c_char);
        window_bits = 8 as std::os::raw::c_int
    }
    (*png_ptr).zlib_window_bits = window_bits;
}

pub fn png_set_compression_method(png_ptr: *mut png_structrp, method: i32) {
    if png_ptr.is_null() { return; }
    unsafe {
        if method != 8 {
            png_warning(*png_ptr, b"Only compression method 8 is supported by PNG\x00" as *const u8 as *const std::os::raw::c_char);
        }
        (*(*png_ptr)).zlib_method = method;
    }
}

/* WRITE_CUSTOMIZE_COMPRESSION */
/* The following were added to libpng-1.5.4 */
/* WRITE_CUSTOMIZE_ZTXT_COMPRESSION */
/* end of API added to libpng-1.5.4 */

pub fn png_set_write_status_fn(png_ptr: &mut png_structrp, write_row_fn: png_write_status_ptr) {
    if png_ptr.is_null() { return; }
    unsafe {
        (*(*png_ptr)).write_row_fn = write_row_fn;
    }
}


pub fn png_write_png(png_ptr: *mut png_struct, info_ptr: *mut png_info, transforms: i32, params: Option<*mut std::ffi::c_void>) {
    if png_ptr.is_null() || info_ptr.is_null() { return; }
    let info = unsafe { &*info_ptr };
    if info.valid & 0x8000 as u32 == 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"no rows for png_write_image to write\0").unwrap().as_ptr());
        }
        return;
    }
    /* Write the file header information. */
    unsafe {
        png_write_info(png_ptr, info_ptr);
    }
    /* ------ these transformations don't touch the info structure ------- */
    /* Invert monochrome pixels */
    if transforms & 0x20 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_INVERT_MONO not supported\0").unwrap().as_ptr());
        }
    }
    /* Shift the pixels up to a legal bit depth and fill in
    * as appropriate to correctly scale the image.
    */
    if transforms & 0x40 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_SHIFT not supported\0").unwrap().as_ptr());
        }
    }
    /* Pack pixels into bytes */
    if transforms & 0x4 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_PACKING not supported\0").unwrap().as_ptr());
        }
    }
    /* Swap location of alpha bytes from ARGB to RGBA */
    if transforms & 0x100 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_SWAP_ALPHA not supported\0").unwrap().as_ptr());
        }
    }
    /* Remove a filler (X) from XRGB/RGBX/AG/GA into to convert it into
    * RGB, note that the code expects the input color type to be G or RGB; no
    * alpha channel.
    */
    if transforms & (0x1000 | 0x800) != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_STRIP_FILLER not supported\0").unwrap().as_ptr());
        }
    }
    /* Flip BGR pixels to RGB */
    if transforms & 0x80 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_BGR not supported\0").unwrap().as_ptr());
        }
    }
    /* Swap bytes of 16-bit files to most significant byte first */
    if transforms & 0x200 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_SWAP_ENDIAN not supported\0").unwrap().as_ptr());
        }
    }
    /* Swap bits of 1-bit, 2-bit, 4-bit packed pixel formats */
    if transforms & 0x8 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_PACKSWAP not supported\0").unwrap().as_ptr());
        }
    }
    /* Invert the alpha channel from opacity to transparency */
    if transforms & 0x400 != 0 {
        unsafe {
            png_app_error(png_ptr, CStr::from_bytes_with_nul(b"PNG_TRANSFORM_INVERT_ALPHA not supported\0").unwrap().as_ptr());
        }
    }
    /* ----------------------- end of transformations ------------------- */
    /* Write the bits */
    unsafe {
        png_write_image(png_ptr, info.row_pointers);
        /* It is REQUIRED to call this to finish writing the rest of the file */
        png_write_end(png_ptr, info_ptr);
    }
}

/* WRITE */
/* SIMPLIFIED_WRITE */
