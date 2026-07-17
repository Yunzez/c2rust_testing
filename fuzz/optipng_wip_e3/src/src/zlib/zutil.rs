


use std::ffi::CStr;

extern "C" {
    
    fn calloc(_: std::os::raw::c_ulong, _: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(__ptr: *mut std::os::raw::c_void);
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
}
pub type uInt = crate::src::libpng::png::uInt;
pub type uLong = crate::src::libpng::png::uLong;
pub type voidpf = crate::src::libpng::png::voidpf;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type off_t = crate::src::zlib::adler32::off_t;
/* zutil.c -- target dependent utility functions for the compression library
 * Copyright (C) 1995-2017 Jean-loup Gailly
 * For conditions of distribution and use, see copyright notice in zlib.h
 */
/* @(#) $Id$ */

pub static mut z_errmsg: [*mut std::os::raw::c_char; 10] =
    [b"need dictionary\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"stream end\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char,
     b"file error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"stream error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"data error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"insufficient memory\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"buffer error\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"incompatible version\x00" as *const u8 as *const std::os::raw::c_char as
         *mut std::os::raw::c_char,
     b"\x00" as *const u8 as *const std::os::raw::c_char as *mut std::os::raw::c_char];

pub fn zlibVersion() -> *const std::os::raw::c_char {
    b"1.2.11-optipng\x00" as *const u8 as *const std::os::raw::c_char
}


pub fn zlibCompileFlags() -> u64 {
    let mut flags: u64 = 0;

    match std::mem::size_of::<u32>() {
        2 => {}
        4 => {
            flags += 1;
        }
        8 => {
            flags += 2;
        }
        _ => {
            flags += 3;
        }
    }

    match std::mem::size_of::<u64>() {
        2 => {}
        4 => {
            flags += 1 << 2;
        }
        8 => {
            flags += 2 << 2;
        }
        _ => {
            flags += 3 << 2;
        }
    }

    match std::mem::size_of::<*mut std::ffi::c_void>() {
        2 => {}
        4 => {
            flags += 1 << 4;
        }
        8 => {
            flags += 2 << 4;
        }
        _ => {
            flags += 3 << 4;
        }
    }

    match std::mem::size_of::<i64>() {
        2 => {}
        4 => {
            flags += 1 << 6;
        }
        8 => {
            flags += 2 << 6;
        }
        _ => {
            flags += 3 << 6;
        }
    }

    flags += 1 << 16;
    flags += 1 << 17;

    flags
}

/* exported to allow conversion of error code to string for compress() and
 * uncompress()
 */

pub fn zError(err: std::os::raw::c_int) -> &'static str {
    let index = (2 - err) as usize;
    if index < 10 {
        unsafe { std::ffi::CStr::from_ptr(z_errmsg[index]).to_str().unwrap_or("unknown error") }
    } else {
        "unknown error"
    }
}

/* SYS16BIT */
/* Any system without a special alloc function */

pub unsafe extern "C" fn zcalloc(mut opaque: voidpf, mut items: std::os::raw::c_uint,
                                 mut size: std::os::raw::c_uint) -> voidpf {
    return if ::std::mem::size_of::<uInt>() as std::os::raw::c_ulong >
                  2 as std::os::raw::c_int as std::os::raw::c_ulong {
               malloc(items.wrapping_mul(size) as std::os::raw::c_ulong)
           } else { calloc(items as std::os::raw::c_ulong, size as std::os::raw::c_ulong) };
}

pub unsafe extern "C" fn zcfree(mut opaque: voidpf, mut ptr: voidpf) {
    free(ptr);
}
/* !Z_SOLO */
/* MY_ZCALLOC */
