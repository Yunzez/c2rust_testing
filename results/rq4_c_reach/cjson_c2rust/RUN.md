# Same-corpus C reach — cjson_c2rust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `cJSON.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 49 / 58 (0.845) | 708 / 926 (0.765) |
| Rust (archived campaign) | 49 / 59 (0.831) | 1816 / 2237 (0.812) |

Inputs replayed on the C side: {'completed': 8796} over 39 / 39 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/cjson__c2rust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 49 (out of scope: 1).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 41 | 0 (0) | 0 (0) | 8 | 8 | 1 (1) | 2 (1) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| cJSON_CreateArray | ok | 1 | {'completed': 1} | 2/58 | 6/926 |
| cJSON_CreateBool | ok | 5 | {'completed': 5} | 2/58 | 7/926 |
| cJSON_CreateDoubleArray | ok | 19 | {'completed': 19} | 5/58 | 19/926 |
| cJSON_CreateFalse | ok | 1 | {'completed': 1} | 2/58 | 6/926 |
| cJSON_CreateFloatArray | ok | 20 | {'completed': 20} | 5/58 | 19/926 |
| cJSON_CreateIntArray | ok | 20 | {'completed': 20} | 5/58 | 19/926 |
| cJSON_CreateNull | ok | 1 | {'completed': 1} | 2/58 | 6/926 |
| cJSON_CreateNumber | ok | 8 | {'completed': 8} | 2/58 | 6/926 |
| cJSON_CreateObject | ok | 1 | {'completed': 1} | 2/58 | 6/926 |
| cJSON_CreateString | ok | 16 | {'completed': 16} | 3/58 | 9/926 |
| cJSON_CreateStringArray | ok | 41 | {'completed': 41} | 6/58 | 22/926 |
| cJSON_CreateTrue | ok | 1 | {'completed': 1} | 2/58 | 6/926 |
| cJSON_Delete | ok | 426 | {'completed': 426} | 11/58 | 285/926 |
| cJSON_DeleteItemFromArray | ok | 465 | {'completed': 465} | 13/58 | 296/926 |
| cJSON_DeleteItemFromObject | ok | 470 | {'completed': 470} | 15/58 | 321/926 |
| cJSON_DetachItemFromArray | ok | 443 | {'completed': 443} | 12/58 | 297/926 |
| cJSON_DetachItemFromObject | ok | 426 | {'completed': 426} | 13/58 | 296/926 |
| cJSON_Duplicate | ok | 440 | {'completed': 440} | 13/58 | 300/926 |
| cJSON_GetArrayItem | ok | 433 | {'completed': 433} | 12/58 | 285/926 |
| cJSON_GetArraySize | ok | 433 | {'completed': 433} | 12/58 | 288/926 |
| cJSON_GetErrorPtr | ok | 1 | {'completed': 1} | 1/58 | 1/926 |
| cJSON_GetObjectItem | ok | 447 | {'completed': 447} | 13/58 | 304/926 |
| cJSON_Minify | ok | 1 | {'completed': 1} | 1/58 | 2/926 |
| cJSON_New_Item | ok | 1 | {'completed': 1} | 1/58 | 3/926 |
| cJSON_Parse | ok | 410 | {'completed': 410} | 11/58 | 279/926 |
| cJSON_Print | ok | 600 | {'completed': 600} | 19/58 | 484/926 |
| cJSON_PrintBuffered | ok | 476 | {'completed': 476} | 21/58 | 476/926 |
| cJSON_PrintUnformatted | ok | 547 | {'completed': 547} | 19/58 | 468/926 |
| cJSON_strcasecmp | ok | 1 | {'completed': 1} | 1/58 | 11/926 |
| cJSON_strdup | ok | 16 | {'completed': 16} | 1/58 | 3/926 |
| create_reference | ok | 424 | {'completed': 424} | 12/58 | 285/922 |
| parse_array | ok | 445 | {'completed': 445} | 11/58 | 275/926 |
| parse_hex4 | ok | 5 | {'completed': 5} | 1/58 | 13/926 |
| parse_number | ok | 423 | {'completed': 423} | 11/58 | 268/926 |
| parse_object | ok | 438 | {'completed': 438} | 10/58 | 173/926 |
| parse_string | ok | 421 | {'completed': 421} | 11/58 | 278/926 |
| parse_value | ok | 441 | {'completed': 441} | 11/58 | 266/926 |
| pow2gt | ok | 6 | {'completed': 6} | 1/58 | 0/925 |
| skip | ok | 23 | {'completed': 23} | 1/58 | 0/919 |

## Procedure, deviations, and what is not established

<!-- prose -->
