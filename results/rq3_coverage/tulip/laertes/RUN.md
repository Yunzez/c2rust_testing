# tulip × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 213 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 213 |
| built | 212 |
| executed (corpus > 0) | 212 |
| coverage exported | 211 |

Planned but not built:

- `ti_buffer_free`: ^^^^^^^^^^^^^^^^^^^^^
  File "/home/yunzez/c2rust_testing/tools/stu_selector/harness_plan.py", line 2370, in lower_to_sc

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `ti_abs` | no | 18 | 1 | normal 18 | batch |
| `ti_abs_start` | no | 1 | 0 | normal 1 | batch |
| `ti_acos` | no | 20 | 0 | normal 20 | batch |
| `ti_acos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ad` | no | 21 | 0 | normal 21 | batch |
| `ti_ad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_add` | no | 19 | 0 | normal 19 | batch |
| `ti_add_start` | no | 1 | 0 | normal 1 | batch |
| `ti_adosc` | no | 10 | 1 | normal 10 | batch |
| `ti_adosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_adx` | no | 9 | 0 | normal 9 | batch |
| `ti_adx_start` | no | 1 | 1 | normal 1 | batch |
| `ti_adxr` | no | 9 | 0 | normal 9 | batch |
| `ti_adxr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ao` | no | 18 | 0 | normal 18 | batch |
| `ti_ao_start` | no | 1 | 0 | normal 1 | batch |
| `ti_apo` | no | 24 | 0 | normal 24 | batch |
| `ti_apo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_aroon` | no | 10 | 1 | normal 10 | batch |
| `ti_aroon_start` | no | 8 | 0 | normal 8 | batch |
| `ti_aroonosc` | no | 10 | 1 | normal 10 | batch |
| `ti_aroonosc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_asin` | no | 20 | 1 | normal 20 | batch |
| `ti_asin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atan` | no | 18 | 1 | normal 18 | batch |
| `ti_atan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atr` | no | 9 | 1 | normal 9 | batch |
| `ti_atr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_avgprice` | no | 18 | 0 | normal 18 | batch |
| `ti_avgprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_bbands` | no | 11 | 0 | normal 11 | batch |
| `ti_bbands_start` | no | 1 | 1 | normal 1 | batch |
| `ti_bop` | no | 17 | 0 | normal 17 | batch |
| `ti_bop_start` | no | 1 | 0 | normal 1 | batch |
| `ti_buffer_new` | no | 1 | 1 | normal 1 | batch |
| `ti_build` | no | 1 | 0 | normal 1 | batch |
| `ti_cci` | no | 11 | 1 | normal 11 | batch |
| `ti_cci_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ceil` | no | 33 | 1 | normal 33 | batch |
| `ti_ceil_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cmo` | no | 11 | 0 | normal 11 | batch |
| `ti_cmo_start` | no | 8 | 0 | normal 8 | batch |
| `ti_cos` | no | 20 | 0 | normal 20 | batch |
| `ti_cos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cosh` | no | 34 | 0 | normal 34 | batch |
| `ti_cosh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossany` | no | 36 | 0 | normal 36 | batch |
| `ti_crossany_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossover` | no | 21 | 0 | normal 21 | batch |
| `ti_crossover_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cvi` | no | 10 | 0 | normal 10 | batch |
| `ti_cvi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_decay` | no | 16 | 0 | normal 16 | batch |
| `ti_decay_start` | no | 1 | 1 | normal 1 | batch |
| `ti_dema` | no | 22 | 1 | normal 22 | batch |
| `ti_dema_start` | no | 1 | 1 | normal 1 | batch |
| `ti_di` | no | 9 | 0 | normal 9 | batch |
| `ti_di_start` | no | 1 | 1 | normal 1 | batch |
| `ti_div` | no | 20 | 1 | normal 20 | batch |
| `ti_div_start` | no | 1 | 0 | normal 1 | batch |
| `ti_dm` | no | 9 | 1 | normal 9 | batch |
| `ti_dm_start` | no | 1 | 1 | normal 1 | batch |
| `ti_dpo` | no | 10 | 1 | normal 10 | batch |
| `ti_dpo_start` | no | 1 | 1 | normal 1 | batch |
| `ti_dx` | no | 9 | 0 | normal 9 | batch |
| `ti_dx_start` | no | 1 | 1 | normal 1 | batch |
| `ti_edecay` | no | 16 | 0 | normal 16 | batch |
| `ti_edecay_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ema` | no | 15 | 0 | normal 15 | batch |
| `ti_ema_start` | no | 1 | 0 | normal 1 | batch |
| `ti_emv` | no | 16 | 0 | normal 16 | batch |
| `ti_emv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_exp` | no | 20 | 0 | normal 20 | batch |
| `ti_exp_start` | no | 1 | 0 | normal 1 | batch |
| `ti_find_indicator` | no | 1 | 2 | signal 1 | failed rc=1 |
| `ti_fisher` | no | 9 | 1 | normal 9 | batch |
| `ti_fisher_start` | no | 1 | 1 | normal 1 | batch |
| `ti_floor` | no | 19 | 0 | normal 19 | batch |
| `ti_floor_start` | no | 1 | 0 | normal 1 | batch |
| `ti_fosc` | no | 11 | 0 | normal 11 | batch |
| `ti_fosc_start` | no | 8 | 1 | normal 8 | batch |
| `ti_hma` | no | 11 | 1 | normal 11 | batch |
| `ti_hma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kama` | no | 11 | 0 | normal 11 | batch |
| `ti_kama_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kvo` | no | 10 | 1 | normal 10 | batch |
| `ti_kvo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_lag` | no | 19 | 0 | normal 19 | batch |
| `ti_lag_start` | no | 8 | 0 | normal 8 | batch |
| `ti_linreg` | no | 10 | 0 | normal 10 | batch |
| `ti_linreg_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregintercept` | no | 11 | 1 | normal 11 | batch |
| `ti_linregintercept_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregslope` | no | 11 | 0 | normal 11 | batch |
| `ti_linregslope_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ln` | no | 28 | 1 | normal 28 | batch |
| `ti_ln_start` | no | 1 | 1 | normal 1 | batch |
| `ti_log10` | no | 20 | 0 | normal 20 | batch |
| `ti_log10_start` | no | 1 | 0 | normal 1 | batch |
| `ti_macd` | no | 15 | 0 | normal 15 | batch |
| `ti_macd_start` | no | 1 | 1 | normal 1 | batch |
| `ti_marketfi` | no | 15 | 0 | normal 15 | batch |
| `ti_marketfi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_mass` | no | 13 | 0 | normal 13 | batch |
| `ti_mass_start` | no | 1 | 1 | normal 1 | batch |
| `ti_max` | no | 12 | 0 | normal 12 | batch |
| `ti_max_start` | no | 1 | 1 | normal 1 | batch |
| `ti_md` | no | 11 | 0 | normal 11 | batch |
| `ti_md_start` | no | 1 | 1 | normal 1 | batch |
| `ti_medprice` | no | 15 | 0 | normal 15 | batch |
| `ti_medprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_mfi` | no | 10 | 0 | normal 10 | batch |
| `ti_mfi_start` | no | 8 | 0 | normal 8 | batch |
| `ti_min` | no | 11 | 0 | normal 11 | batch |
| `ti_min_start` | no | 1 | 1 | normal 1 | batch |
| `ti_mom` | no | 10 | 0 | normal 10 | batch |
| `ti_mom_start` | no | 8 | 0 | normal 8 | batch |
| `ti_msw` | no | 11 | 1 | normal 11 | batch |
| `ti_msw_start` | no | 8 | 1 | normal 8 | batch |
| `ti_mul` | no | 15 | 0 | normal 15 | batch |
| `ti_mul_start` | no | 1 | 1 | normal 1 | batch |
| `ti_natr` | no | 9 | 0 | normal 9 | batch |
| `ti_natr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_nvi` | no | 17 | 0 | normal 17 | batch |
| `ti_nvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_obv` | no | 29 | 0 | normal 29 | batch |
| `ti_obv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ppo` | no | 22 | 0 | normal 22 | batch |
| `ti_ppo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_psar` | no | 13 | 0 | normal 13 | batch |
| `ti_psar_start` | no | 1 | 0 | normal 1 | batch |
| `ti_pvi` | no | 16 | 0 | normal 16 | batch |
| `ti_pvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_qstick` | no | 9 | 1 | normal 9 | batch |
| `ti_qstick_start` | no | 1 | 1 | normal 1 | batch |
| `ti_roc` | no | 11 | 0 | normal 11 | batch |
| `ti_roc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_rocr` | no | 11 | 0 | normal 11 | batch |
| `ti_rocr_start` | no | 8 | 0 | normal 8 | batch |
| `ti_round` | no | 31 | 1 | normal 31 | batch |
| `ti_round_start` | no | 1 | 0 | normal 1 | batch |
| `ti_rsi` | no | 11 | 0 | normal 11 | batch |
| `ti_rsi_start` | no | 8 | 0 | normal 8 | batch |
| `ti_sin` | no | 21 | 1 | normal 21 | batch |
| `ti_sin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sinh` | no | 20 | 0 | normal 20 | batch |
| `ti_sinh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sma` | no | 11 | 0 | normal 11 | batch |
| `ti_sma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sqrt` | no | 19 | 1 | normal 19 | batch |
| `ti_sqrt_start` | no | 1 | 0 | normal 1 | batch |
| `ti_stddev` | no | 11 | 0 | normal 11 | batch |
| `ti_stddev_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stderr` | no | 11 | 0 | normal 11 | batch |
| `ti_stderr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stoch` | no | 11 | 0 | normal 11 | batch |
| `ti_stoch_start` | no | 23 | 1076 | normal 22, ub-gated 1 | batch |
| `ti_stochrsi` | no | 11 | 1 | normal 11 | batch |
| `ti_stochrsi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sub` | no | 15 | 0 | normal 15 | batch |
| `ti_sub_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sum` | no | 11 | 0 | normal 11 | batch |
| `ti_sum_start` | no | 1 | 1 | normal 1 | batch |
| `ti_tan` | no | 20 | 1 | normal 20 | batch |
| `ti_tan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tanh` | no | 20 | 0 | normal 20 | batch |
| `ti_tanh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tema` | no | 15 | 1 | normal 15 | batch |
| `ti_tema_start` | no | 1 | 1 | normal 1 | batch |
| `ti_todeg` | no | 18 | 0 | normal 18 | batch |
| `ti_todeg_start` | no | 1 | 0 | normal 1 | batch |
| `ti_torad` | no | 19 | 0 | normal 19 | batch |
| `ti_torad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tr` | no | 16 | 0 | normal 16 | batch |
| `ti_tr_start` | no | 1 | 0 | normal 1 | batch |
| `ti_trima` | no | 11 | 1 | normal 11 | batch |
| `ti_trima_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trix` | no | 11 | 0 | normal 11 | batch |
| `ti_trix_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trunc` | no | 31 | 0 | normal 14, ub-gated 17 | batch |
| `ti_trunc_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tsf` | no | 10 | 0 | normal 10 | batch |
| `ti_tsf_start` | no | 1 | 1 | normal 1 | batch |
| `ti_typprice` | no | 16 | 1 | normal 16 | batch |
| `ti_typprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ultosc` | no | 12 | 0 | normal 12 | batch |
| `ti_ultosc_start` | no | 23 | 0 | normal 23 | batch |
| `ti_var` | no | 11 | 0 | normal 11 | batch |
| `ti_var_start` | no | 1 | 1 | normal 1 | batch |
| `ti_version` | no | 1 | 0 | normal 1 | batch |
| `ti_vhf` | no | 11 | 0 | normal 11 | batch |
| `ti_vhf_start` | no | 8 | 0 | normal 8 | batch |
| `ti_vidya` | no | 11 | 0 | normal 11 | batch |
| `ti_vidya_start` | no | 1 | 1 | normal 1 | batch |
| `ti_volatility` | no | 10 | 0 | normal 10 | batch |
| `ti_volatility_start` | no | 8 | 1 | normal 8 | batch |
| `ti_vosc` | no | 11 | 1 | normal 11 | batch |
| `ti_vosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_vwma` | no | 9 | 0 | normal 9 | batch |
| `ti_vwma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wad` | no | 19 | 0 | normal 19 | batch |
| `ti_wad_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wcprice` | no | 20 | 0 | normal 20 | batch |
| `ti_wcprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_wilders` | no | 11 | 0 | normal 11 | batch |
| `ti_wilders_start` | no | 1 | 1 | normal 1 | batch |
| `ti_willr` | no | 9 | 0 | normal 9 | batch |
| `ti_willr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wma` | no | 11 | 0 | normal 11 | batch |
| `ti_wma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_zlema` | no | 11 | 0 | normal 11 | batch |
| `ti_zlema_start` | no | 1 | 1 | normal 1 | batch |

