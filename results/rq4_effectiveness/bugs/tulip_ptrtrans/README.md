# tulipindicators × PtrTrans (Trans_PA): ✗(PA) — its required program-analysis stage crashes on the fn-ptr dispatch table

**Verdict: `✗(PA)`** — process failure, measured at **$0** (2026-07-10). PtrTrans's *required*
pointer-analysis pre-pass (the "PA" in Trans_PA) cannot produce its inputs on tulip: **pa_struct
hard-crashes** and **pa_func does not terminate**. No faithful Trans_PA run is possible.

## Setup (all $0)

- Entry: `crown_dataset/tulipindicators/` = **tulip's own shipped amalgamation**
  (`make tiamalgamation.c`, upstream TulipCharts v0.9.2, 248 KB single .c — the same single-file
  entry style as PtrTrans's qsort) + local `compile_commands.json`. Compiles clean (`gcc` exit 0).
- Money guard: the pipeline ran with a **dummy API key** — setup stages don't touch the API; the
  first real LLM call fails auth → guaranteed $0.

## What passed

PtrTrans's own pipeline stages all run: expand/deal → doxygen → KG construction (856 relationships,
100%) → slicing → **reaches the first LLM call** (stopped by the auth guard). So the *pipeline*
ingests tulip fine.

## What failed — the PA pre-pass (SVF-based `pa_func` / `pa_struct`)

IR: `clang-14 -S -emit-llvm` on the amalgamation → 4.4 MB `.ll`, clean.

1. **`pa_struct` hard-crashes** — uncaught C++ exception, abort:
   ```
   Analyzing struct: struct.ti_indicator_info
     Getting the usage scenario of the structure...
   terminate called after throwing an instance of 'std::invalid_argument'
     what():  stoul
   ```
   The crash site is **`ti_indicator_info` — the 104-entry function-pointer dispatch-table struct.**
   Reproduced twice: with `-g` debug info and without (`pa_struct_crash.log`,
   `pa_struct_crash_nodebug.log`) — not an IR-flag artifact.
2. **`pa_func` does not terminate in 1 h** (exit 124; qsort's completes in seconds, and optipng's
   **larger** 8.6 MB linked IR completes fine — so it is not size; the 104-way fn-ptr table is the
   pathological case for its context-sensitive analysis).

Without `struct_analysis_report.json` / a completed `func_analysis_report.json`, Trans_PA — the
configuration PtrTrans's paper claims — cannot run on tulip.

## Why this matters (cross-tool echo)

The **function-pointer dispatch table defeats a second tool at its analysis stage**: SACTOR's
link-closure can't see the table's address-taken refs (`results/rq4_effectiveness/bugs/tulip_sactor/`, ✗ verify), and
PtrTrans's struct-analysis crashes *on the very struct that holds the pointers*. Same construct,
two tools, two different stages — the fn-ptr nemesis is not a SACTOR quirk.

We did **not** patch the SA tools (as-shipped evaluation, same policy as tulip×SACTOR).

## Files
- `pa_struct_crash.log` / `pa_struct_crash_nodebug.log` — the stoul abort, ×2
- `pa_func_timeout_tail.log` — 1 h non-termination (partial output)
- `pipeline_reached_llm_boundary.txt` — the dummy-key APIError proving every pre-LLM stage passed
