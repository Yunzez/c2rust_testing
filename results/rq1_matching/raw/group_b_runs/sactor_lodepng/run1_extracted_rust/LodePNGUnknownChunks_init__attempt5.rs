// SACTOR unidiomatic translation of `LodePNGUnknownChunks_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:50:06; attempt 5). Verification verdict: Rust code failed to compile
pub unsafe fn LodePNGUnknownChunks_init(info: *mut LodePNGInfo) {
    let mut i: ::core::ffi::c_uint = 0;
    while i != 3 {
        (*info).unknown_chunks_data[i as usize] = ::core::ptr::null_mut();
        i = i.wrapping_add(1);
    }
    i = 0;
    while i != 3 {
        (*info).unknown_chunks_size[i as usize] = 0;
        i = i.wrapping_add(1);
    }
}
