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
| lil | C2SaferRust | 145 | 47 | 47 | 47 | 24 | 340 | 3670 | 27 |
| lil | CROWN | 145 | 42 | 42 | 42 | 39 | 1859 | 617 | 3 |
| lil | Laertes | 145 | 51 | 51 | 51 | 48 | 4907 | 5112 | 7 |
| tulip | c2rust | 213 | 213 | 212 | 212 | 212 | 1903 | 1149 | 0 |
| tulip | C2SaferRust | 213 | 212 | 212 | 212 | 212 | 1776 | 1038 | 9 |
| tulip | CROWN | 213 | 213 | 212 | 212 | 212 | 1814 | 1076 | 0 |
| tulip | Laertes | 213 | 213 | 212 | 212 | 211 | 1813 | 1157 | 1 |
| qsort | c2rust | 3 | 3 | 3 | 3 | 3 | 121 | 0 | 0 |
| qsort | Laertes | 3 | 3 | 3 | 3 | 3 | 118 | 0 | 0 |
| qsort | C2SaferRust | 3 | 3 | 3 | 3 | 3 | 118 | 5006 | 0 |
| qsort | CROWN | 3 | 3 | 3 | 3 | 3 | 120 | 0 | 0 |
| qsort | SACTOR | 3 | 3 | 3 | 3 | 3 | 132 | 0 | 0 |
| qsort | PtrTrans | 3 | 3 | 3 | 3 | 3 | 131 | 0 | 57 |
| urlparser | c2rust | 21 | 21 | 20 | 20 | 9 | 50 | 35 | 11 |
| urlparser | C2SaferRust | 21 | 19 | 18 | 18 | 9 | 47 | 30 | 9 |
| urlparser | CROWN | 21 | 20 | 19 | 19 | 8 | 49 | 33 | 11 |
| urlparser | Laertes | 21 | 21 | 20 | 20 | 3 | 48 | 34 | 17 |
| quadtree | c2rust | 24 | 17 | 17 | 17 | 16 | 128 | 11 | 1 |
| quadtree | CROWN | 24 | 13 | 13 | 13 | 10 | 82 | 11 | 3 |
| quadtree | PtrTrans | 24 | 11 | 11 | 11 | 11 | 87 | 0 | 18 |
| lodepng | c2rust | 235 | 64 | 54 | 54 | 47 | 879 | 55796 | 20 |
| lodepng | CROWN | 235 | 57 | 54 | 54 | 47 | 880 | 55985 | 20 |
| optipng | c2rust | 552 | 128 | 54 | 54 | 47 | 2134 | 14212 | 65 |
| optipng | C2SaferRust | 552 | 121 | 96 | 96 | 79 | 1792 | 49522 | 391 |
| optipng | Laertes | 552 | 121 | 55 | 55 | 48 | 999 | 15603 | 163 |

