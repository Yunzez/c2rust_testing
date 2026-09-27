
extern "C" {
    
    
    
    #[no_mangle]
    fn __errno_location() -> * mut std::os::raw::c_int;
    #[no_mangle]
    fn getc(__stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn ungetc(__c: std::os::raw::c_int, __stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int;
    #[no_mangle]
    fn fread(_: * mut core::ffi::c_void, _: std::os::raw::c_ulong, _: std::os::raw::c_ulong,
             _: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_ulong;
    #[no_mangle]
    fn memset(_: * mut core::ffi::c_void, _: std::os::raw::c_int, _: std::os::raw::c_ulong)
     -> * mut core::ffi::c_void;
    
    
}
pub use crate::src::pnmio::pnmutil::pnm_is_valid;
pub use crate::src::pngxtern::pngxread::_IO_marker;
pub use crate::src::pngxtern::pngxrpnm::_IO_wide_data;
pub use crate::src::pnmio::pnmout::_IO_codecvt;
pub type size_t = std::os::raw::c_ulong;
pub type __off_t = std::os::raw::c_long;
pub type __off64_t = std::os::raw::c_long;
// #[derive(Copy, Clone)]

pub type _IO_FILE = crate::src::gifread::gifread::_IO_FILE;
pub type _IO_lock_t = ();
pub type FILE = crate::src::gifread::gifread::_IO_FILE;
pub type C2RustUnnamed = std::os::raw::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
// #[derive(Copy, Clone)]

pub type pnm_struct = crate::src::pngxtern::pngxrpnm::pnm_struct;
/* *
 * Reads a character from a file stream.
 * Comment sequences starting with '#' are skipped until the end of line.
 * End of line sequences (LF, CR, CR+LF) are translated to '\n'.
 * Returns the character read, or EOF on input failure.
 **/
unsafe extern "C" fn pnm_fget_char(mut stream: * mut crate::src::gifread::gifread::_IO_FILE) -> std::os::raw::c_int {
    let mut ch: i32 = getc(stream);
    /* skip the comments */
    if ch == '#' as i32 {
        loop  {
            ch = getc(stream);
            if !(ch != -(1 as std::os::raw::c_int) && ch != '\n' as i32 &&
                     ch != '\r' as i32) {
                break ;
            }
        }
    }
    /* translate the line endings */
    if ch == '\r' as i32 {
        ch = getc(stream);
        if ch != '\n' as i32 { ungetc(ch, stream); ch = '\n' as i32 }
    }
    return ch;
}
/* *
 * Reads (scans) an unsigned integer from a file stream.
 * Returns 1 on success, 0 on matching failure, or EOF on input failure.
 **/
unsafe extern "C" fn pnm_fscan_uint<'a1>(mut stream: * mut crate::src::gifread::gifread::_IO_FILE,
                                    mut value: Option<&'a1 mut std::os::raw::c_uint>)
 -> std::os::raw::c_int {
    let mut ch: i32 = 0;
    let mut tmp: u32 = 0;
    loop 
         /* skip the leading whitespaces */
         {
        ch = pnm_fget_char(stream);
        if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 ||
                 ch == '\r' as i32) {
            break ;
        }
    }
    if ch == -(1 as std::os::raw::c_int) {
        /* input failure */
        return -(1 as std::os::raw::c_int)
    }
    if !(ch >= '0' as i32 && ch <= '9' as i32) {
        /* matching failure */
        ungetc(ch, stream);
        return 0 as std::os::raw::c_int
    }
    /* read the value */
    *(borrow_mut(&mut value)).unwrap() = 0 as std::os::raw::c_int as std::os::raw::c_uint;
    loop  {
        tmp =
            (*(borrow(& value)).unwrap()).wrapping_mul(10 as std::os::raw::c_int as
                                      std::os::raw::c_uint).wrapping_add((ch -
                                                                      '0' as
                                                                          i32)
                                                                     as
                                                                     std::os::raw::c_uint);
        if tmp >= *(borrow(& value)).unwrap() {
            *(borrow_mut(&mut value)).unwrap() = tmp
        } else {
            /* overflow */
            *(borrow_mut(&mut value)).unwrap() =
                (2147483647 as std::os::raw::c_int as
                     std::os::raw::c_uint).wrapping_mul(2 as
                                                    std::os::raw::c_uint).wrapping_add(1
                                                                                   as
                                                                                   std::os::raw::c_uint);
            *__errno_location() = 34 as std::os::raw::c_int
        }
        ch = getc(stream);
        if !(ch >= '0' as i32 && ch <= '9' as i32) { break ; }
    }
    /* put back the trailing non-whitespace, if any */
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 ||
             ch == '\r' as i32) {
        ungetc(ch, stream);
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * Reads a PNM header structure from a file stream and validates it.
 * Returns 1 on success, 0 on validation failure,
 * or -1 on input or matching failure.
 * Reading PAM ("P7") headers is not currently implemented.
 **/
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_header(mut pnm_ptr: * mut crate::src::pngxtern::pngxrpnm::pnm_struct,
                                         mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_int {
    let mut format: u32 = 0;
    let mut ch: i32 = 0;
    /* clear the PNM info structure */
    memset(pnm_ptr as *mut std::os::raw::c_void, 0 as std::os::raw::c_int,
           ::std::mem::size_of::<pnm_struct>() as std::os::raw::c_ulong);
    /* read the PNM file signature */
    ch = getc(stream);
    if ch == -(1 as std::os::raw::c_int) {
        /* input failure */
        return -(1 as std::os::raw::c_int)
    }
    /* any subsequent failure is a matching failure */
    if ch != 'P' as i32 {
        return -(1 as std::os::raw::c_int)
    } /* start using pnm_fget_char() */
    ch = getc(stream);
    if ch < '1' as i32 || ch > '9' as i32 { return -(1 as std::os::raw::c_int) }
    format = (ch - '0' as i32) as std::os::raw::c_uint;
    ch = pnm_fget_char(stream);
    if !(ch == ' ' as i32 || ch == '\t' as i32 || ch == '\n' as i32 ||
             ch == '\r' as i32) {
        return -(1 as std::os::raw::c_int)
    }
    /* read the header */
    (*pnm_ptr).format = format;
    if format >= PNM_P1 as std::os::raw::c_int as std::os::raw::c_uint &&
           format <= PNM_P6 as std::os::raw::c_int as std::os::raw::c_uint {
        /* old-style PNM header */
        (*pnm_ptr).depth =
            if format == PNM_P3 as std::os::raw::c_int as std::os::raw::c_uint ||
                   format == PNM_P6 as std::os::raw::c_int as std::os::raw::c_uint {
                3 as std::os::raw::c_int
            } else { 1 as std::os::raw::c_int } as std::os::raw::c_uint;
        if pnm_fscan_uint(stream, Some(&mut (*pnm_ptr).width)) != 1 as std::os::raw::c_int
               ||
               pnm_fscan_uint(stream, Some(&mut (*pnm_ptr).height)) !=
                   1 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
        if format == PNM_P1 as std::os::raw::c_int as std::os::raw::c_uint ||
               format == PNM_P4 as std::os::raw::c_int as std::os::raw::c_uint {
            (*pnm_ptr).maxval = 1 as std::os::raw::c_int as std::os::raw::c_uint
        } else if pnm_fscan_uint(stream, Some(&mut (*pnm_ptr).maxval)) !=
                      1 as std::os::raw::c_int {
            return -(1 as std::os::raw::c_int)
        }
        return if pnm_is_valid(pnm_ptr) != 0 {
                   1 as std::os::raw::c_int
               } else { 0 as std::os::raw::c_int }
    } else {
        /* TODO: if (format == PNM_P7) ... */
        return -(1 as std::os::raw::c_int)
    };
}
/* *
 * Reads an array of PNM sample values from a file stream.
 * The values are expected to be in the format specified by pnm_ptr->format.
 * The array length is pnm_ptr->depth * pnm_ptr->width * num_rows.
 * The validity check performed on the PNM structure is only partial.
 * Returns 1 on success, 0 on validation failure,
 * or -1 on input or matching failure.
 * If reading is incomplete, the remaining sample values are set to 0.
 **/
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_values<'a1>(mut pnm_ptr: Option<&'a1 crate::src::pngxtern::pngxrpnm::pnm_struct>,
                                         mut sample_values: * mut std::os::raw::c_uint,
                                         mut num_rows: std::os::raw::c_uint,
                                         mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_int {
    let mut format: u32 = (*(pnm_ptr).unwrap()).format;
    let mut depth: u32 = (*(pnm_ptr).unwrap()).depth;
    let mut width: u32 = (*(pnm_ptr).unwrap()).width;
    let mut maxval: u32 = (*(pnm_ptr).unwrap()).maxval;
    let mut row_length: u64 =
        (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: u64 =
        (num_rows as std::os::raw::c_ulong).wrapping_mul(row_length);
    let mut ch: i32 = 0;
    let mut ch8: i32 = 0;
    let mut ch16: i32 = 0;
    let mut ch24: i32 = 0;
    let mut mask: i32 = 0;
    let mut i: u64 = 0;
    let mut j: u64 = 0;
    /* read the sample values */
    match format {
        1 => {
            i = 0 as std::os::raw::c_int as size_t;
            while i < num_samples {
                loop  {
                    ch = pnm_fget_char(stream);
                    if !(ch == ' ' as i32 || ch == '\t' as i32 ||
                             ch == '\n' as i32 || ch == '\r' as i32) {
                        break ;
                    }
                }
                if ch != '0' as i32 && ch != '1' as i32 {
                    ungetc(ch, stream);
                    break ;
                } else {
                    *sample_values.offset(i as isize) =
                        if ch == '0' as i32 {
                            1 as std::os::raw::c_int
                        } else { 0 as std::os::raw::c_int } as std::os::raw::c_uint;
                    i = i.wrapping_add(1)
                }
            }
        }
        2 | 3 => {
            i = 0 as std::os::raw::c_int as size_t;
            while i < num_samples {
                if pnm_fscan_uint(stream,
                                  Some(&mut *sample_values.offset(i as isize))) !=
                       1 as std::os::raw::c_int {
                    break ;
                }
                i = i.wrapping_add(1)
            }
        }
        4 => {
            j = 0 as std::os::raw::c_int as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == -(1 as std::os::raw::c_int) { break ; }
                mask = 0x80 as std::os::raw::c_int;
                while mask != 0 as std::os::raw::c_int {
                    let mut fresh0 = i;
                    i = i.wrapping_add(1);
                    *sample_values.offset(fresh0 as isize) =
                        if ch & mask != 0 {
                            0 as std::os::raw::c_int
                        } else { 1 as std::os::raw::c_int } as std::os::raw::c_uint;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as std::os::raw::c_int as size_t;
                        break ;
                    } else { mask >>= 1 as std::os::raw::c_int }
                }
            }
        }
        5 | 6 | 7 => {
            if maxval <= 0xff as std::os::raw::c_uint {
                /* 1 byte per sample */
                i = 0 as std::os::raw::c_int as size_t;
                while i < num_samples {
                    ch = getc(stream);
                    if ch == -(1 as std::os::raw::c_int) { break ; }
                    *sample_values.offset(i as isize) = ch as std::os::raw::c_uint;
                    i = i.wrapping_add(1)
                }
            } else if maxval <= 0xffff as std::os::raw::c_uint {
                /* 2 bytes per sample */
                i = 0 as std::os::raw::c_int as size_t;
                while i < num_samples {
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == -(1 as std::os::raw::c_int) { break ; }
                    *sample_values.offset(i as isize) =
                        ((ch8 as std::os::raw::c_uint) <<
                             8 as
                                 std::os::raw::c_int).wrapping_add(ch as
                                                               std::os::raw::c_uint);
                    i = i.wrapping_add(1)
                }
            } else if maxval <= 0xffffffff as std::os::raw::c_uint {
                /* 3 or 4 bytes per sample */
                ch24 = 0 as std::os::raw::c_int;
                i = 0 as std::os::raw::c_int as size_t;
                while i < num_samples {
                    if maxval > 0xffffff as std::os::raw::c_uint {
                        ch24 = getc(stream)
                    }
                    ch16 = getc(stream);
                    ch8 = getc(stream);
                    ch = getc(stream);
                    if ch == -(1 as std::os::raw::c_int) { break ; }
                    *sample_values.offset(i as isize) =
                        ((ch24 as std::os::raw::c_uint) <<
                             24 as
                                 std::os::raw::c_int).wrapping_add((ch16 as
                                                                std::os::raw::c_uint)
                                                               <<
                                                               16 as
                                                                   std::os::raw::c_int).wrapping_add((ch8
                                                                                                  as
                                                                                                  std::os::raw::c_uint)
                                                                                                 <<
                                                                                                 8
                                                                                                     as
                                                                                                     std::os::raw::c_int).wrapping_add(ch
                                                                                                                                   as
                                                                                                                                   std::os::raw::c_uint);
                    i = i.wrapping_add(1)
                }
            } else {
                /* maxval > PNM_UINT_MAX */
                *__errno_location() = 22 as std::os::raw::c_int;
                return 0 as std::os::raw::c_int
            }
        }
        _ => {
            *__errno_location() = 22 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
    }
    /* check the result */
    if i < num_samples {
        memset(sample_values.offset(i as isize) as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               num_samples.wrapping_sub(i).wrapping_mul(::std::mem::size_of::<std::os::raw::c_uint>()
                                                            as
                                                            std::os::raw::c_ulong));
        return -(1 as std::os::raw::c_int)
    }
    return 1 as std::os::raw::c_int;
}
/* *
 * Reads an array of sample bytes from a raw PNM file stream.
 * Multi-byte samples are stored in network order, as in the PNM stream.
 * The byte count is sample_size * pnm_ptr->depth * pnm_ptr->width * num_rows.
 * The validity check performed on the PNM structure is only partial.
 * Returns 1 on success, 0 on validation failure, or -1 on input failure.
 * If reading is incomplete, the remaining sample bytes are set to 0.
 **/
#[no_mangle]
pub unsafe extern "C" fn pnm_fget_bytes<'a1>(mut pnm_ptr: Option<&'a1 crate::src::pngxtern::pngxrpnm::pnm_struct>,
                                        mut sample_bytes: * mut std::os::raw::c_uchar,
                                        mut sample_size: std::os::raw::c_ulong,
                                        mut num_rows: std::os::raw::c_uint,
                                        mut stream: * mut crate::src::gifread::gifread::_IO_FILE)
 -> std::os::raw::c_int {
    let mut format: u32 = (*(pnm_ptr).unwrap()).format;
    let mut depth: u32 = (*(pnm_ptr).unwrap()).depth;
    let mut width: u32 = (*(pnm_ptr).unwrap()).width;
    let mut maxval: u32 = (*(pnm_ptr).unwrap()).maxval;
    let mut row_length: u64 =
        (depth as size_t).wrapping_mul(width as size_t);
    let mut num_samples: u64 =
        (num_rows as std::os::raw::c_ulong).wrapping_mul(row_length);
    let mut raw_sample_size: u64 = 0;
    let mut ch: i32 = 0;
    let mut mask: i32 = 0;
    let mut i: u64 = 0;
    let mut j: u64 = 0;
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
    /* read the raw sample bytes */
    match format {
        4 => {
            j = 0 as std::os::raw::c_int as size_t;
            i = j;
            while i < num_samples {
                ch = getc(stream);
                if ch == -(1 as std::os::raw::c_int) { break ; }
                mask = 0x80 as std::os::raw::c_int;
                while mask != 0 as std::os::raw::c_int {
                    let mut fresh1 = i;
                    i = i.wrapping_add(1);
                    *sample_bytes.offset(fresh1 as isize) =
                        if ch & mask != 0 {
                            0 as std::os::raw::c_int
                        } else { 1 as std::os::raw::c_int } as std::os::raw::c_uchar;
                    j = j.wrapping_add(1);
                    if j == row_length {
                        j = 0 as std::os::raw::c_int as size_t;
                        break ;
                    } else { mask >>= 1 as std::os::raw::c_int }
                }
            }
        }
        5 | 6 | 7 => {
            i =
                fread(sample_bytes as *mut std::os::raw::c_void, sample_size,
                      num_samples, stream)
        }
        _ => {
            *__errno_location() = 22 as std::os::raw::c_int;
            return 0 as std::os::raw::c_int
        }
    }
    /* check the result */
    if i < num_samples {
        memset(sample_bytes.offset(i as isize) as *mut std::os::raw::c_void,
               0 as std::os::raw::c_int,
               sample_size.wrapping_mul(num_samples).wrapping_sub(i));
        return -(1 as std::os::raw::c_int)
    }
    return 1 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
