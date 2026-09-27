# tulip × c2saferrust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 213 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 212 |
| built | 212 |
| executed (corpus > 0) | 212 |
| coverage exported | 212 |

Plan failures, by the generator's own reason:

- **1** × struct parameter has Rust type Box<ti_buffer>

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `ti_abs` | no | 20 | 0 | normal 20 | batch |
| `ti_abs_start` | no | 1 | 0 | normal 1 | batch |
| `ti_acos` | no | 27 | 0 | normal 27 | batch |
| `ti_acos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ad` | no | 20 | 0 | normal 20 | batch |
| `ti_ad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_add` | no | 16 | 0 | normal 16 | batch |
| `ti_add_start` | no | 1 | 0 | normal 1 | batch |
| `ti_adosc` | no | 10 | 0 | normal 10 | batch |
| `ti_adosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_adx` | no | 9 | 0 | normal 9 | batch |
| `ti_adx_start` | no | 9 | 3 | divergence 6, panic 3 | per-input (6/9 completed) |
| `ti_adxr` | no | 9 | 0 | normal 9 | batch |
| `ti_adxr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ao` | no | 16 | 0 | normal 16 | batch |
| `ti_ao_start` | no | 1 | 0 | normal 1 | batch |
| `ti_apo` | no | 13 | 0 | normal 13 | batch |
| `ti_apo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_aroon` | no | 9 | 0 | normal 9 | batch |
| `ti_aroon_start` | no | 8 | 0 | normal 8 | batch |
| `ti_aroonosc` | no | 10 | 0 | normal 10 | batch |
| `ti_aroonosc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_asin` | no | 32 | 0 | normal 32 | batch |
| `ti_asin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atan` | no | 31 | 0 | normal 31 | batch |
| `ti_atan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atr` | no | 9 | 0 | normal 9 | batch |
| `ti_atr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_avgprice` | no | 15 | 0 | normal 15 | batch |
| `ti_avgprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_bbands` | no | 10 | 0 | normal 10 | batch |
| `ti_bbands_start` | no | 1 | 1 | normal 1 | batch |
| `ti_bop` | no | 17 | 0 | normal 17 | batch |
| `ti_bop_start` | no | 1 | 0 | normal 1 | batch |
| `ti_buffer_new` | no | 1 | 1 | normal 1 | batch |
| `ti_build` | no | 1 | 0 | normal 1 | batch |
| `ti_cci` | no | 12 | 0 | normal 12 | batch |
| `ti_cci_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ceil` | no | 31 | 0 | normal 31 | batch |
| `ti_ceil_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cmo` | no | 10 | 0 | normal 10 | batch |
| `ti_cmo_start` | no | 8 | 0 | normal 8 | batch |
| `ti_cos` | no | 28 | 0 | normal 28 | batch |
| `ti_cos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cosh` | no | 32 | 0 | normal 32 | batch |
| `ti_cosh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossany` | no | 26 | 0 | normal 26 | batch |
| `ti_crossany_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossover` | no | 18 | 0 | normal 18 | batch |
| `ti_crossover_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cvi` | no | 9 | 0 | normal 9 | batch |
| `ti_cvi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_decay` | no | 16 | 0 | normal 16 | batch |
| `ti_decay_start` | no | 1 | 0 | normal 1 | batch |
| `ti_dema` | no | 13 | 0 | normal 13 | batch |
| `ti_dema_start` | no | 1 | 1 | normal 1 | batch |
| `ti_di` | no | 9 | 0 | normal 9 | batch |
| `ti_di_start` | no | 1 | 1 | normal 1 | batch |
| `ti_div` | no | 15 | 0 | normal 15 | batch |
| `ti_div_start` | no | 1 | 0 | normal 1 | batch |
| `ti_dm` | no | 9 | 0 | normal 9 | batch |
| `ti_dm_start` | no | 1 | 1 | normal 1 | batch |
| `ti_dpo` | no | 10 | 0 | normal 10 | batch |
| `ti_dpo_start` | no | 1 | 1 | normal 1 | batch |
| `ti_dx` | no | 9 | 0 | normal 9 | batch |
| `ti_dx_start` | no | 1 | 1 | normal 1 | batch |
| `ti_edecay` | no | 15 | 0 | normal 15 | batch |
| `ti_edecay_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ema` | no | 13 | 0 | normal 13 | batch |
| `ti_ema_start` | no | 1 | 0 | normal 1 | batch |
| `ti_emv` | no | 15 | 0 | normal 15 | batch |
| `ti_emv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_exp` | no | 16 | 0 | normal 16 | batch |
| `ti_exp_start` | no | 1 | 0 | normal 1 | batch |
| `ti_find_indicator` | no | 19 | 0 | normal 19 | batch |
| `ti_fisher` | no | 10 | 0 | normal 10 | batch |
| `ti_fisher_start` | no | 1 | 1 | normal 1 | batch |
| `ti_floor` | no | 28 | 0 | normal 28 | batch |
| `ti_floor_start` | no | 1 | 0 | normal 1 | batch |
| `ti_fosc` | no | 10 | 0 | normal 10 | batch |
| `ti_fosc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_hma` | no | 10 | 0 | normal 10 | batch |
| `ti_hma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kama` | no | 10 | 0 | normal 10 | batch |
| `ti_kama_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kvo` | no | 10 | 0 | normal 10 | batch |
| `ti_kvo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_lag` | no | 17 | 0 | normal 17 | batch |
| `ti_lag_start` | no | 8 | 0 | normal 8 | batch |
| `ti_linreg` | no | 10 | 0 | normal 10 | batch |
| `ti_linreg_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregintercept` | no | 10 | 1 | normal 10 | batch |
| `ti_linregintercept_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregslope` | no | 10 | 0 | normal 10 | batch |
| `ti_linregslope_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ln` | no | 16 | 0 | normal 16 | batch |
| `ti_ln_start` | no | 1 | 0 | normal 1 | batch |
| `ti_log10` | no | 17 | 0 | normal 17 | batch |
| `ti_log10_start` | no | 1 | 0 | normal 1 | batch |
| `ti_macd` | no | 14 | 0 | normal 14 | batch |
| `ti_macd_start` | no | 1 | 1 | normal 1 | batch |
| `ti_marketfi` | no | 15 | 0 | normal 15 | batch |
| `ti_marketfi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_mass` | no | 11 | 0 | normal 11 | batch |
| `ti_mass_start` | no | 1 | 1 | normal 1 | batch |
| `ti_max` | no | 10 | 0 | normal 10 | batch |
| `ti_max_start` | no | 1 | 1 | normal 1 | batch |
| `ti_md` | no | 10 | 0 | normal 10 | batch |
| `ti_md_start` | no | 1 | 1 | normal 1 | batch |
| `ti_medprice` | no | 15 | 0 | normal 15 | batch |
| `ti_medprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_mfi` | no | 10 | 0 | normal 10 | batch |
| `ti_mfi_start` | no | 8 | 0 | normal 8 | batch |
| `ti_min` | no | 10 | 0 | normal 10 | batch |
| `ti_min_start` | no | 1 | 1 | normal 1 | batch |
| `ti_mom` | no | 10 | 0 | normal 10 | batch |
| `ti_mom_start` | no | 8 | 0 | normal 8 | batch |
| `ti_msw` | no | 10 | 0 | normal 10 | batch |
| `ti_msw_start` | no | 8 | 0 | normal 8 | batch |
| `ti_mul` | no | 15 | 0 | normal 15 | batch |
| `ti_mul_start` | no | 1 | 0 | normal 1 | batch |
| `ti_natr` | no | 9 | 0 | normal 9 | batch |
| `ti_natr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_nvi` | no | 17 | 0 | normal 17 | batch |
| `ti_nvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_obv` | no | 18 | 0 | normal 18 | batch |
| `ti_obv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ppo` | no | 13 | 0 | normal 13 | batch |
| `ti_ppo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_psar` | no | 10 | 0 | normal 10 | batch |
| `ti_psar_start` | no | 1 | 0 | normal 1 | batch |
| `ti_pvi` | no | 17 | 0 | normal 17 | batch |
| `ti_pvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_qstick` | no | 9 | 0 | normal 9 | batch |
| `ti_qstick_start` | no | 1 | 1 | normal 1 | batch |
| `ti_roc` | no | 10 | 0 | normal 10 | batch |
| `ti_roc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_rocr` | no | 10 | 0 | normal 10 | batch |
| `ti_rocr_start` | no | 8 | 0 | normal 8 | batch |
| `ti_round` | no | 18 | 0 | normal 18 | batch |
| `ti_round_start` | no | 1 | 0 | normal 1 | batch |
| `ti_rsi` | no | 10 | 0 | normal 10 | batch |
| `ti_rsi_start` | no | 8 | 0 | normal 8 | batch |
| `ti_sin` | no | 16 | 0 | normal 16 | batch |
| `ti_sin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sinh` | no | 20 | 0 | normal 20 | batch |
| `ti_sinh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sma` | no | 10 | 0 | normal 10 | batch |
| `ti_sma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sqrt` | no | 31 | 1 | normal 31 | batch |
| `ti_sqrt_start` | no | 1 | 0 | normal 1 | batch |
| `ti_stddev` | no | 10 | 1 | normal 10 | batch |
| `ti_stddev_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stderr` | no | 11 | 0 | normal 11 | batch |
| `ti_stderr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stoch` | no | 11 | 0 | normal 11 | batch |
| `ti_stoch_start` | no | 23 | 989 | normal 22, ub-gated 1 | batch |
| `ti_stochrsi` | no | 10 | 0 | normal 10 | batch |
| `ti_stochrsi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sub` | no | 14 | 0 | normal 14 | batch |
| `ti_sub_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sum` | no | 10 | 0 | normal 10 | batch |
| `ti_sum_start` | no | 1 | 1 | normal 1 | batch |
| `ti_tan` | no | 18 | 0 | normal 18 | batch |
| `ti_tan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tanh` | no | 18 | 0 | normal 18 | batch |
| `ti_tanh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tema` | no | 24 | 0 | normal 24 | batch |
| `ti_tema_start` | no | 1 | 1 | normal 1 | batch |
| `ti_todeg` | no | 33 | 0 | normal 33 | batch |
| `ti_todeg_start` | no | 1 | 0 | normal 1 | batch |
| `ti_torad` | no | 32 | 0 | normal 32 | batch |
| `ti_torad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tr` | no | 16 | 0 | normal 16 | batch |
| `ti_tr_start` | no | 1 | 0 | normal 1 | batch |
| `ti_trima` | no | 10 | 0 | normal 10 | batch |
| `ti_trima_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trix` | no | 10 | 0 | normal 10 | batch |
| `ti_trix_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trunc` | no | 33 | 0 | normal 16, ub-gated 17 | batch |
| `ti_trunc_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tsf` | no | 10 | 0 | normal 10 | batch |
| `ti_tsf_start` | no | 1 | 1 | normal 1 | batch |
| `ti_typprice` | no | 14 | 0 | normal 14 | batch |
| `ti_typprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ultosc` | no | 11 | 0 | normal 11 | batch |
| `ti_ultosc_start` | no | 23 | 0 | normal 23 | batch |
| `ti_var` | no | 10 | 0 | normal 10 | batch |
| `ti_var_start` | no | 1 | 1 | normal 1 | batch |
| `ti_version` | no | 1 | 0 | normal 1 | batch |
| `ti_vhf` | no | 10 | 0 | normal 10 | batch |
| `ti_vhf_start` | no | 8 | 0 | normal 8 | batch |
| `ti_vidya` | no | 10 | 0 | normal 10 | batch |
| `ti_vidya_start` | no | 1 | 1 | normal 1 | batch |
| `ti_volatility` | no | 11 | 0 | normal 11 | batch |
| `ti_volatility_start` | no | 8 | 0 | normal 8 | batch |
| `ti_vosc` | no | 10 | 0 | normal 10 | batch |
| `ti_vosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_vwma` | no | 9 | 1 | normal 9 | batch |
| `ti_vwma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wad` | no | 14 | 0 | normal 14 | batch |
| `ti_wad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_wcprice` | no | 14 | 0 | normal 14 | batch |
| `ti_wcprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_wilders` | no | 10 | 0 | normal 10 | batch |
| `ti_wilders_start` | no | 1 | 1 | normal 1 | batch |
| `ti_willr` | no | 9 | 0 | normal 9 | batch |
| `ti_willr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wma` | no | 10 | 0 | normal 10 | batch |
| `ti_wma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_zlema` | no | 10 | 0 | normal 10 | batch |
| `ti_zlema_start` | no | 1 | 1 | normal 1 | batch |

