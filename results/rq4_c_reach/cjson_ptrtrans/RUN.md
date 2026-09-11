# Same-corpus C reach — cjson_ptrtrans

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `cJSON.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 10 / 113 (0.088) | 33 / 1618 (0.020) |
| Rust (archived campaign) | 10 / 121 (0.083) | 68 / 2125 (0.032) |

Inputs replayed on the C side: {'completed': 20} over 9 / 9 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_b/ptrtrans_cjson/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 23 (out of scope: 2).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 3 | 0 (0) | 0 (0) | 20 | 83 | 7 (0) | 5 (0) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| cJSON_CreateArray | ok | 1 | {'completed': 1} | 2/113 | 6/1618 |
| cJSON_CreateBool | ok | 5 | {'completed': 5} | 2/113 | 7/1618 |
| cJSON_CreateFalse | ok | 1 | {'completed': 1} | 2/113 | 6/1618 |
| cJSON_CreateNull | ok | 1 | {'completed': 1} | 2/113 | 6/1618 |
| cJSON_CreateNumber | ok | 8 | {'completed': 8} | 2/113 | 12/1618 |
| cJSON_CreateObject | ok | 1 | {'completed': 1} | 2/113 | 6/1618 |
| cJSON_CreateTrue | ok | 1 | {'completed': 1} | 2/113 | 6/1618 |
| cJSON_GetErrorPtr | ok | 1 | {'completed': 1} | 1/113 | 1/1618 |
| cJSON_Version | ok | 1 | {'completed': 1} | 1/113 | 1/1618 |

## Procedure, deviations, and what is not established

<!-- prose -->
