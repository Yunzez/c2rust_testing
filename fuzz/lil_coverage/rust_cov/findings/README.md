# Fuzzer findings — raw-LLM lil (libFuzzer, 2026-07-07)

## LEAK (LeakSanitizer), first input
`leak_env_cycle.input` — a valid script using `topeval`/`eval`/nested blocks leaks memory.
Root cause class: the translation models the environment chain as `Rc<RefCell<LilEnv>>`
with `parent` links; scoped evals (topeval/eval/jaileval push a child env pointing at its
parent) create **reference cycles that Rc never frees**. Detected instantly by ASan/LSan.
NOTE: this is incidental (found while setting up the coverage run); kept because it is a
real memory-safety divergence class in the LLM output. Coverage run below uses
`-detect_leaks=0` so the loop can proceed.

## Real coverage-guided fuzz run (libFuzzer, fork mode, 300s, 2026-07-07)
The raw-LLM lil crate crashes constantly; plain in-process libFuzzer dies almost immediately,
which is WHY fork mode + catch_unwind + a silent panic hook were needed. Genuine
translation-divergence findings (recorded, C-vs-Rust behavior differs):
- **integer-overflow panic** — number parsing `self.ival * 10` overflows i64 → Rust aborts;
  C wraps (lil.c uses plain int arithmetic). site: lib.rs:2303.
- **index-out-of-bounds panic** — `codeat`/`charat`-style handlers index past the string.
- **Rc-cycle memory LEAK** — env chain `Rc<RefCell<LilEnv>>` with parent links; scoped evals
  (topeval/eval/jaileval) form reference cycles Rc never frees → repeated OOM. C frees.
NOT bugs: the 8 `timeout-*` artifacts are the `while {expr $x>0}` dialect quirk — an infinite
loop in BOTH C and Rust (verified C also hangs, exit 124), i.e. our grammar's own inputs,
not a translation defect.

## Coverage (the deliverable)
Measured per-process over the fuzzer corpus (avoids the leak OOM-ing a batch replay):
**paired functions 115/117 = 98.3% · whole-crate functions 96.1% · lines 74.8% (1601/2140).**
Claim: per-function MATCHING is ~50%, but fuzzing the matched functions covers 96% of the
translated crate — matching recall does not cap differential coverage.
