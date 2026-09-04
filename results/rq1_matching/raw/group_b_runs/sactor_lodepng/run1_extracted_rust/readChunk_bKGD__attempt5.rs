// SACTOR unidiomatic translation of `readChunk_bKGD` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:56:33; attempt 5). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_bKGD(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    unsafe fn byte_at(
        ptr: *const ::core::ffi::c_uchar,
        idx: usize,
    ) -> ::core::ffi::c_uint {
        *ptr.add(idx) as ::core::ffi::c_uint
    }
    let info_ref: &mut LodePNGInfo = &mut *info;
    let color: &LodePNGColorMode = &info_ref.color;
    if color.colortype == LodePNGColorType::LCT_PALETTE {
        if chunkLength != 1 {
            return 43;
        }
        if byte_at(data, 0) as usize >= color.palettesize {
            return 103;
        }
        info_ref.background_defined = 1;
        let v = byte_at(data, 0);
        info_ref.background_r = v;
        info_ref.background_g = v;
        info_ref.background_b = v;
    } else if color.colortype == LodePNGColorType::LCT_GREY
        || color.colortype == LodePNGColorType::LCT_GREY_ALPHA
    {
        if chunkLength != 2 {
            return 44;
        }
        info_ref.background_defined = 1;
        let v = 256u32 * byte_at(data, 0) + byte_at(data, 1);
        info_ref.background_r = v;
        info_ref.background_g = v;
        info_ref.background_b = v;
    } else if color.colortype == LodePNGColorType::LCT_RGB
        || color.colortype == LodePNGColorType::LCT_RGBA
    {
        if chunkLength != 6 {
            return 45;
        }
        info_ref.background_defined = 1;
        info_ref.background_r = 256u32 * byte_at(data, 0) + byte_at(data, 1);
        info_ref.background_g = 256u32 * byte_at(data, 2) + byte_at(data, 3);
        info_ref.background_b = 256u32 * byte_at(data, 4) + byte_at(data, 5);
    }
    0
}
