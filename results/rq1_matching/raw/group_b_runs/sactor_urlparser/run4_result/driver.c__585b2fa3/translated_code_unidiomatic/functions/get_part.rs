use libc::{c_char, c_int, c_void, free, malloc, sscanf, strcmp, strcpy};
pub unsafe fn get_part(url: *mut c_char, format: *const c_char, l: c_int) -> *mut c_char {
    let mut has: bool = false;
    let tmp = malloc(1) as *mut c_char;
    let tmp_url = strdup(url as *const c_char);
    let mut fmt_url = strdup(url as *const c_char);
    let mut ret = malloc(1) as *mut c_char;
    if tmp.is_null() || tmp_url.is_null() || fmt_url.is_null() || ret.is_null() {
        return core::ptr::null_mut();
    }
    strcpy(tmp, b"\0".as_ptr() as *const c_char);
    strcpy(fmt_url, b"\0".as_ptr() as *const c_char);
    fmt_url = strff(fmt_url, l);
    sscanf(fmt_url as *const c_char, format, tmp);
    if strcmp(tmp as *const c_char, tmp_url as *const c_char) != 0 {
        has = true;
        ret = strdup(tmp as *const c_char);
    }
    fmt_url = strrwd(fmt_url, l);
    free(tmp as *mut c_void);
    free(tmp_url as *mut c_void);
    free(fmt_url as *mut c_void);
    if has {
        ret
    } else {
        core::ptr::null_mut()
    }
}
