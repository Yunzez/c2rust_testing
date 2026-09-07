# RQ4 — all libraries, one table each

Assembled by `scripts/rq4/summary_all.py` from every library's `cells.json` (itself from the cells' own files) and `results/rq4_effectiveness/defect_manifest.json`. Protocol: `PROTOCOL.md`. A library without a `cells.json` is listed as *in flight*.

## 1. Funnel per cell

| library | tool | matched | planned | built | executed | exported | corpus | term. cands | div. cands |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| bzip2 | c2rust | 64 | 19 | 19 | 19 | 16 | 1338 | 50197 | 6 |
| bzip2 | Laertes | 64 | 19 | 19 | 19 | 16 | 974 | 35505 | 317 |
| bzip2 | CROWN | 64 | 19 | 19 | 19 | 16 | 1003 | 59807 | 267 |
| bzip2 | C2SaferRust | 64 | 17 | 17 | 17 | 15 | 302 | 46725 | 11 |
| genann | c2rust | 12 | 10 | 10 | 10 | 10 | 183 | 1630 | 1 |
| genann | Laertes | 12 | 10 | 10 | 10 | 10 | 190 | 1243 | 1 |
| genann | C2SaferRust | 12 | 10 | 10 | 10 | 10 | 186 | 916 | 1 |
| genann | CROWN | 12 | 10 | 10 | 10 | 10 | 167 | 1307 | 1 |
| genann | SACTOR | 15 | 13 | 13 | 13 | 13 | 311 | 13379 | 52 |
| cjson | c2rust | 58 | 39 | 39 | 39 | 39 | 8796 | 99999 | 0 |
| cjson | PtrTrans | 113 | 15 | 9 | 9 | 9 | 20 | 0 | 18 |
| lil | c2rust | 145 | 51 | 50 | 50 | 9 | 4761 | 1703 | 7 |
| lil | Laertes | 145 | 51 | 51 | 51 | 48 | 4907 | 5112 | 7 |
| lil | C2SaferRust | 145 | 47 | 47 | 47 | 24 | 340 | 3670 | 27 |
| lil | CROWN | 145 | 42 | 42 | 42 | 39 | 1859 | 617 | 3 |
| tulip | c2rust | 213 | 213 | 212 | 212 | 212 | 1903 | 1149 | 0 |
| tulip | Laertes | 213 | 213 | 212 | 212 | 211 | 1813 | 1157 | 1 |
| tulip | C2SaferRust | 213 | 212 | 212 | 212 | 212 | 1776 | 1038 | 9 |
| tulip | CROWN | 213 | 213 | 212 | 212 | 212 | 1814 | 1076 | 0 |

19 cells: 1223 planned of 1922 matched boundaries, 1213 built, 1213 executed, 1131 with a coverage export; 32643 corpus inputs in total.

## 2. Coverage of the translation: shipped suite vs validator

Fractions are of the cell's universe (the passing suite's instrumented build where the suite is a baseline, else the translation's own rlib objects). `—` = the suite is not a baseline for that cell (TEST-UNAVAILABLE or TEST-FAILS): no tests column, never 0 %. Raw counts are per-translation identities and are not comparable across tools.

