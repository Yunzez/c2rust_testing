# Same-corpus C reach — genann_sactor

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `genann.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 13 / 15 (0.867) | 149 / 182 (0.819) |
| Rust (archived campaign) | 15 / 21 (0.714) | 506 / 716 (0.707) |

Inputs replayed on the C side: {'completed': 310, 'crash': 1} over 13 / 13 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_b/sactor_genann/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 12 (out of scope: 1).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 10 | 0 (0) | 0 (0) | 2 | 2 | 1 (1) | 7 (3) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| genann_act_hidden_indirect | ok | 21 | {'completed': 21} | 7/15 | 46/182 |
| genann_act_linear | ok | 20 | {'completed': 20} | 6/15 | 37/182 |
| genann_act_output_indirect | ok | 21 | {'completed': 21} | 7/15 | 46/182 |
| genann_act_sigmoid | ok | 22 | {'completed': 22} | 5/15 | 38/182 |
| genann_act_sigmoid_cached | ok | 21 | {'completed': 21} | 6/15 | 45/182 |
| genann_act_threshold | ok | 19 | {'completed': 19} | 6/15 | 37/182 |
| genann_copy | ok | 16 | {'completed': 16} | 6/15 | 39/182 |
| genann_free | ok | 16 | {'completed': 16} | 5/15 | 36/182 |
| genann_init | ok | 25 | {'completed': 24, 'crash': 1} | 4/15 | 38/182 |
| genann_init_sigmoid_lookup | ok | 16 | {'completed': 16} | 5/15 | 36/182 |
| genann_randomize | ok | 15 | {'completed': 15} | 5/15 | 36/182 |
| genann_run | ok | 46 | {'completed': 46} | 9/15 | 84/182 |
| genann_train | ok | 53 | {'completed': 53} | 10/15 | 139/182 |

## Procedure, deviations, and what is not established

<!-- prose -->
