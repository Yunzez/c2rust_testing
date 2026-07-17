
extern "C" {
    
    
    
    
    #[no_mangle]
    static mut stderr: * mut crate::src::gifread::gifread::_IO_FILE;
    #[no_mangle]
    fn fprintf(_: * mut crate::src::gifread::gifread::_IO_FILE, _: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    #[no_mangle]
    fn abort() -> !;
}
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
pub type png_charp = * mut std::os::raw::c_char;
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
pub type png_ptruint = std::os::raw::c_ulong;
/* WARNINGS */
/* This function is called whenever there is a fatal error.  This function
 * should not be changed.  If there is a need to handle errors differently,
 * you should supply a replacement error function and use png_set_error_fn()
 * to replace the error function at run-time.
 */
#[no_mangle]
pub unsafe extern "C" fn png_error(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                   mut error_message: * const std::os::raw::c_char) -> ! {
    if !png_ptr.is_null() && (*png_ptr).error_fn.is_some() {
        Some((*png_ptr).error_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr
                                                                                                              as
                                                                                                              *const std::os::raw::c_void
                                                                                                              as
                                                                                                              png_ptruint
                                                                                                              as
                                                                                                              *mut png_struct,
                                                                                                          error_message);
    }
    /* If the custom handler doesn't exist, or if it returns,
      use the default handler, which will not return. */
    png_default_error(png_ptr, error_message);
}
/* ERROR_TEXT */
/* Utility to safely appends strings to a buffer.  This never errors out so
 * error checking is not required in the caller.
 */
#[no_mangle]
pub unsafe extern "C" fn png_safecat(mut buffer: * mut std::os::raw::c_char,
                                     mut bufsize: std::os::raw::c_ulong, mut pos: std::os::raw::c_ulong,
                                     mut string: * const std::os::raw::c_char) -> std::os::raw::c_ulong {
    if !buffer.is_null() && pos < bufsize {
        if !string.is_null() {
            while *string as std::os::raw::c_int != '\u{0}' as i32 &&
                      pos <
                          bufsize.wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) {
                let mut fresh0 = string;
                string = string.offset(1);
                let mut fresh1 = pos;
                pos = pos.wrapping_add(1);
                *buffer.offset(fresh1 as isize) = *fresh0
            }
        }
        *buffer.offset(pos as isize) = '\u{0}' as i32 as std::os::raw::c_char
    }
    return pos;
}
/* Utility to dump an unsigned value into a buffer, given a start pointer and
 * and end pointer (which should point just *beyond* the end of the buffer!)
 * Returns the pointer to the start of the formatted string.
 */
