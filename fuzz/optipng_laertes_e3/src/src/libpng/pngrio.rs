
extern "C" {
    
    
    
    
    
    
    
    
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub use crate::src::zlib::deflate::internal_state;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
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
pub type png_const_structrp = * const crate::src::libpng::png::png_struct_def;
/* pngrio.c - functions for data input
 *
 * Last changed in libpng 1.6.24 [August 4, 2016]
 * Copyright (c) 1998-2002,2004,2006-2016 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all input.  Users who need
 * special handling are expected to write a function that has the same
 * arguments as this and performs a similar function, but that possibly
 * has a different input method.  Note that you shouldn't change this
 * function, but rather write a replacement function and then make
 * libpng use it at run time with png_set_read_fn(...).
 */
/* Read the data from whatever input you are using.  The default routine
 * reads from a file pointer.  Note that this routine sometimes gets called
 * with very small lengths, so you should implement some kind of simple
 * buffering if you are using unbuffered reads.  This should never be asked
 * to read more than 64K on a 16-bit machine.
 */
#[no_mangle]
pub unsafe extern "C" fn png_read_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                       mut data: * mut std::os::raw::c_uchar,
                                       mut length: std::os::raw::c_ulong) {
    if (*png_ptr).read_data_fn.is_some() {
        Some((*png_ptr).read_data_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                              data,
                                                                                                              length);
    } else {
        png_error(png_ptr,
                  b"Call to NULL read function\x00" as *const u8 as
                      *const std::os::raw::c_char);
    };
}
/* This is the function that does the actual reading of data.  If you are
 * not reading from a standard C stream, you should create a replacement
 * read_data function and use it at run time with png_set_read_fn(), rather
 * than changing the library.
 */
#[no_mangle]
pub unsafe extern "C" fn png_default_read_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                               mut data: * mut std::os::raw::c_uchar,
                                               mut length: std::os::raw::c_ulong) {
    let mut check: u64 = 0;
    if png_ptr.is_null() { return }
    /* fread() returns 0 on error, so it is OK to store this in a png_size_t
    * instead of an int, which is what fread() actually returns.
    */
    check =
        fread(data as *mut std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
              length, (*png_ptr).io_ptr as *mut FILE);
    if check != length {
        png_error(png_ptr as *const png_struct,
                  b"Read Error\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* This function allows the application to supply a new input function
 * for libpng if standard C streams aren't being used.
 *
 * This function takes as its arguments:
 *
 * png_ptr      - pointer to a png input data structure
 *
 * io_ptr       - pointer to user supplied structure containing info about
 *                the input functions.  May be NULL.
 *
 * read_data_fn - pointer to a new input function that takes as its
 *                arguments a pointer to a png_struct, a pointer to
 *                a location where input data can be stored, and a 32-bit
 *                unsigned int that is the number of bytes to be read.
 *                To exit and output any fatal error messages the new write
 *                function should call png_error(png_ptr, "Error msg").
 *                May be NULL, in which case libpng's default function will
 *                be used.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_read_fn(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                         mut io_ptr: * mut core::ffi::c_void,
                                         mut read_data_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut std::os::raw::c_uchar,_: std::os::raw::c_ulong,) -> ()>) {
    if png_ptr.is_null() { return }
    (*png_ptr).io_ptr = io_ptr;
    if read_data_fn.is_some() {
        (*png_ptr).read_data_fn = read_data_fn
    } else {
        (*png_ptr).read_data_fn =
            Some(png_default_read_data)
    }
    /* It is an error to write to a read device */
    if (*png_ptr).write_data_fn.is_some() {
        (*png_ptr).write_data_fn = None;
        png_warning(png_ptr,
                    b"Can\'t set both read_data_fn and write_data_fn in the same structure\x00"
                        as *const u8 as *const std::os::raw::c_char);
    }
    (*png_ptr).output_flush_fn = None;
}
use crate::laertes_rt::*;
/* READ */
