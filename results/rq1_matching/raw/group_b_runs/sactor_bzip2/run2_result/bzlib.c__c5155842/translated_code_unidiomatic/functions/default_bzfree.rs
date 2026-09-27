pub unsafe fn default_bzfree(opaque: *mut libc::c_void, addr: *mut libc::c_void) {
    if !addr.is_null() {
        libc::free(addr);
    }
}