## 3. Tests side

Status **TEST-FAILS**. smoke panics in its own buffer test (smoke.rs:450, `index out of bounds: the len is 1 but the index is 1`): ti_buffer's flexible array member `vals[1]` became a fixed one-element array, so ti_buffer_new(size>1) cannot hold its ring; exit 134; denominator universe. The library-side boundaries ti_buffer_* take `Box<ti_buffer>` in this translation and are refused by the planner, so the campaign cannot reach the same defect directly -- recorded, not promoted

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 212 harnesses, 0 crash-all (none).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=65536`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 213 | 0 | 212 | 0 | 0 | 212 | 1 | 0.000 | 0.995 |
| regions | 9306 | 0 | 3167 | 0 | 0 | 3167 | 6139 | 0.000 | 0.340 |

Sanity checks: function pass, region pass. Harnesses unioned: 212. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `ti_abs` | 15 | 15 | 16 | 18 | 20 |
| `ti_acos` | 14 | 14 | 15 | 17 | 27 |
| `ti_ad` | 20 | 20 | 20 | 20 | 20 |
| `ti_add` | 15 | 15 | 15 | 16 | 16 |
| `ti_adosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_adx` | 9 | 9 | 9 | 9 | 9 |
| `ti_adx_start` | 9 | 9 | 9 | 9 | 9 |
| `ti_adxr` | 9 | 9 | 9 | 9 | 9 |
| `ti_ao` | 16 | 16 | 16 | 16 | 16 |
| `ti_apo` | 11 | 11 | 11 | 13 | 13 |
| `ti_aroon` | 9 | 9 | 9 | 9 | 9 |
| `ti_aroon_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_aroonosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_aroonosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_asin` | 15 | 17 | 18 | 18 | 32 |
| `ti_atan` | 15 | 15 | 16 | 16 | 31 |
| `ti_atr` | 9 | 9 | 9 | 9 | 9 |
| `ti_avgprice` | 15 | 15 | 15 | 15 | 15 |
| `ti_bbands` | 10 | 10 | 10 | 10 | 10 |
| `ti_bop` | 17 | 17 | 17 | 17 | 17 |
| `ti_cci` | 11 | 11 | 11 | 11 | 12 |
| `ti_ceil` | 15 | 16 | 16 | 16 | 31 |
| `ti_cmo` | 10 | 10 | 10 | 10 | 10 |
| `ti_cmo_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_cos` | 14 | 15 | 15 | 17 | 28 |
| `ti_cosh` | 14 | 14 | 14 | 14 | 32 |
| `ti_crossany` | 24 | 24 | 24 | 24 | 26 |
| `ti_crossover` | 18 | 18 | 18 | 18 | 18 |
| `ti_cvi` | 9 | 9 | 9 | 9 | 9 |
| `ti_decay` | 16 | 16 | 16 | 16 | 16 |
| `ti_dema` | 10 | 11 | 11 | 13 | 13 |
| `ti_di` | 9 | 9 | 9 | 9 | 9 |
| `ti_div` | 15 | 15 | 15 | 15 | 15 |
| `ti_dm` | 9 | 9 | 9 | 9 | 9 |
| `ti_dpo` | 10 | 10 | 10 | 10 | 10 |
| `ti_dx` | 9 | 9 | 9 | 9 | 9 |
| `ti_edecay` | 15 | 15 | 15 | 15 | 15 |
| `ti_ema` | 10 | 10 | 11 | 13 | 13 |
| `ti_emv` | 15 | 15 | 15 | 15 | 15 |
| `ti_exp` | 15 | 15 | 16 | 16 | 16 |
| `ti_find_indicator` | 18 | 19 | 19 | 19 | 19 |
| `ti_fisher` | 10 | 10 | 10 | 10 | 10 |
| `ti_floor` | 13 | 14 | 16 | 16 | 28 |
| `ti_fosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_fosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_hma` | 10 | 10 | 10 | 10 | 10 |
| `ti_kama` | 10 | 10 | 10 | 10 | 10 |
| `ti_kvo` | 10 | 10 | 10 | 10 | 10 |
| `ti_lag` | 17 | 17 | 17 | 17 | 17 |
| `ti_lag_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_linreg` | 10 | 10 | 10 | 10 | 10 |
| `ti_linregintercept` | 10 | 10 | 10 | 10 | 10 |
| `ti_linregslope` | 10 | 10 | 10 | 10 | 10 |
| `ti_ln` | 13 | 14 | 14 | 16 | 16 |
| `ti_log10` | 14 | 15 | 15 | 17 | 17 |
| `ti_macd` | 11 | 12 | 12 | 14 | 14 |
| `ti_marketfi` | 15 | 15 | 15 | 15 | 15 |
| `ti_mass` | 11 | 11 | 11 | 11 | 11 |
| `ti_max` | 10 | 10 | 10 | 10 | 10 |
| `ti_md` | 10 | 10 | 10 | 10 | 10 |
| `ti_medprice` | 15 | 15 | 15 | 15 | 15 |
| `ti_mfi` | 10 | 10 | 10 | 10 | 10 |
| `ti_mfi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_min` | 10 | 10 | 10 | 10 | 10 |
| `ti_mom` | 10 | 10 | 10 | 10 | 10 |
| `ti_mom_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_msw` | 10 | 10 | 10 | 10 | 10 |
| `ti_msw_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_mul` | 15 | 15 | 15 | 15 | 15 |
| `ti_natr` | 9 | 9 | 9 | 9 | 9 |
| `ti_nvi` | 17 | 17 | 17 | 17 | 17 |
| `ti_obv` | 18 | 18 | 18 | 18 | 18 |
| `ti_ppo` | 11 | 11 | 11 | 13 | 13 |
| `ti_psar` | 10 | 10 | 10 | 10 | 10 |
| `ti_pvi` | 16 | 16 | 16 | 16 | 17 |
| `ti_qstick` | 9 | 9 | 9 | 9 | 9 |
| `ti_roc` | 10 | 10 | 10 | 10 | 10 |
| `ti_roc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_rocr` | 10 | 10 | 10 | 10 | 10 |
| `ti_rocr_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_round` | 15 | 15 | 18 | 18 | 18 |
| `ti_rsi` | 10 | 10 | 10 | 10 | 10 |
| `ti_rsi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_sin` | 15 | 15 | 16 | 16 | 16 |
| `ti_sinh` | 16 | 17 | 17 | 19 | 20 |
| `ti_sma` | 10 | 10 | 10 | 10 | 10 |
| `ti_sqrt` | 15 | 16 | 16 | 18 | 31 |
| `ti_stddev` | 10 | 10 | 10 | 10 | 10 |
| `ti_stderr` | 10 | 10 | 10 | 10 | 11 |
| `ti_stoch` | 11 | 11 | 11 | 11 | 11 |
| `ti_stoch_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_stochrsi` | 10 | 10 | 10 | 10 | 10 |
| `ti_sub` | 14 | 14 | 14 | 14 | 14 |
| `ti_sum` | 10 | 10 | 10 | 10 | 10 |
| `ti_tan` | 15 | 16 | 17 | 18 | 18 |
| `ti_tanh` | 15 | 16 | 16 | 16 | 18 |
| `ti_tema` | 10 | 11 | 11 | 13 | 24 |
| `ti_todeg` | 15 | 15 | 15 | 17 | 33 |
| `ti_torad` | 14 | 14 | 17 | 17 | 32 |
| `ti_tr` | 16 | 16 | 16 | 16 | 16 |
| `ti_trima` | 10 | 10 | 10 | 10 | 10 |
| `ti_trix` | 10 | 10 | 10 | 10 | 10 |
| `ti_trunc` | 15 | 18 | 18 | 18 | 33 |
| `ti_tsf` | 10 | 10 | 10 | 10 | 10 |
| `ti_typprice` | 14 | 14 | 14 | 14 | 14 |
| `ti_ultosc` | 11 | 11 | 11 | 11 | 11 |
| `ti_ultosc_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_var` | 10 | 10 | 10 | 10 | 10 |
| `ti_vhf` | 10 | 10 | 10 | 10 | 10 |
| `ti_vhf_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vidya` | 10 | 10 | 10 | 10 | 10 |
| `ti_volatility` | 10 | 10 | 10 | 10 | 11 |
| `ti_volatility_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_vwma` | 9 | 9 | 9 | 9 | 9 |
| `ti_wad` | 14 | 14 | 14 | 14 | 14 |
| `ti_wcprice` | 14 | 14 | 14 | 14 | 14 |
| `ti_wilders` | 10 | 10 | 10 | 10 | 10 |
| `ti_willr` | 9 | 9 | 9 | 9 | 9 |
| `ti_wma` | 10 | 10 | 10 | 10 | 10 |
| `ti_zlema` | 10 | 10 | 10 | 10 | 10 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1749 |
| ub-gated | 18 |
| divergence | 6 |
| panic | 3 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ti_adosc_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_adx_start` | 12 / 12 | confirmed_divergence 5, confirmed_termination 4, instrument_only 3 | 2 |
| `ti_adxr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_atr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_bbands_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_buffer_new` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_cci_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_cvi_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dema_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_di_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dm_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dpo_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dx_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_fisher_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_hma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_kama_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_linreg_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_linregintercept` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_linregintercept_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_linregslope_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_macd_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_mass_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_max_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_md_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_min_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_natr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_qstick_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_sma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_sqrt` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_stddev` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_stddev_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_stderr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_stoch_start` | 200 / 500 | ub_associated_termination 200 | 1 |
| `ti_stochrsi_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_sum_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_tema_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_trima_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_trix_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_tsf_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_var_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_vidya_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_vosc_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_vwma` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_vwma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wilders_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_willr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_zlema_start` | 1 / 1 | ub_associated_termination 1 | 1 |

Total: confirmed_divergence 5, confirmed_termination 4, instrument_only 3, not_reproducible 4, ub_associated_termination 242  *(re-classified offline 2026-09-09: 3 rows `confirmed_termination` → `instrument_only`, no-sanitizer replay normal; see §7)*

<!-- prose -->
## 7. What this cell is, and is not

**One new defect, S15.** C2SaferRust's tulip fails its own smoke (a panic in the buffer test: the
flexible array `vals[1]` became a one-element array), so the universe is the denominator. 212 planned
(`ti_buffer_new` takes `Box<ti_buffer>`, refused), 212 built, preflight clean, 3 600 s, replay 1 749
`normal` + **6 `divergence` + 3 `panic`** + 18 `ub-gated`. Confirmation sample: **5 `confirmed_divergence`
+ 7 `confirmed_termination`, all on `ti_adx_start`**, 242 `ub_associated_termination`, 4
`not_reproducible`. The site: C's `((int)options[0]-1) * 2` became `(options.offset(0) as i32 - 1) * 2`
(`tulip_c2saferrust.rs:393`) — the **pointer** is cast to an integer, the value is never loaded, so the
start offset is the low 32 bits of a heap address: wrong on every valid input, and an
`attempt to multiply with overflow` panic when those bits are large. C is in contract on all twelve.

**One site, checked, not a family.** A text search for `options.offset(0) as i32` matched 19 `_start`
functions; a deterministic probe (options[0] = 5.0, combined replay on the cell's own binaries) showed 18
of them agree with C — the pattern had not excluded the leading `*` of a correct dereference. Only
`ti_adx_start` diverges. E1's tulip × C2SaferRust evidence (C6, S13, the 150 000-record indicator-value
certificate through the sample driver) never called `ti_adx_start`.

**Coverage:** functions 212/213, regions 3 167 / 9 306 (0.340) — the same option-domain ceiling as the
two paired cells (`c2rust/RUN.md` §7).

**Deviations:** the archive step was re-run after the subdirectory-hash fix.

**Not established:** whether `ti_adx`'s own output is wrong downstream of the wrong start (its
boundary's corpus never passed the option domain); anything about `ti_buffer_*`.

**Re-classified 2026-09-09.** 3 of the 7 `ti_adx_start` rows labelled `confirmed_termination` returned normally in the no-sanitizer replay (the classifier had not consulted channel D for a `combined` panic); they are now `instrument_only`. S15 stands on the 5 confirmed_divergence rows and the 4 terminations that trap with no sanitizer; its manifest wording was corrected.
