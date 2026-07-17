
extern "C" {
    
    
    
    
    #[no_mangle]
    fn fflush(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fwrite(_: * const core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
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
pub type png_FILE_p = * mut crate::src::gifread::gifread::_IO_FILE;
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
pub type png_ptruint = std::os::raw::c_ulong;
/* pngwio.c - functions for data output
 *
 * Last changed in libpng 1.6.24 [August 4, 2016]
 * Copyright (c) 1998-2002,2004,2006-2014,2016 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all output.  Users who need
 * special handling are expected to write functions that have the same
 * arguments as these and perform similar functions, but that possibly
 * use different output methods.  Note that you shouldn't change these
 * functions, but rather write replacement functions and then change
 * them at run time with png_set_write_fn(...).
 */
/* Write the data to whatever output you are using.  The default routine
 * writes to a file pointer.  Note that this routine sometimes gets called
 * with very small lengths, so you should implement some kind of simple
 * buffering if you are using unbuffered writes.  This should never be asked
 * to write more than 64K on a 16-bit machine.
 */
#[no_mangle]
pub unsafe extern "C" fn png_write_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                        mut data: * const std::os::raw::c_uchar,
                                        mut length: std::os::raw::c_ulong) {
    /* NOTE: write_data_fn must not change the buffer! */
    if (*png_ptr).write_data_fn.is_some() {
        Some((*png_ptr).write_data_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr,
                                                                                                               data
                                                                                                                   as
                                                                                                                   *const std::os::raw::c_void
                                                                                                                   as
                                                                                                                   png_ptruint
                                                                                                                   as
                                                                                                                   png_bytep,
                                                                                                               length);
    } else {
        png_error(png_ptr,
                  b"Call to NULL write function\x00" as *const u8 as
                      *const std::os::raw::c_char);
    };
}
/* This is the function that does the actual writing of data.  If you are
 * not writing to a standard C stream, you should create a replacement
 * write_data function and use it at run time with png_set_write_fn(), rather
 * than changing the library.
 */
#[no_mangle]
pub unsafe extern "C" fn png_default_write_data(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                                mut data: * mut std::os::raw::c_uchar,
                                                mut length: std::os::raw::c_ulong) {
    let mut check: u64 = 0;
    if png_ptr.is_null() { return }
    check =
        fwrite(data as *const std::os::raw::c_void, 1 as std::os::raw::c_int as std::os::raw::c_ulong,
               length, (*png_ptr).io_ptr as png_FILE_p);
    if check != length {
        png_error(png_ptr as *const png_struct,
                  b"Write Error\x00" as *const u8 as *const std::os::raw::c_char);
    };
}
/* This function is called to output any data pending writing (normally
 * to disk).  After png_flush is called, there should be no data pending
 * writing in any buffers.
 */
#[no_mangle]
pub unsafe extern "C" fn png_flush(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    if (*png_ptr).output_flush_fn.is_some() {
        Some((*png_ptr).output_flush_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_default_flush(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    let mut io_ptr: * mut crate::src::gifread::gifread::_IO_FILE = 0 as *mut FILE;
    if png_ptr.is_null() { return }
    io_ptr = (*png_ptr).io_ptr as png_FILE_p;
    fflush(io_ptr);
}
/* This function allows the application to supply new output functions for
 * libpng if standard C streams aren't being used.
 *
 * This function takes as its arguments:
 * png_ptr       - pointer to a png output data structure
 * io_ptr        - pointer to user supplied structure containing info about
 *                 the output functions.  May be NULL.
 * write_data_fn - pointer to a new output function that takes as its
 *                 arguments a pointer to a png_struct, a pointer to
 *                 data to be written, and a 32-bit unsigned int that is
 *                 the number of bytes to be written.  The new write
 *                 function should call png_error(png_ptr, "Error msg")
 *                 to exit and output any fatal error messages.  May be
 *                 NULL, in which case libpng's default function will
 *                 be used.
 * flush_data_fn - pointer to a new flush function that takes as its
 *                 arguments a pointer to a png_struct.  After a call to
 *                 the flush function, there should be no data in any buffers
 *                 or pending transmission.  If the output method doesn't do
 *                 any buffering of output, a function prototype must still be
 *                 supplied although it doesn't have to do anything.  If
 *                 PNG_WRITE_FLUSH_SUPPORTED is not defined at libpng compile
 *                 time, output_flush_fn will be ignored, although it must be
 *                 supplied for compatibility.  May be NULL, in which case
 *                 libpng's default function will be used, if
 *                 PNG_WRITE_FLUSH_SUPPORTED is defined.  This is not
 *                 a good idea if io_ptr does not point to a standard
 *                 *FILE structure.
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_write_fn(mut png_ptr: * mut crate::src::libpng::png::png_struct_def,
                                          mut io_ptr: * mut core::ffi::c_void,
                                          mut write_data_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * mut std::os::raw::c_uchar,_: std::os::raw::c_ulong,) -> ()>,
                                          mut output_flush_fn:
                                              Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,) -> ()>) {
    if png_ptr.is_null() { return }
    (*png_ptr).io_ptr = io_ptr;
    if write_data_fn.is_some() {
        (*png_ptr).write_data_fn = write_data_fn
    } else {
        (*png_ptr).write_data_fn =
            Some(png_default_write_data)
    }
    if output_flush_fn.is_some() {
        (*png_ptr).output_flush_fn = output_flush_fn
    } else {
        (*png_ptr).output_flush_fn =
            Some(png_default_flush)
    }
    /* WRITE_FLUSH */
    /* It is an error to read while writing a png file */
    if (*png_ptr).read_data_fn.is_some() {
        (*png_ptr).read_data_fn = None;
        png_warning(png_ptr,
                    b"Can\'t set both read_data_fn and write_data_fn in the same structure\x00"
                        as *const u8 as *const std::os::raw::c_char);
    };
}
use crate::laertes_rt::*;
/* WRITE */
