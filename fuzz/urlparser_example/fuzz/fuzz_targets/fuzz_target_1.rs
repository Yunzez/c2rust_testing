#![no_main]

use core::ffi::c_char;
use libfuzzer_sys::fuzz_target;
use std::ffi::CStr;

use urlparser_example as translated;

extern "C" {
    fn c_url_parse(url: *mut c_char) -> *mut translated::url_data_t;
    fn c_url_free(data: *mut translated::url_data_t);

    fn c_url_get_protocol(url: *mut c_char) -> *mut c_char;
    fn c_url_get_host(url: *mut c_char) -> *mut c_char;
    fn c_url_get_path(url: *mut c_char) -> *mut c_char;
}

fn to_opt_str(ptr: *mut c_char) -> Option<&'static CStr> {
    if ptr.is_null() {
        None
    } else {
        // SAFETY: The C API promises NUL-terminated strings.
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

fuzz_target!(|data: &[u8]| {
    if data.is_empty() {
        return;
    }

    // Make a NUL-terminated buffer for C APIs.
    let mut buf = Vec::with_capacity(data.len() + 1);
    buf.extend_from_slice(data);
    buf.push(0);
    let c_str = buf.as_mut_ptr() as *mut c_char;

    unsafe {
        let c_parsed = c_url_parse(c_str);
        let r_parsed = translated::url_parse(c_str);

        // Compare a few representative getters.
        let c_proto = to_opt_str(c_url_get_protocol(c_str));
        let r_proto = to_opt_str(translated::url_get_protocol(c_str));
        if c_proto.map(|s| s.to_bytes()) != r_proto.map(|s| s.to_bytes()) {
            panic!("protocol divergence");
        }

        let c_host = to_opt_str(c_url_get_host(c_str));
        let r_host = to_opt_str(translated::url_get_host(c_str));
        if c_host.map(|s| s.to_bytes()) != r_host.map(|s| s.to_bytes()) {
            panic!("host divergence");
        }

        let c_path = to_opt_str(c_url_get_path(c_str));
        let r_path = to_opt_str(translated::url_get_path(c_str));
        if c_path.map(|s| s.to_bytes()) != r_path.map(|s| s.to_bytes()) {
            panic!("path divergence");
        }

        if !c_parsed.is_null() {
            c_url_free(c_parsed);
        }
        if !r_parsed.is_null() {
            translated::url_free(r_parsed);
        }
    }
});
