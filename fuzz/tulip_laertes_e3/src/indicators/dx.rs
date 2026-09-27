
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
pub unsafe extern "C" fn ti_dx_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
               1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_dx(mut size: std::os::raw::c_int,
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
    if size <= ti_dx_start(options) { return 0 as std::os::raw::c_int }
    let per: f64 =
        (period as std::os::raw::c_double - 1 as std::os::raw::c_int as std::os::raw::c_double) /
            period as std::os::raw::c_double;
    let mut atr: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dmup: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut dmdown: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i < period {
        let mut truerange: f64 = 0.;
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
        atr += truerange;
        let mut dp: f64 = 0.;
        let mut dm: f64 = 0.;
        dp =
            *high.offset(i as isize) -
                *high.offset((i - 1 as std::os::raw::c_int) as isize);
        dm =
            *low.offset((i - 1 as std::os::raw::c_int) as isize) -
                *low.offset(i as isize);
        if dp < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dp = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dp > dm { dm = 0 as std::os::raw::c_int as std::os::raw::c_double }
        if dm < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dm = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dm > dp { dp = 0 as std::os::raw::c_int as std::os::raw::c_double }
        dmup += dp;
        dmdown += dm;
        i += 1
    }
    let mut di_up: f64 = dmup / atr;
    let mut di_down: f64 = dmdown / atr;
    let mut dm_diff: f64 = fabs(di_up - di_down);
    let mut dm_sum: f64 = di_up + di_down;
    let mut dx: f64 =
        dm_diff / dm_sum * 100 as std::os::raw::c_int as std::os::raw::c_double;
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = dx;
    i = period;
    while i < size {
        let mut truerange_0: f64 = 0.;
        let l_0: f64 = *low.offset(i as isize);
        let h_0: f64 = *high.offset(i as isize);
        let c_0: f64 =
            *close.offset((i - 1 as std::os::raw::c_int) as isize);
        let ych_0: f64 = fabs(h_0 - c_0);
        let ycl_0: f64 = fabs(l_0 - c_0);
        let mut v_0: f64 = h_0 - l_0;
        if ych_0 > v_0 { v_0 = ych_0 }
        if ycl_0 > v_0 { v_0 = ycl_0 }
        truerange_0 = v_0;
        atr = atr * per + truerange_0;
        let mut dp_0: f64 = 0.;
        let mut dm_0: f64 = 0.;
        dp_0 =
            *high.offset(i as isize) -
                *high.offset((i - 1 as std::os::raw::c_int) as isize);
        dm_0 =
            *low.offset((i - 1 as std::os::raw::c_int) as isize) -
                *low.offset(i as isize);
        if dp_0 < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dp_0 = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dp_0 > dm_0 { dm_0 = 0 as std::os::raw::c_int as std::os::raw::c_double }
        if dm_0 < 0 as std::os::raw::c_int as std::os::raw::c_double {
            dm_0 = 0 as std::os::raw::c_int as std::os::raw::c_double
        } else if dm_0 > dp_0 { dp_0 = 0 as std::os::raw::c_int as std::os::raw::c_double }
        dmup = dmup * per + dp_0;
        dmdown = dmdown * per + dm_0;
        let mut di_up_0: f64 = dmup / atr;
        let mut di_down_0: f64 = dmdown / atr;
        let mut dm_diff_0: f64 = fabs(di_up_0 - di_down_0);
        let mut dm_sum_0: f64 = di_up_0 + di_down_0;
        let mut dx_0: f64 =
            dm_diff_0 / dm_sum_0 * 100 as std::os::raw::c_int as std::os::raw::c_double;
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = dx_0;
        i += 1
    }
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_dx_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 6], &'_ [i8; 6]>(b"ti_dx\x00")).as_ptr(),
                     b"indicators/dx.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 101 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_dx_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
