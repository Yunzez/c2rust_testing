// SACTOR unidiomatic translation of `ti_hma_start` (extracted from sactor-20260902T035700.jsonl at 2026-09-02 04:08:36; attempt 1). Verification verdict: Rust code failed to compile
pub unsafe fn ti_hma_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    let periodsqrt = libc::sqrt(period as libc::c_double) as libc::c_int;
    period + periodsqrt - 2
}
