// SACTOR unidiomatic translation of `ti_hma_start` (extracted from sactor-20260902T024638.jsonl at 2026-09-02 02:58:01; attempt 1). Verification verdict: Rust code failed to compile
use libc::c_int;
pub unsafe fn ti_hma_start(options: *const libc::c_double) -> c_int {
    let period = *options as c_int;
    let periodsqrt = (libc::sqrt(period as libc::c_double)) as c_int;
    period + periodsqrt - 2
}