## 3. Tests side

Status **TEST-FAILS**. smoke SIGSEGV (exit 139) before any output; denominator universe

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`, `a8925ba6292dd9ac (bin reused from the killed run)` — more than one: see deviations.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 216 | 0 | 211 | 0 | 0 | 211 | 5 | 0.000 | 0.977 |
| regions | 13191 | 0 | 3168 | 0 | 0 | 3168 | 10023 | 0.000 | 0.240 |

Sanity checks: function pass, region pass. Harnesses unioned: 211. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `ti_abs` | 13 | 14 | 16 | 18 | 18 |
| `ti_acos` | 15 | 18 | 19 | 20 | 20 |
| `ti_ad` | 20 | 20 | 20 | 20 | 21 |
| `ti_add` | 14 | 15 | 15 | 15 | 19 |
| `ti_adosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_adx` | 9 | 9 | 9 | 9 | 9 |
| `ti_adxr` | 9 | 9 | 9 | 9 | 9 |
| `ti_ao` | 17 | 17 | 17 | 17 | 18 |
| `ti_apo` | 10 | 11 | 11 | 14 | 24 |
| `ti_aroon` | 10 | 10 | 10 | 10 | 10 |
| `ti_aroon_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_aroonosc` | 10 | 10 | 10 | 10 | 10 |
| `ti_aroonosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_asin` | 15 | 18 | 18 | 19 | 20 |
| `ti_atan` | 15 | 17 | 17 | 18 | 18 |
| `ti_atr` | 9 | 9 | 9 | 9 | 9 |
| `ti_avgprice` | 17 | 17 | 17 | 17 | 18 |
| `ti_bbands` | 10 | 10 | 10 | 10 | 11 |
| `ti_bop` | 17 | 17 | 17 | 17 | 17 |
| `ti_cci` | 11 | 11 | 11 | 11 | 11 |
| `ti_ceil` | 15 | 16 | 18 | 20 | 33 |
| `ti_cmo` | 10 | 10 | 10 | 10 | 11 |
| `ti_cmo_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_cos` | 16 | 17 | 19 | 20 | 20 |
| `ti_cosh` | 16 | 17 | 18 | 19 | 34 |
| `ti_crossany` | 24 | 26 | 27 | 27 | 36 |
| `ti_crossover` | 19 | 19 | 21 | 21 | 21 |
| `ti_cvi` | 10 | 10 | 10 | 10 | 10 |
| `ti_decay` | 16 | 16 | 16 | 16 | 16 |
| `ti_dema` | 10 | 13 | 13 | 15 | 22 |
| `ti_di` | 9 | 9 | 9 | 9 | 9 |
| `ti_div` | 15 | 15 | 15 | 15 | 20 |
| `ti_dm` | 9 | 9 | 9 | 9 | 9 |
| `ti_dpo` | 10 | 10 | 10 | 10 | 10 |
| `ti_dx` | 9 | 9 | 9 | 9 | 9 |
| `ti_edecay` | 16 | 16 | 16 | 16 | 16 |
| `ti_ema` | 11 | 13 | 13 | 15 | 15 |
| `ti_emv` | 15 | 15 | 15 | 16 | 16 |
| `ti_exp` | 15 | 18 | 18 | 20 | 20 |
| `ti_fisher` | 9 | 9 | 9 | 9 | 9 |
| `ti_floor` | 15 | 17 | 18 | 19 | 19 |
| `ti_fosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_fosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_hma` | 10 | 10 | 10 | 11 | 11 |
| `ti_kama` | 10 | 10 | 10 | 10 | 11 |
| `ti_kvo` | 10 | 10 | 10 | 10 | 10 |
| `ti_lag` | 18 | 18 | 18 | 18 | 19 |
| `ti_lag_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_linreg` | 10 | 10 | 10 | 10 | 10 |
| `ti_linregintercept` | 10 | 10 | 10 | 10 | 11 |
| `ti_linregslope` | 10 | 10 | 10 | 10 | 11 |
| `ti_ln` | 15 | 18 | 18 | 19 | 28 |
| `ti_log10` | 16 | 18 | 18 | 20 | 20 |
| `ti_macd` | 11 | 12 | 14 | 15 | 15 |
| `ti_marketfi` | 14 | 14 | 14 | 14 | 15 |
| `ti_mass` | 10 | 11 | 11 | 11 | 13 |
| `ti_max` | 10 | 10 | 10 | 10 | 12 |
| `ti_md` | 10 | 10 | 10 | 10 | 11 |
| `ti_medprice` | 15 | 15 | 15 | 15 | 15 |
| `ti_mfi` | 10 | 10 | 10 | 10 | 10 |
| `ti_mfi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_min` | 10 | 10 | 10 | 10 | 11 |
| `ti_mom` | 10 | 10 | 10 | 10 | 10 |
| `ti_mom_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_msw` | 10 | 10 | 10 | 10 | 11 |
| `ti_msw_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_mul` | 15 | 15 | 15 | 15 | 15 |
| `ti_natr` | 9 | 9 | 9 | 9 | 9 |
| `ti_nvi` | 16 | 17 | 17 | 17 | 17 |
| `ti_obv` | 18 | 18 | 18 | 18 | 29 |
| `ti_ppo` | 10 | 11 | 13 | 15 | 22 |
| `ti_psar` | 10 | 10 | 10 | 10 | 13 |
| `ti_pvi` | 15 | 15 | 15 | 16 | 16 |
| `ti_qstick` | 9 | 9 | 9 | 9 | 9 |
| `ti_roc` | 10 | 10 | 10 | 10 | 11 |
| `ti_roc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_rocr` | 10 | 10 | 10 | 10 | 11 |
| `ti_rocr_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_round` | 16 | 18 | 18 | 20 | 31 |
| `ti_rsi` | 10 | 10 | 10 | 10 | 11 |
| `ti_rsi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_sin` | 15 | 18 | 18 | 20 | 21 |
| `ti_sinh` | 15 | 17 | 18 | 20 | 20 |
| `ti_sma` | 10 | 10 | 10 | 10 | 11 |
| `ti_sqrt` | 14 | 15 | 15 | 19 | 19 |
| `ti_stddev` | 10 | 10 | 10 | 10 | 11 |
| `ti_stderr` | 10 | 10 | 10 | 10 | 11 |
| `ti_stoch` | 11 | 11 | 11 | 11 | 11 |
| `ti_stoch_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_stochrsi` | 10 | 10 | 10 | 11 | 11 |
| `ti_sub` | 14 | 14 | 15 | 15 | 15 |
| `ti_sum` | 10 | 10 | 10 | 10 | 11 |
| `ti_tan` | 16 | 19 | 19 | 20 | 20 |
| `ti_tanh` | 16 | 18 | 18 | 20 | 20 |
| `ti_tema` | 10 | 13 | 13 | 15 | 15 |
| `ti_todeg` | 14 | 17 | 18 | 18 | 18 |
| `ti_torad` | 14 | 17 | 17 | 18 | 19 |
| `ti_tr` | 16 | 16 | 16 | 16 | 16 |
| `ti_trima` | 10 | 10 | 10 | 10 | 11 |
| `ti_trix` | 10 | 10 | 10 | 10 | 11 |
| `ti_trunc` | 14 | 17 | 17 | 19 | 31 |
| `ti_tsf` | 10 | 10 | 10 | 10 | 10 |
| `ti_typprice` | 16 | 16 | 16 | 16 | 16 |
| `ti_ultosc` | 11 | 11 | 11 | 11 | 12 |
| `ti_ultosc_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_var` | 10 | 10 | 10 | 10 | 11 |
| `ti_vhf` | 10 | 10 | 10 | 10 | 11 |
| `ti_vhf_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vidya` | 10 | 10 | 10 | 10 | 11 |
| `ti_volatility` | 10 | 10 | 10 | 10 | 10 |
| `ti_volatility_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_vwma` | 9 | 9 | 9 | 9 | 9 |
| `ti_wad` | 15 | 15 | 15 | 15 | 19 |
| `ti_wcprice` | 15 | 15 | 15 | 15 | 20 |
| `ti_wilders` | 10 | 10 | 10 | 10 | 11 |
| `ti_willr` | 9 | 9 | 9 | 9 | 9 |
| `ti_wma` | 10 | 10 | 10 | 10 | 11 |
| `ti_zlema` | 10 | 10 | 10 | 10 | 11 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1794 |
| ub-gated | 18 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ti_abs` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_adosc` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_adosc_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_adx_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_adxr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_aroon` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_aroonosc` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_asin` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_atan` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_atr` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_atr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_bbands_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_buffer_new` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_cci` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_cci_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_ceil` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_cvi_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_decay_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_dema` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_dema_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_di_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_div` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_dm` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_dm_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dpo` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_dpo_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_dx_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_find_indicator` | 3 / 3 | confirmed_termination 2 | 1 |
| `ti_fisher` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_fisher_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_fosc_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_hma` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_hma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_kama_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_kvo` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_linreg_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_linregintercept` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_linregintercept_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_linregslope_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_ln` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_ln_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_macd_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_mass_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_max_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_md_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_min_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_msw` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_msw_start` | 1 / 1 | ub_associated 1 | 1 |
| `ti_mul_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_natr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_qstick` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_qstick_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_round` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_sin` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_sma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_sqrt` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_stddev_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_stderr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_stoch_start` | 200 / 500 | ub_associated_termination 200 | 1 |
| `ti_stochrsi` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_stochrsi_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_sum_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_tan` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_tema` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_tema_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_trima` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_trima_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_trix_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_tsf_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_typprice` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_var_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_vidya_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_volatility_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_vosc` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_vosc_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_vwma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wad_start` | 1 / 1 | not_reproducible 1 | 1 |
| `ti_wilders_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_willr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_zlema_start` | 1 / 1 | ub_associated_termination 1 | 1 |

