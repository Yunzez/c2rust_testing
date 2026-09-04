// SACTOR × tulip (run 1, 2026-09-02, PARTIAL, non-building): verbatim concatenation of the unidiomatic
// translations extracted from run1_result/logs/sactor-20260902T024638.jsonl (scripts/rq1_sactor_extract_log_rust.py);
// one function per TU (the first in dependency order), none verified (harness link failure, see RUN.md).
// Where a function was translated twice the LAST attempt is kept (ti_hma_start). run2_extracted_rust is a
// second independent draw of the same 69 functions and is not assembled.
#![allow(unused)]

// --- ti_ad_start.rs (attempt 1 of 1)
pub unsafe fn ti_ad_start(options: *const libc::c_double) -> libc::c_int {
    0
}

// --- ti_adosc_start.rs (attempt 1 of 1)
pub unsafe fn ti_adosc_start(options: *const libc::c_double) -> libc::c_int {
    (*options.add(1)) as libc::c_int - 1
}

// --- ti_ao_start.rs (attempt 1 of 1)
pub unsafe fn ti_ao_start(options: *const libc::c_double) -> libc::c_int {
    33
}

// --- ti_apo_start.rs (attempt 1 of 1)
pub unsafe fn ti_apo_start(options: *const libc::c_double) -> libc::c_int {
    1
}

// --- ti_aroon_start.rs (attempt 1 of 1)
pub unsafe fn ti_aroon_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_aroonosc_start.rs (attempt 1 of 1)
pub unsafe fn ti_aroonosc_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_avgprice_start.rs (attempt 1 of 1)
pub unsafe fn ti_avgprice_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_bbands_start.rs (attempt 1 of 1)
pub unsafe fn ti_bbands_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}

// --- ti_bop_start.rs (attempt 1 of 1)
pub unsafe fn ti_bop_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_buffer_new.rs (attempt 1 of 1)
pub unsafe fn ti_buffer_new(size: ::core::ffi::c_int) -> *mut ti_buffer {
    let s: ::core::ffi::c_int = ::core::mem::size_of::<ti_buffer>() as ::core::ffi::c_int
        + (size - 1)
            * ::core::mem::size_of::<::core::ffi::c_double>() as ::core::ffi::c_int;
    let ret = libc::malloc(s as libc::size_t) as *mut ti_buffer;
    (*ret).size = size;
    (*ret).pushes = 0;
    (*ret).index = 0;
    (*ret).sum = 0.0;
    ret
}

// --- ti_cci_start.rs (attempt 1 of 1)
pub unsafe fn ti_cci_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    (period - 1) * 2
}

// --- ti_cmo_start.rs (attempt 1 of 1)
pub unsafe fn ti_cmo_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_crossany_start.rs (attempt 1 of 1)
pub unsafe fn ti_crossany_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_crossover_start.rs (attempt 1 of 1)
pub unsafe fn ti_crossover_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_cvi_start.rs (attempt 1 of 1)
pub unsafe fn ti_cvi_start(options: *const libc::c_double) -> libc::c_int {
    let n = *options as libc::c_int;
    n * 2 - 1
}

// --- ti_decay_start.rs (attempt 1 of 1)
pub unsafe fn ti_decay_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_dema_start.rs (attempt 1 of 1)
pub unsafe fn ti_dema_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    (period - 1) * 2
}

// --- ti_dpo_start.rs (attempt 1 of 1)
pub unsafe fn ti_dpo_start(options: *const libc::c_double) -> libc::c_int {
    let first_option = *options;
    (first_option as libc::c_int) - 1
}

// --- ti_edecay_start.rs (attempt 1 of 1)
pub unsafe fn ti_edecay_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_ema_start.rs (attempt 1 of 1)
pub unsafe fn ti_ema_start(options: *const libc::c_double) -> libc::c_int {
    0
}

// --- ti_emv_start.rs (attempt 1 of 1)
pub unsafe fn ti_emv_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_fisher_start.rs (attempt 1 of 1)
pub unsafe fn ti_fisher_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_hma_start__attempt2.rs (attempt 2 of 2)
use libc::c_int;
pub unsafe fn ti_hma_start(options: *const libc::c_double) -> c_int {
    let period = *options as c_int;
    let periodsqrt = ((period as libc::c_double).sqrt()) as c_int;
    period + periodsqrt - 2
}

// --- ti_kama_start.rs (attempt 1 of 1)
pub unsafe fn ti_kama_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_kvo_start.rs (attempt 1 of 1)
pub unsafe fn ti_kvo_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_lag_start.rs (attempt 1 of 1)
pub unsafe fn ti_lag_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_macd_start.rs (attempt 1 of 1)
pub unsafe fn ti_macd_start(options: *const libc::c_double) -> libc::c_int {
    let long_period = *options.add(1) as libc::c_int;
    long_period - 1
}

// --- ti_marketfi_start.rs (attempt 1 of 1)
pub unsafe fn ti_marketfi_start(options: *const libc::c_double) -> libc::c_int {
    0
}

// --- ti_mass_start.rs (attempt 1 of 1)
pub unsafe fn ti_mass_start(options: *const libc::c_double) -> libc::c_int {
    let sum_p = *options as libc::c_int - 1;
    16 + sum_p
}

// --- ti_max_start.rs (attempt 1 of 1)
pub unsafe fn ti_max_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_md_start.rs (attempt 1 of 1)
pub unsafe fn ti_md_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_medprice_start.rs (attempt 1 of 1)
pub unsafe fn ti_medprice_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_mfi_start.rs (attempt 1 of 1)
pub unsafe fn ti_mfi_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_min_start.rs (attempt 1 of 1)
pub unsafe fn ti_min_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int - 1
}

