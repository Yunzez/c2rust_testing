// SACTOR unidiomatic translation of `readChunk_tIME` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:56:49; attempt 1). Verification verdict: Rust code failed to compile
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
    let bytes = ::core::slice::from_raw_parts(data, 7);
    info_ref.time.year = 256u32 * (bytes[0] as u32) + (bytes[1] as u32);
    info_ref.time.month = bytes[2] as u32;
    info_ref.time.day = bytes[3] as u32;
    info_ref.time.hour = bytes[4] as u32;
    info_ref.time.minute = bytes[5] as u32;
    info_ref.time.second = bytes[6] as u32;
    0
}
