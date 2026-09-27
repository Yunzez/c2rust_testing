// SACTOR unidiomatic translation of `lodepng_is_alpha_type` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:48:53; attempt 5). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_is_alpha_type(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    (((*info).colortype as u8) & 4 != 0) as ::core::ffi::c_uint
}
