// SACTOR unidiomatic translation of `lodepng_malloc` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:32:28; attempt 1). Verification verdict: rust compiled
use libc::{c_void, size_t, malloc};
unsafe fn lodepng_malloc(size: size_t) -> *mut c_void {
    malloc(size)
}
