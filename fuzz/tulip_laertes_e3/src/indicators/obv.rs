
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
pub extern "C" fn ti_obv_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return 0 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ti_obv(mut size: std::os::raw::c_int,
                                mut inputs: * const * const std::os::raw::c_double,
                                mut options: * const std::os::raw::c_double,
                                mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut close: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut volume: * const f64 =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    let mut sum: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut fresh0 = output;
    output = output.offset(1);
    *fresh0 = sum;
    let mut prev: f64 = *close.offset(0 as std::os::raw::c_int as isize);
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i < size {
        if *close.offset(i as isize) > prev {
            sum += *volume.offset(i as isize)
        } else if *close.offset(i as isize) < prev {
            sum -= *volume.offset(i as isize)
        }
        prev = *close.offset(i as isize);
        let mut fresh1 = output;
        output = output.offset(1);
        *fresh1 = sum;
        i += 1
    }
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
