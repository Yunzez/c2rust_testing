# OBS matrix cell — cJSON × PtrTrans `parse_string` (seed 42)

Question: which observation channel sees the PtrTrans `parse_string` defect cluster
(results/rq4_effectiveness/bugs/cjson_ptrtrans/README.md: (1) `\u` escapes / (3) non-UTF-8 bytes rejected → return value 0 vs 1;
(2) parse succeeds but `item.valuestring` is silently `None` → return values equal, struct field differs)?

Commit `747f5f09eac314bc4e32687e71497e269af36848` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; 139 valid records)

| driver \ channel | O-R (return only) | O-P (stdout+exit) | O-S (ret+item fields+offset) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent-consumer | **31 / 0.086 s** (idx 6) | **0 / none** (blind) | **113 / 0.015 s** (idx 0) | **113 / 0.015 s** |
| printing        | **31 / 0.086 s** (idx 6) | **111 / 0.015 s** (idx 0) | **113 / 0.015 s** (idx 0) | **113 / 0.015 s** |

## Per-class split of the 113 semantic-difference records (the point of this cell)

| class | records | O-R | O-P silent | O-P print | O-S | O-F |
|---|---|---|---|---|---|---|
| (a) return differs (C ret=1, Rust ret=0): 30 non-UTF-8 payloads + 1 `\u` escape | **31** | sees all 31 | blind | sees all 31 | sees all 31 | sees all 31 |
| (b) return equal (both ret=1), `valuestring` C=bytes / Rust=`NULL` — the silent value-loss defect | **80** | **blind (0/80)** | blind | sees all 80 | sees all 80 | sees all 80 |
| (c) return AND valuestring equal (both fail, ret=0), only `offset` differs (C advances past the bad char, Rust does not) | **2** | blind | blind | blind (offset not printed) | sees both | sees both |

Expected pattern — O-R blind to the valuestring loss, O-S detects it, O-P detects it only under the printing driver:
**HOLDS for class (b) exactly** (O-R 0/80; O-S 80/80; O-P 0/80 silent vs 80/80 print).
Refinement the single-number table hides: O-R is *not* blind to the cell as a whole (31/113) because the sibling
classes (a) flip the return code — so a return-only oracle would "find a bug" in this cell while missing the
value-loss defect entirely, and would attribute the cell to the wrong root cause (UTF-8 gate rather than discarded
value). Class (c) is a new, minor, O-S-only divergence (failure-path `offset` bookkeeping) not in the archived README.
First divergence: idx 0 (`003a28cc…`, payload `"="…`, C valuestring `3d`, Rust `NULL`) for every O-S/O-F/O-P-print cell;
O-R's first hit is idx 6 (`05712e99…`, 24 bytes containing `0xff` → C ret=1, Rust ret=0).

## Corpus
- Real coverage-guided libFuzzer (cargo-fuzz, toolchain nightly-2025-09-01), target `harness/obs_ps.rs` = one
  `parse_string` call on the whole input (identical decoding to both drivers), scratchpad copy of the translated crate,
  FRESH empty corpus dir, `-seed=42 -max_total_time=300 -max_len=256 -dict=obs.dict`
  (dictionary: `\u`, `\ud834`, `\udd1e`, `A`, `"`, `\\`, `\n`, `\/`, `\b`, `\xff\xfe`).
  301 s wall, 82,789,918 execs, cov 104 edges / 329 features (saturated early), final corpus **139 files** (3,122 B; len 1–136, median 10).
  Saved verbatim in `corpus_seed42/`, replayed through all 8 cells (4 binaries × 1 execution; channels are projections of the same execution).
- The `\u`-escape sub-class is under-represented (1 record) because the empty-`input_end` bug makes every `\u` path a
  1-edge early return, so libFuzzer gets no coverage reward for deeper escape sequences; this is a corpus property, not a channel property.

