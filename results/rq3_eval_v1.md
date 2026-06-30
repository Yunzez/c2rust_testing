# RQ3 — name-independent matcher precision/recall under renaming (v1)

**Question:** can the matcher recover the C↔Rust function correspondence when the translation renames/
restructures — where name-based pairing (Fluorine, RustAssure) fails? **Metric:** precision/recall vs a
hand-labeled ground-truth map, with the name-equality baseline (= what name-based oracles rely on) and a
call-graph-topology ablation. Runner: `scripts/rq3_eval.py` (stops at the matcher — frontier/harness/fuzz
are NOT part of RQ3). Table ROWS = data sources (renaming regimes), micro-averaged over their programs.

## Main table (rows = data source)

| source (renaming regime) | #prog | #truth pairs | precision | recall | recall −topo | **recall name-eq** |
|---|--:|--:|--:|--:|--:|--:|
| raw-LLM (gpt-5-mini) | 10 | 89 | 0.867 | **0.876** | 0.820 | **0.124** |
| raw-LLM (2nd model) | — | — | (TODO) | | | |
| SACTOR idiomatic | — | — | (TODO, tool-emitted map) | | | |
| human port (tinyexpr/heatshrink/QOI) | — | — | (TODO, hand-labeled) | | | |

**Headline (row 1):** the structural matcher recovers **87.6%** of renamed correspondences; the
name-equality baseline recovers **12.4%** → **~7× better**. This is the quantified C1 value: name-based
oracles are blind to the renamed/restructured translations our matcher handles. Removing call-graph
topology propagation drops recall to 82.0% (topology contributes +5.6pp).

## Per-program (raw-LLM gpt-5-mini)

| pair | truth | matched | correct | correct −topo | name-eq |
|---|--:|--:|--:|--:|--:|
| base64 | 2 | 2 | 2 | 2 | 0 |
| bignum | 27 | 27 | 25 | 22 | 0 |
| hash_table | 8 | 8 | 8 | 8 | 0 |
| hex_encode | 2 | 2 | 2 | 2 | 1 |
| leb128 | 3 | 3 | 3 | 3 | 0 |
| linked_list | 5 | 5 | 5 | 5 | 0 |
| opcode_dispatch | 8 | 8 | 8 | 8 | 0 |
| rle_codec | 2 | 2 | 2 | 2 | 0 |
| rpn_eval | 4 | 4 | 4 | 4 | 0 |
| tinyexpr | 28 | 29 | 19 | 17 | 10 |

Notes: most programs have name-eq=0 (the LLM fully renamed) — name-based pairing would recover nothing.
tinyexpr is the hardest (the LLM exploded the C builtin table into ~40 `builtin_*` one-liners →
matched 29 vs 28 truth, correct 19); it also has the most name-preserved (10) since math builtins kept
names. Coverage can slightly exceed 1 when the translation exposes extra functions the matcher pairs;
precision (correct/matched) accounts for any wrong extra pairs.

## Next rows (to make it a table, per the data plan)
1. **raw-LLM 2nd/3rd model** (gpt-4o / claude) — re-translate same programs → new renamings, mostly reuse
   truth. Cheapest way to add rows.
2. **SACTOR idiomatic** (hamming/fft + more) — SACTOR emits `function_name_map.json` = ground truth, no
   hand-labeling.
3. **human port** — tinyexpr→tinyexpr-rs, heatshrink→embedded-heatshrink, QOI→qoi-rust (hand-label map;
   see `results/rq3_human_port_candidates.md`).

Reproduce: `python3 scripts/rq3_eval.py --source "raw-LLM (gpt-5-mini)" --truth-dir
experiments/llm_transpiler/truth --c-pairs benchmark/pairs --rust-out experiments/llm_transpiler/out
--json results/rq3_rows/rawllm_gpt5mini.json`
