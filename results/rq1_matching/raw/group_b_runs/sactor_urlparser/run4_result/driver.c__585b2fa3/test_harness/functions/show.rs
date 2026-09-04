use libc;
use std::ffi::CStr;
use std::ffi::CString;
pub fn show_idiomatic(label: &str, v: Option<String>) {
    fn print_line(label: &str, value: Option<&str>) {
        match value {
            Some(text) => println!("{label}: {text}"),
            None => println!("{label}: (null)"),
        }
    }
    print_line(label, v.as_deref());
}
fn show(label: *const libc::c_char, v: *mut libc::c_char) {
    let label = unsafe { CStr::from_ptr(label).to_str().unwrap_or("") };
    let v: Option<String> = if v.is_null() {
        None
    } else {
        let s = unsafe { CStr::from_ptr(v) };
        Some(s.to_string_lossy().into_owned())
    };
    show_idiomatic(label, v);
}
