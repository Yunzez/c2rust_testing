# quadtree × ptrtrans — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 24 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 11 |
| built | 11 |
| executed (corpus > 0) | 11 |
| coverage exported | 11 |

Plan failures, by the generator's own reason:

- **2** × struct parameter has Rust type Option<&QuadtreePoint>
- **2** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **2** × signature: callback parameter key_free deferred: function pointers (callback binding) not 
- **1** × signature: elision_ is not present in the Rust translation (no boundary)
- **1** × signature: quadtree_bounds_free is not present in the Rust translation (no boundary)
- **1** × signature: quadtree_free is not present in the Rust translation (no boundary)
- **1** × Rust signature has 5 parameters, C has 4: reshaped API, no positional bridge
- **1** × signature: quadtree_point_free is not present in the Rust translation (no boundary)
- **1** × signature: callback parameter descent deferred: function pointers (callback binding) not y
- **1** × signature: reset_node_ is not present in the Rust translation (no boundary)

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `find_` | no | 9 | 0 | signal 9 | batch |
| `quadtree_bounds_extend` | no | 6 | 0 | normal 6 | batch |
| `quadtree_bounds_new` | no | 1 | 0 | normal 1 | batch |
| `quadtree_new` | no | 11 | 0 | normal 11 | batch |
| `quadtree_node_isempty` | no | 11 | 0 | normal 11 | batch |
| `quadtree_node_isleaf` | no | 11 | 0 | normal 11 | batch |
| `quadtree_node_ispointer` | no | 11 | 0 | normal 11 | batch |
| `quadtree_node_new` | no | 1 | 0 | normal 1 | batch |
| `quadtree_node_with_bounds` | no | 11 | 0 | normal 11 | batch |
| `quadtree_point_new` | no | 6 | 0 | normal 6 | batch |
| `quadtree_search` | no | 9 | 0 | signal 9 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no transpiled driver in the shipped crate; denominator: 19 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 11 harnesses, 2 crash-all (`find_` accepted, `quadtree_search` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 19 | 0 | 13 | 0 | 0 | 13 | 6 | 0.000 | 0.684 |
| regions | 407 | 0 | 134 | 0 | 0 | 134 | 273 | 0.000 | 0.329 |

Sanity checks: function pass, region pass. Harnesses unioned: 11. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `find_` | 9 | 9 | 9 | 9 | 9 |
| `quadtree_bounds_extend` | 6 | 6 | 6 | 6 | 6 |
| `quadtree_new` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_node_isempty` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_node_isleaf` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_node_ispointer` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_node_with_bounds` | 11 | 11 | 11 | 11 | 11 |
| `quadtree_point_new` | 6 | 6 | 6 | 6 | 6 |
| `quadtree_search` | 9 | 9 | 9 | 9 | 9 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 69 |
| signal | 18 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `find_` | 9 / 9 | ub_associated 9 | 1 |
| `quadtree_search` | 9 / 9 | ub_associated 9 | 1 |

Total: ub_associated 18

<!-- prose -->
