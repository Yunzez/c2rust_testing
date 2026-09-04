// SACTOR unidiomatic translation of `lodepng_convert_rgb` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:53:16; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe fn lodepng_convert_rgb(
    r_out: *mut u32,
    g_out: *mut u32,
    b_out: *mut u32,
    r_in: u32,
    g_in: u32,
    b_in: u32,
    mode_out: *const LodePNGColorMode,
    mode_in: *const LodePNGColorMode,
) -> u32 {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let mode_in_ref = &*mode_in;
    let mode_out_ref = &*mode_out;
    let mul: u32 = 65535 / ((1u32 << mode_in_ref.bitdepth) - 1u32);
    let shift: u32 = 16 - mode_out_ref.bitdepth;
    if mode_in_ref.colortype as u32 == LodePNGColorType::LCT_GREY as u32
        || mode_in_ref.colortype as u32 == LodePNGColorType::LCT_GREY_ALPHA as u32
    {
        r = r_in.wrapping_mul(mul);
        g = r;
        b = r;
    } else if mode_in_ref.colortype as u32 == LodePNGColorType::LCT_RGB as u32
        || mode_in_ref.colortype as u32 == LodePNGColorType::LCT_RGBA as u32
    {
        r = r_in.wrapping_mul(mul);
        g = g_in.wrapping_mul(mul);
        b = b_in.wrapping_mul(mul);
    } else if mode_in_ref.colortype as u32 == LodePNGColorType::LCT_PALETTE as u32 {
        if (r_in as usize) >= mode_in_ref.palettesize as usize {
            return 82;
        }
        let idx = (r_in as usize) * 4;
        let pal_ptr = mode_in_ref.palette;
        r = (*pal_ptr.add(idx + 0)) as u32 * 257u32;
        g = (*pal_ptr.add(idx + 1)) as u32 * 257u32;
        b = (*pal_ptr.add(idx + 2)) as u32 * 257u32;
    } else {
        return 31;
    }
    if mode_out_ref.colortype as u32 == LodePNGColorType::LCT_GREY as u32
        || mode_out_ref.colortype as u32 == LodePNGColorType::LCT_GREY_ALPHA as u32
    {
        *r_out = r >> shift;
    } else if mode_out_ref.colortype as u32 == LodePNGColorType::LCT_RGB as u32
        || mode_out_ref.colortype as u32 == LodePNGColorType::LCT_RGBA as u32
    {
        *r_out = r >> shift;
        *g_out = g >> shift;
        *b_out = b >> shift;
    } else if mode_out_ref.colortype as u32 == LodePNGColorType::LCT_PALETTE as u32 {
        if (r >> 8) != (r & 255) || (g >> 8) != (g & 255) || (b >> 8) != (b & 255) {
            return 82;
        }
        let mut i: u32 = 0;
        while (i as usize) < mode_out_ref.palettesize as usize {
            let j = (i as usize) * 4;
            let pal_ptr = mode_out_ref.palette;
            let pr = (*pal_ptr.add(j + 0)) as u32;
            let pg = (*pal_ptr.add(j + 1)) as u32;
            let pb = (*pal_ptr.add(j + 2)) as u32;
            if (r >> 8) == pr && (g >> 8) == pg && (b >> 8) == pb {
                *r_out = i;
                return 0;
            }
            i = i.wrapping_add(1);
        }
        return 82;
    } else {
        return 31;
    }
    0
}
