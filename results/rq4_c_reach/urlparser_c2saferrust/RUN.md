# Same-corpus C reach — urlparser_c2saferrust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `test.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 9 / 21 (0.429) | 57 / 307 (0.186) |
| Rust (archived campaign) | 7 / 24 (0.292) | 107 / 1183 (0.090) |

Inputs replayed on the C side: {'completed': 39, 'crash': 8} over 18 / 18 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/urlparser__c2saferrust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 13 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 5 | 1 (1) | 0 (0) | 7 | 8 | 0 (0) | 2 (0) |

`c_only`: get_part→get_part

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| get_part | ok | 1 | {'completed': 1} | 4/21 | 25/307 |
| strdup | ok | 14 | {'completed': 14} | 1/21 | 3/307 |
| strff | ok | 15 | {'completed': 15} | 2/21 | 7/307 |
| url_data_inspect | ok | 2 | {'completed': 2} | 4/21 | 25/307 |
| url_free | ok | 2 | {'completed': 2} | 4/21 | 25/307 |
| url_get_auth | ok | 1 | {'completed': 1} | 3/21 | 18/307 |
| url_get_hash | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_host | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_hostname | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_path | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_pathname | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_port | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_protocol | ok | 1 | {'completed': 1} | 2/21 | 14/307 |
| url_get_query | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_get_search | ok | 1 | {'crash': 1} | 0/21 | 0/307 |
| url_is_protocol | ok | 1 | {'completed': 1} | 1/21 | 7/307 |
| url_is_ssh | ok | 1 | {'completed': 1} | 2/21 | 9/307 |
| url_parse | ok | 1 | {'completed': 1} | 4/21 | 25/307 |

## Procedure, deviations, and what is not established

<!-- prose -->
