# Feature Study v1 — which STU features actually carry signal (2026-06-22)

First empirical pass at the Stage 3 question: of the candidate region features, **which ones
actually vary and capture C↔Rust restructuring?** Method-over-memory — measured, not guessed.

## Setup

- **Benchmark:** 12 self-contained C programs across 4 themes (strings/parsing, data
  structures, numeric/bitops, algorithms/stateful), complexity gradient incl. recursion,
  pointer-heavy, allocation, opcode VM. All transpiled with **c2rust 0.22.1** (12/12 succeeded)
  → C↔Rust pairs under `benchmark/pairs/`. (qsort/urlparser available too.)
- **Extraction:** `tools/stu_selector/features.py` over **54 matched functions**
  (`benchmark/features.csv`). C metrics via libclang, Rust metrics via the `syn` helper.

## What carries signal

| Feature | Verdict | Evidence |
|---|---|---|
| `c_cyclomatic`, `c_stmts` (C-side complexity/size) | **keep** | wide range (1–44, 1–104); predicts divergence: `corr(c_cyclomatic, d_stmts)=+0.53` |
| `d_cyclomatic` (control-flow divergence) | **keep** | mostly 0 but spikes on the hard cases (tiny_vm `vm_step`: +4) — cleanly flags real restructuring |
| `d_stmts` (statement-count divergence) | **keep, but fix metric** | strongest spread (0–17); ⚠ partly a switch-vs-match representation artifact (see below) |
| pointer intensity (`c_pointer_access`, C→Rust ratio) | **keep** | c2rust inflates pointer ops **2.06×** (232→477) — systematic, so use C-side count + ratio, not raw Rust count |
| `n_pointer_params`, `returns_pointer`, `allocs` | **keep** | vary meaningfully; drive fuzzability/normalization |

## What does NOT carry signal (drop or defer)

| Feature | Why |
|---|---|
| `d_loops`, `d_max_loop_depth` | **always 0** — c2rust preserves loop count & nesting *exactly*. A stable invariant, useless for ranking divergence (keep only as a cheap equivalence sanity check). |
| `fuzzability`, `norm_burden` | **redundant** — `corr(fuzzability, n_pointer_params) = −1.00`, `corr(norm_burden, returns_pointer) = +0.68`. They are linear re-encodings of the raw pointer counts; don't treat as independent features. |
| `callee_mismatch`, `callee_agreement`, `n_nested_pointer_params`, `has_fn_pointer_param` | **flat (untested, NOT proven useless)** — this corpus is single-file with no 1:N restructuring, nested pointers, or fn-pointers, so these never fired. Need a harder corpus to evaluate. |

## Known issue to fix

Statement counting is **not apples-to-apples across a C `switch` and a Rust `match`**: C counts
each case as a statement; Rust `match` arms are expressions. This spuriously inflates `d_stmts`
(e.g. `intmath_eval` 13→1, `bitutils_eval` 15→3 — these *collapsed*, not restructured). Fix:
count match arms as statements, or switch to a representation-agnostic size (AST node count).

## Recommendation for the model (Stage 4)

Carry this reduced feature set into the learned harness-validity model: C-side complexity
(`c_cyclomatic`, `c_stmts`), divergence (`d_cyclomatic`, fixed `d_stmts`), pointer intensity
(C count + C→Rust ratio), and signature fuzzability (`n_pointer_params`, `returns_pointer`,
`allocs`). Drop the redundant scalar re-encodings and the loop deltas.

## The honest caveat

This pass measures **which features vary and capture restructuring** — a necessary precondition.
It does **not** yet measure the real target: whether a feature predicts **harness validity**
(true-divergence precision / false-divergence rate). That needs the G1/G2/G3 labels
(`docs/stu_selection.md` §7) — the next loop. Two corpus gaps to close first: (1) multi-file /
restructured projects to exercise call-correspondence & 1:N mapping; (2) nested-pointer /
fn-pointer inputs to exercise the fuzzability gates.
