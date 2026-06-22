# Demo Smoke Test — Pipeline Recovery (2026-06-22)

First end-to-end verification after the SSD loss. Goal: prove the surviving differential-fuzz
pipeline still builds and runs.

## Toolchain (confirmed working)

- `rustup` toolchain: **nightly-2025-09-01** → rustc **1.91.0-nightly** (matches README pin)
- components: `rust-src`, **`llvm-tools`** (required by `libafl_libfuzzer` build.rs for `llvm-nm`)
- `cargo-fuzz` 0.13.2
- `clang` 21.1.8 (C oracle, sanitizer coverage)
- fuzz dep: `libfuzzer-sys = { version = "0.15.4", package = "libafl_libfuzzer" }`

> Gotcha: the LibAFL libfuzzer shim **ignores `-max_total_time`**. Bound runs with an external
> `timeout -s KILL <sec>` wrapper (or `-runs=N`), not the libFuzzer flag.

## Results

| Demo | Build | Run | Outcome |
|------|-------|-----|---------|
| `fuzz/qsort_example` | ✅ | ✅ ~12 min, 36 corpus | **0 crashes / 0 divergence** — clean aligned boundary (control case) |
| `fuzz/urlparser_example` | ✅ | ✅ | 🔴 **SEGV on execution #1** (15 random bytes) |

### urlparser crash attribution

- ASAN: `SEGV on unknown address 0x000000000000`, fault inside libc string routine
  (`libc.so.6+0x19a95d`) → **NULL-pointer dereference**.
- Crash input: `[121,218,56,101,152,192,123,69,203,143,146,13,83,241,43]` (not a valid URL).
- This is a **hard SIGSEGV (exit 139), NOT the controlled `panic!("...divergence")`**.

### Interpretation (important)

The urlparser crash is **not** a confirmed C↔Rust translation divergence. Feeding raw bytes to
`url_parse` runs outside any valid input domain; the C parser is not hardened against malformed
input and the c2rust-translated unsafe Rust faithfully reproduces that. C and Rust most likely
crash the **same** way → this is a **false-divergence-class crash / shared UB**, not a bug in the
translation.

This is a live illustration of why STU / frontier selection is the core contribution (cf.
`docs/stu_selection.md` §6–§7): the urlparser harness picked a **bad boundary + unconstrained
input domain**. STU selection exists precisely to exclude such boundaries (input state not
synchronizable, C triggers UB) and fuzz only on a trustworthy frontier. qsort is a good frontier;
this urlparser boundary is not.

## Verdict

Infrastructure **100% recovered**: toolchain → build → fuzz → ASAN → crash capture all work
end to end. qsort = working control. urlparser = bonus motivating example for STU.