| library | tool | tests side | fn tests | fn ours | fn only-ours | reg tests | reg ours | reg only-ours |
|---|---|---|---:|---:|---:|---:|---:|---:|
| bzip2 | c2rust | PASS 6/6 | 51/66 (0.773) | 46/66 (0.697) | 3 | 7007/8789 (0.797) | 7090/8789 (0.807) | 481 |
| bzip2 | Laertes | TEST-FAILS 0/6 | — | 45/82 (0.549) | 45 | — | 6206/10065 (0.617) | 6206 |
| bzip2 | CROWN | TEST-ADAPTER-FAILS | — | 51/74 (0.689) | 51 | — | 5414/9084 (0.596) | 5414 |
| bzip2 | C2SaferRust | TEST-FAILS 0/6 | — | 15/69 (0.217) | 15 | — | 1158/8227 (0.141) | 1158 |
| genann | c2rust | PASS | 11/12 (0.917) | 10/12 (0.833) | 1 | 513/573 (0.895) | 462/573 (0.806) | 19 |
| genann | Laertes | PASS | 13/17 (0.765) | 12/17 (0.706) | 1 | 518/587 (0.882) | 467/587 (0.796) | 19 |
| genann | C2SaferRust | TEST-FAILS | — | 10/12 (0.833) | 10 | — | 459/563 (0.815) | 459 |
| genann | CROWN | PASS | 11/12 (0.917) | 10/12 (0.833) | 1 | 514/574 (0.895) | 467/574 (0.814) | 21 |
| genann | SACTOR | TEST-UNAVAILABLE | — | 15/21 (0.714) | 15 | — | 506/716 (0.707) | 506 |
| cjson | c2rust | TEST-UNAVAILABLE | — | 49/59 (0.831) | 49 | — | 1816/2237 (0.812) | 1816 |
| cjson | PtrTrans | TEST-UNAVAILABLE | — | 10/121 (0.083) | 10 | — | 68/2125 (0.032) | 68 |
| lil | c2rust | TEST-UNAVAILABLE | — | 143/151 (0.947) | 143 | — | 4999/5730 (0.872) | 4999 |
| lil | Laertes | TEST-UNAVAILABLE | — | 144/183 (0.787) | 144 | — | 5028/6143 (0.818) | 5028 |
| lil | C2SaferRust | TEST-UNAVAILABLE | — | 25/154 (0.162) | 25 | — | 362/5751 (0.063) | 362 |
| lil | CROWN | TEST-UNAVAILABLE | — | 127/134 (0.948) | 127 | — | 5294/6409 (0.826) | 5294 |
| tulip | c2rust | PASS | 213/213 (1.000) | 212/213 (0.995) | 0 | 8616/9298 (0.927) | 3197/9298 (0.344) | 73 |
| tulip | Laertes | TEST-FAILS | — | 211/216 (0.977) | 211 | — | 3168/13191 (0.240) | 3168 |
| tulip | C2SaferRust | TEST-FAILS | — | 212/213 (0.995) | 212 | — | 3167/9306 (0.340) | 3167 |
| tulip | CROWN | PASS | 213/213 (1.000) | 212/213 (0.995) | 0 | 8616/9219 (0.935) | 3197/9219 (0.347) | 73 |

## 3. Candidates, confirmation, defects

Confirmation totals are the cell's labelled sample (or its full confirmation where one exists). `confirmed` = `confirmed_divergence` + `confirmed_termination`; everything else is a recorded non-defect class. Defect ids are manifest rows whose evidence names this cell.

| library | tool | confirmed div. | confirmed term. | ub-associated | instrument-only / out-of-contract | inconclusive / not reproducible | defects (manifest) |
|---|---|---:|---:|---:|---:|---:|---|
| bzip2 | c2rust | 0 | 0 | 1024 | 5 | 17 | — |
| bzip2 | Laertes | 299 | 233 | 0 | 0 | 0 | C8, S3 |
| bzip2 | CROWN | 242 | 4 | 0 | 8277 | 0 | C7, S10, S11 |
| bzip2 | C2SaferRust | 3 | 0 | 0 | 0 | 0 | S14 |
| genann | c2rust | 0 | 0 | 201 | 0 | 200 | — |
| genann | Laertes | 0 | 0 | 201 | 0 | 200 | — |
| genann | C2SaferRust | 0 | 0 | 201 | 0 | 134 | — |
| genann | CROWN | 0 | 0 | 201 | 0 | 200 | — |
| genann | SACTOR | 51 | 0 | 801 | 0 | 200 | S5 |
| cjson | c2rust | 0 | 0 | 3729 | 0 | 115 | — |
| cjson | PtrTrans | 18 | 0 | 0 | 0 | 0 | CAND-5 (candidate) |
| lil | c2rust | 0 | 0 | 647 | 53 | 116 | — |
| lil | Laertes | 0 | 0 | 662 | 52 | 102 | — |
| lil | C2SaferRust | 0 | 57 | 413 | 3 | 1 | C9 |
| lil | CROWN | 0 | 69 | 218 | 61 | 43 | C10 |
| tulip | c2rust | 0 | 0 | 243 | 0 | 0 | — |
| tulip | Laertes | 0 | 2 | 244 | 0 | 35 | C11 |
| tulip | C2SaferRust | 5 | 7 | 242 | 0 | 4 | S15 |
| tulip | CROWN | 0 | 0 | 243 | 0 | 1 | — |

Confirmed across all cells: 618 value divergences, 372 terminations, before clustering (one site = one defect; a producer's crash counts once and blocks its dependants).

## 4. Defects the RQ4 cells found or re-found

| id | library × tool | family | found by |
|---|---|---|---|
| C7 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| C8 | bzip2 × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| C9 | lil × C2SaferRust | byte-string domain narrowing | NEW — found by the RQ4 plan pipeline |
| C10 | lil × CROWN | null/empty conflation | NEW — found by the RQ4 plan pipeline |
| S15 | tulipindicators × C2SaferRust | semantic computation substitution | catalogued earlier; re-found by the RQ4 cell |
| C11 | tulipindicators × Laertes | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S3 | bzip2 × Laertes | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S5 | genann × SACTOR | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S10 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| S11 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| S14 | bzip2 × C2SaferRust | semantic computation substitution | catalogued earlier; re-found by the RQ4 cell |

