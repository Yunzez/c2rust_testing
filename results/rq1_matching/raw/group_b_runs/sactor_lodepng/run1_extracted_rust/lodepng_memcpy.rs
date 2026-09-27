// SACTOR unidiomatic translation of `lodepng_memcpy` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:33:26; attempt 1). Verification verdict: rust compiled
fn lodepng_memcpy(dst: *mut libc::c_void, src: *const libc::c_void, size: libc::size_t) {
    unsafe {
        let dst = dst as *mut libc::c_char;
        let src = src as *const libc::c_char;
        let mut i: libc::size_t = 0;
        while i < size {
            *dst.add(i) = *src.add(i);
            i += 1;
        }
    }
}
