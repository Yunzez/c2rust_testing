# RQ3 — name-independent matcher precision/recall under renaming (v1)

> **SUPERSEDED by `results/rq3_matcher_v1.md`** (full ablation ladder: name-eq / shape-only / node-only /
> full / full+abstain, micro+macro, matched/extra/ambiguous breakout). This v1 doc is the earlier
> 2-column (recall + name-eq) cut; kept for history.


**Question:** can the matcher recover the C↔Rust function correspondence when the translation renames/
restructures — where name-based pairing (Fluorine, RustAssure) fails? **Metric:** precision/recall vs a
hand-labeled ground-truth map, with the name-equality baseline (= what name-based oracles rely on) and a
call-graph-topology ablation. Runner: `scripts/rq3_eval.py` (stops at the matcher — frontier/harness/fuzz
are NOT part of RQ3). Table ROWS = data sources (renaming regimes), micro-averaged over their programs.

## Main table (rows = data source)

| source (renaming regime) | #prog | #truth pairs | precision | recall | recall −topo | **recall name-eq** |
|---|--:|--:|--:|--:|--:|--:|
| raw-LLM (gpt-5-mini) — **renamed** | 10 | 89 | 0.867 | **0.876** | 0.820 | **0.124** |
| SACTOR idiomatic — **names kept, io-shape transformed** | 2 | 12 | 0.857 | **1.000** | 1.000 | **1.000** |
| raw-LLM (2nd model) | — | — | (TODO) | | | |
| human port (tinyexpr/heatshrink/QOI) | — | — | (TODO, hand-labeled) | | | |

**The two rows together = the full C1 story.**
- **Renamed regime (raw-LLM):** name-equality recovers only **12.4%**; the structural matcher recovers
  **87.6%** → **~7×**. Removing call-graph topology drops it to 82.0% (topology +5.6pp). This is where
  name-based oracles (Fluorine, RustAssure) are blind.
- **Names-kept-but-io-shape-transformed regime (SACTOR):** SACTOR preserves function names but rewrites
  signatures to idiomatic Rust (`ptr+len → &[u8]`, `T* out → Option<usize>` return). name-equality is
  trivially 100% here; the matcher ALSO recovers 100% — i.e. it does NOT break under idiomatic shape
  transformation. (SACTOR's `function_name_map.json` = tool-emitted ground truth, no hand-labeling.)

So: name-equality works ONLY when names are preserved and collapses under renaming; the matcher works in
BOTH regimes. (SACTOR coverage > 1 = the matcher also pairs a couple of extra top-level helpers not in
SACTOR's map; precision 0.857 accounts for them.)

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

## SACTOR row reproduce
Staged from `tools/frameworks/sactor/tests/c_examples/{hamming,fft}_crust/`: truth = the idiomatic
`function_name_map.json` (minus `main`); C = `c_for_analyzer/` (has compile_commands.json); Rust = the
idiomatic `combined.rs` wrapped in a minimal crate. Then `scripts/rq3_eval.py --source "SACTOR idiomatic"`.

## Next rows (to make it a table, per the data plan)
1. **raw-LLM 2nd/3rd model** (gpt-4o / claude) — re-translate same programs → new renamings, mostly reuse
   truth. Cheapest way to add rows. (DONE: gpt-5-mini.)
2. **SACTOR idiomatic** — DONE (hamming/fft); add more SACTOR successes if available.
3. **human port** — tinyexpr→tinyexpr-rs, heatshrink→embedded-heatshrink, QOI→qoi-rust (hand-label map;
   see `results/rq3_human_port_candidates.md`).

Reproduce: `python3 scripts/rq3_eval.py --source "raw-LLM (gpt-5-mini)" --truth-dir
experiments/llm_transpiler/truth --c-pairs benchmark/pairs --rust-out experiments/llm_transpiler/out
--json results/rq3_rows/rawllm_gpt5mini.json`