// --- ti_mom_start.rs (attempt 1 of 1)
pub unsafe fn ti_mom_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_msw_start.rs (attempt 1 of 1)
pub unsafe fn ti_msw_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_nvi_start.rs (attempt 1 of 1)
pub unsafe fn ti_nvi_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_obv_start.rs (attempt 1 of 1)
pub unsafe fn ti_obv_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_ppo_start.rs (attempt 1 of 1)
pub unsafe fn ti_ppo_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_psar_start.rs (attempt 1 of 1)
pub unsafe fn ti_psar_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_pvi_start.rs (attempt 1 of 1)
pub unsafe fn ti_pvi_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_qstick_start.rs (attempt 1 of 1)
pub unsafe fn ti_qstick_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_roc_start.rs (attempt 1 of 1)
pub unsafe fn ti_roc_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_rocr_start.rs (attempt 1 of 1)
pub unsafe fn ti_rocr_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_rsi_start.rs (attempt 1 of 1)
pub unsafe fn ti_rsi_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_sma_start.rs (attempt 1 of 1)
pub unsafe fn ti_sma_start(options: *const libc::c_double) -> libc::c_int {
    let first = *options;
    (first as libc::c_int) - 1
}

// --- ti_stddev_start.rs (attempt 1 of 1)
pub unsafe fn ti_stddev_start(options: *const libc::c_double) -> libc::c_int {
    let first = *options;
    (first as libc::c_int) - 1
}

// --- ti_stderr_start.rs (attempt 1 of 1)
pub unsafe fn ti_stderr_start(options: *const libc::c_double) -> libc::c_int {
    let first = *options;
    (first as libc::c_int) - 1
}

// --- ti_stoch_start.rs (attempt 1 of 1)
pub unsafe fn ti_stoch_start(options: *const libc::c_double) -> libc::c_int {
    let kperiod = *options.add(0) as libc::c_int;
    let kslow = *options.add(1) as libc::c_int;
    let dperiod = *options.add(2) as libc::c_int;
    kperiod + kslow + dperiod - 3
}

// --- ti_stochrsi_start.rs (attempt 1 of 1)
pub unsafe fn ti_stochrsi_start(options: *const libc::c_double) -> libc::c_int {
    ((*(options as *const f64)) as libc::c_int) * 2 - 1
}

// --- ti_sum_start.rs (attempt 1 of 1)
pub unsafe fn ti_sum_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}

// --- ti_tema_start.rs (attempt 1 of 1)
pub unsafe fn ti_tema_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    (period - 1) * 3
}

// --- ti_trima_start.rs (attempt 1 of 1)
pub unsafe fn ti_trima_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}

// --- ti_trix_start.rs (attempt 1 of 1)
pub unsafe fn ti_trix_start(options: *const libc::c_double) -> libc::c_int {
    let period = *options as libc::c_int;
    ((period - 1) * 3) + 1
}

// --- ti_typprice_start.rs (attempt 1 of 1)
pub unsafe fn ti_typprice_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_ultosc_start.rs (attempt 1 of 1)
pub unsafe fn ti_ultosc_start(options: *const libc::c_double) -> libc::c_int {
    *options.add(2) as libc::c_int
}

// --- ti_var_start.rs (attempt 1 of 1)
pub unsafe fn ti_var_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}

// --- ti_version.rs (attempt 1 of 1)
pub unsafe fn ti_version() -> *const libc::c_char {
    b"0.9.2\0".as_ptr() as *const libc::c_char
}

// --- ti_vhf_start.rs (attempt 1 of 1)
pub unsafe fn ti_vhf_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_vidya_start.rs (attempt 1 of 1)
pub unsafe fn ti_vidya_start(options: *const libc::c_double) -> libc::c_int {
    let val = *options.add(1);
    (val as libc::c_int) - 2
}

// --- ti_volatility_start.rs (attempt 1 of 1)
pub unsafe fn ti_volatility_start(options: *const libc::c_double) -> libc::c_int {
    *options as libc::c_int
}

// --- ti_vosc_start.rs (attempt 1 of 1)
pub unsafe fn ti_vosc_start(options: *const libc::c_double) -> libc::c_int {
    (*options.add(1) as libc::c_int) - 1
}

// --- ti_vwma_start.rs (attempt 1 of 1)
pub unsafe fn ti_vwma_start(options: *const libc::c_double) -> libc::c_int {
    let first_option = *options;
    (first_option as libc::c_int) - 1
}

// --- ti_wad_start.rs (attempt 1 of 1)
pub unsafe fn ti_wad_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    1
}

// --- ti_wcprice_start.rs (attempt 1 of 1)
pub unsafe fn ti_wcprice_start(options: *const libc::c_double) -> libc::c_int {
    let _ = options;
    0
}

// --- ti_wilders_start.rs (attempt 1 of 1)
pub unsafe fn ti_wilders_start(options: *const libc::c_double) -> libc::c_int {
    let first = *options;
    (first as libc::c_int) - 1
}

// --- ti_willr_start.rs (attempt 1 of 1)
pub unsafe fn ti_willr_start(options: *const libc::c_double) -> libc::c_int {
    (*options as libc::c_int) - 1
}

// --- ti_wma_start.rs (attempt 1 of 1)
pub unsafe fn ti_wma_start(options: *const libc::c_double) -> libc::c_int {
    (*options.offset(0) as libc::c_int) - 1
}

// --- ti_zlema_start.rs (attempt 1 of 1)
pub unsafe fn ti_zlema_start(options: *const libc::c_double) -> libc::c_int {
    (((*options) as libc::c_int) - 1) / 2 - 1
}
