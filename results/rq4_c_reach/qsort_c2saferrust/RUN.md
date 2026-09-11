# Same-corpus C reach — qsort_c2saferrust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `qsort.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 3 / 3 (1.000) | 10 / 10 (1.000) |
| Rust (archived campaign) | 3 / 3 (1.000) | 57 / 57 (1.000) |

Inputs replayed on the C side: {'completed': 118} over 3 / 3 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/qsort__c2saferrust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 3 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 3 | 0 (0) | 0 (0) | 0 | 0 | 0 (0) | 0 (0) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| partition | ok | 38 | {'completed': 38} | 2/3 | 7/10 |
| quickSort | ok | 72 | {'completed': 72} | 3/3 | 10/10 |
| swap | ok | 8 | {'completed': 8} | 1/3 | 1/10 |

## Procedure, deviations, and what is not established

<!-- prose -->
