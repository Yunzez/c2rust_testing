# OBS matrix cell — qsort × C2SaferRust (seed 42) — Rust-failure branch

Question: how does the C2SaferRust `quickSort` crash (`int→usize` index, `i.wrapping_sub(1)` → runaway recursion
/ OOB; results/rq4_effectiveness/bugs/qsort_c2saferrust/README.md) project onto the 4 observation channels under a
silent-consumer vs an array-printing driver? Expected: all channels see it, and it must be recorded as
**Rust-failure**, not semantic-difference.

Commit `747f5f09eac314bc4e32687e71497e269af36848` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; MERGED set, 113 valid records)

| driver \ channel | O-R (return only) | O-P (stdout+exit) | O-S (ret+array+globals) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent-consumer | **51 / 2.28 s** | **51 / 2.28 s** | **51 / 2.28 s** | **51 / 2.28 s** |
| array-printing  | **51 / 2.28 s** | **51 / 2.28 s** | **51 / 2.28 s** | **51 / 2.28 s** |

**Expected pattern HOLDS.** Every channel sees every failing input, in both driver modes. All 51 divergent records are
`Rust-failure` (SIGABRT, rc=−6, `thread 'main' has overflowed its stack`); **semantic-difference = 0**.
- O-R sees it as **`NO-RETURN`**: the Rust driver never gets past the `quickSort` call, so the state file (written only
  after the call returns) does not exist — the return channel observes "the call did not return", not a wrong value.
- O-P sees rc −6 vs 0 (and, in print mode, missing stdout). O-S/O-F inherit NO-RETURN + missing state.
- Time-to-first is input #0 in every cell; the 2.28 s is the cost of the first crashing input (stack-overflow abort +
  core dump ≈ 1.1 s per process × 2 driver modes), not a search time.

Per-set breakdown (same 8 cells, all channels identical within a set):

| set | files | valid | Rust-failure | agree | divergences per cell | first div |
|---|---|---|---|---|---|---|
| `corpus_seed42/` (fresh libFuzzer, seed 42) | 3 | 3 | 0 | 3 | 0 | none |
| `corpus_crashes/` (libFuzzer-saved unique crashes, seed 42) | 6 | 6 | 6 | 0 | 6 | 2.26 s (#0) |
| `corpus_archived_seed42/` (obs_qsort_ptrtrans pilot corpus, identical decoding) | 104 | 104 | 45 | 59 | 45 | 2.27 s (#0) |
| merged | 113 | 113 | 51 | 62 | 51 | 2.28 s (#0) |

Agree breakdown (62): already-sorted 33 · n≤1 3 · **unsorted but never triggering 26** (the crash needs `partition` to
return index 0 at some recursion level, i.e. the range minimum chosen as pivot; 45/71 unsorted archived inputs do).

## Corpus
- Real coverage-guided libFuzzer (cargo-fuzz, toolchain nightly-2025-09-01) on a scratchpad crate holding the WIP lib
  verbatim (`harness/c2saferrust_wip_lib.rs` = fuzz/qsort_c2saferrust/src/lib.rs = laertes_benchmarks/qsort_WIP/qsort.rs
  minus crate attributes), target `harness/q_ft.rs`, FRESH empty corpus dir:
  `-seed=42 -max_total_time=300 -max_len=1024 -fork=1 -ignore_crashes=1`. 309 s wall, 2,018,124 execs, cov 71 edges /
  72 features, **10,398 fork jobs ended in a crash**, 6 unique crash artifacts saved, **final corpus only 3 files**
  (1 B, 4 B, 8 B).
- **What did not work:** the crash cell cannot build its own corpus. In fork mode each child dies on (nearly) the first
  unsorted n≥2 input it mutates to, so coverage never accumulates beyond the trivial 3 files (same phenomenon as the
  E3 "crash-cell can't depth-census itself" note). Fix applied: replay additionally (a) the 6 saved crash inputs and
  (b) the archived seed-42 corpus of the obs_qsort_ptrtrans pilot (104 files; byte-identical decoding in fuzz target and
  both drivers), and report per-set + merged tables. The fresh 3-file corpus alone shows 0 divergences in all cells —
  stated plainly; it is not a detection failure of any channel, it is a corpus with no triggering input.
- Nothing under fuzz/ or results/rq4_effectiveness/bugs/ was modified; the archived corpus was copied, not touched.

## UB gate / validity
C side: clang `-O1 -g -fsanitize=address,undefined -fno-sanitize-recover=all`, `halt_on_error=1`, both driver modes, then a
second C replay (C-unstable). **C-UB 0 · C-unstable 0 · valid 113/113.** Per-process timeout 10 s (`OBS_TIMEOUT`); no
timeouts occurred — the runaway recursion always ends in a stack-overflow abort well under 10 s.

## Classification (113 valid)
C-UB 0 · C-unstable 0 · **Rust-failure 51** · semantic-difference 0 · abstention 0 · agree 62.

## Channel definitions as implemented (harness/)
`driver.c` = template driver verbatim (`#include "qsort.c"` = results/rq4_effectiveness/bugs/qsort_c2saferrust/source/qsort.c). `main.rs` =
template driver with the reshaped call `quickSort(&mut a[..], 0, n-1)` (usize); for n≤1 the call is skipped (the C
contract `high=n-1` would be `usize::MAX`; C's `low<high` is false there anyway, so this mirrors the C no-op — the
archived bridge target returns early for empty input the same way). State file `ret:void / globals:none / arr:…` written
after the call, never to stdout; `print` additionally prints the array. O-R = first state line iff the state file exists
else `NO-RETURN`; O-P = (exit code, stdout); O-S = (O-R, state); O-F = (O-S, O-P). Shared generic `replay.py`.

## Files
`result.json`, `raw/libfuzzer_seed42.log`, `raw/replay_{fresh,crashes,archived,merged}/{replay_summary.json,replay_records.jsonl}`,
`raw/replay_*_stdout.log`, `corpus_seed42/` (3), `corpus_crashes/` (6), `corpus_archived_seed42/` (104),
`harness/{driver.c,main.rs,Cargo.toml,c2saferrust_wip_lib.rs,q_ft.rs,replay.py}`.
Build dir: /tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/obsmx/qsort_c2saferrust/
