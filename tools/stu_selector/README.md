# stu_selector

Implements the **STU / differential-testing frontier** selection from
[`docs/stu_selection.md`](../../docs/stu_selection.md). The frontier is computed bottom-up over
the **SCC DAG** of the call graph, so the components are built in that order.

## Status

| Stage | Module | Status |
|-------|--------|--------|
| 1. C call graph → SCC DAG | `callgraph.py` | ✅ done, verified on qsort |
| 2. Cross-language region mapping (C ↔ Rust) | `mapping.py` | ⬜ next |
| 3. Region distance / feature vector `x_f` | `features.py` | ⬜ |
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

## Requirements

- `python3-clang` (libclang bindings) + `libclang-21.so` — both present in the dev env.
- A `compile_commands.json` with **paths valid on this machine** (regenerate with cmake, or
  the bundled qsort one has been path-fixed).
