// SACTOR unidiomatic translation of `readChunk_tRNS` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:55:28; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_tRNS(
    color: *mut LodePNGColorMode,
    data: *const ::core::ffi::c_uchar,
    chunkLength: libc::size_t,
) -> libc::c_uint {
    unsafe fn get_colortype(color: *const LodePNGColorMode) -> LodePNGColorType {
        (*color).colortype
    }
    let colortype = unsafe { get_colortype(color) };
    if colortype == LodePNGColorType::LCT_PALETTE {
        unsafe {
            if chunkLength > (*color).palettesize {
                return 39;
            }
            let palette = (*color).palette;
            let mut i: libc::size_t = 0;
            while i != chunkLength {
                *palette.add(4 * i + 3) = *data.add(i);
                i += 1;
            }
        }
    } else if colortype == LodePNGColorType::LCT_GREY {
        if chunkLength != 2 {
            return 30;
        }
        unsafe {
            (*color).key_defined = 1;
            let v = 256u32 * (*data.add(0) as u32) + (*data.add(1) as u32);
            (*color).key_r = v;
            (*color).key_g = v;
            (*color).key_b = v;
        }
    } else if colortype == LodePNGColorType::LCT_RGB {
        if chunkLength != 6 {
            return 41;
        }
        unsafe {
            (*color).key_defined = 1;
            (*color).key_r = 256u32 * (*data.add(0) as u32) + (*data.add(1) as u32);
            (*color).key_g = 256u32 * (*data.add(2) as u32) + (*data.add(3) as u32);
            (*color).key_b = 256u32 * (*data.add(4) as u32) + (*data.add(5) as u32);
        }
    } else {
        return 42;
    }
    0
}
