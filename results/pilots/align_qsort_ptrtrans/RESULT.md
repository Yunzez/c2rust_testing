# ALIGN pilot — qsort × PtrTrans (Trans_PA, gpt-5.1)

Single seed (42), 50,000 records per pair, batch differential. Commit `dda70a4d`. Date 2026-08-25.
Question: does the alignment source (which C↔Rust pairs get compared) change whether the confirmed
quickSort defect is recovered, and does any source produce false divergences?

## Subject
- C: `results/rq1_bugs/qsort_ptrtrans/original_qsort.c` — `swap`, `partition`, `quickSort`
- Rust (verbatim PtrTrans output): `translated_qsort.rs` — `swap`, `partition`, `quick_sort`
- Manual truth: swap↔swap, partition↔partition, quickSort↔quick_sort
- Known defect: `split_at_mut` index error inside `partition` (right.get_mut(j-i) instead of right[0]);
  wrong-index swap silently no-ops via `swap(Option,Option)`.

## Build / harness (all buildable, all harnessable)
- `harness/c_pairs.c` (op ∈ swap|partition|quicksort) compiled `clang -O1 -g -fsanitize=address,undefined
  -fno-sanitize-recover=all`; whole-batch UB gate: rc=0 and empty stderr for all three pairs (`logs/log_*_c.stderr`).
- `harness/main.rs` + `Cargo.toml`: release, debug-assertions + overflow-checks on; wraps translation verbatim.
- `harness/gen_inputs.py`: seed 42, same value distribution as `gen_and_diff.py`. quicksort input is
  byte-identical to the original batch (low=0, high=n-1); partition uses n≥1 (n=0 → C reads arr[-1], UB,
  excluded a priori) with low=0, high=n-1; swap = two random ints.
- Reached functions: n/a (not coverage-instrumented; every record calls the pair's function directly).

## Per-pair differential (identical for every source that proposes the pair)
| pair | valid records | divergences | TTFD (s) | classification |
|---|---|---|---|---|
| swap ↔ swap | 50,000 | **0** | none | — (certificate on this batch) |
| partition ↔ partition | 50,000 | **30,480** (61.0%) | 0.000 (record #0) | 30,480 semantic-difference; 0 C-UB, 0 C-unstable, 0 Rust-failure |
| quickSort ↔ quick_sort | 50,000 | **34,012** (68.0%) | 0.0001 (record #2) | 34,012 semantic-difference; 0/0/0 |

quickSort reproduces the archived 34,012 exactly. `partition` diverges on its own (the bug lives there;
e.g. `[-43, INT_MIN]`: C → ret=0, `[INT_MIN,-43]`; Rust → ret=0, array untouched, because the second
`get_mut(1)` on a 1-element `right` is None). That is data: the defect is observable at the
partition boundary, not only through the top-level sort. `swap` itself is faithful.

## Alignment sources — what each proposes
| source | proposed pairs | abstentions | note |
|---|---|---|---|
| name equality | swap, partition | — | quickSort has no name-equal Rust fn |
| tool map (`qsort_Trans_PA_trans_metadata.jsonl`, `rust_definition_name`) | swap, partition, quickSort↔quick_sort | — | correct on this crate |
| matcher, main (Hungarian, partial) | swap, partition, quickSort↔quick_sort | 0 | reproduces ★ cell 3/3 |
| matcher, abstention eps=0.01 | swap, quickSort↔quick_sort | **partition** | partition isolated (low two-sided confidence) |
| manual | all three | — | — |

Matcher diagnostics (`matcher/matcher_*.log`): quickSort→quick_sort 0.495 vs 2nd 0.271 (margin 0.225, clean);
swap→swap 0.353 vs 0.172. **partition's row-best is `quick_sort` (0.296) over the true `partition` (0.278),
margin 0.018** — the correct pair is obtained only because the 1-1 assignment gives `quick_sort` to
quickSort. Abstention correctly flags this as the shaky pair. Honest reading: the ★ 3/3 rests on the
assignment, not on partition being individually confident.

## Downstream table
| source | harnessable | buildable | true defects recovered | false divergences | missed defects | abstentions |
|---|---|---|---|---|---|---|
| name-eq | 2/2 | 2/2 | **0** (quickSort pair never proposed) | 0 | 1 | 0 |
| tool map | 3/3 | 3/3 | 1 | 0 | 0 | 0 |
| matcher (main) | 3/3 | 3/3 | 1 | 0 | 0 | 0 |
| matcher (abstain 0.01) | 2/2 | 2/2 | 1 | 0 | 0 | 1 (partition) |
| manual | 3/3 | 3/3 | 1 | 0 | 0 | 0 |

Expectation confirmed by test: name-eq misses the defect; tool map / matcher / manual recover it; zero
false divergences everywhere (every divergence is on a manual-truth pair). Caveat on name-eq: it still
sees 30,480 divergences on partition↔partition — the *root cause* is reachable by name-eq, only the
quickSort-level symptom is not. With abstention the matcher loses the partition observation but keeps the
defect.

## Secondary: cJSON × PtrTrans buildable wrong-map witness
- **No `cjson*_trans_metadata.jsonl` exists anywhere on disk** (searched whole FS; only bzip2/lodepng/qsort
  under PA_trans_projects, plus parsed_projects/Trans_* for other libs). It lived in a lost scratchpad.
- The cJSON 1.7.x source PtrTrans translated is also not on disk (only older CRUST-bench/c2rust cJSON.c, 58
  fns, lacking `skip_multiline_comment`/`cJSON_ParseWithLengthOpts`).
- Fallback audit (`harness/cjson_doc_audit.py`, `logs/cjson_audit.json`): the archived crate builds
  (`cargo check` green, 186 warnings), 108 fns; its self-declared `/// Translated from:` map has 10 records,
  **0 mismatches**.
- **Verdict: no buildable wrong-map witness established.** Wrong-map downstream harm stays *potential*
  (lodepng 143/255 evidence, non-buildable). To get a witness, re-run PtrTrans on cJSON and keep the jsonl.

## What did not work / fixed
1. `tools/stu_selector/analyzer` binary was absent → `cargo build --release` (~1 min).
2. `results/rq2_cells/ptrtrans_rename/qsort_ptrtrans.rs` carries a stray `use crate::*;` → stripped to make a
   standalone lib crate for the analyzer.
3. No per-pair harness existed (archived one drives only quickSort) → wrote op-switched C/Rust drivers.
4. TTFD in batch mode = wall-clock × (first-diverging index+1)/N; sub-millisecond here, reported as such.
5. `eval_rq2_matcher.py` named in the task does not exist; the runner is `scripts/eval_rq3_matcher.py`
   (same CFGS: main / full_abstain eps=0.01); I called `tools/stu_selector/matcher.py` directly with
   `--truth qsort_truth.json`, which is what the ★ cell recorded (`name_preserving_v1.json:_ptrtrans_qsort_rename_demo`).

## Files
`result.json` (all fields), `harness/`, `logs/res_*.json` + sanitizer stderr (empty), `matcher/` (analyzer JSON,
matcher logs, emitted pairs). Full 50k outputs in scratchpad `align/out_*_{c,rs}.txt` (68 MB, not copied).
