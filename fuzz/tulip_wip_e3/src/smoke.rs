



use std::println;

use std::ffi::CStr;
use std::sync::Mutex;

use std::ffi::CString;
use std::io::BufRead;

extern "C" {
    pub type __sFILEX;
    
    fn fabs(_: std::os::raw::c_double) -> std::os::raw::c_double;
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn fgets(_: *mut std::os::raw::c_char, _: std::os::raw::c_int, _: *mut FILE)
     -> *mut std::os::raw::c_char;
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn clock() -> clock_t;
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn strtok(_: *mut std::os::raw::c_char, _: *const std::os::raw::c_char)
     -> *mut std::os::raw::c_char;
    
    fn malloc(_: std::os::raw::c_ulong) -> *mut std::os::raw::c_void;
    
    fn free(_: *mut std::os::raw::c_void);
    
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    
    
    
    
    
    
    
    
    
    
    
    
}
pub use crate::indicators_index::ti_indicators;
pub use crate::indicators_index::ti_build;
pub use crate::indicators_index::ti_find_indicator;
pub use crate::indicators_index::ti_version;
pub use crate::utils::buffer::ti_buffer_free;
pub use crate::utils::buffer::ti_buffer_new;
pub type __int64_t = std::os::raw::c_longlong;
pub type __darwin_clock_t = std::os::raw::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut std::os::raw::c_uchar,
    pub _size: std::os::raw::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut std::os::raw::c_uchar,
    pub _r: std::os::raw::c_int,
    pub _w: std::os::raw::c_int,
    pub _flags: std::os::raw::c_short,
    pub _file: std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: std::os::raw::c_int,
    pub _cookie: *mut std::os::raw::c_void,
    pub _close: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void)
                           -> std::os::raw::c_int>,
    pub _read: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                           _: *mut std::os::raw::c_char,
                                           _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _seek: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void, _: fpos_t,
                                           _: std::os::raw::c_int) -> fpos_t>,
    pub _write: Option<unsafe extern "C" fn(_: *mut std::os::raw::c_void,
                                            _: *const std::os::raw::c_char,
                                            _: std::os::raw::c_int) -> std::os::raw::c_int>,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: std::os::raw::c_int,
    pub _ubuf: [std::os::raw::c_uchar; 3],
    pub _nbuf: [std::os::raw::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
pub type clock_t = __darwin_clock_t;
pub type ti_indicator_start_function = crate::example2::ti_indicator_start_function;
pub type ti_indicator_function = crate::example2::ti_indicator_function;
// #[derive(Copy, Clone)]

pub type ti_indicator_info = crate::fuzzer::ti_indicator_info;
// #[derive(Copy, Clone)]

pub type ti_buffer = crate::indicators::cci::ti_buffer;
static mut lfails: std::os::raw::c_int = 0 as std::os::raw::c_int;
static mut ltests: std::os::raw::c_int = 0 as std::os::raw::c_int;
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
pub static mut tested: [std::os::raw::c_int; 104] =
    [0 as std::os::raw::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
/* Compare two arrays, assert each element is equal. */
#[no_mangle]
pub fn compare_arrays(a: *mut f64, b: *mut f64, size_a: i32, size_b: i32) {
    unsafe {
        ltests += 1;
        if size_a != size_b {
            lfails += 1;
            println!("smoke.c:36 ({} != {})", size_a, size_b);
            println!("Size mismatch.");
            return;
        }
        
        let a_slice = std::slice::from_raw_parts(a, size_a as usize);
        let b_slice = std::slice::from_raw_parts(b, size_b as usize);

        for (i, (&val_a, &val_b)) in a_slice.iter().zip(b_slice.iter()).enumerate() {
            ltests += 1;
            if (val_a - val_b).abs() > 0.001 {
                lfails += 1;
                println!("smoke.c:42 ({} != {})", val_a, val_b);
            }
        }
    }
}

/*Return next non-comment, non-blank line.*/
#[no_mangle]
pub unsafe extern "C" fn next_line(mut fp: *mut FILE) -> *mut std::os::raw::c_char {
    static mut buf: [std::os::raw::c_char; 1024] = [0; 1024];
    while !fgets(buf.as_mut_ptr(), 1024 as std::os::raw::c_int, fp).is_null() {
        /*Skip Comments*/
        if buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == '#' as i32 {
            continue ;
        }
        /*Skip blank lines*/
        if buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int == 10 as std::os::raw::c_int
               ||
               buf[0 as std::os::raw::c_int as usize] as std::os::raw::c_int ==
                   13 as std::os::raw::c_int {
            continue ;
        }
        return buf.as_mut_ptr()
    }
    return 0 as *mut std::os::raw::c_char;
}
/*Read in float array.*/
#[no_mangle]
pub unsafe extern "C" fn get_array(mut fp: *mut FILE,
                                   mut s: *mut std::os::raw::c_double)
 -> std::os::raw::c_int {
    let mut line: *mut std::os::raw::c_char = next_line(fp);
    if *line.offset(0 as std::os::raw::c_int as isize) as std::os::raw::c_int != '{' as i32 {
        printf(b"Bad input\n\x00" as *const u8 as *const std::os::raw::c_char);
        ltests += 1;
        if 0 as std::os::raw::c_int == 0 {
            lfails += 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   70 as std::os::raw::c_int);
        }
        return 0 as std::os::raw::c_int
    }
    //#pragma warning(disable:4996) //MSVC
    let mut num: *mut std::os::raw::c_char =
        strtok(line.offset(1 as std::os::raw::c_int as isize),
               b",}\r\n\x00" as *const u8 as *const std::os::raw::c_char);
    if num.is_null() { return 0 as std::os::raw::c_int }
    let mut inp: *mut std::os::raw::c_double = s;
    loop  {
        *inp = atof(num);
        inp = inp.offset(1);
        num =
            strtok(0 as *mut std::os::raw::c_char,
                   b",}\r\n\x00" as *const u8 as *const std::os::raw::c_char);
        if num.is_null() { break ; }
    }
    return inp.offset_from(s) as std::os::raw::c_long as std::os::raw::c_int;
}
/*Read in options, inputs, answers, and test.*/
#[no_mangle]
pub unsafe extern "C" fn test_ind_name(mut fp: *mut FILE,
                                       mut name: *const std::os::raw::c_char,
                                       mut count: std::os::raw::c_int) {
    /*Find indicator from name.*/
    let mut info: *const ti_indicator_info = ti_find_indicator(name);
    if info.is_null() {
        printf(b"Couldn\'t find indicator %s\n\x00" as *const u8 as
                   *const std::os::raw::c_char, name);
        ltests += 1;
        if 0 as std::os::raw::c_int == 0 {
            lfails += 1;
            printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
                   b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
                   99 as std::os::raw::c_int);
        }
        return
    }
    let ind_num: std::os::raw::c_int =
        info.offset_from(ti_indicators.as_mut_ptr()) as std::os::raw::c_long
            as std::os::raw::c_int;
    if count != 0 { tested[ind_num as usize] = 1 as std::os::raw::c_int }
    let mut options: [std::os::raw::c_double; 10] = [0.; 10];
    let mut o: *mut std::os::raw::c_double = options.as_mut_ptr();
    let mut s: *const std::os::raw::c_char = 0 as *const std::os::raw::c_char;
    loop  {
        s =
            strtok(0 as *mut std::os::raw::c_char,
                   b" \n\r\x00" as *const u8 as *const std::os::raw::c_char);
        if s.is_null() { break ; }
        *o = atof(s);
        o = o.offset(1)
    }
    ltests += 1;
    if o.offset_from(options.as_mut_ptr()) as std::os::raw::c_long as
           std::os::raw::c_int != (*info).options {
        lfails += 1;
        printf(b"%s:%d (%d != %d)\n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               117 as std::os::raw::c_int,
               o.offset_from(options.as_mut_ptr()) as std::os::raw::c_long as
                   std::os::raw::c_int, (*info).options);
    }
    let mut i: std::os::raw::c_int = 0;
    let mut inputs: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut answers: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut outputs: [*mut std::os::raw::c_double; 10] =
        [0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double,
         0 as *mut std::os::raw::c_double, 0 as *mut std::os::raw::c_double];
    let mut input_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < (*info).inputs {
        inputs[i as usize] =
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        input_size = get_array(fp, inputs[i as usize]);
        i += 1
    }
    let mut answer_size: std::os::raw::c_int = 0 as std::os::raw::c_int;
    i = 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        answers[i as usize] =
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        outputs[i as usize] =
            malloc((::std::mem::size_of::<std::os::raw::c_double>() as
                        std::os::raw::c_ulong).wrapping_mul(512 as std::os::raw::c_int as
                                                        std::os::raw::c_ulong)) as
                *mut std::os::raw::c_double;
        answer_size = get_array(fp, answers[i as usize]);
        i += 1
    }
    let ret: std::os::raw::c_int =
        (*info).indicator.expect("non-null function pointer")(input_size,
                                                              inputs.as_mut_ptr()
                                                                  as
                                                                  *const *const std::os::raw::c_double,
                                                              options.as_mut_ptr(),
                                                              outputs.as_mut_ptr());
    ltests += 1;
    if !(ret == 0 as std::os::raw::c_int) {
        lfails += 1;
        printf(b"%s:%d error \n\x00" as *const u8 as *const std::os::raw::c_char,
               b"smoke.c\x00" as *const u8 as *const std::os::raw::c_char,
               139 as std::os::raw::c_int);
    }
    let mut output_size: std::os::raw::c_int =
        input_size -
            (*info).start.expect("non-null function pointer")(options.as_mut_ptr());
    if output_size < 0 as std::os::raw::c_int { output_size = 0 as std::os::raw::c_int }
    i = 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        compare_arrays(answers[i as usize], outputs[i as usize], answer_size,
                       output_size);
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < (*info).inputs {
        free(inputs[i as usize] as *mut std::os::raw::c_void);
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        free(answers[i as usize] as *mut std::os::raw::c_void);
        i += 1
    }
    i = 0 as std::os::raw::c_int;
    while i < (*info).outputs {
        free(outputs[i as usize] as *mut std::os::raw::c_void);
        i += 1
    };
}
#[no_mangle]
pub fn test(fname: &str, count: i32) {
    // Read in test values from external file.
    println!("{}:", fname);
    
    let fp = std::fs::File::open(fname).unwrap_or_else(|_| {
        unsafe {
            ltests += 1;
            lfails += 1;
        }
        eprintln!("{}:{} error", "smoke.c", 159);
        std::process::exit(1);
    });

    let reader = std::io::BufReader::new(fp);
    for line in reader.lines() {
        let line = line.unwrap();
        // Looking for function name.
        if let Some(first_char) = line.chars().next() {
            if !first_char.is_ascii_lowercase() {
                unsafe {
                    ltests += 1;
                    lfails += 1;
                }
                eprintln!("{}:{} error", "smoke.c", 169);
            } else {
                // Found function
                let name = line.split_whitespace().next().unwrap();
                let ts: i32;
                let fs: i32;
                unsafe {
                    ts = ltests;
                    fs = lfails;
                }
                let start = std::time::Instant::now();
                print!("\t{:16}", name);
                let name_cstr = std::ffi::CString::new(name).unwrap();
                unsafe {
                    test_ind_name(std::ptr::null_mut(), name_cstr.as_ptr(), count);
                }
                let duration = start.elapsed();
                unsafe {
                    println!("pass:{:2}   fail:{:2}   {}ms", ltests - ts - (lfails - fs), lfails - fs, duration.as_millis());
                }
            }
        }
    }
}

