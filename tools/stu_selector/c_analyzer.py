#!/usr/bin/env python3
"""C-side per-function analyzer — parity with the rust-analyzer-based Rust analyzer.

Frontend = libclang (the real Clang frontend; the C analogue of rust-analyzer). Emits,
for every C function DEFINITION in a compile_commands.json project, the SAME JSON schema
as tools/stu_selector/analyzer (the Rust side), so the two can be compared apples-to-apples
by the (future) name-independent matcher:

  { "functions": [ { "name", "line",
                     "signature": {"params":[{"name","ty","ptr_kind","dir"}], "ret"},
                     "io": {"inputs":[{"ty","shape"}], "output":{"ty","shape"}},
                     "metrics": {...}  # only with --enable-metrics
                   } ],
    "raw_edges": [{"from","to"}],
    "indirect_calls": [{"from","line","kind"}] }

Shared `shape` grammar (IDENTICAL to analyzer/src/io.rs):
  pointer `*S` · reference `&S` (Rust-only) · array `[S;N]` · struct `{S,..}` ·
  union `union{S,..}` · generic container `Name<S,..>` (Rust-only) · fn-ptr `fn` ·
  leaf = canonical primitive.
Canonical leaf vocabulary: fixed-width by (sign,width); usize≡u64, isize≡i64 — so C
`size_t` (usize) and the Rust side's resolved `u64` collapse to the same token.

Reuses: callgraph.py (call graph), gen_diff_harness.map_scalar (C scalar -> rust name),
features._fn_metrics / c_metrics_from_cc (metrics). The type `shape` is walked directly
over the clang type (depth + visited guarded) rather than via describe_type, which is not
recursion-safe on self-referential structs.
"""
import argparse
import json
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402
import gen_diff_harness as gdh  # noqa: E402
import features as featmod  # noqa: E402

from clang.cindex import CompilationDatabase, CursorKind, Index, TypeKind  # noqa: E402

_MAX_DEPTH = 8
_COMPARABLE_METRICS = (
    "cyclomatic", "stmts", "nodes", "loops", "max_loop_depth", "derefs", "allocs",
)


def canon_leaf(rust_name: str) -> str:
    """Shared canonical leaf vocabulary (mirror analyzer/src/io.rs)."""
    return {"usize": "u64", "isize": "i64"}.get(rust_name, rust_name)


def _peel(t):
    s = t
    n = 0
    while s.kind in (TypeKind.TYPEDEF, TypeKind.ELABORATED) and n < 8:
        s = s.get_canonical()
        n += 1
    return s


def clang_shape(t, depth: int = 0, seen: frozenset = frozenset()) -> str:
    if depth > _MAX_DEPTH:
        return "_"
    s = _peel(t)
    if s.kind == TypeKind.VOID:
        return "()"  # match Rust unit
    if s.kind == TypeKind.POINTER:
        return "*" + clang_shape(s.get_pointee(), depth + 1, seen)
    if s.kind == TypeKind.CONSTANTARRAY:
        return f"[{clang_shape(s.element_type, depth + 1, seen)};{s.element_count}]"
    if s.kind == TypeKind.INCOMPLETEARRAY:
        return f"[{clang_shape(s.element_type, depth + 1, seen)}]"
    if s.kind in (TypeKind.FUNCTIONPROTO, TypeKind.FUNCTIONNOPROTO):
        return "fn"
    sc = gdh.map_scalar(t.spelling) or gdh.map_scalar(s.spelling)
    if sc:
        return canon_leaf(sc[0])
    if s.kind == TypeKind.RECORD:
        decl = s.get_declaration()
        key = s.spelling or decl.spelling
        if key in seen:
            return "<rec>"
        seen2 = seen | {key}
        is_union = decl.kind == CursorKind.UNION_DECL
        fields = [
            clang_shape(f.type, depth + 1, seen2)
            for f in decl.get_children()
            if f.kind == CursorKind.FIELD_DECL
        ]
        body = "{" + ",".join(fields) + "}"
        return ("union" + body) if is_union else body
    return t.spelling


_BINOPS = {"+", "-", "*", "/", "%", "<<", ">>", "^", "|", "&",
           "==", "!=", "<", ">", "<=", ">=", "&&", "||"}


def _c_binop(cur) -> str | None:
    """The operator of a binary/compound-assign cursor: the token between the two
    operand subtrees. Compound assignment `OP=` is normalized to `OP`."""
    kids = list(cur.get_children())
    if len(kids) != 2:
        return None
    lo = kids[0].extent.end.offset
    ro = kids[1].extent.start.offset
    for t in cur.get_tokens():
        o = t.extent.start.offset
        if lo <= o < ro:
            s = t.spelling
            if s in _BINOPS:
                return s
            if len(s) > 1 and s.endswith("=") and s not in ("==", "<=", ">=", "!="):
                u = s[:-1]
                if u in _BINOPS:
                    return u
    return None


