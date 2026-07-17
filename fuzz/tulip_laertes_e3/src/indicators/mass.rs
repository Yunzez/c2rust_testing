
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
pub unsafe extern "C" fn ti_mass_start(mut options: * const std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut sum_p: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int -
            1 as std::os::raw::c_int;
    /* The ema uses a hard-coded period of 9.
     * (9-1)*2 = 16 */
    return 16 as std::os::raw::c_int + sum_p;
}
#[no_mangle]
pub unsafe extern "C" fn ti_mass(mut size: std::os::raw::c_int,
                                 mut inputs: * const * const std::os::raw::c_double,
                                 mut options: * const std::os::raw::c_double,
                                 mut outputs: * const * mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut high: * const f64 =
        *inputs.offset(0 as std::os::raw::c_int as isize);
    let mut low: * const f64 =
        *inputs.offset(1 as std::os::raw::c_int as isize);
    let period: i32 =
        *options.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int;
    let mut output: * mut f64 =
        *outputs.offset(0 as std::os::raw::c_int as isize);
    if period < 1 as std::os::raw::c_int { return 1 as std::os::raw::c_int }
    if size <= ti_mass_start(options) { return 0 as std::os::raw::c_int }
    /*mass uses a hard-coded 9 period for the ema*/
    let per: f64 =
        2 as std::os::raw::c_int as std::os::raw::c_double /
            (9.0f64 + 1 as std::os::raw::c_int as std::os::raw::c_double);
    let per1: f64 = 1.0f64 - per;
    /*Calculate EMA(h-l)*/
    let mut ema: f64 =
        *high.offset(0 as std::os::raw::c_int as isize) -
            *low.offset(0 as std::os::raw::c_int as isize);
    /*Calculate EMA(EMA(h-l))*/
    let mut ema2: f64 = ema;
    let mut sum: * mut crate::indicators::adxr::ti_buffer = ti_buffer_new(period);
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        let mut hl: f64 =
            *high.offset(i as isize) - *low.offset(i as isize);
        ema = ema * per1 + hl * per;
        if i == 8 as std::os::raw::c_int { ema2 = ema }
        if i >= 8 as std::os::raw::c_int {
            ema2 = ema2 * per1 + ema * per;
            if i >= 16 as std::os::raw::c_int {
                if (*sum).pushes >= (*sum).size {
                    (*sum).sum -=
                        *(*sum).vals.as_mut_ptr().offset((*sum).index as
                                                             isize)
                }
                (*sum).sum += ema / ema2;
                *(*sum).vals.as_mut_ptr().offset((*sum).index as isize) =
                    ema / ema2;
                (*sum).pushes += 1 as std::os::raw::c_int;
                (*sum).index = (*sum).index + 1 as std::os::raw::c_int;
                if (*sum).index >= (*sum).size {
                    (*sum).index = 0 as std::os::raw::c_int
                }
                if i >= 16 as std::os::raw::c_int + period - 1 as std::os::raw::c_int {
                    let mut fresh0 = output;
                    output = output.offset(1);
                    *fresh0 = (*sum).sum
                }
            }
        }
        i += 1
    }
    ti_buffer_free(sum);
    if !(output.offset_from(*outputs.offset(0 as std::os::raw::c_int as
                                                         isize)) as
             std::os::raw::c_long == (size - ti_mass_start(options)) as std::os::raw::c_long)
           as std::os::raw::c_int as std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 8], &'_ [i8; 8]>(b"ti_mass\x00")).as_ptr(),
                     b"indicators/mass.c\x00" as *const u8 as
                         *const std::os::raw::c_char, 80 as std::os::raw::c_int,
                     b"output - outputs[0] == size - ti_mass_start(options)\x00"
                         as *const u8 as *const std::os::raw::c_char);
    } else { };
    return 0 as std::os::raw::c_int;
}
use crate::laertes_rt::*;
