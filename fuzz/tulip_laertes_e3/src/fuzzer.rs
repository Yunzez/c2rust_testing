
extern "C" {
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn __assert_rtn(_: * const std::os::raw::c_char, _: * const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: * const std::os::raw::c_char) -> !;
    
    
    
    
    
    fn printf(_: * const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators_index::ti_indicators;
pub use crate::indicators_index::ti_find_indicator;
pub type ti_indicator_start_function<'a1> = Option<unsafe extern "C"  fn(_: Option<&'a1 std::os::raw::c_double>,) -> std::os::raw::c_int>;
pub type ti_indicator_function<'a1, 'a2, 'a3, 'a4, 'a5> = Option<unsafe extern "C"  fn(_: std::os::raw::c_int,_: Option<&'a1 Option<&'a2 std::os::raw::c_double>>,_: Option<&'a3 std::os::raw::c_double>,_: Option<&'a4 Option<&'a5 mut std::os::raw::c_double>>,) -> std::os::raw::c_int>;
// #[derive(Copy, Clone)]

pub type ti_indicator_info = crate::example2::ti_indicator_info;
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
pub static mut optionsd: [std::os::raw::c_double; 29] =
    [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_optionsd() {
optionsd = [-(20 as std::os::raw::c_int) as std::os::raw::c_double,
     -(1 as std::os::raw::c_int) as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0.1f64, 0.5f64, 0.7f64,
     1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     2.5f64, 3 as std::os::raw::c_int as std::os::raw::c_double,
     4 as std::os::raw::c_int as std::os::raw::c_double, 5 as std::os::raw::c_int as std::os::raw::c_double,
     6 as std::os::raw::c_int as std::os::raw::c_double, 7 as std::os::raw::c_int as std::os::raw::c_double,
     8 as std::os::raw::c_int as std::os::raw::c_double, 9 as std::os::raw::c_int as std::os::raw::c_double,
     10 as std::os::raw::c_int as std::os::raw::c_double, 11 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double, 20 as std::os::raw::c_int as std::os::raw::c_double,
     21 as std::os::raw::c_int as std::os::raw::c_double, 22 as std::os::raw::c_int as std::os::raw::c_double,
     23 as std::os::raw::c_int as std::os::raw::c_double, 24 as std::os::raw::c_int as std::os::raw::c_double,
     25 as std::os::raw::c_int as std::os::raw::c_double, 26 as std::os::raw::c_int as std::os::raw::c_double,
     100 as std::os::raw::c_int as std::os::raw::c_double];}//;
#[no_mangle]
pub static mut dummy_in: [std::os::raw::c_double; 20] =
    [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_dummy_in() {
dummy_in = [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];}//;
#[no_mangle]
pub static mut dummy_in0: [std::os::raw::c_double; 20] =
    [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_dummy_in0() {
dummy_in0 = [0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double];}//;
#[no_mangle]
pub static mut dummy_ot: [std::os::raw::c_double; 20] =
    [0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0,]; unsafe fn laertes_init_dummy_ot() {
dummy_ot = [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];}//;
#[no_mangle]
pub unsafe extern "C" fn banner() {
    printf(b"  ______ _    _ __________________ _____  \n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" |  ____| |  | |___  /___  /  ____|  __ \\ \n\x00" as *const u8
               as *const std::os::raw::c_char);
    printf(b" | |__  | |  | |  / /   / /| |__  | |__) |\n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" |  __| | |  | | / /   / / |  __| |  _  / \n\x00" as *const u8 as
               *const std::os::raw::c_char);
    printf(b" | |    | |__| |/ /__ / /__| |____| | \\ \\ \n\x00" as *const u8
               as *const std::os::raw::c_char);
    printf(b" |_|     \\____//_____/_____|______|_|  \\_\\\n\n\x00" as
               *const u8 as *const std::os::raw::c_char);
    printf(b"This program tries each indicator with a lot of options.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    printf(b"It\'s good for checking for crashes or assertion failures.\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    printf(b"Use with a memory tool to check for memory errors in ti.\n\n\x00"
               as *const u8 as *const std::os::raw::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn check_output(mut info: * const crate::example2::ti_indicator_info,
                                      mut size: std::os::raw::c_int,
                                      mut inputs:
                                          * const * const std::os::raw::c_double,
                                      mut options: * const std::os::raw::c_double,
                                      mut outputs:
                                          * const * mut std::os::raw::c_double) {
    let mut s: i32 = 0;
    s = (*info).start.expect("non-null function pointer")(options);
    let mut o: i32 = 0;
    o = 0 as std::os::raw::c_int;
    while o < (*info).outputs {
        let mut max: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut min: f64 = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: i32 = 0;
        i = 0 as std::os::raw::c_int;
        while i < size - s {
            let op: f64 =
                *(*outputs.offset(o as isize)).offset(i as isize);
            let in_0: f64 =
                *(*inputs.offset(0 as std::os::raw::c_int as
                                     isize)).offset((i + s) as isize);
            max = if in_0 > max { in_0 } else { max };
            min = if in_0 < min { in_0 } else { min };
            match (*info).type_0 {
                1 => {
                    if op > max * 1.5f64 + 2 as std::os::raw::c_int as std::os::raw::c_double
                           ||
                           op <
                               min * 0.5f64 -
                                   2 as std::os::raw::c_int as std::os::raw::c_double {
                        let mut k: i32 = 0;
                        let mut j: i32 = 0;
                        printf(b"\nInputs:\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                        k = 0 as std::os::raw::c_int;
                        while k < size {
                            printf(b" %f\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   *(*inputs.offset(0 as std::os::raw::c_int as
                                                        isize)).offset(k as
                                                                           isize));
                            k += 1
                        }
                        printf(b"\nOptions:\x00" as *const u8 as
                                   *const std::os::raw::c_char);
                        k = 0 as std::os::raw::c_int;
                        while k < (*info).options {
                            printf(b" %f\x00" as *const u8 as
                                       *const std::os::raw::c_char,
                                   *options.offset(k as isize));
                            k += 1
                        }
                        j = 0 as std::os::raw::c_int;
                        while j < (*info).outputs {
                            printf(b"\nOutput[%d]:\x00" as *const u8 as
                                       *const std::os::raw::c_char, j);
                            k = 0 as std::os::raw::c_int;
                            while k < size {
                                printf(b" %f\x00" as *const u8 as
                                           *const std::os::raw::c_char,
                                       *(*outputs.offset(j as
                                                             isize)).offset(k
                                                                                as
                                                                                isize));
                                k += 1
                            }
                            j += 1
                        }
                        printf(b"\nERROR Output is out of range for input: input: %f output: %f\n\x00"
                                   as *const u8 as *const std::os::raw::c_char, in_0,
                               op);
                        if (0 as std::os::raw::c_int == 0) as std::os::raw::c_int as
                               std::os::raw::c_long != 0 {
                            __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 13], &'_ [i8; 13]>(b"check_output\x00")).as_ptr(),
                                         b"fuzzer.c\x00" as *const u8 as
                                             *const std::os::raw::c_char,
                                         94 as std::os::raw::c_int,
                                         b"0\x00" as *const u8 as
                                             *const std::os::raw::c_char);
                        } else { };
                    }
                }
                _ => { }
            }
            i += 1
        }
        o += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn stress(mut info: * const crate::example2::ti_indicator_info) {
    let opt_count: i32 = (*info).options;
    printf(b"%s (%s) (%d options)\n\x00" as *const u8 as *const std::os::raw::c_char,
           (*info).name, (*info).full_name, opt_count);
    let mut inputs: [* const f64; 10] =
        [0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double];
    let mut inputs0: [* const f64; 10] =
        [0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double,
         0 as *const std::os::raw::c_double, 0 as *const std::os::raw::c_double];
    let mut outputs: [* mut f64; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut i: i32 = 0;
    i = 0 as std::os::raw::c_int;
    while i < 10 as std::os::raw::c_int {
        inputs[i as usize] =
            if i < (*info).inputs {
                dummy_in.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        inputs0[i as usize] =
            if i < (*info).inputs {
                dummy_in0.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        outputs[i as usize] =
            if i < (*info).outputs {
                dummy_ot.as_mut_ptr()
            } else { 0 as *mut std::os::raw::c_double };
        i += 1
    }
    let mut options_index: [i32; 11] =
        [0 as std::os::raw::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut options: [f64; 10] = [0.; 10];
    let mut choices: i32 =
        (::std::mem::size_of::<[std::os::raw::c_double; 29]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut size: i32 =
        (::std::mem::size_of::<[std::os::raw::c_double; 20]>() as
             std::os::raw::c_ulong).wrapping_div(::std::mem::size_of::<std::os::raw::c_double>()
                                             as std::os::raw::c_ulong) as std::os::raw::c_int;
    let mut j: i32 = 0;
    let mut count: i32 = 0 as std::os::raw::c_int;
    loop  {
        count += 1;
        j = 0 as std::os::raw::c_int;
        while j < opt_count {
            options[j as usize] =
                optionsd[options_index[j as usize] as usize];
            printf(b" %f\x00" as *const u8 as *const std::os::raw::c_char,
                   options[j as usize]);
            j += 1
        }
        let mut r: i32 = 0;
        printf(b" 0\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(0 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 0 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 1\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(1 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 1 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 2\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(2 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 2 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 3\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(3 as
                                                                      std::os::raw::c_int,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, 3 as std::os::raw::c_int, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" %d\x00" as *const u8 as *const std::os::raw::c_char, size);
        r =
            (*info).indicator.expect("non-null function pointer")(size,
                                                                  inputs.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, size, inputs.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b" 0s\x00" as *const u8 as *const std::os::raw::c_char);
        r =
            (*info).indicator.expect("non-null function pointer")(size,
                                                                  inputs0.as_mut_ptr(),
                                                                  options.as_mut_ptr(),
                                                                  outputs.as_mut_ptr());
        if r == 0 as std::os::raw::c_int {
            check_output(info, size, inputs0.as_mut_ptr(),
                         options.as_mut_ptr(), outputs.as_mut_ptr());
        }
        printf(b"\r                                                  \r\x00"
                   as *const u8 as *const std::os::raw::c_char);
        j = 0 as std::os::raw::c_int;
        loop  {
            options_index[j as usize] =
                (options_index[j as usize] + 1 as std::os::raw::c_int) % choices;
            let mut fresh0 = j;
            j = j + 1;
            if !(options_index[fresh0 as usize] == 0 as std::os::raw::c_int) {
                break ;
            }
        }
        if !(j <= opt_count) { break ; }
    }
    if !(0.1f64 >
             fabs(count as std::os::raw::c_double -
                      pow(choices as std::os::raw::c_double,
                          opt_count as std::os::raw::c_double))) as std::os::raw::c_int as
           std::os::raw::c_long != 0 {
        __assert_rtn((*core::intrinsics::transmute::<&'_ [u8; 7], &'_ [i8; 7]>(b"stress\x00")).as_ptr(),
                     b"fuzzer.c\x00" as *const u8 as *const std::os::raw::c_char,
                     154 as std::os::raw::c_int,
                     b".1 > fabs(count - (pow(choices, opt_count)))\x00" as
                         *const u8 as *const std::os::raw::c_char);
    } else { };
}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: * mut * mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    banner();
    if argc > 1 as std::os::raw::c_int {
        let mut info: * const crate::example2::ti_indicator_info =
            ti_find_indicator(*argv.offset(1 as std::os::raw::c_int as isize));
        if !info.is_null() { stress(info); }
    } else {
        let mut info_0: * const crate::example2::ti_indicator_info = ti_indicators.as_mut_ptr();
        loop  {
            stress(info_0);
            info_0 = info_0.offset(1);
            if (*info_0).name.is_null() { break ; }
        }
    }
    printf(b"\r                                                  \rDone\n\x00"
               as *const u8 as *const std::os::raw::c_char);
    return 0 as std::os::raw::c_int;
}
pub fn main() {
    let mut args: Vec<*mut std::os::raw::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(::std::ffi::CString::new(arg).expect("Failed to convert argument into CString.").into_raw());
    };
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0((args.len() - 1) as std::os::raw::c_int,
                                    args.as_mut_ptr() as
                                        *mut *mut std::os::raw::c_char) as i32)
    }
}
use crate::laertes_rt::*;
