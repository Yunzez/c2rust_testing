// SACTOR unidiomatic translation of `lodepng_is_palette_type` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:49:02; attempt 2). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_is_palette_type(info: *const LodePNGColorMode) -> libc::c_uint {
    if (*info).colortype as libc::c_uint == 3 { 1 } else { 0 }
}
