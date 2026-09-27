// SACTOR unidiomatic translation of `lodepng_addofl` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:34:06; attempt 1). Verification verdict: rust compiled
fn lodepng_addofl(
    a: libc::size_t,
    b: libc::size_t,
    result: *mut libc::size_t,
) -> libc::c_int {
    unsafe {
        *result = a.wrapping_add(b);
        if *result < a { 1 } else { 0 }
    }
}
