
extern "C" {
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
    
    
    
    
}
pub use crate::utils::buffer::ti_buffer_free;
pub use crate::utils::buffer::ti_buffer_new;
// #[derive(Copy, Clone)]

pub type ti_buffer = crate::indicators::adxr::ti_buffer;
/*
 * Tulip Indicators
 * https://tulipindicators.org/
 * Copyright (c) 2010-2018 Tulip Charts LLC
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
pub unsafe extern "C" fn ti_stochrsi_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    return *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int *
               2 as std::os::raw::c_int -
               1 as std::os::raw::c_int; /* If period = 0 then min-max = 0. */
}
#[no_mangle]
pub unsafe extern "C" fn ti_stochrsi(mut size: std::os::raw::c_int,
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
    let per: f64 = 1.0f64 / period as std::os::raw::c_double;
    if period < 2 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_stochrsi_start(options) { return 0 as std::os::raw::c_int }
    let mut rsi: * mut crate::indicators::adxr::ti_buffer = ti_buffer_new(period);
    let mut smooth_up: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut smooth_down: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
    let mut i: i32 = 0;
    i = 1 as std::os::raw::c_int;
    while i <= period {
        let upward: f64 =
            if *input.offset(i as isize) >
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset(i as isize)) -
                    *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        let downward: f64 =
            if *input.offset(i as isize) <
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) -
                    *input.offset(i as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        smooth_up += upward;
        smooth_down += downward;
        i += 1
    }
    smooth_up /= period as std::os::raw::c_double;
    smooth_down /= period as std::os::raw::c_double;
    let mut r: f64 =
        100.0f64 * (smooth_up / (smooth_up + smooth_down));
    if (*rsi).pushes >= (*rsi).size {
        (*rsi).sum -= *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize)
    }
    (*rsi).sum += r;
    *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize) = r;
    (*rsi).pushes += 1 as std::os::raw::c_int;
    (*rsi).index = (*rsi).index + 1 as std::os::raw::c_int;
    if (*rsi).index >= (*rsi).size { (*rsi).index = 0 as std::os::raw::c_int }
    let mut min: f64 = r;
    let mut max: f64 = r;
    let mut mini: i32 = 0 as std::os::raw::c_int;
    let mut maxi: i32 = 0 as std::os::raw::c_int;
    i = period + 1 as std::os::raw::c_int;
    while i < size {
        let upward_0: f64 =
            if *input.offset(i as isize) >
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset(i as isize)) -
                    *input.offset((i - 1 as std::os::raw::c_int) as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        let downward_0: f64 =
            if *input.offset(i as isize) <
                   *input.offset((i - 1 as std::os::raw::c_int) as isize) {
                (*input.offset((i - 1 as std::os::raw::c_int) as isize)) -
                    *input.offset(i as isize)
            } else { 0 as std::os::raw::c_int as std::os::raw::c_double };
        smooth_up = (upward_0 - smooth_up) * per + smooth_up;
        smooth_down = (downward_0 - smooth_down) * per + smooth_down;
        r = 100.0f64 * (smooth_up / (smooth_up + smooth_down));
        if r > max {
            max = r;
            maxi = (*rsi).index
        } else if maxi == (*rsi).index {
            max = r;
            let mut j: i32 = 0;
            j = 0 as std::os::raw::c_int;
            while j < (*rsi).size {
                if !(j == (*rsi).index) {
                    if *(*rsi).vals.as_mut_ptr().offset(j as isize) > max {
                        max = *(*rsi).vals.as_mut_ptr().offset(j as isize);
                        maxi = j
                    }
                }
                j += 1
            }
        }
        if r < min {
            min = r;
            mini = (*rsi).index
        } else if mini == (*rsi).index {
            min = r;
            let mut j_0: i32 = 0;
            j_0 = 0 as std::os::raw::c_int;
            while j_0 < (*rsi).size {
                if !(j_0 == (*rsi).index) {
                    if *(*rsi).vals.as_mut_ptr().offset(j_0 as isize) < min {
                        min = *(*rsi).vals.as_mut_ptr().offset(j_0 as isize);
                        mini = j_0
                    }
                }
                j_0 += 1
            }
        }
        *(*rsi).vals.as_mut_ptr().offset((*rsi).index as isize) = r;
        (*rsi).index = (*rsi).index + 1 as std::os::raw::c_int;
        if (*rsi).index >= (*rsi).size { (*rsi).index = 0 as std::os::raw::c_int }
        if i > period * 2 as std::os::raw::c_int - 2 as std::os::raw::c_int {
            let diff: f64 = max - min;
            if diff == 0.0f64 {
                let mut fresh0 = output;
                output = output.offset(1);
                *fresh0 = 0.0f64
            } else {
                let mut fresh1 = output;
                output = output.offset(1);
                *fresh1 = (r - min) / diff
            }
        }
        i += 1
    }
    ti_buffer_free(rsi);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long ==
             (size - ti_stochrsi_start(options)) as std::os::raw::c_long) as
           std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 12], &'_ [i8; 12]>(b"ti_stochrsi\x00")).as_ptr(),
                     b"indicators/stochrsi.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 115 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_stochrsi_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
