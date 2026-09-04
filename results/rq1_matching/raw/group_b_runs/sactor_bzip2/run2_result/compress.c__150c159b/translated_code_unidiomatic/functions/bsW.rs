#[inline]
pub unsafe fn bsW(s: *mut EState, n: i32, v: u32) {
    while (*s).bsLive >= 8 {
        *(*s).zbits.add((*s).numZ as usize) = ((*s).bsBuff >> 24) as u8;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8;
        (*s).bsLive -= 8;
    }
    (*s).bsBuff |= v << (32 - (*s).bsLive - n);
    (*s).bsLive += n;
}
