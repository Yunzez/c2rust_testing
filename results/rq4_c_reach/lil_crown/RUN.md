# Same-corpus C reach — lil_crown

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `lil.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 130 / 145 (0.897) | 1690 / 2249 (0.751) |
| Rust (archived campaign) | 127 / 134 (0.948) | 5294 / 6409 (0.826) |

Inputs replayed on the C side: {'completed': 1856, 'crash': 3} over 42 / 42 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/lil__crown/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 109 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 104 | 0 (0) | 0 (0) | 5 | 19 | 17 (9) | 1 (1) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| add_func | ok | 16 | {'completed': 16} | 17/145 | 76/2249 |
| alloc_value | ok | 16 | {'completed': 16} | 2/145 | 11/2249 |
| ateol | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| find_cmd | ok | 14 | {'completed': 14} | 17/145 | 71/2249 |
| get_bracketpart | ok | 1 | {'completed': 1} | 19/145 | 80/2249 |
| get_dollarpart | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| islilspecial | ok | 8 | {'completed': 8} | 1/145 | 15/2249 |
| lil_alloc_double | ok | 10 | {'completed': 10} | 3/145 | 12/2249 |
| lil_alloc_integer | ok | 8 | {'completed': 8} | 3/145 | 13/2249 |
| lil_alloc_list | ok | 1 | {'completed': 1} | 1/145 | 2/2249 |
| lil_alloc_string | ok | 16 | {'completed': 16} | 3/145 | 12/2249 |
| lil_append_char | ok | 9 | {'completed': 9} | 5/145 | 18/2249 |
| lil_append_string | ok | 21 | {'completed': 21} | 6/145 | 24/2249 |
| lil_clone_value | ok | 8 | {'completed': 8} | 5/145 | 25/2249 |
| lil_free | ok | 1 | {'completed': 1} | 17/145 | 70/2249 |
| lil_free_list | ok | 1 | {'completed': 1} | 2/145 | 6/2249 |
| lil_free_value | ok | 8 | {'completed': 8} | 4/145 | 15/2249 |
| lil_get_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_get_var | ok | 15 | {'completed': 15} | 21/145 | 82/2249 |
| lil_list_get | ok | 8 | {'completed': 8} | 3/145 | 9/2249 |
| lil_list_size | ok | 1 | {'completed': 1} | 3/145 | 7/2249 |
| lil_list_to_value | ok | 5 | {'completed': 5} | 5/145 | 19/2249 |
| lil_new | ok | 1 | {'completed': 1} | 13/145 | 45/2249 |
| lil_parse | ok | 1466 | {'completed': 1466} | 128/145 | 1687/2249 |
| lil_pop_env | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| lil_push_env | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_data | ok | 1 | {'completed': 1} | 18/145 | 71/2249 |
| lil_set_error | ok | 16 | {'completed': 16} | 18/145 | 76/2249 |
| lil_set_error_at | ok | 22 | {'completed': 22} | 18/145 | 76/2249 |
| lil_to_boolean | ok | 14 | {'completed': 14} | 6/145 | 36/2249 |
| lil_to_double | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_integer | ok | 8 | {'completed': 8} | 6/145 | 21/2249 |
| lil_to_string | ok | 8 | {'completed': 8} | 5/145 | 20/2249 |
| lil_unused_name | ok | 16 | {'completed': 16} | 21/145 | 88/2249 |
| needs_escape | ok | 22 | {'completed': 22} | 1/145 | 14/2249 |
| next_word | ok | 1 | {'crash': 1} | 0/145 | 0/2249 |
| real_inc | ok | 21 | {'completed': 21} | 28/145 | 138/2249 |
| real_trim | ok | 64 | {'completed': 64} | 5/145 | 37/2249 |
| register_stdcmds | ok | 1 | {'completed': 1} | 17/145 | 76/2249 |
| skip_spaces | ok | 1 | {'completed': 1} | 18/145 | 72/2249 |
| strclone | ok | 16 | {'completed': 16} | 1/145 | 3/2249 |
| substitute | ok | 1 | {'completed': 1} | 20/145 | 79/2249 |

## Procedure, deviations, and what is not established

<!-- prose -->
