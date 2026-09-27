# Three-arm seed experiment — tulip × laertes (600 s per arm, seed 42, -max_len 65536, one harness build)

## Application level (c2r_coverage union over all harnesses; universe = the cell's archived universe)

| arm | initial corpus | final corpus | fn t=600 | regions t=0 (seeds only) | regions t=600 | Δ fuzzing beyond seeds | region cov t=0 → t=600 |
|---|---:|---:|---:|---:|---:|---:|---:|
| base | 213 | 1751 | 211/216 | 3155 | 3169 | 14 | 0.239 → **0.240** |
| grid | 939 | 3335 | 212/216 | 8384 | 8563 | 179 | 0.636 → **0.649** |

## Per boundary: regions of the boundary's own function, seeded boundaries only (covered/total; t=0 → t=600)

| boundary | seed len | base | grid | grid−base t=600 | Δfuzz base / grid |
|---|---:|---:|---:|---:|---:|
| ti_adosc | 32788 | 31→31 | 104→106 | +75 | +0 / +2 |
| ti_adosc_start | 16 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_adx | 98316 | 24→24 | 242→242 | +218 | +0 / +0 |
| ti_adx_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_adxr | 98316 | 24→24 | 289→289 | +265 | +0 / +0 |
| ti_adxr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_apo | 8212 | 20→20 | 73→73 | +53 | +0 / +0 |
| ti_aroon | 65548 | 24→24 | 132→132 | +108 | +0 / +0 |
| ti_aroon_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_aroonosc | 65548 | 20→20 | 118→118 | +98 | +0 / +0 |
| ti_aroonosc_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_atr | 98316 | 24→24 | 137→137 | +113 | +0 / +0 |
| ti_atr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_bbands | 65556 | 31→31 | 131→132 | +101 | +0 / +1 |
| ti_bbands_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_cci | 24588 | 23→23 | 128→128 | +105 | +0 / +0 |
| ti_cci_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_cmo | 32780 | 16→16 | 129→129 | +113 | +0 / +0 |
| ti_cmo_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_cvi | 65548 | 20→20 | 116→116 | +96 | +0 / +0 |
| ti_cvi_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_decay | 40972 | 40→41 | 41→41 | +0 | +1 / +0 |
| ti_dema | 8204 | 16→16 | 72→72 | +56 | +0 / +0 |
| ti_dema_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_di | 98316 | 28→28 | 214→214 | +186 | +0 / +0 |
| ti_di_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_dm | 65548 | 24→24 | 137→137 | +113 | +0 / +0 |
| ti_dm_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_dpo | 32780 | 22→22 | 71→71 | +49 | +0 / +0 |
| ti_dpo_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_dx | 98316 | 24→24 | 227→227 | +203 | +0 / +0 |
| ti_dx_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_edecay | 40972 | 41→41 | 41→41 | +0 | +0 / +0 |
| ti_ema | 8204 | 16→16 | 55→55 | +39 | +0 / +0 |
| ti_fisher | 65548 | 24→24 | 168→168 | +144 | +0 / +0 |
| ti_fisher_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_fosc | 32780 | 16→16 | 103→104 | +88 | +0 / +1 |
| ti_fosc_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_hma | 32780 | 16→16 | 170→170 | +154 | +0 / +0 |
| ti_hma_start | 8 | 10→10 | 10→10 | +0 | +0 / +0 |
| ti_kama | 32780 | 16→16 | 105→106 | +90 | +0 / +1 |
| ti_kama_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_kvo | 32788 | 28→28 | 136→137 | +109 | +0 / +1 |
| ti_lag | 32780 | 43→44 | 44→43 | -1 | +1 / -1 |
| ti_lag_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_linreg | 32780 | 16→16 | 92→93 | +77 | +0 / +1 |
| ti_linreg_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_linregintercept | 32780 | 16→16 | 92→93 | +77 | +0 / +1 |
| ti_linregintercept_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_linregslope | 32780 | 16→16 | 84→85 | +69 | +0 / +1 |
| ti_linregslope_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_macd | 8220 | 32→32 | 130→132 | +100 | +0 / +2 |
| ti_macd_start | 16 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_mass | 16396 | 20→20 | 124→124 | +104 | +0 / +0 |
| ti_mass_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_max | 32780 | 16→16 | 79→79 | +63 | +0 / +0 |
| ti_max_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_md | 32780 | 19→19 | 78→78 | +59 | +0 / +0 |
| ti_md_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_mfi | 32780 | 24→24 | 195→198 | +174 | +0 / +3 |
| ti_mfi_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_min | 32780 | 16→16 | 79→80 | +64 | +0 / +1 |
| ti_min_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_mom | 32780 | 16→16 | 46→47 | +31 | +0 / +1 |
| ti_mom_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_msw | 32780 | 20→20 | 110→110 | +90 | +0 / +0 |
| ti_msw_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_natr | 98316 | 24→24 | 142→142 | +118 | +0 / +0 |
| ti_natr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_ppo | 8212 | 20→20 | 73→73 | +53 | +0 / +0 |
| ti_psar | 16404 | 24→24 | 27→27 | +3 | +0 / +0 |
| ti_qstick | 65548 | 23→23 | 73→73 | +50 | +0 / +0 |
| ti_qstick_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_roc | 32780 | 16→16 | 49→50 | +34 | +0 / +1 |
| ti_roc_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_rocr | 32780 | 16→16 | 46→46 | +30 | +0 / +0 |
| ti_rocr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_rsi | 32780 | 19→19 | 118→119 | +100 | +0 / +1 |
| ti_rsi_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_sma | 32780 | 19→19 | 62→62 | +43 | +0 / +0 |
| ti_sma_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_stddev | 32780 | 19→19 | 94→95 | +76 | +0 / +1 |
| ti_stddev_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_stderr | 32780 | 19→19 | 97→98 | +79 | +0 / +1 |
| ti_stderr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_stoch | 73756 | 42→42 | 240→240 | +198 | +0 / +0 |
| ti_stoch_start | 24 | 15→15 | 15→15 | +0 | +0 / +0 |
| ti_stochrsi | 32780 | 19→19 | 228→228 | +209 | +0 / +0 |
| ti_stochrsi_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_sum | 32780 | 16→16 | 59→60 | +44 | +0 / +1 |
| ti_sum_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_tema | 8204 | 16→16 | 85→85 | +69 | +0 / +0 |
| ti_tema_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_trima | 32780 | 16→16 | 190→191 | +175 | +0 / +1 |
| ti_trima_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_trix | 32780 | 16→16 | 91→91 | +75 | +0 / +0 |
| ti_trix_start | 8 | 7→7 | 7→7 | +0 | +0 / +0 |
| ti_tsf | 32780 | 16→16 | 93→94 | +78 | +0 / +1 |
| ti_tsf_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_ultosc | 24604 | 32→32 | 208→210 | +178 | +0 / +2 |
| ti_ultosc_start | 24 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_var | 32780 | 19→19 | 83→84 | +65 | +0 / +1 |
| ti_var_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_vhf | 32780 | 16→16 | 139→140 | +124 | +0 / +1 |
| ti_vhf_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_vidya | 32796 | 30→30 | 39→181 | +151 | +0 / +142 |
| ti_vidya_start | 16 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_volatility | 32780 | 23→23 | 96→96 | +73 | +0 / +0 |
| ti_volatility_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_vosc | 32788 | 26→26 | 95→96 | +70 | +0 / +1 |
| ti_vosc_start | 16 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_vwma | 65548 | 20→20 | 80→80 | +60 | +0 / +0 |
| ti_vwma_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_wilders | 32780 | 16→16 | 62→63 | +47 | +0 / +1 |
| ti_wilders_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_willr | 73740 | 24→24 | 126→126 | +102 | +0 / +0 |
| ti_willr_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_wma | 32780 | 16→16 | 69→70 | +54 | +0 / +1 |
| ti_wma_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| ti_zlema | 32780 | 19→19 | 69→69 | +50 | +0 / +0 |
| ti_zlema_start | 8 | 4→4 | 4→4 | +0 | +0 / +0 |
| **sum over 121** | | 1669→1671 (harness-level, totals n/a) | 7391→7562 (harness-level, totals n/a) | +5891 | +2 / +171 |

- boundaries where grid > base at t=600: 61 of 121
- grid − base per boundary at t=600: min -1, median 3, max 265
- initial seeds longer than -max_len 65536: 16 boundaries × 6 seeds, longest 98316 bytes (ti_adx, ti_adxr, ti_aroon, ti_aroonosc, ti_atr, ti_bbands, ti_cvi, ti_di, ti_dm, ti_dx, ti_fisher, ti_natr, ti_qstick, ti_stoch, ti_vwma, ti_willr). Measured on ti_stoch (layout validation): such a seed is executed and its options decoded, so the seed breaks the INITIAL reachability barrier; whether the fuzzer keeps mutating at that length is not established (libFuzzer bounds generated inputs by -max_len), so no claim is made that the max_len limit is gone.
