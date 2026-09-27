use libc::c_char;
pub unsafe fn strff(ptr: *mut c_char, n: i32) -> *mut c_char {
    let mut y: i32 = 0;
    let mut p = ptr;
    for _ in 0..n {
        y = *p as i32;
        p = p.add(1);
    }
    strdup(p as *const c_char)
}
