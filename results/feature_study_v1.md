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

---

# v2 update — metric fix + harder corpus (2026-06-22)

Closed gap (2) above and fixed the `d_stmts` artifact.

- **Corpus → 18 programs / 85 matched functions** (added `fn_pointers/` ×3 and
  `nested_pointers/` ×3; all transpiled by c2rust). `benchmark/features.csv` regenerated.
- **Metric fix (the `d_stmts` artifact):** added a representation-agnostic size = expr+stmt AST
  **node count** (`c_nodes`/`r_nodes`), excluding libclang `UNEXPOSED_EXPR` (implicit casts syn
  has no equivalent for), plus a size-normalized **`size_ratio = r_nodes / c_nodes`**. The
  absurd `intmath_eval` 13→1 stmt case is now `c_nodes=36, r_nodes=29, size_ratio=0.81` — sane.
  Across the corpus `size_ratio` ranges 0.8–2.4, **mean 1.48** (c2rust grows AST ~1.5×).
- **Previously-flat features now fire (corpus gap closed):**
  `has_fn_pointer_param` (on `array_map_reduce`), `n_nested_pointer_params` (on
  `matrix_reduce`/`word_tokens`/`graph_dfs`), and a new **`c_indirect_calls`** boundary-uncertainty
  feature (on the dispatch-table programs). Fixed a false positive: pointer-to-array
  `(*edges)[2]` was misread as a fn-pointer; now uses the canonical pointee TypeKind only.
- **Still flat — now a robust finding across 18 programs, not a corpus gap:**
  - `d_loops`, `d_max_loop_depth` = 0 everywhere → **c2rust preserves loop structure exactly.**
  - `callee_mismatch`, `callee_agreement` flat → **c2rust single-TU output is 1:1 by name**; the
    call-correspondence / 1:N features will only matter for **multi-file projects or LLM
    transpilers**, not single-file c2rust. Defer them to that setting.

**Reduced feature set going into Stage 4:** C-side complexity (`c_cyclomatic`, `c_nodes`),
divergence (`size_ratio`, `d_cyclomatic`), pointer intensity (`c_pointer_access` + C→Rust ratio),
boundary uncertainty (`c_indirect_calls`), and signature gates (`n_pointer_params`,
`n_nested_pointer_params`, `has_fn_pointer_param`, `returns_pointer`, `allocs`). Drop loop deltas
and the redundant `fuzzability`/`norm_burden` re-encodings. The real usefulness test (vs harness
validity) still requires the G1/G2/G3 labels — that is the next loop.
