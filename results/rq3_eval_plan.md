# RQ3 evaluation plan (matcher under renaming / restructuring)

Authoritative plan for the RQ3 experiment. Agreed design (Claude + Codex, 2026-06-30). Survives compact.

## RQ3 (tightened)
> Can the matcher recover **high-confidence** C↔Rust function correspondences under **renaming and
> idiomatic restructuring, WITHOUT using function names** — isolating uncertain cases as `ambiguous`,
> and significantly beating the name-equality baseline on recall?

Not "we always get 100%". The claim is: recover usable correspondences at high precision, abstain on the
rest, and crush name-equality (what Fluorine/RustAssure rely on) under renaming.

## Metrics (precise; Codex's split, avoids coverage>1 confusion)
- `truth_scorable` = |truth map| (C functions that SHOULD be matched; scaffolding main/tests excluded)
- `accepted_on_truth` = accepted pairs whose C ∈ truth
- `extra_accepted`    = accepted pairs whose C ∉ truth (matcher paired a non-truth C)
- `correct` = accepted pairs with truth[c] == r
- **precision = correct / (accepted_on_truth + extra_accepted)**  (extra/wrong pairs penalize precision)
- **recall    = correct / truth_scorable**
- **coverage  = accepted_on_truth / truth_scorable**
- Report **both precision and recall**, **micro AND macro** (main text micro; macro in parens/appendix —
  macro stops big programs like tinyexpr/lil from dominating).

## Table 1 — main results (rows = regime)
| regime | programs | scorable | matched | correct | precision (μ/M) | recall (μ/M) | name-eq recall |
|---|--|--|--|--|--|--|--|
| raw-LLM (gpt-5-mini) | 10 | 89 | … | … | … | … | … |
| SACTOR idiomatic (names kept) | 2+ | … | … | … | … | … | ~1.0 |
| **SACTOR mechanically-renamed** | 2+ | … | … | … | … | … | ~0 |
| (manual port — later) | … | … | … | … | … | … | … |
| ALL | … | … | … | … | … | … | … |

## Table 2 — ablation (THE key table; kills "signatures are enough")
| method | what it uses | raw-LLM recall | precision | tinyexpr | bignum | lil |
|---|---|--|--|--|--|--|
| name-equality | identity c→c (from truth, no matcher) | low | — | … | … | … |
| **shape-only** | param/return io-shape ONLY | … | … | … | … | … |
| node-only | shape + metrics + operator histogram (NO topology) = `--no-topo` | … | … | … | … | … |
| full | node + call-graph topology + assignment (forced) | … | … | … | … | … |
| full + abstain | full + ambiguity isolation (deployment mode) | … | … | … | … | … |
- shape-only is the must-have: reviewers will ask "isn't type signature enough?" — homogeneous clusters
  (tinyexpr builtins, bignum twins, lil fnc_* , opcode_dispatch) should make shape-only collapse.
- full vs full+abstain: forced (every C gets best match) vs deployment (abstain on ambiguous) — report
  the precision/coverage tradeoff, don't conflate.
- Needs matcher flags: `--shape-only` (NEW, add), `--no-topo` (have), `--no-partial`/`--abstain-eps`
  (have) — confirm all five rows map to flags before running.

## Table 3 — failure taxonomy (more valuable than chasing 100%)
| failure class | example | cause | intended handling |
|---|---|---|---|
| homogeneous trivial cluster | tinyexpr builtin_* | structure saturated | ambiguous / future signal-C (constants/literals) |
| semantic return rewrite | bignum to_int/to_string | shape lost | ambiguous / literals |
| Rust-only hub | bignum require/default | topology pollution | boilerplate/hub filtering |
| decomposition (1→N) | base64 2→5 | extra Rust helper | partial matching / rust_only |
Report residual misses honestly; lil (128 fns, ~61%) is the headline stress fixture.

## Hard artifact: "matcher does NOT use names" (3 layers of evidence)
1. **Code-level**: matcher.py documents features USED (io-shape, metrics, operator histogram, call-graph
   topology) vs NOT USED (raw name, edit distance, common prefix/suffix, tokenized identifier). Names are
   used ONLY as opaque node IDs within each graph, never as cross-language similarity features.
2. **Experiment-level (`--name-scramble`)**: replace C names → c_0001… and Rust names → r_0001… (only
   identifiers; structure/edges unchanged, consistent within each side), re-run analyzer + matcher.
   **scrambled_result == normal_result (delta ≤ 1 pair).** If it differs, something still leaks names.
3. **Negative control**: name-equality baseline on scrambled names → ~0 (proves the name channel was
   actually destroyed).

## Data layers
1. **raw-LLM** (10 programs, `experiments/llm_transpiler/truth/*.json`) — primary, hand-labeled truth.
2. **independent-truth**: SACTOR `function_name_map.json` (hamming/fft + more SACTOR successes). Add a
   **mechanically-renamed SACTOR** (scramble Rust fn names, keep the map as truth) → real rename test with
   INDEPENDENT truth (removes the "raw-LLM truth is self-labeled" objection). HIGH PRIORITY.
3. **stress fixtures**: lil / bignum / tinyexpr — for failure/stress analysis (not external benchmarks).

## DoD
1. One script `scripts/eval_rq3_matcher.py --suite <...> --out results/rq3_matcher.json` emitting
   `{programs:[...], micro:{}, macro:{}, by_regime:{}, ablations:{}, failures:[...]}`.
2. Per-program fields: program, regime, scorable, matched, correct, ambiguous, precision, recall,
   name_eq_recall, shape_only_recall, node_only_recall, full_forced_recall, full_abstain_recall.
3. Generated `results/rq3_matcher_v1.md` (the 3 tables).
4. name-scramble check passes (scrambled ≈ normal; name-eq-on-scrambled ≈ 0).

## Execution order
1. `eval_rq3_matcher.py` reusing existing truth + matcher; raw-LLM 10 → Table 1 row 1.
2. Add ablation columns (needs `--shape-only` in matcher) → Table 2.
3. **name-scramble self-check** (highest-value rebuttal). 
4. **mechanically-renamed SACTOR** (independent-truth rename regime).
5. Failure taxonomy (Table 3); do NOT chase signal-C to 100%.
6. (later) manual ports.

## Top-2 design decisions (must-do; block the main reviewer attacks)
- **io-shape-only baseline** (Table 2) — rebuts "signatures are enough".
- **name-scramble self-check** (+ renamed-SACTOR) — rebuts "you used names" AND "you self-labeled truth".

## Current progress (already done, 2026-06-30)
- `scripts/rq3_eval.py` (v0 runner): raw-LLM row (precision .867 / recall .876 / name-eq .124) +
  SACTOR-idiomatic row (.857 / 1.0 / 1.0). `results/rq3_eval_v1.md`, rows in `results/rq3_rows/`.
- v0 → upgrade to the DoD script: add macro, ablations (shape-only/node-only/forced/abstain),
  name-scramble, renamed-SACTOR, failure taxonomy.
