use libc::c_char;
pub unsafe fn strrwd(mut ptr: *mut c_char, n: i32) -> *mut c_char {
    let mut y: i32 = 0;
    let mut i: i32 = 0;
    while i < n {
        y = *ptr as i32;
        ptr = ptr.offset(-1);
        i += 1;
    }
    strdup(ptr as *const c_char)
}
