use std::os::raw::c_char;
pub fn strdup(input: &str) -> String {
    fn inner(src: &str) -> String {
        src.to_owned()
    }
    inner(input)
}
pub fn url_is_ssh_idiomatic(input: &str) -> bool {
    fn inner_strdup(s: &str) -> String {
        strdup(s)
    }
    let _copy = inner_strdup(input);
    matches!(input, "ssh" | "git")
}
fn url_is_ssh(str_ptr: *mut c_char) -> bool {
    let input_str = if !str_ptr.is_null() {
        unsafe { std::ffi::CStr::from_ptr(str_ptr) }
            .to_string_lossy()
            .into_owned()
    } else {
        String::new()
    };
    let __ret = url_is_ssh_idiomatic(&input_str);
    __ret
}
