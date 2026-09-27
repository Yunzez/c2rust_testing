# ATTR pilot — Part 1: urlparser, seed 42, commit dda70a4d

Study question (FSE_PLAN constraint C1): does the in-loop `--ub-free` UBSan gate catch urlparser's
C-side memory UB, or does only the isolated ASan+UBSan oracle catch it? C1 says the gate does NOT —
this pilot measures it rather than assuming it.

Input: `http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash` (fn-27 example).
Oracle defect: `url_parse -> get_part`, `sscanf` at `url.h:208` writes 10 bytes into `malloc(1)` — a
heap-buffer-overflow on the **first, entirely ordinary URL**.

## Table
| config | raw failure candidates | admissible comparisons | confirmed translation divergences | C-UB exclusions | outcome | TTFD |
|---|---|---|---|---|---|---|
| (a) none | 1 | 0 | 0 | 0 | **unattributed crash candidate** (the crash can occur in the C call itself) | 0.01 s |
| (b) in-loop UBSan gate | 1 | 0 | 0 | 0 | **unattributed crash candidate** — identical to (a) | 0.01 s |
| (c) isolated ASan+UBSan | 1 | 0 | 0 | 1 | C-UB excluded (heap-buffer-overflow, `get_part` url.h:208) | none |

Classification totals: C-UB 1 · C-unstable 0 · Rust-failure 0 · semantic-difference 0 · abstention 0.
No configuration produces a confirmed translation divergence; (a)/(b) leave an unattributed crash
candidate that they cannot assign to either side, (c) attributes it to the C reference.
Reached functions: n/a (differential harness, not instrumented for coverage census).

## The C1 measurement (the point of this pilot)
(a) and (b) **both crash on the seed input** — `double free or corruption (!prev)`, libFuzzer deadly
signal, rc=77, wall 0.01 s. They are indistinguishable.

Decisive probe (`raw/url_gate_probe.c`, built with the exact `--ub-free` flag set + `ubshim.c`): after
`url_parse` + the three getters on the seed URL, the gate's UB flag reads

    UB_FLAG_AFTER_PARSE=0

i.e. the UBSan minimal-runtime gate (`signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,
unreachable`) sees **zero** UB. `-fsanitize=bounds` covers compile-time-known array bounds, not a heap
`malloc` overflow through `sscanf`. So the gate does not reject the input; the process aborts on the corrupted
heap — an unattributed crash candidate, exactly as in config (a). **C1 CONFIRMED, measured: (b) really fails to catch it.**

(c) the fresh-process ASan+UBSan oracle exits with `AddressSanitizer: heap-buffer-overflow` (WRITE size
10, `get_part` @ url.h:208, `url_parse` @ url.h:247) → the input is discarded as C-UB. 0 admissible
comparisons → 0 confirmed translation divergences. This is the only configuration that attributes correctly.

Note on the "libFuzzer ≤10 min" step: it degenerates. The seed URL crashes on exec 0, so no
generative fuzzing happens under (a)/(b) — an unattributed crash candidate appears immediately (this matches fn 27: the UB
gate/oracle catches it on the first input). The library is a library-level exclusion, not a translation
defect, in every configuration that runs the C reference under memory sanitization.

## What did not work → what was changed
1. `cargo-fuzz` default dep `libfuzzer-sys 0.15.4` (package `libafl_libfuzzer`) — its `build.rs` panics
   `Option::unwrap() on None` under this cargo-fuzz → switched the harness to `libfuzzer-sys = "0.4"`.
2. `build.rs` used `-fsanitize-coverage=trace-pc-guard,trace-cmp`, which clang-21's libFuzzer refuses
   → changed to `inline-8bit-counters,pc-table,trace-cmp`.
3. Gate variant needed `-fno-sanitize-link-runtime` (clang-21's `ubsan_minimal` already defines
   `__ubsan_handle_load_invalid_value_minimal`; the shim collides otherwise).
4. Isolated oracle appeared to "time out" at 10 s — that was `llvm-symbolizer` on the overflow;
   `ASAN_OPTIONS=symbolize=0` drops it to 0.01 s.

## Raw
`raw/`: `url_gate_probe.c` + `gate_probe_seed.err` (UB_FLAG=0), `oracle_asan_seed.err` (ASan report),
`none_shot.err`/`gate_shot.err` (identical crash), `url_oracle.c`, `url_isolated.py`,
`url_isolated_seed.json`, `build_none.rs`/`build_gate.rs`, `fuzz_target_gate.rs`. Build tree: scratchpad `attr/`.
