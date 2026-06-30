# RQ3 — matcher under renaming / restructuring (v1, full ablation)

> **RQ3:** Can the matcher recover **high-confidence** C↔Rust function correspondences under **renaming
> and idiomatic restructuring, WITHOUT using function names** — isolating uncertain cases as `ambiguous`,
> and significantly beating the name-equality baseline?

Runner: `scripts/eval_rq3_matcher.py` (imports `matcher.match()` directly; stops at the matcher —
frontier/harness/fuzz are NOT part of RQ3). Rows of JSON in `results/rq3_rows/*.v2.json`. Plan +
metric definitions: `results/rq3_eval_plan.md`. Metrics: `precision = correct/matched`,
`recall = correct/scorable`, `coverage = accepted_on_truth/scorable`; both **micro** (pooled functions)
and **macro** (unweighted mean over programs) reported.

## Table 1 — main results (full matcher, deployment; rows = regime)
| regime | progs | scorable | matched | extra | ambig | correct | precision μ/M | recall μ/M | name-eq recall μ/M |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| raw-LLM (gpt-5-mini) — **renamed** (hand-labeled truth) | 10 | 89 | 90 | 1 | 0 | 78 | .867 / .958 | **.876 / .961** | **.124 / .086** |
| raw-LLM (gpt-5-nano) — **renamed** (hand-labeled truth) | 10 | 75 | 86 | 12 | 0 | 65 | .756 / .891 | **.867 / .914** | **.613 / .502** |
| SACTOR idiomatic — **names kept, io-shape transformed** (independent truth) | 2 | 12 | 14 | 2 | 0 | 12 | .857 / .854 | **1.000 / 1.000** | 1.000 / 1.000 |
| **SACTOR mechanically-renamed** — **renamed, INDEPENDENT truth** | 2 | 12 | 14 | 2 | 0 | 12 | .857 / .854 | **1.000 / 1.000** | **0.000 / 0.000** |

- **Renamed regime is the real test.** name-equality recovers only **12.4% (micro) / 8.6% (macro)** — the
  LLM renamed nearly everything; this is what Fluorine/RustAssure's name pairing gets. The structural
  matcher recovers **87.6% / 96.1%** → **~7–11×**. (micro < macro because the two big hard programs
  bignum/tinyexpr dominate the pooled function count; per-program the matcher is near-perfect on 8/10.)
- **Cross-model robustness (gpt-5-mini vs gpt-5-nano).** Two models rename very differently — name-equality
  is .124 for mini but **.613** for nano (nano kept the original names on bignum/opcode_dispatch/leb128,
  renamed the rest). Yet the matcher's recall is **stable across both (.876 vs .867 micro; .961 vs .914
  macro)** — i.e. matcher quality does NOT depend on how aggressively the model renames, whereas the
  name-equality baseline swings wildly (.124 → .613) with the model's naming whim. nano's lower micro
  precision (.756) is one program: **tinyexpr — nano DISSOLVED 13 of 28 C functions** (constants/operators
  folded into match-arms / enum methods, no standalone fn). Forced matching mis-pairs those no-counterpart
  C functions (precision .308 on tinyexpr); **abstention isolates them and lifts precision to .963** — a
  clean live demonstration that "isolate, don't guess" beats forcing when the translation drops functions.
- **SACTOR names-kept is a robustness check, NOT a renaming challenge** (state this in the caption):
  SACTOR preserves names (name-eq trivially 1.0) but rewrites signatures to idiomatic Rust
  (`ptr+len→&[u8]`, `T* out → Option<usize>`); the matcher recovers 100% — it does NOT break under
  io-shape transformation. `extra=2` = it also pairs 2 top-level helpers absent from SACTOR's map.
- **SACTOR mechanically-renamed is the independent-truth renaming challenge** (the strongest single row):
  the Rust functions are mechanically renamed to `r_####` (so name-eq → **0.0**) while SACTOR's own
  `function_name_map.json` stays the ground truth — **zero hand-labeling**. The matcher still recovers
  **100%** by structure (identical to the names-kept run; only name-eq changed). This rebuts the "raw-LLM
  truth is self-labeled" objection: on a corpus we did NOT label, under genuine renaming, name-equality
  gets 0 and the matcher gets everything.

## Table 2 — ablation (THE key table; kills "signatures are enough"), raw-LLM micro
| method | uses | recall | precision | coverage | ambig | lil (homog. cluster) |
|---|---|--:|--:|--:|--:|--:|
| name-equality | identity c→c (no matcher) | .124 | — | — | — | 1.000 (names kept) |
| **shape-only** | normalized param/return categories + arity ONLY | **.652** | **.644** | 1.0 | 0 | **.359** (46/128) |
| node-only | shape + metrics + operator histogram (`--no-topo`) | .820 | .811 | 1.0 | 0 | .742 (95/128) |
| full (forced) | node + call-graph topology, forced 1-1 | **.876** | .867 | 1.0 | 0 | **.984** (126/128) |
| full + abstain | full + ambiguity isolation (deployment) | .708 | **.969** | .73 | 25 | — |

