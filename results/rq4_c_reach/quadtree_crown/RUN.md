# Same-corpus C reach — quadtree_crown

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `quadtree_all.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 11 / 24 (0.458) | 68 / 198 (0.343) |
| Rust (archived campaign) | 17 / 47 (0.362) | 208 / 610 (0.341) |

Inputs replayed on the C side: {'completed': 79, 'crash': 3} over 13 / 13 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/quadtree__crown/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 21 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 9 | 0 (0) | 0 (0) | 12 | 3 | 0 (0) | 4 (2) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| elision_ | ok | 1 | {'completed': 1} | 1/24 | 1/198 |
| find_ | ok | 1 | {'crash': 1} | 0/24 | 0/198 |
| get_quadrant_ | ok | 1 | {'crash': 1} | 0/24 | 0/198 |
| node_contains_ | ok | 11 | {'completed': 11} | 6/24 | 37/198 |
| quadtree_bounds_new | ok | 1 | {'completed': 1} | 2/24 | 11/198 |
| quadtree_new | ok | 11 | {'completed': 11} | 6/24 | 33/198 |
| quadtree_node_isempty | ok | 12 | {'completed': 12} | 7/24 | 42/198 |
| quadtree_node_isleaf | ok | 12 | {'completed': 12} | 6/24 | 29/198 |
| quadtree_node_ispointer | ok | 12 | {'completed': 12} | 6/24 | 36/198 |
| quadtree_node_new | ok | 1 | {'completed': 1} | 1/24 | 10/198 |
| quadtree_node_with_bounds | ok | 12 | {'completed': 12} | 5/24 | 27/198 |
| quadtree_point_new | ok | 6 | {'completed': 6} | 1/24 | 3/198 |
| quadtree_search | ok | 1 | {'crash': 1} | 0/24 | 0/198 |

## Procedure, deviations, and what is not established

<!-- prose -->
