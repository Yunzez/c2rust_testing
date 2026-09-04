// SACTOR unidiomatic translation of `rgba16ToPixel` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:52:10; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe fn rgba16ToPixel(
    out: *mut ::core::ffi::c_uchar,
    i: usize,
    mode: *const LodePNGColorMode,
    r: u16,
    g: u16,
    b: u16,
    a: u16,
) {
    let mode_ref = &*mode;
    if mode_ref.colortype == 0 {
        let gray: u16 = r;
        *out.add(i * 2 + 0) = ((gray >> 8) & 255) as u8;
        *out.add(i * 2 + 1) = (gray & 255) as u8;
    } else if mode_ref.colortype == 2 {
        *out.add(i * 6 + 0) = ((r >> 8) & 255) as u8;
        *out.add(i * 6 + 1) = (r & 255) as u8;
        *out.add(i * 6 + 2) = ((g >> 8) & 255) as u8;
        *out.add(i * 6 + 3) = (g & 255) as u8;
        *out.add(i * 6 + 4) = ((b >> 8) & 255) as u8;
        *out.add(i * 6 + 5) = (b & 255) as u8;
    } else if mode_ref.colortype == 4 {
        let gray: u16 = r;
        *out.add(i * 4 + 0) = ((gray >> 8) & 255) as u8;
        *out.add(i * 4 + 1) = (gray & 255) as u8;
        *out.add(i * 4 + 2) = ((a >> 8) & 255) as u8;
        *out.add(i * 4 + 3) = (a & 255) as u8;
    } else if mode_ref.colortype == 6 {
        *out.add(i * 8 + 0) = ((r >> 8) & 255) as u8;
        *out.add(i * 8 + 1) = (r & 255) as u8;
        *out.add(i * 8 + 2) = ((g >> 8) & 255) as u8;
        *out.add(i * 8 + 3) = (g & 255) as u8;
        *out.add(i * 8 + 4) = ((b >> 8) & 255) as u8;
        *out.add(i * 8 + 5) = (b & 255) as u8;
        *out.add(i * 8 + 6) = ((a >> 8) & 255) as u8;
        *out.add(i * 8 + 7) = (a & 255) as u8;
    }
}
