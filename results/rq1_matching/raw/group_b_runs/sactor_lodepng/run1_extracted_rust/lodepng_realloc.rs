// SACTOR unidiomatic translation of `lodepng_realloc` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:32:48; attempt 1). Verification verdict: rust compiled
use libc::{c_void, size_t, realloc};
#[inline]
unsafe fn lodepng_realloc(ptr: *mut c_void, new_size: size_t) -> *mut c_void {
    #[cfg(LODEPNG_MAX_ALLOC)]
    {
        extern "C" {
            static LODEPNG_MAX_ALLOC: size_t;
        }
        if new_size > LODEPNG_MAX_ALLOC {
            return core::ptr::null_mut();
        }
    }
    realloc(ptr, new_size)
}
