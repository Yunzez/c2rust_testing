// SACTOR unidiomatic translation of `ti_var_start` (extracted from sactor-20260902T035700.jsonl at 2026-09-02 04:22:22; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators/var.c: Error: Failed to link project-level harne
pub unsafe fn ti_var_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}
