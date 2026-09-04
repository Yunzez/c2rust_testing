#[no_mangle]
pub unsafe fn uivector_init(p: *mut uivector) {
    if p.is_null() {
        return;
    }
    (*p).data = core::ptr::null_mut();
    (*p).size = 0;
    (*p).allocsize = 0;
}
