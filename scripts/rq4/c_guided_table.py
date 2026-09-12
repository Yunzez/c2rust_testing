#!/usr/bin/env python3
"""Aggregate frozen controlled-guidance cells without redefining metrics post hoc."""
import argparse
import json
from pathlib import Path


def pct(side):
    value = side.get("region_reach") if side else None
    return "--" if value is None else f"{100 * value:.1f}"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", required=True, help="directory containing <cell>/FINAL/result.json")
    ap.add_argument("--out", required=True)
    args = ap.parse_args()
    root, out = Path(args.root), Path(args.out)
    rows, invalid = [], []
    for result_path in sorted(root.glob("*/FINAL/result.json")):
        result = json.load(open(result_path))
        done_path = result_path.parent / "DONE.json"
        done = json.load(open(done_path)) if done_path.exists() else {}
        if not done.get("valid") or not all(result.get("checks", {}).values()):
            invalid.append(result["cell"])
            continue
        matrix = result["matrix"]
        matched = result.get("matched", {})
        rows.append({
            "cell": result["cell"],
            "boundaries": len(result["boundaries"]),
            "rust_guided": matrix["rust"],
            "c_guided": matrix["c"],
            "union": matrix["union"],
            "sets_rust": matched.get("rust"),
            "sets_c": matched.get("c"),
            "sets_union": matched.get("union"),
            "c2rust_control": result.get("c2rust_control"),
        })
    payload = {"schema": 1, "valid_cells": len(rows), "invalid_cells": invalid, "rows": rows}
    out.parent.mkdir(parents=True, exist_ok=True)
    out.with_suffix(".json").write_text(json.dumps(payload, indent=1) + "\n")
    lines = ["# Controlled C/Rust-guided reach", "",
             "| cell | harnesses | R-guided Rust fn | C-guided Rust fn | R-guided Rust reg | C-guided Rust reg | C fn R/C |",
             "|---|---:|---:|---:|---:|---:|---:|"]
    for row in rows:
        rr, cc = row["rust_guided"], row["c_guided"]
        lines.append(f"| {row['cell']} | {row['boundaries']} | "
                     f"{rr['Rust']['functions_reached']}/{rr['Rust']['functions_total']} | "
                     f"{cc['Rust']['functions_reached']}/{cc['Rust']['functions_total']} | "
                     f"{pct(rr['Rust'])} | {pct(cc['Rust'])} | "
                     f"{rr['C']['functions_reached']}/{cc['C']['functions_reached']} |")
    if invalid:
        lines += ["", "Invalid cells (excluded): " + ", ".join(invalid)]
    out.write_text("\n".join(lines) + "\n")


if __name__ == "__main__":
    main()
