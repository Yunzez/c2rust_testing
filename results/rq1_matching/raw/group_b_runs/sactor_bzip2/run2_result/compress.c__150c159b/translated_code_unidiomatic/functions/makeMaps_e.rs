pub unsafe fn makeMaps_e(s: *mut EState) {
    if s.is_null() {
        return;
    }
    unsafe fn body(s: &mut EState) {
        let mut i: i32;
        s.nInUse = 0;
        i = 0;
        while i < 256 {
            if s.inUse[i as usize] != 0 {
                s.unseqToSeq[i as usize] = s.nInUse as u8;
                s.nInUse += 1;
            }
            i += 1;
        }
    }
    body(&mut *s);
}
