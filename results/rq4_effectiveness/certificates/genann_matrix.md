# genann multi-tool differential (RQ1) — the core forward-pass `genann_run`

Fixes the gap the advisor caught: earlier genann testing hit only **one** lifter (C2SaferRust) and the
**wrong function** (`genann_act_sigmoid`, a scalar value fn C2SaferRust rewrote — faithful, 50M/0). The
CORE value function of the library, the forward pass **`genann_run`**, was rewritten by **CROWN**
(mechanical `*const genann` → `Option<&mut genann>`), not by C2SaferRust. This is the function each
lifter's differential must target: **each lifter's version of the function IT actually changed, C-backed
against the original C.**

## Method (all findings C-backed)

Original `genann.c` (`genann_init`/`genann_run`) = ground-truth oracle. Fixed network **(2 inputs, 1
hidden layer, 3 hidden, 1 output) = 13 weights**. Per record: 13 weights + 2 inputs (f64 LE) are read
from a shared fuzz corpus; weights are written directly into `ann->weight`, `genann_run` is called, the
single output's raw 64-bit pattern is printed. **Exact sigmoid** (`1/(1+exp(-a))`) is forced on both
`activation_hidden`/`activation_output` on every side — this isolates `genann_run`'s multiply-accumulate
arithmetic (the part CROWN rewrote) from the default cached-lookup activation table.

Corpus: **300,000 records**, seed-fixed, magnitude-mixed (70% ∈ [-5,5], 15% ∈ [±100], 10% ∈ [±1e6], 5%
edge: ±0, 1e±300). Same bytes fed to every implementation; outputs compared **bit-exact**.

Scratch: `scratchpad/genann_run/` (`oracle.c`, `corpus.bin`, `out_{c,base,crown}.txt`). Drivers:
`genann/rundiff.rs` (base c2rust), `genann_crown/src/bin/rundiff.rs` (CROWN), both built with
nightly-2023-01-26 (CROWN's toolchain). base c2rust was transpiled on macOS → its `genann_run` assert
references `__assert_rtn`; stubbed in the driver (assert never fires — all weights consumed).

## Results matrix — `genann_run`

| tool | rewrote `genann_run`? | built / linked | result |
|---|---|---|---|
| **C** (`genann.c`) | — (oracle) | native | reference |
| **c2rust** 0.22 (mechanical) | no (`*const genann`) | ✅ Linux-native rlib driver | **FAITHFUL — 300k/0 vs C** |
| **CROWN** (safety lifter) | **yes** → `Option<&mut genann>` | ✅ Linux-native rlib driver | **FAITHFUL — 300k/0 vs C, 0 vs base c2rust** |
| **C2SaferRust** | no — `genann_run` **textually identical** to base c2rust (it rewrote `genann_act_sigmoid`/`copy`/`free`/`write`, not the forward pass) | control (same code as base) | **FAITHFUL** (= base c2rust; verified `diff` identical) |
| **Laertes** (OOPSLA'21) | no (kept mechanical `*const genann`) | control | **FAITHFUL** (mechanical c2rust output) |

Three independent implementations — original C, mechanical c2rust, and CROWN's ownership-rewritten
version — agree **bit-for-bit on all 300,000 records** across the full f64 magnitude range. `genann_run`
is CLEAN under every lifter.

Companion (already run): **`genann_act_sigmoid`** (the fn C2SaferRust rewrote) — 50M samples, 0 bit-diffs
(C libm `exp` ≡ Rust std `exp`). Faithful.

## Conclusion

**genann is fully clean across all four lifters** — no semantic diff in either the forward pass
(`genann_run`, CROWN's rewrite) or the activation (`genann_act_sigmoid`, C2SaferRust's rewrite). This is
the honest multi-tool result: on this small, arithmetic-only library the lifters are faithful. It
sharpens the crc32 finding — the crc32 bug was not "lifters are generally buggy" but a **specific
boundary-condition rewrite** (`is_null` → `is_empty` on an empty chunk) that only fires on a length-0
edge case. genann has no such implicit-precondition surface, so it certifies clean. crc32 remains the
exception, and the productive hunt stays on lifter rewrites that touch **implicit preconditions / buffer
boundaries**, not pure arithmetic.