## UB gate / validity
- C side: cJSON **v1.7.19** (`harness/cJSON.c.sha256`; the translated crate's version string is 1.7.19; no cJSON.c with the
  1.7.x `parse_buffer` API exists in the repo — the CRUST-bench/oldc2rust copies are the pre-1.7 API — so it was fetched from
  upstream at tag v1.7.19). Built `clang -O1 -g -fsanitize=address,undefined -fno-sanitize-recover=all`, `halt_on_error=1`;
  both driver modes run; second C replay for the C-unstable check.
- **Excluded for C-UB: 0. C-unstable: 0. Valid differential records: 139 / 139.**

## Classification (per input, 139 valid)
C-UB 0 · C-unstable 0 · Rust-failure 0 (no panic, exit 0 everywhere) · **semantic-difference 113** · abstention 0 · agree 26.
Agree = inputs where both sides fail identically (not starting with `"`, unterminated, bad escape) with the same offset.

## Channel definitions as implemented (harness/)
- Drivers `driver.c` (C, `#include "cJSON.c"` to reach the static `parse_string`) / `obsdrv.rs` (bin inside a copy of the
  PtrTrans crate; raw-pointer laundering as in the archived `diffdrv.rs`): argv `silent|print`, argv[2] = state file, stdin =
  the raw parse-buffer content, one `parse_string(&item,&buffer)` call at offset 0. Both write
  `ret:<0|1>\ntype:<item.type>\nvaluestring:<hex|NULL>\noffset:<buffer.offset>\nglobals:none` AFTER the call (never to stdout).
  `print` additionally prints `ret=<r> valuestring=<hex|NULL>` to stdout (deliberately not `offset`/`type`).
- O-R = the `ret:` line (`NO-RETURN` if no state file). O-P = (exit code, stdout). O-S = (O-R, state file). O-F = (O-S, O-P).
- Rust hooks are the archived fake allocator (dangling non-null pointer, no-op free), as in diffdrv.rs / the E3 fuzz target.

## What had to be fixed / deviations
- No 1.7-API `cJSON.c` on disk (see above) → fetched v1.7.19 from GitHub; the archived oracle.c was written against the same API.
- libFuzzer dictionary: a backslash must be written `\\` inside the quoted token (first attempt `"\u"` aborted the run with
  `ParseDictionaryFile: error in line 1`; corpus dir was wiped and the run restarted from empty).
- The generic replay script (`harness/replay.py`, shared across obs_matrix cells) replaces the qsort-specific one: O-R is the
  first state-file line instead of the void sentinel.
- Whole pipeline: 5 min fuzz + 1.6 s replay. Nothing else failed.

## Exact commands
```
cd $B/c && clang -O1 -g -fsanitize=address,undefined -fno-sanitize-recover=all -I. driver.c -o driver_c -lm
cd $B/rs && cargo +nightly-2025-09-01 build --release --bin obsdrv
cd $B/rs && cargo +nightly-2025-09-01 fuzz run obs_ps fuzz/corpus/obs_ps -- -seed=42 -max_total_time=300 -max_len=256 -dict=fuzz/obs.dict
OBS_TIMEOUT=30 python3 replay.py corpus_seed42 raw c/driver_c rs/target/release/obsdrv
```
($B = scratchpad obsmx/cjson_ptrtrans; $B/rs = copy of results/rq4_effectiveness/bugs/cjson_ptrtrans/translated_crate with `src/bin/obsdrv.rs` + `fuzz/`.)

## Files
`result.json`, `raw/libfuzzer_seed42.log`, `raw/replay_stdout.log`, `raw/replay_summary.json`, `raw/replay_records.jsonl`
(per-input C/Rust state, per-cell divergence flags), `corpus_seed42/`,
`harness/{driver.c,obsdrv.rs,obs_ps.rs,fuzz_Cargo.toml,obs.dict,replay.py,cJSON.c.sha256}`.
