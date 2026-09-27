// SACTOR unidiomatic translation of `init_RL` (extracted from sactor-20260902T143332.jsonl at 2026-09-02 14:35:59; attempt 1). Verification verdict: rust compiled
pub unsafe fn init_RL(s: *mut EState) {
    if !s.is_null() {
        (*s).state_in_ch = 256;
        (*s).state_in_len = 0;
    }
}
