// SACTOR unidiomatic translation of `lodepng_free` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:33:07; attempt 1). Verification verdict: rust compiled
fn lodepng_free(ptr: *mut libc::c_void) {
    unsafe {
        libc::free(ptr);
    }
}
