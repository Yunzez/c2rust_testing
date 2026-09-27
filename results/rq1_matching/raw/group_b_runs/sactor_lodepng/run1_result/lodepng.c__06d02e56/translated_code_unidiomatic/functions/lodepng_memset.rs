pub unsafe fn lodepng_memset(dst: *mut libc::c_void, value: libc::c_int, num: libc::size_t) {
    let mut i: libc::size_t = 0;
    while i < num {
        *(dst as *mut libc::c_char).add(i) = value as libc::c_char;
        i += 1;
    }
}
