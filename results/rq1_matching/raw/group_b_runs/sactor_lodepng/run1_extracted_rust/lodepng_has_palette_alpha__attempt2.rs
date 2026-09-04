// SACTOR unidiomatic translation of `lodepng_has_palette_alpha` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:49:21; attempt 2). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_has_palette_alpha(
    info: *const LodePNGColorMode,
) -> ::core::ffi::c_uint {
    let mut i: libc::size_t = 0;
    while i != (*info).palettesize {
        let alpha = *(*info).palette.add(i.wrapping_mul(4).wrapping_add(3));
        if alpha < 255 {
            return 1;
        }
        i = i.wrapping_add(1);
    }
    0
}
