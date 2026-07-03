# RQ1 auto-sweep — corpus-scale base-vs-C2SaferRust differential (#1 + #2)

Tooling: `tools/stu_selector/rust_diff.py` (per-function generator) + `crate_sweep.py` (links a base
c2rust crate + a C2SaferRust WIP crate as rlibs and differential-tests every value-shaped function
present in both, each in its own subprocess). Compares return VALUES over randomized inputs → catches
silent diffs; no crash needed. Reuses the aligned-signature logic (ptr↔slice reshape, scalar passthrough,
buffer-length binding, type-alias resolution). Value-comparability filters: struct-pointer buffers,
pointer/reference returns, and `main` are excluded.

## What ran (Linux linkability is the gate)

| crate | links on Linux? | value fns swept | result |
|---|---|---|---|
| **optipng** | ✅ yes | 24 (of 374; 350 pointer-graph) | **crc32_z + crc32 DIVERGE (the known bug, auto-refound)**; 13 clean; 6 segv (raw-ptr, isolated) |
| **urlparser** | ✅ yes | 2 (of 17) | both CRASH (the C-UB getters — matches manual finding); no silent diff |
| bzip2 | ❌ macOS-c2rust | 3 | link error: `__maskrune`, `_DefaultRuneLocale` |
| genann | ❌ macOS-c2rust | 10 | link error: `__assert_rtn`, `__error` |
| lil | ❌ macOS-c2rust | 3 | link error: `_DefaultRuneLocale`, `__maskrune` |
| snudown | ❌ (pkg-name + macOS) | 1 | — |
| tulipindicators | — | (not rewritten; base==WIP) | skipped |

## Findings

1. **The sweep auto-refinds the crc32 silent bug** with zero manual harnessing — validating the
   method end-to-end at crate scale.
2. **No NEW silent diff** in the Linux-linkable surface (optipng value fns + urlparser). The
   silent-diff surface in C2SaferRust remains concentrated in the zlib checksum family (crc32; adler32
   found separately by module extraction).
3. **Coverage is gated by the macOS-c2rust artifacts.** Only optipng and urlparser link on Linux; the
   rest reference macOS libc internals (`__maskrune`, `__assert_rtn`, `_DefaultRuneLocale`) and would
   need Linux re-transpilation of the base + a re-run of C2SaferRust for the WIP (heavy; the base recipe
   is in [[rq1-realbug-hunt-state]], the WIP needs nightly-2022-08-08).
4. **False-positive classes identified (need triage/tightening):**
   - *pointer/reference returns* — e.g. `gzdopen` returns a `gzFile` handle; base/WIP addresses differ
     (not a value). Filter missed the alias; add handle-type aliases to the return guard.
   - *out-of-domain scalars* — `crc32_combine(crc1, crc2, len2)` diverged only because the fuzzer passed
     full 64-bit random values as 32-bit CRCs; on in-domain (32-bit) inputs the functions agree. Scalar
     inputs should respect the function's real domain (hard to infer automatically → manual triage, or a
     mask heuristic for CRC/checksum params).

## Honest read

The auto-sweep works and scales, but the C2SaferRust laertes corpus is mostly macOS-locked on Linux, so
the reachable value surface is small (optipng's ~24 + urlparser's 2). It re-confirms the crc32 headline
automatically and surfaces no new silent diff there. Growing the sweep needs Linux-re-transpiled crates
(or a different corpus — CRUST-Bench, coreutils). The FP classes (pointer returns, out-of-domain scalars)
are the precision work before a clean "silent-diff rate" number can be reported.
