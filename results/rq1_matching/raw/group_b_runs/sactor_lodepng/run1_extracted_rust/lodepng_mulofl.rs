// SACTOR unidiomatic translation of `lodepng_mulofl` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:34:25; attempt 1). Verification verdict: rust compiled
fn lodepng_mulofl(
    a: libc::size_t,
    b: libc::size_t,
    result: &mut libc::size_t,
) -> libc::c_int {
    *result = a.wrapping_mul(b);
    ((a != 0) && (*result / a != b)) as libc::c_int
}
