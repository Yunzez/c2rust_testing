// SACTOR unidiomatic translation of `getPixelColorRGBA16` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:52:23; attempt 2). Verification verdict: Rust code failed to compile
pub unsafe fn getPixelColorRGBA16(
    r: *mut u16,
    g: *mut u16,
    b: *mut u16,
    a: *mut u16,
    in_ptr: *const u8,
    i: usize,
    mode: *const LodePNGColorMode,
) {
    unsafe fn read_u16_be(base: *const u8, offset: usize) -> u16 {
        (256u16 * *base.add(offset) as u16) + *base.add(offset + 1) as u16
    }
    let mode_ref = &*mode;
    let in_base = in_ptr;
    const LCT_GREY_VAL: u8 = 0;
    const LCT_RGB_VAL: u8 = 2;
    const LCT_GREY_ALPHA_VAL: u8 = 4;
    const LCT_RGBA_VAL: u8 = 6;
    let colortype_val = mode_ref.colortype as u8;
    if colortype_val == LCT_GREY_VAL {
        let gray = read_u16_be(in_base, i * 2);
        *r = gray;
        *g = gray;
        *b = gray;
        if mode_ref.key_defined != 0 && gray as u32 == mode_ref.key_r {
            *a = 0;
        } else {
            *a = 65535;
        }
    } else if colortype_val == LCT_RGB_VAL {
        let r_val = read_u16_be(in_base, i * 6);
        let g_val = read_u16_be(in_base, i * 6 + 2);
        let b_val = read_u16_be(in_base, i * 6 + 4);
        *r = r_val;
        *g = g_val;
        *b = b_val;
        if mode_ref.key_defined != 0 && r_val as u32 == mode_ref.key_r
            && g_val as u32 == mode_ref.key_g && b_val as u32 == mode_ref.key_b
        {
            *a = 0;
        } else {
            *a = 65535;
        }
    } else if colortype_val == LCT_GREY_ALPHA_VAL {
        let gray = read_u16_be(in_base, i * 4);
        let alpha = read_u16_be(in_base, i * 4 + 2);
        *r = gray;
        *g = gray;
        *b = gray;
        *a = alpha;
    } else if colortype_val == LCT_RGBA_VAL {
        *r = read_u16_be(in_base, i * 8);
        *g = read_u16_be(in_base, i * 8 + 2);
        *b = read_u16_be(in_base, i * 8 + 4);
        *a = read_u16_be(in_base, i * 8 + 6);
    }
}
