
extern "C" {
    
    
    
    
    
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
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
pub type png_const_uint_16p = * const std::os::raw::c_ushort;
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
pub type png_structpp = * mut * mut crate::src::libpng::png::png_struct_def;
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::png::png_info_def;
pub type png_unknown_chunkp = * mut crate::src::libpng::png::png_unknown_chunk_t;
pub type png_info = crate::src::libpng::png::png_info_def;
pub type png_infopp = * mut * mut crate::src::libpng::png::png_info_def;
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
pub type png_inforp = * mut crate::src::libpng::png::png_info_def;
pub type png_const_inforp = * const crate::src::libpng::png::png_info_def;
pub type png_const_colorp = * const crate::src::libpng::png::png_color_struct;
pub type png_const_color_16p = * const crate::src::libpng::png::png_color_16_struct;
pub type png_const_color_8p = * const crate::src::libpng::png::png_color_8_struct;
pub type png_const_unknown_chunkp = * const crate::src::libpng::png::png_unknown_chunk_t;
pub type png_malloc_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: std::os::raw::c_ulong,) -> Option<&'a2 mut core::ffi::c_void>>;
pub type png_free_ptr<'a1, 'a2> = Option<unsafe extern "C"  fn(_: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,_: Option<&'a2 mut core::ffi::c_void>,) -> ()>;
pub type z_streamp = * mut crate::src::libpng::png::z_stream_s;
pub type voidp = * mut core::ffi::c_void;
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
unsafe extern "C" fn write_unknown_chunks(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                          mut info_ptr: * const crate::src::libpng::png::png_info_def,
                                          mut where_0: std::os::raw::c_uint) {
    if (*info_ptr).unknown_chunks_num != 0 as std::os::raw::c_int {
        let mut up: * const crate::src::libpng::png::png_unknown_chunk_t = 0 as *const png_unknown_chunk;
        up = (*info_ptr).unknown_chunks as png_const_unknown_chunkp;
        while up <
                  (*info_ptr).unknown_chunks.offset((*info_ptr).unknown_chunks_num
                                                        as isize) as
                      png_const_unknown_chunkp {
            if (*up).location as std::os::raw::c_uint & where_0 !=
                   0 as std::os::raw::c_int as std::os::raw::c_uint {
                /* If per-chunk unknown chunk handling is enabled use it, otherwise
          * just write the chunks the application has set.
          */
                let mut keep: i32 =
                    png_handle_as_unknown(png_ptr, (*up).name.as_ptr());
                /* NOTE: this code is radically different from the read side in the
          * matter of handling an ancillary unknown chunk.  In the read side
          * the default behavior is to discard it, in the code below the default
          * behavior is to write it.  Critical chunks are, however, only
          * written if explicitly listed or if the default is set to write all
          * unknown chunks.
          *
          * The default handling is also slightly weird - it is not possible to
          * stop the writing of all unsafe-to-copy chunks!
          *
          * TODO: REVIEW: this would seem to be a bug.
          */
                if keep != 1 as std::os::raw::c_int &&
                       ((*up).name[3 as std::os::raw::c_int as usize] as std::os::raw::c_int &
                            0x20 as std::os::raw::c_int != 0 ||
                            keep == 3 as std::os::raw::c_int ||
                            keep == 0 as std::os::raw::c_int &&
                                (*png_ptr).unknown_default ==
                                    3 as std::os::raw::c_int) {
                    /* TODO: review, what is wrong with a zero length unknown chunk? */
                    if (*up).size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                        png_warning(png_ptr,
                                    b"Writing zero-length unknown chunk\x00"
                                        as *const u8 as *const std::os::raw::c_char);
                    }
                    png_write_chunk(png_ptr, (*up).name.as_ptr(),
                                    (*up).data as png_const_bytep,
                                    (*up).size);
                }
            }
            up = up.offset(1)
        }
    };
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
#[no_mangle]
pub unsafe extern "C" fn png_write_info_before_PLTE(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                    mut info_ptr:
                                                        * const crate::src::libpng::png::png_info_def) {
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
            png_write_sBIT(png_ptr, (Some(&(*info_ptr).sig_bit)).clone(),
                           (*info_ptr).color_type as std::os::raw::c_int);
        }
        write_unknown_chunks(png_ptr, info_ptr,
                             0x1 as std::os::raw::c_int as std::os::raw::c_uint);
        (*png_ptr).mode |= 0x400 as std::os::raw::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_write_info(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                        mut info_ptr: * const crate::src::libpng::png::png_info_def) {
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
                       (Some(&(*info_ptr).trans_color)).clone(),
                       (*info_ptr).num_trans as std::os::raw::c_int,
                       (*info_ptr).color_type as std::os::raw::c_int);
    }
    if (*info_ptr).valid & 0x20 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_write_bKGD(png_ptr, (Some(&(*info_ptr).background)).clone(),
                       (*info_ptr).color_type as std::os::raw::c_int);
    }
    if (*info_ptr).valid & 0x40 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_write_hIST(png_ptr, (*info_ptr).hist as png_const_uint_16p,
                       (*info_ptr).num_palette as std::os::raw::c_int);
    }
    /* sCAL */
    /* pHYs */
    /* tIME */
    /* sPLT */
    /* tEXt */
    write_unknown_chunks(png_ptr, info_ptr,
                         0x2 as std::os::raw::c_int as std::os::raw::c_uint);
}
/* Writes the end of the PNG file.  If you don't want to write comments or
 * time information, you can pass NULL for info.  If you already wrote these
 * in png_write_info(), do not write them again here.  If you have long
 * comments, I suggest writing them here, and compressing them.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_end(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def) {
    if png_ptr.is_null() { return }
    if (*png_ptr).mode & 0x4 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_error(png_ptr,
                  b"No IDATs written into file\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    if (*png_ptr).num_palette_max > (*png_ptr).num_palette as std::os::raw::c_int {
        png_benign_error(png_ptr,
                         b"Wrote palette index exceeding num_palette\x00" as
                             *const u8 as *const std::os::raw::c_char);
    }
    /* See if user wants us to write information chunks */
    if !info_ptr.is_null() {
        write_unknown_chunks(png_ptr, info_ptr,
                             0x8 as std::os::raw::c_int as std::os::raw::c_uint);
    }
    (*png_ptr).mode |= 0x8 as std::os::raw::c_int as std::os::raw::c_uint;
    /* Write end of PNG file */
    png_write_IEND(png_ptr);
    /* This flush, added in libpng-1.0.8, removed from libpng-1.0.9beta03,
    * and restored again in libpng-1.2.30, may cause some applications that
    * do not set png_ptr->output_flush_fn to crash.  If your application
    * experiences a problem, please try building libpng with
    * PNG_WRITE_FLUSH_AFTER_IEND_SUPPORTED defined, and report the event to
    * png-mng-implement at lists.sf.net .
    */
}
/* Initialize png_ptr structure, and allocate any memory needed */
#[no_mangle]
pub unsafe extern "C" fn png_create_write_struct(mut user_png_ver:
                                                     * const std::os::raw::c_char,
                                                 mut error_ptr: * mut core::ffi::c_void,
                                                 mut error_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
                                                 mut warn_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>)
 -> * mut crate::src::libpng::png::png_struct_def {
    let mut png_ptr: * mut crate::src::libpng::png::png_struct_def =
        png_create_png_struct(user_png_ver, error_ptr, error_fn, warn_fn,
                              (0 as * mut core::ffi::c_void), None, None);
    /* USER_MEM */
    if !png_ptr.is_null() {
        /* Set the zlib control values to defaults; they can be overridden by the
       * application after the struct has been created.
       */
        (*png_ptr).zbuffer_size = 8192 as std::os::raw::c_int as uInt;
        /* The 'zlib_strategy' setting is irrelevant because png_default_claim in
       * pngwutil.c defaults it according to whether or not filters will be
       * used, and ignores this setting.
       */
        (*png_ptr).zlib_strategy = 1 as std::os::raw::c_int;
        (*png_ptr).zlib_level = -(1 as std::os::raw::c_int);
        (*png_ptr).zlib_mem_level = 8 as std::os::raw::c_int;
        (*png_ptr).zlib_window_bits = 15 as std::os::raw::c_int;
        (*png_ptr).zlib_method = 8 as std::os::raw::c_int;
        /* WRITE_COMPRESSED_TEXT */
        /* This is a highly dubious configuration option; by default it is off,
       * but it may be appropriate for private builds that are testing
       * extensions not conformant to the current specification, or of
       * applications that must not fail to write at all costs!
       */
        /* App warnings are warnings in release (or release candidate) builds but
       * are errors during development.
       */
        (*png_ptr).flags |= 0x200000 as std::os::raw::c_uint;
        /* TODO: delay this, it can be done in png_init_io() (if the app doesn't
       * do it itself) avoiding setting the default function if it is not
       * required.
       */
        png_set_write_fn(png_ptr, 0 as *mut std::os::raw::c_void, None, None);
    }
    return png_ptr;
}
/* Write a few rows of image data.  If the image is interlaced,
 * either you will have to write the 7 sub images, or, if you
 * have called png_set_interlace_handling(), you will have to
 * "write" the image seven times.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_rows(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                        mut row: * mut * mut std::os::raw::c_uchar,
                                        mut num_rows: std::os::raw::c_uint) {
    let mut i: u32 = 0; /* row counter */
    let mut rp: * mut * mut u8 = (0 as * mut * mut u8); /* row pointer */
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
#[no_mangle]
pub unsafe extern "C" fn png_write_image(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                         mut image: * mut * mut std::os::raw::c_uchar) {
    let mut i: u32 = 0; /* row index */
    let mut pass: i32 = 0; /* pass variables */
    let mut num_pass: i32 = 0; /* points to current row */
    let mut rp: * mut * mut u8 = 0 as *mut *mut png_byte;
    if png_ptr.is_null() { return }
    /* Initialize interlace handling.  If image is not interlaced,
    * this will set pass to 1
    */
    num_pass = png_set_interlace_handling(png_ptr);
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
#[no_mangle]
pub unsafe extern "C" fn png_write_row(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut row: * const std::os::raw::c_uchar) {
    /* 1.5.6: moved from png_struct to be a local structure: */
    let mut row_info: crate::src::libpng::png::png_row_info_struct =
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
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            1 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 5 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            2 => {
                if (*png_ptr).row_number & 0x7 as std::os::raw::c_int as std::os::raw::c_uint
                       != 4 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            3 => {
                if (*png_ptr).row_number & 0x3 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 3 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            4 => {
                if (*png_ptr).row_number & 0x3 as std::os::raw::c_int as std::os::raw::c_uint
                       != 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            5 => {
                if (*png_ptr).row_number & 0x1 as std::os::raw::c_int as std::os::raw::c_uint
                       != 0 as std::os::raw::c_int as std::os::raw::c_uint ||
                       (*png_ptr).width < 2 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
                    return
                }
            }
            6 => {
                if (*png_ptr).row_number & 0x1 as std::os::raw::c_int as std::os::raw::c_uint
                       == 0 as std::os::raw::c_int as std::os::raw::c_uint {
                    png_write_finish_row(png_ptr);
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
        png_do_write_interlace(Some(&mut row_info),
                               (*png_ptr).row_buf.offset(1 as std::os::raw::c_int as
                                                             isize),
                               (*png_ptr).pass as std::os::raw::c_int);
        /* This should always get caught above, but still ... */
        if row_info.width == 0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_write_finish_row(png_ptr);
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
        png_do_check_palette_indexes(png_ptr, Some(&mut row_info));
    }
    /* Find a filter if necessary, filter the row and write it out. */
    png_write_find_filter(png_ptr, Some(&mut row_info));
    if (*png_ptr).write_row_fn.is_some() {
        Some((*png_ptr).write_row_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                              (*png_ptr).row_number,
                                                                                                              (*png_ptr).pass
                                                                                                                  as
                                                                                                                  std::os::raw::c_int);
    };
}
/* Set the automatic flush interval or 0 to turn flushing off */
#[no_mangle]
pub unsafe extern "C" fn png_set_flush<'a1>(mut png_ptr: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,
                                       mut nrows: std::os::raw::c_int) {
    if borrow(& png_ptr).is_none() { return }
    (*(borrow_mut(&mut png_ptr)).unwrap()).flush_dist =
        if nrows < 0 as std::os::raw::c_int {
            0 as std::os::raw::c_int as std::os::raw::c_uint
        } else { nrows as png_uint_32 };
}
/* Flush the current output buffers now */
#[no_mangle]
pub unsafe extern "C" fn png_write_flush(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    if png_ptr.is_null() { return }
    /* We have already written out all of the data */
    if (*png_ptr).row_number >= (*png_ptr).num_rows { return }
    png_compress_IDAT(png_ptr, 0 as png_const_bytep,
                      0 as std::os::raw::c_int as png_alloc_size_t, 2 as std::os::raw::c_int);
    (*png_ptr).flush_rows = 0 as std::os::raw::c_int as png_uint_32;
    png_flush(png_ptr);
}
/* WRITE_FLUSH */
/* Free any memory used in png_ptr struct without freeing the struct itself. */
unsafe extern "C" fn png_write_destroy(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    /* Free any memory zlib uses */
    if (*png_ptr).flags & 0x2 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        deflateEnd(&mut (*png_ptr).zstream);
    }
    /* Free our memory.  png_free checks NULL for us. */
    png_free_buffer_list(png_ptr, Some(&mut (*png_ptr).zbuffer_list));
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
#[no_mangle]
pub unsafe extern "C" fn png_destroy_write_struct<'a1, 'a2>(mut png_ptr_ptr:
                                                      Option<&'a1 mut * mut crate::src::libpng::png::png_struct_def>,
                                                  mut info_ptr_ptr:
                                                      Option<&'a2 mut * mut crate::src::libpng::png::png_info_def>) {
    if !borrow(& png_ptr_ptr).is_none() {
        let mut png_ptr: * mut crate::src::libpng::png::png_struct_def = *(borrow_mut(&mut png_ptr_ptr)).unwrap();
        if !png_ptr.is_null() {
            /* added in libpng 1.6.0 */
            png_destroy_info_struct(png_ptr, borrow_mut(&mut info_ptr_ptr));
            *(borrow_mut(&mut png_ptr_ptr)).unwrap() = 0 as *mut png_struct;
            png_write_destroy(png_ptr);
            png_destroy_png_struct(png_ptr);
        }
    };
}
/* Allow the application to select one or more row filters to use. */
#[no_mangle]
pub unsafe extern "C" fn png_set_filter(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                        mut method: std::os::raw::c_int,
                                        mut filters: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    if method == 0 as std::os::raw::c_int {
        let mut current_block_9: u64;
        match filters &
                  (0x8 as std::os::raw::c_int | 0x10 as std::os::raw::c_int |
                       0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                       0x80 as std::os::raw::c_int | 0x7 as std::os::raw::c_int) {
            5 | 6 | 7 => {
                png_app_error(png_ptr,
                              b"Unknown row filter for method 0\x00" as
                                  *const u8 as *const std::os::raw::c_char);
                current_block_9 = 12876746043691347196;
                /* WRITE_FILTER */
            }
            0 => { current_block_9 = 12876746043691347196; }
            1 => {
                (*png_ptr).do_filter = 0x10 as std::os::raw::c_int as png_byte;
                current_block_9 = 1054647088692577877;
            }
            2 => {
                (*png_ptr).do_filter = 0x20 as std::os::raw::c_int as png_byte;
                current_block_9 = 1054647088692577877;
            }
            3 => {
                (*png_ptr).do_filter = 0x40 as std::os::raw::c_int as png_byte;
                current_block_9 = 1054647088692577877;
            }
            4 => {
                (*png_ptr).do_filter = 0x80 as std::os::raw::c_int as png_byte;
                current_block_9 = 1054647088692577877;
            }
            _ => {
                (*png_ptr).do_filter = filters as png_byte;
                current_block_9 = 1054647088692577877;
            }
        }
        match current_block_9 {
            12876746043691347196 =>
            /* WRITE_FILTER */
            /* FALLTHROUGH */
            {
                (*png_ptr).do_filter = 0x8 as std::os::raw::c_int as png_byte
            }
            _ => { }
        }
        /* If we have allocated the row_buf, this means we have already started
       * with the image and we should have allocated all of the filter buffers
       * that have been selected.  If prev_row isn't already allocated, then
       * it is too late to start using the filters that need it, since we
       * will be missing the data in the previous row.  If an application
       * wants to start and stop using particular filters during compression,
       * it should start out with all of the filters, and then remove them
       * or add them back after the start of compression.
       *
       * NOTE: this is a nasty constraint on the code, because it means that the
       * prev_row buffer must be maintained even if there are currently no
       * 'prev_row' requiring filters active.
       */
        if !(*png_ptr).row_buf.is_null() {
            let mut num_filters: i32 = 0;
            let mut buf_size: u64 = 0;
            /* Repeat the checks in png_write_start_row; 1 pixel high or wide
          * images cannot benefit from certain filters.  If this isn't done here
          * the check below will fire on 1 pixel high images.
          */
            if (*png_ptr).height == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                filters &=
                    !(0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                          0x80 as std::os::raw::c_int)
            }
            if (*png_ptr).width == 1 as std::os::raw::c_int as std::os::raw::c_uint {
                filters &=
                    !(0x10 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                          0x80 as std::os::raw::c_int)
            }
            if filters &
                   (0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                        0x80 as std::os::raw::c_int) != 0 as std::os::raw::c_int &&
                   (*png_ptr).prev_row.is_null() {
                /* This is the error case, however it is benign - the previous row
             * is not available so the filter can't be used.  Just warn here.
             */
                png_app_warning(png_ptr,
                                b"png_set_filter: UP/AVG/PAETH cannot be added after start\x00"
                                    as *const u8 as *const std::os::raw::c_char);
                filters &=
                    !(0x20 as std::os::raw::c_int | 0x40 as std::os::raw::c_int |
                          0x80 as std::os::raw::c_int)
            }
            num_filters = 0 as std::os::raw::c_int;
            if filters & 0x10 as std::os::raw::c_int != 0 { num_filters += 1 }
            if filters & 0x20 as std::os::raw::c_int != 0 { num_filters += 1 }
            if filters & 0x40 as std::os::raw::c_int != 0 { num_filters += 1 }
            if filters & 0x80 as std::os::raw::c_int != 0 { num_filters += 1 }
            /* Allocate needed row buffers if they have not already been
          * allocated.
          */
            buf_size =
                (if (*png_ptr).usr_channels as std::os::raw::c_int *
                        (*png_ptr).usr_bit_depth as std::os::raw::c_int >=
                        8 as std::os::raw::c_int {
                     ((*png_ptr).width as
                          png_size_t).wrapping_mul(((*png_ptr).usr_channels as
                                                        std::os::raw::c_int *
                                                        (*png_ptr).usr_bit_depth
                                                            as std::os::raw::c_int) as
                                                       png_size_t >>
                                                       3 as std::os::raw::c_int)
                 } else {
                     (((*png_ptr).width as
                           png_size_t).wrapping_mul(((*png_ptr).usr_channels
                                                         as std::os::raw::c_int *
                                                         (*png_ptr).usr_bit_depth
                                                             as std::os::raw::c_int)
                                                        as
                                                        png_size_t).wrapping_add(7
                                                                                     as
                                                                                     std::os::raw::c_int
                                                                                     as
                                                                                     std::os::raw::c_ulong))
                         >> 3 as std::os::raw::c_int
                 }).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong);
            if (*png_ptr).try_row.is_null() {
                (*png_ptr).try_row =
                    png_malloc(png_ptr, buf_size) as png_bytep
            }
            if num_filters > 1 as std::os::raw::c_int {
                if (*png_ptr).tst_row.is_null() {
                    (*png_ptr).tst_row =
                        png_malloc(png_ptr, buf_size) as png_bytep
                }
            }
        }
        (*png_ptr).do_filter = filters as png_byte
    } else {
        png_error(png_ptr,
                  b"Unknown custom filter method\x00" as *const u8 as
                      *const std::os::raw::c_char);
    };
}
/* DEPRECATED */
/* WRITE_WEIGHTED_FILTER */
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_level(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                   mut level: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    (*png_ptr).zlib_level = level;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_mem_level(mut png_ptr:
                                                           * mut crate::src::libpng::png::png_struct_def,
                                                       mut mem_level:
                                                           std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    (*png_ptr).zlib_mem_level = mem_level;
}
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_strategy(mut png_ptr:
                                                          * mut crate::src::libpng::png::png_struct_def,
                                                      mut strategy:
                                                          std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    /* The flag setting here prevents the libpng dynamic selection of strategy.
    */
    (*png_ptr).flags |= 0x1 as std::os::raw::c_uint;
    (*png_ptr).zlib_strategy = strategy;
}
/* If PNG_WRITE_OPTIMIZE_CMF_SUPPORTED is defined, libpng will use a
 * smaller value of window_bits if it can do so safely.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_window_bits(mut png_ptr:
                                                             * mut crate::src::libpng::png::png_struct_def,
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
#[no_mangle]
pub unsafe extern "C" fn png_set_compression_method(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                    mut method: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    /* This would produce an invalid PNG file if it worked, but it doesn't and
    * deflate will fault it, so it is harmless to just warn here.
    */
    if method != 8 as std::os::raw::c_int {
        png_warning(png_ptr,
                    b"Only compression method 8 is supported by PNG\x00" as
                        *const u8 as *const std::os::raw::c_char);
    }
    (*png_ptr).zlib_method = method;
}
/* WRITE_CUSTOMIZE_COMPRESSION */
/* The following were added to libpng-1.5.4 */
/* WRITE_CUSTOMIZE_ZTXT_COMPRESSION */
/* end of API added to libpng-1.5.4 */
#[no_mangle]
pub unsafe extern "C" fn png_set_write_status_fn<'a1>(mut png_ptr: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,
                                                 mut write_row_fn:
                                                     Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: std::os::raw::c_uint,_: std::os::raw::c_int,) -> ()>) {
    if borrow(& png_ptr).is_none() { return }
    (*(borrow_mut(&mut png_ptr)).unwrap()).write_row_fn = write_row_fn;
}
#[no_mangle]
pub unsafe extern "C" fn png_write_png(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut info_ptr: * mut crate::src::libpng::png::png_info_def,
                                       mut transforms: std::os::raw::c_int,
                                       mut params: * mut core::ffi::c_void) {
    if png_ptr.is_null() || info_ptr.is_null() { return }
    if (*info_ptr).valid & 0x8000 as std::os::raw::c_uint ==
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_app_error(png_ptr,
                      b"no rows for png_write_image to write\x00" as *const u8
                          as *const std::os::raw::c_char);
        return
    }
    /* Write the file header information. */
    png_write_info(png_ptr, info_ptr);
    /* ------ these transformations don't touch the info structure ------- */
    /* Invert monochrome pixels */
    if transforms & 0x20 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_INVERT_MONO not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Shift the pixels up to a legal bit depth and fill in
    * as appropriate to correctly scale the image.
    */
    if transforms & 0x40 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SHIFT not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
    }
    /* Pack pixels into bytes */
    if transforms & 0x4 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_PACKING not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* Swap location of alpha bytes from ARGB to RGBA */
    if transforms & 0x100 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SWAP_ALPHA not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Remove a filler (X) from XRGB/RGBX/AG/GA into to convert it into
    * RGB, note that the code expects the input color type to be G or RGB; no
    * alpha channel.
    */
    if transforms & (0x1000 as std::os::raw::c_int | 0x800 as std::os::raw::c_int) !=
           0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_STRIP_FILLER not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Flip BGR pixels to RGB */
    if transforms & 0x80 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_BGR not supported\x00" as *const u8 as
                          *const std::os::raw::c_char);
    }
    /* Swap bytes of 16-bit files to most significant byte first */
    if transforms & 0x200 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_SWAP_ENDIAN not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* Swap bits of 1-bit, 2-bit, 4-bit packed pixel formats */
    if transforms & 0x8 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_PACKSWAP not supported\x00" as *const u8
                          as *const std::os::raw::c_char);
    }
    /* Invert the alpha channel from opacity to transparency */
    if transforms & 0x400 as std::os::raw::c_int != 0 as std::os::raw::c_int {
        png_app_error(png_ptr,
                      b"PNG_TRANSFORM_INVERT_ALPHA not supported\x00" as
                          *const u8 as *const std::os::raw::c_char);
    }
    /* ----------------------- end of transformations ------------------- */
    /* Write the bits */
    png_write_image(png_ptr, (*info_ptr).row_pointers);
    /* It is REQUIRED to call this to finish writing the rest of the file */
    png_write_end(png_ptr, info_ptr);
}
use crate::laertes_rt::*;
/* WRITE */
/* SIMPLIFIED_WRITE */
