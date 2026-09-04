// SACTOR unidiomatic translation of `BZ2_bzerror` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:41:01; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe extern "C" fn BZ2_bzerror(
    b: *mut bzFile,
    errnum: *mut ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    extern "C" {
        static bzerrorstrings: [*const ::core::ffi::c_char; 16];
    }
    let mut err = (*b).lastErr;
    if err > 0 {
        err = 0;
    }
    *errnum = err;
    let idx = (-err) as usize;
    bzerrorstrings[idx]
}
