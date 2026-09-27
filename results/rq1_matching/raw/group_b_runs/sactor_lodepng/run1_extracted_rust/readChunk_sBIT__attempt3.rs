// SACTOR unidiomatic translation of `readChunk_sBIT` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:09:14; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_sBIT(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    #[inline]
    unsafe fn byte_at(
        ptr: *const ::core::ffi::c_uchar,
        idx: usize,
    ) -> ::core::ffi::c_uchar {
        *ptr.add(idx)
    }
    let info_ref: &mut LodePNGInfo = &mut *info;
    let colortype = info_ref.color.colortype;
    let bitdepth: ::core::ffi::c_uint = if colortype == LodePNGColorType::LCT_PALETTE {
        8
    } else {
        info_ref.color.bitdepth
    };
    if colortype == LodePNGColorType::LCT_GREY {
        if chunkLength != 1 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        if d0 == 0 || d0 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d0;
        info_ref.sbit_b = d0;
    } else if colortype == LodePNGColorType::LCT_RGB
        || colortype == LodePNGColorType::LCT_PALETTE
    {
        if chunkLength != 3 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        let d2 = byte_at(data, 2) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 || d2 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth || d2 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d1;
        info_ref.sbit_b = d2;
    } else if colortype == LodePNGColorType::LCT_GREY_ALPHA {
        if chunkLength != 2 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d0;
        info_ref.sbit_b = d0;
        info_ref.sbit_a = d1;
    } else if colortype == LodePNGColorType::LCT_RGBA {
        if chunkLength != 4 {
            return 114;
        }
        let d0 = byte_at(data, 0) as ::core::ffi::c_uint;
        let d1 = byte_at(data, 1) as ::core::ffi::c_uint;
        let d2 = byte_at(data, 2) as ::core::ffi::c_uint;
        let d3 = byte_at(data, 3) as ::core::ffi::c_uint;
        if d0 == 0 || d1 == 0 || d2 == 0 || d3 == 0 {
            return 115;
        }
        if d0 > bitdepth || d1 > bitdepth || d2 > bitdepth || d3 > bitdepth {
            return 115;
        }
        info_ref.sbit_defined = 1;
        info_ref.sbit_r = d0;
        info_ref.sbit_g = d1;
        info_ref.sbit_b = d2;
        info_ref.sbit_a = d3;
    }
    0
}
