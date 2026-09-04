// SACTOR unidiomatic translation of `lodepng_memset` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:33:46; attempt 1). Verification verdict: rust compiled
unsafe fn lodepng_memset(dst: *mut libc::c_void, value: libc::c_int, num: libc::size_t) {
    let mut i: libc::size_t = 0;
    while i < num {
        *(dst as *mut libc::c_char).add(i) = value as libc::c_char;
        i += 1;
    }
}
