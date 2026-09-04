// SACTOR unidiomatic translation of `lodepng_gtofl` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:11:32; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_lodepng/lodepng.c: Error: Failed to link project-level harness fo
fn lodepng_gtofl(a: libc::size_t, b: libc::size_t, c: libc::size_t) -> libc::c_int {
    let mut d: libc::size_t = 0;
    if unsafe { lodepng_addofl(a, b, &mut d as *mut libc::size_t) } != 0 {
        return 1;
    }
    if d > c { 1 } else { 0 }
}
