use libc::{c_char, free, strcmp};
pub unsafe fn url_is_ssh(mut str_ptr: *mut c_char) -> bool {
    str_ptr = strdup(str_ptr as *const c_char);
    let ssh = b"ssh\0".as_ptr() as *const c_char;
    let git = b"git\0".as_ptr() as *const c_char;
    if strcmp(str_ptr, ssh) == 0 || strcmp(str_ptr, git) == 0 {
        free(str_ptr as *mut libc::c_void);
        return true;
    }
    false
}
