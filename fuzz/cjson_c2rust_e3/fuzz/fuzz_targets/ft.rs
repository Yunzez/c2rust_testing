#![no_main]
#![allow(unused, non_snake_case)]
use libfuzzer_sys::fuzz_target;
use cjson_c2rust_e3 as translated;
fuzz_target!(|data: &[u8]| {
    // cap length: this cJSON has no nesting limit -> deep [[[ overflows the stack (a property of
    // the C lib, not the translation). Bounded input still drives all parse_* functions deeply.
    let n = data.len().min(512);
    let mut s = data[..n].to_vec(); s.push(0);
    unsafe {
        let root = translated::cJSON_Parse(s.as_ptr() as *const std::os::raw::c_char);
        if !root.is_null() { translated::cJSON_Delete(root); }
    }
});
