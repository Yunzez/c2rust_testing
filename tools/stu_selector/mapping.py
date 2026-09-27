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
# rust-analyzer-based whole-crate analyzer (multi-module, name/type resolved).
# Used for Cargo-crate inputs; the syn tool above is the legacy single-.rs fallback.
_ANALYZER_BIN = (
    Path(__file__).resolve().parent / "analyzer" / "target" / "release" / "analyzer"
)


def build_c_graph(cc_dir: Path) -> dict:
    cgmod._configure_libclang()
    cg = cgmod.CallGraph()
    cgmod.parse_compile_commands(cc_dir, cg)
    return cgmod.condense(cg)


def build_rust_graph(rust_file: Path, rust_bin: Path | None = None) -> dict:
    rust_file = Path(rust_file)
    # Route by input: a Cargo crate dir -> rust-analyzer analyzer (whole-crate,
    # multi-module, name/type resolved); a bare .rs file -> legacy syn tool.
    if rust_file.is_dir() and (rust_file / "Cargo.toml").exists():
        bin_path = _ANALYZER_BIN
    else:
        bin_path = Path(rust_bin) if rust_bin else _DEFAULT_RUST_BIN
    if not Path(bin_path).exists():
        raise FileNotFoundError(
            f"rust call-graph binary not found at {bin_path}; build it: "
            f"(cd {Path(bin_path).parents[2]} && cargo build --release)"
        )

    proc = subprocess.run([str(bin_path), str(rust_file)], capture_output=True, text=True)
    if proc.returncode == 2:
        # analyzer load_failed: the translator produced a crate that does not even
        # load. Per design that is a finding about the translator, not a tool crash.
        raise RuntimeError(f"rust crate failed to load (translator defect): {proc.stdout.strip()}")
    if proc.returncode != 0:
        raise RuntimeError(
            f"rust call-graph extraction failed ({bin_path}): "
            f"{proc.stderr.strip() or proc.stdout.strip()}"
        )
    data = json.loads(proc.stdout)

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
    ap.add_argument("--rust", required=True,
                    help="Translated Rust .rs file (legacy syn) OR Cargo crate dir (rust-analyzer)")
    ap.add_argument("--rust-bin", default=str(_DEFAULT_RUST_BIN),
                    help="Path to the legacy syn rust_callgraph binary (used for .rs inputs)")
    ap.add_argument("-o", "--out", help="Write JSON here (default: stdout)")
    args = ap.parse_args()

    c = build_c_graph(Path(args.compile_commands))
    r = build_rust_graph(Path(args.rust), Path(args.rust_bin))
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