- **shape-only is the rebuttal to "isn't the type signature enough?":** it gets only **65% recall at 64%
  precision** — much weaker than full (88%/87%). On the **lil** topology stress fixture (55 identical
  `fnc_*` command handlers) shape-only **collapses to 36%**; metrics/operators lift it to 74%; only
  call-graph **topology** cracks the homogeneous cluster → **98%** (+24pp over node-only). lil is where the
  per-function ceiling is starkest. (The old "lil 61%" figure was an even-earlier no-topology baseline —
  the full matcher is 98%, NOT 61%.)
- **forced vs abstain report different metrics** (don't conflate): forced matches all scorable C
  (recall/precision); **full+abstain** is the deployment mode — at eps=0.01 it trades recall for
  **96.9% precision** (coverage 73%, 25 ambiguous isolated rather than guessed). A confident-but-wrong
  alignment breaks that boundary's differential test, so isolation > forcing.
- SACTOR shows the same ladder even with names kept: shape-only .75 (io-shape was transformed!) →
  node-only/full 1.0 — i.e. signatures alone are insufficient even in the names-kept regime.

## Table 3 — failure taxonomy (raw-LLM; report honestly, do NOT chase 100%)
| failure class | example | cause | handling |
|---|---|---|---|
| homogeneous trivial cluster | tinyexpr `builtin_*` (LLM exploded the C builtin table into ~40 one-liners) | structure saturated — many near-identical shapes | isolated as `ambiguous` under abstain (future: signal-C constants/literals) |
| semantic return rewrite | bignum `to_int` ↔ `to_string` 2-cycle swap | return shape lost; high two-sided margin so abstain does NOT catch it | needs a literal/constant feature (signal-C), not topology |
| Rust-only hub | bignum `require`/`default` (LLM-added) | high in-degree pollutes propagation | df-cap hub-stopword filtering (already in matcher) |
| decomposition (1→N) | base64 1 C fn → several Rust helpers | extra Rust nodes | partial matching leaves them `rust_only` (here base64 still 2/2) |
| dissolution (N→0) | gpt-5-nano tinyexpr: 13 C fns (pi/e/add/…/optimize) folded into match-arms / enum methods / inlined | C function has NO standalone Rust counterpart | abstention isolates them as `ambiguous` (forced matching mis-pairs → precision .31; abstain → .96) |
The two real raw-LLM residuals are bignum (25/27; the to_int/to_string swap) and tinyexpr (19/28; the
builtin cluster). Both are correctly the HARDEST cases; abstain isolates the tinyexpr cluster, the bignum
swap is the known signal-C case (optional, not chased).

## Hard artifact — name-scramble self-check (matcher uses NO names)
`scripts/name_scramble_check.py` (JSON-level): rewrite every C name → `c_0000…` and Rust name →
`r_0000…` over the analyzer outputs (`functions[].name`, `raw_edges.from/to`, truth map), preserving
topology (names = opaque node IDs), then re-run the matcher and compare matched pairs.

| | result |
|---|---|
| matched-pair delta (normal vs scrambled), 10 programs / 90 pairs | **0** (byte-identical matching) |
| per-program correctness normal == scrambled | yes, all 10 |
| **negative control**: name-equality recall | normal .124 → **scrambled .000** |

delta=0 (stronger than the ≤1 tolerance) proves the matcher's output is invariant to function names; the
negative control (name-eq → 0) proves the scramble actually destroyed the name channel. This is the hard
rebuttal to "you used names". (`results/rq3_rows/name_scramble_check.json`.)

## Status vs plan
DONE: runner with matched/extra/ambiguous + micro/macro; 5-method ablation incl. **shape-only** (new
`matcher --shape-only`); **2 LLM models** (gpt-5-mini + gpt-5-nano, both hand-labeled) + 2 SACTOR rows;
failure taxonomy; **name-scramble self-check (delta=0)**; **mechanically-renamed SACTOR** (independent-truth
rename row, name-eq 0.0 / matcher 1.0). lil regression gate still PASS (126/128). nano truth_nano/ labeled
by 5 source-reading agents (matcher-blind) + validated (every value exists in analyzer output, no missing).
NEXT (optional): manual ports (tinyexpr/heatshrink/QOI, external validity).

Reproduce: `python3 scripts/eval_rq3_matcher.py --source "raw-LLM (gpt-5-mini)" --regime renamed
--truth-dir experiments/llm_transpiler/truth --c-pairs benchmark/pairs --rust-out
experiments/llm_transpiler/out --json results/rq3_rows/rawllm_gpt5mini.v2.json`
