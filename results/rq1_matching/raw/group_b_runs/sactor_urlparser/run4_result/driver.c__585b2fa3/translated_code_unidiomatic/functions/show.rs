pub fn show(label: *const libc::c_char, v: *mut libc::c_char) {
    unsafe {
        libc::printf(
            b"%s: %s\n\0".as_ptr() as *const libc::c_char,
            label,
            if v.is_null() {
                b"(null)\0".as_ptr() as *const libc::c_char
            } else {
                v as *const libc::c_char
            },
        );
        if !v.is_null() {
            libc::free(v as *mut libc::c_void);
        }
    }
}
