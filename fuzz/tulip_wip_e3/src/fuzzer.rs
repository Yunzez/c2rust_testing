


use std::ffi::CString;

use std::convert::TryInto;

extern "C" {
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn pow(_: std::os::raw::c_double, _: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators_index::ti_indicators;
pub use crate::indicators_index::ti_find_indicator;
pub type ti_indicator_start_function = crate::example2::ti_indicator_start_function;
pub type ti_indicator_function = crate::example2::ti_indicator_function;
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
    [-(20 as std::os::raw::c_int) as std::os::raw::c_double,
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
     100 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_in: [std::os::raw::c_double; 20] =
    [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_in0: [std::os::raw::c_double; 20] =
    [0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double,
     0 as std::os::raw::c_int as std::os::raw::c_double, 0 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut dummy_ot: [std::os::raw::c_double; 20] =
    [1 as std::os::raw::c_int as std::os::raw::c_double, 2 as std::os::raw::c_int as std::os::raw::c_double,
     3 as std::os::raw::c_int as std::os::raw::c_double, 4 as std::os::raw::c_int as std::os::raw::c_double,
     5 as std::os::raw::c_int as std::os::raw::c_double, 6 as std::os::raw::c_int as std::os::raw::c_double,
     7 as std::os::raw::c_int as std::os::raw::c_double, 8 as std::os::raw::c_int as std::os::raw::c_double,
     9 as std::os::raw::c_int as std::os::raw::c_double, 10 as std::os::raw::c_int as std::os::raw::c_double,
     11 as std::os::raw::c_int as std::os::raw::c_double, 12 as std::os::raw::c_int as std::os::raw::c_double,
     13 as std::os::raw::c_int as std::os::raw::c_double, 14 as std::os::raw::c_int as std::os::raw::c_double,
     15 as std::os::raw::c_int as std::os::raw::c_double, 16 as std::os::raw::c_int as std::os::raw::c_double,
     17 as std::os::raw::c_int as std::os::raw::c_double, 18 as std::os::raw::c_int as std::os::raw::c_double,
     19 as std::os::raw::c_int as std::os::raw::c_double,
     20 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub fn banner() {
    println!("  ______ _    _ __________________ _____  ");
    println!(" |  ____| |  | |___  /___  /  ____|  __ \\ ");
    println!(" | |__  | |  | |  / /   / /| |__  | |__) |");
    println!(" |  __| | |  | | / /   / / |  __| |  _  / ");
    println!(" | |    | |__| |/ /__ / /__| |____| | \\ \\ ");
    println!(" |_|     \\____//_____/_____|______|_|  \\_\\");
    println!();
    println!("This program tries each indicator with a lot of options.");
    println!("It's good for checking for crashes or assertion failures.");
    println!("Use with a memory tool to check for memory errors in ti.");
    println!();
}

#[no_mangle]
pub unsafe extern "C" fn check_output(mut info: *const ti_indicator_info,
                                      mut size: std::os::raw::c_int,
                                      mut inputs:
                                          *const *const std::os::raw::c_double,
                                      mut options: *const std::os::raw::c_double,
                                      mut outputs:
                                          *const *mut std::os::raw::c_double) {
    let mut s: std::os::raw::c_int = 0;
    s = (*info).start.expect("non-null function pointer")(options);
    let mut o: std::os::raw::c_int = 0;
    o = 0 as std::os::raw::c_int;
    while o < (*info).outputs {
        let mut max: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut min: std::os::raw::c_double = 0 as std::os::raw::c_int as std::os::raw::c_double;
        let mut i: std::os::raw::c_int = 0;
        i = 0 as std::os::raw::c_int;
        while i < size - s {
            let op: std::os::raw::c_double =
                *(*outputs.offset(o as isize)).offset(i as isize);
            let in_0: std::os::raw::c_double =
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
                        let mut k: std::os::raw::c_int = 0;
                        let mut j: std::os::raw::c_int = 0;
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
                            __assert_rtn((*::std::mem::transmute::<&[u8; 13],
                                                                   &[std::os::raw::c_char; 13]>(b"check_output\x00")).as_ptr(),
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
pub unsafe extern "C" fn stress(mut info: *const ti_indicator_info) {
    let opt_count = unsafe { (*info).options };
let name = unsafe { std::ffi::CStr::from_ptr((*info).name).to_string_lossy() };
let full_name = unsafe { std::ffi::CStr::from_ptr((*info).full_name).to_string_lossy() };
println!("{} ({}) ({} options)", name, full_name, opt_count);

let mut inputs: Vec<*const f64> = vec![std::ptr::null(); 10];
let mut inputs0: Vec<*const f64> = vec![std::ptr::null(); 10];
let mut outputs: Vec<*mut f64> = vec![std::ptr::null_mut(); 10];

for i in 0..10 {
    inputs[i] = if i < unsafe { (*info).inputs.try_into().unwrap() } {
        dummy_in.as_mut_ptr()
    } else {
        std::ptr::null()
    };
    inputs0[i] = if i < unsafe { (*info).inputs.try_into().unwrap() } {
        dummy_in0.as_mut_ptr()
    } else {
        std::ptr::null()
    };
    outputs[i] = if i < unsafe { (*info).outputs.try_into().unwrap() } {
        dummy_ot.as_mut_ptr()
    } else {
        std::ptr::null_mut()
    };
}

let mut options_index = [0; 11];
let mut options = [0.0; 10];
let choices = (std::mem::size_of::<[f64; 29]>() / std::mem::size_of::<f64>()) as i32;
let size = (std::mem::size_of::<[f64; 20]>() / std::mem::size_of::<f64>()) as i32;
let mut count = 0;

loop {
    count += 1;
    for j in 0..opt_count {
        options[j as usize] = optionsd[options_index[j as usize] as usize];
        print!(" {}", options[j as usize]);
    }

    let mut r: i32;
    println!(" 0");
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(0, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, 0, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    println!(" 1");
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(1, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, 1, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    println!(" 2");
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(2, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, 2, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    println!(" 3");
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(3, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, 3, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    println!(" {}", size);
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(size, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, size, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    println!(" 0s");
    r = (unsafe { (*info).indicator.expect("non-null function pointer") })(size, inputs0.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    if r == 0 {
        check_output(info, size, inputs0.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());
    }

    print!("\r                                                  \r");
    let mut j = 0;
    loop {
        options_index[j as usize] = (options_index[j as usize] + 1) % choices;
        if options_index[j as usize] != 0 {
            break;
        }
        j += 1;
    }
    if j > opt_count {
        break;
    }
}

    use std::f64;

if !(0.1 > (count as f64 - (choices as f64).powi(opt_count)).abs()) {
    panic!(
        "Assertion failed: .1 > fabs(count - (pow(choices, opt_count))) at fuzzer.c:154"
    );
} else { }

}
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    banner();
    if argc > 1 as std::os::raw::c_int {
        let mut info: *const ti_indicator_info =
            ti_find_indicator(*argv.offset(1 as std::os::raw::c_int as isize));
        if !info.is_null() { stress(info); }
    } else {
        let mut info_0: *const ti_indicator_info = ti_indicators.as_mut_ptr();
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
    let args: Vec<String> = ::std::env::args().collect();
    let args_c: Vec<std::ffi::CString> = args.iter()
        .map(|arg| ::std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let args_ptr: Vec<*mut std::os::raw::c_char> = args_c.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    // Call the unsafe function within an unsafe block
    let result = unsafe {
        main_0((args.len() - 1) as std::os::raw::c_int, args_ptr.as_ptr() as *mut *mut std::os::raw::c_char)
    };
    
    ::std::process::exit(result as i32);
}

