
extern "C" {
    
    #[no_mangle]
    fn free(__ptr: * mut core::ffi::c_void);
    #[no_mangle]
    fn memcpy(_: * mut core::ffi::c_void, _: * const core::ffi::c_void, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    #[no_mangle]
    fn malloc(_: std::os::raw::c_ulong) -> * mut core::ffi::c_void;
    
    
    
    
}
pub use crate::src::libpng::pngerror::png_error;
pub use crate::src::libpng::pngerror::png_warning;
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
pub type png_const_voidp = * const core::ffi::c_void;
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
/* pngmem.c - stub functions for memory allocation
 *
 * Last changed in libpng 1.6.26 [October 20, 2016]
 * Copyright (c) 1998-2002,2004,2006-2014,2016 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all memory allocation.  Users who
 * need special memory handling are expected to supply replacement
 * functions for png_malloc() and png_free(), and to use
 * png_create_read_struct_2() and png_create_write_struct_2() to
 * identify the replacement functions.
 */
/* Free a png_struct */
#[no_mangle]
pub unsafe extern "C" fn png_destroy_png_struct(mut png_ptr: * mut crate::src::libpng::png::png_struct_def) {
    if !png_ptr.is_null() {
        /* png_free might call png_error and may certainly call
       * png_get_mem_ptr, so fake a temporary png_struct to support this.
       */
        let mut dummy_struct: crate::src::libpng::png::png_struct_def = *png_ptr;
        memset(png_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
               ::std::mem::size_of::<png_struct>() as std::os::raw::c_ulong);
        png_free(&mut dummy_struct, png_ptr as png_voidp);
    };
}
/* Allocate memory.  For reasonable files, size should never exceed
 * 64K.  However, zlib may allocate more than 64K if you don't tell
 * it not to.  See zconf.h and png.h for more information.  zlib does
 * need to allocate exactly 64K, so whatever you call here must
 * have the ability to do that.
 */
#[no_mangle]
pub unsafe extern "C" fn png_calloc(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                    mut size: std::os::raw::c_ulong) -> * mut core::ffi::c_void {
    let mut ret: * mut core::ffi::c_void = 0 as *mut std::os::raw::c_void;
    ret = png_malloc(png_ptr, size);
    if !ret.is_null() { memset(ret, 0 as std::os::raw::c_int, size); }
    return ret;
}
/* png_malloc_base, an internal function added at libpng 1.6.0, does the work of
 * allocating memory, taking into account limits and PNG_USER_MEM_SUPPORTED.
 * Checking and error handling must happen outside this routine; it returns NULL
 * if the allocation cannot be done (for any reason.)
 */
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_base(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                         mut size: std::os::raw::c_ulong)
 -> * mut core::ffi::c_void {
    /* Moved to png_malloc_base from png_malloc_default in 1.6.0; the DOS
    * allocators have also been removed in 1.6.0, so any 16-bit system now has
    * to implement a user memory handler.  This checks to be sure it isn't
    * called with big numbers.
    */
    /* Some compilers complain that this is always true.  However, it
    * can be false when integer overflow happens.
    */
    if size > 0 as std::os::raw::c_int as std::os::raw::c_ulong &&
           size <= -(1 as std::os::raw::c_int) as png_size_t {
        return malloc(size)
        /* checked for truncation above */
    } else { return 0 as *mut std::os::raw::c_void };
}
/* This is really here only to work round a spurious warning in GCC 4.6 and 4.7
 * that arises because of the checks in png_realloc_array that are repeated in
 * png_malloc_array.
 */
unsafe extern "C" fn png_malloc_array_checked(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                              mut nelements: std::os::raw::c_int,
                                              mut element_size: std::os::raw::c_ulong)
 -> * mut core::ffi::c_void {
    let mut req: u64 =
        nelements as png_alloc_size_t; /* known to be > 0 */
    if req <= (-(1 as std::os::raw::c_int) as png_size_t).wrapping_div(element_size) {
        return png_malloc_base(png_ptr, req.wrapping_mul(element_size))
    }
    /* The failure case when the request is too large */
    return 0 as *mut std::os::raw::c_void;
}
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_array(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                          mut nelements: std::os::raw::c_int,
                                          mut element_size: std::os::raw::c_ulong)
 -> * mut core::ffi::c_void {
    if nelements <= 0 as std::os::raw::c_int ||
           element_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
        png_error(png_ptr,
                  b"internal error: array alloc\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    return png_malloc_array_checked(png_ptr, nelements, element_size);
}
/* PRIVATE */
#[no_mangle]
pub unsafe extern "C" fn png_realloc_array(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                           mut old_array: * const core::ffi::c_void,
                                           mut old_elements: std::os::raw::c_int,
                                           mut add_elements: std::os::raw::c_int,
                                           mut element_size: std::os::raw::c_ulong)
 -> * mut core::ffi::c_void {
    /* These are internal errors: */
    if add_elements <= 0 as std::os::raw::c_int ||
           element_size == 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
           old_elements < 0 as std::os::raw::c_int ||
           old_array.is_null() && old_elements > 0 as std::os::raw::c_int {
        png_error(png_ptr,
                  b"internal error: array realloc\x00" as *const u8 as
                      *const std::os::raw::c_char);
    }
    /* Check for overflow on the elements count (so the caller does not have to
    * check.)
    */
    if add_elements <= 2147483647 as std::os::raw::c_int - old_elements {
        let mut new_array: * mut core::ffi::c_void =
            png_malloc_array_checked(png_ptr, old_elements + add_elements,
                                     element_size);
        if !new_array.is_null() {
            /* Because png_malloc_array worked the size calculations below cannot
          * overflow.
          */
            if old_elements > 0 as std::os::raw::c_int {
                memcpy(new_array, old_array,
                       element_size.wrapping_mul(old_elements as std::os::raw::c_uint
                                                     as std::os::raw::c_ulong));
            }
            memset((new_array as
                        *mut std::os::raw::c_char).offset(element_size.wrapping_mul(old_elements
                                                                                as
                                                                                std::os::raw::c_uint
                                                                                as
                                                                                std::os::raw::c_ulong)
                                                      as isize) as
                       *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
                   element_size.wrapping_mul(add_elements as std::os::raw::c_uint as
                                                 std::os::raw::c_ulong));
            return new_array
        }
    }
    return 0 as *mut std::os::raw::c_void;
    /* error */
}
/* TEXT || sPLT || STORE_UNKNOWN_CHUNKS */
/* Various functions that have different error handling are derived from this.
 * png_malloc always exists, but if PNG_USER_MEM_SUPPORTED is defined a separate
 * function png_malloc_default is also provided.
 */
#[no_mangle]
pub unsafe extern "C" fn png_malloc(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                    mut size: std::os::raw::c_ulong) -> * mut core::ffi::c_void {
    let mut ret: * mut core::ffi::c_void =
        0 as *mut std::os::raw::c_void; /* 'm' means png_malloc */
    if png_ptr.is_null() { return 0 as *mut std::os::raw::c_void }
    ret = png_malloc_base(png_ptr, size);
    if ret.is_null() {
        png_error(png_ptr,
                  b"Out of memory\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return ret;
}
/* USER_MEM */
/* This function was added at libpng version 1.2.3.  The png_malloc_warn()
 * function will issue a png_warning and return NULL instead of issuing a
 * png_error, if it fails to allocate the requested memory.
 */
#[no_mangle]
pub unsafe extern "C" fn png_malloc_warn(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                         mut size: std::os::raw::c_ulong)
 -> * mut core::ffi::c_void {
    if !png_ptr.is_null() {
        let mut ret: * mut core::ffi::c_void = png_malloc_base(png_ptr, size);
        if !ret.is_null() { return ret }
        png_warning(png_ptr,
                    b"Out of memory\x00" as *const u8 as *const std::os::raw::c_char);
    }
    return 0 as *mut std::os::raw::c_void;
}
/* Free a pointer allocated by png_malloc().  If ptr is NULL, return
 * without taking any action.
 */
#[no_mangle]
pub unsafe extern "C" fn png_free(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                  mut ptr: * mut core::ffi::c_void) {
    if png_ptr.is_null() || ptr.is_null() { return }
    /* USER_MEM */
    free(ptr);
}
use crate::laertes_rt::*;
/* READ || WRITE */
/* USER_MEM */
