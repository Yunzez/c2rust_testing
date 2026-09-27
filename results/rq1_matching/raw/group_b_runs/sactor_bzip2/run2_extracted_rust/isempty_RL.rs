// SACTOR unidiomatic translation of `isempty_RL` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:36:14; attempt 1). Verification verdict: rust compiled
pub unsafe fn isempty_RL(s: *mut EState) -> ::core::ffi::c_int {
    if !s.is_null() && (*s).state_in_ch < 256 && (*s).state_in_len > 0 { 0 } else { 1 }
}
