# OBS matrix cell — tulipindicators × C2SaferRust (seed 42, CLI boundary)

Question: at a **process/CLI boundary**, which observation channel sees C2SaferRust's rewritten `sample` driver
defects — bug 2 (`main()` wrapper computes `argc−1`, so `sample sma 5` → `*ERROR NOT ENOUGH OPTIONS*`, exit 1)
and bug 1 (zero args → `CStr::from_ptr(NULL)` SIGSEGV)? See `results/rq4_effectiveness/bugs/tulip_c2saferrust/README.md`.

Commit `747f5f09eac314bc4e32687e71497e269af36848` (branch dataset-v2.1). Seed 42. Single run, no variance claim.

## Table (divergences / time-to-first-divergence, wall-clock s from replay start; 201 valid records)

| driver \ channel | O-R (exit code) | O-P (stdout+exit) | O-S (= O-R: no output memory at CLI boundary) | O-F (O-S ∪ O-P) |
|---|---|---|---|---|
| silent (stdout discarded) | **48 / 2.41 s** | **48 / 2.41 s** | **48 / 2.41 s** | **48 / 2.41 s** |
| print (stdout forwarded)  | **48 / 2.41 s** | **78 / 2.41 s** | **48 / 2.41 s** | **78 / 2.41 s** |

First divergence is corpus record 0 (the canonical no-args record: C exit 1 + usage, Rust SIGSEGV) in every cell, so
time-to-first is just the per-input cost (~2.4 s = 5 sanitized-C + Rust launches through the Python wrapper); the second
record (`sma 5`) is the exit-code-visible argc bug (C 0 / Rust 1).

**Expected pattern ("O-P detects; O-S depends on whether the boundary is the CLI"): holds, with one honest addition —
O-R is NOT blind here.** The argc bug flips the exit code (0→1) whenever the options are supplied exactly, so the
return-value channel alone catches 47 of the 77 semantic differences plus the crash. What O-P-print adds over O-R
(30 records) is stdout-only: 21 records where both sides exit 1 but print different error paths (C `*ERROR NOT ENOUGH
OPTIONS*`+echo vs Rust `No indicator given.` — the same argc bug routed through the `argc < 2` guard), and 9 records
where both exit 0 but the printed table differs (alt-input `input` column dropped ×5, extra `close` column for `dx`/`adxr` ×4)
— a **separate, untriaged WIP `sample.rs` display divergence, not the argc bug**. O-S adds nothing over O-R at a CLI
boundary (state file = exit code + `globals:none`), exactly as the "depends on boundary" caveat predicts.
O-P-silent = O-R (48) because with stdout discarded only the exit code survives.

## Corpus — seeded generator, NOT coverage-guided
`harness/gen_corpus.py`, `random.seed(42)`: 2 canonical records (`0xFF` = no args; `sma 5`) + 104 one-per-indicator +
96 random-indicator records, k∈0..4 options each; 202 files in `corpus_seed42/`. Decoding (shared, in `harness/wrap.py`):
byte0 → indicator index into `harness/indicators.txt` (the 104 names from `sample_c --list`), byte1 → k = b%5, next k
bytes → option `b%20+1` as decimal argv strings. A libFuzzer run is not the natural mechanism when the boundary is a CLI
(argv, not a byte buffer); this is stated as a deviation from the qsort template.

## UB gate / validity
C `sample` = `sample.c + tiamalgamation.c`, clang `-fsanitize=address,undefined -fno-sanitize-recover=all`,
`halt_on_error=1:exitcode=99`; both driver modes run, then a second C replay (C-unstable).
**Cell-specific gate rule:** C exit 1 (usage/option error) is a normal return, not UB — an input is excluded only on a
sanitizer report / exit 99 / signal / timeout. Likewise Rust-failure = signal or panic(101) only.
**Excluded C-UB: 1** (`hma 1 4` — real ASan global-buffer-overflow in C's `sample.c`, period 1 → negative start index).
**C-unstable: 0. Valid differential records: 201 / 202.**

## Classification (201 valid)
C-UB 1 (excluded) · C-unstable 0 · **Rust-failure 1** (no args, SIGSEGV 139) · **semantic-difference 77** · abstention 0 · agree 123.
Visibility breakdown of the 78 non-agree records: exit-code-visible 47 · crash 1 · stdout-only/both-exit-1 21 · stdout-only/both-exit-0 9.
Why 123 agree: the argc bug only bites when `k == options needed`; over-supplied records (`k >` needed) pass the
`argc < 3+i` check on both sides and use the same `argv[2+i]` values → identical output; under-supplied by ≥2 → both error
identically (2 records). Argument-free indicators (k=0, options=0) agree trivially.

## Channel definitions as implemented (harness/)
One Python wrapper `wrap.py CLI silent|print STATEFILE` used identically for both sides (`wrap_c`, `wrap_rs` are 2-line
shell stubs pointing at `sample_c` / `target/release/sample`). Decodes stdin → argv, runs the CLI, then writes
`ret:<exit code>\nglobals:none\nargv:…` to the state file AFTER the CLI returns (never to stdout). `print` forwards the
CLI's stdout; `silent` discards it. Wrapper exit = CLI exit; CLI killed by signal → no state file, exit 128+sig.
O-R = `ret:` line (NO-RETURN if no state file). O-P = (exit code, stdout). O-S = (O-R, state file) — equals O-R here.
O-F = (O-S, O-P). `harness/replay.py` = shared obsmx replay with the two CELL PATCH lines above.

## What did not work / fixed
1. First replay: `hma 1 4` (C-UB) — ASan printed the overflow but `abort_on_error=1` hung; the wrapper's 20 s timeout
   raised uncaught, so no state file and no sanitizer text reached the gate → the record was mis-filed as a
   "semantic-difference" (49/79 in the first table). Fix: `halt_on_error=1:exitcode=99`, wrapper catches the timeout
   (exit 137 + `[TIMEOUT]`), gate keys on rc 99 / `[TIMEOUT]` too. Rerun → C-UB 1, table above (48/78).
2. Generic replay's `rc!=0 ⇒ UB` / `rc!=0 ⇒ Rust-failure` rules are wrong at a CLI boundary; patched (see gate).
3. Nothing under `tools/` modified; the WIP crate was copied to the scratchpad (its `sample_bin.rs` already had the
   `__assert_rtn` Linux shim). Build: `cargo +nightly-2025-09-01 build --release --bin sample` — clean.
Whole pipeline: ~2 min build + 71 s replay.

## Files
`result.json`, `raw/replay_summary.json`, `raw/replay_records.jsonl` (per-input C/Rust exit + state + stdout head, per-cell
flags), `raw/replay_stdout.log`, `corpus_seed42/`, `harness/{wrap.py,wrap_c,wrap_rs,gen_corpus.py,replay.py,indicators.txt,BUILD.txt}`.
