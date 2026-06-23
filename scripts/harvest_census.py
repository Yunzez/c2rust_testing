#!/usr/bin/env python3

"""Boundary harvester — Stage A: static constructibility census.

The learned boundary-validity model predicts, from STATIC features of a candidate boundary,
whether a SOUND differential harness can be built there. To train/evaluate it we need a dataset
whose unit is a *boundary* (one matched C<->Rust function), not a *program*. This stage harvests
every matched function across benchmark/pairs and decides — WITHOUT building or fuzzing — whether
the generator can construct a harness for it, and if not, why (the hard-gate reason). It joins
that with the existing features.py vector.

Output is intentionally cheap and fully reproducible (libclang only, no cargo): it tells us the
dataset SHAPE (how many constructible boundaries, how many hard-gated, the feature distribution)
before Stage B spends build+fuzz time on validity labels.

Constructibility outcomes (per boundary):
  SUPPORTED            generator can build a harness (signature shape covered)
  UNSUPPORTED_PARAM    a parameter type is hard-gated (callback / ptr-to-ptr-to-ptr / ...)
  UNSUPPORTED_SHAPE    param types fine but adjacency/return shape unsupported (e.g. T** w/o dims)
  NO_C_DEFINITION      matched on the Rust side but no C definition found (skip)

Each boundary also records `pub_in_rust` (callable as-is) vs `needs_expose` (a `static` C function
c2rust made private — exposable as #[no_mangle] pub to test it; a meaningful boundary choice we tag
so the dataset can be analyzed with/without exposed internals).

Usage: python3 scripts/harvest_census.py            # writes dataset/ + results/boundary_census_v1.md
"""

from __future__ import annotations

import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools" / "stu_selector"
sys.path.insert(0, str(TOOLS))

import callgraph as cgmod  # noqa: E402
import features as feat  # noqa: E402
import gen_diff_harness as gdh  # noqa: E402
from clang.cindex import CompilationDatabase, CursorKind, Index, TypeKind  # noqa: E402

# Rust fns c2rust marks `pub` (non-static C). Capture the name so we can tell which boundaries are
# callable as-is vs. need exposing.
_PUB_RE = re.compile(r"(?m)^\s*pub\s+(?:unsafe\s+)?extern\s+\"C\"\s+fn\s+(\w+)")


def collect_signatures(cc_dir: Path) -> dict[str, dict]:
    """Parse each TU once; for every C function definition record its harness params or the
    SystemExit reason a param type triggered (callback, ptr-to-ptr-to-ptr, ...)."""
    cgmod._configure_libclang()
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
            if not f or f.startswith(("/usr/", "/lib/", "/opt/")) or not cur.spelling:
                continue
            if cur.spelling in out:
                continue
            rt = cur.result_type
            ret = "void" if rt.kind == TypeKind.VOID else (
                (gdh.map_scalar(rt.spelling) or ("i32", 4))[0])
            try:
                params = [gdh._param_from_descriptor(gdh.describe_type(a.type),
                                                     gdh.safe_name(a.spelling, i))
                          for i, a in enumerate(cur.get_arguments())]
                out[cur.spelling] = {"params": params, "ret": ret, "param_error": None}
            except SystemExit as e:
                out[cur.spelling] = {"params": None, "ret": ret, "param_error": str(e)}
    return out


def constructibility(sig: dict) -> tuple[str, str]:
    """Map a collected signature to a constructibility outcome + reason."""
    if sig["param_error"]:
        return "UNSUPPORTED_PARAM", sig["param_error"]
    try:
        gdh.classify(sig["params"])
        return "SUPPORTED", ""
    except SystemExit as e:
        return "UNSUPPORTED_SHAPE", str(e)


def _gate_reason(reason: str) -> str:
    """Bucket the free-text SystemExit message into a coarse hard-gate category."""
    r = reason.lower()
    if "callback" in r or "function pointer" in r:
        return "callback"
    if "char**" in r or "string-pointer" in r:
        return "char_table"
    if "pointer-to-pointer-to" in r:
        return "ptr3"
    if "t** param" in r and "without dimensions" in r:
        return "ptr2_no_dims"
    if "pointer-to-array" in r and "without a length" in r:
        return "arr_no_len"
    if "mutable pointer-to-array" in r:
        return "arr_mutable"
    # non-POD struct: a field is a pointer / nested struct / union / fn-ptr (precise validity signal).
    if "struct-invariant" in r:
        if "pointer field" in r:
            return "struct_ptr_field"
        if "nested struct" in r:
            return "struct_nested"
        if "union" in r:
            return "struct_union"
        return "struct_invariant"
    # pointer to a record/union/enum (scalar/array/function/pointer targets get distinct messages):
    if "unsupported pointer target for" in r:
        return "struct_ptr"
    if "unsupported param type for" in r and ": array" in r:
        return "array_value"
    if "unsupported param type for" in r:
        return "struct_value"
    if reason:
        return "other"
    return ""


