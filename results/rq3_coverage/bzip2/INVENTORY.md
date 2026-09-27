# RQ4 (coverage) — bzip2 inventory, stage A

*Directory name `rq3_coverage/` is legacy; this is current **RQ4 — coverage beyond shipped tests**
(see `results/INDEX.md` rename map).*

Produced 2026-09-03, before any build. Nothing has been executed for this experiment yet.
Sources: `results/rq2_attribution/harness_manifest.json` (Phase-0 harness audit, 12 bzip2 records),
`results/rq4_effectiveness/translation_matrix.md`, direct inspection of the trees below.

## 0. Shared facts

- **C source**: `tools/frameworks/crown/c-code/bzip2/` — bzip2 1.0.8, `bzlib.c` sha256 `d06cf1bd991df1f2…`.
  This is CROWN's copy of the Laertes benchmark (`compile_commands.json` directory
  `/home/vagrant/orc-benchmark/bzip2-laertes`). Phase 0 recorded that **no harness ever bound a C hash
  to a translator input**; `matches_translator_input` is `unknown` for c2rust / Laertes / C2SaferRust
  and `yes` (by directory provenance, not hash) for CROWN / SACTOR / PtrTrans.
- **Shipped test suite**: bzip2 ships **no unit tests**. Its acceptance suite is the `test:` target of
  `tools/frameworks/crown/c-code/bzip2/Makefile` — a **CLI-level roundtrip**: compress
  `sample{1,2,3}.ref` at `-1/-2/-3`, decompress `sample{1,2,3}.bz2` (`-d`, `-d`, `-ds`), `cmp` all six
  against the shipped references. No `#[test]` exists in any translated bzip2 crate.
- **Every translated crate carries a transpiled CLI** (`bzip2.rs` with `fn main`), but none declares it
  as a `[[bin]]` (`autobins = false`). Running the shipped suite against a translation therefore needs a
  **representation-only adapter**: declare `bzip2.rs` as a bin in a *copy* of the crate and run the same
  six commands plus the same six `cmp`s. The suite's content is unchanged.
- **Toolchain**: `nightly-2025-09-01` (rustc 1.91.0-nightly, **LLVM 21.1.0**) builds all four surviving
  crates (`cargo check --lib` exit 0, Phase-0 verified). Matching coverage tools ship with it:
  `~/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/{llvm-profdata,llvm-cov}`
  → `LLVM 21.1.0-rust-1.91.0-nightly`. The system `llvm-profdata-21` (LLVM 21.1.8) must **not** be used.
- **Disk**: 3.1 GB free on `/`. All builds go to the session scratchpad with `CARGO_TARGET_DIR` set
  there, and each tool's target dir is deleted before the next tool starts.

## 1. Per-tool inventory

