# Same-corpus C reach — qsort_crown

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `qsort.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 3 / 3 (1.000) | 10 / 10 (1.000) |
| Rust (archived campaign) | 3 / 3 (1.000) | 61 / 61 (1.000) |

Inputs replayed on the C side: {'completed': 120} over 3 / 3 archived-built boundaries.

## Matched functions

No accepted correspondence map for this cell (side-specific numbers only).

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| partition | ok | 41 | {'completed': 41} | 2/3 | 7/10 |
| quickSort | ok | 71 | {'completed': 71} | 3/3 | 10/10 |
| swap | ok | 8 | {'completed': 8} | 1/3 | 1/10 |

## Procedure, deviations, and what is not established

<!-- prose -->
