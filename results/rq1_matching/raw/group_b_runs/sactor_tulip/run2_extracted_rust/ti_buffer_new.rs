// SACTOR unidiomatic translation of `ti_buffer_new` (extracted from sactor-20260902T035700.jsonl at 2026-09-02 03:59:34; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/utils/buffer.c: Error: Failed to link project-level harness
pub unsafe fn ti_buffer_new(size: ::core::ffi::c_int) -> *mut ti_buffer {
    let s: ::core::ffi::c_int = ::core::mem::size_of::<ti_buffer>() as ::core::ffi::c_int
        + (size - 1)
            * ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int;
    let ret = libc::malloc(s as libc::size_t) as *mut ti_buffer;
    (*ret).size = size;
    (*ret).pushes = 0;
    (*ret).index = 0;
    (*ret).sum = 0.0;
    ret
}
