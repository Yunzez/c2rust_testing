# Same-corpus C reach — genann_crown

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `genann.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 10 / 12 (0.833) | 141 / 172 (0.820) |
| Rust (archived campaign) | 10 / 12 (0.833) | 467 / 574 (0.814) |

Inputs replayed on the C side: {'completed': 166, 'crash': 1} over 10 / 10 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/genann__crown/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 12 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 10 | 0 (0) | 0 (0) | 2 | 0 | 0 (0) | 0 (0) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| genann_act_linear | ok | 8 | {'completed': 8} | 1/12 | 1/172 |
| genann_act_sigmoid | ok | 10 | {'completed': 10} | 1/12 | 7/172 |
| genann_act_sigmoid_cached | ok | 10 | {'completed': 10} | 2/12 | 17/172 |
| genann_act_threshold | ok | 8 | {'completed': 8} | 1/12 | 1/172 |
| genann_copy | ok | 14 | {'completed': 14} | 4/12 | 29/172 |
| genann_free | ok | 13 | {'completed': 13} | 3/12 | 26/172 |
| genann_init | ok | 22 | {'completed': 21, 'crash': 1} | 2/12 | 28/172 |
| genann_randomize | ok | 12 | {'completed': 12} | 3/12 | 26/172 |
| genann_run | ok | 33 | {'completed': 33} | 6/12 | 71/172 |
| genann_train | ok | 37 | {'completed': 37} | 7/12 | 130/172 |

## Procedure, deviations, and what is not established

<!-- prose -->
