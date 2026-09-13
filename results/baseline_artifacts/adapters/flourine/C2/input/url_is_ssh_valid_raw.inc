pub fn url_is_ssh(str: *mut std::os::raw::c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(str) };
    let str_slice = c_str.to_str().unwrap();
    str_slice == "ssh" || str_slice == "git"
}