#[no_mangle]
pub unsafe extern "C" fn png_format_number(mut start: * const std::os::raw::c_char,
                                           mut end: * mut std::os::raw::c_char,
                                           mut format: std::os::raw::c_int,
                                           mut number: std::os::raw::c_ulong)
 -> * mut std::os::raw::c_char {
    let mut count: i32 =
        0 as std::os::raw::c_int; /* number of digits output */
    let mut mincount: i32 =
        1 as std::os::raw::c_int; /* minimum number required */
    let mut output: i32 =
        0 as std::os::raw::c_int; /* digit output (for the fixed point format) */
    end = end.offset(-1);
    *end = '\u{0}' as i32 as std::os::raw::c_char;
    /* This is written so that the loop always runs at least once, even with
    * number zero.
    */
    while end > start as png_charp &&
              (number != 0 as std::os::raw::c_int as std::os::raw::c_ulong ||
                   count < mincount) {
        static mut digits: [std::os::raw::c_char; 17] =
            [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_digits() {
digits = unsafe {
                *core::intrinsics::transmute::<&'_ [u8; 17], &'_ [i8; 17]>(b"0123456789ABCDEF\x00")
            };}//;
        let mut current_block_13: u64;
        match format {
            5 => {
                /* Needs five digits (the fraction) */
                mincount = 5 as std::os::raw::c_int;
                if output != 0 as std::os::raw::c_int ||
                       number.wrapping_rem(10 as std::os::raw::c_int as std::os::raw::c_ulong)
                           != 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                    end = end.offset(-1);
                    *end =
                        digits[number.wrapping_rem(10 as std::os::raw::c_int as
                                                       std::os::raw::c_ulong) as
                                   usize];
                    output = 1 as std::os::raw::c_int
                }
                number =
                    (number as
                         std::os::raw::c_ulong).wrapping_div(10 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        png_alloc_size_t as png_alloc_size_t;
                current_block_13 = 13797916685926291137;
            }
            2 => {
                /* Expects at least 2 digits. */
                mincount = 2 as std::os::raw::c_int;
                current_block_13 = 8801382743313005935;
            }
            1 => { current_block_13 = 8801382743313005935; }
            4 => {
                /* This format expects at least two digits */
                mincount = 2 as std::os::raw::c_int;
                current_block_13 = 361767692244619964;
            }
            3 => { current_block_13 = 361767692244619964; }
            _ => {
                /* an error */
                number = 0 as std::os::raw::c_int as png_alloc_size_t;
                current_block_13 = 13797916685926291137;
            }
        }
        match current_block_13 {
            361767692244619964 =>
            /* FALLTHROUGH */
            {
                end = end.offset(-1);
                *end =
                    digits[(number & 0xf as std::os::raw::c_int as std::os::raw::c_ulong) as
                               usize];
                number >>= 4 as std::os::raw::c_int
            }
            8801382743313005935 =>
            /* FALLTHROUGH */
            {
                end = end.offset(-1);
                *end =
                    digits[number.wrapping_rem(10 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) as usize];
                number =
                    (number as
                         std::os::raw::c_ulong).wrapping_div(10 as std::os::raw::c_int as
                                                         std::os::raw::c_ulong) as
                        png_alloc_size_t as png_alloc_size_t
            }
            _ => { }
        }
        /* Keep track of the number of digits added */
        count += 1;
        /* Float a fixed number here: */
        if format == 5 as std::os::raw::c_int && count == 5 as std::os::raw::c_int &&
               end > start as png_charp {
            /* End of the fraction, but maybe nothing was output?  In that case
          * drop the decimal point.  If the number is a true zero handle that
          * here.
          */
            if output != 0 as std::os::raw::c_int {
                end = end.offset(-1);
                *end = '.' as i32 as std::os::raw::c_char
            } else if number == 0 as std::os::raw::c_int as std::os::raw::c_ulong {
                /* and !output */
                end = end.offset(-1);
                *end = '0' as i32 as std::os::raw::c_char
            }
        }
    }
    return end;
}
/* This function is called whenever there is a non-fatal error.  This function
 * should not be changed.  If there is a need to handle warnings differently,
 * you should supply a replacement warning function and use
 * png_set_error_fn() to replace the warning function at run-time.
 */
#[no_mangle]
pub unsafe extern "C" fn png_warning(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                     mut warning_message: * const std::os::raw::c_char) {
    let mut offset: i32 = 0 as std::os::raw::c_int;
    if !png_ptr.is_null() {
        if *warning_message as std::os::raw::c_int == 0x23 as std::os::raw::c_int {
            offset = 1 as std::os::raw::c_int;
            while offset < 15 as std::os::raw::c_int {
                if *warning_message.offset(offset as isize) as std::os::raw::c_int ==
                       ' ' as i32 {
                    break ;
                }
                offset += 1
            }
        }
    }
    if !png_ptr.is_null() && (*png_ptr).warning_fn.is_some() {
        Some((*png_ptr).warning_fn.expect("non-null function pointer")).expect("non-null function pointer")(png_ptr
                                                                                                                as
                                                                                                                *const std::os::raw::c_void
                                                                                                                as
                                                                                                                png_ptruint
                                                                                                                as
                                                                                                                *mut png_struct,
                                                                                                            warning_message.offset(offset
                                                                                                                                       as
                                                                                                                                       isize));
    } else {
        png_default_warning(png_ptr, warning_message.offset(offset as isize));
    };
}
/* These functions support 'formatted' warning messages with up to
 * PNG_WARNING_PARAMETER_COUNT parameters.  In the format string the parameter
 * is introduced by @<number>, where 'number' starts at 1.  This follows the
 * standard established by X/Open for internationalizable error messages.
 */
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter(mut p: * mut [std::os::raw::c_char; 32],
                                               mut number: std::os::raw::c_int,
                                               mut string: * const std::os::raw::c_char) {
    if number > 0 as std::os::raw::c_int && number <= 8 as std::os::raw::c_int {
        png_safecat((*p.offset((number - 1 as std::os::raw::c_int) as
                                   isize)).as_mut_ptr(),
                    ::std::mem::size_of::<[std::os::raw::c_char; 32]>() as
                        std::os::raw::c_ulong, 0 as std::os::raw::c_int as size_t, string);
    };
}
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter_unsigned(mut p:
                                                            * mut [std::os::raw::c_char; 32],
                                                        mut number:
                                                            std::os::raw::c_int,
                                                        mut format:
                                                            std::os::raw::c_int,
                                                        mut value:
                                                            std::os::raw::c_ulong) {
    let mut buffer: [i8; 24] = [0; 24];
    png_warning_parameter(p, number,
                          png_format_number(buffer.as_mut_ptr() as
                                                png_const_charp,
                                            buffer.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 24]>()
                                                                           as
                                                                           std::os::raw::c_ulong
                                                                           as
                                                                           isize),
                                            format, value) as
                              png_const_charp);
}
#[no_mangle]
pub unsafe extern "C" fn png_warning_parameter_signed(mut p:
                                                          * mut [std::os::raw::c_char; 32],
                                                      mut number: std::os::raw::c_int,
                                                      mut format: std::os::raw::c_int,
                                                      mut value: std::os::raw::c_int) {
    let mut u: u64 = 0;
    let mut str: * mut i8 = (0 as * mut i8);
    let mut buffer: [i8; 24] = [0; 24];
    /* Avoid overflow by doing the negate in a png_alloc_size_t: */
    u = value as png_alloc_size_t;
    if value < 0 as std::os::raw::c_int {
        u = (!u).wrapping_add(1 as std::os::raw::c_int as std::os::raw::c_ulong)
    }
    str =
        png_format_number(buffer.as_mut_ptr() as png_const_charp,
                          buffer.as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 24]>()
                                                         as std::os::raw::c_ulong as
                                                         isize), format, u);
    if value < 0 as std::os::raw::c_int && str > buffer.as_mut_ptr() {
        str = str.offset(-1);
        *str = '-' as i32 as std::os::raw::c_char
    }
    png_warning_parameter(p, number, str as png_const_charp);
}
#[no_mangle]
pub unsafe extern "C" fn png_formatted_warning(mut png_ptr:
                                                   * const crate::src::libpng::png::png_struct_def,
                                               mut p: * mut [std::os::raw::c_char; 32],
                                               mut message: * const std::os::raw::c_char) {
    /* The internal buffer is just 192 bytes - enough for all our messages,
    * overflow doesn't happen because this code checks!  If someone figures
    * out how to send us a message longer than 192 bytes, all that will
    * happen is that the message will be truncated appropriately.
    */
    let mut i: u64 =
        0 as std::os::raw::c_int as size_t; /* Index in the msg[] buffer: */
    let mut msg: [i8; 192] = [0; 192];
    /* Each iteration through the following loop writes at most one character
    * to msg[i++] then returns here to validate that there is still space for
    * the trailing '\0'.  It may (in the case of a parameter) read more than
    * one character from message[]; it must check for '\0' and continue to the
    * test if it finds the end of string.
    */
    while i <
              (::std::mem::size_of::<[std::os::raw::c_char; 192]>() as
                   std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                   std::os::raw::c_ulong) &&
              *message as std::os::raw::c_int != '\u{0}' as i32 {
        /* '@' at end of string is now just printed (previously it was skipped);
       * it is an error in the calling code to terminate the string with @.
       */
        if !p.is_null() && *message as std::os::raw::c_int == '@' as i32 &&
               *message.offset(1 as std::os::raw::c_int as isize) as std::os::raw::c_int !=
                   '\u{0}' as i32 {
            message = message.offset(1); /* Consume the '@' */
            let mut parameter_char: i32 = *message as std::os::raw::c_int;
            static mut valid_parameters: [std::os::raw::c_char; 10] =
                [0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_valid_parameters() {
valid_parameters = unsafe {
                    *core::intrinsics::transmute::<&'_ [u8; 10], &'_ [i8; 10]>(b"123456789\x00")
                };}//;
            let mut parameter: i32 = 0 as std::os::raw::c_int;
            /* Search for the parameter digit, the index in the string is the
          * parameter to use.
          */
            while valid_parameters[parameter as usize] as std::os::raw::c_int !=
                      parameter_char &&
                      valid_parameters[parameter as usize] as std::os::raw::c_int !=
                          '\u{0}' as i32 {
                parameter += 1
            }
            /* If the parameter digit is out of range it will just get printed. */
            if parameter < 8 as std::os::raw::c_int {
                /* Append this parameter */
                let mut parm: * const i8 =
                    (*p.offset(parameter as isize)).as_mut_ptr() as
                        png_const_charp;
                let mut pend: * const i8 =
                    (*p.offset(parameter as
                                   isize)).as_mut_ptr().offset(::std::mem::size_of::<[std::os::raw::c_char; 32]>()
                                                                   as
                                                                   std::os::raw::c_ulong
                                                                   as isize)
                        as png_const_charp;
                /* No need to copy the trailing '\0' here, but there is no guarantee
             * that parm[] has been initialized, so there is no guarantee of a
             * trailing '\0':
             */
                while i <
                          (::std::mem::size_of::<[std::os::raw::c_char; 192]>() as
                               std::os::raw::c_ulong).wrapping_sub(1 as std::os::raw::c_int as
                                                               std::os::raw::c_ulong)
                          && *parm as std::os::raw::c_int != '\u{0}' as i32 &&
                          parm < pend {
                    let mut fresh2 = parm;
                    parm = parm.offset(1);
                    let mut fresh3 = i;
                    i = i.wrapping_add(1);
                    msg[fresh3 as usize] = *fresh2
                }
                /* Consume the parameter digit too: */
                message = message.offset(1);
                continue ;
            }
            /* else not a parameter and there is a character after the @ sign; just
          * copy that.  This is known not to be '\0' because of the test above.
          */
        }
        /* At this point *message can't be '\0', even in the bad parameter case
       * above where there is a lone '@' at the end of the message string.
       */
        let mut fresh4 = message;
        message = message.offset(1);
        let mut fresh5 = i;
        i = i.wrapping_add(1);
        msg[fresh5 as usize] = *fresh4
    }
    /* i is always less than (sizeof msg), so: */
    msg[i as usize] = '\u{0}' as i32 as std::os::raw::c_char;
    /* And this is the formatted message. It may be larger than
    * PNG_MAX_ERROR_TEXT, but that is only used for 'chunk' errors and these
    * are not (currently) formatted.
    */
    png_warning(png_ptr, msg.as_mut_ptr() as png_const_charp);
}
/* WARNINGS */
#[no_mangle]
pub unsafe extern "C" fn png_benign_error(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                          mut error_message:
                                              * const std::os::raw::c_char) {
    if (*png_ptr).flags & 0x100000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
               0 as std::os::raw::c_int as std::os::raw::c_uint &&
               (*png_ptr).chunk_name != 0 as std::os::raw::c_int as std::os::raw::c_uint {
            png_chunk_warning(png_ptr, error_message);
        } else { png_warning(png_ptr, error_message); }
    } else if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
                  0 as std::os::raw::c_int as std::os::raw::c_uint &&
                  (*png_ptr).chunk_name != 0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_chunk_error(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
#[no_mangle]
pub unsafe extern "C" fn png_app_warning(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                         mut error_message: * const std::os::raw::c_char) {
    if (*png_ptr).flags & 0x200000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
#[no_mangle]
pub unsafe extern "C" fn png_app_error(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                       mut error_message: * const std::os::raw::c_char) {
    if (*png_ptr).flags & 0x400000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_warning(png_ptr, error_message);
    } else { png_error(png_ptr, error_message); };
}
static mut png_digit: [std::os::raw::c_char; 16] =
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,]; unsafe fn laertes_init_png_digit() {
png_digit = ['0' as i32 as std::os::raw::c_char, '1' as i32 as std::os::raw::c_char,
     '2' as i32 as std::os::raw::c_char, '3' as i32 as std::os::raw::c_char,
     '4' as i32 as std::os::raw::c_char, '5' as i32 as std::os::raw::c_char,
     '6' as i32 as std::os::raw::c_char, '7' as i32 as std::os::raw::c_char,
     '8' as i32 as std::os::raw::c_char, '9' as i32 as std::os::raw::c_char,
     'A' as i32 as std::os::raw::c_char, 'B' as i32 as std::os::raw::c_char,
     'C' as i32 as std::os::raw::c_char, 'D' as i32 as std::os::raw::c_char,
     'E' as i32 as std::os::raw::c_char, 'F' as i32 as std::os::raw::c_char];}//;
unsafe extern "C" fn png_format_buffer(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                       mut buffer: * mut std::os::raw::c_char,
                                       mut error_message: * const std::os::raw::c_char) {
    let mut chunk_name: u32 = (*png_ptr).chunk_name;
    let mut iout: i32 = 0 as std::os::raw::c_int;
    let mut ishift: i32 = 24 as std::os::raw::c_int;
    while ishift >= 0 as std::os::raw::c_int {
        let mut c: i32 =
            (chunk_name >> ishift) as std::os::raw::c_int & 0xff as std::os::raw::c_int;
        ishift -= 8 as std::os::raw::c_int;
        if (c < 65 as std::os::raw::c_int || c > 122 as std::os::raw::c_int ||
                c > 90 as std::os::raw::c_int && c < 97 as std::os::raw::c_int) as std::os::raw::c_int
               != 0 as std::os::raw::c_int {
            let mut fresh6 = iout;
            iout = iout + 1;
            *buffer.offset(fresh6 as isize) =
                0x5b as std::os::raw::c_int as std::os::raw::c_char;
            let mut fresh7 = iout;
            iout = iout + 1;
            *buffer.offset(fresh7 as isize) =
                png_digit[((c & 0xf0 as std::os::raw::c_int) >> 4 as std::os::raw::c_int) as
                              usize];
            let mut fresh8 = iout;
            iout = iout + 1;
            *buffer.offset(fresh8 as isize) =
                png_digit[(c & 0xf as std::os::raw::c_int) as usize];
            let mut fresh9 = iout;
            iout = iout + 1;
            *buffer.offset(fresh9 as isize) =
                0x5d as std::os::raw::c_int as std::os::raw::c_char
        } else {
            let mut fresh10 = iout;
            iout = iout + 1;
            *buffer.offset(fresh10 as isize) = c as std::os::raw::c_char
        }
    }
    if error_message.is_null() {
        *buffer.offset(iout as isize) = '\u{0}' as i32 as std::os::raw::c_char
    } else {
        let mut iin: i32 = 0 as std::os::raw::c_int;
        let mut fresh11 = iout;
        iout = iout + 1;
        *buffer.offset(fresh11 as isize) = ':' as i32 as std::os::raw::c_char;
        let mut fresh12 = iout;
        iout = iout + 1;
        *buffer.offset(fresh12 as isize) = ' ' as i32 as std::os::raw::c_char;
        while iin < 196 as std::os::raw::c_int - 1 as std::os::raw::c_int &&
                  *error_message.offset(iin as isize) as std::os::raw::c_int !=
                      '\u{0}' as i32 {
            let mut fresh13 = iin;
            iin = iin + 1;
            let mut fresh14 = iout;
            iout = iout + 1;
            *buffer.offset(fresh14 as isize) =
                *error_message.offset(fresh13 as isize)
        }
        /* iin < PNG_MAX_ERROR_TEXT, so the following is safe: */
        *buffer.offset(iout as isize) = '\u{0}' as i32 as std::os::raw::c_char
    };
}
/* WARNINGS || ERROR_TEXT */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_error(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                         mut error_message: * const std::os::raw::c_char)
 -> ! {
    let mut msg: [i8; 214] = [0; 214];
    if png_ptr.is_null() {
        png_error(png_ptr, error_message);
    } else {
        png_format_buffer(png_ptr, msg.as_mut_ptr(), error_message);
        png_error(png_ptr, msg.as_mut_ptr() as png_const_charp);
    };
}
/* READ && ERROR_TEXT */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_warning(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                           mut warning_message:
                                               * const std::os::raw::c_char) {
    let mut msg: [i8; 214] = [0; 214];
    if png_ptr.is_null() {
        png_warning(png_ptr, warning_message);
    } else {
        png_format_buffer(png_ptr, msg.as_mut_ptr(), warning_message);
        png_warning(png_ptr, msg.as_mut_ptr() as png_const_charp);
    };
}
/* WARNINGS */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_benign_error(mut png_ptr:
                                                    * const crate::src::libpng::png::png_struct_def,
                                                mut error_message:
                                                    * const std::os::raw::c_char) {
    if (*png_ptr).flags & 0x100000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        png_chunk_warning(png_ptr, error_message);
    } else { png_chunk_error(png_ptr, error_message); };
}
/* READ */
#[no_mangle]
pub unsafe extern "C" fn png_chunk_report(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                          mut message: * const std::os::raw::c_char,
                                          mut error: std::os::raw::c_int) {
    /* This is always supported, but for just read or just write it
    * unconditionally does the right thing.
    */
    if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint !=
           0 as std::os::raw::c_int as std::os::raw::c_uint {
        if error < 2 as std::os::raw::c_int {
            png_chunk_warning(png_ptr, message);
        } else { png_chunk_benign_error(png_ptr, message); }
    } else if (*png_ptr).mode & 0x8000 as std::os::raw::c_uint ==
                  0 as std::os::raw::c_int as std::os::raw::c_uint {
        if error < 1 as std::os::raw::c_int {
            png_app_warning(png_ptr, message);
        } else { png_app_error(png_ptr, message); }
    };
}
/* pngerror.c - stub functions for i/o and memory allocation
 *
 * Last changed in libpng 1.6.31 [July 27, 2017]
 * Copyright (c) 1998-2002,2004,2006-2017 Glenn Randers-Pehrson
 * (Version 0.96 Copyright (c) 1996, 1997 Andreas Dilger)
 * (Version 0.88 Copyright (c) 1995, 1996 Guy Eric Schalnat, Group 42, Inc.)
 *
 * This code is released under the libpng license.
 * For conditions of distribution and use, see the disclaimer
 * and license in png.h
 *
 * This file provides a location for all error handling.  Users who
 * need special error handling are expected to write replacement functions
 * and use png_set_error_fn() to use those functions.  See the instructions
 * at each function.
 */
