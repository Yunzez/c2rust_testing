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
| regime | programs | scorable | matched | extra_acc | ambig | correct | precision (μ/M) | recall (μ/M) | name-eq recall |
|---|--|--|--|--|--|--|--|--|--|
| raw-LLM (gpt-5-mini) | 10 | 89 | … | … | … | … | … | … | … |
| SACTOR idiomatic (names kept) | 2+ | … | … | … | … | … | … | … | ~1.0 |
| **SACTOR mechanically-renamed** | 2+ | … | … | … | … | … | … | … | ~0 |
| (manual port — later) | … | … | … | … | … | … | … | … | … |
| ALL | … | … | … | … | … | … | … | … | … |

Break out `matched`, `extra_accepted` (accepted pairs whose C ∉ truth), and `ambiguous` so the reader
sees WHY precision can be < recall (it happens when matched > scorable, i.e. the matcher accepted an
extra/wrong pair — e.g. v0 tinyexpr matched 29 vs 28 truth → raw-LLM precision .867 < recall .876).

**SACTOR rows framing (state explicitly in the caption):** *SACTOR names-kept is a signature-
restructuring ROBUSTNESS row (name-eq trivially ~1.0), NOT a renaming challenge; the mechanically-renamed
SACTOR row is the independent-truth RENAMING challenge.* Do not cite names-kept SACTOR as evidence of
beating name-equality.

## Table 2 — ablation (THE key table; kills "signatures are enough")
| method | what it uses | recall | precision | coverage | ambig | tinyexpr | bignum | lil |
|---|---|--|--|--|--|--|--|--|
| name-equality | identity c→c (from truth, no matcher) | low | — | — | — | … | … | … |
| **shape-only** | normalized param/return CATEGORIES only (see below) | … | … | — | — | … | … | … |
| node-only | shape + metrics + operator histogram, NO topology = `--no-topo` | … | … | — | — | … | … | … |
| full (forced) | node + topology + assignment, every C forced to best match | … | … | — | — | … | … | … |
| full + abstain | full + ambiguity isolation (DEPLOYMENT mode) | … | … | … | … | … | … | … |
- **shape-only must be defined precisely** (or it's not a clean baseline): uses ONLY the normalized
  parameter/return CATEGORIES — `scalar / input_buffer / output_buffer / inout_buffer / string / struct /
  callback / unknown` (+ arity if implied by shape). EXCLUDES function name, operators, metrics, call-graph
  topology, AND degree. (If it uses degree it is not shape-only.) Homogeneous clusters (tinyexpr builtins,
  bignum twins, lil fnc_*, opcode_dispatch) should make shape-only collapse — that's the point.
- **forced vs abstain report DIFFERENT metrics** (don't conflate / don't compare unfairly):
  - *full (forced)*: matches ~all scorable C; report forced precision + recall.
  - *full + abstain* (deployment): report **accepted precision + recall + coverage + #ambiguous** — the
    high precision is meaningless without coverage, so coverage is mandatory here.
- Matcher flags: `--shape-only` (NEW, add), `--no-topo` (have), forced vs `--abstain-eps` (have) —
  confirm all five rows map to flags before running.

## Table 3 — failure taxonomy (more valuable than chasing 100%)
| failure class | example | cause | intended handling |
|---|---|---|---|
| homogeneous trivial cluster | tinyexpr builtin_* | structure saturated | ambiguous / future signal-C (constants/literals) |
| semantic return rewrite | bignum to_int/to_string | shape lost | ambiguous / literals |
| Rust-only hub | bignum require/default | topology pollution | boilerplate/hub filtering |
| decomposition (1→N) | base64 2→5 | extra Rust helper | partial matching / rust_only |
Report residual misses honestly. **lil is the TOPOLOGY stress fixture**: node-only / per-function
matching collapses on the homogeneous `fnc_*` command-handler cluster (the old ~61% figure was the
no-topology baseline), while topology recovers ~126/128. Use lil to demonstrate topology's contribution
in Table 2 — NOT to claim the full matcher is only 61%.

## Hard artifact: "matcher does NOT use names" (3 layers of evidence)
1. **Code-level**: matcher.py documents features USED (io-shape, metrics, operator histogram, call-graph
   topology) vs NOT USED (raw name, edit distance, common prefix/suffix, tokenized identifier). Names are
   used ONLY as opaque node IDs within each graph, never as cross-language similarity features.
2. **Experiment-level (`--name-scramble`)**: **JSON-level scramble FIRST (recommended)** — rename over the
   analyzer OUTPUTS, not the source: rewrite `functions[].name`, `raw_edges.from/to` (names are opaque node
   IDs, scrambled consistently within each side so topology is preserved), and the truth map keys/values →
   c_0001…/r_0001…, then run matcher. This directly tests "matcher uses no name similarity" without risking
   source-rewrite breakage (C decls/calls/macros, Rust call sites, `#[no_mangle]`, extern symbols).
   **scrambled_result == normal_result (delta ≤ 1 pair).** Source-level scramble = optional STRONGER
   evidence, later; do NOT let it block RQ3.
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
3. Generated `results/rq1_matching/matcher_ablation_v1.md` (the 3 tables).
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
  SACTOR-idiomatic row (.857 / 1.0 / 1.0). `results/rq3_eval_v1.md`, rows in `results/rq1_matching/rows/`.
- v0 → upgrade to the DoD script: add macro, ablations (shape-only/node-only/forced/abstain),
  name-scramble, renamed-SACTOR, failure taxonomy.
