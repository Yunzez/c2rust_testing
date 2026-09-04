// SACTOR unidiomatic translation of `lodepng_color_mode_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:47:43; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_color_mode_init(info: *mut LodePNGColorMode) {
    type ColorType = LodePNGColorType;
    (*info).key_defined = 0;
    (*info).key_r = 0;
    (*info).key_g = 0;
    (*info).key_b = 0;
    (*info).colortype = ColorType::LCT_RGBA;
    (*info).bitdepth = 8;
    (*info).palette = core::ptr::null_mut();
    (*info).palettesize = 0;
}