Total: confirmed_termination 2, not_reproducible 35, ub_associated 1, ub_associated_termination 243

<!-- prose -->


## 7. What this cell is, and is not

**A defect on a certified cell, predicted by the scanner, caught by the preflight.** Laertes' tulip fails
its own smoke (SIGSEGV before any output), so the universe is the denominator. The cell's first start was
stopped by the preflight: `ti_find_indicator` crashed on the empty input and on 129 of 130 test jobs on
the Rust side while the C side was clean. Source: Laertes renders the static table `ti_indicators[105]`
as 105 `ti_indicator_info::new()` — every `name` pointer NULL — with the real initialiser moved into a
`laertes_init_*` function nothing calls (the severed-init pattern the scanner had already flagged for
exactly this table; E3 had worked around it by calling the initialiser through `Once`). The boundary
was accepted in `preflight_accept.txt` as a translation-side crash-all and the cell re-run with its 212
built harnesses (`--reuse-bins`, chain 5). Replay: 1 794 `normal`, **1 `signal`**, 18 `ub-gated`;
confirmation sample: **2 `confirmed_termination` on `ti_find_indicator`** (C clean under ASan+UBSan,
the translation alone faults on the zero page with and without a sanitizer: `strcmp(name, NULL)` in the
binary search), 243 `ub_associated_termination`, 35 `not_reproducible`, 1 `ub_associated`. **C11 in the
manifest.** E1's Laertes tulip was a *certificate* (11 arithmetic indicators, 150 000 records, 0 diffs)
that never touched the lookup table.

