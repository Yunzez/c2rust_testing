#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use std::os::raw::c_char;
use std::ffi::CString;
fuzz_target!(|data: &[u8]| {
    if data.len() < 8 || data.len() > 65536 { return; }
    let path = "/tmp/optipng_e3_laertes.png";
    if std::fs::write(path, data).is_err() { return; }
    let prog = CString::new("optipng").unwrap();
    let sim = CString::new("-simulate").unwrap();
    let cpath = CString::new(path).unwrap();
    let mut argv: [*mut c_char; 4] = [
        prog.as_ptr() as *mut c_char,
        sim.as_ptr() as *mut c_char,
        cpath.as_ptr() as *mut c_char,
        std::ptr::null_mut(),
    ];
    unsafe { let _ = optipng_laertes_e3::e3_main(3, argv.as_mut_ptr()); }
});
