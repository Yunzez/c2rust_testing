// SACTOR unidiomatic translation of `BZ2_bzCompressEnd` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:37:13; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe extern "C" fn BZ2_bzCompressEnd(strm: *mut bz_stream) -> ::core::ffi::c_int {
    use ::core::ffi::{c_int, c_void};
    const BZ_OK: c_int = 0;
    const BZ_PARAM_ERROR: c_int = -2;
    if strm.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s_ptr = (*strm).state as *mut EState;
    if s_ptr.is_null() {
        return BZ_PARAM_ERROR;
    }
    if (*s_ptr).strm != strm {
        return BZ_PARAM_ERROR;
    }
    unsafe fn bzfree_call(strm: *mut bz_stream, ptr: *mut c_void) {
        if ptr.is_null() {
            return;
        }
        if let Some(f) = (*strm).bzfree {
            f((*strm).opaque, ptr);
        }
    }
    bzfree_call(strm, (*s_ptr).arr1 as *mut c_void);
    bzfree_call(strm, (*s_ptr).arr2 as *mut c_void);
    bzfree_call(strm, (*s_ptr).ftab as *mut c_void);
    let state_ptr = (*strm).state;
    if !state_ptr.is_null() {
        bzfree_call(strm, state_ptr);
    }
    (*strm).state = ::core::ptr::null_mut::<c_void>();
    BZ_OK
}
