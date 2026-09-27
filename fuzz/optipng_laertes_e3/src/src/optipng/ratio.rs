
extern "C" {
    #[no_mangle]
    fn vsnprintf(_: * mut std::os::raw::c_char, _: std::os::raw::c_ulong,
                 _: * const std::os::raw::c_char, _: core::ffi::VaList)
     -> std::os::raw::c_int;
}
pub type __builtin_va_list = [crate::src::optipng::optipng::__va_list_tag; 1];
// #[derive(Copy, Clone)]

pub type __va_list_tag = crate::src::optipng::optipng::__va_list_tag;
pub type size_t = std::os::raw::c_ulong;
pub type opng_ullong_t = std::os::raw::c_ulonglong;
// #[derive(Copy, Clone)]

pub type opng_ulratio = crate::src::optipng::optim::opng_ulratio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ullratio {
    pub num: std::os::raw::c_ulonglong,
    pub denom: std::os::raw::c_ulonglong,
}
impl opng_ullratio {
    pub const fn new() -> Self {
        opng_ullratio {
        num: 0,
        denom: 0
        }
    }
}

impl std::default::Default for opng_ullratio {
    fn default() -> Self { opng_ullratio::new() }
}

pub type opng_uxlong_impl_t = std::os::raw::c_ulonglong;
pub type va_list = [crate::src::optipng::optipng::__va_list_tag; 1];
/*
 * Writes formatted output to a memory buffer.
 * This is a wrapper to [v]snprintf which avoids well-known defects
 * occurring in some of the underlying snprintf implementations.
 * The function returns the number of characters written, excluding the
 * null-termination character, if the buffer size is large enough, or -1
 * otherwise. (Unlike the proper snprintf, this function does not return
 * a number larger than zero if the buffer size is too small.)
 */
unsafe extern "C" fn opng_snprintf_impl(mut buffer: * mut std::os::raw::c_char,
                                        mut buffer_size: std::os::raw::c_ulong,
                                        mut format: * const std::os::raw::c_char,
                                        mut args: ...) -> std::os::raw::c_int {
    let mut arg_ptr: core::ffi::VaListImpl;
    let mut result: i32 = 0;
    arg_ptr = args.clone();
    result = vsnprintf(buffer, buffer_size, format, arg_ptr.as_va_list());
    if result < 0 as std::os::raw::c_int || result as size_t >= buffer_size {
        /* Guard against broken [v]snprintf implementations. */
        if buffer_size > 0 as std::os::raw::c_int as std::os::raw::c_ulong {
            *buffer.offset(buffer_size.wrapping_sub(1 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong) as
                               isize) = '\u{0}' as i32 as std::os::raw::c_char
        }
        return -(1 as std::os::raw::c_int)
    }
    return result;
}
/*
 * Writes a decomposed rational value to a memory buffer.
 * This is the base implementation used internally by the the other
 * ratio-to-string conversion functions.
 */
