# Same-corpus C reach — lil_c2saferrust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `lil.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 62 / 145 (0.428) | 457 / 2249 (0.203) |
| Rust (archived campaign) | 25 / 154 (0.162) | 362 / 5751 (0.063) |

Inputs replayed on the C side: {'completed': 334, 'crash': 6} over 47 / 47 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/lil__c2saferrust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 119 (out of scope: 1).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 24 | 37 (37) | 0 (0) | 58 | 25 | 1 (0) | 6 (0) |

`c_only`: hm_hash→hm_hash, hm_init→hm_init, hm_destroy→hm_destroy, hm_put→hm_put, hm_get→hm_get, lil_append_val→lil_append_val, lil_list_append→lil_list_append, lil_alloc_env→lil_alloc_env, lil_free_env→lil_free_env, lil_find_local_var→lil_find_local_var, lil_find_var→lil_find_var, find_cmd→find_cmd, add_func→add_func, lil_register→lil_register, lil_set_var→lil_set_var, lil_get_var→lil_get_var, lil_get_var_or→lil_get_var_or, lil_push_env→lil_push_env, lil_pop_env→lil_pop_env, lil_new→lil_new, ateol→ateol, skip_spaces→skip_spaces, get_bracketpart→get_bracketpart, next_word→next_word, substitute→substitute, lil_parse→lil_parse, lil_parse_value→lil_parse_value, lil_set_error→lil_set_error, lil_set_error_at→lil_set_error_at, lil_free→lil_free, lil_set_data→lil_set_data, lil_get_data→lil_get_data, lil_embedded→lil_embedded, lil_write→lil_write, fnc_write→fnc_write, real_inc→real_inc, register_stdcmds→register_stdcmds

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| add_func | ok | 1 | {'completed': 1} | 17/145 | 70/2249 |
| alloc_value | ok | 15 | {'completed': 15} | 2/145 | 11/2249 |
| alloc_value_len | ok | 15 | {'completed': 15} | 1/145 | 8/2249 |
| ateol | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| ee_invalidpunct | ok | 9 | {'completed': 6, 'crash': 3} | 1/145 | 12/2249 |
| find_cmd | ok | 1 | {'completed': 1} | 17/145 | 70/2249 |
| fnc_embed_write | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| get_bracketpart | ok | 1 | {'completed': 1} | 19/145 | 80/2249 |
| get_dollarpart | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| islilspecial | ok | 3 | {'completed': 3} | 1/145 | 15/2249 |
| lil_alloc_double | ok | 11 | {'completed': 11} | 3/145 | 12/2249 |
| lil_alloc_integer | ok | 13 | {'completed': 13} | 3/145 | 13/2249 |
| lil_alloc_list | ok | 1 | {'completed': 1} | 1/145 | 2/2249 |
| lil_alloc_string | ok | 15 | {'completed': 15} | 3/145 | 12/2249 |
| lil_append_char | ok | 9 | {'completed': 9} | 5/145 | 18/2249 |
| lil_append_string | ok | 21 | {'completed': 21} | 6/145 | 24/2249 |
| lil_append_string_len | ok | 19 | {'completed': 19} | 5/145 | 24/2249 |
| lil_clone_value | ok | 8 | {'completed': 8} | 5/145 | 25/2249 |
| lil_embedded | ok | 1 | {'completed': 1} | 34/145 | 281/2249 |
| lil_free | ok | 1 | {'completed': 1} | 17/145 | 70/2249 |
| lil_free_list | ok | 1 | {'completed': 1} | 2/145 | 6/2249 |
| lil_free_value | ok | 8 | {'completed': 8} | 4/145 | 15/2249 |
| lil_get_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_get_var | ok | 1 | {'completed': 1} | 21/145 | 82/2249 |
| lil_list_get | ok | 8 | {'completed': 8} | 3/145 | 9/2249 |
| lil_list_size | ok | 1 | {'completed': 1} | 3/145 | 7/2249 |
| lil_list_to_value | ok | 6 | {'completed': 6} | 5/145 | 19/2249 |
| lil_new | ok | 1 | {'completed': 1} | 13/145 | 45/2249 |
| lil_parse | ok | 1 | {'completed': 1} | 29/145 | 231/2249 |
| lil_pop_env | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| lil_push_env | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_error | ok | 1 | {'completed': 1} | 18/145 | 76/2249 |
| lil_set_error_at | ok | 1 | {'completed': 1} | 18/145 | 76/2249 |
| lil_to_boolean | ok | 13 | {'completed': 13} | 6/145 | 36/2249 |
| lil_to_double | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_integer | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_string | ok | 8 | {'completed': 8} | 5/145 | 20/2249 |
| lil_write | ok | 1 | {'completed': 1} | 18/145 | 73/2249 |
| needs_escape | ok | 40 | {'completed': 40} | 1/145 | 14/2249 |
| next_word | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| real_inc | ok | 1 | {'completed': 1} | 27/145 | 133/2249 |
| real_trim | ok | 62 | {'completed': 62} | 5/145 | 37/2249 |
| register_stdcmds | ok | 1 | {'completed': 1} | 17/145 | 76/2249 |
| skip_spaces | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| strclone | ok | 15 | {'completed': 15} | 1/145 | 3/2249 |
| substitute | ok | 1 | {'completed': 1} | 20/145 | 79/2249 |

## Procedure, deviations, and what is not established

<!-- prose -->
