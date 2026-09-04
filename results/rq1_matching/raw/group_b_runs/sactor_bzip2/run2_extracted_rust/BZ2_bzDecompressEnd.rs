// SACTOR unidiomatic translation of `BZ2_bzDecompressEnd` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:38:45; attempt 1). Verification verdict: rust compiled
pub unsafe extern "C" fn BZ2_bzDecompressEnd(
    strm: *mut bz_stream,
) -> ::core::ffi::c_int {
    const BZ_OK: ::core::ffi::c_int = 0;
    const BZ_PARAM_ERROR: ::core::ffi::c_int = -2;
    if strm.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s_ptr = (*strm).state as *mut DState;
    if s_ptr.is_null() {
        return BZ_PARAM_ERROR;
    }
    let s = &mut *s_ptr;
    if s.strm != strm {
        return BZ_PARAM_ERROR;
    }
    if !s.tt.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.tt as *mut ::core::ffi::c_void);
        }
    }
    if !s.ll16.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.ll16 as *mut ::core::ffi::c_void);
        }
    }
    if !s.ll4.is_null() {
        if let Some(bzfree_fn) = (*strm).bzfree {
            bzfree_fn((*strm).opaque, s.ll4 as *mut ::core::ffi::c_void);
        }
    }
    if let Some(bzfree_fn) = (*strm).bzfree {
        bzfree_fn((*strm).opaque, (*strm).state as *mut ::core::ffi::c_void);
    }
    (*strm).state = ::core::ptr::null_mut();
    BZ_OK
}
