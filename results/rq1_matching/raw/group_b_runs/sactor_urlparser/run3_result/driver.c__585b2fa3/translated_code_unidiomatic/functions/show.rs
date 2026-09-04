pub unsafe fn show(label: *const libc::c_char, v: *mut libc::c_char) {
    let fmt = b"%s: %s\n\0".as_ptr() as *const libc::c_char;
    let null_str = b"(null)\0".as_ptr() as *const libc::c_char;
    libc::printf(fmt, label, if !v.is_null() { v } else { null_str });
    if !v.is_null() {
        libc::free(v as *mut libc::c_void);
    }
}
