# Milestone 2 — lowering a HarnessPlan into a real harness

Design: [`docs/harness_plan_architecture.md`](../../../../docs/harness_plan_architecture.md).
Plans: [`SUMMARY.md`](SUMMARY.md). Raw logs: [`lowering_run/`](lowering_run/).

```
python3 tools/stu_selector/gen_diff_harness.py --pair <pair> --entry <fn> --plan --ub-free
```

`--plan` reads no schema and writes none: `harness_plan.build_plan()` produces the IR,
`harness_plan.lower_to_schema()` hands it to the existing code emitters. A boundary whose plan is
incomplete prints `harness construction failed: <reason>` and exits 2.

## Result

**14 / 14 plan candidates lowered, built and ran.** The lowering itself has no gaps for bzip2.
Every harness ran 45 s of libFuzzer against the c2rust translation (an ASan-instrumented build).

| boundary | exit | runs in 45 s | divergences | sanitizer / libFuzzer |
|---|---|---:|---:|---|
| `BZ2_bzBuffToBuffCompress` | 0 | 45 128 | 0 | — |
| `BZ2_bzBuffToBuffDecompress` | 0 | 87 959 | 0 | — |
| `BZ2_hbAssignCodes` | 0 | 405 244 | 0 | — |
| `BZ2_hbCreateDecodeTables` | 0 | 316 932 | 0 | — |
| `BZ2_indexIntoF` | 0 | 385 831 | 0 | — |
| `fallbackSort` | 0 | 45 941 | 0 | — |
| `mmed3` | 0 | 25 539 616 | 0 | — |
| `BZ2_hbMakeCodeLengths` | 124 | 276 (3/s) | 0 | **hang** — killed at the 80 s wall clock |
| `fallbackQSort3` | 1 | 1 833 | 0 | ASan SEGV (READ) |
| `fallbackSimpleSort` | 1 | 1 782 | 0 | ASan SEGV (READ) |
| `mainQSort3` | 1 | 802 | 0 | ASan SEGV (READ) |
| `mainSimpleSort` | 77 | 765 | 0 | libFuzzer deadly signal |
| `mainGtU` | 1 | **19** | 0 | ASan **heap-buffer-overflow**, READ of size 1 |
| `mainSort` | 1 | **1** | 0 | ASan SEGV (WRITE) |

**Zero divergences everywhere.** c2rust is a faithful translation, so that is the expected result
and it is the signal that says the comparators are wired correctly: 27.1 M executions across the
seven sound harnesses produced no false positive.

**The seven failures are defects in the plan's input model, not in the translation.** None of them
may be counted as a finding. They are the empirical confirmation of the soundness bugs recorded in
[`SUMMARY.md`](SUMMARY.md#known-unsoundness-found-by-review-2026-09-04--not-yet-fixed):

| candidate | predicted by review | what actually happened |
|---|---|---|
| `mainSort` | proven `ftab[65536]` vs a 4096-element allocation → guaranteed rejection | dies on run **1** |
| `mainGtU` | indices advance past the initial `i1`/`i2` and wrap with an unconstrained `nblock`; the label `in_process_ub_gate` is wrong | ASan heap-buffer-overflow at run **19**. ASan saw it; the in-process UBSan-minimal gate would not have |
| `BZ2_hbMakeCodeLengths` | `alphaSize` unconstrained by the function's internal fixed-size arrays; the label `in_process_ub_gate` is wrong | 3 executions per second, then a wall-clock kill — a liveness failure, before the memory-safety one |
| `fallbackQSort3`, `fallbackSimpleSort`, `mainQSort3`, `mainSimpleSort` | same class: an index whose bound the analyser could not prove, clamped only to the harness's own allocation | SEGV within 800–1 900 runs |

So the honest count is:

* **7 of 14** candidates carry a sound input model and run clean;
* **7 of 14** are rejected by their own harness — 6 on memory safety, 1 on termination;
* the `c_execution` field is unusable: two of the four `in_process_ub_gate` labels are among the
  failures.

## What this does and does not establish

It establishes that the **plan → harness lowering is complete for bzip2**: every adapter the
planner emits has a lowering, the ABI order round-trips, the guards, capacities, prefix lengths and
buffer comparisons all reach the generated Rust, and the resulting harnesses build and execute.

It does not establish that the plans are correct. Seven of them are not, and the next milestone is
the safety analysis, not more lowering. The two fixes with the best ratio of effort to correctness:

1. a **proven** required extent that exceeds the policy allocation must fail harness construction,
   not be capped;
2. index bounds must be derived against the function's **local fixed-extent arrays** as well as its
   pointer parameters — a `CONSTANTARRAY` declaration is the easiest extent in the language to
   prove, and omitting it is what leaves `alphaSize` unbounded.
