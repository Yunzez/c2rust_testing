# tulip × crown — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 213 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 213 |
| built | 212 |
| executed (corpus > 0) | 212 |
| coverage exported | 212 |

Planned but not built:

- `ti_buffer_free`: ^^^^^^^^^^^^^^^^^^^^^
  File "/home/yunzez/c2rust_testing/tools/stu_selector/harness_plan.py", line 2370, in lower_to_sc

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `ti_abs` | no | 19 | 0 | normal 19 | batch |
| `ti_abs_start` | no | 1 | 0 | normal 1 | batch |
| `ti_acos` | no | 36 | 0 | normal 36 | batch |
| `ti_acos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ad` | no | 20 | 0 | normal 20 | batch |
| `ti_ad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_add` | no | 15 | 0 | normal 15 | batch |
| `ti_add_start` | no | 1 | 0 | normal 1 | batch |
| `ti_adosc` | no | 11 | 0 | normal 11 | batch |
| `ti_adosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_adx` | no | 9 | 0 | normal 9 | batch |
| `ti_adx_start` | no | 1 | 1 | normal 1 | batch |
| `ti_adxr` | no | 9 | 0 | normal 9 | batch |
| `ti_adxr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ao` | no | 17 | 0 | normal 17 | batch |
| `ti_ao_start` | no | 1 | 0 | normal 1 | batch |
| `ti_apo` | no | 24 | 0 | normal 24 | batch |
| `ti_apo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_aroon` | no | 9 | 0 | normal 9 | batch |
| `ti_aroon_start` | no | 8 | 0 | normal 8 | batch |
| `ti_aroonosc` | no | 9 | 0 | normal 9 | batch |
| `ti_aroonosc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_asin` | no | 19 | 0 | normal 19 | batch |
| `ti_asin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atan` | no | 30 | 0 | normal 30 | batch |
| `ti_atan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_atr` | no | 9 | 0 | normal 9 | batch |
| `ti_atr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_avgprice` | no | 14 | 0 | normal 14 | batch |
| `ti_avgprice_start` | no | 1 | 0 | normal 1 | batch |
| `ti_bbands` | no | 10 | 0 | normal 10 | batch |
| `ti_bbands_start` | no | 1 | 1 | normal 1 | batch |
| `ti_bop` | no | 18 | 0 | normal 18 | batch |
| `ti_bop_start` | no | 1 | 0 | normal 1 | batch |
| `ti_buffer_new` | no | 1 | 1 | normal 1 | batch |
| `ti_build` | no | 1 | 0 | normal 1 | batch |
| `ti_cci` | no | 12 | 0 | normal 12 | batch |
| `ti_cci_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ceil` | no | 18 | 0 | normal 18 | batch |
| `ti_ceil_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cmo` | no | 11 | 1 | normal 11 | batch |
| `ti_cmo_start` | no | 8 | 0 | normal 8 | batch |
| `ti_cos` | no | 34 | 0 | normal 34 | batch |
| `ti_cos_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cosh` | no | 18 | 0 | normal 18 | batch |
| `ti_cosh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossany` | no | 27 | 0 | normal 27 | batch |
| `ti_crossany_start` | no | 1 | 0 | normal 1 | batch |
| `ti_crossover` | no | 21 | 0 | normal 21 | batch |
| `ti_crossover_start` | no | 1 | 0 | normal 1 | batch |
| `ti_cvi` | no | 9 | 0 | normal 9 | batch |
| `ti_cvi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_decay` | no | 16 | 0 | normal 16 | batch |
| `ti_decay_start` | no | 1 | 0 | normal 1 | batch |
| `ti_dema` | no | 23 | 0 | normal 23 | batch |
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
| `ti_edecay` | no | 16 | 0 | normal 16 | batch |
| `ti_edecay_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ema` | no | 24 | 0 | normal 24 | batch |
| `ti_ema_start` | no | 1 | 0 | normal 1 | batch |
| `ti_emv` | no | 20 | 0 | normal 20 | batch |
| `ti_emv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_exp` | no | 31 | 0 | normal 31 | batch |
| `ti_exp_start` | no | 1 | 0 | normal 1 | batch |
| `ti_find_indicator` | no | 21 | 0 | normal 21 | batch |
| `ti_fisher` | no | 9 | 0 | normal 9 | batch |
| `ti_fisher_start` | no | 1 | 1 | normal 1 | batch |
| `ti_floor` | no | 17 | 0 | normal 17 | batch |
| `ti_floor_start` | no | 1 | 0 | normal 1 | batch |
| `ti_fosc` | no | 11 | 0 | normal 11 | batch |
| `ti_fosc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_hma` | no | 10 | 0 | normal 10 | batch |
| `ti_hma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kama` | no | 10 | 0 | normal 10 | batch |
| `ti_kama_start` | no | 1 | 1 | normal 1 | batch |
| `ti_kvo` | no | 10 | 0 | normal 10 | batch |
| `ti_kvo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_lag` | no | 18 | 0 | normal 18 | batch |
| `ti_lag_start` | no | 8 | 0 | normal 8 | batch |
| `ti_linreg` | no | 11 | 0 | normal 11 | batch |
| `ti_linreg_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregintercept` | no | 10 | 0 | normal 10 | batch |
| `ti_linregintercept_start` | no | 1 | 1 | normal 1 | batch |
| `ti_linregslope` | no | 10 | 0 | normal 10 | batch |
| `ti_linregslope_start` | no | 1 | 1 | normal 1 | batch |
| `ti_ln` | no | 15 | 0 | normal 15 | batch |
| `ti_ln_start` | no | 1 | 0 | normal 1 | batch |
| `ti_log10` | no | 19 | 0 | normal 19 | batch |
| `ti_log10_start` | no | 1 | 0 | normal 1 | batch |
| `ti_macd` | no | 16 | 0 | normal 16 | batch |
| `ti_macd_start` | no | 1 | 1 | normal 1 | batch |
| `ti_marketfi` | no | 15 | 0 | normal 15 | batch |
| `ti_marketfi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_mass` | no | 12 | 0 | normal 12 | batch |
| `ti_mass_start` | no | 1 | 1 | normal 1 | batch |
| `ti_max` | no | 10 | 0 | normal 10 | batch |
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
| `ti_msw` | no | 12 | 0 | normal 12 | batch |
| `ti_msw_start` | no | 8 | 0 | normal 8 | batch |
| `ti_mul` | no | 20 | 0 | normal 20 | batch |
| `ti_mul_start` | no | 1 | 0 | normal 1 | batch |
| `ti_natr` | no | 9 | 0 | normal 9 | batch |
| `ti_natr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_nvi` | no | 17 | 0 | normal 17 | batch |
| `ti_nvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_obv` | no | 30 | 0 | normal 30 | batch |
| `ti_obv_start` | no | 1 | 0 | normal 1 | batch |
| `ti_ppo` | no | 25 | 0 | normal 25 | batch |
| `ti_ppo_start` | no | 1 | 0 | normal 1 | batch |
| `ti_psar` | no | 11 | 0 | normal 11 | batch |
| `ti_psar_start` | no | 1 | 0 | normal 1 | batch |
| `ti_pvi` | no | 16 | 0 | normal 16 | batch |
| `ti_pvi_start` | no | 1 | 0 | normal 1 | batch |
| `ti_qstick` | no | 10 | 0 | normal 10 | batch |
| `ti_qstick_start` | no | 1 | 1 | normal 1 | batch |
| `ti_roc` | no | 11 | 0 | normal 11 | batch |
| `ti_roc_start` | no | 8 | 0 | normal 8 | batch |
| `ti_rocr` | no | 11 | 0 | normal 11 | batch |
| `ti_rocr_start` | no | 8 | 0 | normal 8 | batch |
| `ti_round` | no | 19 | 0 | normal 19 | batch |
| `ti_round_start` | no | 1 | 0 | normal 1 | batch |
| `ti_rsi` | no | 10 | 0 | normal 10 | batch |
| `ti_rsi_start` | no | 8 | 0 | normal 8 | batch |
| `ti_sin` | no | 20 | 0 | normal 20 | batch |
| `ti_sin_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sinh` | no | 32 | 0 | normal 32 | batch |
| `ti_sinh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sma` | no | 10 | 0 | normal 10 | batch |
| `ti_sma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sqrt` | no | 17 | 0 | normal 17 | batch |
| `ti_sqrt_start` | no | 1 | 0 | normal 1 | batch |
| `ti_stddev` | no | 10 | 0 | normal 10 | batch |
| `ti_stddev_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stderr` | no | 10 | 0 | normal 10 | batch |
| `ti_stderr_start` | no | 1 | 1 | normal 1 | batch |
| `ti_stoch` | no | 10 | 0 | normal 10 | batch |
| `ti_stoch_start` | no | 23 | 1032 | normal 22, ub-gated 1 | batch |
| `ti_stochrsi` | no | 10 | 0 | normal 10 | batch |
| `ti_stochrsi_start` | no | 1 | 1 | normal 1 | batch |
| `ti_sub` | no | 16 | 0 | normal 16 | batch |
| `ti_sub_start` | no | 1 | 0 | normal 1 | batch |
| `ti_sum` | no | 10 | 0 | normal 10 | batch |
| `ti_sum_start` | no | 1 | 1 | normal 1 | batch |
| `ti_tan` | no | 19 | 0 | normal 19 | batch |
| `ti_tan_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tanh` | no | 30 | 0 | normal 30 | batch |
| `ti_tanh_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tema` | no | 22 | 0 | normal 22 | batch |
| `ti_tema_start` | no | 1 | 1 | normal 1 | batch |
| `ti_todeg` | no | 19 | 0 | normal 19 | batch |
| `ti_todeg_start` | no | 1 | 0 | normal 1 | batch |
| `ti_torad` | no | 17 | 0 | normal 17 | batch |
| `ti_torad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_tr` | no | 14 | 0 | normal 14 | batch |
| `ti_tr_start` | no | 1 | 0 | normal 1 | batch |
| `ti_trima` | no | 10 | 0 | normal 10 | batch |
| `ti_trima_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trix` | no | 10 | 0 | normal 10 | batch |
| `ti_trix_start` | no | 1 | 1 | normal 1 | batch |
| `ti_trunc` | no | 30 | 0 | normal 28, ub-gated 2 | batch |
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
| `ti_volatility` | no | 10 | 0 | normal 10 | batch |
| `ti_volatility_start` | no | 8 | 0 | normal 8 | batch |
| `ti_vosc` | no | 11 | 0 | normal 11 | batch |
| `ti_vosc_start` | no | 1 | 1 | normal 1 | batch |
| `ti_vwma` | no | 9 | 0 | normal 9 | batch |
| `ti_vwma_start` | no | 1 | 1 | normal 1 | batch |
| `ti_wad` | no | 15 | 0 | normal 15 | batch |
| `ti_wad_start` | no | 1 | 0 | normal 1 | batch |
| `ti_wcprice` | no | 15 | 0 | normal 15 | batch |
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