37 cells: 1854 planned of 4222 matched boundaries, 1662 built, 1662 executed, 1483 with a coverage export; 40558 corpus inputs in total.

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
| lil | C2SaferRust | TEST-UNAVAILABLE | — | 25/154 (0.162) | 25 | — | 362/5751 (0.063) | 362 |
| lil | CROWN | TEST-UNAVAILABLE | — | 127/134 (0.948) | 127 | — | 5294/6409 (0.826) | 5294 |
| lil | Laertes | TEST-UNAVAILABLE | — | 144/183 (0.787) | 144 | — | 5028/6143 (0.818) | 5028 |
| tulip | c2rust | PASS | 213/213 (1.000) | 212/213 (0.995) | 0 | 8616/9298 (0.927) | 3197/9298 (0.344) | 73 |
| tulip | C2SaferRust | TEST-FAILS | — | 212/213 (0.995) | 212 | — | 3167/9306 (0.340) | 3167 |
| tulip | CROWN | PASS | 213/213 (1.000) | 212/213 (0.995) | 0 | 8616/9219 (0.935) | 3197/9219 (0.347) | 73 |
| tulip | Laertes | TEST-FAILS | — | 211/216 (0.977) | 211 | — | 3168/13191 (0.240) | 3168 |
| qsort | c2rust | TEST-UNAVAILABLE | — | 3/3 (1.000) | 3 | — | 55/55 (1.000) | 55 |
| qsort | Laertes | TEST-UNAVAILABLE | — | 3/3 (1.000) | 3 | — | 62/62 (1.000) | 62 |
| qsort | C2SaferRust | TEST-UNAVAILABLE | — | 3/3 (1.000) | 3 | — | 57/57 (1.000) | 57 |
| qsort | CROWN | TEST-UNAVAILABLE | — | 3/3 (1.000) | 3 | — | 61/61 (1.000) | 61 |
| qsort | SACTOR | TEST-UNAVAILABLE | — | 6/8 (0.750) | 6 | — | 99/196 (0.505) | 99 |
| qsort | PtrTrans | TEST-UNAVAILABLE | — | 3/3 (1.000) | 3 | — | 105/120 (0.875) | 105 |
| urlparser | c2rust | PASS | 21/22 (0.955) | 7/22 (0.318) | 0 | 893/1202 (0.743) | 110/1202 (0.092) | 5 |
| urlparser | C2SaferRust | TEST-FAILS | — | 7/24 (0.292) | 7 | — | 107/1183 (0.090) | 107 |
| urlparser | CROWN | PASS | 20/21 (0.952) | 7/21 (0.333) | 0 | 839/1143 (0.734) | 98/1143 (0.086) | 7 |
| urlparser | Laertes | TEST-FAILS | — | 3/25 (0.120) | 3 | — | 41/1477 (0.028) | 41 |
| quadtree | c2rust | PASS | 24/24 (1.000) | 20/24 (0.833) | 0 | 406/436 (0.931) | 220/436 (0.505) | 2 |
| quadtree | CROWN | TEST-FAILS | — | 17/47 (0.362) | 17 | — | 208/610 (0.341) | 208 |
| quadtree | PtrTrans | TEST-UNAVAILABLE | — | 13/19 (0.684) | 13 | — | 134/407 (0.329) | 134 |
| lodepng | c2rust | TEST-UNAVAILABLE | — | 54/236 (0.229) | 54 | — | 1675/13260 (0.126) | 1675 |
| lodepng | CROWN | TEST-UNAVAILABLE | — | 54/257 (0.210) | 54 | — | 1449/14332 (0.101) | 1449 |
| optipng | c2rust | TEST-UNAVAILABLE | — | 106/555 (0.191) | 106 | — | 9970/37840 (0.263) | 9970 |
| optipng | C2SaferRust | TEST-UNAVAILABLE | — | 144/564 (0.255) | 144 | — | 9815/37297 (0.263) | 9815 |
| optipng | Laertes | TEST-UNAVAILABLE | — | 71/820 (0.087) | 71 | — | 6611/49009 (0.135) | 6611 |

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
| lil | C2SaferRust | 0 | 57 | 413 | 3 | 1 | C9 |
| lil | CROWN | 0 | 67 | 218 | 61 | 45 | C10 |
| lil | Laertes | 0 | 0 | 662 | 52 | 102 | — |
| tulip | c2rust | 0 | 0 | 243 | 0 | 0 | — |
| tulip | C2SaferRust | 5 | 4 | 242 | 3 | 4 | S15 |
| tulip | CROWN | 0 | 0 | 243 | 0 | 1 | — |
| tulip | Laertes | 0 | 2 | 244 | 0 | 35 | C11 |
| qsort | c2rust | 0 | 0 | 0 | 0 | 0 | — |
| qsort | Laertes | 0 | 0 | 0 | 0 | 0 | — |
| qsort | C2SaferRust | 0 | 0 | 0 | 200 | 0 | — |
| qsort | CROWN | 0 | 0 | 0 | 0 | 0 | — |
| qsort | SACTOR | 0 | 0 | 0 | 0 | 0 | — |
| qsort | PtrTrans | 57 | 0 | 0 | 0 | 0 | S6 |
| urlparser | c2rust | 0 | 0 | 33 | 0 | 13 | — |
| urlparser | C2SaferRust | 0 | 0 | 26 | 1 | 12 | — |
| urlparser | CROWN | 0 | 0 | 33 | 0 | 11 | — |
| urlparser | Laertes | 0 | 18 | 33 | 0 | 0 | C12 |
| quadtree | c2rust | 0 | 0 | 3 | 0 | 9 | — |
| quadtree | CROWN | 0 | 0 | 9 | 0 | 5 | — |
| quadtree | PtrTrans | 0 | 0 | 18 | 0 | 0 | — |
| lodepng | c2rust | 0 | 0 | 1837 | 0 | 7 | — |
| lodepng | CROWN | 0 | 0 | 1870 | 0 | 7 | — |
| optipng | c2rust | 0 | 0 | 439 | 208 | 63 | — |
| optipng | C2SaferRust | 397 | 771 | 1421 | 31 | 122 | S1, S2, C13, S16, S19, S20, C15, S21, C16 |
| optipng | Laertes | 97 | 0 | 419 | 410 | 73 | S4, S17, S18 |

