pub fn lodepng_free(ptr: *mut libc::c_void) {
    unsafe {
        libc::free(ptr);
    }
}
