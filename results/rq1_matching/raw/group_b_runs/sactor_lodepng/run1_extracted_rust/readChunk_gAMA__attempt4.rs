// SACTOR unidiomatic translation of `readChunk_gAMA` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 05:07:52; attempt 4). Verification verdict: Rust code failed to compile
pub unsafe fn readChunk_gAMA(
    info: *mut LodePNGInfo,
    data: *const ::core::ffi::c_uchar,
    chunkLength: usize,
) -> u32 {
    if chunkLength != 4 {
        return 96;
    }
    unsafe fn at(ptr: *const ::core::ffi::c_uchar, idx: usize) -> u32 {
        *ptr.add(idx) as u32
    }
    (*info).gama_defined = 1;
    (*info).gama_gamma = 16777216u32 * at(data, 0) + 65536u32 * at(data, 1)
        + 256u32 * at(data, 2) + at(data, 3);
    0
}
