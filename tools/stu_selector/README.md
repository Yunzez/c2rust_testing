# stu_selector

Implements the **STU / differential-testing frontier** selection from
[`docs/stu_selection.md`](../../docs/stu_selection.md). The frontier is computed bottom-up over
the **SCC DAG** of the call graph, so the components are built in that order.

## Status

| Stage | Module | Status |
|-------|--------|--------|
| 1. C call graph → SCC DAG | `callgraph.py` | ✅ done, verified on qsort |
| 2. Cross-language region mapping (C ↔ Rust) | `mapping.py` + `rust_callgraph/` | ✅ done, verified on qsort |
| 3. Region distance / feature vector `x_f` | `features.py` | ⬜ next |
| 4. Harness-validity model `P(valid \| x_f)` + frontier (antichain) selection | `frontier.py` | ⬜ |

## Stage 1 — `callgraph.py`

Extracts the C call graph with libclang (`clang.cindex`) and condenses it into its SCC DAG
(Tarjan, iterative). Recursion / mutual recursion collapse into one SCC. Indirect / unresolved
calls (function pointers, dynamic dispatch) are recorded separately — they feed the uncertainty /
risk model (spec §4) and are never silently dropped.

### Usage

```bash
# From a compile_commands.json directory (preferred):
python3 tools/stu_selector/callgraph.py --compile-commands projects/qsort_example/build

# Single file:
python3 tools/stu_selector/callgraph.py --file projects/qsort_example/source/qsort.c -- -I.

# Write JSON:
python3 tools/stu_selector/callgraph.py --compile-commands projects/qsort_example/build -o cg.json
```

### Output (JSON)

- `functions`: name, file, line, defined, calls_self
- `edges`: resolved caller→callee
- `indirect_calls`: unresolved/indirect call sites (caller, line)
- `sccs`: each `{id, members, recursive}`
- `scc_dag_edges`: condensation DAG edges
- `topo_order` / `bottom_up_order`: SCC ids; `bottom_up_order` is the order the frontier
  algorithm walks.

### Verified on qsort

`quickSort → partition → swap`, with `quickSort` self-recursive → its own recursive SCC.
`bottom_up_order = [swap, partition, quickSort]`. Matches the clang ground truth
(`clang -cc1 -analyze -analyzer-checker=debug.DumpCallGraph`).

## Stage 2 — `mapping.py` + `rust_callgraph/`

`rust_callgraph/` is a small `syn`-based Rust binary that extracts the Rust call graph
(functions, by-name call edges, indirect/method calls) — the Rust-side mirror of `callgraph.py`.
Graph algorithms (SCC/condensation) are kept in Python and shared via `import callgraph`.

`mapping.py` builds both call graphs and aligns them by name (C2Rust preserves names via
`#[no_mangle]`), classifying every function as `matched` / `c_only` / `rust_only` (candidate
absorbed helper → 1:N), and reporting per-matched **call-structure agreement** as an early
distance signal.

```bash
(cd tools/stu_selector/rust_callgraph && cargo build --release)
python3 tools/stu_selector/mapping.py \
  --compile-commands projects/qsort_example/build \
  --rust projects/qsort_example/translated/src/qsort.rs
```

Verified on qsort: 3/3 matched, `name_match_coverage = 1.0`, all structurally agreeing —
confirms the spec §11 assumption that name-preserving output makes mapping near-free. The
1:N / rust-only (helper-absorption) path is implemented but not yet exercised on a restructured
case (qsort is 1:1); needs a project where the translation outlines/inlines.

## Stage 3 — `features.py` (next)

Compute the per-region feature vector `x_f` (spec §6) over each matched region — candidates:
call-graph correspondence, control-flow distance (Δ basic blocks / cyclomatic / loop nesting),
side-effect surface, input-domain fuzzability, boundary uncertainty, normalization burden.
**Design note:** the feature set + the learned `P(valid harness | x_f)` model are exactly where
the project lead's critique applies (don't hand-weight; keep the vector; learn validity) — align
on the feature list before committing the model, and generate the G3 (semantics-preserving
refactor) data needed to train/evaluate it.

## Requirements

- `python3-clang` (libclang bindings) + `libclang-21.so` — both present in the dev env.
- A `compile_commands.json` with **paths valid on this machine** (regenerate with cmake, or
  the bundled qsort one has been path-fixed).
