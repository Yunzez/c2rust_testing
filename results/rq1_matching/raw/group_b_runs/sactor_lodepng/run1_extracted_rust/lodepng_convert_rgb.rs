// SACTOR unidiomatic translation of `lodepng_convert_rgb` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:52:58; attempt 1). Verification verdict: Rust code failed to compile
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
    let mode_in_ref: &LodePNGColorMode = &*mode_in;
    let mode_out_ref: &LodePNGColorMode = &*mode_out;
    let mul: u32 = 65535 / ((1u32 << mode_in_ref.bitdepth) - 1u32);
    let shift: u32 = 16 - mode_out_ref.bitdepth;
    if mode_in_ref.colortype == LodePNGColorType::LCT_GREY
        || mode_in_ref.colortype == LodePNGColorType::LCT_GREY_ALPHA
    {
        r = r_in.wrapping_mul(mul);
        g = r;
        b = r;
    } else if mode_in_ref.colortype == LodePNGColorType::LCT_RGB
        || mode_in_ref.colortype == LodePNGColorType::LCT_RGBA
    {
        r = r_in.wrapping_mul(mul);
        g = g_in.wrapping_mul(mul);
        b = b_in.wrapping_mul(mul);
    } else if mode_in_ref.colortype == LodePNGColorType::LCT_PALETTE {
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
    if mode_out_ref.colortype == LodePNGColorType::LCT_GREY
        || mode_out_ref.colortype == LodePNGColorType::LCT_GREY_ALPHA
    {
        if !r_out.is_null() {
            *r_out = r >> shift;
        }
    } else if mode_out_ref.colortype == LodePNGColorType::LCT_RGB
        || mode_out_ref.colortype == LodePNGColorType::LCT_RGBA
    {
        if !r_out.is_null() {
            *r_out = r >> shift;
        }
        if !g_out.is_null() {
            *g_out = g >> shift;
        }
        if !b_out.is_null() {
            *b_out = b >> shift;
        }
    } else if mode_out_ref.colortype == LodePNGColorType::LCT_PALETTE {
        if ((r >> 8) != (r & 255)) || ((g >> 8) != (g & 255)) || ((b >> 8) != (b & 255))
        {
            return 82;
        }
        let mut i: u32 = 0;
        while (i as usize) < mode_out_ref.palettesize as usize {
            let j = (i as usize) * 4;
            let pal_ptr = mode_out_ref.palette;
            let pr = (*pal_ptr.add(j + 0)) as u32;
            let pg = (*pal_ptr.add(j + 1)) as u32;
            let pb = (*pal_ptr.add(j + 2)) as u32;
            if ((r >> 8) == pr) && ((g >> 8) == pg) && ((b >> 8) == pb) {
                if !r_out.is_null() {
                    *r_out = i;
                }
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
