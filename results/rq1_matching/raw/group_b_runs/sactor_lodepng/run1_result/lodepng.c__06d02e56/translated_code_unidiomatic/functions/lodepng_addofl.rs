pub fn lodepng_addofl(a: libc::size_t, b: libc::size_t, result: *mut libc::size_t) -> libc::c_int {
    unsafe {
        *result = a.wrapping_add(b);
        if *result < a {
            1
        } else {
            0
        }
    }
}
