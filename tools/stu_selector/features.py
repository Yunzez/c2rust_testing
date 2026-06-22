#!/usr/bin/env python3

"""Stage 3 — candidate feature extraction over matched C<->Rust regions.

For every matched function (same name on both sides) we compute a feature vector that
estimates how far apart the C function and its Rust translation are, and how amenable the
boundary is to differential fuzzing (spec docs/stu_selection.md §6). This is deliberately a
*wide* candidate set — the point is to measure empirically which features actually vary and
capture restructuring, before committing to a learned harness-validity model.

Per matched function we emit:
  - C metrics  (libclang):  cyclomatic, stmts, loops, max_loop_depth, pointer_access, allocs
  - Rust metrics (syn):     cyclomatic, stmts, loops, max_loop_depth, derefs, method_calls, allocs
  - deltas:                 |C - Rust| for the shared structural metrics
  - call-structure:         callee_mismatch count, callee_agreement bool
  - signature features:     n_params, n_pointer_params, n_nested_pointer_params,
                            has_fn_pointer_param, returns_pointer, fuzzability, norm_burden

Modes:
  # one pair:
  python3 tools/stu_selector/features.py --compile-commands <dir> --rust <file.rs>
  # whole benchmark (benchmark/pairs/<name>/{build,translated}):
  python3 tools/stu_selector/features.py --pairs benchmark/pairs --csv benchmark/features.csv
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import statistics
import subprocess
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402
import mapping as mapmod  # noqa: E402

import clang.cindex  # noqa: E402
from clang.cindex import CursorKind, Index  # noqa: E402

ALLOC_FNS = {"malloc", "calloc", "realloc", "free"}
_SYS = ("/usr/", "/lib/", "/opt/")

STRUCT_DELTAS = ["cyclomatic", "stmts", "loops", "max_loop_depth"]


def _is_sys(p: str | None) -> bool:
    return (not p) or any(p.startswith(s) for s in _SYS)


# ---------- C metrics via libclang ----------

def _sig_features(fn) -> dict:
    args = list(fn.get_arguments())
    n_params = len(args)
    n_ptr = 0
    n_nested = 0
    has_fnptr = False
    for a in args:
        t = a.type
        spelling = t.spelling
        if t.kind == clang.cindex.TypeKind.POINTER:
            n_ptr += 1
            pointee = t.get_pointee()
            if pointee.kind == clang.cindex.TypeKind.POINTER:
                n_nested += 1
            if pointee.kind in (clang.cindex.TypeKind.FUNCTIONPROTO,
                                 clang.cindex.TypeKind.FUNCTIONNOPROTO):
                has_fnptr = True
        if "(*" in spelling:  # function pointer typedef-free form
            has_fnptr = True
    ret = fn.result_type
    returns_pointer = ret.kind == clang.cindex.TypeKind.POINTER
    # fuzzability: scalars are cheap; pointers add cost; nested pointers / fn-pointers expensive.
    fuzzability = 1.0
    fuzzability -= 0.15 * n_ptr
    fuzzability -= 0.4 * n_nested
    fuzzability -= 0.6 * (1 if has_fnptr else 0)
    fuzzability = max(0.0, round(fuzzability, 3))
    # normalization burden: comparing pointer/struct output is harder than a scalar return.
    norm_burden = round(0.5 * (1 if returns_pointer else 0) + 0.2 * n_ptr, 3)
    return {
        "n_params": n_params,
        "n_pointer_params": n_ptr,
        "n_nested_pointer_params": n_nested,
        "has_fn_pointer_param": has_fnptr,
        "returns_pointer": returns_pointer,
        "fuzzability": fuzzability,
        "norm_burden": norm_burden,
    }


def _fn_metrics(fn) -> dict:
    stmts = loops = allocs = ptr = dp = 0
    maxd = 0
    toks = [t.spelling for t in fn.get_tokens()]
    dp += toks.count("&&") + toks.count("||")

    def rec(node, depth: int) -> None:
        nonlocal stmts, loops, allocs, ptr, dp, maxd
        k = node.kind
        if k.name.endswith("_STMT"):
            stmts += 1
        if k in (CursorKind.IF_STMT, CursorKind.FOR_STMT, CursorKind.WHILE_STMT,
                 CursorKind.DO_STMT, CursorKind.CASE_STMT):
            dp += 1
        nd = depth
        if k in (CursorKind.FOR_STMT, CursorKind.WHILE_STMT, CursorKind.DO_STMT):
            loops += 1
            nd = depth + 1
            maxd = max(maxd, nd)
        if k == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            ptr += 1
        elif k == CursorKind.UNARY_OPERATOR:
            ts = [t.spelling for t in node.get_tokens()]
            if ts and ts[0] == "*":
                ptr += 1
        elif k == CursorKind.MEMBER_REF_EXPR:
            ts = [t.spelling for t in node.get_tokens()]
            if "->" in ts:
                ptr += 1
        elif k == CursorKind.CALL_EXPR:
            ref = node.referenced
            if ref is not None and ref.spelling in ALLOC_FNS:
                allocs += 1
        for c in node.get_children():
            rec(c, nd)

    rec(fn, 0)
    m = {
        "cyclomatic": 1 + dp,
        "stmts": stmts,
        "loops": loops,
        "max_loop_depth": maxd,
        "pointer_access": ptr,
        "allocs": allocs,
    }
    m.update(_sig_features(fn))
    return m


def c_metrics_from_cc(cc_dir: Path) -> dict:
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    out: dict[str, dict] = {}
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
            f = cur.location.file.name if cur.location and cur.location.file else None
            if _is_sys(f) or not cur.spelling:
                continue
            out[cur.spelling] = _fn_metrics(cur)
    return out


def rust_metrics(rust_file: Path, rust_bin: Path) -> dict:
    raw = subprocess.run([str(rust_bin), str(rust_file)], check=True,
                         capture_output=True, text=True).stdout
    data = json.loads(raw)
    return {f["name"]: f["metrics"] for f in data["functions"]}


# ---------- per-pair feature rows ----------

def features_for_pair(cc_dir: Path, rust_file: Path, rust_bin: Path, label: str) -> list[dict]:
    c_cond = mapmod.build_c_graph(cc_dir)
    r_cond = mapmod.build_rust_graph(rust_file, rust_bin)
    aln = mapmod.align(c_cond, r_cond, rust_file)
    c_met = c_metrics_from_cc(cc_dir)
    r_met = rust_metrics(rust_file, rust_bin)

    rows = []
    for mrec in aln["matched"]:
        name = mrec["name"]
        cm = c_met.get(name, {})
        rm = r_met.get(name, {})
        row = {"pair": label, "fn": name}
        for k in ["cyclomatic", "stmts", "loops", "max_loop_depth"]:
            row[f"c_{k}"] = cm.get(k, 0)
            row[f"r_{k}"] = rm.get(k, 0)
            row[f"d_{k}"] = abs(cm.get(k, 0) - rm.get(k, 0))
        row["c_pointer_access"] = cm.get("pointer_access", 0)
        row["r_pointer_intensity"] = rm.get("derefs", 0) + rm.get("method_calls", 0)
        row["c_allocs"] = cm.get("allocs", 0)
        row["r_allocs"] = rm.get("allocs", 0)
        row["callee_mismatch"] = len(mrec["only_in_c_callees"]) + len(mrec["only_in_rust_callees"])
        row["callee_agreement"] = int(mrec["callee_agreement"])
        for k in ["n_params", "n_pointer_params", "n_nested_pointer_params",
                  "has_fn_pointer_param", "returns_pointer", "fuzzability", "norm_burden"]:
            v = cm.get(k, 0)
            row[k] = int(v) if isinstance(v, bool) else v
        rows.append(row)
    return rows


def _summ(rows: list[dict]) -> None:
    if not rows:
        print("no matched functions")
        return
    numeric = [k for k in rows[0] if k not in ("pair", "fn")]
    print(f"\n{'feature':24} {'min':>6} {'max':>6} {'mean':>7} {'stdev':>7}  discriminative?")
    print("-" * 70)
    for k in numeric:
        vals = [float(r[k]) for r in rows]
        mn, mx = min(vals), max(vals)
        mean = statistics.mean(vals)
        sd = statistics.pstdev(vals)
        # crude: a feature that never varies carries no signal.
        disc = "yes" if sd > 1e-9 and mx != mn else "FLAT (no signal)"
        print(f"{k:24} {mn:6.1f} {mx:6.1f} {mean:7.2f} {sd:7.2f}  {disc}")


def main() -> int:
    ap = argparse.ArgumentParser(description="Stage 3 candidate feature extraction")
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--pairs", help="benchmark/pairs dir (batch mode)")
    g.add_argument("--compile-commands", help="single pair: compile_commands dir")
    ap.add_argument("--rust", help="single pair: translated .rs")
    ap.add_argument("--rust-bin", default=str(mapmod._DEFAULT_RUST_BIN))
    ap.add_argument("--csv", help="write aggregate CSV here (batch mode)")
    args = ap.parse_args()

    rust_bin = Path(args.rust_bin)
    all_rows: list[dict] = []

    if args.pairs:
        pairs_dir = Path(args.pairs)
        for pair in sorted(p for p in pairs_dir.iterdir() if p.is_dir() and not p.name.startswith("_")):
            cc = pair / "build"
            rs = next((pair / "translated").glob("*.rs"), None)
            if not cc.exists() or rs is None:
                continue
            try:
                all_rows += features_for_pair(cc, rs, rust_bin, pair.name)
            except Exception as e:  # noqa: BLE001
                print(f"[warn] {pair.name}: {e}", file=sys.stderr)
        if args.csv and all_rows:
            with open(args.csv, "w", newline="") as fh:
                w = csv.DictWriter(fh, fieldnames=list(all_rows[0].keys()))
                w.writeheader()
                w.writerows(all_rows)
            print(f"wrote {len(all_rows)} rows to {args.csv}")
        _summ(all_rows)
    else:
        rows = features_for_pair(Path(args.compile_commands), Path(args.rust), rust_bin, "single")
        print(json.dumps(rows, indent=2))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