def main() -> int:
    import argparse
    ap = argparse.ArgumentParser(description="Stage A boundary constructibility census")
    ap.add_argument("--out", default=None, help="output jsonl (default dataset/boundaries_static.jsonl)")
    cli = ap.parse_args()
    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_"))
    rust_bin = Path(feat.mapmod._DEFAULT_RUST_BIN) if hasattr(feat, "mapmod") else \
        Path(__import__("mapping")._DEFAULT_RUST_BIN)

    rows = []
    for pair in pairs:
        cc = pair / "build"
        rs = next((pair / "translated").glob("*.rs"), None)
        if not cc.exists() or rs is None:
            continue
        pub_fns = set(_PUB_RE.findall(rs.read_text(encoding="utf-8", errors="ignore")))
        try:
            frows = feat.features_for_pair(cc, rs, rust_bin, pair.name)
        except Exception as e:  # noqa: BLE001
            print(f"[warn] {pair.name}: features failed: {e}", file=sys.stderr)
            frows = []
        try:
            sigs = collect_signatures(cc)
        except Exception as e:  # noqa: BLE001
            print(f"[warn] {pair.name}: signatures failed: {e}", file=sys.stderr)
            sigs = {}

        for fr in frows:
            fn = fr["fn"]
            sig = sigs.get(fn)
            if sig is None:
                outcome, reason = "NO_C_DEFINITION", ""
            else:
                outcome, reason = constructibility(sig)
            row = dict(fr)  # carry every feature
            row["constructible"] = outcome == "SUPPORTED"
            row["constructibility"] = outcome
            row["gate_reason"] = _gate_reason(reason)
            row["unsupported_detail"] = reason
            row["pub_in_rust"] = fn in pub_fns
            row["needs_expose"] = (outcome == "SUPPORTED") and (fn not in pub_fns)
            rows.append(row)
        print(f"[{pair.name}] {len(frows)} matched fns")

    # ---- persist dataset + human summary ----
    ds_dir = ROOT / "dataset"
    ds_dir.mkdir(exist_ok=True)
    jl = Path(cli.out) if cli.out else ds_dir / "boundaries_static.jsonl"
    jl.parent.mkdir(parents=True, exist_ok=True)
    jl.write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")

    by_outcome: dict[str, int] = {}
    by_gate: dict[str, int] = {}
    n_pub = n_expose = 0
    for r in rows:
        by_outcome[r["constructibility"]] = by_outcome.get(r["constructibility"], 0) + 1
        if r["gate_reason"]:
            by_gate[r["gate_reason"]] = by_gate.get(r["gate_reason"], 0) + 1
        if r["constructible"]:
            if r["pub_in_rust"]:
                n_pub += 1
            if r["needs_expose"]:
                n_expose += 1

    n = len(rows)
    n_constructible = by_outcome.get("SUPPORTED", 0)
    md = [f"# Boundary census v1 — Stage A (static constructibility)\n",
          f"Unit = one matched C↔Rust function (a candidate STU boundary). Source: "
          f"{len(pairs)} programs in benchmark/pairs. **No build/fuzz** — pure libclang signature "
          f"analysis + features.py. Raw: `dataset/boundaries_static.jsonl`.\n",
          f"## Totals\n",
          f"- **{n} candidate boundaries** across {len(pairs)} programs "
          f"(vs {len(pairs)} program-level entries before).",
          f"- **{n_constructible} constructible** ({100*n_constructible//max(n,1)}%) — a harness "
          f"signature is supported.",
          f"  - {n_pub} already `pub` in the Rust (callable as-is); "
          f"{n_expose} are `static` internals that would need exposing (`#[no_mangle] pub`).",
          f"- **{n - n_constructible} not yet constructible** — the generator's hard-gate set.\n",
          "## Constructibility breakdown\n",
          "| outcome | n |", "|---|---|"]
    for k in sorted(by_outcome, key=lambda k: -by_outcome[k]):
        md.append(f"| {k} | {by_outcome[k]} |")
    md += ["\n## Hard-gate reasons (non-constructible)\n", "| gate | n |", "|---|---|"]
    for k in sorted(by_gate, key=lambda k: -by_gate[k]):
        md.append(f"| {k} | {by_gate[k]} |")
    md += ["\n> Stage B will build + fuzz the constructible subset (exposing `static` internals "
           "where needed) to attach validity labels: NO_DIVERGENCE_OBSERVED vs FALSE_DIVERGENCE (C-UB) vs … "
           "The C-UB boundaries are the first natural NEGATIVES for the validity model.\n"]
    (ROOT / "results" / "boundary_census_v1.md").write_text("\n".join(md) + "\n", encoding="utf-8")

    print(f"\nwrote {n} boundary rows to {jl}")
    print(f"constructible: {n_constructible}/{n} "
          f"(pub={n_pub}, needs_expose={n_expose}); gates={by_gate}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
