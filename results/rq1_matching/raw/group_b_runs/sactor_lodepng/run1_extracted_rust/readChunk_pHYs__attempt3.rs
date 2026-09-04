// SACTOR unidiomatic translation of `readChunk_pHYs` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:07:23; attempt 3). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_pHYs(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> ::core::ffi::c_uint {
    if chunkLength != 9 {
        return 74;
    }
    unsafe fn get_byte(
        ptr: *const ::core::ffi::c_uchar,
        offset: isize,
    ) -> ::core::ffi::c_uint {
        *ptr.offset(offset) as ::core::ffi::c_uint
    }
    (*info).phys_defined = 1;
    (*info).phys_x = 16_777_216u32 * get_byte(data, 0) + 65_536u32 * get_byte(data, 1)
        + 256u32 * get_byte(data, 2) + get_byte(data, 3);
    (*info).phys_y = 16_777_216u32 * get_byte(data, 4) + 65_536u32 * get_byte(data, 5)
        + 256u32 * get_byte(data, 6) + get_byte(data, 7);
    (*info).phys_unit = get_byte(data, 8);
    0
}
