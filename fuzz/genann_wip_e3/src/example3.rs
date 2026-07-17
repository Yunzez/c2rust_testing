

use std::ffi::CString;

extern "C" {
    
    
    fn fclose(_: *mut FILE) -> std::os::raw::c_int;
    
    fn fopen(_: *const std::os::raw::c_char, _: *const std::os::raw::c_char) -> *mut FILE;
    
    fn printf(_: *const std::os::raw::c_char, _: ...) -> std::os::raw::c_int;
    
    fn exit(_: std::os::raw::c_int) -> !;
    
    
    
    
    
    
}
pub use crate::genann::genann_free;
pub use crate::genann::genann_read;
pub use crate::genann::genann_run;
pub use crate::genann::__sFILEX;
pub type __int64_t = std::os::raw::c_longlong;
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
pub type genann_actfun = crate::example1::genann_actfun;
// #[derive(Copy, Clone)]

pub type genann = crate::example2::genann;

pub static mut save_name: *const std::os::raw::c_char =
    b"example/xor.ann\x00" as *const u8 as *const std::os::raw::c_char;
fn main_0(argc: i32, argv: Vec<*mut std::os::raw::c_char>) -> i32 {
    println!("GENANN example 3.");
    println!("Load a saved ANN to solve the XOR function.");
    
    let save_name_cstr = std::ffi::CString::new("example/xor.ann").unwrap();
    let saved = unsafe { fopen(save_name_cstr.as_ptr(), b"r\x00".as_ptr() as *const i8) };
    
    if saved.is_null() {
        eprintln!("Couldn't open file: {}.", save_name_cstr.to_str().unwrap());
        std::process::exit(1);
    }
    
    let ann = unsafe { genann_read(saved) };
    unsafe { fclose(saved) };
    
    if ann.is_null() {
        eprintln!("Error loading ANN from file: {}.", save_name_cstr.to_str().unwrap());
        std::process::exit(1);
    }
    
    /* Input data for the XOR function. */
    let input: [[f64; 2]; 4] = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    
    /* Run the network and see what it predicts. */
    for i in 0..4 {
        let output = unsafe { *genann_run(ann, input[i].as_ptr()) };
        println!("Output for [{:.1}, {:.1}] is {:.1}.", input[i][0], input[i][1], output);
    }
    
    unsafe { genann_free(ann) };
    0
}

pub fn main() {
    let args: Vec<String> = ::std::env::args().collect();
    let arg_count = (args.len() - 1) as std::os::raw::c_int;
    let arg_ptrs: Vec<std::ffi::CString> = args.iter()
        .map(|arg| ::std::ffi::CString::new(arg.clone()).expect("Failed to convert argument into CString."))
        .collect();
    
    let mut raw_ptrs: Vec<*mut std::os::raw::c_char> = arg_ptrs.iter()
        .map(|cstr| cstr.as_ptr() as *mut std::os::raw::c_char)
        .collect();
    
    raw_ptrs.push(std::ptr::null_mut());

    ::std::process::exit(main_0(arg_count, raw_ptrs) as i32);
}

