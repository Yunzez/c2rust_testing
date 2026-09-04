// SACTOR unidiomatic translation of `BZ2_bzerror` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:40:52; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe extern "C" fn BZ2_bzerror(
    b: *mut bzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    unsafe fn get_last_err(b: *mut bzFile) -> ::core::ffi::c_int {
        (*b).lastErr
    }
    let mut err = get_last_err(b);
    if err > 0 {
        err = 0;
    }
    *errnum = err;
    bzerrorstrings[(-err) as usize]
}
