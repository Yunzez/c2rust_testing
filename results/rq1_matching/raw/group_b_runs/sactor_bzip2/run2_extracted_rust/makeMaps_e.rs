// SACTOR unidiomatic translation of `makeMaps_e` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:43:38; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_bzip2/compress.c: Error: Failed to link project-level harness for
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