#[no_mangle]
pub fn test_version() {
    static LTESTS: Mutex<i32> = Mutex::new(0);
    static LFAILS: Mutex<i32> = Mutex::new(0);

    {
        let mut tests = LTESTS.lock().unwrap();
        *tests += 1;

        let expected_version = CString::new("0.8.4").unwrap();
        if expected_version.as_c_str() != unsafe { CStr::from_ptr(ti_version()) } {
            let mut fails = LFAILS.lock().unwrap();
            *fails += 1;
            println!("smoke.c:183 error");
        }
    }

    {
        let mut tests = LTESTS.lock().unwrap();
        *tests += 1;

        if 1537377628 != unsafe { ti_build() } {
            let mut fails = LFAILS.lock().unwrap();
            *fails += 1;
            println!("smoke.c:184 error");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_buffer() {
    let mut b = ti_buffer_new(3);
let b_ref = unsafe { &mut *b }; // Dereference the raw pointer to get a mutable reference

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 5.0;
b_ref.vals[b_ref.index as usize] = 5.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 5.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 190, b_ref.sum, 5.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 5.0;
b_ref.vals[b_ref.index as usize] = 5.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 10.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 191, b_ref.sum, 10.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 1.0;
b_ref.vals[b_ref.index as usize] = 1.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 11.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 192, b_ref.sum, 11.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 1.0;
b_ref.vals[b_ref.index as usize] = 1.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 7.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 193, b_ref.sum, 7.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 3.0;
b_ref.vals[b_ref.index as usize] = 3.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 5.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 194, b_ref.sum, 5.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 1.0;
b_ref.vals[b_ref.index as usize] = 1.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 5.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 195, b_ref.sum, 5.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 2.0;
b_ref.vals[b_ref.index as usize] = 2.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 6.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 196, b_ref.sum, 6.0);
}

if b_ref.pushes >= b_ref.size {
    b_ref.sum -= b_ref.vals[b_ref.index as usize];
}
b_ref.sum += 3.0;
b_ref.vals[b_ref.index as usize] = 3.0;
b_ref.pushes += 1;
b_ref.index += 1;
if b_ref.index >= b_ref.size {
    b_ref.index = 0;
}
ltests += 1;
if (b_ref.sum - 6.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 197, b_ref.sum, 6.0);
}

ltests += 1;
if (b_ref.vals[((b_ref.index + b_ref.size - 1) % b_ref.size) as usize] - 3.0).abs() > 0.001 {
    lfails += 1;
    println!("smoke.c:{} ({} != {})", 199, b_ref.vals[((b_ref.index + b_ref.size - 1) % b_ref.size) as usize], 3.0);
}

    let b_ref = &mut *b; // Dereference the raw pointer to get a mutable reference
let vals = &mut b_ref.vals;
let size = b_ref.size;
let index = b_ref.index;

let check_value = |expected: f64, line: usize| { // Change line to usize
    let offset_index = (index + size - 1 - line as i32) % size; // Cast line to i32
    let value = vals[offset_index as usize]; // Cast offset_index to usize
    if (value - expected).abs() > 0.001 {
        lfails += 1;
        println!("smoke.c:{} ({} != {})", line, value, expected);
    }
};

ltests += 1;
check_value(2.0, 200);
ltests += 1;
check_value(1.0, 201);
ltests += 1;
check_value(3.0, 202);

ti_buffer_free(Box::from_raw(b)); // Convert the raw pointer back to a Box
;
}
fn main_0() -> std::os::raw::c_int {
    println!("TI TEST SUITE");
    
    let (ts, fs) = unsafe { (ltests, lfails) };
    let start: std::time::Instant = std::time::Instant::now();
    print!("\t{:<16}", "buffer");
    
    unsafe { test_buffer(); }
    
    let duration = start.elapsed().as_millis();
    println!("pass:{:2}   fail:{:2}   {}ms", unsafe { ltests - ts - (lfails - fs) }, unsafe { lfails - fs }, duration);
    
    let (ts_0, fs_0) = unsafe { (ltests, lfails) };
    let start_0: std::time::Instant = std::time::Instant::now();
    print!("\t{:<16}", "version");
    
    unsafe { test_version(); }
    
    let duration_0 = start_0.elapsed().as_millis();
    println!("pass:{:2}   fail:{:2}   {}ms", unsafe { ltests - ts_0 - (lfails - fs_0) }, unsafe { lfails - fs_0 }, duration_0);
    
    test("tests/untest.txt", 0);
    test("tests/atoz.txt", 1);
    test("tests/extra.txt", 1);
    
    for i in 0..104 {
        if unsafe { tested[i as usize] } == 0 {
            println!("WARNING: no test for {}", unsafe { CStr::from_ptr(ti_indicators[i as usize].name).to_string_lossy() });
        }
    }
    
    if unsafe { lfails } == 0 {
        println!("ALL TESTS PASSED ({}/{})", unsafe { ltests }, unsafe { ltests });
    } else {
        println!("{} TESTS FAILED (of {})", unsafe { lfails }, unsafe { ltests });
    }
    
    (unsafe { lfails } != 0) as std::os::raw::c_int
}

pub fn main() { let exit_code = main_0() as i32;
std::process::exit(exit_code); }
