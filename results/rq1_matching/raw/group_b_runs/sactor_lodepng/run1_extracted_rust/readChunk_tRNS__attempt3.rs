// SACTOR unidiomatic translation of `readChunk_tRNS` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:55:40; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_tRNS(
    color: *mut LodePNGColorMode,
    data: *const ::core::ffi::c_uchar,
    chunkLength: libc::size_t,
) -> libc::c_uint {
    use LodePNGColorType::{LCT_GREY, LCT_PALETTE, LCT_RGB};
    if (*color).colortype == LCT_PALETTE {
        if chunkLength > (*color).palettesize {
            return 39;
        }
        let mut i: libc::size_t = 0;
        while i != chunkLength {
            *(*color).palette.add(4 * i + 3) = *data.add(i);
            i += 1;
        }
    } else if (*color).colortype == LCT_GREY {
        if chunkLength != 2 {
            return 30;
        }
        (*color).key_defined = 1;
        let v: libc::c_uint = 256u32 * (*data.add(0) as libc::c_uint)
            + (*data.add(1) as libc::c_uint);
        (*color).key_r = v;
        (*color).key_g = v;
        (*color).key_b = v;
    } else if (*color).colortype == LCT_RGB {
        if chunkLength != 6 {
            return 41;
        }
        (*color).key_defined = 1;
        (*color).key_r = 256u32 * (*data.add(0) as libc::c_uint)
            + (*data.add(1) as libc::c_uint);
        (*color).key_g = 256u32 * (*data.add(2) as libc::c_uint)
            + (*data.add(3) as libc::c_uint);
        (*color).key_b = 256u32 * (*data.add(4) as libc::c_uint)
            + (*data.add(5) as libc::c_uint);
    } else {
        return 42;
    }
    0
}
