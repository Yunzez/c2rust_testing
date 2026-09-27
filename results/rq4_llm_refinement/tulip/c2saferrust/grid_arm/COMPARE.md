# Three-arm seed experiment — tulip × c2saferrust (600 s per arm, seed 42, -max_len 65536, one harness build)

## Application level (c2r_coverage union over all harnesses; universe = the cell's archived universe)

| arm | initial corpus | final corpus | fn t=600 | regions t=0 (seeds only) | regions t=600 | Δ fuzzing beyond seeds | region cov t=0 → t=600 |
|---|---:|---:|---:|---:|---:|---:|---:|
| base | 212 | None | 212/213 | 3147 | 3167 | 20 | 0.338 → **0.340** |
| grid | 938 | 3477 | 213/213 | 8203 | 8393 | 190 | 0.881 → **0.902** |

## Per boundary: regions of the boundary's own function, seeded boundaries only (covered/total; t=0 → t=600)

| boundary | seed len | base | grid | grid−base t=600 | Δfuzz base / grid |
|---|---:|---:|---:|---:|---:|
| ti_adosc | 32788 | 31→31/109 | 104→106 | +75 | +0 / +2 |
| ti_adosc_start | 16 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_adx | 98316 | 24→24/244 | 31→31 | +7 | +0 / +0 |
| ti_adx_start | 8 | 3/3 | 3 | +0 | — / — |
| ti_adxr | 98316 | 24→24/277 | 290→290 | +266 | +0 / +0 |
| ti_adxr_start | 8 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_apo | 8212 | 20→20/78 | 73→73 | +53 | +0 / +0 |
| ti_aroon | 65548 | 24→24/141 | 131→131 | +107 | +0 / +0 |
| ti_aroon_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_aroonosc | 65548 | 20→20/117 | 118→118 | +98 | +0 / +0 |
| ti_aroonosc_start | 8 | 5→5/5 | 5→5 | +0 | +0 / +0 |
| ti_atr | 98316 | 24→24/140 | 136→136 | +112 | +0 / +0 |
| ti_atr_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_bbands | 65556 | 31→31/146 | 130→131 | +100 | +0 / +1 |
| ti_bbands_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_cci | 24588 | 23→23/113 | 128→128 | +105 | +0 / +0 |
| ti_cci_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_cmo | 32780 | 16→16/132 | 131→132 | +116 | +0 / +1 |
| ti_cmo_start | 8 | 6→6/7 | 6→6 | +0 | +0 / +0 |
| ti_cvi | 65548 | 20→20/101 | 116→116 | +96 | +0 / +0 |
| ti_cvi_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_decay | 40972 | 40→40/41 | 41→41 | +1 | +0 / +0 |
| ti_dema | 8204 | 16→16/72 | 71→71 | +55 | +0 / +0 |
| ti_dema_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_di | 98316 | 28→28/223 | 213→213 | +185 | +0 / +0 |
| ti_di_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_dm | 65548 | 24→24/146 | 136→136 | +112 | +0 / +0 |
| ti_dm_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_dpo | 32780 | 22→22/74 | 70→70 | +48 | +0 / +0 |
| ti_dpo_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_dx | 98316 | 24→24/230 | 226→226 | +202 | +0 / +0 |
| ti_dx_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_edecay | 40972 | 41→41/41 | 41→41 | +0 | +0 / +0 |
| ti_ema | 8204 | 16→16/59 | 55→55 | +39 | +0 / +0 |
| ti_fisher | 65548 | 24→24/177 | 167→167 | +143 | +0 / +0 |
| ti_fisher_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_fosc | 32780 | 16→16/106 | 105→106 | +90 | +0 / +1 |
| ti_fosc_start | 8 | 6→6/7 | 6→6 | +0 | +0 / +0 |
| ti_hma | 32780 | 16→16/152 | 170→170 | +154 | +0 / +0 |
| ti_hma_start | 8 | 9→9/9 | 9→9 | +0 | +0 / +0 |
| ti_kama | 32780 | 16→16/108 | 104→105 | +89 | +0 / +1 |
| ti_kama_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_kvo | 32788 | 28→28/141 | 136→137 | +109 | +0 / +1 |
| ti_lag | 32780 | 45→40/47 | 46→46 | +6 | -5 / +0 |
| ti_lag_start | 8 | 6→6/7 | 6→6 | +0 | +0 / +0 |
| ti_linreg | 32780 | 16→16/95 | 92→93 | +77 | +0 / +1 |
| ti_linreg_start | 8 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_linregintercept | 32780 | 16→16/95 | 91→92 | +76 | +0 / +1 |
| ti_linregintercept_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_linregslope | 32780 | 16→16/87 | 83→84 | +68 | +0 / +1 |
| ti_linregslope_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_macd | 8220 | 32→32/147 | 130→133 | +101 | +0 / +3 |
| ti_macd_start | 16 | 7→7/7 | 7→7 | +0 | +0 / +0 |
| ti_mass | 16396 | 20→20/109 | 124→124 | +104 | +0 / +0 |
| ti_mass_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_max | 32780 | 16→16/82 | 78→79 | +63 | +0 / +1 |
| ti_max_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_md | 32780 | 19→19/81 | 77→78 | +59 | +0 / +1 |
| ti_md_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_mfi | 32780 | 24→24/188 | 201→204 | +180 | +0 / +3 |
| ti_mfi_start | 8 | 6→6/7 | 6→6 | +0 | +0 / +0 |
| ti_min | 32780 | 16→16/82 | 78→79 | +63 | +0 / +1 |
| ti_min_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_mom | 32780 | 16→16/49 | 45→46 | +30 | +0 / +1 |
| ti_mom_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_msw | 32780 | 20→20/119 | 109→110 | +90 | +0 / +1 |
| ti_msw_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_natr | 98316 | 24→24/145 | 141→141 | +117 | +0 / +0 |
| ti_natr_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_ppo | 8212 | 20→20/78 | 73→73 | +53 | +0 / +0 |
| ti_psar | 16404 | 24→24/163 | 27→27 | +3 | +0 / +0 |
| ti_qstick | 65548 | 23→23/76 | 72→72 | +49 | +0 / +0 |
| ti_qstick_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_roc | 32780 | 16→16/52 | 48→48 | +32 | +0 / +0 |
| ti_roc_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_rocr | 32780 | 16→16/49 | 47→47 | +31 | +0 / +0 |
| ti_rocr_start | 8 | 5→5/5 | 5→5 | +0 | +0 / +0 |
| ti_rsi | 32780 | 19→19/121 | 119→120 | +101 | +0 / +1 |
| ti_rsi_start | 8 | 5→5/5 | 5→5 | +0 | +0 / +0 |
| ti_sma | 32780 | 19→19/65 | 61→61 | +42 | +0 / +0 |
| ti_sma_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_stddev | 32780 | 19→19/98 | 93→94 | +75 | +0 / +1 |
| ti_stddev_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_stderr | 32780 | 19→19/101 | 96→97 | +78 | +0 / +1 |
| ti_stderr_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_stoch | 73756 | 42→42/229 | 241→241 | +199 | +0 / +0 |
| ti_stoch_start | 24 | 12→12/12 | 12→12 | +0 | +0 / +0 |
| ti_stochrsi | 32780 | 19→19/220 | 229→229 | +210 | +0 / +0 |
| ti_stochrsi_start | 8 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_sum | 32780 | 16→16/62 | 59→59 | +43 | +0 / +0 |
| ti_sum_start | 8 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_tema | 8204 | 16→16/85 | 84→85 | +69 | +0 / +1 |
| ti_tema_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_trima | 32780 | 16→16/132 | 189→190 | +174 | +0 / +1 |
| ti_trima_start | 8 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_trix | 32780 | 16→16/97 | 90→90 | +74 | +0 / +0 |
| ti_trix_start | 8 | 6→6/6 | 6→6 | +0 | +0 / +0 |
| ti_tsf | 32780 | 16→16/96 | 92→92 | +76 | +0 / +0 |
| ti_tsf_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_ultosc | 24604 | 32→32/204 | 212→217 | +185 | +0 / +5 |
| ti_ultosc_start | 24 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_var | 32780 | 19→19/86 | 82→82 | +63 | +0 / +0 |
| ti_var_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_vhf | 32780 | 16→16/142 | 138→139 | +123 | +0 / +1 |
| ti_vhf_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_vidya | 32796 | 30→30/187 | 39→180 | +150 | +0 / +141 |
| ti_vidya_start | 16 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_volatility | 32780 | 23→23/99 | 95→95 | +72 | +0 / +0 |
| ti_volatility_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_vosc | 32788 | 26→26/100 | 95→96 | +70 | +0 / +1 |
| ti_vosc_start | 16 | 4→4/4 | 4→4 | +0 | +0 / +0 |
| ti_vwma | 65548 | 20→20/83 | 79→79 | +59 | +0 / +0 |
| ti_vwma_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_wilders | 32780 | 16→16/65 | 61→61 | +45 | +0 / +0 |
| ti_wilders_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_willr | 73740 | 24→24/130 | 125→125 | +101 | +0 / +0 |
| ti_willr_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_wma | 32780 | 16→16/72 | 68→69 | +53 | +0 / +1 |
| ti_wma_start | 8 | 3→3/3 | 3→3 | +0 | +0 / +0 |
| ti_zlema | 32780 | 19→19/72 | 70→71 | +52 | +0 / +1 |
| ti_zlema_start | 8 | 5→5/5 | 5→5 | +0 | +0 / +0 |
| **sum over 121** | | 1638→1636/7698 | 7135→7314 (harness-level, totals n/a) | +5678 | -5 / +176 |

- boundaries where grid > base at t=600: 63 of 121
- grid − base per boundary at t=600: min 0, median 6, max 266
- initial seeds longer than -max_len 65536: 16 boundaries × 6 seeds, longest 98316 bytes (ti_adx, ti_adxr, ti_aroon, ti_aroonosc, ti_atr, ti_bbands, ti_cvi, ti_di, ti_dm, ti_dx, ti_fisher, ti_natr, ti_qstick, ti_stoch, ti_vwma, ti_willr). Measured on ti_stoch (layout validation): such a seed is executed and its options decoded, so the seed breaks the INITIAL reachability barrier; whether the fuzzer keeps mutating at that length is not established (libFuzzer bounds generated inputs by -max_len), so no claim is made that the max_len limit is gone.
