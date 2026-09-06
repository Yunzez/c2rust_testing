# lil × c2saferrust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 145 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 47 |
| built | 47 |
| executed (corpus > 0) | 47 |
| coverage exported | 24 |

Plan failures, by the generator's own reason:

- **58** × signature: unsupported: pointer-to-pointer-to-struct param argv
- **15** × signature: struct-invariant param ee: expreval_t has pointer field 'code' (needs invariant
- **8** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **5** × signature: struct-invariant param hm: hashmap_t has unsupported field 'cell' (array) (need
- **3** × input buffer of i8 has Rust type &CStr, which is not a raw pointer, a slice, a Vec or a Bo
- **3** × signature: struct-invariant param env: _lil_env_t has pointer field 'parent' (needs invari
- **2** × signature: callback parameter proc deferred: function pointers (callback binding) not yet 
- **1** × signature: struct-invariant param cmd: _lil_func_t has pointer field 'name' (needs invaria
- **1** × signature: struct-invariant param parent: _lil_env_t has pointer field 'parent' (needs inv
- **1** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is
- **1** × void* parameter has Rust type Option<Box<dyn std::any::Any>>; only a raw pointer is lossle

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `add_func` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `alloc_value` | yes | 15 | 0 | normal 15 | batch |
| `alloc_value_len` | yes | 15 | 1 | normal 15 | batch |
| `ateol` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `ee_invalidpunct` | yes | 9 | 0 | normal 6, signal 3 | batch |
| `find_cmd` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `fnc_embed_write` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `get_bracketpart` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `get_dollarpart` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `islilspecial` | yes | 3 | 0 | normal 3 | batch |
| `lil_alloc_double` | no | 11 | 1144 | normal 11 | batch |
| `lil_alloc_integer` | no | 13 | 0 | normal 13 | batch |
| `lil_alloc_list` | no | 1 | 0 | normal 1 | batch |
| `lil_alloc_string` | no | 15 | 0 | normal 15 | batch |
| `lil_append_char` | no | 9 | 0 | normal 9 | batch |
| `lil_append_string` | no | 21 | 0 | normal 21 | batch |
| `lil_append_string_len` | no | 19 | 2479 | normal 18, signal 1 | per-input (18/19 completed) |
| `lil_clone_value` | no | 8 | 0 | normal 8 | batch |
| `lil_embedded` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_free` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_free_list` | no | 1 | 0 | normal 1 | batch |
| `lil_free_value` | no | 8 | 0 | normal 8 | batch |
| `lil_get_data` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_get_var` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_list_get` | no | 8 | 0 | normal 8 | batch |
| `lil_list_size` | no | 1 | 0 | normal 1 | batch |
| `lil_list_to_value` | no | 6 | 0 | normal 6 | batch |
| `lil_new` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_parse` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_pop_env` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_push_env` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_set_data` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_set_error` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_set_error_at` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lil_to_boolean` | no | 13 | 0 | normal 13 | batch |
| `lil_to_double` | no | 8 | 0 | normal 8 | batch |
| `lil_to_integer` | no | 8 | 0 | normal 8 | batch |
| `lil_to_string` | no | 8 | 0 | normal 8 | batch |
| `lil_write` | no | 1 | 2 | signal 1 | failed rc=1 |
| `needs_escape` | yes | 40 | 0 | normal 40 | batch |
| `next_word` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `real_inc` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `real_trim` | yes | 62 | 0 | normal 62 | batch |
| `register_stdcmds` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `skip_spaces` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `strclone` | yes | 15 | 0 | normal 15 | batch |
| `substitute` | yes | 1 | 2 | signal 1 | failed rc=1 |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no shipped suite; denominator: 167 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 154 | 0 | 25 | 0 | 0 | 25 | 129 | 0.000 | 0.162 |
| regions | 5751 | 0 | 362 | 0 | 0 | 362 | 5389 | 0.000 | 0.063 |

Sanity checks: function pass, region pass. Harnesses unioned: 24. Identities outside the universe (excluded, never added): 0 fn / 3 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `alloc_value` | 15 | 15 | 15 | 15 | 15 |
| `alloc_value_len` | 15 | 15 | 15 | 15 | 15 |
| `ee_invalidpunct` | 9 | 9 | 9 | 9 | 9 |
| `islilspecial` | 3 | 3 | 3 | 3 | 3 |
| `lil_alloc_double` | 11 | 11 | 11 | 11 | 11 |
| `lil_alloc_integer` | 13 | 13 | 13 | 13 | 13 |
| `lil_alloc_string` | 15 | 15 | 15 | 15 | 15 |
| `lil_append_char` | 9 | 9 | 9 | 9 | 9 |
| `lil_append_string` | 21 | 21 | 21 | 21 | 21 |
| `lil_append_string_len` | 19 | 19 | 19 | 19 | 19 |
| `lil_clone_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_free_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_list_get` | 8 | 8 | 8 | 8 | 8 |
| `lil_list_to_value` | 6 | 6 | 6 | 6 | 6 |
| `lil_to_boolean` | 13 | 13 | 13 | 13 | 13 |
| `lil_to_double` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_string` | 8 | 8 | 8 | 8 | 8 |
| `needs_escape` | 40 | 40 | 40 | 40 | 40 |
| `real_trim` | 62 | 62 | 62 | 62 | 62 |
| `strclone` | 15 | 15 | 15 | 15 | 15 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 313 |
| signal | 27 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `add_func` | 3 / 3 | confirmed_termination 3 | 1 |
| `alloc_value_len` | 1 / 1 | not_reproducible 1 | 1 |
| `ateol` | 3 / 3 | ub_associated_termination 3 | 1 |
| `ee_invalidpunct` | 3 / 3 | ub_associated 3 | 1 |
| `find_cmd` | 3 / 3 | confirmed_termination 3 | 1 |
| `fnc_embed_write` | 3 / 3 | confirmed_termination 3 | 1 |
| `get_bracketpart` | 3 / 3 | confirmed_termination 3 | 1 |
| `get_dollarpart` | 3 / 3 | ub_associated_termination 3 | 1 |
| `lil_alloc_double` | 200 / 500 | ub_associated 200 | 1 |
| `lil_append_string_len` | 201 / 540 | ub_associated 201 | 3 |
| `lil_embedded` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_free` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_get_data` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_get_var` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_new` | 3 / 3 | instrument_only 3 | 1 |
| `lil_parse` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_pop_env` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_push_env` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_set_data` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_set_error` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_set_error_at` | 3 / 3 | confirmed_termination 3 | 1 |
| `lil_write` | 3 / 3 | confirmed_termination 3 | 1 |
| `next_word` | 3 / 3 | ub_associated_termination 3 | 1 |
| `real_inc` | 3 / 3 | confirmed_termination 3 | 1 |
| `register_stdcmds` | 3 / 3 | confirmed_termination 3 | 1 |
| `skip_spaces` | 3 / 3 | confirmed_termination 3 | 1 |
| `substitute` | 3 / 3 | confirmed_termination 3 | 1 |

Total: confirmed_termination 57, instrument_only 3, not_reproducible 1, ub_associated 404, ub_associated_termination 9

<!-- prose -->
## 7. Deviations, and what this cell is

**One defect, and it blocks the cell.** 47 of 47 harnesses built, but 23 boundaries are crash-all (`campaign_status.crash_all`): every execution crashed and the corpus never grew, so the coverage export for them failed — that is the result, not a collection error. The confirmation sample has **57 `confirmed_termination` across 19 boundaries, all one site**: `lil_new()` → `register_stdcmds` → `lil_register` → `add_func` → `find_cmd` → `hm_get`, where C2SaferRust passes Rust string literals (`"reflect".as_ptr() as *const i8`, `lil_c2saferrust.rs:4725`) as C strings **without a NUL terminator** and `hm_get`'s `CStr::from_ptr(key)` runs `strlen` past the literal (ASan: global-buffer-overflow; without a sanitizer the `NonNull::new_unchecked` precondition panic). C is in contract on every one of these inputs (C-only normal under ASan+UBSan). By the clustering rule this is **one defect — C9 in the manifest** — and the other 18 boundaries are *downstream blocked*: they take the `lil_t` the producer cannot build. It is the root of the CRASH-ALL that the reach census recorded for this translation without a cause. 404 `ub_associated` and 9 `ub_associated_termination` are lil's own out-of-contract reads on the C side; 1 `not_reproducible`; 3 `instrument_only`.

**Coverage:** 25 of 154 functions (0.162), 362 of 5 751 regions (0.063) — the 24 boundaries that do not need an interpreter object (`lil_alloc_*`, `lil_to_*`, the string helpers). Nothing else is reachable while `lil_new()` crashes.

**Deviations:** none in procedure. This cell ran before the preflight step existed (its 23 crash-alls would have been flagged after one minute; they were seen after the hour) and before per-harness generator hashes were recorded. Its post-processing was run once early (2026-09-06 02:58, `nice 10`, concurrently with the CROWN campaign) and again by the chain; the results are identical.

**Not established:** any coverage or behaviour of the interpreter beyond construction; whether other C2SaferRust `*.as_ptr()` C-string sites (lil has several) hide further defects — the first one masks them.
