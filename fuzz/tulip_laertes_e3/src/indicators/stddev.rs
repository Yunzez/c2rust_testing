
extern "C" {
    
    fn sqrt(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
}
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2017 Tulip Charts LLC
 * Lewis Van Winkle (LV@tulipcharts.org)
 *
 * This file is part of Tulip Indicators.
 *
 * Tulip Indicators is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * Tulip Indicators is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License
 * for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with Tulip Indicators.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn ti_stddev_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_stddev(mut size: std::os::raw::c_int,
                                   mut inputs: * const * const std::os::raw::c_double,
                                   mut options: * const std::os::raw::c_double,
                                   mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut input: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let scale: f64 = 1.0f64 / period as std::os::raw::c_double;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_stddev_start(options) { return 0 as std::os::raw::c_int }
    let mut sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut sum2: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < period {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        i += 1
    }
    let mut s2s2: f64 = sum2 * scale - sum * scale * (sum * scale);
    if s2s2 > 0.0f64 { s2s2 = sqrt(s2s2) }
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = s2s2;
    i = period;
    while i < size {
        sum += *input.offset(i as isize);
        sum2 += *input.offset(i as isize) * *input.offset(i as isize);
        sum -= *input.offset((i - period) as isize);
        sum2 -=
            *input.offset((i - period) as isize) *
                *input.offset((i - period) as isize);
        let mut s2s2_0: f64 =
            sum2 * scale - sum * scale * (sum * scale);
        if s2s2_0 > 0.0f64 { s2s2_0 = sqrt(s2s2_0) }
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = s2s2_0;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_stddev_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 10], &'_ [i8; 10]>(b"ti_stddev\x00")).as_ptr(),
                     b"indicators/stddev.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 68 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_stddev_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
