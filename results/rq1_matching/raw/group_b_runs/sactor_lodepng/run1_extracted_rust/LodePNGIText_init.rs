// SACTOR unidiomatic translation of `LodePNGIText_init` (extracted from sactor-20260902T043148.jsonl at 2026-09-02 04:50:28; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn LodePNGIText_init(info: *mut LodePNGInfo) {
    (*info).itext_num = 0;
    (*info).itext_keys = ::core::ptr::null_mut();
    (*info).itext_langtags = ::core::ptr::null_mut();
    (*info).itext_transkeys = ::core::ptr::null_mut();
    (*info).itext_strings = ::core::ptr::null_mut();
}
