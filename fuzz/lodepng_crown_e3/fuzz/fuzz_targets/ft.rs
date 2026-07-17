#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use lodepng_crown_e3::src::lodepng as translated;
extern "C" { fn free(p: *mut std::os::raw::c_void); }
fuzz_target!(|data: &[u8]| {
    unsafe {
        let mut out: *mut std::os::raw::c_uchar = std::ptr::null_mut();
        let mut w: std::os::raw::c_uint = 0; let mut h: std::os::raw::c_uint = 0;
        let _rc = translated::lodepng_decode32(Some(&mut out), Some(&mut w), Some(&mut h), data.as_ptr(), data.len() as _);
        if !out.is_null() { free(out as *mut _); }
    }
});
