// SACTOR unidiomatic translation of `ti_md_start` (extracted from sactor-20260902T024638.jsonl at 2026-09-02 03:01:15; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators/md.c: Error: Failed to link project-level harnes
pub unsafe fn ti_md_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}
