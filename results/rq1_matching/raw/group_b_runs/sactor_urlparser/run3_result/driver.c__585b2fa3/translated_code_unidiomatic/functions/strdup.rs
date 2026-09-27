pub unsafe fn strdup(str_ptr: *const libc::c_char) -> *mut libc::c_char {
    let n = libc::strlen(str_ptr).wrapping_add(1);
    let dup = libc::malloc(n) as *mut libc::c_char;
    if !dup.is_null() {
        libc::strcpy(dup, str_ptr);
    }
    dup
}
