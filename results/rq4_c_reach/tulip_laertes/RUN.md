# Same-corpus C reach — tulip_laertes

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `tulip.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 212 / 213 (0.995) | 431 / 1584 (0.272) |
| Rust (archived campaign) | 211 / 216 (0.977) | 3168 / 13191 (0.240) |

Inputs replayed on the C side: {'completed': 1813} over 212 / 212 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/tulip__laertes/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 23 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 21 | 1 (1) | 0 (0) | 1 | 190 | 0 (0) | 3 (0) |

`c_only`: ti_find_indicator→ti_find_indicator

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| ti_abs | ok | 18 | {'completed': 18} | 1/213 | 4/1584 |
| ti_abs_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_acos | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_acos_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ad | ok | 21 | {'completed': 21} | 1/213 | 6/1584 |
| ti_ad_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_add | ok | 19 | {'completed': 19} | 1/213 | 4/1584 |
| ti_add_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_adosc | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_adosc_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_adx | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_adx_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_adxr | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_adxr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ao | ok | 18 | {'completed': 18} | 2/213 | 15/1584 |
| ti_ao_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_apo | ok | 24 | {'completed': 24} | 1/213 | 3/1584 |
| ti_apo_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_aroon | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_aroon_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_aroonosc | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_aroonosc_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_asin | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_asin_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_atan | ok | 18 | {'completed': 18} | 1/213 | 4/1584 |
| ti_atan_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_atr | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_atr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_avgprice | ok | 18 | {'completed': 18} | 1/213 | 4/1584 |
| ti_avgprice_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_bbands | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_bbands_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_bop | ok | 17 | {'completed': 17} | 1/213 | 6/1584 |
| ti_bop_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_buffer_new | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_build | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_cci | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_cci_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ceil | ok | 33 | {'completed': 33} | 1/213 | 4/1584 |
| ti_ceil_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_cmo | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_cmo_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_cos | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_cos_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_cosh | ok | 34 | {'completed': 34} | 1/213 | 4/1584 |
| ti_cosh_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_crossany | ok | 36 | {'completed': 36} | 1/213 | 10/1584 |
| ti_crossany_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_crossover | ok | 21 | {'completed': 21} | 1/213 | 6/1584 |
| ti_crossover_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_cvi | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_cvi_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_decay | ok | 16 | {'completed': 16} | 1/213 | 6/1584 |
| ti_decay_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_dema | ok | 22 | {'completed': 22} | 1/213 | 3/1584 |
| ti_dema_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_di | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_di_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_div | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_div_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_dm | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_dm_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_dpo | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_dpo_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_dx | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_dx_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_edecay | ok | 16 | {'completed': 16} | 1/213 | 7/1584 |
| ti_edecay_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ema | ok | 15 | {'completed': 15} | 1/213 | 3/1584 |
| ti_ema_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_emv | ok | 16 | {'completed': 16} | 2/213 | 10/1584 |
| ti_emv_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_exp | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_exp_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_find_indicator | ok | 1 | {'completed': 1} | 1/213 | 8/1584 |
| ti_fisher | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_fisher_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_floor | ok | 19 | {'completed': 19} | 1/213 | 4/1584 |
| ti_floor_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_fosc | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_fosc_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_hma | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_hma_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_kama | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_kama_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_kvo | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_kvo_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_lag | ok | 19 | {'completed': 19} | 2/213 | 12/1584 |
| ti_lag_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_linreg | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_linreg_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_linregintercept | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_linregintercept_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_linregslope | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_linregslope_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ln | ok | 28 | {'completed': 28} | 1/213 | 4/1584 |
| ti_ln_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_log10 | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_log10_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_macd | ok | 15 | {'completed': 15} | 1/213 | 3/1584 |
| ti_macd_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_marketfi | ok | 15 | {'completed': 15} | 2/213 | 10/1584 |
| ti_marketfi_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_mass | ok | 13 | {'completed': 13} | 1/213 | 3/1584 |
| ti_mass_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_max | ok | 12 | {'completed': 12} | 1/213 | 3/1584 |
| ti_max_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_md | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_md_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_medprice | ok | 15 | {'completed': 15} | 1/213 | 4/1584 |
| ti_medprice_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_mfi | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_mfi_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_min | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_min_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_mom | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_mom_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_msw | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_msw_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_mul | ok | 15 | {'completed': 15} | 1/213 | 4/1584 |
| ti_mul_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_natr | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_natr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_nvi | ok | 17 | {'completed': 17} | 2/213 | 11/1584 |
| ti_nvi_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_obv | ok | 29 | {'completed': 29} | 1/213 | 10/1584 |
| ti_obv_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ppo | ok | 22 | {'completed': 22} | 1/213 | 3/1584 |
| ti_ppo_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_psar | ok | 13 | {'completed': 13} | 1/213 | 3/1584 |
| ti_psar_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_pvi | ok | 16 | {'completed': 16} | 2/213 | 11/1584 |
| ti_pvi_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_qstick | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_qstick_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_roc | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_roc_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_rocr | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_rocr_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_round | ok | 31 | {'completed': 31} | 1/213 | 4/1584 |
| ti_round_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_rsi | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_rsi_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_sin | ok | 21 | {'completed': 21} | 1/213 | 4/1584 |
| ti_sin_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_sinh | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_sinh_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_sma | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_sma_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_sqrt | ok | 19 | {'completed': 19} | 1/213 | 4/1584 |
| ti_sqrt_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_stddev | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_stddev_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_stderr | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_stderr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_stoch | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_stoch_start | ok | 23 | {'completed': 23} | 1/213 | 1/1584 |
| ti_stochrsi | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_stochrsi_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_sub | ok | 15 | {'completed': 15} | 1/213 | 4/1584 |
| ti_sub_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_sum | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_sum_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_tan | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_tan_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_tanh | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_tanh_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_tema | ok | 15 | {'completed': 15} | 1/213 | 3/1584 |
| ti_tema_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_todeg | ok | 18 | {'completed': 18} | 1/213 | 4/1584 |
| ti_todeg_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_torad | ok | 19 | {'completed': 19} | 1/213 | 4/1584 |
| ti_torad_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_tr | ok | 16 | {'completed': 16} | 1/213 | 4/1584 |
| ti_tr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_trima | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_trima_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_trix | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_trix_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_trunc | ok | 31 | {'completed': 31} | 1/213 | 4/1584 |
| ti_trunc_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_tsf | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_tsf_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_typprice | ok | 16 | {'completed': 16} | 1/213 | 4/1584 |
| ti_typprice_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_ultosc | ok | 12 | {'completed': 12} | 1/213 | 3/1584 |
| ti_ultosc_start | ok | 23 | {'completed': 23} | 1/213 | 1/1584 |
| ti_var | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_var_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_version | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_vhf | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_vhf_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_vidya | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_vidya_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_volatility | ok | 10 | {'completed': 10} | 1/213 | 3/1584 |
| ti_volatility_start | ok | 8 | {'completed': 8} | 1/213 | 1/1584 |
| ti_vosc | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_vosc_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_vwma | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_vwma_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_wad | ok | 19 | {'completed': 19} | 2/213 | 14/1584 |
| ti_wad_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_wcprice | ok | 20 | {'completed': 20} | 1/213 | 4/1584 |
| ti_wcprice_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_wilders | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_wilders_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_willr | ok | 9 | {'completed': 9} | 1/213 | 3/1584 |
| ti_willr_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_wma | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_wma_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |
| ti_zlema | ok | 11 | {'completed': 11} | 1/213 | 3/1584 |
| ti_zlema_start | ok | 1 | {'completed': 1} | 1/213 | 1/1584 |

## Procedure, deviations, and what is not established

<!-- prose -->
