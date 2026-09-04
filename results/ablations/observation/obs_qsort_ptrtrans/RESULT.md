# OBS pilot — qsort × PtrTrans (seed 42)

Question: which observation channel makes the PtrTrans quicksort bug (wrong `split_at_mut` index,
silenced by the None-no-op swap; see results/rq4_effectiveness/bugs/qsort_ptrtrans/README.md) visible, under a
silent-consumer driver vs an array-printing driver?

Commit `dda70a4d5228453aa8850ae7b59b2bb033680065` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; 104 valid records)

| driver \ channel | O-R (return only) | O-P (stdout+exit) | O-S (ret+array+globals) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent-consumer | **0 / none** (blind) | **0 / none** (blind) | **71 / 0.013 s** | **71 / 0.013 s** |
| array-printing  | **0 / none** (blind) | **71 / 0.013 s** | **71 / 0.013 s** | **71 / 0.013 s** |

Expected pattern (O-R blind both; O-S,O-F detect both; O-P blind under silent, detects under printing):
**HOLDS exactly.** O-R is blind because `quickSort` is void and the translation never fails to return
(no panic, exit 0 always). O-P under the silent driver is blind because nothing about the wrong sort
reaches stdout or the exit code. Every channel that can see the array sees 71/104.

First divergence is the very first corpus file (index 0, `00a3050b…`, n=33: C sorted `-1005927195 -989855746 -39322 -1 …`, Rust `-1 1717976575 1717986918 …`, unsorted) for
every detecting cell, so time-to-first-divergence is the per-input cost (~13 ms = 5 subprocess launches).

## Corpus
- Real coverage-guided libFuzzer run (cargo-fuzz, `fuzz/qsort_ptrtrans_e3` target `q_ft`, copied to the
  scratchpad with a FRESH empty corpus dir so the E3 corpus was not touched): `-seed=42 -max_total_time=300 -max_len=1024`.
  310 s wall, 6,466,519 execs, coverage saturated at 46 edges / 237 features within seconds; final corpus **104 files** (18 KB).
  Saved verbatim in `corpus_seed42/` and replayed through all 8 cells (4 binaries × 1 execution each; channels are projections of the same execution).
- Decoding is identical in the fuzz target and both drivers: raw bytes → i32 LE chunks, take 256. Inputs with n<2 are
  kept (driver still calls `quickSort(a,0,n-1)`), unlike the fuzz target's early return.

## UB gate / validity
- C side: clang `-fsanitize=address,undefined -fno-sanitize-recover=all`, `halt_on_error=1`; both driver modes run; then a
  second C replay for the C-unstable check.
- **Excluded for C-UB: 0. C-unstable: 0. Valid differential records: 104 / 104.**

## Classification (per input, 104 valid)
C-UB 0 · C-unstable 0 · Rust-failure 0 (no panic, no nonzero exit anywhere) · **semantic-difference 71** · abstention 0 · agree 33.
Agree breakdown by input shape: already-sorted input 31, constant array 1, n≤1 1 — i.e. **every input that actually
needs sorting diverged (71/71)**, and every agreeing input is one where the identity permutation is the answer.
(Rate is higher than the README's 68 % on random arrays because the libFuzzer corpus is coverage-minimised toward
branchy, unsorted inputs.)

## Channel definitions as implemented (harness/)
- Drivers `driver.c` / `main.rs`: argv `silent|print`, argv[2] = state file. Both write `ret:void / globals:none / arr:…`
  to the state file AFTER the call returns (never to stdout). `print` additionally prints the array to stdout. qsort has
  no globals, recorded as `none` on both sides.
- O-R = `"void"` iff the state file exists (call returned) else `NO-RETURN`. O-P = (exit code, stdout bytes).
  O-S = (O-R, state file). O-F = (O-S, O-P). A cell diverges when the C and Rust projections differ.

## What had to be fixed / deviations
- The existing `gen_and_diff.py` is a batched 50k-case driver (one process for all trials), which cannot express
  O-P (per-process stdout/exit) or a silent consumer; new per-input drivers were written (harness/), same decoding as the libFuzzer target.
- `fuzz/qsort_ptrtrans_e3/fuzz/corpus/q_ft` already held 212 E3 files; to keep "generated once with seed=42" honest the
  fuzzer was run in a scratchpad copy with an empty corpus dir. Nothing under fuzz/ or results/rq4_effectiveness/bugs/ was modified.
- Nothing else failed. Whole pipeline: 5 min fuzz + 1.2 s replay.
- Caveat: with 104 inputs and detection on input #0, time-to-first-divergence does not discriminate between detecting cells here;
  that needs a larger/randomised corpus order in the scaled run.

## Files
`result.json`, `raw/libfuzzer_seed42.log`, `raw/replay_summary.json`, `raw/replay_records.jsonl` (per-input C/Rust arrays,
per-cell divergence flags), `corpus_seed42/`, `harness/{driver.c,main.rs,Cargo.toml,replay.py,q_ft.rs}`.
