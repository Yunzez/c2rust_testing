
extern "C" {
    
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility_start(mut options:
                                                 * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as
               std::os::raw::c_int; /* Multiplier, number of trading days in year. */
}
#[no_mangle]
pub unsafe extern "C" fn ti_volatility(mut size: std::os::raw::c_int,
                                       mut inputs:
                                           * const * const std::os::raw::c_double,
                                       mut options: * const std::os::raw::c_double,
                                       mut outputs:
                                           * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let scale: f64 = 1.0f64 / period as std::os::raw::c_double;
    let annual: f64 = sqrt(252 as std::os::raw::c_int as std::os::raw::c_double);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_volatility_start(options) { return 0 as std::os::raw::c_int }
    let mut sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum2: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i <= period {
        let c: f64 =
            *input.offset(i as isize) /
                *input.offset((i - 1 as std::os::raw::c_int) as isize) - 1.0f64;
        sum += c;
        sum2 += c * c;
        i += 1
    }
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
    i = period + 1 as std::os::raw::c_int;
    while i < size {
        let c_0: f64 =
            *input.offset(i as isize) /
                *input.offset((i - 1 as std::os::raw::c_int) as isize) - 1.0f64;
        sum += c_0;
        sum2 += c_0 * c_0;
        let cp: f64 =
            *input.offset((i - period) as isize) /
                *input.offset((i - period - 1 as std::os::raw::c_int) as isize) -
                1.0f64;
        sum -= cp;
        sum2 -= cp * cp;
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = sqrt(sum2 * scale - sum * scale * (sum * scale)) * annual;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_volatility_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 14], &'_ [i8; 14]>(b"ti_volatility\x00")).as_ptr(),
                     b"indicators/volatility.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 69 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_volatility_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
