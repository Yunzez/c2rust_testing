
extern "C" {
    
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
pub unsafe extern "C" fn ti_aroon_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_aroon(mut size: std::os::raw::c_int,
                                  mut inputs: * const * const std::os::raw::c_double,
                                  mut options: * const std::os::raw::c_double,
                                  mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: * const f64 =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut adown: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut aup: * mut f64 =
        *outputs.offset(1 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_aroon_start(options) { return 0 as std::os::raw::c_int }
    let scale: f64 = 100.0f64 / period as std::os::raw::c_double;
    let mut trail: i32 = 0 as std::os::raw::c_int;
    let mut maxi: i32 = -(1 as std::os::raw::c_int);
    let mut mini: i32 = -(1 as std::os::raw::c_int);
    let mut max: f64 = *high.offset(0 as std::os::raw::c_int as isize);
    let mut min: f64 = *low.offset(0 as std::os::raw::c_int as isize);
    let mut bar: f64 = 0.;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = period;
    while i < size {
        /* Maintain highest. */
        bar = *high.offset(i as isize);
        if maxi < trail {
            maxi = trail;
            max = *high.offset(maxi as isize);
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar = *high.offset(j as isize);
                if bar >= max { max = bar; maxi = j }
            }
        } else if bar >= max { maxi = i; max = bar }
        /* Maintain lowest. */
        bar = *low.offset(i as isize);
        if mini < trail {
            mini = trail;
            min = *low.offset(mini as isize);
            j = trail;
            loop  {
                j += 1;
                if !(j <= i) { break ; }
                bar = *low.offset(j as isize);
                if bar <= min { min = bar; mini = j }
            }
        } else if bar <= min { mini = i; min = bar }
        /* Calculate the indicator. */
        let mut fresh0 = adown;
        adown = adown.offset(1);
        *fresh0 =
            (period as std::os::raw::c_double - (i - mini) as std::os::raw::c_double) * scale;
        let mut fresh1 = aup;
        aup = aup.offset(1);
        *fresh1 =
            (period as std::os::raw::c_double - (i - maxi) as std::os::raw::c_double) * scale;
        i += 1;
        trail += 1
    }
    if !(adown.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                        isize)) as
             std::os::raw::c_long == (size - ti_aroon_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"ti_aroon\x00")).as_ptr(),
                     b"indicators/aroon.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 96 as std::os::raw::c_int,
                     b"adown - outputs[0] == size - ti_aroon_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    if !(aup.offset_from(*outputs.offset(1 as std::os::raw::c_int as isize))
             as std::os::raw::c_long ==
             (size - ti_aroon_start(options)) as std::os::raw::c_long) as std::os::raw::c_int
           as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 9], &'_ [i8; 9]>(b"ti_aroon\x00")).as_ptr(),
                     b"indicators/aroon.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 97 as std::os::raw::c_int,
                     b"aup - outputs[1] == size - ti_aroon_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
