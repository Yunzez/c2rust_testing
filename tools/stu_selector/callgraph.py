#!/usr/bin/env python3

"""Extract a C call graph and condense it into an SCC DAG.

This is the first building block of the STU selector (see docs/stu_selection.md §4):
the differential-testing frontier is computed bottom-up over the SCC DAG of the call
graph, so we first need a faithful call graph with strongly-connected components
(recursion / mutual recursion) collapsed.

It uses libclang (clang.cindex) so that later stages can reuse the same AST to compute
region features (CFG metrics, side effects, signature shape). Indirect / unresolved
calls are recorded separately — per the spec they feed the uncertainty / risk model and
must never be silently dropped.

Usage:
  # From a compile_commands.json directory (preferred):
  python3 tools/stu_selector/callgraph.py --compile-commands projects/qsort_example/build

  # From a single translation unit:
  python3 tools/stu_selector/callgraph.py --file projects/qsort_example/source/qsort.c -- -I.

  # Write JSON to a file:
  python3 tools/stu_selector/callgraph.py --compile-commands projects/qsort_example/build -o cg.json
"""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

import clang.cindex
from clang.cindex import CompilationDatabase, CursorKind, Index, TranslationUnit

# Common locations for libclang on Debian/Ubuntu; the bindings sometimes need an
# explicit path even when the .so is installed.
_LIBCLANG_CANDIDATES = [
    "/usr/lib/x86_64-linux-gnu/libclang-21.so",
    "/usr/lib/x86_64-linux-gnu/libclang-21.so.21",
    "/usr/lib/llvm-21/lib/libclang.so",
]

# Definitions under these prefixes are treated as system code and skipped.
_SYSTEM_PREFIXES = ("/usr/", "/lib/", "/opt/")


def _configure_libclang() -> None:
    for cand in _LIBCLANG_CANDIDATES:
        if Path(cand).exists():
            try:
                clang.cindex.Config.set_library_file(cand)
            except Exception:
                # Already configured / loaded; ignore.
                pass
            return


def _is_system_path(path: str | None) -> bool:
    if not path:
        return True
    return any(path.startswith(p) for p in _SYSTEM_PREFIXES)


_SRC_EXTS = (".c", ".cc", ".cpp", ".cxx", ".c++", ".m", ".mm")


def _filter_compile_args(args: list[str], src_names: set[str]) -> list[str]:
    """Strip the compiler driver, output flags, and the input source token.

    libclang receives the source path separately (as `path`), so the bare source
    token in the original command must be removed to avoid a duplicate input.
    """
    out: list[str] = []
    skip_next = False
    for i, a in enumerate(args):
        if i == 0:
            # argv[0] is the compiler (clang/cc); drop it.
            continue
        if skip_next:
            skip_next = False
            continue
        if a == "-c":
            continue
        if a == "-o":
            skip_next = True
            continue
        if a.startswith("-o"):
            continue
        # Drop the bare source-file token(s).
        if not a.startswith("-") and (a in src_names or a.endswith(_SRC_EXTS)):
            continue
        out.append(a)
    return out


class CallGraph:
    def __init__(self) -> None:
        # name -> {"file", "line", "defined"}
        self.functions: dict[str, dict] = {}
        # set of (caller, callee) over resolved function->function calls
        self.edges: set[tuple[str, str]] = set()
        # unresolved/indirect calls: list of {"from", "line"}
        self.indirect: list[dict] = []

    def add_function(self, name: str, file: str | None, line: int, defined: bool) -> None:
        existing = self.functions.get(name)
        if existing is None:
            self.functions[name] = {"file": file, "line": line, "defined": defined}
        elif defined and not existing["defined"]:
            # Prefer the definition's location over a prior declaration.
            self.functions[name] = {"file": file, "line": line, "defined": True}

    def add_edge(self, caller: str, callee: str) -> None:
        self.edges.add((caller, callee))

    def add_indirect(self, caller: str, line: int) -> None:
        self.indirect.append({"from": caller, "line": line})


def _walk_calls(fn_cursor, caller: str, cg: CallGraph) -> None:
    """Record call edges found inside a function definition body."""
    for node in fn_cursor.walk_preorder():
        if node.kind != CursorKind.CALL_EXPR:
            continue
        callee = node.referenced
        if callee is not None and callee.kind in (
            CursorKind.FUNCTION_DECL,
            CursorKind.CXX_METHOD,
        ):
            name = callee.spelling
            if name:
                cg.add_edge(caller, name)
                continue
        # Function pointer / unresolved dynamic dispatch / builtin without decl.
        line = node.location.line if node.location else 0
        cg.add_indirect(caller, line)


def build_from_tu(tu: TranslationUnit, cg: CallGraph) -> None:
    for cursor in tu.cursor.walk_preorder():
        if cursor.kind != CursorKind.FUNCTION_DECL:
            continue
        loc_file = cursor.location.file.name if cursor.location and cursor.location.file else None
        if _is_system_path(loc_file):
            continue
        name = cursor.spelling
        if not name:
            continue
        defined = cursor.is_definition()
        line = cursor.location.line if cursor.location else 0
        cg.add_function(name, loc_file, line, defined)
        if defined:
            _walk_calls(cursor, name, cg)


# ---- Tarjan SCC (iterative, safe for deep graphs) ----

