# Same-corpus C reach — quadtree_c2rust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `quadtree_all.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 20 / 24 (0.833) | 106 / 205 (0.517) |
| Rust (archived campaign) | 20 / 24 (0.833) | 220 / 436 (0.505) |

Inputs replayed on the C side: {'completed': 127, 'crash': 1} over 17 / 17 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/quadtree__c2rust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 21 (out of scope: 0).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 18 | 0 (0) | 0 (0) | 3 | 3 | 0 (0) | 0 (0) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| elision_ | ok | 1 | {'completed': 1} | 1/24 | 1/205 |
| find_ | ok | 9 | {'completed': 9} | 8/24 | 46/205 |
| get_quadrant_ | ok | 1 | {'crash': 1} | 0/24 | 0/205 |
| node_contains_ | ok | 11 | {'completed': 11} | 6/24 | 37/205 |
| quadtree_bounds_extend | ok | 6 | {'completed': 6} | 5/24 | 14/205 |
| quadtree_bounds_free | ok | 1 | {'completed': 1} | 4/24 | 13/205 |
| quadtree_bounds_new | ok | 1 | {'completed': 1} | 2/24 | 11/205 |
| quadtree_free | ok | 12 | {'completed': 12} | 12/24 | 50/205 |
| quadtree_insert | ok | 10 | {'completed': 10} | 17/24 | 88/205 |
| quadtree_new | ok | 12 | {'completed': 12} | 6/24 | 33/205 |
| quadtree_node_isempty | ok | 12 | {'completed': 12} | 7/24 | 42/205 |
| quadtree_node_isleaf | ok | 12 | {'completed': 12} | 6/24 | 29/205 |
| quadtree_node_ispointer | ok | 12 | {'completed': 12} | 6/24 | 36/205 |
| quadtree_node_new | ok | 1 | {'completed': 1} | 1/24 | 10/205 |
| quadtree_node_with_bounds | ok | 12 | {'completed': 12} | 5/24 | 27/205 |
| quadtree_point_new | ok | 6 | {'completed': 6} | 1/24 | 3/205 |
| quadtree_search | ok | 9 | {'completed': 9} | 16/24 | 70/205 |

## Procedure, deviations, and what is not established

<!-- prose -->