Confirmed across all cells: 1169 value divergences, 1156 terminations, before clustering (one site = one defect; a producer's crash counts once and blocks its dependants).

## 4. Defects the RQ4 cells found or re-found

| id | library × tool | family | found by |
|---|---|---|---|
| C7 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| C8 | bzip2 × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| C9 | lil × C2SaferRust | byte-string domain narrowing | NEW — found by the RQ4 plan pipeline |
| C10 | lil × CROWN | null/empty conflation | NEW — found by the RQ4 plan pipeline |
| S15 | tulipindicators × C2SaferRust | semantic computation substitution | NEW — found by the RQ4 plan pipeline |
| C11 | tulipindicators × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| S1 | optipng (zlib) × C2SaferRust | null/empty conflation | catalogued earlier; re-found by the RQ4 cell |
| S2 | optipng (zlib) × C2SaferRust | null/empty conflation | catalogued earlier; re-found by the RQ4 cell |
| S3 | bzip2 × Laertes | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S4 | optipng (zlib) × Laertes | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S5 | genann × SACTOR | initialization loss or corruption | catalogued earlier; re-found by the RQ4 cell |
| S6 | qsort × PtrTrans | interface-contract loss | catalogued earlier; re-found by the RQ4 cell |
| S10 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| S11 | bzip2 × CROWN | ownership-state corruption | catalogued earlier; re-found by the RQ4 cell |
| S14 | bzip2 × C2SaferRust | semantic computation substitution | catalogued earlier; re-found by the RQ4 cell |
| C12 | urlparser × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| C13 | optipng × C2SaferRust | null/empty conflation | NEW — found by the RQ4 plan pipeline |
| S16 | optipng × C2SaferRust | byte-string domain narrowing | NEW — found by the RQ4 plan pipeline |
| S17 | optipng × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| S18 | optipng (zlib) × Laertes | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| S19 | optipng (zlib) × C2SaferRust | semantic computation substitution | NEW — found by the RQ4 plan pipeline |
| S20 | optipng (zlib) × C2SaferRust | semantic computation substitution | NEW — found by the RQ4 plan pipeline |
| C15 | optipng (zlib) × C2SaferRust | initialization loss or corruption | NEW — found by the RQ4 plan pipeline |
| S21 | optipng (zlib) × C2SaferRust | semantic computation substitution | NEW — found by the RQ4 plan pipeline |
| C16 | optipng (libpng) × C2SaferRust | semantic computation substitution | NEW — found by the RQ4 plan pipeline |

