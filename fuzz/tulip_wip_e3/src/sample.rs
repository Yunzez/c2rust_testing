

use std::ffi::CString;

use std::ffi::CStr;

extern "C" {
    
    fn strcmp(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> std::os::raw::c_int;
    
    fn atof(_: *const std::os::raw::c_char) -> std::os::raw::c_double;
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn __assert_rtn(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char,
                    _: std::os::raw::c_int, _: *const std::os::raw::c_char) -> !;
    
    
    
    
}
pub use crate::indicators_index::ti_indicators;
pub use crate::indicators_index::ti_find_indicator;
pub type ti_indicator_start_function = crate::example2::ti_indicator_start_function;
pub type ti_indicator_function = crate::example2::ti_indicator_function;
// #[derive(Copy, Clone)]

pub type ti_indicator_info = crate::fuzzer::ti_indicator_info;
#[no_mangle]
pub static mut out: [[std::os::raw::c_double; 15]; 5] = [[0.; 15]; 5];
/* Example data on IBM. */
#[no_mangle]
pub static mut datet: [*const std::os::raw::c_char; 15] =
    [b"2005-11-01\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-02\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-03\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-04\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-07\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-08\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-09\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-10\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-11\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-14\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-15\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-16\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-17\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-18\x00" as *const u8 as *const std::os::raw::c_char,
     b"2005-11-21\x00" as *const u8 as *const std::os::raw::c_char];
#[no_mangle]
pub static mut date: [std::os::raw::c_double; 15] =
    [51101 as std::os::raw::c_int as std::os::raw::c_double,
     51102 as std::os::raw::c_int as std::os::raw::c_double,
     51103 as std::os::raw::c_int as std::os::raw::c_double,
     51104 as std::os::raw::c_int as std::os::raw::c_double,
     51107 as std::os::raw::c_int as std::os::raw::c_double,
     51108 as std::os::raw::c_int as std::os::raw::c_double,
     51109 as std::os::raw::c_int as std::os::raw::c_double,
     51110 as std::os::raw::c_int as std::os::raw::c_double,
     51111 as std::os::raw::c_int as std::os::raw::c_double,
     51114 as std::os::raw::c_int as std::os::raw::c_double,
     51115 as std::os::raw::c_int as std::os::raw::c_double,
     51116 as std::os::raw::c_int as std::os::raw::c_double,
     51117 as std::os::raw::c_int as std::os::raw::c_double,
     51118 as std::os::raw::c_int as std::os::raw::c_double,
     51121 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut open: [std::os::raw::c_double; 15] =
    [81.85f64, 81.2f64, 81.55f64, 82.91f64, 83.1f64, 83.41f64, 82.71f64,
     82.7f64, 84.2f64, 84.25f64, 84.03f64, 85.45f64, 86.18f64,
     88 as std::os::raw::c_int as std::os::raw::c_double, 87.6f64];
#[no_mangle]
pub static mut high: [std::os::raw::c_double; 15] =
    [82.15f64, 81.89f64, 83.03f64, 83.3f64, 83.85f64, 83.9f64, 83.33f64,
     84.3f64, 84.84f64, 85 as std::os::raw::c_int as std::os::raw::c_double, 85.9f64,
     86.58f64, 86.98f64, 88 as std::os::raw::c_int as std::os::raw::c_double, 87.87f64];
#[no_mangle]
pub static mut low: [std::os::raw::c_double; 15] =
    [81.29f64, 80.64f64, 81.31f64, 82.65f64, 83.07f64, 83.11f64, 82.49f64,
     82.3f64, 84.15f64, 84.11f64, 84.03f64, 85.39f64, 85.76f64, 87.17f64,
     87.01f64];
#[no_mangle]
pub static mut close: [std::os::raw::c_double; 15] =
    [81.59f64, 81.06f64, 82.87f64, 83 as std::os::raw::c_int as std::os::raw::c_double,
     83.61f64, 83.15f64, 82.84f64, 83.99f64, 84.55f64, 84.36f64, 85.53f64,
     86.54f64, 86.89f64, 87.77f64, 87.29f64];
#[no_mangle]
pub static mut volume: [std::os::raw::c_double; 15] =
    [5653100 as std::os::raw::c_int as std::os::raw::c_double,
     6447400 as std::os::raw::c_int as std::os::raw::c_double,
     7690900 as std::os::raw::c_int as std::os::raw::c_double,
     3831400 as std::os::raw::c_int as std::os::raw::c_double,
     4455100 as std::os::raw::c_int as std::os::raw::c_double,
     3798000 as std::os::raw::c_int as std::os::raw::c_double,
     3936200 as std::os::raw::c_int as std::os::raw::c_double,
     4732000 as std::os::raw::c_int as std::os::raw::c_double,
     4841300 as std::os::raw::c_int as std::os::raw::c_double,
     3915300 as std::os::raw::c_int as std::os::raw::c_double,
     6830800 as std::os::raw::c_int as std::os::raw::c_double,
     6694100 as std::os::raw::c_int as std::os::raw::c_double,
     5293600 as std::os::raw::c_int as std::os::raw::c_double,
     7985800 as std::os::raw::c_int as std::os::raw::c_double,
     4807900 as std::os::raw::c_int as std::os::raw::c_double];
#[no_mangle]
pub static mut alternative: [std::os::raw::c_double; 15] =
    [0.2f64, 0.3f64, 0.4f64, 0.3f64, 0.5f64, 0.7f64, 0.75f64, 0.9f64, 0.9f64,
     1 as std::os::raw::c_int as std::os::raw::c_double, 1 as std::os::raw::c_int as std::os::raw::c_double,
     0.2f64, 0.1f64, -0.1f64, -0.5f64];
unsafe fn main_0(mut argc: std::os::raw::c_int, mut argv: *mut *mut std::os::raw::c_char)
 -> std::os::raw::c_int {
    let argv1 = unsafe { CStr::from_ptr(*argv.offset(1)).to_str().unwrap() };
let info = ti_indicators.iter().find(|indicator| unsafe { CStr::from_ptr(indicator.name).to_str().unwrap() } == argv1);
if argc < 2 {
    println!("No indicator given.");
    println!("Example:");
    println!("\tsample ma 5");
    return 1;
}
if argv1 == "--version" {
    println!("TI VERSION: {}, TI BUILD: {}", "0.8.4", 1537377628);
    return 0;
}
if argv1 == "--list" {
    for indicator in ti_indicators.iter() {
        match indicator.type_0 {
            1 => println!("type overlay"),
            2 => println!("type indicator"),
            3 => println!("type math"),
            4 => println!("type simple"),
            _ => println!("type unknown"),
        }
        println!("name {}", unsafe { CStr::from_ptr(indicator.name).to_str().unwrap() });
        println!("full_name {{{}}}", unsafe { CStr::from_ptr(indicator.full_name).to_str().unwrap() });
        
        println!("inputs {{");
        for (i, input_name) in indicator.input_names.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{}", unsafe { CStr::from_ptr(*input_name).to_str().unwrap() });
        }
        println!("}}");
        
        println!("options {{");
        for (i, option_name) in indicator.option_names.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{{{}}}", unsafe { CStr::from_ptr(*option_name).to_str().unwrap() });
        }
        println!("}}");
        
        println!("outputs {{");
        for (i, output_name) in indicator.output_names.iter().enumerate() {
            if i != 0 {
                print!(" ");
            }
            print!("{{{}}}", unsafe { CStr::from_ptr(*output_name).to_str().unwrap() });
        }
        println!("}}");
        println!();
    }
    return 0;
}
let info_ptr = unsafe { CStr::from_ptr(*argv.offset(1)).as_ptr() };
let info = ti_find_indicator(info_ptr);
if info.is_null() {
    println!("Couldn't find indicator: {}", argv1);
    return 1;
}
let mut inputs: Vec<*const f64> = vec![std::ptr::null(); 5];
let mut outputs: Vec<*mut f64> = vec![out[0].as_mut_ptr(), out[1].as_mut_ptr(), out[2].as_mut_ptr(), out[3].as_mut_ptr(), out[4].as_mut_ptr()];
let (mut o, mut h, mut l, mut c, mut r, mut r2, mut v, mut a) = (0, 0, 0, 0, 0, 0, 0, 0);
let mut j = 0;

    while j < info.as_ref().unwrap().inputs {
    let input_name = unsafe { std::ffi::CStr::from_ptr(info.as_ref().unwrap().input_names[j as usize]) }
        .to_string_lossy()
        .into_owned();

    match input_name.as_str() {
        "open" => {
            inputs[j as usize] = open.as_mut_ptr();
            o = 1;
        }
        "high" => {
            inputs[j as usize] = high.as_mut_ptr();
            h = 1;
        }
        "low" => {
            inputs[j as usize] = low.as_mut_ptr();
            l = 1;
        }
        "close" => {
            inputs[j as usize] = close.as_mut_ptr();
            c = 1;
        }
        "volume" => {
            inputs[j as usize] = volume.as_mut_ptr();
            v = 1;
        }
        "real" => {
            if r == 0 {
                inputs[j as usize] = close.as_mut_ptr();
                r = 1;
            } else {
                inputs[j as usize] = open.as_mut_ptr();
                r2 = 1;
            }
        }
        _ => {
            assert!(false, "Unexpected input name");
        }
    }
    j += 1;
}

let alts: [&str; 8] = [
    "acos",
    "asin",
    "atan",
    "cosh",
    "sinh",
    "tanh",
    "todeg",
    "",
];

let mut alt = alts.iter();
while let Some(&a) = alt.next() {
    if a == unsafe { std::ffi::CStr::from_ptr(info.as_ref().unwrap().name) }
        .to_string_lossy()
        .as_ref() {
        r = 0;
        // Removed the assignment to `a` since it was incorrect
        j = 0;
        while j < info.as_ref().unwrap().inputs {
            inputs[j as usize] = alternative.as_mut_ptr();
            j += 1;
        }
        break;
    }
}

let mut options: [f64; 10] = [0.0; 10];
for i_0 in 0..info.as_ref().unwrap().options {
    if argc < 3 + i_0 {
        eprintln!("*ERROR NOT ENOUGH OPTIONS*");
        return 1;
    }
    let arg = unsafe { std::ffi::CStr::from_ptr(*argv.offset((2 + i_0) as isize)) }
        .to_string_lossy()
        .into_owned();
    options[i_0 as usize] = arg.parse::<f64>().unwrap();
}

let start: i32 = info.as_ref().unwrap().start.expect("non-null function pointer")(options.as_mut_ptr());
let ret: i32 = info.as_ref().unwrap().indicator.expect("non-null function pointer")(15, inputs.as_mut_ptr(), options.as_mut_ptr(), outputs.as_mut_ptr());

    if ret == 0 as std::os::raw::c_int {
        let mut i_1: std::os::raw::c_int = 0;
        let mut k: std::os::raw::c_int = 0;
        let mut bad: std::os::raw::c_int = 0 as std::os::raw::c_int;
        printf(b"date        \x00" as *const u8 as *const std::os::raw::c_char);
        if o != 0 {
            printf(b" open   \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if h != 0 {
            printf(b" high   \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if l != 0 {
            printf(b" low    \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if c != 0 {
            printf(b" close  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if v != 0 {
            printf(b" volume \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if r != 0 {
            printf(b" input  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if r2 != 0 {
            printf(b" input2  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        if a != 0 {
            printf(b" input  \x00" as *const u8 as *const std::os::raw::c_char);
        }
        i_1 = 0 as std::os::raw::c_int;
        while i_1 < (*info).outputs {
            printf(b" %s\x00" as *const u8 as *const std::os::raw::c_char,
                   (*info).output_names[i_1 as usize]);
            i_1 += 1
        }
        printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
        i_1 = 0 as std::os::raw::c_int;
        while i_1 < 15 as std::os::raw::c_int {
            printf(b"%s\x00" as *const u8 as *const std::os::raw::c_char,
                   datet[i_1 as usize]);
            if o != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       open[i_1 as usize]);
            }
            if h != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       high[i_1 as usize]);
            }
            if l != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       low[i_1 as usize]);
            }
            if c != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       close[i_1 as usize]);
            }
            if v != 0 {
                printf(b" %8.0f\x00" as *const u8 as *const std::os::raw::c_char,
                       volume[i_1 as usize]);
            }
            if r != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       close[i_1 as usize]);
            }
            if r2 != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       open[i_1 as usize]);
            }
            if a != 0 {
                printf(b" %8.2f\x00" as *const u8 as *const std::os::raw::c_char,
                       alternative[i_1 as usize]);
            }
            if i_1 >= start {
                k = 0 as std::os::raw::c_int;
                while k < (*info).outputs {
                    if out[k as usize][(i_1 - start) as usize] !=
                           out[k as usize][(i_1 - start) as usize] {
                        bad = 1 as std::os::raw::c_int
                    }
                    printf(b" %8.3f\x00" as *const u8 as *const std::os::raw::c_char,
                           out[k as usize][(i_1 - start) as usize]);
                    k += 1
                }
            }
            printf(b"\n\x00" as *const u8 as *const std::os::raw::c_char);
            i_1 += 1
        }
        if bad != 0 {
            printf(b"\nERROR NaN in outputs (%s).\n\x00" as *const u8 as
                       *const std::os::raw::c_char, (*info).name);
            return 1 as std::os::raw::c_int
        }
        return 0 as std::os::raw::c_int
    } else {
        if ret == 1 as std::os::raw::c_int {
            printf(b"*ERROR INVALID OPTION*\n\x00" as *const u8 as
                       *const std::os::raw::c_char);
        } else {
            printf(b"*ERROR*\n\x00" as *const u8 as *const std::os::raw::c_char);
        }
        return 1 as std::os::raw::c_int
    };
}
pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = (args.len() - 1) as std::os::raw::c_int;
    let arg_ptrs: Vec<std::ffi::CString> = args.iter()
        .map(|arg| std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut raw_args: Vec<*mut std::os::raw::c_char> = arg_ptrs.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    raw_args.push(std::ptr::null_mut());
    
    unsafe {
        let exit_code = main_0(arg_count, raw_args.as_mut_ptr());
        ::std::process::exit(exit_code as i32);
    }
}

