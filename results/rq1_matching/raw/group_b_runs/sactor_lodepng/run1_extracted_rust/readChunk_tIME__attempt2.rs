// SACTOR unidiomatic translation of `readChunk_tIME` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:56:54; attempt 2). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_tIME(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 7 {
        return 73;
    }
    let info_ref: &mut LodePNGInfo = &mut *info;
    info_ref.time_defined = 1;
    let d0 = *data.add(0) as ::core::ffi::c_uint;
    let d1 = *data.add(1) as ::core::ffi::c_uint;
    info_ref.time.year = 256u32 * d0 + d1;
    info_ref.time.month = *data.add(2) as ::core::ffi::c_uint;
    info_ref.time.day = *data.add(3) as ::core::ffi::c_uint;
    info_ref.time.hour = *data.add(4) as ::core::ffi::c_uint;
    info_ref.time.minute = *data.add(5) as ::core::ffi::c_uint;
    info_ref.time.second = *data.add(6) as ::core::ffi::c_uint;
    0
}
