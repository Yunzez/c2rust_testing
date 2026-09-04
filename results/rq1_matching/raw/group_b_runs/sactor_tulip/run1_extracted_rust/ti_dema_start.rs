// SACTOR unidiomatic translation of `ti_dema_start` (extracted from sactor-20260902T024638.jsonl at 2026-09-02 02:55:37; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators/dema.c: Error: Failed to link project-level harn
pub unsafe fn ti_dema_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    (period - 1) * 2
}
