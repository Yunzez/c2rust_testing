// SACTOR unidiomatic translation of `LodePNGText_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:50:25; attempt 6). Verification verdict: Rust code failed to compile
pub unsafe fn LodePNGText_init(info: *mut LodePNGInfo) {
    (*info).text_num = 0;
    (*info).text_keys = ::core::ptr::null_mut();
    (*info).text_strings = ::core::ptr::null_mut();
}
