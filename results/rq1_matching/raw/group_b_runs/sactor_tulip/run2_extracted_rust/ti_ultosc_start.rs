// SACTOR unidiomatic translation of `ti_ultosc_start` (extracted from sactor-20260902T035700.jsonl at 2026-09-02 04:21:57; attempt 1). Verification verdict: Unidiomatic translation failed for /tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/sactor_tulip/indicators/ultosc.c: Error: Failed to link project-level ha
pub unsafe fn ti_ultosc_start(options: *const libc::c_double) -> libc::c_int {
    *options.add(2) as libc::c_int
}
