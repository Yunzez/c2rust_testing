# quadtree × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 24 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 17 |
| built | 17 |
| executed (corpus > 0) | 17 |
| coverage exported | 16 |

Plan failures, by the generator's own reason:

- **3** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **2** × signature: callback parameter key_free deferred: function pointers (callback binding) not 
- **1** × point: the boundary passes this POD struct pointer to free(); the harness owns the value a
- **1** × signature: callback parameter descent deferred: function pointers (callback binding) not y

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `elision_` | yes | 1 | 0 | normal 1 | batch |
| `find_` | yes | 9 | 1 | normal 9 | batch |
| `get_quadrant_` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `node_contains_` | yes | 11 | 1 | normal 11 | batch |
| `quadtree_bounds_extend` | no | 6 | 0 | normal 6 | batch |
| `quadtree_bounds_free` | no | 1 | 0 | normal 1 | batch |
| `quadtree_bounds_new` | no | 1 | 1 | normal 1 | batch |
| `quadtree_free` | no | 12 | 0 | normal 12 | batch |
| `quadtree_insert` | no | 10 | 0 | normal 10 | batch |
| `quadtree_new` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_isempty` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_isleaf` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_ispointer` | no | 12 | 1 | normal 12 | batch |
| `quadtree_node_new` | no | 1 | 1 | normal 1 | batch |
| `quadtree_node_with_bounds` | no | 12 | 1 | normal 12 | batch |
| `quadtree_point_new` | no | 6 | 0 | normal 6 | batch |
| `quadtree_search` | no | 9 | 0 | normal 9 | batch |

## 3. Tests side

Status **PASS**. bin/test equivalent exits 0 (4 groups); raw/tests_run.log

Mode used for the partition: **measured**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 17 harnesses, 1 crash-all (`get_quadrant_` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 24 | 24 | 20 | 20 | 4 | 0 | 0 | 1.000 | 0.833 |
| regions | 436 | 406 | 220 | 218 | 188 | 2 | 28 | 0.931 | 0.505 |

Sanity checks: function pass, region pass. Harnesses unioned: 16. Identities outside the universe (excluded, never added): 0 fn / 2 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `find_` | 9 | 9 | 9 | 9 | 9 |
| `node_contains_` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_bounds_extend` | 6 | 6 | 6 | 6 | 6 |
| `quadtree_free` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_insert` | 10 | 10 | 10 | 10 | 10 |
| `quadtree_new` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_isempty` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_isleaf` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_ispointer` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_node_with_bounds` | 12 | 12 | 12 | 12 | 12 |
| `quadtree_point_new` | 6 | 6 | 6 | 6 | 6 |
| `quadtree_search` | 9 | 9 | 9 | 9 | 9 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 127 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `find_` | 1 / 1 | not_reproducible 1 | 1 |
| `get_quadrant_` | 3 / 3 | ub_associated_termination 3 | 1 |
| `node_contains_` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_bounds_new` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_new` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_isempty` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_isleaf` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_ispointer` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_new` | 1 / 1 | not_reproducible 1 | 1 |
| `quadtree_node_with_bounds` | 1 / 1 | not_reproducible 1 | 1 |

Total: not_reproducible 9, ub_associated_termination 3

<!-- prose -->
