// SACTOR unidiomatic translation of `bsFinishWrite` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:43:07; attempt 1). Verification verdict: rust compiled
pub unsafe fn bsFinishWrite(s: *mut EState) {
    while (*s).bsLive > 0 {
        *(*s).zbits.add((*s).numZ as usize) = (((*s).bsBuff >> 24) & 0xFF)
            as ::core::ffi::c_uchar;
        (*s).numZ += 1;
        (*s).bsBuff <<= 8;
        (*s).bsLive -= 8;
    }
}
