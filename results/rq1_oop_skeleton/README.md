# Out-of-process differential harness — minimal skeleton (validated on qsort)

A second harness design that removes the in-process FFI / `extern "C"` / C-ABI-signature
constraint, so we can differentially test **idiomatic / reshaped-signature** Rust translations
(SACTOR, C2SaferRust) without per-function hand-bridges.

## Design

- **Rust side (in the fuzz process):** the translation is a normal Rust crate dependency; the
  boundary fn is called **natively** — `translated::quickSort(&mut v, 0, n-1)`. Any idiomatic
  signature works (`&mut [i32]`, `Option<_>`, methods, …); no FFI, no `extern "C"`.
- **C side (subprocess oracle):** `oracle_main.c` `#include`s the real C verbatim, reads bytes
  from stdin, decodes them with the SAME format as the Rust harness, calls the C fn, prints the
  result. Compiled with `-fsanitize=undefined,address -fno-sanitize-recover=all`.
- **UB gate == attribution, one mechanism:** the C oracle's exit code is the gate. Exit 0 = C ran
  clean on this input (usable oracle); nonzero = C hit UB/crashed → discard the input. So when the
  Rust side crashes or mismatches while the C oracle exited 0, the divergence is unambiguously
  **Rust-side** — no separate C-alone replay needed (unlike the in-process classifier, which sees a
  Rust ASan crash as MEMORY_UB and must re-attribute).
- **Shared byte format:** both sides decode identically (here: byte0 = n mod 33, then n LE i32,
  zero-filled). This is the only "bridge", and each side builds its own native args from it.

## Result (qsort, C2SaferRust translation)

Reproduces RQ1 bug #1 through the new path. Trigger `trigger.bin` = `[2, 0xf0, 0x6b, 0x00]` →
array `[27632, 0]`; C oracle prints `2 0 27632` (correct) and **exits 0** (gate passes); the Rust
`quickSort` ASan-crashes with heap-buffer-overflow READ (the `*arr.offset(-1)` from the usize
index wraparound). Found by libFuzzer in a 60 s campaign, seed 7.

## Tradeoffs vs in-process (see also results/rq1_bugs/qsort_c2saferrust)

| | in-process (eval_rq2_ubgate) | out-of-process (this) |
|---|---|---|
| Rust call | native (crate) | native (crate) |
| C call | FFI `extern "C"` block | subprocess, no FFI |
| Rust signature | must be C-ABI-shaped for the auto-gen | **any idiomatic shape** |
| speed | µs/exec | ms/exec (subprocess spawn) |
| bridge | C-ABI arg construction | shared input byte format + output serialization |
| attribution of Rust mem-crash | needs C-alone replay | built in (oracle exit code) |

## Files

- `oracle_main.c` — the C subprocess oracle template (`#include "qsort.c"` + stdin decode + print)
- `oop_ft.rs` — the cargo-fuzz target (native Rust call + subprocess oracle + gate + compare)
- `trigger.bin` — the minimized crashing input

## To generalize (next)

The two per-boundary pieces are templatable: (1) the shared decode (from the C signature's arg
types — reuse gen_diff_harness's type→take_* logic), (2) the output serializer (return value +
mutated out-buffers). A generator that emits `oracle_main.c` + the fuzz target from a pair would
make this the default path for idiomatic tools, with in-process kept for C-ABI (c2rust) output.
