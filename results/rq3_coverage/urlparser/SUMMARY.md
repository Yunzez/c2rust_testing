# RQ4 — reach of the generated campaign: urlparser

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md)
(§2 amendment 2026-09-07); plan: [`../../../docs/rq4_plan_ten_libraries.md`](../../../docs/rq4_plan_ten_libraries.md);
denominator: [`../../../docs/rq4_denominator_decision_2026-09-08.md`](../../../docs/rq4_denominator_decision_2026-09-08.md).
Status 2026-09-09: **four cells complete and verified** (`check_cell.py`), 3 600 s each, one campaign and
one corpus per cell, `-max_len 4096`, seed 42, generator **0.8**. C source: jwerle's `url.h`, a header-only
library whose translation unit is its own test program (`test.c`); `main` is excluded from the pair.

**Tests side** (recorded, not compared in the paper — RQ4 reframing): the transpiled test program **passes**
on c2rust (21/22 functions) and CROWN (20/21); it **fails** on Laertes (SIGSEGV) and C2SaferRust (double
free), so those two are denominator-only. Every universe is the translation's own instrumented objects.

## Cell table

| tool | tests side | planned / built of 21 | exported | corpus | fn ours | reg ours | confirmed (sample) |
|---|---|---:|---:|---:|---|---|---|
| **c2rust** | PASS | 21 / 20 | 9 | 50 | 7 / 22 (0.318) | 110 / 1 202 (0.092) | **0** (30 ub_associated, 13 not reproducible, 3 ub_associated_termination) |
| **C2SaferRust** | TEST-FAILS | 19 / 18 | 9 | 47 | 7 / 24 (0.292) | 107 / 1 183 (0.090) | **0** (26 / 12 / 1 out_of_contract_access) |
| **CROWN** | PASS | 20 / 19 | 8 | 49 | 7 / 21 (0.333) | 98 / 1 143 (0.086) | **0** (30 / 11 / 3) |
| **Laertes** | TEST-FAILS | 21 / 20 | 3 | 48 | 3 / 25 (0.120) | 41 / 1 477 (0.028) | **18 `confirmed_termination`** on `url_is_protocol` + 5 boundaries through it → **C12** |

## What this library says

1. **One named cause for the low reach on every cell: the C reference.** `get_part` does `malloc(1)` +
   `sscanf` into it (url.h:208; E1 #27, upstream issue #3), a heap overflow on every well-formed URL, and
   9 of the 15 public boundaries go through it. Under the harness's sanitizer C provides no reference on
   those boundaries, so libFuzzer restarts on every input and they export nothing. The remaining
   boundaries are the small string helpers, hence 7 functions and ~9 % of regions on the three clean
   cells. Pre-registered per pair in `preflight_accept.txt`; every sampled input on them adjudicated
   `ub_associated`.
2. **A new defect on Laertes, C12.** `URL_SCHEMES[177]` is emitted as 177 NULL pointers with its initializer
   in `laertes_init_URL_SCHEMES()`, which nothing calls; `url_is_protocol` then `strcmp`s NULL. 3/3
   `confirmed_termination` (C normal, Rust SIGSEGV on the zero page without a sanitizer), and the five
   boundaries that call it (`url_parse`, `url_get_protocol`, `url_get_auth`, `url_data_inspect`,
   `url_free`) fail identically — one root cause, +1 in the *initialization loss* family. E1's severed-init
   scanner had filed url.h as a fixture: a false negative for a header-only library.
3. **c2rust, C2SaferRust and CROWN confirm nothing** (the negative control and two safety lifters).

## Gaps, deviations and limits

- Confirmation on c2rust / C2SaferRust / CROWN was recovered on 2026-09-09 from the archived candidate
  inputs with rebuilt binaries (the original post had skipped it after a regex edit of the chain script
  appended `--dest` to the confirm line); Laertes was re-run in full. Each RUN.md §7 records it.
- C2SaferRust: two boundaries have no lossless bridge (`&str` lift of a C byte string; one parameter
  fewer than C) — recorded, not counted as findings since no harness reached them.
- Region reach says nothing about translation quality here beyond the two crash-all families.

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md with §7 prose, funnel.json, plans.json, analysis/,
divergences/, confirm_sample/, candidates_sample/, corpus.tar.gz, harness_exports.tar.gz,
artifact_hashes.json, raw/denominator.json). Pairs: `benchmark/pairs/rq4/urlparser_{c2rust,c2saferrust,
crown,laertes}/` (+ `exclude.txt`, `preflight_accept.txt`, `PROVENANCE.json`). Manifest: **C12** (new).
