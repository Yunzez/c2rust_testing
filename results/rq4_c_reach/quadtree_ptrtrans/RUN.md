# Same-corpus C reach — quadtree_ptrtrans

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `quadtree_all.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 9 / 24 (0.375) | 57 / 198 (0.288) |
| Rust (archived campaign) | 13 / 19 (0.684) | 134 / 407 (0.329) |

Inputs replayed on the C side: {'crash': 18, 'completed': 69} over 11 / 11 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_b/ptrtrans_quadtree/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 11 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 4 | 0 (0) | 3 (3) | 4 | 6 | 7 (0) | 0 (0) |

`rust_only`: get_quadrant_→get_quadrant_, find_→find_, quadtree_search→quadtree_search

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| find_ | ok | 9 | {'crash': 9} | 0/24 | 0/198 |
| quadtree_bounds_extend | ok | 6 | {'completed': 6} | 3/24 | 12/198 |
| quadtree_bounds_new | ok | 1 | {'completed': 1} | 2/24 | 11/198 |
| quadtree_new | ok | 11 | {'completed': 11} | 6/24 | 33/198 |
| quadtree_node_isempty | ok | 11 | {'completed': 11} | 7/24 | 42/198 |
| quadtree_node_isleaf | ok | 11 | {'completed': 11} | 6/24 | 29/198 |
| quadtree_node_ispointer | ok | 11 | {'completed': 11} | 6/24 | 36/198 |
| quadtree_node_new | ok | 1 | {'completed': 1} | 1/24 | 10/198 |
| quadtree_node_with_bounds | ok | 11 | {'completed': 11} | 5/24 | 27/198 |
| quadtree_point_new | ok | 6 | {'completed': 6} | 1/24 | 3/198 |
| quadtree_search | ok | 9 | {'crash': 9} | 0/24 | 0/198 |

## Procedure, deviations, and what is not established

<!-- prose -->
