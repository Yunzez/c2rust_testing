pub unsafe fn strff(mut ptr: *mut libc::c_char, n: libc::c_int) -> *mut libc::c_char {
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    while i < n {
        y = *ptr as libc::c_int;
        ptr = ptr.add(1);
        i += 1;
    }
    strdup(ptr as *const libc::c_char)
}
