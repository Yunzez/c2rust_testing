# genann × SACTOR (gpt-5.1): translate-fail — scaffold cannot express fn-pointer typedefs

**Verdict: `✗(translate-fail)`** — SACTOR's own unidiomatic stage gave up: **0/15 functions translated**,
every one exhausting all 6 attempts (60 attempt-rounds logged), then `MAX_ATTEMPTS_EXCEEDED`; the driver
TU then aborts on the untranslated `genann_init` dependency.

## Root cause (systematic, not per-function)
Every `build_attempt` fails with the **same two E0425s regardless of what the LLM writes** — the failure
is in SACTOR's verification scaffold, not the LLM output:

```rust
pub type FILE = _IO_FILE;      // _IO_FILE never defined (stdio typedef chain cut off)
pub struct genann {
    ...
    pub activation_hidden: genann_actfun,   // genann_actfun never defined
```

- `genann_actfun` is C's `typedef double (*genann_actfun)(const struct genann*, double);` — a
  **function-pointer typedef**. SACTOR's type-alias extraction handles only simple `alias = type` pairs;
  the fn-ptr typedef is dropped from the scaffold's typedef closure, so the (accepted) struct
  translation references an undefined type in every subsequent function build.
- `FILE` (from `genann_read/write(FILE*)` in the public API) resolves to `_IO_FILE`, whose opaque
  struct is likewise never emitted (SACTOR knows `libc::FILE` only for stdin/stdout/stderr globals).

Even `genann_act_linear` (`return a;`) fails 6/6 — proof the scaffold, not the translation, is broken.

## Attribution note
Environment verified healthy: macro expansion, the internal c2rust step, include paths, and the test
harness all worked (the same setup translated SACTOR's own `add` example end-to-end clean, both
stages, all tests green). Three local-copy C adaptations were needed just to get SACTOR's *parser*
past genann (`__builtin_expect` → plain expr, `isnan(a)` → `a != a`) and are output-identical — the
subsequent failure is the tool's scaffold. This mirrors the 2026-07-02 round's utf8_crust verdict
(`MAX_ATTEMPTS_EXCEEDED` = SACTOR faithful-or-fail → fail).

## Contrast (the genann row)
genann was the **all-faithful row**: c2rust ✓F(300k) / Laertes ✓F(200k) / C2SaferRust ✓F(50M) /
CROWN ✓F(300k). The two newest LLM tools break the streak: PtrTrans ▲decl-only (slicer fed only
headers), SACTOR ✗ (scaffold can't express the activation-fn-pointer design). The callback-driven
API that every 2021-era lifter handled is what defeats both 2025/26-era LLM pipelines' *plumbing* —
in both cases before semantics could even be tested.

## Files
- `batch_summary.json` — SACTOR's own failure record for both TUs
- `scaffold_lib_rs_head.rs` — the broken scaffold (FILE/_IO_FILE + genann_actfun refs)
- `systematic_E0425.txt` — the repeated compile error across attempts
- `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json` — the deterministic
  2-4-1-net harness (weight overwrite, run+train), archived here (scratchpad is reboot-volatile)
