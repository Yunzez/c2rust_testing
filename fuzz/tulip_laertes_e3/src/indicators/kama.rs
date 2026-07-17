
extern "C" {
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
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
pub unsafe extern "C" fn ti_kama_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_kama(mut size: std::os::raw::c_int,
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
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_kama_start(options) { return 0 as std::os::raw::c_int }
    /* The caller selects the period used in the efficiency ratio.
     * The fast and slow periods are hard set by the algorithm. */
    let short_per: f64 =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (2.0f64 + 1 as std::os::raw::c_int as std::os::raw::c_double);
    let long_per: f64 =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (30.0f64 + 1 as std::os::raw::c_int as std::os::raw::c_double);
    let mut sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i < period {
        sum +=
            fabs(*input.offset(i as isize) -
                     *input.offset((i - 1 as std::os::raw::c_int) as isize));
        i += 1
    }
    let mut kama: f64 =
        *input.offset((period - 1 as std::os::raw::c_int) as isize);
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = kama;
    let mut er: f64 = 0.;
    let mut sc: f64 = 0.;
    i = period;
    while i < size {
        sum +=
            fabs(*input.offset(i as isize) -
                     *input.offset((i - 1 as std::os::raw::c_int) as isize));
        if i > period {
            sum -=
                fabs(*input.offset((i - period) as isize) -
                         *input.offset((i - period - 1 as std::os::raw::c_int) as
                                           isize))
        }
        if sum != 0.0f64 {
            er =
                fabs(*input.offset(i as isize) -
                         *input.offset((i - period) as isize)) / sum
        } else { er = 1.0f64 }
        sc =
            pow(er * (short_per - long_per) + long_per,
                2 as std::os::raw::c_int as std::os::raw::c_double);
        kama = kama + sc * (*input.offset(i as isize) - kama);
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = kama;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_kama_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 8], &'_ [i8; 8]>(b"ti_kama\x00")).as_ptr(),
                     b"indicators/kama.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 75 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_kama_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
