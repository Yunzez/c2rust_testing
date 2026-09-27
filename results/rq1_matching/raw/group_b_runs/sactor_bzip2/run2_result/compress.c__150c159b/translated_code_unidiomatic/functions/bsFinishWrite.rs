pub unsafe fn bsFinishWrite(s: *mut EState) {
    while (*s).bsLive > 0 {
        *(*s).zbits.add((*s).numZ as usize) = (((*s).bsBuff >> 24) & 0xFF) as ::core::ffi::c_uchar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8;
        (*s).bsLive -= 8;
    }
}
