use std::ffi::CStr;

include!["/input/url_is_ssh_valid_raw.inc"];

#[no_mangle]
pub fn url_is_ssh_valid(value: &mut [i8; 16]) -> bool { value[15] = 0; url_is_ssh(value.as_mut_ptr()) }
