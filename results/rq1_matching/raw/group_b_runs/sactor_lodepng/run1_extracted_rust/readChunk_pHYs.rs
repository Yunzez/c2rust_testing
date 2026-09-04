// SACTOR unidiomatic translation of `readChunk_pHYs` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:57:13; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_pHYs(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 9 {
        return 74;
    }
    unsafe fn get_byte(ptr: *const ::core::ffi::c_uchar, offset: isize) -> u32 {
        *ptr.offset(offset) as u32
    }
    unsafe {
        (*info).phys_defined = 1;
        (*info).phys_x = 16777216u32 * get_byte(data, 0) + 65536u32 * get_byte(data, 1)
            + 256u32 * get_byte(data, 2) + get_byte(data, 3);
        (*info).phys_y = 16777216u32 * get_byte(data, 4) + 65536u32 * get_byte(data, 5)
            + 256u32 * get_byte(data, 6) + get_byte(data, 7);
        (*info).phys_unit = get_byte(data, 8);
    }
    0
}
