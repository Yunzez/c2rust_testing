# quadtree × crown — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 24 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 13 |
| built | 13 |
| executed (corpus > 0) | 13 |
| coverage exported | 10 |

Plan failures, by the generator's own reason:

- **3** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **2** × the produced object is owned as Option<Box<quadtree_bounds_t>> in Rust and the target take
- **2** × signature: callback parameter key_free deferred: function pointers (callback binding) not 
- **1** × the produced object is owned as Option<Box<quadtree_t>> in Rust and the target takes Optio
- **1** × the produced object is owned as Option<Box<quadtree_t>> in Rust and the target takes *mut 
- **1** × point: the boundary passes this POD struct pointer to free(); the harness owns the value a
- **1** × signature: callback parameter descent deferred: function pointers (callback binding) not y

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `elision_` | yes | 1 | 0 | normal 1 | batch |
| `find_` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `get_quadrant_` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `node_contains_` | yes | 11 | 1 | normal 11 | batch |
| `quadtree_bounds_new` | no | 1 | 0 | normal 1 | batch |
| `quadtree_new` | no | 11 | 0 | normal 11 | batch |
| `quadtree_node_isempty` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_isleaf` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_ispointer` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_new` | no | 1 | 0 | normal 1 | batch |
| `quadtree_node_with_bounds` | no | 12 | 1 | normal 12 | batch |
| `quadtree_point_new` | no | 6 | 0 | normal 6 | batch |
| `quadtree_search` | no | 1 | 2 | signal 1 | failed rc=1 |

## 3. Tests side

Status **TEST-FAILS**. exit 134: `unsafe precondition(s) violated: NonNull::new_unchecked requires that the pointer is non-null` (core/src/ptr/unique.rs) under -C debug-assertions -- CROWN's lifted Box/NonNull conversion of a null pointer inside the suite's first group; the shipped suite does not pass through the translation, so denominator only (raw/tests_run.log). E1 had certified this cell (translation_matrix #18) with a differential harness that never took this path.

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 13 harnesses, 3 crash-all (`find_` accepted, `get_quadrant_` accepted, `quadtree_search` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 47 | 0 | 17 | 0 | 0 | 17 | 30 | 0.000 | 0.362 |
| regions | 610 | 0 | 208 | 0 | 0 | 208 | 402 | 0.000 | 0.341 |

Sanity checks: function pass, region pass. Harnesses unioned: 10. Identities outside the universe (excluded, never added): 0 fn / 2 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `node_contains_` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_new` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_node_isempty` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_isleaf` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_ispointer` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_with_bounds` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_point_new` | 6 | 6 | 6 | 6 | 6 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 79 |
| signal | 3 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `find_` | 3 / 3 | ub_associated_termination 3 | 1 |
| `get_quadrant_` | 3 / 3 | ub_associated_termination 3 | 1 |
| `node_contains_` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_isempty` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_isleaf` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_ispointer` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_with_bounds` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_search` | 3 / 3 | ub_associated_termination 3 | 1 |

Total: not_reproducible 5, ub_associated_termination 9

<!-- prose -->
