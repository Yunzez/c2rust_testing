# RQ1 — CROWN safety-lifter as a bug-hunt target: reconnaissance + rgba PoC (2026-07-02)

Goal: hunt real semantic bugs in NON-faithful (safety-lifted) translations, using the OOP harness
now proven 0-FP on faithful c2rust. First free lifter: CROWN. This note records what CROWN's output
actually looks like and a validated end-to-end PoC on `rgba`.

## What CROWN actually rewrites (the value-scope intersection is real but narrow)

CROWN's lifted outputs live in `tools/frameworks/crown/results/` (~20 programs). Scanning them:

- **Rewrites pointer objects**: `*mut Struct` → `Option<&mut Struct>`, owning-pointer annotations
  (pointer-graph — out of the value-differential scope; that is where CROWN's own bugs would live).
- **Rewrites single out-scalars**: `*mut int` → `Option<&mut c_int>` (46×), `Option<&mut size_t>`
  (55×), `Option<&mut uint32_t>` (16×) — **value-oriented, in scope**.
- **Does NOT rewrite pure-arithmetic functions** — they stay raw-ptr / unchanged (faithful → TN).
- **Produces ZERO `&[T]` buffer→slice lifts.**

So CROWN's semantic-risk is concentrated in pointer-graph code (out of scope). The value-oriented
intersection is the OUT-SCALAR lifts: **33 CROWN-lifted functions are value-oriented AND have a
`Option<&mut scalar>` out-param** — concentrated in `rgba` (5, pure string→color), `brotli` (19,
mixed), `lodepng` (5), plus json.h/binn/bzip2. These are the testable CROWN targets.

## rgba PoC — pipeline validated end-to-end, function clean

`rgba_from_string(const char *str, short *ok) -> uint32_t` (C) ↔ CROWN-lifted
`rgba_from_string(str: *const c_char, ok: Option<&mut c_short>) -> u32`. Color-string parser
(`#rrggbb`, `rgb(r,g,b)`, named colors) — fully value-oriented.

- **Version mismatch is NOT a blocker**: the lifted `rgba` crate (targets nightly-2023-01-26) also
  compiles cleanly under the fuzz toolchain nightly-2025-09-01 (only stable-feature warnings).
- **UB gate works on real UB**: `#ffffff` etc. trigger UBSan in the ORIGINAL C (`255 << 24`,
  signed-shift overflow, rgba.c:274) → gated out, correctly NOT counted as divergences.
- **Idiomatic `Option<&mut>` path works**: Rust called natively with `Some(&mut ok)`; C is the
  subprocess oracle. Determinism gate + stdout isolation carried over from the census fixes.
- **Result: 15,864 execs, DONE, 0 divergences** → CROWN's rgba lift is faithful on UB-free inputs.

PoC harness: `scratchpad/rgba_poc/` (oracle/rgba_oracle.c + fuzz/fuzz_targets/rgba_ft.rs). Reusable
template for the other 32 CROWN out-scalar targets. NOTE it depends on the lifted crate under
`tools/frameworks/crown/results/rgba` (gitignored).

## Assessment / next decision

- The CROWN pipeline is VIABLE (compiles, harnesses, gates UB) — one program (rgba) is clean.
- To SCALE to the other 32 targets, either (a) hand-bridge each (slow, ~1 harness/function) or
  (b) generalize `gen_oop_harness.py` to (i) detect `Option<&mut scalar>` out-params and emit
  `Some(&mut cell)`, and (ii) consume a multi-module CROWN crate (not a single translated .rs).
- CROWN's value-bug yield is uncertain (rgba clean; its rich rewrite surface is out of scope).
  C2SaferRust rewrites value ARITHMETIC (that is where bug #1 qsort int→usize came from) →
  higher expected value-bug yield, but its artifacts are macOS-c2rust (need re-transpile).

Recommendation pending user steer: generalize the generator for the CROWN out-scalar set (scales to
33 targets + validates the idiomatic path in the tool, not just by hand), then run C2SaferRust for
the arithmetic-rewrite bug surface.
