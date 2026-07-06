# genann × SACTOR (gpt-5.1): the immutable lookup table — 100% divergence, zero panics

**Verdict: `s:1` ★ (headline #6)** — after unblocking SACTOR's pipeline (see "interventions"), its
verified translation compiles and runs — and **diverges on 5,000/5,000 = 100.00% of random inputs**:
the C network computes sigmoids; the Rust network outputs **all zeros, on every input, silently**
(release build, exit 0, no panic).

## Mechanism: mutability loss in the global lift (4th zeroed-table instance)
C: `double lookup[4096];` — a file-scope **mutable** table, populated at runtime by
`genann_init_sigmoid_lookup`. SACTOR's translation (verbatim, `verbatim_lookup_global.rs` +
`verbatim_init_writer.rs`):

```rust
static lookup: [f64; 4096] = [0.0; 4096];          // IMMUTABLE static
// ... in genann_init_sigmoid_lookup (which genann_init DOES call):
let ptr = &lookup as *const [f64; 4096] as *mut [f64; 4096];
(*ptr)[i as usize] = genann_act_sigmoid(...);       // write through const-cast = Rust UB
```

Writing an immutable static through a const-cast pointer is **undefined behavior in Rust**. The
proof is the profile split:
- **Release**: LLVM elides the UB writes / folds every `lookup[j]` read to `0.0` → **silent all-zero
  outputs** (what we measure).
- **Debug**: the write actually targets `.rodata` → **SIGSEGV** (reproduced, exit 139).

So the sigmoid table stays zero forever and `genann_run` returns 0.0 for everything. **The
zeroed-table class, 4th instance** — with a perfect intra-row contrast: on the SAME library and the
SAME table, Laertes' uncalled-init was *harmless* (the lazy rebuild repopulates the table), while
SACTOR's rebuild *runs and its writes vanish* — same symptom class, opposite mechanism, opposite
outcome.

## Why SACTOR's own verification structurally cannot see this
SACTOR verifies each translated function by **embedding it into the C program via FFI** — where
`lookup` is still C's mutable array. Every function passed. The bug only exists in the all-Rust
assembly, which SACTOR's combiner never successfully produced. **Test-driven per-function
verification ≠ whole-program correctness.** And a Rust-only fuzzer sees: release = no crash, wrong
values (invisible without an oracle); debug = a crash it can't map to "outputs are wrong in release".
Only a C-vs-Rust differential exposes it as what it is.

## Interventions (full disclosure — none touch the buggy content)
To get SACTOR's translation to a runnable artifact we fixed/bypassed 4 of its pipeline bugs:
1. **Scaffold typedef-closure patch** (`unidiomatic_translator.py: _close_type_closure`): its struct
   scaffold emitted `pub type FILE = _IO_FILE;` (undefined) and dropped fn-ptr typedefs
   (`genann_actfun`) → EVERY build_attempt failed regardless of LLM output (0/15). With the patch:
   **15/15 functions translate and pass verification**.
2. **TU renamed** genann.c → genann_lib.c (its combiner collides TU-module name with struct name).
3. **Flat assembly**: its project combiner puts function bodies that say `crate::lookup` into a
   module (breaking every such path), so we flat-merged its own per-TU `combined.rs` outputs —
   SACTOR's translation content verbatim — into one crate.
4. Two mechanical assembly repairs its combiner should have done: `extern "C"` ABI on the activation
   fns (its own `genann_actfun` typedef demands it), and defining `interval` (its fallback emitted an
   `extern` declaration for C's tentative definition → link error).
The immutable `static lookup` and the const-cast writer — the bug — are **SACTOR's verbatim
translation files**, untouched.

## Numbers
- 5,000/5,000 random `(input1, input2, train_iters)` triples diverge (C-side clean, deterministic
  driver: fixed 2-4-1 net, weights overwritten with a fixed sequence, run + train + run).
- 12/12 of the original embedded test cases diverge identically.
- Release: 100% silent wrong (all-zero). Debug: 100% SIGSEGV.

## Files
- `assembled_translation.rs` — the runnable artifact (SACTOR content + listed assembly repairs)
- `verbatim_lookup_global.rs`, `verbatim_init_writer.rs`, `verbatim_interval_extern.rs` — SACTOR's
  unmodified output files showing the bug
- `scaffold_lib_rs_head.rs`, `systematic_E0425.txt`, `batch_summary.json` — the pre-patch 0/15 record
- `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json` — harness
