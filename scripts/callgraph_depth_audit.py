#!/usr/bin/env python3

"""Layer-2 Step 0 — call-graph plumbing + depth audit.

Two questions, answered by REUSING the existing tooling (this also validates that the plumbing
actually runs end-to-end on the corpus):

  1. Plumbing: does `mapping.build_c_graph` / `build_rust_graph` / `align` run on each pair, and how
     healthy is the C<->Rust name mapping (the c2rust "#[no_mangle] => free mapping" assumption)?
  2. Depth: does the corpus have call-graph DEPTH? Frontier selection only matters when "which
     abstraction level to test" is a real choice — i.e. the SCC DAG has a longest call chain >= 3
     with at least one INTERNAL node (indeg>0 AND outdeg>0). A flat program (all roots/leaves)
     collapses root/leaf/all/frontier strategies into the same answer.

Output: results/callgraph_depth_audit_v1.md + a console summary. Read-only; no model/generator touched.

Usage: python3 scripts/callgraph_depth_audit.py [--only prog,prog]
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import mapping as mapmod  # noqa: E402


def longest_chain(cond: dict) -> int:
    """Longest path (in #SCC nodes) over the condensation DAG = #levels in the deepest call chain."""
    succ: dict[int, list[int]] = {}
    for a, b in cond["scc_dag_edges"]:
        succ.setdefault(a, []).append(b)
    dp: dict[int, int] = {rec["id"]: 1 for rec in cond["sccs"]}
    for u in cond["topo_order"]:           # callers before callees
        for v in succ.get(u, []):
            if dp[u] + 1 > dp[v]:
                dp[v] = dp[u] + 1
    return max(dp.values(), default=0)


def graph_shape(cond: dict) -> dict:
    sccs = cond["sccs"]
    ids = [r["id"] for r in sccs]
    indeg = {i: 0 for i in ids}
    outdeg = {i: 0 for i in ids}
    for a, b in cond["scc_dag_edges"]:
        outdeg[a] += 1
        indeg[b] += 1
    internal = sum(1 for i in ids if indeg[i] > 0 and outdeg[i] > 0)
    return {
        "n_funcs": len(cond["functions"]),
        "n_edges": len(cond["edges"]),
        "n_sccs": len(sccs),
        "n_recursive_scc": sum(1 for r in sccs if r["recursive"]),
        "n_indirect": len(cond.get("indirect_calls", [])),
        "depth": longest_chain(cond),
        "max_fanout": max(outdeg.values(), default=0),
        "n_internal": internal,           # nodes that are neither pure root nor pure leaf
        "n_roots": sum(1 for i in ids if indeg[i] == 0),
        "n_leaves": sum(1 for i in ids if outdeg[i] == 0),
    }


def audit_pair(pair: Path) -> dict:
    name = pair.name
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"), None)
    if not cc.exists() or rs is None:
        return {"program": name, "error": "no build/ or translated/*.rs"}
    try:
        c = mapmod.build_c_graph(cc)
        r = mapmod.build_rust_graph(rs, mapmod._DEFAULT_RUST_BIN)
        m = mapmod.align(c, r, rs)
    except Exception as e:  # noqa: BLE001 - plumbing health is exactly what we are measuring
        return {"program": name, "error": f"{type(e).__name__}: {e}"}
    cs = graph_shape(c)
    rs_shape = graph_shape(r)
    s = m["summary"]
    # A frontier choice is REAL only if the C call graph has a >=3 chain with an internal node.
    frontier_choice = cs["depth"] >= 3 and cs["n_internal"] >= 1
    return {
        "program": name, "error": None,
        "c": cs, "r": rs_shape,
        "map_cov": s["name_match_coverage"], "matched": s["matched"],
        "c_only": len(m["c_only"]), "rust_only": s["rust_only_count"],
        "struct_agree": s["structurally_agreeing"],
        "frontier_choice": frontier_choice,
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="call-graph plumbing + depth audit")
    ap.add_argument("--only", default=None, help="comma list of program names")
    args = ap.parse_args()
    only = set(args.only.split(",")) if args.only else None

    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_")
                   and (only is None or p.name in only))
    rows = [audit_pair(p) for p in pairs]
    ok = [r for r in rows if not r["error"]]
    bad = [r for r in rows if r["error"]]
    ok.sort(key=lambda r: (-r["c"]["depth"], -r["c"]["n_funcs"]))

    md = ["# Call-graph plumbing + depth audit (Layer 2, Step 0)\n",
          f"{len(ok)}/{len(rows)} pairs processed; {len(bad)} plumbing failures.\n",
          "**Frontier choice** = C call graph has a longest chain >= 3 AND >= 1 internal node "
          "(neither pure root nor pure leaf). Only these programs make root/leaf/all/frontier "
          "strategies differ — i.e. only these can populate the headline table meaningfully.\n",
          "| program | C funcs | C depth | C internal | maxfan | recurse | indirect | "
          "map cov | rust_only | frontier? |",
          "|---|--:|--:|--:|--:|--:|--:|--:|--:|:--:|"]
    n_choice = 0
    for r in ok:
        c = r["c"]
        n_choice += int(r["frontier_choice"])
        md.append(f"| {r['program']} | {c['n_funcs']} | {c['depth']} | {c['n_internal']} | "
                  f"{c['max_fanout']} | {c['n_recursive_scc']} | {c['n_indirect']} | "
                  f"{r['map_cov']:.2f} | {r['rust_only']} | "
                  f"{'YES' if r['frontier_choice'] else '—'} |")
    if bad:
        md += ["\n## Plumbing failures\n"] + [f"- `{r['program']}`: {r['error']}" for r in bad]

    depths = [r["c"]["depth"] for r in ok]
    md += ["\n## Verdict\n",
           f"- **Programs with a real frontier choice: {n_choice}/{len(ok)}.** "
           f"The rest are too shallow (depth<3 or no internal node) for frontier selection to differ "
           f"from root/leaf/all.",
           f"- C call-graph depth: max {max(depths, default=0)}, "
           f"median {sorted(depths)[len(depths)//2] if depths else 0}.",
           f"- Mapping health: mean name-match coverage "
           f"{round(sum(r['map_cov'] for r in ok)/len(ok), 3) if ok else 0} "
           f"(c2rust #[no_mangle] => near-free mapping); "
           f"total rust_only (absorbed-helper candidates) {sum(r['rust_only'] for r in ok)}.",
           f"- Plumbing: {len(ok)}/{len(rows)} pairs ran clean."
           + (f" FAILURES on {len(bad)} (see above)." if bad else "")]
    out = ROOT / "results" / "callgraph_depth_audit_v1.md"
    out.write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"wrote {out}")
    print(f"\nframe choice: {n_choice}/{len(ok)} programs have a real frontier choice; "
          f"depth max {max(depths, default=0)}; plumbing {len(ok)}/{len(rows)} clean"
          + (f"; {len(bad)} FAILED" if bad else ""))
    for r in ok[:12]:
        c = r["c"]
        print(f"  {r['program']:20} funcs={c['n_funcs']:2} depth={c['depth']} "
              f"internal={c['n_internal']} mapcov={r['map_cov']:.2f} "
              f"{'FRONTIER' if r['frontier_choice'] else ''}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
