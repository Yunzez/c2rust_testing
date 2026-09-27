// SACTOR unidiomatic translation of `readChunk_sRGB` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:08:40; attempt 5). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_sRGB(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: size_t,
) -> ::core::ffi::c_uint {
    if chunkLength != 1 {
        return 98;
    }
    (*info).srgb_defined = 1 as ::core::ffi::c_uint;
    (*info).srgb_intent = *data as ::core::ffi::c_uint;
    0
}
