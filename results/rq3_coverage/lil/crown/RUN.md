# lil × crown — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 145 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 42 |
| built | 42 |
| executed (corpus > 0) | 42 |
| coverage exported | 39 |

Plan failures, by the generator's own reason:

- **58** × signature: unsupported: pointer-to-pointer-to-struct param argv
- **15** × signature: struct-invariant param ee: expreval_t has pointer field 'code' (needs invariant
- **8** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **3** × signature: struct-invariant param env: _lil_env_t has pointer field 'parent' (needs invari
- **2** × signature: callback parameter proc deferred: function pointers (callback binding) not yet 
- **1** × signature: alloc_value_len is not present in the Rust translation (no boundary)
- **1** × signature: del_func is not present in the Rust translation (no boundary)
- **1** × signature: ee_invalidpunct is not present in the Rust translation (no boundary)
- **1** × signature: fnc_embed_write is not present in the Rust translation (no boundary)
- **1** × signature: hm_destroy is not present in the Rust translation (no boundary)
- **1** × signature: hm_get is not present in the Rust translation (no boundary)
- **1** × signature: hm_has is not present in the Rust translation (no boundary)
- **1** × signature: hm_hash is not present in the Rust translation (no boundary)
- **1** × signature: hm_init is not present in the Rust translation (no boundary)
- **1** × signature: hm_put is not present in the Rust translation (no boundary)
- **1** × signature: struct-invariant param parent: _lil_env_t has pointer field 'parent' (needs inv
- **1** × signature: lil_alloc_string_len is not present in the Rust translation (no boundary)
- **1** × signature: lil_append_string_len is not present in the Rust translation (no boundary)
- **1** × signature: lil_embedded is not present in the Rust translation (no boundary)
- **1** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is
- **1** × signature: lil_freemem is not present in the Rust translation (no boundary)
- **1** × signature: lil_write is not present in the Rust translation (no boundary)

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `add_func` | yes | 16 | 0 | normal 16 | batch |
| `alloc_value` | yes | 16 | 0 | normal 16 | batch |
| `ateol` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `find_cmd` | yes | 14 | 0 | normal 14 | batch |
| `get_bracketpart` | yes | 1 | 0 | normal 1 | batch |
| `get_dollarpart` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `islilspecial` | yes | 8 | 0 | normal 8 | batch |
| `lil_alloc_double` | no | 10 | 137 | normal 10 | batch |
| `lil_alloc_integer` | no | 8 | 0 | normal 8 | batch |
| `lil_alloc_list` | no | 1 | 0 | normal 1 | batch |
| `lil_alloc_string` | no | 16 | 0 | normal 16 | batch |
| `lil_append_char` | no | 9 | 0 | normal 9 | batch |
| `lil_append_string` | no | 21 | 0 | normal 21 | batch |
| `lil_clone_value` | no | 8 | 0 | normal 8 | batch |
| `lil_free` | no | 1 | 0 | normal 1 | batch |
| `lil_free_list` | no | 1 | 0 | normal 1 | batch |
| `lil_free_value` | no | 8 | 0 | normal 8 | batch |
| `lil_get_data` | no | 1 | 0 | normal 1 | batch |
| `lil_get_var` | no | 15 | 0 | normal 15 | batch |
| `lil_list_get` | no | 8 | 0 | normal 8 | batch |
| `lil_list_size` | no | 1 | 0 | normal 1 | batch |
| `lil_list_to_value` | no | 5 | 0 | normal 5 | batch |
| `lil_new` | no | 1 | 1 | normal 1 | batch |
| `lil_parse` | no | 1466 | 473 | normal 1466 | batch |
| `lil_pop_env` | no | 1 | 0 | normal 1 | batch |
| `lil_push_env` | no | 1 | 0 | normal 1 | batch |
| `lil_set_data` | no | 1 | 0 | normal 1 | batch |
| `lil_set_error` | no | 16 | 0 | normal 16 | batch |
| `lil_set_error_at` | no | 22 | 0 | normal 22 | batch |
| `lil_to_boolean` | no | 14 | 0 | normal 14 | batch |
| `lil_to_double` | no | 8 | 0 | normal 8 | batch |
| `lil_to_integer` | no | 8 | 0 | normal 8 | batch |
| `lil_to_string` | no | 8 | 0 | normal 8 | batch |
| `lil_unused_name` | no | 16 | 0 | normal 16 | batch |
| `needs_escape` | yes | 22 | 0 | normal 22 | batch |
| `next_word` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `real_inc` | yes | 21 | 0 | normal 20, ub-gated 1 | batch |
| `real_trim` | yes | 64 | 0 | normal 64 | batch |
| `register_stdcmds` | yes | 1 | 0 | normal 1 | batch |
| `skip_spaces` | yes | 1 | 0 | normal 1 | batch |
| `strclone` | yes | 16 | 0 | normal 16 | batch |
| `substitute` | yes | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no shipped suite; denominator: 143 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 134 | 0 | 127 | 0 | 0 | 127 | 7 | 0.000 | 0.948 |
| regions | 6409 | 0 | 5294 | 0 | 0 | 5294 | 1115 | 0.000 | 0.826 |

Sanity checks: function pass, region pass. Harnesses unioned: 39. Identities outside the universe (excluded, never added): 0 fn / 8 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `add_func` | 14 | 16 | 16 | 16 | 16 |
| `alloc_value` | 16 | 16 | 16 | 16 | 16 |
| `find_cmd` | 12 | 14 | 14 | 14 | 14 |
| `islilspecial` | 8 | 8 | 8 | 8 | 8 |
| `lil_alloc_double` | 10 | 10 | 10 | 10 | 10 |
| `lil_alloc_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_alloc_string` | 16 | 16 | 16 | 16 | 16 |
| `lil_append_char` | 9 | 9 | 9 | 9 | 9 |
| `lil_append_string` | 21 | 21 | 21 | 21 | 21 |
| `lil_clone_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_free_value` | 8 | 8 | 8 | 8 | 8 |
| `lil_get_var` | 13 | 15 | 15 | 15 | 15 |
| `lil_list_get` | 8 | 8 | 8 | 8 | 8 |
| `lil_list_to_value` | 5 | 5 | 5 | 5 | 5 |
| `lil_parse` | 397 | 658 | 837 | 1211 | 1466 |
| `lil_set_error` | 14 | 16 | 16 | 16 | 16 |
| `lil_set_error_at` | 20 | 21 | 22 | 22 | 22 |
| `lil_to_boolean` | 14 | 14 | 14 | 14 | 14 |
| `lil_to_double` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_integer` | 8 | 8 | 8 | 8 | 8 |
| `lil_to_string` | 8 | 8 | 8 | 8 | 8 |
| `lil_unused_name` | 14 | 16 | 16 | 16 | 16 |
| `needs_escape` | 22 | 22 | 22 | 22 | 22 |
| `real_inc` | 19 | 21 | 21 | 21 | 21 |
| `real_trim` | 63 | 64 | 64 | 64 | 64 |
| `strclone` | 16 | 16 | 16 | 16 | 16 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 1855 |
| signal | 3 |
| ub-gated | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `ateol` | 3 / 3 | ub_associated_termination 3 | 1 |
| `get_dollarpart` | 3 / 3 | ub_associated_termination 3 | 1 |
| `lil_alloc_double` | 137 / 137 | ub_associated 137 | 1 |
| `lil_new` | 1 / 1 | not_reproducible 1 | 1 |
| `lil_parse` | 244 / 473 | confirmed_termination 69, inconclusive 42, instrument_only 61, ub_associated 55, ub_associated_termination 17 | 12 |
| `next_word` | 3 / 3 | ub_associated_termination 3 | 1 |

Total: confirmed_termination 69, inconclusive 42, instrument_only 61, not_reproducible 1, ub_associated 192, ub_associated_termination 26

<!-- prose -->
## 7. Deviations, and what this cell is

**A defect on a certified cell.** CROWN's lil was an E1 *certificate* (111 043-record expr/variable/list/string corpus, 0 diffs). Here 42 of 42 harnesses built, the preflight flagged nothing (the three parser internals are accepted crash-alls), 1 859 corpus inputs, 3 terminations and 1 `ub-gated` on replay — and the confirmation sample on `lil_parse` holds **67 `confirmed_termination`**, all one site: `lil_subst_to_list` **dropped** C's `if (!words) words = lil_alloc_list();` (lil.c:920; `lil_crown.rs:983-988` returns `substitute()`'s result unguarded), so when substitution fails on a malformed argument (the confirmed scripts begin `enveval '(`…) it returns NULL where C returns an empty list, and `fnc_enveval` dereferences it in `lil_list_size` (`lil.rs:404`, "null pointer dereference occurred" without a sanitizer, SEGV with one). C-only is normal under ASan+UBSan on every one. **C10 in the manifest.** The E1 corpus never called `enveval` with a malformed list; the plan-generated `lil_parse` harness found it in the first hour. Two further `lil_parse` samples are timeouts in the no-sanitizer run, not panics, and are not counted. 42 `inconclusive`, 26 `ub_associated_termination`, 192 `ub_associated`, 61 `instrument_only`, 1 `not_reproducible`.

**Coverage:** 127 of 134 functions (0.948), 5 294 of 6 409 regions (0.826).

**Deviation:** the cell was started twice. The first start built 27 of 42 harnesses — the 15 C `static` entries failed with `unresolved import crate::lil`, because CROWN wraps its modules in `pub mod src { .. }` and the funnel's `--expose-entry` re-export used the unprefixed path — and its campaign was killed after 20 minutes; nothing from it was kept. The fix (the re-export takes the prefix from the flatten's own `pub use` lines) is generic; the cell then ran in full with the preflight. Generator hash `a8925ba6292dd9ac` for every harness.

**Not established:** whether the same dropped-fallback pattern exists at CROWN's other `substitute()` callers (`lil_subst_to_value`, `lil_parse_value`) — they were not reached by a failing substitution in the sample.
