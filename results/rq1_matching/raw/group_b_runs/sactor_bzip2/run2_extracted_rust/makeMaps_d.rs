// SACTOR unidiomatic translation of `makeMaps_d` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:44:16; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/decompress.c: Dependency 'BZ2_hbCreateDecodeTables' of type
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
