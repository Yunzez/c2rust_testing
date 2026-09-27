// SACTOR unidiomatic translation of `lodepng_is_palette_type` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:48:59; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_is_palette_type(info: *const LodePNGColorMode) -> libc::c_uint {
    if (*info).colortype == LodePNGColorType::LCT_PALETTE { 1 } else { 0 }
}
