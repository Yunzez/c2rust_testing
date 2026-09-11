# Same-corpus C reach — lil_laertes

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `lil.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 140 / 145 (0.966) | 1901 / 2249 (0.845) |
| Rust (archived campaign) | 144 / 183 (0.787) | 5028 / 6143 (0.818) |

Inputs replayed on the C side: {'completed': 4902, 'crash': 5} over 51 / 51 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/lil__laertes/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 122 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 120 | 0 (0) | 0 (0) | 2 | 23 | 0 (0) | 10 (6) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| add_func | ok | 30 | {'completed': 30} | 17/145 | 76/2249 |
| alloc_value | ok | 15 | {'completed': 15} | 2/145 | 11/2249 |
| alloc_value_len | ok | 15 | {'completed': 15} | 1/145 | 8/2249 |
| ateol | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| ee_invalidpunct | ok | 11 | {'completed': 9, 'crash': 2} | 1/145 | 13/2249 |
| find_cmd | ok | 30 | {'completed': 30} | 17/145 | 71/2249 |
| fnc_embed_write | ok | 14 | {'completed': 14} | 18/145 | 71/2249 |
| get_bracketpart | ok | 1 | {'completed': 1} | 19/145 | 80/2249 |
| get_dollarpart | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| hm_hash | ok | 21 | {'completed': 21} | 1/145 | 3/2249 |
| islilspecial | ok | 8 | {'completed': 8} | 1/145 | 15/2249 |
| lil_alloc_double | ok | 11 | {'completed': 11} | 3/145 | 12/2249 |
| lil_alloc_integer | ok | 8 | {'completed': 8} | 3/145 | 13/2249 |
| lil_alloc_list | ok | 1 | {'completed': 1} | 1/145 | 2/2249 |
| lil_alloc_string | ok | 15 | {'completed': 15} | 3/145 | 12/2249 |
| lil_alloc_string_len | ok | 15 | {'completed': 15} | 2/145 | 9/2249 |
| lil_append_char | ok | 9 | {'completed': 9} | 5/145 | 18/2249 |
| lil_append_string | ok | 21 | {'completed': 21} | 6/145 | 24/2249 |
| lil_append_string_len | ok | 19 | {'completed': 19} | 5/145 | 24/2249 |
| lil_clone_value | ok | 8 | {'completed': 8} | 5/145 | 25/2249 |
| lil_embedded | ok | 1812 | {'completed': 1812} | 129/145 | 1682/2249 |
| lil_free | ok | 1 | {'completed': 1} | 17/145 | 70/2249 |
| lil_free_list | ok | 1 | {'completed': 1} | 2/145 | 6/2249 |
| lil_free_value | ok | 8 | {'completed': 8} | 4/145 | 15/2249 |
| lil_freemem | ok | 1 | {'completed': 1} | 1/145 | 1/2249 |
| lil_get_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_get_var | ok | 29 | {'completed': 29} | 21/145 | 82/2249 |
| lil_list_get | ok | 8 | {'completed': 8} | 3/145 | 9/2249 |
| lil_list_size | ok | 1 | {'completed': 1} | 3/145 | 7/2249 |
| lil_list_to_value | ok | 6 | {'completed': 6} | 5/145 | 19/2249 |
| lil_new | ok | 1 | {'completed': 1} | 13/145 | 45/2249 |
| lil_parse | ok | 2517 | {'completed': 2517} | 132/145 | 1790/2249 |
| lil_pop_env | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| lil_push_env | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_error | ok | 15 | {'completed': 15} | 18/145 | 76/2249 |
| lil_set_error_at | ok | 22 | {'completed': 22} | 18/145 | 76/2249 |
| lil_to_boolean | ok | 14 | {'completed': 14} | 6/145 | 36/2249 |
| lil_to_double | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_integer | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_string | ok | 8 | {'completed': 8} | 5/145 | 20/2249 |
| lil_unused_name | ok | 27 | {'completed': 27} | 21/145 | 88/2249 |
| lil_write | ok | 14 | {'completed': 14} | 18/145 | 73/2249 |
| needs_escape | ok | 27 | {'completed': 27} | 1/145 | 14/2249 |
| next_word | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| real_inc | ok | 42 | {'completed': 42} | 28/145 | 138/2249 |
| real_trim | ok | 60 | {'completed': 60} | 5/145 | 37/2249 |
| register_stdcmds | ok | 1 | {'completed': 1} | 17/145 | 76/2249 |
| skip_spaces | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| strclone | ok | 15 | {'completed': 15} | 1/145 | 3/2249 |
| substitute | ok | 1 | {'completed': 1} | 20/145 | 79/2249 |

## Procedure, deviations, and what is not established

<!-- prose -->
