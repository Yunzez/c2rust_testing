# Hand-seed probe: tulip × c2rust, `ti_sma` (2026-09-09)

Go/no-go for `docs/llm_input_refinement_plan.md` step 2 (does a plan-materialised seed move region
coverage at all, before any LLM is involved). One boundary, one harness binary, two 60 s rust-only
campaigns with the cell's own parameters (`-max_len 65536`, seed 42, fork mode), generator sha256[:16] = 3d43f75aea807da1.

| condition | initial corpus | final corpus | `ti_sma` regions covered (of 65) |
|---|---|---|---|
| baseline (cell.py default 64-byte seed) | 1 | 15 | **19** — identical to the archived 3 600 s campaign |
| + one hand seed `size64_p5` | 2 | 24 | **58** |

The 7 regions still uncovered: 6 are the `assert` failure arm (`__assert_rtn`, unreachable when the
translation is correct) and 1 is the gap region of `if size <= ti_sma_start(options)`. So the seed
takes the function from 19 to every reachable region except the assert.

**Seed layout** (read off `gen_diff_harness.py`: scalars first, then `buffer_table` / `plan_arr` in
declaration order; the cursor returns 0 past the end):

    size        i32 LE          4 bytes   decoded as 0 + v.rem_euclid(1025) -> 64
    inputs[0]   4096 × f64 LE   32 768 B  fills_from_fuzz (row values 100.0 + (i%17)*0.5)
    options[0]  f64 LE          8 bytes   5.0   (byte offset 32 772)
    outputs[0]  —               0 bytes   zero-filled by the harness

Total 32 780 bytes. Any input shorter than that decodes `options[0] = 0.0`, so `period = 0` and the
indicator returns `TI_INVALID_OPTION` before its loops — this, plus the double→int cast domain, is why
the baseline saturates at 19.

Files: `seeds/ti_sma/size64_p5`, `run.sh` (the two cell.py invocations), `compare.py`,
`{base,hint}_ti_sma_export.json` (llvm-cov exports), `{base,hint}_funnel.json`.
Not in this probe: combined replay / confirmation (rust-only campaign only), other boundaries, other
translators, the deterministic-grid and LLM arms.
