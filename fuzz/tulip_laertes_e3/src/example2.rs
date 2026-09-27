
extern "C" {
    
    
    
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators_index::ti_indicators;
pub type ti_indicator_start_function<'a1>
    =
    Option<unsafe extern "C"  fn(_: Option<&'a1 std::os::raw::c_double>,) -> std::os::raw::c_int>;
pub type ti_indicator_function<'a1, 'a2, 'a3, 'a4, 'a5>
    =
    Option<unsafe extern "C"  fn(_: std::os::raw::c_int,_: Option<&'a1 Option<&'a2 std::os::raw::c_double>>,_: Option<&'a3 std::os::raw::c_double>,_: Option<&'a4 Option<&'a5 mut std::os::raw::c_double>>,) -> std::os::raw::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ti_indicator_info {
    pub name: * mut std::os::raw::c_char,
    pub full_name: * mut std::os::raw::c_char,
    pub start: Option<unsafe extern "C"  fn(_: * const std::os::raw::c_double,) -> std::os::raw::c_int>,
    pub indicator: Option<unsafe extern "C"  fn(_: std::os::raw::c_int,_: * const * const std::os::raw::c_double,_: * const std::os::raw::c_double,_: * const * mut std::os::raw::c_double,) -> std::os::raw::c_int>,
    pub type_0: std::os::raw::c_int,
    pub inputs: std::os::raw::c_int,
    pub options: std::os::raw::c_int,
    pub outputs: std::os::raw::c_int,
    pub input_names: [* mut std::os::raw::c_char; 10],
    pub option_names: [* mut std::os::raw::c_char; 10],
    pub output_names: [* mut std::os::raw::c_char; 10],
}
impl ti_indicator_info {
    pub const fn new() -> Self {
        ti_indicator_info {
        name: (0 as * mut std::os::raw::c_char),
        full_name: (0 as * mut std::os::raw::c_char),
        start: None,
        indicator: None,
        type_0: 0,
        inputs: 0,
        options: 0,
        outputs: 0,
        input_names: [(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),],
        option_names: [(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),],
        output_names: [(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),(0 as * mut std::os::raw::c_char),]
        }
    }
}

impl std::default::Default for ti_indicator_info {
    fn default() -> Self { ti_indicator_info::new() }
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
unsafe fn main_0() -> std::os::raw::c_int {
    printf(b"This program is an example of looping through\n\x00" as *const u8
               as *const std::os::raw::c_char);
    printf(b"each of the available indicators.\n\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    /* Set info to first indicators in array. */
    let mut info: * const crate::example2::ti_indicator_info = ti_indicators.as_mut_ptr();
    /* The last item is all zeros, so we'll stop when we get there. */
    while !(*info).name.is_null() {
        let mut i: i32 = 0;
        printf(b"%s (%s) has type %d with: %d inputs, %d options, %d outputs.\n\x00"
                   as *const u8 as *const std::os::raw::c_char, (*info).name,
               (*info).full_name, (*info).type_0, (*info).inputs,
               (*info).options, (*info).outputs);
        printf(b"   inputs: \x00" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < (*info).inputs {
            printf(b"%s%s\x00" as *const u8 as *const std::os::raw::c_char,
                   if i != 0 {
                       b", \x00" as *const u8 as *const std::os::raw::c_char
                   } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                   (*info).input_names[i as usize]);
            i += 1
        }
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        printf(b"   options: \x00" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < (*info).options {
            printf(b"%s%s\x00" as *const u8 as *const std::os::raw::c_char,
                   if i != 0 {
                       b", \x00" as *const u8 as *const std::os::raw::c_char
                   } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                   (*info).option_names[i as usize]);
            i += 1
        }
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        printf(b"   outputs: \x00" as *const u8 as *const std::os::raw::c_char);
        i = 0 as std::os::raw::c_int;
        while i < (*info).outputs {
            printf(b"%s%s\x00" as *const u8 as *const std::os::raw::c_char,
                   if i != 0 {
                       b", \x00" as *const u8 as *const std::os::raw::c_char
                   } else { b"\x00" as *const u8 as *const std::os::raw::c_char },
                   (*info).output_names[i as usize]);
            i += 1
        }
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        /* Next indicator. */
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        info = info.offset(1)
    }
    return 0 as std::os::raw::c_int;
}
pub fn main() { unsafe { ::std::process::exit(main_0() as i32) } }
use crate::laertes_rt::*;
/* To use this indicator, call the start function like:
         * info->start(options);
         * and then call the actual indicator function like:
         * info->indicator(size, inputs, options, outputs);
         */
