# OBS matrix cell — crc32_z × C2SaferRust (optipng zlib; seed 42)

Question: which observation channel sees the C2SaferRust `crc32_z` NULL-vs-empty conflation
(`buf.is_null()` → `buf.is_empty()`: a zero-length chunk resets the running CRC to 0; see
results/rq4_effectiveness/bugs/crc32_c2saferrust/README.md) under a silent-consumer vs a printing driver?
Defect class: **silent, return-value-visible** (the wrong CRC is the function's return value).

Commit `747f5f09eac314bc4e32687e71497e269af36848` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; 65 valid records)

| driver \ channel | O-R (return) | O-P (stdout+exit) | O-S (ret+globals) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent-consumer | **15 / 0.041 s** | **0 / none** (blind) | **15 / 0.041 s** | **15 / 0.041 s** |
| printing        | **15 / 0.041 s** | **15 / 0.041 s** | **15 / 0.041 s** | **15 / 0.041 s** |

Expected pattern (O-R already detects; O-P blind only under the silent consumer): **HOLDS.**
O-R detects because the defect is in the returned value. O-P under the silent driver is blind (exit 0, no
stdout on both sides); once the driver prints the CRC, O-P sees exactly the same 15. O-S adds nothing over O-R here
(no designated output memory; the WIP `crc_table` global is read-only and identical on both sides).
First divergence = corpus index 2 (`129d49db…`): C `ret:0x0000003d`, Rust `ret:0x00000000` (seed crc 0x3d, then an
empty chunk → reset to 0).

## Corpus
- Real coverage-guided libFuzzer (cargo-fuzz, nightly-2025-09-01) on the C2SaferRust `crc32_z` with the driver decoding,
  FRESH empty corpus dir: `-seed=42 -max_total_time=300 -max_len=1024`. 301 s, **95,624,711 execs**, cov 68 edges / 253
  features (saturated within seconds), final corpus **65 files** (3.8 KB). Saved verbatim in `corpus_seed42/`.
- Decoding (identical in fuzz target, C driver, Rust driver): bytes[0..4] = seed crc (u32 LE); then a chunk stream of
  `1 byte L, min(L, remaining) bytes` → `crc = crc32_z(crc, chunk, L)`. This is the incremental shape optipng uses for the
  IDAT CRC (optim.rs:1612); `L == 0` is a legal empty write segment.
- Post-hoc check: every one of the 15 divergent inputs contains an empty chunk while the running CRC is nonzero, and
  **none** of the 50 agreeing inputs does (15/15 vs 0/50) — the divergence set is exactly the defect's trigger condition.

## UB gate / validity
- C side: zlib **1.2.11** `crc32.c` (the version optipng vendors, `zutil.rs: "1.2.11-optipng"`; fetched from
  zlib.net/fossils, tarball sha256 `c3e5e9fd…cb1a1`, because no in-repo C zlib copy has `crc32_z`), clang
  `-fsanitize=address,undefined -fno-sanitize-recover=all`, `halt_on_error=1`; both driver modes; second C replay for C-unstable.
- **Excluded for C-UB: 0. C-unstable: 0. Valid differential records: 65 / 65.**

## Classification (per input, 65 valid)
C-UB 0 · C-unstable 0 · Rust-failure 0 · **semantic-difference 15** · abstention 0 · agree 50.

## Channel definitions as implemented (harness/)
- `driver.c` / `main.rs`: argv `silent|print`, argv[2] = state file. State file `ret:0x%08x / chunks:n / globals:none`
  written AFTER the call (never to stdout). `print` additionally prints `crc=0x…` to stdout.
- O-R = the `ret:` line (NO-RETURN if no state file). O-P = (exit code, stdout). O-S = (O-R, state file). O-F = (O-S, O-P).
- Rust side = `results/rq4_effectiveness/bugs/crc32_c2saferrust/src/wip.rs` (C2SaferRust `crc32_z`, verbatim extraction) as a lib.

## What had to be fixed / deviations
- The only in-repo C zlib copies (rustassure's optipng, PtrTrans crown_dataset) predate `crc32_z` (zlib <1.2.9) —
  compile error. Fixed by fetching zlib 1.2.11 (exact vendored version). Nothing under tools/ or results/rq4_effectiveness/bugs/ touched.
- Replay script is the shared generic one (`harness/replay.py`, derived from the qsort pilot; O-R parses the `ret:` line
  instead of assuming void). Whole pipeline: 5 min fuzz + 0.8 s replay.

## Files
`result.json`, `raw/libfuzzer_seed42.log`, `raw/replay_summary.json`, `raw/replay_records.jsonl`, `raw/replay_stdout.log`,
`corpus_seed42/`, `harness/{driver.c,main.rs,lib.rs,Cargo.toml,ft.rs,replay.py,NOTE.txt}`.
