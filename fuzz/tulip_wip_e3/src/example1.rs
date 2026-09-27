

extern "C" {
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    
    
    
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
}
pub use crate::indicators::sma::ti_sma;
pub use crate::indicators::sma::ti_sma_start;
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
pub unsafe extern "C" fn print_array(mut p: *const std::os::raw::c_double,
                                     size: std::os::raw::c_int) {
    let mut i: std::os::raw::c_int = 0;
    i = 0 as std::os::raw::c_int;
    while i < size {
        if i != 0 { printf(b", \x00" as *const u8 as *const std::os::raw::c_char); }
        printf(b"%.1f\x00" as *const u8 as *const std::os::raw::c_char,
               *p.offset(i as isize));
        i += 1
    }
    printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
}
fn main_0() -> i32 {
    let data_in: [f64; 10] = [5.0, 8.0, 12.0, 11.0, 9.0, 8.0, 7.0, 10.0, 11.0, 13.0];
    let input_length = data_in.len() as i32;
    println!("We have {} bars of input data.", input_length);
    
    unsafe {
        print_array(data_in.as_ptr(), input_length);
    }
    
    let options: [f64; 1] = [3.0];
    println!("Our option array is: ");
    
    unsafe {
        print_array(options.as_ptr(), options.len() as i32);
    }
    
    let start;
    unsafe {
        start = ti_sma_start(options.as_ptr());
    }
    println!("The start amount is: {}", start);
    
    let output_length = input_length - start;
    let mut data_out: Vec<f64> = vec![0.0; output_length as usize];
    
    println!("The output length is: {}", output_length);
    
    let all_inputs: Vec<*const f64> = vec![data_in.as_ptr()];
    let mut all_outputs: Vec<*mut f64> = vec![data_out.as_mut_ptr()];
    
    let error;
    unsafe {
        error = ti_sma(input_length, all_inputs.as_ptr(), options.as_ptr(), all_outputs.as_mut_ptr());
    }
    if error != 0 {
        panic!("Error: {}", error);
    }
    
    println!("The output data is: ");
    
    unsafe {
        print_array(data_out.as_ptr(), output_length);
    }
    
    0
}

pub fn main() { let exit_code = main_0();
std::process::exit(exit_code); }
