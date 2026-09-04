use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
pub fn strdup_idiomatic(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}
fn strdup(str_ptr: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(str_ptr) };
    let input = c_str.to_str().unwrap();
    let result = strdup_idiomatic(input);
    let c_string = CString::new(result).unwrap();
    c_string.into_raw()
}
