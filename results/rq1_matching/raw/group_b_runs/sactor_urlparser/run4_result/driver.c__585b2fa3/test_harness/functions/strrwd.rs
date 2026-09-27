use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
pub fn strdup(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}
pub fn strrwd_idiomatic(ptr: &str, n: usize) -> String {
    fn byte_suffix_from_end(s: &str, n: usize) -> &str {
        let len = s.len();
        if n >= len {
            return "";
        }
        let mut idx = len - n;
        while !s.is_char_boundary(idx) && idx < len {
            idx += 1;
        }
        &s[idx..]
    }
    let slice = byte_suffix_from_end(ptr, n);
    strdup(slice)
}
fn strrwd(ptr: *mut c_char, n: i32) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(ptr) };
    let input_str = c_str.to_str().unwrap();
    let n_usize = n as usize;
    let result = strrwd_idiomatic(input_str, n_usize);
    CString::new(result).unwrap().into_raw()
}
