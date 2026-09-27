pub unsafe fn makeMaps_d(s: *mut DState) {
    if s.is_null() {
        return;
    }
    unsafe fn body(s: &mut DState) {
        s.nInUse = 0;
        let mut i: ::core::ffi::c_int = 0;
        while i < 256 {
            if s.inUse[i as usize] != 0 {
                s.seqToUnseq[s.nInUse as usize] = i as ::core::ffi::c_uchar;
                s.nInUse += 1;
            }
            i += 1;
        }
    }
    body(&mut *s);
}
