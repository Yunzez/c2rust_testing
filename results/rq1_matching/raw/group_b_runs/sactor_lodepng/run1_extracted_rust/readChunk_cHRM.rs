// SACTOR unidiomatic translation of `readChunk_cHRM` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:08:04; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_cHRM(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 32 {
        return 97;
    }
    unsafe fn read_be_u32(
        data: *const ::core::ffi::c_uchar,
        i: usize,
    ) -> ::core::ffi::c_uint {
        (16777216u32 * *data.add(i + 0) as u32) + (65536u32 * *data.add(i + 1) as u32)
            + (256u32 * *data.add(i + 2) as u32) + (*data.add(i + 3) as u32)
    }
    (*info).chrm_defined = 1;
    (*info).chrm_white_x = read_be_u32(data, 0);
    (*info).chrm_white_y = read_be_u32(data, 4);
    (*info).chrm_red_x = read_be_u32(data, 8);
    (*info).chrm_red_y = read_be_u32(data, 12);
    (*info).chrm_green_x = read_be_u32(data, 16);
    (*info).chrm_green_y = read_be_u32(data, 20);
    (*info).chrm_blue_x = read_be_u32(data, 24);
    (*info).chrm_blue_y = read_be_u32(data, 28);
    0
}
