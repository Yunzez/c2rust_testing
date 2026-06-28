# Motivating example: a real semantic bug in a published LLM C→Rust translation

**Found:** 2026-06-28, by an ad-hoc hand-written differential harness.
**Subject:** C2SaferRust (arXiv:2501.14257), its shipped translation of the
[Tulip Indicators](https://tulipindicators.org/) library (v0.9.2), function `ti_adx_start`.
**Status:** confirmed, deterministic, root-caused, three-way attributed.

---

## TL;DR

C2SaferRust's LLM rewrite of `ti_adx_start` **dropped a pointer dereference**, so the
function casts the *address* of the options array to a 32-bit int instead of reading the
*period value*. The public API function therefore returns a constant garbage value
(`-1765662978`, the truncated stack address) for **every** input. The original C and the
mechanical c2rust translation are both correct; only the LLM rewrite is wrong.

This bug **passed C2SaferRust's own validation** (which checks that translations compile and
pass a fixed test suite) — exactly the gap a differential fuzzer closes.

---

## The bug, three ways

`ti_adx_start(options)` returns how many leading output samples an ADX computation skips —
in C it is `(period - 1) * 2`, where `period = (int)options[0]`.

```c
/* ORIGINAL C  (tulipindicators/indicators/adx.c:28) — correct */
int ti_adx_start(TI_REAL const *options) {
    return ((int)options[0] - 1) * 2;          /* reads options[0] (the period) */
}
```
```rust
// c2rust baseline  — correct (faithful: keeps the dereference `*`)
pub unsafe extern "C" fn ti_adx_start(options: *const c_double) -> c_int {
    return (*options.offset(0) as c_int - 1) * 2;
}
```
```rust
// C2SaferRust (LLM rewrite)  — WRONG: the dereference `*` is gone
pub extern "C" fn ti_adx_start(options: *const f64) -> i32 {
    unsafe { (options.offset(0) as i32 - 1) * 2 }   // casts the POINTER, not *options
}
```

`options.offset(0)` is a `*const f64` (a pointer); `options.offset(0) as i32` truncates that
**address** to 32 bits. The intended value is `*options.offset(0)` (the period). One missing
`*` turns a numeric computation into reading a pointer as an integer.

## Evidence

Calling both compiled functions on the same inputs:

```
            C ti_adx_start    C2SaferRust ti_adx_start
period =  2        2               -1765662978   <<< DIVERGE
period =  8       14               -1765662978   <<< DIVERGE
period = 14       26               -1765662978   <<< DIVERGE
period = 20       38               -1765662978   <<< DIVERGE
```

The C value tracks `(period-1)*2`; the C2SaferRust value is a constant independent of the
period (the options buffer sits at the same stack address each call).

How the fuzzer first surfaced it: `ti_adx` contains the internal invariant
`assert(output - outputs[0] == size - ti_adx_start(options))`. With the garbage `ti_adx_start`,
this assert fires on **every** ADX input (3000/3000 iterations at seed 1, and again at seed 777
— deterministic, seed-independent), while the original C returns `TI_OKAY`.

## Three-way attribution (why it is the LLM rewrite's fault, not drift or c2rust)

We hold three aligned artifacts shipped together: the original **C**, the **c2rust** output,
and the **C2SaferRust** output. Differential fuzzing C-vs-c2rust (relative tolerance 1e-9) shows
**zero** divergences and zero traps across all 104 indicators — so (a) the upstream C version is
aligned with the benchmark, and (b) c2rust is faithful here. Only C-vs-C2SaferRust diverges.
The defect is therefore introduced by the LLM rewrite step.

(Aside: under *strict bitwise* comparison, C-vs-c2rust shows ~1-ULP differences on `adx/adxr/dx`
from floating-point op-order/fma — benign, not bugs. This calibrates the float tolerance.)

## Impact

`ti_adx_start` is part of the **public API**: callers use it to size the output buffer
(`outputs[0]` must hold `size - ti_adx_start(options)` samples). A garbage return value makes a
caller mis-size the buffer — an out-of-bounds hazard — and it breaks the function's own
documented contract. The translation is "safe" Rust in the borrow-check sense yet semantically
corrupt.

## Reproduce

```sh
cd tools/tulip_difffuzz && bash build.sh
./fuzz_safer 1 3000 1e-9     # C vs C2SaferRust: adx asserts every iteration
./fuzz_c2r   1 3000 1e-9     # C vs c2rust: clean (attribution control)
./verify                     # prints the ti_adx_start divergence table above
```

---

## Why this motivates the rest of the work

We found this with a **hand-written, one-off harness** — and that was only possible because
Tulip Indicators is an unusually friendly target:

1. **A uniform ABI.** All 104 indicators share one signature, so a single hand-written harness
   drives all of them. Real projects do not have this.
2. **Name preservation.** c2rust and C2SaferRust keep the C function names, so pairing C↔Rust
   is trivial (`ti_adx` ↔ `ti_adx`). We could hard-code the correspondence.

Neither property holds for the translations that matter most going forward. When an **LLM
translates C to idiomatic Rust** it *renames, merges, splits, and re-shapes* functions
(a C-ABI `bool f(const uint8_t*, size_t, uint8_t*, size_t*)` becomes
`fn f(&[u8], &mut [u8]) -> Option<usize>`; see the FastHamming case). For such translations you
**cannot** hand-write the harness, because you first have to answer: *which Rust function
corresponds to which C function, and how do its arguments line up?*

This is exactly what our two contributions provide, turning this lucky manual find into a
**systematic** capability:

- **Name-independent C↔Rust matching** recovers the function correspondence from *structure*
  (I/O shape, operator profile, call-graph topology) when names and signatures no longer line
  up — making it clear *what corresponds to what* and *how to wire the differential harness*.
- **UB-correct differential fuzzing with frontier selection** then decides *what is safe to
  fuzz*, *what is risky and must be guarded*, and counts a divergence as a real translation bug
  only on inputs that are well-defined in C.

In short: a hand-written harness already finds real, shipped-LLM-translation bugs that the
translator's own validation missed. The contribution of this paper is the machinery that makes
that possible **automatically and at scale**, including on the renamed, restructured output that
LLM translators actually produce.
