#[no_mangle]
pub unsafe fn BZ2_bsInitWrite(s: *mut EState) {
    if !s.is_null() {
        (*s).bsLive = 0;
        (*s).bsBuff = 0;
    }
}
