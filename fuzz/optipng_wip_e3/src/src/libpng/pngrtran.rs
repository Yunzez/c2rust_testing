



extern "C" {
    
    
    
    
    
    
    
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::libpng::pngtrans::png_do_check_palette_indexes;
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
// #[derive(Copy, Clone)]

pub type png_info_def = crate::src::libpng::pngget::png_info_def;
pub type png_unknown_chunkp = crate::src::libpng::png::png_unknown_chunkp;
pub type png_info = crate::src::libpng::png::png_info;
pub type png_structrp = crate::src::libpng::png::png_structrp;
pub type png_const_structrp = crate::src::libpng::png::png_const_structrp;
pub type png_inforp = crate::src::libpng::png::png_inforp;
/* pngrtran.c - transforms the data in a row for PNG readers
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
 * This file contains functions optionally called by an application
 * in order to tell libpng how to handle data when reading a PNG.
 * Transformations that are used in both reading and writing are
 * in pngtrans.c.
 */
/* Set the action on getting a CRC error for an ancillary or critical chunk. */

pub unsafe extern "C" fn png_set_crc_action(mut png_ptr: png_structrp,
                                            mut crit_action: std::os::raw::c_int,
                                            mut ancil_action: std::os::raw::c_int) {
    if png_ptr.is_null() { return }
    let mut current_block_8: u64;
    /* Tell libpng how we react to CRC errors in critical chunks */
    match crit_action {
        5 => { current_block_8 = 1841672684692190573; }
        3 => {
            /* Warn/use data */
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x400 as std::os::raw::c_uint;
            current_block_8 = 1841672684692190573;
        }
        4 => {
            /* Quiet/use data */
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint;
            current_block_8 = 1841672684692190573;
        }
        2 => {
            /* Not a valid action for critical data */
            png_warning(png_ptr,
                        b"Can\'t discard critical data on CRC error\x00" as
                            *const u8 as *const std::os::raw::c_char);
            current_block_8 = 71504420348896963;
        }
        1 => {
            /* Error/quit */
            current_block_8 = 71504420348896963;
        }
        0 | _ => { current_block_8 = 71504420348896963; }
    }
    match current_block_8 {
        71504420348896963 =>
        /* FALLTHROUGH */
        {
            (*png_ptr).flags &=
                !(0x400 as std::os::raw::c_uint | 0x800 as std::os::raw::c_uint)
        }
        _ => { }
    }
    /* Tell libpng how we react to CRC errors in ancillary chunks */
    match ancil_action {
        5 => { }
        3 => {
            /* Warn/use data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x100 as std::os::raw::c_uint
        }
        4 => {
            /* Quiet/use data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint
        }
        1 => {
            /* Error/quit */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint);
            (*png_ptr).flags |= 0x200 as std::os::raw::c_uint
        }
        2 | 0 | _ => {
            /* Warn/discard data */
            (*png_ptr).flags &=
                !(0x100 as std::os::raw::c_uint | 0x200 as std::os::raw::c_uint)
        }
    };
}
/* READ_BACKGROUND */
/* Scale 16-bit depth files to 8-bit depth.  If both of these are set then the
 * one that pngrtran does first (scale) happens.  This is necessary to allow the
 * TRANSFORM and API behavior to be somewhat consistent, and it's simpler.
 */
/* READ_ALPHA_MODE || READ_GAMMA */
/* READ_QUANTIZE */
/* READ_GAMMA */
/* READ_EXPAND */
/* RGB_TO_GRAY */
/* Initialize everything needed for the read.  This includes modifying
 * the palette.
 */
/* For the moment 'png_init_palette_transformations' and
 * 'png_init_rgb_transformations' only do some flag canceling optimizations.
 * The intent is that these two routines should have palette or rgb operations
 * extracted from 'png_init_read_transformations'.
 */
fn png_init_palette_transformations(png_ptr: &mut crate::src::libpng::png::png_struct) {
    let mut input_has_alpha = false;
    let mut input_has_transparency = false;

    if png_ptr.num_trans > 0 {
        for i in 0..png_ptr.num_trans {
            let alpha = unsafe { *png_ptr.trans_alpha.offset(i as isize) };
            if alpha != 255 {
                if alpha == 0 {
                    input_has_transparency = true;
                } else {
                    input_has_transparency = true;
                    input_has_alpha = true;
                    break;
                }
            }
        }
    }

    if !input_has_alpha {
        png_ptr.transformations &= !(0x800000);
        png_ptr.flags &= !(0x2000);
        if !input_has_transparency {
            png_ptr.transformations &=
                !(0x80 | 0x100);
        }
    }
}

