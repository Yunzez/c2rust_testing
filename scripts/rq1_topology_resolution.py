#!/usr/bin/env python3
"""RQ1 threat measurement — call-graph resolution on translated crates.

For each Rust artifact, run the rust-analyzer-based analyzer and report:
  fns                 functions defined in the crate
  unique_local_edges  deduplicated (from, to) call edges with both endpoints local
  unique_local_noself same, self-loops removed  (the density topology propagation consumes)
  edges_per_fn        unique_local_noself / fns
  local_sites / nonlocal_sites   raw call sites by target locality
  unresolved          `indirect_calls` (call_unresolved)
  unresolved_rate     unresolved / (all sites + unresolved)

Raw analyzer output per artifact and the tool-version block are written next to the table
(results/rq1_matching/raw/topology/). Companion note: results/rq1_matching/topology_resolution.md.

Usage:  python3 scripts/rq1_topology_resolution.py [--out results/rq1_matching/rows/topology.json]
"""
import argparse, hashlib, json, os, subprocess, sys

ROOT = "/home/yunzez/c2rust_testing"
AN = f"{ROOT}/tools/stu_selector/analyzer/target/release/analyzer"
FW = f"{ROOT}/tools/frameworks"
D = f"{FW}/ptrtrans_rebuild/PtrTrans-C2Rust/dataset"
CS = f"{FW}/c2saferrust/laertes_benchmarks"
CB = f"{FW}/crown/benchmark"
CROWN_R = f"{FW}/crown/results"
RAW = f"{ROOT}/results/rq1_matching/raw/topology"

CASES = [
    # (label, crate, translator, compiles, is_control)
    ("qsort_ptrtrans",     f"{D}/PA_trans_projects/qsort",   "PtrTrans", True,  False),
    ("quadtree_ptrtrans",  f"{D}/Trans_C-Rust-KG/quadtree",  "PtrTrans", True,  False),
    ("bzip2_ptrtrans",     f"{D}/PA_trans_projects/bzip2",   "PtrTrans", False, False),
    ("lodepng_ptrtrans",   f"{D}/PA_trans_projects/lodepng", "PtrTrans", False, False),
    ("bzip2_c2rust",       f"{CS}/bzip2",       "c2rust",  True, True),
    ("bzip2_c2rust_crownin", f"{CB}/bzip2",     "c2rust",  True, True),
    ("bzip2_crown",        f"{CROWN_R}/bzip2",  "CROWN",   True, True),
    ("lodepng_c2rust_crownin", f"{CB}/lodepng", "c2rust",  True, True),
    ("lodepng_crown",      f"{CROWN_R}/lodepng", "CROWN",  True, True),
    ("quadtree_c2rust_crownin", f"{CB}/quadtree", "c2rust", True, True),
    ("quadtree_crown",     f"{CROWN_R}/quadtree", "CROWN", True, True),
    ("qsort_c2rust",       f"{CS}/qsort",       "c2rust",  True, True),
]


def _git(*a):
    return subprocess.run(["git", "-C", ROOT] + list(a), capture_output=True, text=True).stdout.strip()


def versions():
    return {
        "repo_head": _git("rev-parse", "--short", "HEAD"),
        "analyzer_src_commit": _git("log", "-1", "--format=%h", "--", "tools/stu_selector/analyzer/src"),
        "analyzer_bin_sha256": hashlib.sha256(open(AN, "rb").read()).hexdigest()[:16],
    }


def topo(label, crate):
    if not os.path.isdir(crate):
        return {"label": label, "status": "MISSING"}
    r = subprocess.run([AN, crate], capture_output=True, text=True, timeout=900)
    if r.returncode != 0:
        return {"label": label, "status": "ANALYZER-FAIL", "stderr": r.stderr[-400:]}
    os.makedirs(RAW, exist_ok=True)
    open(f"{RAW}/{label}.analyzer.json", "w").write(r.stdout)
    d = json.loads(r.stdout)
    local = {f["name"] for f in d["functions"]}
    leaf = {n.split("::")[-1] for n in local}

    def is_local(t):
        return t in local or t.split("::")[-1] in leaf

    edges = d["raw_edges"]
    unres = len(d["indirect_calls"])
    local_sites = [e for e in edges if is_local(e["to"])]
    uniq_local = {(e["from"], e["to"]) for e in local_sites}
    uniq_noself = {(a, b) for a, b in uniq_local if a != b}
    total_sites = len(edges) + unres
    n = len(local)
    return {
        "label": label, "status": "ok", "fns": n,
        "unique_local_edges": len(uniq_local),
        "unique_local_noself": len(uniq_noself),
        "edges_per_fn": round(len(uniq_noself) / n, 2) if n else 0,
        "local_sites": len(local_sites),
        "nonlocal_sites": len(edges) - len(local_sites),
        "unresolved": unres,
        "unresolved_rate": round(unres / total_sites, 3) if total_sites else 0,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=f"{ROOT}/results/rq1_matching/rows/topology.json")
    args = ap.parse_args()
    out = {"_meta": {"runner": "scripts/rq1_topology_resolution.py", "tool_versions": versions(),
                     "raw_dir": RAW.replace(ROOT + "/", "")}, "rows": []}
    hdr = f"{'artifact':<26}{'st':<8}{'fns':>5}{'uniqE':>7}{'E≠self':>8}{'E/fn':>6}{'local':>7}{'nonloc':>8}{'unres':>7}{'rate':>7}"
    print(hdr); print("-" * len(hdr))
    for label, crate, tool, compiles, ctrl in CASES:
        row = topo(label, crate)
        row.update({"crate": crate.replace(ROOT + "/", ""), "translator": tool,
                    "compiles": compiles, "control": ctrl})
        out["rows"].append(row)
        if row["status"] != "ok":
            print(f"{label:<26}{row['status']:<8}"); continue
        print(f"{label:<26}{'ok':<8}{row['fns']:>5}{row['unique_local_edges']:>7}"
              f"{row['unique_local_noself']:>8}{row['edges_per_fn']:>6}{row['local_sites']:>7}"
              f"{row['nonlocal_sites']:>8}{row['unresolved']:>7}{row['unresolved_rate']:>7}")
    os.makedirs(os.path.dirname(args.out), exist_ok=True)
    json.dump(out, open(args.out, "w"), indent=1)
    print(f"\nwrote {args.out}; raw analyzer outputs in {RAW}")


if __name__ == "__main__":
    main()