**The zero-page rule.** The first adjudication had labelled these `out_of_contract_access` (a raw signal
without a Rust panic: "a wild read faults only if its page is unmapped"). A fault on the zero page is not
layout luck — that page is never mapped, on either side, on any layout — so `classify` now records the
ASan hint ("address points to the zero page", or an address below 0x1000) and returns
`confirmed_termination` for it; the three candidates were re-adjudicated under that rule after the cell's
post-processing, and this section records that the archived verdicts are the re-adjudicated ones
(`confirm_sample/summary.json` marks the boundary `reclassified`).

**Coverage:** functions 211/216, regions 3 168 / 13 191 (0.240): the universe carries Laertes' own runtime
(`laertes_init_*`, `__laertes_array`) as with lil, so the fraction is lower than the two paired cells'
0.34 for a reason unrelated to exploration; the only-ours count (3 168 regions) is the comparable number,
and the option-domain ceiling applies here too (`c2rust/RUN.md` §7).

**Deviations:** the preflight stop and re-run above; the archive step re-run after the subdirectory-hash
fix; the re-adjudication of one boundary.

**Not established:** any behaviour of the 104 indicators beyond the option domain; whether other
severed initialisers in this crate (9 `laertes_init_*` functions) affect boundaries the campaign reached
without crashing.
