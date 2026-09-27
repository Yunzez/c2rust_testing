# RQ4 — reach of the generated campaign: quadtree

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md)
(§2 amendment 2026-09-07); plan: [`../../../docs/rq4_plan_ten_libraries.md`](../../../docs/rq4_plan_ten_libraries.md).
Status 2026-09-08: **three cells complete**, 3 600 s each, one campaign and one corpus per cell,
`-max_len 4096`, seed 42. Generator **0.8** — the first library run under the `nullable owned object`
bridge family ([`docs/producer_bridge_pilot.md`](../../../docs/producer_bridge_pilot.md) §6b), which
this library is the reason for.

**Two C revisions, one per translator, each hashed in its cell** (the lil provenance lesson): c2rust
was run by us on the CRUST-bench copy, the newer upstream revision whose `insert_` returns 0/1/2 and
whose `find_` guards `if (!node) return NULL;`; CROWN and PtrTrans consumed quadtree 0.1.0, which has
neither. The pair's C is one translation unit `quadtree_all.c` over `point.c`, `bounds.c`, `node.c`,
`quadtree.c`.

**Tests side**: `make` builds `bin/test` from `test.c` (four assert groups). c2rust and CROWN carry a
transpiled `test.rs`; PtrTrans's shipped crate has none. c2rust **passes → acceptance baseline**;
**CROWN's suite fails** under the translation (`unsafe precondition(s) violated:
NonNull::new_unchecked requires that the pointer is non-null`, exit 134) → denominator only, never
0 %. That failure is itself an E1-relevant observation: E1 certified this cell (`✓F*`, matrix #18)
with a differential harness that never took the path the shipped suite takes.

## Cell table

| tool | tests side | planned / built of 24 | corpus | fn tests / ours | reg tests / ours | replay | confirmed (sample) |
|---|---|---:|---:|---|---|---|---|
| **c2rust** | **PASS** (baseline) | 17 / 17 | 128 | 24 / 20 (**1.000** / 0.833) | **0.931 / 0.505** (only-tests 188, only-ours 2) | 127 normal, 1 signal | **0** (9 not reproducible, 3 `ub_associated_termination`) |
| **CROWN** | TEST-FAILS (NonNull) | 13 / 13 | 82 | — / 17 of 47¹ | — / 0.341 | 79 normal, 3 signal | **0** (9 `ub_associated_termination`, 5 not reproducible) |
| **PtrTrans** | TEST-UNAVAILABLE | 11 / 11 | 87 | — / 13 of 19 (0.684) | — / 0.329 | 69 normal, 18 signal | **0** (18 `ub_associated`) |

¹ CROWN's universe (47 functions, 610 regions) includes the helpers its ownership lift introduces,
which no boundary of the C library names; compare only-ours counts, not the fraction, as for
tulip × Laertes.

## What this library says

1. **The seventh paired cell, and the pattern from the first five holds.** Against a suite that
   passes, the campaign reaches every function but half the regions (0.505 vs 0.931), and each side
   keeps regions the other does not (188 tests-only, 2 ours-only). The 2 ours-only regions are
   rejection paths in `quadtree_insert`/`node_contains_` that `test.c`'s four fixed scenarios never
   take. Function-level parity, region-level complementarity — the same shape as bzip2 and genann.
2. **The bridge family paid for itself here, and the effect is a plan fact, not an estimate.**
   CROWN and PtrTrans both hand quadtree's owning `T*` back as `Option<Box<R>>`. Before the family
   PtrTrans planned **5** of 24 boundaries — its five constructors and nothing that USES an object;
   after it, **11**, and the six new ones are exactly the object-consuming API (`find_`,
   `quadtree_search`, `quadtree_bounds_extend`, `quadtree_node_isempty/isleaf/ispointer`). CROWN
   went 12 → 13 (`quadtree_search`). c2rust, whose translation owns objects as raw pointers, stayed
   at 17: the negative control is untouched by the change that unlocked the other two.
   *Not measured:* what PtrTrans would have covered with only the five constructors. The coverage
   figures here are for the 11-boundary plan; a five-boundary ablation was not run.
3. **Nothing confirmed on any of the three, and the 18 PtrTrans signals are the C reference.**
   quadtree 0.1.0's `get_quadrant_` reads `outer->bounds` of a node's four children without checking
   them and `find_` recurses into it, so searching a tree that has had no insert dereferences NULL
   **in C**. Reaching a non-empty tree needs `quadtree_insert` first — a second produced object,
   which the frozen bridge does not construct — so the precondition is unconstructible, not a
   defect. Verified before the run (c-only SEGVs on the empty input, rust-only is clean) and recorded
   in each pair's `preflight_accept.txt`; every one of the 18 adjudicated `ub_associated`.
   The two translations differ visibly on that same input: CROWN's Rust side panics with its own
   null-pointer check (its corpus never grows past 1), PtrTrans's returns `None` safely (corpus 9).
4. **c2rust clean for a seventh library.**

## Gaps, deviations and limits

- Still unconstructible, each with the reason in the plan: three function-pointer callbacks
  (`quadtree_walk`, `quadtree_node_free`, `quadtree_node_reset`); three boundaries taking two
  produced objects (`insert_`, `split_node_`, `reset_node_`); `quadtree_point_free`, which passes a
  POD struct pointer to `free()` that the harness itself owns; and, on CROWN, targets that CONSUME
  the box (`quadtree_free`, `quadtree_bounds_free`) or take a **raw pointer** from a boxed owner
  (`quadtree_insert`, `quadtree_bounds_extend`) — the latter is an adjacent shape, deliberately
  outside the frozen family and reported for a decision rather than implemented.
- CROWN exports 10 of its 13 built harnesses (three produced no coverage export); c2rust 16 of 17.
- Single campaign per cell; confirmation is the 200-per-channel sample, and every cell's candidate
  count was below that, so all candidates were adjudicated.

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md, funnel.json, plans.json, analysis/,
divergences/, confirm_sample/, candidates_sample/, corpus.tar.gz, harness_exports.tar.gz,
artifact_hashes.json, raw/). Pairs: `benchmark/pairs/rq4/quadtree_{c2rust,crown,ptrtrans}/` with
`PROVENANCE.json` and `preflight_accept.txt`. Manifest: no new entries.
