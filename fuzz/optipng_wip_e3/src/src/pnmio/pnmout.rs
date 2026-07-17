

use std::os::raw::c_int;
use std::os::raw::c_uint;

extern "C" {
    
    pub type _IO_codecvt;
    
    
    fn __errno_location() -> *mut std::os::raw::c_int;
    
    fn fprintf(_: *mut FILE, _: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn putc(__c: std::os::raw::c_int, __stream: *mut FILE) -> std::os::raw::c_int;
    
    fn fwrite(_: *const std::os::raw::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
              _: *mut FILE) -> std::os::raw::c_ulong;
    
    
}
pub use crate::src::pnmio::pnmutil::pnm_is_valid;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub type size_t = crate::src::gifread::gifread::size_t;
pub type __off_t = crate::src::gifread::gifread::__off_t;
pub type __off64_t = crate::src::gifread::gifread::__off64_t;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::libpng::png::_IO_FILE;
pub type _IO_lock_t = crate::src::gifread::gifread::_IO_lock_t;
pub type FILE = crate::src::gifread::gifread::FILE;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
// #[derive(Copy, Clone)]

pub type pnm_struct = crate::src::pnmio::pnmin::pnm_struct;
/* *
 * pnmout.c
 * PNM file output.
 *
 * Copyright (C) 2002-2008 Cosmin Truta.
 * This file is part of the pnmio library, distributed under the zlib license.
 * For conditions of distribution and use, see copyright notice in pnmio.h.
 **/
/* *
 * Validates a PNM header structure and writes it to a file stream.
 * Returns 1 on success, 0 on validation failure, or -1 on output failure.
 **/

pub fn pnm_fput_header(pnm_ptr: &pnm_struct, stream: &mut FILE) -> std::os::raw::c_int {
    let format = pnm_ptr.format;
    let depth = pnm_ptr.depth;
    let width = pnm_ptr.width;
    let height = pnm_ptr.height;
    let maxval = pnm_ptr.maxval;

    // Validate the info structure
    if unsafe { pnm_is_valid(pnm_ptr) } == 0 {
        return 0;
    }

    // Write the PNM file header
    let result = match format {
        1 | 4 => {
            unsafe {
                fprintf(stream,
                        b"P%c\n%u %u\n\x00" as *const u8 as *const std::os::raw::c_char,
                        format.wrapping_add('0' as i32 as std::os::raw::c_uint),
                        width, height)
            }
        }
        2 | 3 | 5 | 6 => {
            unsafe {
                fprintf(stream,
                        b"P%c\n%u %u\n%u\n\x00" as *const u8 as *const std::os::raw::c_char,
                        format.wrapping_add('0' as i32 as std::os::raw::c_uint),
                        width, height, maxval)
            }
        }
        7 => {
            unsafe {
                fprintf(stream,
                        b"P7\nDEPTH %u\nWIDTH %u\nHEIGHT %u\nMAXVAL %u\nENDHDR\n\x00"
                            as *const u8 as *const std::os::raw::c_char, depth, width,
                        height, maxval)
            }
        }
        _ => {
            unsafe {
                *__errno_location() = 22;
            }
            return 0;
        }
    };

    // Check the result
    if result > 0 {
        1
    } else {
        -1
    }
}

/* *
 * Writes an array of PNM sample values to a file stream.
 * The values are written in the format specified by pnm_ptr->format.
 * The array length is pnm_ptr->depth * pnm_ptr->width * num_rows.
 * The validity check performed on the PNM structure is only partial.
 * Returns 1 on success, 0 on validation failure, or -1 on output failure.
 **/

pub unsafe extern "C" fn pnm_fput_values(mut pnm_ptr: *const pnm_struct,
                                         mut sample_values:
                                             *const std::os::raw::c_uint,
                                         mut num_rows: std::os::raw::c_uint,
                                         mut stream: *mut FILE)
 -> std::os::raw::c_int {
    let pnm = &*pnm_ptr; // Dereference the raw pointer safely
let format = pnm.format;
let depth = pnm.depth;
let width = pnm.width;
let maxval = pnm.maxval;
let row_length = (depth as usize).wrapping_mul(width as usize);
let num_samples = (num_rows as usize).wrapping_mul(row_length);
let mut ch: i32;
let mut mask: i32;
let mut i: usize = 0;
let mut j: usize;

match format {
    1 => {
        j = 0;
        while i < num_samples {
            let value = unsafe { *sample_values.offset(i as isize) }; // Use unsafe for accessing the raw pointer
            if putc(if value != 0 { '0' as i32 } else { '1' as i32 }, stream) == -1 {
                break;
            }
            j = j.wrapping_add(1);
            if j == row_length {
                j = 0;
                if putc('\n' as i32, stream) == -1 {
                    break;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    2 | 3 => {
        j = 0;
        while i < num_samples {
            j = j.wrapping_add(1);
            if j == row_length { j = 0; }
            if fprintf(stream, if j == 0 { b"%u\n\x00".as_ptr() as *const i8 } else { b"%u \x00".as_ptr() as *const i8 }, unsafe { *sample_values.offset(i as isize) }) <= 0 {
                break;
            }
            i = i.wrapping_add(1);
        }
    }
    4 => {
        j = 0;
        while i < num_samples {
            ch = 0;
            mask = 0x80;
            while mask != 0 {
                let fresh0 = i;
                i = i.wrapping_add(1);
                if unsafe { *sample_values.offset(fresh0 as isize) } == 0 {
                    ch |= mask;
                }
                j = j.wrapping_add(1);
                if j == row_length {
                    j = 0;
                    break;
                } else {
                    mask >>= 1;
                }
            }
            if putc(ch, stream) == -1 { break; }
        }
    }
    5 | 6 | 7 => {
        if maxval <= 0xff {
            i = 0;
            while i < num_samples {
                if putc((unsafe { *sample_values.offset(i as isize) } & 0xff) as i32, stream) == -1 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        } else if maxval <= 0xffff {
            i = 0;
            while i < num_samples {
                if putc((unsafe { *sample_values.offset(i as isize) } >> 8 & 0xff) as i32, stream) == -1 ||
                   putc((unsafe { *sample_values.offset(i as isize) } & 0xff) as i32, stream) == -1 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        } else if maxval <= 0xffffffff {
            i = 0;
            while i < num_samples {
                if maxval > 0xffffff {
                    if putc((unsafe { *sample_values.offset(i as isize) } >> 24 & 0xff) as i32, stream) == -1 {
                        break;
                    }
                }
                if putc((unsafe { *sample_values.offset(i as isize) } >> 16 & 0xff) as i32, stream) == -1 ||
                   putc((unsafe { *sample_values.offset(i as isize) } >> 8 & 0xff) as i32, stream) == -1 ||
                   putc((unsafe { *sample_values.offset(i as isize) } & 0xff) as i32, stream) == -1 {
                    break;
                }
                i = i.wrapping_add(1);
            }
        } else {
            *__errno_location() = 22; // Set errno to EINVAL
            return 0;
        }
    }
    _ => {
        *__errno_location() = 22; // Set errno to EINVAL
        return 0;
    }
}

// Check the result
return if i == num_samples { 1 } else { -1 };

}
/* *
 * Writes an array of sample bytes to a raw PNM file stream.
 * Multi-byte samples are stored in network order, as in the PNM stream.
 * The byte count is sample_size * pnm_ptr->depth * pnm_ptr->width * num_rows.
 * The validity check performed on the PNM structure is only partial.
 * Returns 1 on success, 0 on validation failure, or -1 on output failure.
 **/

pub unsafe extern "C" fn pnm_fput_bytes(mut pnm_ptr: *const pnm_struct,
                                        mut sample_bytes:
                                            *const std::os::raw::c_uchar,
                                        mut sample_size: size_t,
                                        mut num_rows: std::os::raw::c_uint,
                                        mut stream: *mut FILE)
 -> std::os::raw::c_int {
    let mut format: std::os::raw::c_uint = (*pnm_ptr).format;
    let mut depth: std::os::raw::c_uint = (*pnm_ptr).depth;
    let mut width: std::os::raw::c_uint = (*pnm_ptr).width;
    let mut maxval: std::os::raw::c_uint = (*pnm_ptr).maxval;
    let mut row_length: size_t =
        (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: size_t =
        (num_rows as std::os::raw::c_ulong).wrapping_mul(row_length);
    let mut raw_sample_size: size_t = 0;
    let mut ch: std::os::raw::c_int = 0;
    let mut mask: std::os::raw::c_int = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    /* validate the given sample size */
    if maxval <= 0xff as std::os::raw::c_uint {
        raw_sample_size = 1 as std::os::raw::c_int as size_t
    } else if maxval <= 0xffff as std::os::raw::c_uint {
        raw_sample_size = 2 as std::os::raw::c_int as size_t
    } else if maxval <= 0xffffff as std::os::raw::c_uint {
        raw_sample_size = 3 as std::os::raw::c_int as size_t
    } else if maxval <= 0xffffffff as std::os::raw::c_uint {
        raw_sample_size = 4 as std::os::raw::c_int as size_t
    } else {
        /* maxval > PNM_UINT_MAX */
        raw_sample_size = (sample_size == 0) as std::os::raw::c_int as size_t
    }
    if raw_sample_size != sample_size {
        *__errno_location() = 22 as std::os::raw::c_int;
        return 0 as std::os::raw::c_int
    }
    /* write the raw sample bytes */
    match format {
        4 => {
            j = 0 as std::os::raw::c_int as size_t;
            i = j;
            while i < num_samples {
                ch = 0 as std::os::raw::c_int;
                mask = 0x80 as std::os::raw::c_int;
                while mask != 0 as std::os::raw::c_int {
                    let fresh1 = i;
                    i = i.wrapping_add(1);
                    if *sample_bytes.offset(fresh1 as isize) as std::os::raw::c_int ==
                           0 as std::os::raw::c_int {
                        ch |= mask
                    }
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as std::os::raw::c_int as size_t;
                        break ;
                    } else { mask >>= 1 as std::os::raw::c_int }
                }
                if putc(ch, stream) == -(1 as std::os::raw::c_int) { break ; }
            }
        }
        5 | 6 | 7 => {
            i =
                fwrite(sample_bytes as *const std::os::raw::c_void, sample_size,
                       num_samples, stream)
        }
        _ => {
            *__errno_location() = 22 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
    }
    /* check the result */
    return if i == num_samples {
               1 as std::os::raw::c_int
           } else { -(1 as std::os::raw::c_int) };
}
