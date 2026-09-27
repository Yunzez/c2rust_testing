// SACTOR unidiomatic translation of `lodepng_is_alpha_type` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:48:44; attempt 2). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_is_alpha_type(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let colortype_value = (*info).colortype as u8;
    ((colortype_value & 4) != 0) as ::core::ffi::c_uint
}
