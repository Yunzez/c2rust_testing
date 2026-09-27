
extern "C" {
    
}
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
pub type png_const_charp = crate::src::libpng::png::png_const_charp;
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
pub type png_structrp = crate::src::libpng::png::png_structrp;
/* pngtrans.c - transforms the data in a row (used by both readers and writers)
 *
 * Last changed in libpng 1.6.33 [September 28, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_interlace_handling(mut png_ptr: png_structrp)
 -> std::os::raw::c_int {
    if !png_ptr.is_null() &&
           (*png_ptr).interlaced as std::os::raw::c_int != 0 as std::os::raw::c_int {
        (*png_ptr).transformations |= 0x2 as std::os::raw::c_uint;
        return 7 as std::os::raw::c_int
    }
    return 1 as std::os::raw::c_int;
}
/* PACKSWAP || WRITE_PACKSWAP */
/* READ_BGR || WRITE_BGR */
/* Added at libpng-1.5.10 */
#[no_mangle]
pub unsafe extern "C" fn png_do_check_palette_indexes(mut png_ptr:
                                                          png_structrp,
                                                      mut row_info:
                                                          png_row_infop) {
    if ((*png_ptr).num_palette as std::os::raw::c_int) <
           (1 as std::os::raw::c_int) << (*row_info).bit_depth as std::os::raw::c_int &&
           (*png_ptr).num_palette as std::os::raw::c_int > 0 as std::os::raw::c_int {
        /* num_palette can be 0 in MNG files */
        /* Calculations moved outside switch in an attempt to stop different
       * compiler warnings.  'padding' is in *bits* within the last byte, it is
       * an 'int' because pixel_depth becomes an 'int' in the expression below,
       * and this calculation is used because it avoids warnings that other
       * forms produced on either GCC or MSVC.
       */
        let mut padding: std::os::raw::c_int =
            (8 as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_sub(((*row_info).pixel_depth as
                                                 std::os::raw::c_uint).wrapping_mul((*row_info).width.wrapping_rem(8
                                                                                                               as
                                                                                                               std::os::raw::c_int
                                                                                                               as
                                                                                                               png_uint_32)).wrapping_rem(8
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_int
                                                                                                                                              as
                                                                                                                                              std::os::raw::c_uint)).wrapping_rem(8
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_int
                                                                                                                                                                              as
                                                                                                                                                                              std::os::raw::c_uint)
                as std::os::raw::c_int;
        let mut rp: png_bytep =
            (*png_ptr).row_buf.offset((*row_info).rowbytes as
                                          isize).offset(-(1 as std::os::raw::c_int as
                                                              isize));
        match (*row_info).bit_depth as std::os::raw::c_int {
            1 => {
                /* in this case, all bytes must be 0 so we don't need
             * to unpack the pixels except for the rightmost one.
             */
                while rp > (*png_ptr).row_buf {
                    if *rp as std::os::raw::c_int >> padding != 0 as std::os::raw::c_int {
                        (*png_ptr).num_palette_max = 1 as std::os::raw::c_int
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            2 => {
                while rp > (*png_ptr).row_buf {
                    let mut i: std::os::raw::c_int =
                        *rp as std::os::raw::c_int >> padding & 0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 2 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 4 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    i =
                        *rp as std::os::raw::c_int >> padding >> 6 as std::os::raw::c_int &
                            0x3 as std::os::raw::c_int;
                    if i > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            4 => {
                while rp > (*png_ptr).row_buf {
                    let mut i_0: std::os::raw::c_int =
                        *rp as std::os::raw::c_int >> padding & 0xf as std::os::raw::c_int;
                    if i_0 > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i_0
                    }
                    i_0 =
                        *rp as std::os::raw::c_int >> padding >> 4 as std::os::raw::c_int &
                            0xf as std::os::raw::c_int;
                    if i_0 > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = i_0
                    }
                    padding = 0 as std::os::raw::c_int;
                    rp = rp.offset(-1)
                }
            }
            8 => {
                while rp > (*png_ptr).row_buf {
                    if *rp as std::os::raw::c_int > (*png_ptr).num_palette_max {
                        (*png_ptr).num_palette_max = *rp as std::os::raw::c_int
                    }
                    rp = rp.offset(-1)
                }
            }
            _ => { }
        }
    };
}
/* READ || WRITE */
/* READ_USER_TRANSFORM || WRITE_USER_TRANSFORM */
/* CHECK_FOR_INVALID_INDEX */
