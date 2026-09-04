// SACTOR unidiomatic translation of `BZ2_bzerror` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:40:55; attempt 4). Verification verdict: Rust code failed to compile
pub unsafe extern "C" fn BZ2_bzerror(
    b: *mut bzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut err = (*b).lastErr;
    if err > 0 {
        err = 0;
    }
    *errnum = err;
    bzerrorstrings[(-err) as usize]
}
