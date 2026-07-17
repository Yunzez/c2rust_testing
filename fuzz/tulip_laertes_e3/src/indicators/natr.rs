
extern "C" {
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
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
pub unsafe extern "C" fn ti_natr_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_natr(mut size: std::os::raw::c_int,
                                 mut inputs: * const * const std::os::raw::c_double,
                                 mut options: * const std::os::raw::c_double,
                                 mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: * const f64 =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut close: * const f64 =
        *inputs.offset(2 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_natr_start(options) { return 0 as std::os::raw::c_int }
    let per: f64 = 1.0f64 / period as std::os::raw::c_double;
    let mut sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut truerange: f64 = 0.;
    sum +=
        *high.offset(0 as std::os::raw::c_int as isize) -
            *low.offset(0 as std::os::raw::c_int as isize);
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i < period {
        let l: f64 = *low.offset(i as isize);
        let h: f64 = *high.offset(i as isize);
        let c: f64 =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych: f64 = fabs(h - c);
        let ycl: f64 = fabs(l - c);
        let mut v: f64 = h - l;
        if ych > v { v = ych }
        if ycl > v { v = ycl }
        truerange = v;
        sum += truerange;
        i += 1
    }
    let mut val: f64 = sum / period as std::os::raw::c_double;
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 =
        100 as std::os::raw::c_int as std::os::raw::c_double * val /
            *close.offset((period - 1 as std::os::raw::c_int) as isize);
    i = period;
    while i < size {
        let l_0: f64 = *low.offset(i as isize);
        let h_0: f64 = *high.offset(i as isize);
        let c_0: f64 =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych_0: f64 = fabs(h_0 - c_0);
        let ycl_0: f64 = fabs(l_0 - c_0);
        let mut v_0: f64 = h_0 - l_0;
        if ych_0 > v_0 { v_0 = ych_0 }
        if ycl_0 > v_0 { v_0 = ycl_0 }
        truerange = v_0;
        val = (truerange - val) * per + val;
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 =
            100 as std::os::raw::c_int as std::os::raw::c_double * val /
                *close.offset(i as isize);
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_natr_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 8], &'_ [i8; 8]>(b"ti_natr\x00")).as_ptr(),
                     b"indicators/natr.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 69 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_natr_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
