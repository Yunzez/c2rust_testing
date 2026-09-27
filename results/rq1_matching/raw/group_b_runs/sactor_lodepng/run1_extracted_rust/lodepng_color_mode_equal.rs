// SACTOR unidiomatic translation of `lodepng_color_mode_equal` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:47:57; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_color_mode_equal(
    a: *const LodePNGColorMode,
    b: *const LodePNGColorMode,
) -> ::core::ffi::c_int {
    unsafe fn deref<'a>(p: *const LodePNGColorMode) -> &'a LodePNGColorMode {
        &*p
    }
    let a_ref = deref(a);
    let b_ref = deref(b);
    if a_ref.colortype != b_ref.colortype {
        return 0;
    }
    if a_ref.bitdepth != b_ref.bitdepth {
        return 0;
    }
    if a_ref.key_defined != b_ref.key_defined {
        return 0;
    }
    if a_ref.key_defined != 0 {
        if a_ref.key_r != b_ref.key_r {
            return 0;
        }
        if a_ref.key_g != b_ref.key_g {
            return 0;
        }
        if a_ref.key_b != b_ref.key_b {
            return 0;
        }
    }
    if a_ref.palettesize != b_ref.palettesize {
        return 0;
    }
    let mut i: size_t = 0;
    let total = a_ref.palettesize.wrapping_mul(4);
    while i != total {
        let a_val = *a_ref.palette.add(i as usize);
        let b_val = *b_ref.palette.add(i as usize);
        if a_val != b_val {
            return 0;
        }
        i = i.wrapping_add(1);
    }
    1
}
