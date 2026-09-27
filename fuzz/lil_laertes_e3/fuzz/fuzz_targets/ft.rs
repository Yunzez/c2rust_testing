#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use lil_laertes_e3::lil as translated;
fuzz_target!(|data: &[u8]| {
    if data.len() > 4096 { return; }
    unsafe {
        let lil = translated::lil_new();
        if lil.is_null() { return; }
        let val = translated::lil_parse(lil, data.as_ptr() as *const std::os::raw::c_char,
                                        data.len() as _, 0);
        if !val.is_null() { translated::lil_free_value(val); }
        translated::lil_free(lil);
    }
});