| tool | Rust artifact | artifact status | test support | differential harness | archived corpus | tests + corpus drive the same crate? |
|---|---|---|---|---|---|---|
| **c2rust** | `tools/frameworks/c2saferrust/laertes_benchmarks/bzip2/` (library `.rs` byte-identical to `fuzz/bzip2_c2rust_e3/src/`) | complete, builds | representation-only adapter (CLI bin) | `rundiff.rs` + `bugs/bzip2_crown/oracle_comp.c` | **differential corpus LOST** (`scratchpad/bz_crown_diff/`, 76 of 1608 records quoted, dir gone). Retained: `fuzz/bzip2_c2rust_e3/fuzz/corpus/ft/` — **1163 inputs, 4.7 MB**, but that target is **Rust-only** (`--sanitizer=none`, no C executed) | yes |
| **Laertes** | `…/laertes_benchmarks/bzip2_laertes/` (== `fuzz/bzip2_laertes_e3/src/`) | complete, builds | representation-only adapter (CLI bin) | **`results/ablations/observation/obs_matrix/bzip2_laertes/harness/{driver.c,obs_rs_driver.rs,obs_ft.rs,replay.py}`** — isolated ASan+UBSan C subprocess per record, C-first gate, Rust only on admitted inputs | **`…/bzip2_laertes/corpus_seed42/` — 529 inputs, retained.** Also `fuzz/bzip2_laertes_e3/fuzz/corpus/ft/` 476 (Rust-only) | yes |
| **C2SaferRust** | `…/laertes_benchmarks/bzip2_WIP/` (== `fuzz/bzip2_wip_e3/src/`) | complete, `cargo check` clean; reshaped API `BZ2_bzBuffToBuffCompress(&mut Vec<u8>, &[u8], …)` **SIGSEGVs on 100 %** of E3 inputs (cause unresolved: real crash vs `&mut Vec` contract mismatch) | adapter untested — reshaped signatures may break the CLI | **none.** The `s:1` half of the matrix cell is *source inspection only*; the `c:1` half is a Rust-vs-Rust body copy with one handmade input | `fuzz/bzip2_wip_e3/fuzz/corpus/ft/` = **8 urandom seeds** (corpus never grew, 0/8 survivors) | yes |
| **CROWN** | `…/laertes_benchmarks/bzip2_crown/` (== `tools/frameworks/crown/results/bzip2`, == `fuzz/bzip2_crown_e3/src/`) | complete, `cargo check` clean; **macOS-transpiled** — linking a binary needs `bugs/bzip2_crown/darwin_shims.c` (`__maskrune`, `_DefaultRuneLocale`, `__stderrp`, `__assert_rtn`) | adapter + shims | `rundiff.rs`/`decdrv.rs` + `oracle_comp.c` | **corpus LOST** (1608 records + the 150-record sample, `scratchpad/bz_crown_diff/` gone). Retained: `fuzz/bzip2_crown_e3/fuzz/corpus/ft/` = **8 urandom seeds** (corpus never grew: crashes) | yes |
| **SACTOR** | **none produced** | `✗(parse)` — resolver died at `bzlib.c:168` (`BZALLOC` member-fn-ptr call, USR = None), then at `fdopen`. `bugs/bzip2_sactor/parser_errors.txt` | n/a | n/a — the prepared `driver.c` was never exercised | none | n/a |
| **PtrTrans** | `…/ptrtrans_rebuild/PtrTrans-C2Rust/dataset/PA_trans_projects/bzip2/` | `✗(compile)` — 55/78 stub-reverts, 73 module-assembly errors (`cargo check` exit 101, reproduced in Phase 0) | n/a | n/a | none | n/a |

## 2. Outcome classification before execution

| tool | outcome | reason |
|---|---|---|
| SACTOR | **NON-BUILDING** (artifact never produced) | translation process failed at parse; nothing to instrument |
| PtrTrans | **NON-BUILDING** | assembled crate does not compile |
| CROWN | **FUZZ-CORPUS-UNAVAILABLE** | the 1608-record differential corpus is not on disk; the only retained inputs are 8 unmutated seeds |
| C2SaferRust | **FUZZ-CORPUS-UNAVAILABLE** | no differential harness ever existed for this cell; retained inputs are 8 unmutated seeds |
| c2rust | **differential corpus unavailable**; a 1163-input campaign corpus survives but was generated **without a C reference** | see §3 |
| Laertes | **candidate PAIRED** | the OBS cell is a genuine gated C-vs-Rust harness with its 529-input corpus retained |

**Consequence:** under a strict reading of "archived differential-fuzzing corpora", bzip2 has **one**
paired artifact (Laertes), so the library mean of §H would be a mean over a single cell. This is a
methodological blocker, recorded here rather than papered over. Regenerating the lost corpora is
excluded by the standing instruction not to launch a new fuzzing campaign.

## 3. The corpus definition fork (needs a decision before stage E)

- **Strict** — replay only corpora produced by a harness that executed the C reference. → Laertes only
  (529 inputs). c2rust / CROWN / C2SaferRust are FUZZ-CORPUS-UNAVAILABLE.
- **Relaxed** — also replay the archived coverage-guided campaign corpora from the E3 reach cells,
  labelled as *campaign corpora generated on the translated artifact with no C reference at generation
  time*. → c2rust 1163, Laertes 476 (∪ 529), CROWN 8, C2SaferRust 8. The CROWN and C2SaferRust numbers
  would still be seed-only floors, not campaign coverage.

Either way, the answer must be stated in `SUMMARY.md` and in any paper sentence that uses these numbers.

---

## Addendum 2026-09-03 — §3 is superseded for c2rust

The "corpus definition fork" in §3 asked which *archived* corpus to replay. That question was the
wrong one: replaying one archived corpus measures one harness, not the validator. The c2rust cell
was redone by **generating** differential harnesses for every eligible matched pair and running them
under one artifact-level budget — see [`PROTOCOL.md`](PROTOCOL.md) and
[`c2rust/RUN.md`](c2rust/RUN.md). §3 still applies to any cell for which we choose to replay history
instead of running a campaign; none currently does.

The §1/§2 inventory of artifacts, test suites and archived corpora is unchanged and still correct.
