// SACTOR unidiomatic translation of `ti_version` (extracted from sactor-20260902T035700.jsonl at 2026-09-02 03:58:19; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators.c: Error: Failed to link project-level harness f
pub unsafe fn ti_version() -> *const libc::c_char {
    b"0.9.2\0".as_ptr() as *const libc::c_char
}
