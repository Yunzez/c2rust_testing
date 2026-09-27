
extern "C" {
    
    
    
    
    
    #[no_mangle]
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

pub type png_struct_def = crate::src::libpng::png::png_struct_def;
pub type png_row_infop = crate::src::libpng::png::png_row_infop;
pub type png_row_info = crate::src::libpng::png::png_row_info;
// #[derive(Copy, Clone)]

pub type png_row_info_struct = crate::src::libpng::png::png_row_info_struct;
pub type uInt = crate::src::libpng::png::uInt;
pub type png_unknown_chunk = crate::src::libpng::png::png_unknown_chunk;
// #[derive(Copy, Clone)]

pub type png_unknown_chunk_t = crate::src::libpng::png::png_unknown_chunk_t;
pub type png_write_status_ptr = crate::src::libpng::png::png_write_status_ptr;
pub type png_structp = crate::src::libpng::png::png_structp;
pub type png_struct = crate::src::libpng::png::png_struct;
pub type png_read_status_ptr = crate::src::libpng::png::png_read_status_ptr;
pub type png_color_16 = crate::src::libpng::png::png_color_16;
// #[derive(Copy, Clone)]

pub type png_color_16_struct = crate::src::libpng::png::png_color_16_struct;
pub type png_color_8 = crate::src::libpng::png::png_color_8;
// #[derive(Copy, Clone)]

pub type png_color_8_struct = crate::src::libpng::png::png_color_8_struct;
pub type png_flush_ptr = crate::src::libpng::png::png_flush_ptr;
pub type png_colorp = crate::src::libpng::png::png_colorp;
pub type png_color = crate::src::libpng::png::png_color;
// #[derive(Copy, Clone)]

pub type png_color_struct = crate::src::libpng::png::png_color_struct;
pub type png_compression_bufferp = crate::src::libpng::png::png_compression_bufferp;
// #[derive(Copy, Clone)]

pub type png_compression_buffer = crate::src::libpng::png::png_compression_buffer;
pub type z_stream = crate::src::libpng::png::z_stream;
// #[derive(Copy, Clone)]

pub type z_stream_s = crate::src::libpng::png::z_stream_s;
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

pub type png_info_def = crate::src::libpng::png::png_info_def;
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
#[no_mangle]
pub unsafe extern "C" fn png_create_read_struct(mut user_png_ver:
                                                    png_const_charp,
                                                mut error_ptr: png_voidp,
                                                mut error_fn: png_error_ptr,
                                                mut warn_fn: png_error_ptr)
 -> png_structp {
    let mut png_ptr: png_structp =
        png_create_png_struct(user_png_ver, error_ptr, error_fn, warn_fn,
                              0 as *mut std::os::raw::c_void, None, None);
    /* USER_MEM */
    if !png_ptr.is_null() {
        (*png_ptr).mode = 0x8000 as std::os::raw::c_uint;
        /* Added in libpng-1.6.0; this can be used to detect a read structure if
       * required (it will be zero in a write structure.)
       */
        (*png_ptr).IDAT_read_size = 8192 as std::os::raw::c_int as uInt;
        (*png_ptr).flags |= 0x100000 as std::os::raw::c_uint;
        /* In stable builds only warn if an application error can be completely
          * handled.
          */
        (*png_ptr).flags |= 0x200000 as std::os::raw::c_uint;
        /* TODO: delay this, it can be done in png_init_io (if the app doesn't
       * do it itself) avoiding setting the default function if it is not
       * required.
       */
        png_set_read_fn(png_ptr, 0 as *mut std::os::raw::c_void, None);
    }
    return png_ptr;
}
/* Read the information before the actual image data.  This has been
 * changed in v0.90 to allow reading a file that already has the magic
 * bytes read from the stream.  You can tell libpng how many bytes have
 * been read from the beginning of the stream (up to the maximum of 8)
 * via png_set_sig_bytes(), and we will only check the remaining bytes
 * here.  The application can then have access to the signature bytes we
 * read if it is determined that this isn't a valid PNG file.
 */
#[no_mangle]
pub unsafe extern "C" fn png_read_info(mut png_ptr: png_structrp,
                                       mut info_ptr: png_inforp) {
    let mut keep: std::os::raw::c_int = 0;
    if png_ptr.is_null() || info_ptr.is_null() { return }
    /* Read and check the PNG file signature. */
    png_read_sig(png_ptr, info_ptr);
    loop  {
        let mut length: png_uint_32 = png_read_chunk_header(png_ptr);
        let mut chunk_name: png_uint_32 = (*png_ptr).chunk_name;
        /* IDAT logic needs to happen here to simplify getting the two flags
       * right.
       */
        if chunk_name ==
               (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
                   (68 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
                   (65 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
                   (84 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
            if (*png_ptr).mode & 0x1 as std::os::raw::c_int as std::os::raw::c_uint ==
                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                png_chunk_error(png_ptr,
                                b"Missing IHDR before IDAT\x00" as *const u8
                                    as *const std::os::raw::c_char);
            } else if (*png_ptr).color_type as std::os::raw::c_int ==
                          2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
                          (*png_ptr).mode & 0x2 as std::os::raw::c_int as std::os::raw::c_uint
                              == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                png_chunk_error(png_ptr,
                                b"Missing PLTE before IDAT\x00" as *const u8
                                    as *const std::os::raw::c_char);
            } else if (*png_ptr).mode & 0x8 as std::os::raw::c_int as std::os::raw::c_uint !=
                          0 as std::os::raw::c_int as std::os::raw::c_uint {
                png_chunk_benign_error(png_ptr,
                                       b"Too many IDATs found\x00" as
                                           *const u8 as *const std::os::raw::c_char);
            }
            (*png_ptr).mode |= 0x4 as std::os::raw::c_uint
        } else if (*png_ptr).mode & 0x4 as std::os::raw::c_uint !=
                      0 as std::os::raw::c_int as std::os::raw::c_uint {
            (*png_ptr).mode |= 0x2000 as std::os::raw::c_uint;
            (*png_ptr).mode |= 0x8 as std::os::raw::c_int as std::os::raw::c_uint
        }
        /* This should be a binary subdivision search or a hash for
       * matching the chunk name rather than a linear search.
       */
        if chunk_name ==
               (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
                   (72 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
                   (68 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
                   (82 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
            png_handle_IHDR(png_ptr, info_ptr,
                            length); /* It has been consumed */
        } else if chunk_name ==
                      (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int
                          |
                          (69 as std::os::raw::c_int as png_uint_32) <<
                              16 as std::os::raw::c_int |
                          (78 as std::os::raw::c_int as png_uint_32) <<
                              8 as std::os::raw::c_int |
                          (68 as std::os::raw::c_int as png_uint_32) <<
                              0 as std::os::raw::c_int {
            png_handle_IEND(png_ptr, info_ptr, length);
        } else {
            keep = png_chunk_unknown_handling(png_ptr, chunk_name);
            if keep != 0 as std::os::raw::c_int {
                png_handle_unknown(png_ptr, info_ptr, length, keep);
                if chunk_name ==
                       (80 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int
                           |
                           (76 as std::os::raw::c_int as png_uint_32) <<
                               16 as std::os::raw::c_int |
                           (84 as std::os::raw::c_int as png_uint_32) <<
                               8 as std::os::raw::c_int |
                           (69 as std::os::raw::c_int as png_uint_32) <<
                               0 as std::os::raw::c_int {
                    (*png_ptr).mode |= 0x2 as std::os::raw::c_int as std::os::raw::c_uint
                } else {
                    if !(chunk_name ==
                             (73 as std::os::raw::c_int as png_uint_32) <<
                                 24 as std::os::raw::c_int |
                                 (68 as std::os::raw::c_int as png_uint_32) <<
                                     16 as std::os::raw::c_int |
                                 (65 as std::os::raw::c_int as png_uint_32) <<
                                     8 as std::os::raw::c_int |
                                 (84 as std::os::raw::c_int as png_uint_32) <<
                                     0 as std::os::raw::c_int) {
                        continue ;
                    }
                    (*png_ptr).idat_size = 0 as std::os::raw::c_int as png_uint_32;
                    break ;
                }
            } else if chunk_name ==
                          (80 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (76 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (69 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_PLTE(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (73 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (68 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (65 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                (*png_ptr).idat_size = length;
                break ;
            } else if chunk_name ==
                          (98 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (75 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (71 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (68 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_bKGD(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (104 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (73 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (83 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_hIST(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (115 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (66 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (73 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_sBIT(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (116 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (82 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (78 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (83 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_tRNS(png_ptr, info_ptr, length);
            } else {
                png_handle_unknown(png_ptr, info_ptr, length,
                                   0 as std::os::raw::c_int);
            }
        }
    };
}
/* SEQUENTIAL_READ */
/* Optional call to update the users info_ptr structure */
#[no_mangle]
pub unsafe extern "C" fn png_read_update_info(mut png_ptr: png_structrp,
                                              mut info_ptr: png_inforp) {
    if !png_ptr.is_null() {
        if (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_read_start_row(png_ptr);
            png_read_transform_info(png_ptr, info_ptr);
        } else {
            /* New in 1.6.0 this avoids the bug of doing the initializations twice */
            png_app_error(png_ptr,
                          b"png_read_update_info/png_start_read_image: duplicate call\x00"
                              as *const u8 as *const std::os::raw::c_char);
        }
    };
}
/* Initialize palette, background, etc, after transformations
 * are set, but before any reading takes place.  This allows
 * the user to obtain a gamma-corrected palette, for example.
 * If the user doesn't call this, we will do it ourselves.
 */
#[no_mangle]
pub unsafe extern "C" fn png_start_read_image(mut png_ptr: png_structrp) {
    if !png_ptr.is_null() {
        if (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_read_start_row(png_ptr);
        } else {
            /* New in 1.6.0 this avoids the bug of doing the initializations twice */
            png_app_error(png_ptr,
                          b"png_start_read_image/png_read_update_info: duplicate call\x00"
                              as *const u8 as *const std::os::raw::c_char);
        }
    };
}
/* SEQUENTIAL_READ */
/* MNG_FEATURES */
#[no_mangle]
pub unsafe extern "C" fn png_read_row(mut png_ptr: png_structrp,
                                      mut row: png_bytep,
                                      mut dsp_row: png_bytep) {
    let mut row_info: png_row_info =
        png_row_info{width: 0,
                     rowbytes: 0,
                     color_type: 0,
                     bit_depth: 0,
                     channels: 0,
                     pixel_depth: 0,};
    if png_ptr.is_null() { return }
    /* png_read_start_row sets the information (in particular iwidth) for this
    * interlace pass.
    */
    if (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_read_start_row(png_ptr);
    }
    /* 1.5.6: row_info moved out of png_struct to a local here. */
    row_info.width =
        (*png_ptr).iwidth; /* NOTE: width of current interlaced row */
    row_info.color_type = (*png_ptr).color_type;
    row_info.bit_depth = (*png_ptr).bit_depth;
    row_info.channels = (*png_ptr).channels;
    row_info.pixel_depth = (*png_ptr).pixel_depth;
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
    ((*png_ptr).row_number == 0 as std::os::raw::c_int as std::os::raw::c_uint) &&
        (*png_ptr).pass as std::os::raw::c_int == 0 as std::os::raw::c_int;
    /* WARNINGS */
    /* If interlaced and we do not need a new row, combine row and return.
    * Notice that the pixels we have from previous rows have been transformed
    * already; we can only combine like with like (transformed or
    * untransformed) and, because of the libpng API for interlaced images, this
    * means we must transform before de-interlacing.
    */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int &&
           (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        match (*png_ptr).pass as std::os::raw::c_int {
            0 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            1 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 ||
                       (*png_ptr).width < 5 as std::os::raw::c_int as std::os::raw::c_uint {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            2 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 4 as std::os::raw::c_int as std::os::raw::c_uint {
                    if !dsp_row.is_null() &&
                           (*png_ptr).row_number &
                               4 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            3 => {
                if (*png_ptr).row_number & 3 as std::os::raw::c_int as std::os::raw::c_uint !=
                       0 ||
                       (*png_ptr).width < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            4 => {
                if (*png_ptr).row_number & 3 as std::os::raw::c_int as std::os::raw::c_uint !=
                       2 as std::os::raw::c_int as std::os::raw::c_uint {
                    if !dsp_row.is_null() &&
                           (*png_ptr).row_number &
                               2 as std::os::raw::c_int as std::os::raw::c_uint != 0 {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            5 => {
                if (*png_ptr).row_number & 1 as std::os::raw::c_int as std::os::raw::c_uint !=
                       0 ||
                       (*png_ptr).width < 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    if !dsp_row.is_null() {
                        png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
                    }
                    png_read_finish_row(png_ptr);
                    return
                }
            }
            6 | _ => {
                if (*png_ptr).row_number & 1 as std::os::raw::c_int as std::os::raw::c_uint ==
                       0 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_read_finish_row(png_ptr);
                    return
                }
            }
        }
    }
    if (*png_ptr).mode & 0x4 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr,
                  b"Invalid attempt to read row data\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Fill the row with IDAT data: */
    *(*png_ptr).row_buf.offset(0 as std::os::raw::c_int as isize) =
        255 as std::os::raw::c_int as
            png_byte; /* to force error if no data was found */
    png_read_IDAT_data(png_ptr, (*png_ptr).row_buf,
                       row_info.rowbytes.wrapping_add(1 as std::os::raw::c_int as
                                                          std::os::raw::c_ulong));
    if *(*png_ptr).row_buf.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int >
           0 as std::os::raw::c_int {
        if (*(*png_ptr).row_buf.offset(0 as std::os::raw::c_int as isize) as
                std::os::raw::c_int) < 5 as std::os::raw::c_int {
            png_read_filter_row(png_ptr, &mut row_info,
                                (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as
                                                              isize),
                                (*png_ptr).prev_row.offset(1 as std::os::raw::c_int as
                                                               isize) as
                                    png_const_bytep,
                                *(*png_ptr).row_buf.offset(0 as std::os::raw::c_int as
                                                               isize) as
                                    std::os::raw::c_int);
        } else {
            png_error(png_ptr,
                      b"bad adaptive filter value\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
    }
    /* libpng 1.5.6: the following line was copying png_ptr->rowbytes before
    * 1.5.6, while the buffer really is this big in current versions of libpng
    * it may not be in the future, so this was changed just to copy the
    * interlaced count:
    */
    memcpy((*png_ptr).prev_row as *mut std::os::raw::c_void,
           (*png_ptr).row_buf as *const std::os::raw::c_void,
           row_info.rowbytes.wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong));
    if (*png_ptr).transformations != 0 {
        png_do_read_transformations(png_ptr, &mut row_info);
    }
    /* The transformed pixel depth should match the depth now in row_info. */
    if (*png_ptr).transformed_pixel_depth as std::os::raw::c_int == 0 as std::os::raw::c_int {
        (*png_ptr).transformed_pixel_depth = row_info.pixel_depth;
        if row_info.pixel_depth as std::os::raw::c_int >
               (*png_ptr).maximum_pixel_depth as std::os::raw::c_int {
            png_error(png_ptr,
                      b"sequential row overflow\x00" as *const u8 as
                          *const std::os::raw::c_char);
        }
    } else if (*png_ptr).transformed_pixel_depth as std::os::raw::c_int !=
                  row_info.pixel_depth as std::os::raw::c_int {
        png_error(png_ptr,
                  b"internal sequential row size calculation error\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    /* Expand interlaced rows to full size */
    if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int &&
           (*png_ptr).transformations & 0x2 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        if ((*png_ptr).pass as std::os::raw::c_int) < 6 as std::os::raw::c_int {
            png_do_read_interlace(&mut row_info,
                                  (*png_ptr).row_buf.offset(1 as std::os::raw::c_int
                                                                as isize),
                                  (*png_ptr).pass as std::os::raw::c_int,
                                  (*png_ptr).transformations);
        }
        if !dsp_row.is_null() {
            png_combine_row(png_ptr, dsp_row, 1 as std::os::raw::c_int);
        }
        if !row.is_null() { png_combine_row(png_ptr, row, 0 as std::os::raw::c_int); }
    } else {
        if !row.is_null() {
            png_combine_row(png_ptr, row, -(1 as std::os::raw::c_int));
        }
        if !dsp_row.is_null() {
            png_combine_row(png_ptr, dsp_row, -(1 as std::os::raw::c_int));
        }
    }
    png_read_finish_row(png_ptr);
    if (*png_ptr).read_row_fn.is_some() {
        Some((*png_ptr).read_row_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                             (*png_ptr).row_number,
                                                                                                             (*png_ptr).pass
                                                                                                                 as
                                                                                                                 std::os::raw::c_int);
    };
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
#[no_mangle]
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
#[no_mangle]
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
        pass = png_set_interlace_handling(png_ptr);
        /* And make sure transforms are initialized. */
        png_start_read_image(png_ptr);
    } else {
        if (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int &&
               (*png_ptr).transformations & 0x2 as std::os::raw::c_uint ==
                   0 as std::os::raw::c_int as std::os::raw::c_uint {
            /* Caller called png_start_read_image or png_read_update_info without
          * first turning on the PNG_INTERLACE transform.  We can fix this here,
          * but the caller should do it!
          */
            png_warning(png_ptr,
                        b"Interlace handling should be turned on when using png_read_image\x00"
                            as *const u8 as *const std::os::raw::c_char);
            /* Make sure this is set correctly */
            (*png_ptr).num_rows = (*png_ptr).height
        }
        /* Obtain the pass number, which also turns on the PNG_INTERLACE flag in
       * the above error case.
       */
        pass = png_set_interlace_handling(png_ptr)
    }
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
#[no_mangle]
pub unsafe extern "C" fn png_read_end(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp) {
    let mut keep: std::os::raw::c_int = 0;
    if png_ptr.is_null() { return }
    /* If png_read_end is called in the middle of reading the rows there may
    * still be pending IDAT data and an owned zstream.  Deal with this here.
    */
    if png_chunk_unknown_handling(png_ptr,
                                  (73 as std::os::raw::c_int as png_uint_32) <<
                                      24 as std::os::raw::c_int |
                                      (68 as std::os::raw::c_int as png_uint_32) <<
                                          16 as std::os::raw::c_int |
                                      (65 as std::os::raw::c_int as png_uint_32) <<
                                          8 as std::os::raw::c_int |
                                      (84 as std::os::raw::c_int as png_uint_32) <<
                                          0 as std::os::raw::c_int) ==
           0 as std::os::raw::c_int {
        png_read_finish_IDAT(png_ptr);
    }
    /* Report invalid palette index; added at libng-1.5.10 */
    if (*png_ptr).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           (*png_ptr).num_palette_max > (*png_ptr).num_palette as std::os::raw::c_int
       {
        png_benign_error(png_ptr,
                         b"Read palette index exceeding num_palette\x00" as
                             *const u8 as *const std::os::raw::c_char);
    }
    loop  {
        let mut length: png_uint_32 = png_read_chunk_header(png_ptr);
        let mut chunk_name: png_uint_32 = (*png_ptr).chunk_name;
        if chunk_name !=
               (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
                   (68 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
                   (65 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
                   (84 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
            (*png_ptr).mode |= 0x2000 as std::os::raw::c_uint
        }
        if chunk_name ==
               (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int |
                   (69 as std::os::raw::c_int as png_uint_32) << 16 as std::os::raw::c_int |
                   (78 as std::os::raw::c_int as png_uint_32) << 8 as std::os::raw::c_int |
                   (68 as std::os::raw::c_int as png_uint_32) << 0 as std::os::raw::c_int {
            png_handle_IEND(png_ptr, info_ptr, length);
        } else if chunk_name ==
                      (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int
                          |
                          (72 as std::os::raw::c_int as png_uint_32) <<
                              16 as std::os::raw::c_int |
                          (68 as std::os::raw::c_int as png_uint_32) <<
                              8 as std::os::raw::c_int |
                          (82 as std::os::raw::c_int as png_uint_32) <<
                              0 as std::os::raw::c_int {
            png_handle_IHDR(png_ptr, info_ptr, length);
        } else if info_ptr.is_null() {
            png_crc_finish(png_ptr, length);
        } else {
            keep = png_chunk_unknown_handling(png_ptr, chunk_name);
            if keep != 0 as std::os::raw::c_int {
                if chunk_name ==
                       (73 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int
                           |
                           (68 as std::os::raw::c_int as png_uint_32) <<
                               16 as std::os::raw::c_int |
                           (65 as std::os::raw::c_int as png_uint_32) <<
                               8 as std::os::raw::c_int |
                           (84 as std::os::raw::c_int as png_uint_32) <<
                               0 as std::os::raw::c_int {
                    if length > 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                           (*png_ptr).flags & 0x8 as std::os::raw::c_uint == 0 ||
                           (*png_ptr).mode & 0x2000 as std::os::raw::c_uint !=
                               0 as std::os::raw::c_int as std::os::raw::c_uint {
                        png_benign_error(png_ptr,
                                         b".Too many IDATs found\x00" as
                                             *const u8 as
                                             *const std::os::raw::c_char);
                    }
                }
                png_handle_unknown(png_ptr, info_ptr, length, keep);
                if chunk_name ==
                       (80 as std::os::raw::c_int as png_uint_32) << 24 as std::os::raw::c_int
                           |
                           (76 as std::os::raw::c_int as png_uint_32) <<
                               16 as std::os::raw::c_int |
                           (84 as std::os::raw::c_int as png_uint_32) <<
                               8 as std::os::raw::c_int |
                           (69 as std::os::raw::c_int as png_uint_32) <<
                               0 as std::os::raw::c_int {
                    (*png_ptr).mode |= 0x2 as std::os::raw::c_int as std::os::raw::c_uint
                }
            } else if chunk_name ==
                          (73 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (68 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (65 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                /* Zero length IDATs are legal after the last IDAT has been
          * read, but not after other chunks have been read.  1.6 does not
          * always read all the deflate data; specifically it cannot be relied
          * upon to read the Adler32 at the end.  If it doesn't ignore IDAT
          * chunks which are longer than zero as well:
          */
                if length > 0 as std::os::raw::c_int as std::os::raw::c_uint &&
                       (*png_ptr).flags & 0x8 as std::os::raw::c_uint == 0 ||
                       (*png_ptr).mode & 0x2000 as std::os::raw::c_uint !=
                           0 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_benign_error(png_ptr,
                                     b"..Too many IDATs found\x00" as
                                         *const u8 as *const std::os::raw::c_char);
                }
                png_crc_finish(png_ptr, length);
            } else if chunk_name ==
                          (80 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (76 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (69 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_PLTE(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (98 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (75 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (71 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (68 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_bKGD(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (104 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (73 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (83 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_hIST(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (115 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (66 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (73 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (84 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_sBIT(png_ptr, info_ptr, length);
            } else if chunk_name ==
                          (116 as std::os::raw::c_int as png_uint_32) <<
                              24 as std::os::raw::c_int |
                              (82 as std::os::raw::c_int as png_uint_32) <<
                                  16 as std::os::raw::c_int |
                              (78 as std::os::raw::c_int as png_uint_32) <<
                                  8 as std::os::raw::c_int |
                              (83 as std::os::raw::c_int as png_uint_32) <<
                                  0 as std::os::raw::c_int {
                png_handle_tRNS(png_ptr, info_ptr, length);
            } else {
                png_handle_unknown(png_ptr, info_ptr, length,
                                   0 as std::os::raw::c_int);
            }
        }
        if !((*png_ptr).mode & 0x10 as std::os::raw::c_uint ==
                 0 as std::os::raw::c_int as std::os::raw::c_uint) {
            break ;
        }
    };
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
#[no_mangle]
pub unsafe extern "C" fn png_destroy_read_struct(mut png_ptr_ptr:
                                                     png_structpp,
                                                 mut info_ptr_ptr: png_infopp,
                                                 mut end_info_ptr_ptr:
                                                     png_infopp) {
    let mut png_ptr: png_structrp = 0 as png_structrp;
    if !png_ptr_ptr.is_null() { png_ptr = *png_ptr_ptr }
    if png_ptr.is_null() { return }
    /* libpng 1.6.0: use the API to destroy info structs to ensure consistent
    * behavior.  Prior to 1.6.0 libpng did extra 'info' destruction in this API.
    * The extra was, apparently, unnecessary yet this hides memory leak bugs.
    */
    png_destroy_info_struct(png_ptr, end_info_ptr_ptr);
    png_destroy_info_struct(png_ptr, info_ptr_ptr);
    *png_ptr_ptr = 0 as *mut png_struct;
    png_read_destroy(png_ptr);
    png_destroy_png_struct(png_ptr);
}
#[no_mangle]
pub unsafe extern "C" fn png_set_read_status_fn(mut png_ptr: png_structrp,
                                                mut read_row_fn:
                                                    png_read_status_ptr) {
    if png_ptr.is_null() { return }
    (*png_ptr).read_row_fn = read_row_fn;
}
#[no_mangle]
pub unsafe extern "C" fn png_read_png(mut png_ptr: png_structrp,
                                      mut info_ptr: png_inforp,
                                      mut transforms: std::os::raw::c_int,
                                      mut params: voidp) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    /* png_read_info() gives us all of the information from the
    * PNG file before the first IDAT (image data chunk).
    */
    png_read_info(png_ptr, info_ptr);
    if (*info_ptr).height as std::os::raw::c_ulong >
           (-(1 as std::os::raw::c_int) as png_uint_32 as
                std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<png_bytep>()
                                                as std::os::raw::c_ulong) {
        png_error(png_ptr,
                  b"Image is too high to process with png_read_png()\x00" as
                      *const u8 as *const std::os::raw::c_char);
    }
    /* -------------- image transformations start here ------------------- */
   /* libpng 1.6.10: add code to cause a png_app_error if a selected TRANSFORM
    * is not implemented.  This will only happen in de-configured (non-default)
    * libpng builds.  The results can be unexpected - png_read_png may return
    * short or mal-formed rows because the transform is skipped.
    */
    /* Tell libpng to strip 16-bit/color files down to 8 bits per color.
    */
    if transforms & 0x8000 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        /* Added at libpng-1.5.4. "strip_16" produces the same result that it
       * did in earlier versions, while "scale_16" is now more accurate.
       */
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SCALE_16 not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* If both SCALE and STRIP are required pngrtran will effectively cancel the
    * latter by doing SCALE first.  This is ok and allows apps not to check for
    * which is supported to get the right answer.
    */
    if transforms & 0x1 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_STRIP_16 not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* Strip alpha bytes from the input data without combining with
    * the background (not recommended).
    */
    if transforms & 0x2 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_STRIP_ALPHA not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Extract multiple pixels with bit depths of 1, 2, or 4 from a single
    * byte into separate bytes (useful for paletted and grayscale images).
    */
    if transforms & 0x4 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_PACKING not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* Change the order of packed pixels to least significant bit first
    * (not useful if you are using png_set_packing).
    */
    if transforms & 0x8 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_PACKSWAP not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* Expand paletted colors into true RGB triplets
    * Expand grayscale images to full 8 bits from 1, 2, or 4 bits/pixel
    * Expand paletted or RGB images with transparency to full alpha
    * channels so the data will be available as RGBA quartets.
    */
    if transforms & 0x10 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_EXPAND not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* We don't handle background color or gamma transformation or quantizing.
    */
    /* Invert monochrome files to have 0 as white and 1 as black
    */
    if transforms & 0x20 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_INVERT_MONO not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* If you want to shift the pixel values from the range [0,255] or
    * [0,65535] to the original [0,7] or [0,31], or whatever range the
    * colors were originally in:
    */
    if transforms & 0x40 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SHIFT not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
    }
    /* Flip the RGB pixels to BGR (or RGBA to BGRA) */
    if transforms & 0x80 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_BGR not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
    }
    /* Swap the RGBA or GA data to ARGB or AG (or BGRA to ABGR) */
    if transforms & 0x100 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SWAP_ALPHA not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Swap bytes of 16-bit files to least significant byte first */
    if transforms & 0x200 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SWAP_ENDIAN not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Added at libpng-1.2.41 */
   /* Invert the alpha channel from opacity to transparency */
    if transforms & 0x400 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_INVERT_ALPHA not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Added at libpng-1.2.41 */
   /* Expand grayscale image to RGB */
    if transforms & 0x2000 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_GRAY_TO_RGB not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Added at libpng-1.5.4 */
    if transforms & 0x4000 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_EXPAND_16 not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* We don't handle adding filler bytes */
    /* We use png_read_image and rely on that for interlace handling, but we also
    * call png_read_update_info therefore must turn on interlace handling now:
    */
    png_set_interlace_handling(png_ptr);
    /* Optional call to gamma correct and add the background to the palette
    * and update info structure.  REQUIRED if you are expecting libpng to
    * update the palette for you (i.e., you selected such a transform above).
    */
    png_read_update_info(png_ptr, info_ptr);
    /* -------------- image transformations end here ------------------- */
    png_free_data(png_ptr, info_ptr, 0x40 as std::os::raw::c_uint, 0 as std::os::raw::c_int);
    if (*info_ptr).row_pointers.is_null() {
        let mut iptr: png_uint_32 = 0;
        (*info_ptr).row_pointers =
            png_malloc(png_ptr,
                       ((*info_ptr).height as
                            std::os::raw::c_ulong).wrapping_mul(::std::mem::size_of::<png_bytep>()
                                                            as std::os::raw::c_ulong))
                as png_bytepp;
        iptr = 0 as std::os::raw::c_int as png_uint_32;
        while iptr < (*info_ptr).height {
            let ref mut fresh2 =
                *(*info_ptr).row_pointers.offset(iptr as isize);
            *fresh2 = 0 as *mut png_byte;
            iptr = iptr.wrapping_add(1)
        }
        (*info_ptr).free_me |= 0x40 as std::os::raw::c_uint;
        iptr = 0 as std::os::raw::c_int as png_uint_32;
        while iptr < (*info_ptr).height {
            let ref mut fresh3 =
                *(*info_ptr).row_pointers.offset(iptr as isize);
            *fresh3 =
                png_malloc(png_ptr, (*info_ptr).rowbytes) as *mut png_byte;
            iptr = iptr.wrapping_add(1)
        }
    }
    png_read_image(png_ptr, (*info_ptr).row_pointers);
    (*info_ptr).valid |= 0x8000 as std::os::raw::c_uint;
    /* Read rest of file, and get additional chunks in info_ptr - REQUIRED */
    png_read_end(png_ptr, info_ptr);
}
/* READ */
/* SIMPLIFIED_READ */
/* SEQUENTIAL_READ */
/* INFO_IMAGE */
