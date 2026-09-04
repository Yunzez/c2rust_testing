use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
pub fn strdup(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}
pub fn strff_idiomatic(input: &str, n: usize) -> String {
    fn strdup(inner: &str) -> String {
        inner.to_owned()
    }
    let bytes = input.as_bytes();
    let start = n.min(bytes.len());
    let suffix = std::str::from_utf8(&bytes[start..]).unwrap_or("");
    strdup(suffix)
}
fn strff(ptr: *mut c_char, n: i32) -> *mut c_char {
    let input = unsafe {
        if ptr.is_null() {
            ""
        } else {
            CStr::from_ptr(ptr).to_str().unwrap_or("")
        }
    };
    let n_usize = if n < 0 { 0 } else { n as usize };
    let result = strff_idiomatic(input, n_usize);
    match CString::new(result) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}
