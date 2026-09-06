# lil × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 145 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 51 |
| built | 50 |
| executed (corpus > 0) | 50 |
| coverage exported | 9 |

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

Planned but not built:

- `lil_list_size`: no binary kept

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `add_func` | yes | 30 | 0 | normal 30 | kept |
| `alloc_value` | yes | 15 | 0 | normal 15 | kept |
| `alloc_value_len` | yes | 15 | 0 | normal 15 | kept |
| `ateol` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `ee_invalidpunct` | yes | 11 | 0 | normal 8, signal 3 | kept |
| `find_cmd` | yes | 30 | 0 | normal 30 | kept |
| `fnc_embed_write` | yes | 14 | 0 | normal 14 | kept |
| `get_bracketpart` | yes | 1 | 0 | normal 1 | kept |
| `get_dollarpart` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `hm_hash` | yes | 21 | 0 | normal 21 | kept |
| `islilspecial` | yes | 8 | 0 | normal 8 | kept |
| `lil_alloc_double` | no | 11 | 500 | normal 11 | kept |
| `lil_alloc_integer` | no | 8 | 0 | normal 8 | kept |
| `lil_alloc_list` | no | 1 | 0 | normal 1 | kept |
| `lil_alloc_string` | no | 15 | 0 | normal 15 | kept |
| `lil_alloc_string_len` | no | 15 | 0 | normal 15 | kept |
| `lil_append_char` | no | 9 | 0 | normal 9 | kept |
| `lil_append_string` | no | 21 | 0 | normal 21 | kept |
| `lil_append_string_len` | no | 19 | 539 | normal 18, signal 1 | kept |
| `lil_clone_value` | no | 8 | 0 | normal 8 | kept |
| `lil_embedded` | no | 1908 | 102 | normal 1907, ub-gated 1 | kept |
| `lil_free` | no | 1 | 0 | normal 1 | kept |
| `lil_free_list` | no | 1 | 0 | normal 1 | kept |
| `lil_free_value` | no | 8 | 0 | normal 8 | kept |
| `lil_freemem` | no | 1 | 0 | normal 1 | kept |
| `lil_get_data` | no | 1 | 0 | normal 1 | kept |
| `lil_get_var` | no | 28 | 0 | normal 28 | kept |
| `lil_list_get` | no | 8 | 0 | normal 8 | kept |
| `lil_list_to_value` | no | 6 | 0 | normal 6 | kept |
| `lil_new` | no | 1 | 54 | normal 1 | kept |
| `lil_parse` | no | 2279 | 502 | normal 2278, ub-gated 1 | batch |
| `lil_pop_env` | no | 1 | 0 | normal 1 | kept |
| `lil_push_env` | no | 1 | 0 | normal 1 | kept |
| `lil_set_data` | no | 1 | 0 | normal 1 | kept |
| `lil_set_error` | no | 14 | 0 | normal 14 | kept |
| `lil_set_error_at` | no | 22 | 0 | normal 22 | kept |
| `lil_to_boolean` | no | 14 | 0 | normal 14 | kept |
| `lil_to_double` | no | 8 | 0 | normal 8 | kept |
| `lil_to_integer` | no | 8 | 0 | normal 8 | kept |
| `lil_to_string` | no | 8 | 0 | normal 8 | kept |
| `lil_unused_name` | no | 26 | 0 | normal 26 | kept |
| `lil_write` | no | 15 | 0 | normal 15 | batch |
| `needs_escape` | yes | 26 | 0 | normal 26 | batch |
| `next_word` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `real_inc` | yes | 42 | 0 | normal 39, ub-gated 3 | batch |
| `real_trim` | yes | 60 | 0 | normal 60 | batch |
| `register_stdcmds` | yes | 1 | 0 | normal 1 | batch |
| `skip_spaces` | yes | 1 | 0 | normal 1 | batch |
| `strclone` | yes | 15 | 0 | normal 15 | batch |
| `substitute` | yes | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no shipped suite; denominator: 161 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`, `unrecorded` — more than one: see deviations.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 151 | 0 | 143 | 0 | 0 | 143 | 8 | 0.000 | 0.947 |
| regions | 5730 | 0 | 4999 | 0 | 0 | 4999 | 731 | 0.000 | 0.872 |

Sanity checks: function pass, region pass. Harnesses unioned: 47. Identities outside the universe (excluded, never added): 0 fn / 7 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `add_func` | 28 | 29 | 30 | 30 | 30 |
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
| `lil_embedded` | 67 | 452 | 651 | 1424 | 1908 |
| `lil_free_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_get_var` | 26 | 27 | 28 | 28 | 28 |
| `lil_list_get` | 8 | 8 | 8 | 8 | 8 |
| `lil_list_to_value` | 6 | 6 | 6 | 6 | 6 |
| `lil_parse` | 1 | 1 | 1 | 1 | 2279 |
| `lil_set_error` | 12 | 13 | 14 | 14 | 14 |
| `lil_set_error_at` | 20 | 20 | 22 | 22 | 22 |
| `lil_to_boolean` | 14 | 14 | 14 | 14 | 14 |
| `lil_to_double` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_string` | 8 | 8 | 8 | 8 | 8 |
| `lil_unused_name` | 24 | 25 | 26 | 26 | 26 |
| `lil_write` | 13 | 14 | 15 | 15 | 15 |
| `needs_escape` | 26 | 26 | 26 | 26 | 26 |
| `real_inc` | 40 | 41 | 42 | 42 | 42 |
| `real_trim` | 58 | 60 | 60 | 60 | 60 |
| `strclone` | 15 | 15 | 15 | 15 | 15 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 4749 |
| signal | 7 |
| ub-gated | 5 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ateol` | 3 / 3 | ub_associated_termination 3 | 1 |
| `ee_invalidpunct` | 3 / 3 | ub_associated 3 | 1 |
| `get_dollarpart` | 3 / 3 | ub_associated_termination 3 | 1 |
| `lil_alloc_double` | 200 / 500 | ub_associated 200 | 1 |
| `lil_append_string_len` | 201 / 540 | ub_associated 201 | 3 |
| `lil_embedded` | 102 / 102 | inconclusive 13, instrument_only 25, ub_associated 22, ub_associated_termination 42 | 12 |
| `lil_new` | 54 / 54 | not_reproducible 54 | 1 |
| `lil_parse` | 247 / 502 | inconclusive 47, instrument_only 28, not_reproducible 2, ub_associated 144, ub_associated_termination 26 | 12 |
| `next_word` | 3 / 3 | ub_associated_termination 3 | 1 |

Total: inconclusive 60, instrument_only 53, not_reproducible 56, ub_associated 570, ub_associated_termination 77

<!-- prose -->
## 7. Deviations, and what this cell is

**Negative control, and it holds.** c2rust's lil is the faithful translation; on 4 761 corpus inputs the combined replay shows 7 terminations and 5 `ub-gated` inputs and no value divergence, and the 200-per-channel confirmation sample confirms nothing: 570 `ub_associated`, 77 `ub_associated_termination` (lil's own out-of-contract reads under ASan on the C side), 53 `instrument_only`, 60 `inconclusive`, 56 `not_reproducible`. Coverage: 143 of 151 functions (0.947), 4 999 of 5 730 regions (0.872), against the link-dead-code universe (lil ships no suite: TEST-UNAVAILABLE).

**Three deviations, in order.**
1. *`lil_list_size` did not build* (50 of 51): the generator compared a `size_t` return without resolving c2rust's `u64` typedef (E0308). Fixed after this cell's campaign had started; the boundary is unbuilt here and builds in the three later cells.
2. *`lil_parse` was re-fuzzed alone and merged* (`deviations.json`). In the cell's campaign every execution of `lil_parse` SEGV'd in the C side's `strlen`: the harness handed a length-0 code buffer whose pointer was dangling (an empty `Vec`'s `as_ptr()`), and lil treats `codelen == 0` as "use strlen(code)". A harness bug, not a finding — 70 948 fork jobs crashed and the corpus stayed at one input. After the generator fix (a sentinel byte past `len` on every length-carrying buffer) `lil_parse` ran alone for 3 600 s under the same libFuzzer parameters: corpus 2 279, 502 artifacts. Unlike the cell's campaign it had the machine to itself; the merged row and this note record that. The generator hash of the re-run differs from the cell's (`unrecorded` — hashes were introduced after this cell).
3. *The coverage phase died and was finished* (`finish_cell.py`): `cargo fuzz coverage` echoed a non-UTF-8 corpus file name and the collector decoded it strictly. 39 boundaries kept their exports, 8 were recollected, 3 could not be — `ateol`, `get_dollarpart`, `next_word`, parser internals that read `lil->code[lil->head]` on a freshly constructed interpreter (`lil->code` is NULL) and crash on every input including the empty one. They are listed in the pair's `preflight_accept.txt` as unconstructible preconditions, never promoted.

**Not established:** anything about `lil_list_size`; any coverage for the three crash-all internals; the exact contention under which `lil_parse`'s corpus was grown.
