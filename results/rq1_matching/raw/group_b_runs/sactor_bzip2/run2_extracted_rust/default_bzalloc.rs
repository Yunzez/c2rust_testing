// SACTOR unidiomatic translation of `default_bzalloc` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:35:13; attempt 1). Verification verdict: rust compiled
use libc::{c_void, c_int, size_t, malloc};
unsafe fn default_bzalloc(
    opaque: *mut c_void,
    items: c_int,
    size: c_int,
) -> *mut c_void {
    unsafe fn mul_to_size_t(a: c_int, b: c_int) -> size_t {
        (a as size_t).wrapping_mul(b as size_t)
    }
    let total_size: size_t = mul_to_size_t(items, size);
    let v = malloc(total_size);
    v
}