fn png_init_rgb_transformations(png_ptr: &mut png_structrp) {
    let png = unsafe { &mut **png_ptr }; // Dereference the double pointer
    let input_has_alpha = (png.color_type & 4) != 0;
    let input_has_transparency = png.num_trans > 0;

    if !input_has_alpha {
        if !input_has_transparency {
            png.transformations &= !(0x80 | 0x100);
        }
    }
}


pub unsafe extern "C" fn png_init_read_transformations(mut png_ptr:
                                                           png_structrp) {
    /* This internal function is called from png_read_start_row in pngrutil.c
    * and it is called before the 'rowbytes' calculation is done, so the code
    * in here can change or update the transformations flags.
    *
    * First do updates that do not depend on the details of the PNG image data
    * being processed.
    */
    /* Certain transformations have the effect of preventing other
    * transformations that happen afterward in png_do_read_transformations;
    * resolve the interdependencies here.  From the code of
    * png_do_read_transformations the order is:
    *
    *  1) PNG_EXPAND (including PNG_EXPAND_tRNS)
    *  2) PNG_STRIP_ALPHA (if no compose)
    *  3) PNG_RGB_TO_GRAY
    *  4) PNG_GRAY_TO_RGB iff !PNG_BACKGROUND_IS_GRAY
    *  5) PNG_COMPOSE
    *  6) PNG_GAMMA
    *  7) PNG_STRIP_ALPHA (if compose)
    *  8) PNG_ENCODE_ALPHA
    *  9) PNG_SCALE_16_TO_8
    * 10) PNG_16_TO_8
    * 11) PNG_QUANTIZE (converts to palette)
    * 12) PNG_EXPAND_16
    * 13) PNG_GRAY_TO_RGB iff PNG_BACKGROUND_IS_GRAY
    * 14) PNG_INVERT_MONO
    * 15) PNG_INVERT_ALPHA
    * 16) PNG_SHIFT
    * 17) PNG_PACK
    * 18) PNG_BGR
    * 19) PNG_PACKSWAP
    * 20) PNG_FILLER (includes PNG_ADD_ALPHA)
    * 21) PNG_SWAP_ALPHA
    * 22) PNG_SWAP_BYTES
    * 23) PNG_USER_TRANSFORM [must be last]
    */
    /* STRIP_ALPHA supported, no COMPOSE */
    /* READ_GRAY_TO_RGB */
    /* For indexed PNG data (PNG_COLOR_TYPE_PALETTE) many of the transformations
    * can be performed directly on the palette, and some (such as rgb to gray)
    * can be optimized inside the palette.  This is particularly true of the
    * composite (background and alpha) stuff, which can be pretty much all done
    * in the palette even if the result is expanded to RGB or gray afterward.
    *
    * NOTE: this is Not Yet Implemented, the code behaves as in 1.5.1 and
    * earlier and the palette stuff is actually handled on the first row.  This
    * leads to the reported bug that the palette returned by png_get_PLTE is not
    * updated.
    */
    if (*png_ptr).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int {
        png_init_palette_transformations(&mut *png_ptr);
    } else { png_init_rgb_transformations(&mut png_ptr); };
    /* READ_BACKGROUND && READ_EXPAND_16 */
    /* NOTE: below 'PNG_READ_ALPHA_MODE_SUPPORTED' is presumed to also enable the
    * background support (see the comments in scripts/pnglibconf.dfa), this
    * allows pre-multiplication of the alpha channel to be implemented as
    * compositing on black.  This is probably sub-optimal and has been done in
    * 1.5.4 betas simply to enable external critique and testing (i.e. to
    * implement the new API quickly, without lots of internal changes.)
    */
    /* READ_GAMMA */
    /* READ_BACKGROUND */
    /* READ_SHIFT */
}
/* Modify the info structure to reflect the transformations.  The
 * info should be updated so a PNG file could be written with it,
 * assuming the transformations result in valid PNG data.
 */