Status **PASS**. smoke: 12 pass / 0 fail per group, exit 0 (main_0 exposed, no-arg)

Mode used for the partition: **measured**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 212 harnesses, 0 crash-all (none).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=65536`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 213 | 213 | 212 | 212 | 1 | 0 | 0 | 1.000 | 0.995 |
| regions | 9219 | 8616 | 3197 | 3124 | 5492 | 73 | 530 | 0.935 | 0.347 |

Sanity checks: function pass, region pass. Harnesses unioned: 212. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `ti_abs` | 16 | 17 | 17 | 17 | 19 |
| `ti_acos` | 17 | 17 | 17 | 17 | 36 |
| `ti_ad` | 20 | 20 | 20 | 20 | 20 |
| `ti_add` | 15 | 15 | 15 | 15 | 15 |
| `ti_adosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_adx` | 9 | 9 | 9 | 9 | 9 |
| `ti_adxr` | 9 | 9 | 9 | 9 | 9 |
| `ti_ao` | 17 | 17 | 17 | 17 | 17 |
| `ti_apo` | 10 | 10 | 10 | 13 | 24 |
| `ti_aroon` | 9 | 9 | 9 | 9 | 9 |
| `ti_aroon_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_aroonosc` | 9 | 9 | 9 | 9 | 9 |
| `ti_aroonosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_asin` | 17 | 17 | 19 | 19 | 19 |
| `ti_atan` | 16 | 16 | 18 | 18 | 30 |
| `ti_atr` | 9 | 9 | 9 | 9 | 9 |
| `ti_avgprice` | 14 | 14 | 14 | 14 | 14 |
| `ti_bbands` | 10 | 10 | 10 | 10 | 10 |
| `ti_bop` | 18 | 18 | 18 | 18 | 18 |
| `ti_cci` | 11 | 11 | 11 | 11 | 12 |
| `ti_ceil` | 16 | 16 | 16 | 16 | 18 |
| `ti_cmo` | 10 | 10 | 10 | 10 | 11 |
| `ti_cmo_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_cos` | 16 | 17 | 17 | 19 | 34 |
| `ti_cosh` | 15 | 16 | 16 | 16 | 18 |
| `ti_crossany` | 23 | 23 | 23 | 27 | 27 |
| `ti_crossover` | 19 | 19 | 19 | 19 | 21 |
| `ti_cvi` | 9 | 9 | 9 | 9 | 9 |
| `ti_decay` | 16 | 16 | 16 | 16 | 16 |
| `ti_dema` | 10 | 10 | 10 | 11 | 23 |
| `ti_di` | 9 | 9 | 9 | 9 | 9 |
| `ti_div` | 15 | 15 | 15 | 15 | 15 |
| `ti_dm` | 9 | 9 | 9 | 9 | 9 |
| `ti_dpo` | 10 | 10 | 10 | 10 | 10 |
| `ti_dx` | 9 | 9 | 9 | 9 | 9 |
| `ti_edecay` | 16 | 16 | 16 | 16 | 16 |
| `ti_ema` | 10 | 10 | 13 | 13 | 24 |
| `ti_emv` | 15 | 15 | 15 | 15 | 20 |
| `ti_exp` | 14 | 17 | 17 | 17 | 31 |
| `ti_find_indicator` | 21 | 21 | 21 | 21 | 21 |
| `ti_fisher` | 9 | 9 | 9 | 9 | 9 |
| `ti_floor` | 14 | 15 | 15 | 17 | 17 |
| `ti_fosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_fosc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_hma` | 10 | 10 | 10 | 10 | 10 |
| `ti_kama` | 10 | 10 | 10 | 10 | 10 |
| `ti_kvo` | 10 | 10 | 10 | 10 | 10 |
| `ti_lag` | 18 | 18 | 18 | 18 | 18 |
| `ti_lag_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_linreg` | 10 | 10 | 10 | 10 | 11 |
| `ti_linregintercept` | 10 | 10 | 10 | 10 | 10 |
| `ti_linregslope` | 10 | 10 | 10 | 10 | 10 |
| `ti_ln` | 14 | 14 | 14 | 15 | 15 |
| `ti_log10` | 17 | 17 | 17 | 19 | 19 |
| `ti_macd` | 12 | 12 | 14 | 16 | 16 |
| `ti_marketfi` | 15 | 15 | 15 | 15 | 15 |
| `ti_mass` | 11 | 11 | 11 | 12 | 12 |
| `ti_max` | 10 | 10 | 10 | 10 | 10 |
| `ti_md` | 10 | 10 | 10 | 10 | 11 |
| `ti_medprice` | 15 | 15 | 15 | 15 | 15 |
| `ti_mfi` | 10 | 10 | 10 | 10 | 10 |
| `ti_mfi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_min` | 10 | 10 | 11 | 11 | 11 |
| `ti_mom` | 10 | 10 | 10 | 10 | 10 |
| `ti_mom_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_msw` | 10 | 10 | 10 | 10 | 12 |
| `ti_msw_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_mul` | 15 | 15 | 15 | 15 | 20 |
| `ti_natr` | 9 | 9 | 9 | 9 | 9 |
| `ti_nvi` | 16 | 16 | 16 | 16 | 17 |
| `ti_obv` | 18 | 18 | 18 | 18 | 30 |
| `ti_ppo` | 10 | 10 | 13 | 13 | 25 |
| `ti_psar` | 10 | 10 | 10 | 10 | 11 |
| `ti_pvi` | 15 | 15 | 15 | 16 | 16 |
| `ti_qstick` | 9 | 9 | 9 | 9 | 10 |
| `ti_roc` | 10 | 10 | 10 | 10 | 11 |
| `ti_roc_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_rocr` | 10 | 10 | 10 | 10 | 11 |
| `ti_rocr_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_round` | 17 | 17 | 17 | 17 | 19 |
| `ti_rsi` | 10 | 10 | 10 | 10 | 10 |
| `ti_rsi_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_sin` | 15 | 17 | 17 | 18 | 20 |
| `ti_sinh` | 18 | 18 | 18 | 20 | 32 |
| `ti_sma` | 10 | 10 | 10 | 10 | 10 |
| `ti_sqrt` | 14 | 15 | 17 | 17 | 17 |
| `ti_stddev` | 10 | 10 | 10 | 10 | 10 |
| `ti_stderr` | 10 | 10 | 10 | 10 | 10 |
| `ti_stoch` | 10 | 10 | 10 | 10 | 10 |
| `ti_stoch_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_stochrsi` | 10 | 10 | 10 | 10 | 10 |
| `ti_sub` | 16 | 16 | 16 | 16 | 16 |
| `ti_sum` | 10 | 10 | 10 | 10 | 10 |
| `ti_tan` | 16 | 17 | 17 | 19 | 19 |
| `ti_tanh` | 14 | 17 | 17 | 17 | 30 |
| `ti_tema` | 10 | 11 | 11 | 13 | 22 |
| `ti_todeg` | 16 | 16 | 17 | 19 | 19 |
| `ti_torad` | 15 | 15 | 17 | 17 | 17 |
| `ti_tr` | 14 | 14 | 14 | 14 | 14 |
| `ti_trima` | 10 | 10 | 10 | 10 | 10 |
| `ti_trix` | 10 | 10 | 10 | 10 | 10 |
| `ti_trunc` | 13 | 14 | 16 | 16 | 30 |
| `ti_tsf` | 10 | 10 | 10 | 10 | 10 |
| `ti_typprice` | 14 | 14 | 14 | 14 | 14 |
| `ti_ultosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_ultosc_start` | 23 | 23 | 23 | 23 | 23 |
| `ti_var` | 10 | 10 | 10 | 10 | 10 |
| `ti_vhf` | 10 | 10 | 10 | 10 | 10 |
| `ti_vhf_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vidya` | 10 | 10 | 10 | 10 | 10 |
| `ti_volatility` | 10 | 10 | 10 | 10 | 10 |
| `ti_volatility_start` | 8 | 8 | 8 | 8 | 8 |
| `ti_vosc` | 10 | 10 | 10 | 10 | 11 |
| `ti_vwma` | 9 | 9 | 9 | 9 | 9 |
| `ti_wad` | 15 | 15 | 15 | 15 | 15 |
| `ti_wcprice` | 14 | 14 | 14 | 14 | 15 |
| `ti_wilders` | 10 | 10 | 10 | 10 | 10 |
| `ti_willr` | 9 | 9 | 9 | 9 | 9 |
| `ti_wma` | 10 | 10 | 10 | 10 | 10 |
| `ti_zlema` | 10 | 10 | 10 | 10 | 10 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1811 |
| ub-gated | 3 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ti_adosc_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_adx_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_adxr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_atr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_bbands_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_buffer_new` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_cci_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_cmo` | 1 / 1 | not_reproducible 1 | 1 |
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
| `ti_vwma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wilders_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_willr_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_wma_start` | 1 / 1 | ub_associated_termination 1 | 1 |
| `ti_zlema_start` | 1 / 1 | ub_associated_termination 1 | 1 |

Total: not_reproducible 1, ub_associated_termination 243

<!-- prose -->
## 7. What this cell is, and is not

**The second paired cell, also clean.** CROWN's tulip passes the transpiled smoke 12/12 (with `main_0`
exposed), so its suite is a baseline. 213 planned, 212 built (`ti_buffer_free`, as on every tulip cell),
preflight 212 clean, 3 600 s, replay 1 811 `normal` + 3 `ub-gated`, **0 divergences**; confirmation sample
243 `ub_associated_termination` + 1 `not_reproducible`. Coverage: functions 212/213 ours vs 213/213
tests; regions **3 197 / 9 219 (0.347) ours vs 8 616 (0.935) tests**, both 3 124, only-ours 73 — the same
partition as c2rust to the region, for the same reason (`c2rust/RUN.md` §7: the option domain). CROWN's
tulip keeps the raw `T**` signatures, so the buffer-table bridge is `c_abi` on every boundary.

**Deviation:** none in procedure; the archive step was re-run after the subdirectory-hash fix.

**Not established:** the indicator loops beyond `period`, and `ti_buffer_free`.
