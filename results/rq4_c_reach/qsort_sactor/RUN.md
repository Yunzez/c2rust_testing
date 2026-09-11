# Same-corpus C reach — qsort_sactor

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `qsort.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 3 / 3 (1.000) | 10 / 10 (1.000) |
| Rust (archived campaign) | 6 / 8 (0.750) | 99 / 196 (0.505) |

Inputs replayed on the C side: {'completed': 132} over 3 / 3 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_b/sactor_qsort/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 1 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 1 | 0 (0) | 0 (0) | 0 | 2 | 0 (0) | 5 (3) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| partition | ok | 41 | {'completed': 41} | 2/3 | 7/10 |
| quickSort | ok | 83 | {'completed': 83} | 3/3 | 10/10 |
| swap | ok | 8 | {'completed': 8} | 1/3 | 1/10 |

## Procedure, deviations, and what is not established

<!-- prose -->