pub fn png_read_transform_info(png_ptr: *mut *mut png_struct_def, info_ptr: *mut *mut png_info_def) {
    let info = unsafe { &mut **info_ptr };
    let png = unsafe { &mut **png_ptr };

    let bit_depth = info.bit_depth;
    let color_type = info.color_type;

    if color_type == 2 || color_type == 1 {
        info.channels = 1;
    } else if color_type & 2 != 0 {
        info.channels = 3;
    } else {
        info.channels = 1;
    }

    if color_type & 4 != 0 {
        info.channels += 1;
    }

    info.pixel_depth = (info.channels * bit_depth) as png_byte;

    info.rowbytes = if info.pixel_depth >= 8 {
        (info.width as png_size_t).wrapping_mul((info.pixel_depth >> 3) as png_size_t)
    } else {
        (((info.width as png_size_t).wrapping_mul(info.pixel_depth as png_size_t).wrapping_add(7)) >> 3) as png_size_t
    };

    png.info_rowbytes = info.rowbytes;
}

/* READ_BACKGROUND || READ_ALPHA_MODE */
/* READ_QUANTIZE */
/* Transform the row.  The order of transformations is significant,
 * and is very touchy.  If you add a transformation, take care to
 * decide how it fits in with the other transformations here.
 */

pub unsafe extern "C" fn png_do_read_transformations(mut png_ptr:
                                                         png_structrp,
                                                     mut row_info:
                                                         png_row_infop) {
    if (*png_ptr).row_buf.is_null() {
        /* Prior to 1.5.4 this output row/pass where the NULL pointer is, but this
       * error is incredibly rare and incredibly easy to debug without this
       * information.
       */
        png_error(png_ptr,
                  b"NULL row buffer\x00" as *const u8 as *const std::os::raw::c_char);
    }
    /* The following is debugging; prior to 1.5.4 the code was never compiled in;
    * in 1.5.4 PNG_FLAG_DETECT_UNINITIALIZED was added and the macro
    * PNG_WARN_UNINITIALIZED_ROW removed.  In 1.6 the new flag is set only for
    * all transformations, however in practice the ROW_INIT always gets done on
    * demand, if necessary.
    */
    if (*png_ptr).flags & 0x4000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint &&
           (*png_ptr).flags & 0x40 as std::os::raw::c_uint ==
               0 as std::os::raw::c_int as std::os::raw::c_uint {
        /* Application has failed to call either png_read_start_image() or
       * png_read_update_info() after setting transforms that expand pixels.
       * This check added to libpng-1.2.19 (but not enabled until 1.5.4).
       */
        png_error(png_ptr,
                  b"Uninitialized row\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* From Andreas Dilger e-mail to png-implement, 26 March 1998:
 *
 *   In most cases, the "simple transparency" should be done prior to doing
 *   gray-to-RGB, or you will have to test 3x as many bytes to check if a
 *   pixel is transparent.  You would also need to make sure that the
 *   transparency information is upgraded to RGB.
 *
 *   To summarize, the current flow is:
 *   - Gray + simple transparency -> compare 1 or 2 gray bytes and composite
 *                                   with background "in place" if transparent,
 *                                   convert to RGB if necessary
 *   - Gray + alpha -> composite with gray background and remove alpha bytes,
 *                                   convert to RGB if necessary
 *
 *   To support RGB backgrounds for gray images we need:
 *   - Gray + simple transparency -> convert to RGB + simple transparency,
 *                                   compare 3 or 6 bytes and composite with
 *                                   background "in place" if transparent
 *                                   (3x compare/pixel compared to doing
 *                                   composite with gray bkgrnd)
 *   - Gray + alpha -> convert to RGB + alpha, composite with background and
 *                                   remove alpha bytes (3x float
 *                                   operations/pixel compared with composite
 *                                   on gray background)
 *
 *  Greg's change will do this.  The reason it wasn't done before is for
 *  performance, as this increases the per-pixel operations.  If we would check
 *  in advance if the background was gray or RGB, and position the gray-to-RGB
 *  transform appropriately, then it would save a lot of work/time.
 */
    /* READ_QUANTIZE */
    /* Added at libpng-1.5.10 */
    if (*row_info).color_type as std::os::raw::c_int ==
           2 as std::os::raw::c_int | 1 as std::os::raw::c_int &&
           (*png_ptr).num_palette_max >= 0 as std::os::raw::c_int {
        png_do_check_palette_indexes(png_ptr, row_info);
    };
}
/* READ */
/* READ_TRANSFORMS */
