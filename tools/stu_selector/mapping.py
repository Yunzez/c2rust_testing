#!/usr/bin/env python3

"""Stage 2 — cross-language C <-> Rust function/region mapping.

Builds the C call graph (via callgraph.py / libclang) and the Rust call graph (via the
`rust_callgraph` syn helper), then aligns the two by name. C2Rust preserves function names
(`#[no_mangle]`), so name matching recovers the correspondence almost for free — this stage
both produces the mapping and *measures* how well that assumption holds (spec §3, §11).

Output classifies every function as:
  - matched   : present in both (1:1 by name)
  - c_only    : in C, missing in Rust  (translation dropped / renamed it)
  - rust_only : in Rust, missing in C  (candidate absorbed helper — 1:N on the C side)

For each matched function it also reports whether the *call structure* agrees (same set of
matched callees), an early structural-distance signal that Stage 3 will quantify.

Usage:
  python3 tools/stu_selector/mapping.py \
    --compile-commands projects/qsort_example/build \
    --rust projects/qsort_example/translated/src/qsort.rs
"""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

# Import the C-side extractor as a library (shared graph algorithms).
sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402

_DEFAULT_RUST_BIN = (
    Path(__file__).resolve().parent / "rust_callgraph" / "target" / "release" / "rust_callgraph"
)


def build_c_graph(cc_dir: Path) -> dict:
    cgmod._configure_libclang()
    cg = cgmod.CallGraph()
    cgmod.parse_compile_commands(cc_dir, cg)
    return cgmod.condense(cg)


def build_rust_graph(rust_file: Path, rust_bin: Path) -> dict:
    raw = subprocess.run(
        [str(rust_bin), str(rust_file)], check=True, capture_output=True, text=True
    ).stdout
    data = json.loads(raw)

    cg = cgmod.CallGraph()
    for f in data["functions"]:
        cg.add_function(f["name"], str(rust_file), f["line"], True)
    known = {f["name"] for f in data["functions"]}
    for e in data["raw_edges"]:
        if e["to"] in known:
            cg.add_edge(e["from"], e["to"])
    for ind in data["indirect_calls"]:
        cg.add_indirect(ind["from"], ind.get("line", 0))
    return cgmod.condense(cg)


def _callees(result: dict, restrict: set[str]) -> dict[str, set[str]]:
    out: dict[str, set[str]] = {f["name"]: set() for f in result["functions"]}
    for e in result["edges"]:
        if e["to"] in restrict:
            out.setdefault(e["from"], set()).add(e["to"])
    return out


def align(c: dict, r: dict, rust_file: Path) -> dict:
    c_funcs = {f["name"]: f for f in c["functions"]}
    r_funcs = {f["name"]: f for f in r["functions"]}
    c_names, r_names = set(c_funcs), set(r_funcs)

    matched_names = c_names & r_names
    # Compare call structure only over names that exist on both sides.
    c_callees = _callees(c, matched_names)
    r_callees = _callees(r, matched_names)

    matched = []
    for n in sorted(matched_names):
        cc, rc = c_callees.get(n, set()), r_callees.get(n, set())
        matched.append({
            "name": n,
            "c_line": c_funcs[n]["line"],
            "rust_line": r_funcs[n]["line"],
            "c_recursive": c_funcs[n]["calls_self"],
            "rust_recursive": r_funcs[n]["calls_self"],
            "callee_agreement": cc == rc,
            "only_in_c_callees": sorted(cc - rc),
            "only_in_rust_callees": sorted(rc - cc),
        })

    n_c, n_r, n_m = len(c_names), len(r_names), len(matched_names)
    return {
        "matched": matched,
        "c_only": sorted(c_names - r_names),
        "rust_only": sorted(r_names - c_names),
        "summary": {
            "c_functions": n_c,
            "rust_functions": n_r,
            "matched": n_m,
            "name_match_coverage": round(n_m / n_c, 3) if n_c else 0.0,
            "structurally_agreeing": sum(1 for m in matched if m["callee_agreement"]),
            "rust_only_count": len(r_names - c_names),
        },
        "note": (
            "rust_only functions are candidate absorbed helpers (1:N on the C side); "
            "c_only functions were dropped or renamed by the translation."
        ),
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="C<->Rust function/region mapping")
    ap.add_argument("--compile-commands", required=True, help="Dir with compile_commands.json")
    ap.add_argument("--rust", required=True, help="Translated Rust .rs file")
    ap.add_argument("--rust-bin", default=str(_DEFAULT_RUST_BIN),
                    help="Path to the rust_callgraph binary")
    ap.add_argument("-o", "--out", help="Write JSON here (default: stdout)")
    args = ap.parse_args()

    rust_bin = Path(args.rust_bin)
    if not rust_bin.exists():
        print(f"[error] rust_callgraph binary not found at {rust_bin}\n"
              f"        build it: (cd tools/stu_selector/rust_callgraph && cargo build --release)",
              file=sys.stderr)
        return 1

    c = build_c_graph(Path(args.compile_commands))
    r = build_rust_graph(Path(args.rust), rust_bin)
    result = align(c, r, Path(args.rust))

    text = json.dumps(result, indent=2)
    if args.out:
        Path(args.out).write_text(text, encoding="utf-8")
        print(f"Wrote mapping to {args.out}")
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