unsafe extern "C" fn opng_sprint_uratio_impl(mut buffer: * mut std::os::raw::c_char,
                                             mut buffer_size: std::os::raw::c_ulong,
                                             mut num: std::os::raw::c_ulonglong,
                                             mut denom: std::os::raw::c_ulonglong,
                                             mut always_percent: std::os::raw::c_int)
 -> std::os::raw::c_int {
    /* (1) num/denom == 0/0                 ==> print "??%"
     * (2) num/denom == INFINITY            ==> print "INFINITY%"
     * (3) 0 <= num/denom < 99.995%         ==> use the percent format "99.99%"
     *     if always_percent:
     * (4)    0.995 <= num/denom < INFINITY ==> use the percent format "999%"
     *     else:
     * (5)    0.995 <= num/denom < 99.995   ==> use the factor format "9.99x"
     * (6)    99.5 <= num/denom < INFINITY  ==> use the factor format "999x"
     *     end if
     */
    let mut integer_part: u64 = 0;
    let mut remainder: u64 = 0;
    let mut fractional_part: u32 = 0;
    let mut scale: u32 = 0;
    let mut scaled_ratio: f64 = 0.;
    /* (1,2): num/denom == 0/0 or num/denom == INFINITY */
    if denom == 0 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        return opng_snprintf_impl(buffer, buffer_size,
                                  if num ==
                                         0 as std::os::raw::c_int as std::os::raw::c_ulonglong
                                     {
                                      b"??%%\x00" as *const u8 as
                                          *const std::os::raw::c_char
                                  } else {
                                      b"INFINITY%%\x00" as *const u8 as
                                          *const std::os::raw::c_char
                                  })
    }
    /* (3): 0 <= num/denom < 99.995% */
    /* num/denom < 99.995% <==> denom/(denom-num) < 20000 */
    if num < denom &&
           denom.wrapping_div(denom.wrapping_sub(num)) <
               20000 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        scale = 10000 as std::os::raw::c_int as std::os::raw::c_uint;
        scaled_ratio =
            num as std::os::raw::c_double * scale as std::os::raw::c_double /
                denom as std::os::raw::c_double;
        fractional_part = (scaled_ratio + 0.5f64) as std::os::raw::c_uint;
        /* Adjust the scaled result in the event of a roundoff error. */
        /* Such error may occur only if the numerator is extremely large. */
        if fractional_part >= scale {
            fractional_part =
                scale.wrapping_sub(1 as std::os::raw::c_int as std::os::raw::c_uint)
        }
        return opng_snprintf_impl(buffer, buffer_size,
                                  b"%u.%02u%%\x00" as *const u8 as
                                      *const std::os::raw::c_char,
                                  fractional_part.wrapping_div(100 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint),
                                  fractional_part.wrapping_rem(100 as
                                                                   std::os::raw::c_int
                                                                   as
                                                                   std::os::raw::c_uint))
    }
    /* Extract the integer part out of the fraction for the remaining cases. */
    integer_part = num.wrapping_div(denom);
    remainder = num.wrapping_rem(denom);
    scale = 100 as std::os::raw::c_int as std::os::raw::c_uint;
    scaled_ratio =
        remainder as std::os::raw::c_double * scale as std::os::raw::c_double /
            denom as std::os::raw::c_double;
    fractional_part = (scaled_ratio + 0.5f64) as std::os::raw::c_uint;
    if fractional_part >= scale {
        fractional_part = 0 as std::os::raw::c_int as std::os::raw::c_uint;
        integer_part = integer_part.wrapping_add(1)
    }
    /* (4): 0.995 <= num/denom < INFINITY */
    if always_percent != 0 {
        return opng_snprintf_impl(buffer, buffer_size,
                                  b"%llu%02u%%\x00" as *const u8 as
                                      *const std::os::raw::c_char, integer_part,
                                  fractional_part)
    }
    /* (5): 0.995 <= num/denom < 99.995 */
    if integer_part < 100 as std::os::raw::c_int as std::os::raw::c_ulonglong {
        return opng_snprintf_impl(buffer, buffer_size,
                                  b"%llu.%02ux\x00" as *const u8 as
                                      *const std::os::raw::c_char, integer_part,
                                  fractional_part)
    }
    /* (6): 99.5 <= num/denom < INFINITY */
    /* Round to the nearest integer. */
    /* Recalculate the integer part, for corner cases like 123.999. */
    integer_part = num.wrapping_div(denom);
    if remainder >
           denom.wrapping_sub(1 as std::os::raw::c_int as
                                  std::os::raw::c_ulonglong).wrapping_div(2 as
                                                                      std::os::raw::c_int
                                                                      as
                                                                      std::os::raw::c_ulonglong)
       {
        integer_part = integer_part.wrapping_add(1)
    }
    return opng_snprintf_impl(buffer, buffer_size,
                              b"%llux\x00" as *const u8 as
                                  *const std::os::raw::c_char, integer_part);
}
/*
 * Converts a rational value to a compact factor string representation.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_ulratio_to_factor_string<'a1>(mut buffer:
                                                           * mut std::os::raw::c_char,
                                                       mut buffer_size:
                                                           std::os::raw::c_ulong,
                                                       mut ratio:
                                                           Option<&'a1 crate::src::optipng::optim::opng_ulratio>)
 -> std::os::raw::c_int {
    let mut num: u64 = (*(ratio).unwrap()).num as opng_uxlong_impl_t;
    let mut denom: u64 = (*(ratio).unwrap()).denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom,
                                   0 as std::os::raw::c_int);
}
/*
 * Converts a rational value to a compact percent string representation.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_ulratio_to_percent_string<'a1>(mut buffer:
                                                            * mut std::os::raw::c_char,
                                                        mut buffer_size:
                                                            std::os::raw::c_ulong,
                                                        mut ratio:
                                                            Option<&'a1 crate::src::optipng::optim::opng_ulratio>)
 -> std::os::raw::c_int {
    let mut num: u64 = (*(ratio).unwrap()).num as opng_uxlong_impl_t;
    let mut denom: u64 = (*(ratio).unwrap()).denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom,
                                   1 as std::os::raw::c_int);
}
/*
 * Converts a rational value to a compact factor string representation.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_ullratio_to_factor_string<'a1>(mut buffer:
                                                            * mut std::os::raw::c_char,
                                                        mut buffer_size:
                                                            std::os::raw::c_ulong,
                                                        mut ratio:
                                                            Option<&'a1 crate::src::optipng::ratio::opng_ullratio>)
 -> std::os::raw::c_int {
    let mut num: u64 = (*(ratio).unwrap()).num;
    let mut denom: u64 = (*(ratio).unwrap()).denom;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom,
                                   0 as std::os::raw::c_int);
}
/*
 * Converts a rational value to a compact percent string representation.
 */
#[no_mangle]
pub unsafe extern "C" fn opng_ullratio_to_percent_string<'a1>(mut buffer:
                                                             * mut std::os::raw::c_char,
                                                         mut buffer_size:
                                                             std::os::raw::c_ulong,
                                                         mut ratio:
                                                             Option<&'a1 crate::src::optipng::ratio::opng_ullratio>)
 -> std::os::raw::c_int {
    let mut num: u64 = (*(ratio).unwrap()).num;
    let mut denom: u64 = (*(ratio).unwrap()).denom;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom,
                                   1 as std::os::raw::c_int);
}
use crate::laertes_rt::*;
/* OPNG_LLONG_T_DEFINED */