def tarjan_scc(nodes: list[str], adj: dict[str, list[str]]) -> list[list[str]]:
    index_counter = [0]
    index: dict[str, int] = {}
    lowlink: dict[str, int] = {}
    on_stack: dict[str, bool] = {}
    stack: list[str] = []
    result: list[list[str]] = []

    for root in nodes:
        if root in index:
            continue
        # work stack holds (node, iterator-position)
        work: list[tuple[str, int]] = [(root, 0)]
        while work:
            v, pi = work[-1]
            if pi == 0:
                index[v] = lowlink[v] = index_counter[0]
                index_counter[0] += 1
                stack.append(v)
                on_stack[v] = True
            recursed = False
            neighbors = adj.get(v, [])
            for j in range(pi, len(neighbors)):
                w = neighbors[j]
                if w not in index:
                    work[-1] = (v, j + 1)
                    work.append((w, 0))
                    recursed = True
                    break
                elif on_stack.get(w):
                    lowlink[v] = min(lowlink[v], index[w])
            if recursed:
                continue
            if lowlink[v] == index[v]:
                comp: list[str] = []
                while True:
                    w = stack.pop()
                    on_stack[w] = False
                    comp.append(w)
                    if w == v:
                        break
                result.append(comp)
            work.pop()
            if work:
                parent = work[-1][0]
                lowlink[parent] = min(lowlink[parent], lowlink[v])
    return result


def condense(cg: CallGraph) -> dict:
    names = sorted(cg.functions.keys())
    # Adjacency limited to edges among known functions.
    known = set(names)
    adj: dict[str, list[str]] = {n: [] for n in names}
    for a, b in cg.edges:
        if a in known and b in known:
            adj[a].append(b)

    sccs = tarjan_scc(names, adj)
    # Map each function to its SCC id.
    scc_of: dict[str, int] = {}
    scc_records = []
    for sid, comp in enumerate(sccs):
        members = sorted(comp)
        for m in members:
            scc_of[m] = sid
        self_loop = any((m, m) in cg.edges for m in members)
        recursive = len(members) > 1 or self_loop
        scc_records.append({"id": sid, "members": members, "recursive": recursive})

    # Condensation DAG edges (dedup, no self).
    dag_edges: set[tuple[int, int]] = set()
    for a, b in cg.edges:
        if a in scc_of and b in scc_of:
            sa, sb = scc_of[a], scc_of[b]
            if sa != sb:
                dag_edges.add((sa, sb))

    # Topological order over the DAG (callers before callees); Kahn's algorithm.
    indeg = {rec["id"]: 0 for rec in scc_records}
    out_adj: dict[int, list[int]] = {rec["id"]: [] for rec in scc_records}
    for sa, sb in dag_edges:
        out_adj[sa].append(sb)
        indeg[sb] += 1
    queue = sorted([sid for sid, d in indeg.items() if d == 0])
    topo: list[int] = []
    while queue:
        n = queue.pop(0)
        topo.append(n)
        for m in sorted(out_adj[n]):
            indeg[m] -= 1
            if indeg[m] == 0:
                queue.append(m)
        queue.sort()

    return {
        "functions": [
            {"name": n, **cg.functions[n], "calls_self": (n, n) in cg.edges}
            for n in names
        ],
        "edges": sorted(
            [{"from": a, "to": b} for a, b in cg.edges if a in known and b in known],
            key=lambda e: (e["from"], e["to"]),
        ),
        "indirect_calls": cg.indirect,
        "sccs": scc_records,
        "scc_dag_edges": sorted([[a, b] for a, b in dag_edges]),
        "topo_order": topo,
        "bottom_up_order": list(reversed(topo)),
    }


def parse_compile_commands(cc_dir: Path, cg: CallGraph) -> None:
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    seen_files = set()
    cwd0 = os.getcwd()
    for cmd in cdb.getAllCompileCommands():
        src_abs = str((Path(cmd.directory) / cmd.filename).resolve())
        if src_abs in seen_files:
            continue
        seen_files.add(src_abs)
        src_names = {cmd.filename, src_abs, Path(cmd.filename).name}
        args = _filter_compile_args(list(cmd.arguments), src_names)
        # Resolve relative -I / file paths against the original build directory.
        os.chdir(cmd.directory)
        try:
            tu = index.parse(src_abs, args=args)
        finally:
            os.chdir(cwd0)
        _report_diagnostics(tu, src_abs)
        build_from_tu(tu, cg)


def parse_single_file(file: Path, extra_args: list[str], cg: CallGraph) -> None:
    index = Index.create()
    tu = index.parse(str(file), args=extra_args)
    _report_diagnostics(tu, str(file))
    build_from_tu(tu, cg)


def _report_diagnostics(tu: TranslationUnit, src: str) -> None:
    errs = [d for d in tu.diagnostics if d.severity >= clang.cindex.Diagnostic.Error]
    if errs:
        print(f"[warn] {src}: {len(errs)} parse error(s); call graph may be incomplete",
              file=sys.stderr)


def main() -> int:
    ap = argparse.ArgumentParser(description="Extract C call graph and SCC DAG")
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--compile-commands", help="Directory containing compile_commands.json")
    g.add_argument("--file", help="Single C source file")
    ap.add_argument("-o", "--out", help="Write JSON here (default: stdout)")
    ap.add_argument("rest", nargs=argparse.REMAINDER,
                    help="Extra clang args after -- (single-file mode)")
    args = ap.parse_args()

    _configure_libclang()
    cg = CallGraph()

    if args.compile_commands:
        parse_compile_commands(Path(args.compile_commands), cg)
    else:
        extra = args.rest
        if extra and extra[0] == "--":
            extra = extra[1:]
        parse_single_file(Path(args.file), extra, cg)

    result = condense(cg)
    text = json.dumps(result, indent=2)
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
        print(f"Wrote call graph to {args.out}")
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
