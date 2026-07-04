# CROWN as a third translation lane: 4-way differential attribution

**Date:** 2026-06-28. **Status:** complete, deterministic, won't change (CROWN is rule-based).

CROWN (arXiv:2303.10515) is a *rule-based* safety-lifter: it runs ownership/mutability/fatness
analysis over c2rust output and rewrites raw pointers to safe Rust where the analysis proves it
sound, leaving the rest as `unsafe`. Unlike C2SaferRust it uses **no LLM**. It is deterministic and
name-preserving (`#[no_mangle]`), so it plugs straight into the uniform-ABI Tulip harness.

## The 4-way table (Tulip Indicators, 104 indicators, seed 1, 3000 iters/indicator)

| translation | method | compared | rust_traps | value_div | buggy indicators |
|---|---|---:|---:|---:|---:|
| c2rust        | mechanical transpile      | 297,516 | 0 | 0 | **0** ✓ |
| CROWN         | rule-based safety-lift     | 297,516 | 0 | 0 | **0** ✓ |
| C2SaferRust   | **LLM** safety-lift        | 294,516 | 3000 | 0 | **1** ✗ (`adx`) |

Relative tolerance 1e-9. CROWN clean at seed 1 and seed 777 (deterministic). Under *strict bitwise*
comparison CROWN is **identical to c2rust**: the same 3 indicators (`adx`/`adxr`/`dx`), the same 4205
~1-ULP differences from floating-point op-order — i.e. CROWN introduced **no** numeric change.

## The decisive single-function contrast (`ti_adx_start`)

The same c2rust input function, lifted by two different safety-lifters:

```rust
// CROWN (rule-based)  — CORRECT: keeps the dereference *
pub unsafe extern "C" fn ti_adx_start(mut options: *const c_double) -> c_int {
    return (*options.offset(0) as c_int - 1) * 2;
}
```
```rust
// C2SaferRust (LLM)   — WRONG: dropped the *, casts the pointer address
pub extern "C" fn ti_adx_start(options: *const f64) -> i32 {
    unsafe { (options.offset(0) as i32 - 1) * 2 }
}
```

CROWN, being conservative, left `ti_adx_start` as raw `unsafe` and preserved the deref. C2SaferRust's
LLM rewrite of the *same* function dropped `*` and returns constant garbage for every input.

## Why this matters for the paper

This **isolates the defect to the LLM rewrite step**, not to safety-lifting in general:

- Mechanical transpile (c2rust): correct.
- Rule-based safety-lift (CROWN): correct — semantics preserved by construction.
- **LLM** safety-lift (C2SaferRust): **broken** on `ti_adx_start`.

Two independent, published safety-lifters over the *identical* c2rust input; only the LLM-driven one
introduces a semantic bug that its own validation missed. CROWN serves as the attribution **control**
that rules out "any safe-lift is risky" and pins the risk to LLM-driven rewriting.

## CROWN corpus built (deterministic, reusable)

All shipped `*_crown` projects (from C2SaferRust's `laertes_benchmarks`) build to staticlibs on
nightly-2022-08-08: **tulipindicators, genann, lil, urlparser, bzip2**. tulip / genann / lil /
urlparser additionally have in-repo C → diff-able. The tulip lane is wired
(`tools/tulip_difffuzz/fuzz_crown`); genann/lil/urlparser need bespoke harnesses (no uniform ABI).

## Scope note — why CROWN can't run on our 87-repo c2rust corpus (probed 2026-06-28)

Two gates, the second is a hard wall:

1. **Form**: CROWN consumes *project-mode* c2rust output (a Cargo crate with a `c2rust-lib.rs` entry).
   Our sweep was transpiled *file-by-file* (loose `.rs`). Fixable: re-transpile with
   `c2rust transpile compile_commands.json -e` (verified: produces the right Cargo+entry, compiles
   under `cargo +nightly-2023-01-26`).

2. **Version (hard wall)**: CROWN is pinned to **~2022-era c2rust** output — `libc::c_void`, edition
   2018, `#![feature(register_tool(c2rust)/core_intrinsics/strict_provenance)]`. Our **c2rust 0.22.1**
   emits `::core::ffi::c_void`, edition 2021, no feature gates. CROWN runs as a *rustc driver* (not
   cargo), so it rejects the newer form: `E0433 maybe missing crate core` + `requires 'sized' lang_item`.
   Even after scripting the output back to the old form (paths→`libc::`, edition 2018, re-add feature
   gates), CROWN's rewriter **panics** on 0.22.1's MIR: `not yet implemented: terminator kind
   unreachable @ crates/refactor/src/rewrite_fn.rs:606`. This is inside CROWN; unfixable from our side.

**Verdict**: CROWN coverage over our current corpus ≈ 0, independent of repo. The only path to "CROWN
on the 87" is installing CROWN-era c2rust and re-sweeping all 87 (a separate sub-project, own failure
rate). The valuable, stable CROWN artifacts already exist as the laertes `*_crown` set (5 programs),
captured here — that is the realized CROWN scope.

## Reproduce

```sh
cd tools/tulip_difffuzz && bash build.sh   # builds fuzz_c2r, fuzz_safer, fuzz_crown
./fuzz_crown 1 3000 1e-9    # clean
./fuzz_c2r   1 3000 1e-9    # clean (control)
./fuzz_safer 1 3000 1e-9    # adx asserts every iteration (the bug)
```
