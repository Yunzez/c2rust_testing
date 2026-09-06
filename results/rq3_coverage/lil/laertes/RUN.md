# lil × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 145 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 51 |
| built | 51 |
| executed (corpus > 0) | 51 |
| coverage exported | 48 |

Plan failures, by the generator's own reason:

- **58** × signature: unsupported: pointer-to-pointer-to-struct param argv
- **15** × signature: struct-invariant param ee: expreval_t has pointer field 'code' (needs invariant
- **8** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **5** × signature: struct-invariant param hm: hashmap_t has unsupported field 'cell' (array) (need
- **3** × signature: struct-invariant param env: _lil_env_t has pointer field 'parent' (needs invari
- **2** × signature: callback parameter proc deferred: function pointers (callback binding) not yet 
- **1** × signature: struct-invariant param cmd: _lil_func_t has pointer field 'name' (needs invaria
- **1** × signature: struct-invariant param parent: _lil_env_t has pointer field 'parent' (needs inv
- **1** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `add_func` | yes | 30 | 0 | normal 30 | batch |
| `alloc_value` | yes | 15 | 0 | normal 15 | batch |
| `alloc_value_len` | yes | 15 | 0 | normal 15 | batch |
| `ateol` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `ee_invalidpunct` | yes | 11 | 0 | normal 8, signal 3 | batch |
| `find_cmd` | yes | 30 | 0 | normal 30 | batch |
| `fnc_embed_write` | yes | 14 | 0 | normal 14 | batch |
| `get_bracketpart` | yes | 1 | 0 | normal 1 | batch |
| `get_dollarpart` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `hm_hash` | yes | 21 | 0 | normal 21 | batch |
| `islilspecial` | yes | 8 | 0 | normal 8 | batch |
| `lil_alloc_double` | no | 11 | 1255 | normal 11 | batch |
| `lil_alloc_integer` | no | 8 | 0 | normal 8 | batch |
| `lil_alloc_list` | no | 1 | 0 | normal 1 | batch |
| `lil_alloc_string` | no | 15 | 0 | normal 15 | batch |
| `lil_alloc_string_len` | no | 15 | 0 | normal 15 | batch |
| `lil_append_char` | no | 9 | 0 | normal 9 | batch |
| `lil_append_string` | no | 21 | 0 | normal 21 | batch |
| `lil_append_string_len` | no | 19 | 2478 | normal 18, signal 1 | per-input (18/19 completed) |
| `lil_clone_value` | no | 8 | 0 | normal 8 | batch |
| `lil_embedded` | no | 1812 | 106 | normal 1811, ub-gated 1 | batch |
| `lil_free` | no | 1 | 0 | normal 1 | batch |
| `lil_free_list` | no | 1 | 0 | normal 1 | batch |
| `lil_free_value` | no | 8 | 0 | normal 8 | batch |
| `lil_freemem` | no | 1 | 0 | normal 1 | batch |
| `lil_get_data` | no | 1 | 0 | normal 1 | batch |
| `lil_get_var` | no | 29 | 0 | normal 29 | batch |
| `lil_list_get` | no | 8 | 0 | normal 8 | batch |
| `lil_list_size` | no | 1 | 0 | normal 1 | batch |
| `lil_list_to_value` | no | 6 | 0 | normal 6 | batch |
| `lil_new` | no | 1 | 57 | normal 1 | batch |
| `lil_parse` | no | 2517 | 1210 | normal 2517 | batch |
| `lil_pop_env` | no | 1 | 0 | normal 1 | batch |
| `lil_push_env` | no | 1 | 0 | normal 1 | batch |
| `lil_set_data` | no | 1 | 0 | normal 1 | batch |
| `lil_set_error` | no | 15 | 0 | normal 15 | batch |
| `lil_set_error_at` | no | 22 | 0 | normal 22 | batch |
| `lil_to_boolean` | no | 14 | 0 | normal 14 | batch |
| `lil_to_double` | no | 8 | 0 | normal 8 | batch |
| `lil_to_integer` | no | 8 | 0 | normal 8 | batch |
| `lil_to_string` | no | 8 | 0 | normal 8 | batch |
| `lil_unused_name` | no | 27 | 0 | normal 27 | batch |
| `lil_write` | no | 14 | 0 | normal 14 | batch |
| `needs_escape` | yes | 27 | 0 | normal 27 | batch |
| `next_word` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `real_inc` | yes | 42 | 0 | normal 36, ub-gated 6 | batch |
| `real_trim` | yes | 60 | 0 | normal 60 | batch |
| `register_stdcmds` | yes | 1 | 0 | normal 1 | batch |
| `skip_spaces` | yes | 1 | 0 | normal 1 | batch |
| `strclone` | yes | 15 | 0 | normal 15 | batch |
| `substitute` | yes | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no shipped suite; denominator: 281 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`, `unrecorded` — more than one: see deviations.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 183 | 0 | 144 | 0 | 0 | 144 | 39 | 0.000 | 0.787 |
| regions | 6143 | 0 | 5028 | 0 | 0 | 5028 | 1115 | 0.000 | 0.818 |

Sanity checks: function pass, region pass. Harnesses unioned: 48. Identities outside the universe (excluded, never added): 0 fn / 7 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `add_func` | 29 | 29 | 30 | 30 | 30 |
| `alloc_value` | 15 | 15 | 15 | 15 | 15 |
| `alloc_value_len` | 15 | 15 | 15 | 15 | 15 |
| `ee_invalidpunct` | 11 | 11 | 11 | 11 | 11 |
| `find_cmd` | 28 | 29 | 30 | 30 | 30 |
| `fnc_embed_write` | 12 | 14 | 14 | 14 | 14 |
| `hm_hash` | 21 | 21 | 21 | 21 | 21 |
| `islilspecial` | 8 | 8 | 8 | 8 | 8 |
| `lil_alloc_double` | 11 | 11 | 11 | 11 | 11 |
| `lil_alloc_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_alloc_string` | 15 | 15 | 15 | 15 | 15 |
| `lil_alloc_string_len` | 15 | 15 | 15 | 15 | 15 |
| `lil_append_char` | 9 | 9 | 9 | 9 | 9 |
| `lil_append_string` | 21 | 21 | 21 | 21 | 21 |
| `lil_append_string_len` | 19 | 19 | 19 | 19 | 19 |
| `lil_clone_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_embedded` | 69 | 431 | 685 | 1315 | 1812 |
| `lil_free_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_get_var` | 27 | 28 | 29 | 29 | 29 |
| `lil_list_get` | 8 | 8 | 8 | 8 | 8 |
| `lil_list_to_value` | 6 | 6 | 6 | 6 | 6 |
| `lil_parse` | 1 | 1 | 1 | 1 | 2517 |
| `lil_set_error` | 13 | 14 | 15 | 15 | 15 |
| `lil_set_error_at` | 20 | 22 | 22 | 22 | 22 |
| `lil_to_boolean` | 14 | 14 | 14 | 14 | 14 |
| `lil_to_double` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_string` | 8 | 8 | 8 | 8 | 8 |
| `lil_unused_name` | 25 | 26 | 27 | 27 | 27 |
| `lil_write` | 12 | 14 | 14 | 14 | 14 |
| `needs_escape` | 27 | 27 | 27 | 27 | 27 |
| `real_inc` | 40 | 41 | 42 | 42 | 42 |
| `real_trim` | 60 | 60 | 60 | 60 | 60 |
| `strclone` | 15 | 15 | 15 | 15 | 15 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 4893 |
| signal | 7 |
| ub-gated | 7 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ateol` | 3 / 3 | ub_associated_termination 3 | 1 |
| `ee_invalidpunct` | 3 / 3 | ub_associated 3 | 1 |
| `get_dollarpart` | 3 / 3 | ub_associated_termination 3 | 1 |
| `lil_alloc_double` | 200 / 500 | ub_associated 200 | 1 |
| `lil_append_string_len` | 201 / 540 | ub_associated 201 | 3 |
| `lil_embedded` | 106 / 106 | inconclusive 5, instrument_only 33, ub_associated 7, ub_associated_termination 61 | 12 |
| `lil_new` | 57 / 57 | not_reproducible 57 | 1 |
| `lil_parse` | 240 / 557 | inconclusive 40, instrument_only 19, ub_associated 164, ub_associated_termination 17 | 12 |
| `next_word` | 3 / 3 | ub_associated_termination 3 | 1 |

Total: inconclusive 45, instrument_only 52, not_reproducible 57, ub_associated 575, ub_associated_termination 87

<!-- prose -->
## 7. Deviations, and what this cell is

**Second negative control.** Laertes rewrites lil for safety but keeps its semantics: 51 of 51 harnesses built, 4 907 corpus inputs, 7 terminations and 7 `ub-gated` in the combined replay, no value divergence, and nothing confirmed in the sample (87 `ub_associated_termination`, 575 `ub_associated`, 52 `instrument_only`, 45 `inconclusive`, 57 `not_reproducible`). Coverage 144 of 183 functions (0.787), 5 028 of 6 143 regions (0.818). The universe is larger than c2rust's (183 vs 151 functions) because Laertes' crate carries its own support code (`laertes_init_*` initialisers, the `__laertes_array` runtime) which is in scope and which no boundary can reach — the fraction is lower for that reason, not because less of lil was exercised (the only-ours counts, 144 and 5 028, are the comparable numbers).

**Deviation:** `lil_parse` was re-fuzzed alone and merged, for the same harness bug and in the same way as the c2rust cell (`deviations.json`; corpus 2 517, 1 210 artifacts, 3 600 s alone). The three parser internals (`ateol`, `get_dollarpart`, `next_word`) are crash-all for the same unconstructible-precondition reason and are accepted, not promoted.

**Not established:** any coverage for the three crash-all internals; the exact contention of the `lil_parse` re-run.
