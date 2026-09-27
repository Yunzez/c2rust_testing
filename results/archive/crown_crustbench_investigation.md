# CROWN on the neutral CRUST-bench: investigation & verdict (2026-06-28)

## Why we tried this

Methodology decision (user): evaluate every tool on a **third-party, neutral** benchmark (CRUST-bench,
100 C repos) rather than each tool's **own** curated benchmark — to avoid the "authors picked favorable
programs" (cherry-pick) criticism. So we wanted CROWN (rule-based safety-lifter, arXiv:2303.10515) to run
on CRUST-bench, not just on its shipped 20-program benchmark.

## The wall: CROWN is version-coupled to its exact c2rust

CROWN runs as a **rustc driver** and its rewriter is tightly coupled to the exact c2rust version/flags it
was built against (~2022-era, nightly-2022-08-08, `libc::`-form, edition 2018, gates
`register_tool(c2rust)/core_intrinsics/strict_provenance/raw_ref_op`). Our corpus c2rust is 0.22.1
(`::core::ffi::`, edition 2021) → CROWN's driver rejects it outright (`E0433 missing crate core`; on a
forced downgrade its rewriter panics `not yet implemented: terminator kind unreachable rewrite_fn.rs:606`).

## What we built

- **Old c2rust in docker** (host is Ubuntu 26.04 "resolute" → no apt llvm-15): `crown/oldc2rust/`
  Dockerfile (ubuntu:22.04 + llvm-15 + rustup nightly-2022-08-08 + c2rust from git tag). Images
  `oldc2rust:0.18` and `oldc2rust:0.17`.
- **Sweep pipeline** `crown/oldc2rust/pilot.sh`: per repo → docker old-c2rust project-mode transpile with
  **CROWN's exact flags** `--emit-modules --fail-on-error --reduce-type-annotations --emit-build-files` →
  minimal fixup (edition 2018 + feature gates + `extern crate core/libc`) → CROWN preprocess/explicit-addr/
  rewrite → `cargo +nightly-2023-01-26 check`.

## Results (87 usable repos)

| stage | count |
|---|--:|
| old-c2rust transpile | 74–83 |
| CROWN ran (analysis produced) | 60–68 |
| **CROWN-lift compiles cleanly (usable)** | **12** |
| +residual 1–5 errors (so close) | 30 |
| residual 6–20 / >20 errors | 11 / 2 |

The 12 usable: `2DPartInt, amp, btree-map, c-aces, cJSON, geofence, libwecan, ljmm, Math-Library-in-C,
morton, NandC, vec`.

## Why it caps at 12

- Using CROWN's exact **flags** (esp. `--reduce-type-annotations`, which we'd initially omitted) was the
  big win: buffer went 26 compile-errors → 3, and CROWN lifted 98→75 raw pointers (vs CROWN's own
  benchmark 98→76).
- But a **residual 1-pointer over-lift** persists on most programs: our reconstructed c2rust (0.17 *and*
  0.18, identical behavior here) lifts one pointer more than CROWN's exact (unshipped) commit, leaving a
  few use sites un-rewritten → `E0308/E0614`. `--raw-mutability` did not fix it.
- **CROWN is not push-button.** Its own `run.sh` tunes flags per program (`--raw-mutability` for genann,
  `--force-box` for quadtree, `--no-attempt` for lil). "100 in → clean out" is structurally impossible
  without per-program tuning. The 30 near-misses would each need manual tuning / the exact c2rust commit.

## Verdict

Stop chasing. Realized CROWN corpus = **12 neutral (CRUST-bench)** + **19 established (`crown/results/`,
its own benchmark, all lifted+compiling)**. CROWN is a **~0-bug attribution control** anyway (rule-based,
sound) — its role ("rule-based safe-lift introduces no semantic bug, unlike the LLM lift") is already
proven on tulip, and 12+19 programs amply support it. Not worth more version archaeology for a 0-bug tool.
Docker images kept in `crown/oldc2rust/` for any future use.

## Where CRUST-bench runs going forward

CRUST-bench (87 repos with c2rust, our 0.22.1 sweep) is the **neutral common dataset**. Lanes on it:
1. **C-vs-c2rust** (have, free, 0.22.1) — tests c2rust's own fidelity. Run now via per-program harnesses.
2. **C-vs-CROWN** — only the 12 clean overlap (above).
3. **C-vs-LLM** (SACTOR / C2SaferRust run by us on CRUST-bench) — costs tokens, **the bug-rich lane**,
   non-circular because the dataset is third-party. This is the headline track.
4. **Possibly a more-SOTA static tool** if one exists that is runnable on CRUST-bench (under research).

The shared bottleneck for all lanes is **per-program differential harnesses** (all lanes preserve names →
auto-pair by symbol, auto-gen harness from C signatures).
