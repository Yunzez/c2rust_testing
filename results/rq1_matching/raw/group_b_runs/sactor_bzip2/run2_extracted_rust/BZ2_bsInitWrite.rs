// SACTOR unidiomatic translation of `BZ2_bsInitWrite` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:42:51; attempt 1). Verification verdict: rust compiled
#[no_mangle]
pub unsafe extern "C" fn BZ2_bsInitWrite(s: *mut EState) {
    if !s.is_null() {
        (*s).bsLive = 0;
        (*s).bsBuff = 0;
    }
}
