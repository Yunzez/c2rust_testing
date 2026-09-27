
extern "C" {
    
}
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
pub type png_const_charp = * const std::os::raw::c_char;
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
pub type png_structrp = * mut crate::src::libpng::png::png_struct_def;
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
pub unsafe extern "C" fn png_set_interlace_handling(mut png_ptr: * mut crate::src::libpng::png::png_struct_def)
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
pub unsafe extern "C" fn png_do_check_palette_indexes<'a1>(mut png_ptr:
                                                          * mut crate::src::libpng::png::png_struct_def,
                                                      mut row_info:
                                                          Option<&'a1 mut crate::src::libpng::png::png_row_info_struct>) {
    if ((*png_ptr).num_palette as std::os::raw::c_int) <
           (1 as std::os::raw::c_int) << (*(borrow(& row_info)).unwrap()).bit_depth as std::os::raw::c_int &&
           (*png_ptr).num_palette as std::os::raw::c_int > 0 as std::os::raw::c_int {
        /* num_palette can be 0 in MNG files */
        /* Calculations moved outside switch in an attempt to stop different
       * compiler warnings.  'padding' is in *bits* within the last byte, it is
       * an 'int' because pixel_depth becomes an 'int' in the expression below,
       * and this calculation is used because it avoids warnings that other
       * forms produced on either GCC or MSVC.
       */
        let mut padding: i32 =
            (8 as std::os::raw::c_int as
                 std::os::raw::c_uint).wrapping_sub(((*(borrow(& row_info)).unwrap()).pixel_depth as
                                                 std::os::raw::c_uint).wrapping_mul((*(borrow(& row_info)).unwrap()).width.wrapping_rem(8
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
        let mut rp: * mut u8 =
            (*png_ptr).row_buf.offset((*(borrow(& row_info)).unwrap()).rowbytes as
                                          isize).offset(-(1 as std::os::raw::c_int as
                                                              isize));
        match (*(borrow(& row_info)).unwrap()).bit_depth as std::os::raw::c_int {
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
                    let mut i: i32 =
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
                    let mut i_0: i32 =
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
use crate::laertes_rt::*;
/* READ || WRITE */
/* READ_USER_TRANSFORM || WRITE_USER_TRANSFORM */
/* CHECK_FOR_INVALID_INDEX */
