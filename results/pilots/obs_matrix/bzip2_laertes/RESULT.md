# OBS matrix cell — bzip2 × Laertes (seed 42)

Question: which observation channel makes the Laertes bzip2 defect visible — `BZ2_crc32Table` is a zeroed
static whose `laertes_init_BZ2_crc32Table` is never called, so `BZ2_bzBuffToBuffCompress` returns `BZ_OK`
but the output stream carries a wrong CRC (results/rq1_bugs/bzip2_laertes/README.md) — under a
silent-consumer driver vs a stream-printing driver?

Commit `747f5f09eac314bc4e32687e71497e269af36848` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; 529 valid records)

| driver \ channel | O-R (return code) | O-P (stdout+exit) | O-S (ret+destLen+out buffer+globals) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent-consumer | **0 / none** (blind) | **0 / none** (blind) | **528 / 0.021 s** | **528 / 0.021 s** |
| stream-printing | **0 / none** (blind) | **528 / 0.021 s** | **528 / 0.021 s** | **528 / 0.021 s** |

Expected pattern (O-R blind; O-S detects via the output buffer; O-P detects only when the driver prints the
stream): **HOLDS exactly.** `rc == 0 (BZ_OK)` on both sides for all 529 inputs, so a return-value oracle sees
nothing; the process exit code is 0 everywhere. Every channel that can see the output buffer sees 528/529.

Divergence shape (checked on all 528): output length identical to C; the bytes differ **only** at the block-CRC
field (stream offset 10..13) and/or the trailer combined-CRC — the Huffman payload is byte-identical. This is
the pure checksum-corruption signature (cf. C's `19 93 9b 6b` vs Laertes' `00 00 00 ff` for `"A"` in the README).
The single agreeing input is `ff ff ff ff 00 00` (6 bytes), where the zeroed-table recurrence happens to
coincide with the real CRC.

First divergence is corpus file #0 for every detecting cell, so time-to-first-divergence (≈21 ms = 5 subprocess
launches) is the per-input cost and does not discriminate between detecting cells here.

## Corpus
- Real coverage-guided libFuzzer run (cargo-fuzz 0.13, `+nightly-2025-09-01`) on a scratchpad copy of
  `fuzz/bzip2_laertes_e3` with a new target `obs_ft` (Laertes compress only, **same decoding as the drivers**)
  and a FRESH empty corpus dir: `-seed=42 -max_total_time=300 -max_len=4096 -fork=1 -ignore_crashes=1`.
  317 s wall, 641,171 execs, cov 895 edges / 3,756 features, **0 crashes**; final corpus **529 files** (2.1 MB,
  sizes 1–330 B, median 51 B). Saved verbatim in `corpus_seed42/`.
- Decoding (fuzz target, C driver, Rust driver identical): stdin bytes = source buffer (cap 64 KiB);
  `BZ2_bzBuffToBuffCompress(dest, &dl, src, len, blockSize100k=1, verbosity=0, workFactor=30)`, one call.

## UB gate / validity
- C side: `tools/frameworks/crown/c-code/bzip2/*.c` with clang `-fsanitize=address,undefined -fno-sanitize-recover=all`,
  `halt_on_error=1`; both driver modes run; second C replay for the C-unstable check.
- **Excluded for C-UB: 0. C-unstable: 0. Valid differential records: 529 / 529.**

## Classification (per input, 529 valid)
C-UB 0 · C-unstable 0 · **Rust-failure 0** (no panic, no nonzero exit; the zeroed-`BZ2_rNums` crash path was not
reached at max_len 4096 / block size 1) · **semantic-difference 528** · abstention 0 · agree 1.

## Channel definitions as implemented (harness/)
- `driver.c` / `obs_rs_driver.rs`: argv `silent|print`, argv[2] = state file. Both write
  `ret:<rc> / destLen:<n> / globals:none / out:<hex>` to the state file AFTER the call (never to stdout).
  `print` additionally prints `rc=<rc> len=<n>` and the hex stream to stdout. bzip2 has no relevant globals beyond the
  (zeroed) tables themselves, recorded as `none` on both sides.
- O-R = first state line `ret:<rc>` (NO-RETURN if no state file). O-P = (exit code, stdout). O-S = (O-R, state file).
  O-F = (O-S, O-P). A cell diverges when C and Rust projections differ. `replay.py` is the shared generic OBS replay.

## What had to be fixed / deviations
- No archived corpus existed for this defect with fixed params (the E3 target randomises blockSize/workFactor from
  the first two bytes); a new fixed-parameter target was written so every corpus input is valid for the C oracle.
- `-fork=1 -ignore_crashes=1` was used pre-emptively; no crash actually occurred. Nothing else failed.
  Whole pipeline: 5 min fuzz + 7.2 s replay.

## Files
`result.json`, `raw/libfuzzer_seed42.log`, `raw/replay_summary.json`, `raw/replay_records.jsonl`, `raw/replay_stdout.log`,
`corpus_seed42/`, `harness/{driver.c,obs_rs_driver.rs,obs_ft.rs,replay.py}`.
Build scratch: `/tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/obsmx/bzip2_laertes/`.
