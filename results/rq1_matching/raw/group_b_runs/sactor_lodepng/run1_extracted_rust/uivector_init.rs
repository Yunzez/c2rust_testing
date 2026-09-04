// SACTOR unidiomatic translation of `uivector_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:34:44; attempt 1). Verification verdict: rust compiled
#[no_mangle]
pub unsafe extern "C" fn uivector_init(p: *mut uivector) {
    if p.is_null() {
        return;
    }
    (*p).data = core::ptr::null_mut();
    (*p).size = 0;
    (*p).allocsize = 0;
}