def ops_of(fn_cur) -> dict:
    """Operator histogram (mirror analyzer/src/ops.rs): binary/compound-assign ops +
    unary `!`/`~` (both -> `!`, matching Rust). deref/neg/addr skipped."""
    h: dict = {}
    for cur in fn_cur.walk_preorder():
        kn = cur.kind.name
        if kn in ("BINARY_OPERATOR", "COMPOUND_ASSIGNMENT_OPERATOR"):
            s = _c_binop(cur)
            if s:
                h[s] = h.get(s, 0) + 1
        elif kn == "UNARY_OPERATOR":
            toks = [t.spelling for t in cur.get_tokens()]
            if toks and toks[0] in ("!", "~"):
                h["!"] = h.get("!", 0) + 1
    return h


def clang_ptr_kind(t) -> str:
    s = _peel(t)
    if s.kind == TypeKind.POINTER:
        return "const_ptr" if s.get_pointee().is_const_qualified() else "mut_ptr"
    return "value"


def _dir(ptr_kind: str) -> str:
    return "inout" if ptr_kind in ("mut_ptr", "mut_ref") else "in"


def _fn_record(cur, metrics_by_name: dict, enable_metrics: bool) -> dict:
    name = cur.spelling
    params, inputs = [], []
    for a in cur.get_arguments():
        pk = clang_ptr_kind(a.type)
        params.append({"name": a.spelling or "", "ty": a.type.spelling,
                       "ptr_kind": pk, "dir": _dir(pk)})
        inputs.append({"ty": a.type.spelling, "shape": clang_shape(a.type)})
    ret = cur.result_type
    rec = {
        "name": name,
        "line": cur.location.line if cur.location else 0,
        "signature": {"params": params, "ret": ret.spelling},
        "io": {"inputs": inputs, "output": {"ty": ret.spelling, "shape": clang_shape(ret)}},
        "ops": ops_of(cur),
    }
    if enable_metrics:
        m = dict(metrics_by_name.get(name, {}))
        if "pointer_access" in m:
            m["derefs"] = m.pop("pointer_access")
        rec["metrics"] = m
    return rec


def analyze(cc_dir: Path, enable_metrics: bool = False) -> dict:
    cgmod._configure_libclang()
    cg = cgmod.CallGraph()
    cgmod.parse_compile_commands(cc_dir, cg)
    metrics_by_name = featmod.c_metrics_from_cc(cc_dir) if enable_metrics else {}

    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    seen_fns, functions = set(), []
    cwd0 = os.getcwd()
    for cmd in cdb.getAllCompileCommands():
        src_abs = str((Path(cmd.directory) / cmd.filename).resolve())
        names = {cmd.filename, src_abs, Path(cmd.filename).name}
        args = cgmod._filter_compile_args(list(cmd.arguments), names)
        os.chdir(cmd.directory if Path(cmd.directory).exists() else cc_dir)
        try:
            tu = index.parse(src_abs, args=args)
        finally:
            os.chdir(cwd0)
        for cur in tu.cursor.walk_preorder():
            if cur.kind != CursorKind.FUNCTION_DECL or not cur.is_definition():
                continue
            loc = cur.location.file.name if cur.location and cur.location.file else None
            if cgmod._is_system_path(loc) or not cur.spelling or cur.spelling in seen_fns:
                continue
            seen_fns.add(cur.spelling)
            try:
                functions.append(_fn_record(cur, metrics_by_name, enable_metrics))
            except RecursionError:
                functions.append({"name": cur.spelling,
                                  "line": cur.location.line if cur.location else 0,
                                  "signature": {"params": [], "ret": "<rec>"},
                                  "io": {"inputs": [], "output": {"ty": "<rec>", "shape": "<rec>"}},
                                  "ops": {}})

    return {
        "functions": functions,
        "raw_edges": [{"from": a, "to": b} for (a, b) in sorted(cg.edges)],
        "indirect_calls": [{"from": d["from"], "line": d.get("line", 0),
                            "kind": "call_unresolved"} for d in cg.indirect],
    }


def main() -> int:
    ap = argparse.ArgumentParser(
        description="C-side per-function analyzer (parity with the Rust analyzer)")
    ap.add_argument("--compile-commands", required=True, help="Dir with compile_commands.json")
    ap.add_argument("--enable-metrics", action="store_true")
    ap.add_argument("-o", "--out", help="Write JSON here (default: stdout)")
    args = ap.parse_args()
    out = analyze(Path(args.compile_commands), args.enable_metrics)
    text = json.dumps(out, indent=2)
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
        print(f"wrote {args.out}")
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
