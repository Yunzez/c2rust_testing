// SACTOR unidiomatic translation of `ti_hma_start` (extracted from sactor-20260902T024638.jsonl at 2026-09-02 02:58:04; attempt 2). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators/hma.c: Error: Failed to link project-level harne
use libc::c_int;
pub unsafe fn ti_hma_start(options: *const libc::c_double) -> c_int {
    let period = *options as c_int;
    let periodsqrt = ((period as libc::c_double).sqrt()) as c_int;
    period + periodsqrt - 2
}
