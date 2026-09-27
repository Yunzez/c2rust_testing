// SACTOR unidiomatic translation of `lodepng_is_greyscale_type` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:48:38; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_is_greyscale_type(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let info_ref = &*info;
    (info_ref.colortype == LodePNGColorType::LCT_GREY
        || info_ref.colortype == LodePNGColorType::LCT_GREY_ALPHA) as ::core::ffi::c_uint
}
