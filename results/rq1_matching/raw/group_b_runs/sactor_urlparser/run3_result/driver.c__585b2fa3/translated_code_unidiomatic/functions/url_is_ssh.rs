pub unsafe fn url_is_ssh(mut str_ptr: *mut libc::c_char) -> bool {
    str_ptr = strdup(str_ptr as *const libc::c_char);
    if !str_ptr.is_null() {
        let is_ssh = libc::strcmp(str_ptr, b"ssh\0".as_ptr() as *const libc::c_char) == 0;
        let is_git = libc::strcmp(str_ptr, b"git\0".as_ptr() as *const libc::c_char) == 0;
        if is_ssh || is_git {
            libc::free(str_ptr as *mut libc::c_void);
            return true;
        }
    }
    false
}
