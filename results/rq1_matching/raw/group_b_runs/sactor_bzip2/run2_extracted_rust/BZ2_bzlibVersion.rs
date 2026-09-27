// SACTOR unidiomatic translation of `BZ2_bzlibVersion` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:39:45; attempt 1). Verification verdict: rust compiled
pub unsafe fn BZ2_bzlibVersion() -> *const libc::c_char {
    b"1.0.8, 13-Jul-2019\0".as_ptr() as *const libc::c_char
}