/* This is the default error handling function.  Note that replacements for
 * this function MUST NOT RETURN, or the program will likely crash.  This
 * function is used by default, or if the program supplies NULL for the
 * error function pointer in png_set_error_fn().
 */
unsafe extern "C" fn png_default_error(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                       mut error_message: * const std::os::raw::c_char)
 -> ! {
    fprintf(stderr,
            b"libpng error: %s\x00" as *const u8 as *const std::os::raw::c_char,
            if !error_message.is_null() {
                error_message
            } else { b"undefined\x00" as *const u8 as *const std::os::raw::c_char });
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    png_longjmp(png_ptr, 1 as std::os::raw::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn png_longjmp(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                     mut val: std::os::raw::c_int) -> ! {
    /* If control reaches this point, png_longjmp() must not return. The only
    * choice is to terminate the whole process (or maybe the thread); to do
    * this the ANSI-C abort() function is used unless a different method is
    * implemented by overriding the default configuration setting for
    * PNG_ABORT().
    */
    abort();
}
/* This function is called when there is a warning, but the library thinks
 * it can continue anyway.  Replacement functions don't have to do anything
 * here if you don't want them to.  In the default configuration, png_ptr is
 * not used, but it is passed in case it may be useful.
 */
unsafe extern "C" fn png_default_warning(mut png_ptr: * const crate::src::libpng::png::png_struct_def,
                                         mut warning_message:
                                             * const std::os::raw::c_char) {
    fprintf(stderr,
            b"libpng warning: %s\x00" as *const u8 as *const std::os::raw::c_char,
            warning_message);
    fprintf(stderr, b"\n\x00" as *const u8 as *const std::os::raw::c_char);
    /* Make compiler happy */
}
/* WARNINGS */
/* This function is called when the application wants to use another method
 * of handling errors and warnings.  Note that the error function MUST NOT
 * return to the calling routine or serious problems will occur.  The return
 * method used in the default routine calls longjmp(png_ptr->jmp_buf_ptr, 1)
 */
#[no_mangle]
pub unsafe extern "C" fn png_set_error_fn<'a1>(mut png_ptr: Option<&'a1 mut crate::src::libpng::png::png_struct_def>,
                                          mut error_ptr: * mut core::ffi::c_void,
                                          mut error_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>,
                                          mut warning_fn: Option<unsafe extern "C"  fn(_: * mut crate::src::libpng::png::png_struct_def,_: * const std::os::raw::c_char,) -> ()>) {
    if borrow(& png_ptr).is_none() { return }
    (*(borrow_mut(&mut png_ptr)).unwrap()).error_ptr = error_ptr;
    (*(borrow_mut(&mut png_ptr)).unwrap()).error_fn = error_fn;
    (*(borrow_mut(&mut png_ptr)).unwrap()).warning_fn = warning_fn;
}
/* This function returns a pointer to the error_ptr associated with the user
 * functions.  The application should free any memory associated with this
 * pointer before png_write_destroy and png_read_destroy are called.
 */
#[no_mangle]
pub unsafe extern "C" fn png_get_error_ptr<'a1>(mut png_ptr: Option<&'a1 crate::src::libpng::png::png_struct_def>)
 -> * mut core::ffi::c_void {
    if (png_ptr).clone().is_none() { return 0 as *mut std::os::raw::c_void }
    return (*((png_ptr).clone()).unwrap()).error_ptr;
}
use crate::laertes_rt::*;
/* READ || WRITE */
/* SIMPLIFIED READ || SIMPLIFIED_WRITE */
