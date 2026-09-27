// SACTOR unidiomatic translation of `default_bzfree` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:35:28; attempt 1). Verification verdict: rust compiled
pub unsafe fn default_bzfree(opaque: *mut libc::c_void, addr: *mut libc::c_void) {
    if !addr.is_null() {
        libc::free(addr);
    }
}
