#!/usr/bin/env python3
"""RQ1 group A — aggregate rows/group_a_full.json into the paper's per-library table.

Table `tab:matching-accuracy` reports, per library, pooled over that library's name-preserving
artifacts:  Pairs | Matcher P / R (forced) | Deploy P@C (accepted precision @ coverage).
This script is the ONLY sanctioned path from the row JSON to those numbers; it also emits
micro / macro aggregates per split with artifact and library counts.

Pooling rule: pairs = sum of `scorable`; P = sum(correct)/sum(matched); R = sum(correct)/pairs;
deploy P = sum(dep.correct)/sum(dep.matched); C = sum(accepted-on-truth)/pairs.
Macro = unweighted mean over artifacts.

Every integer is re-derived from the archived raw files (`raw/group_a/<key>/truth.json` +
`matcher_output.json`) and cross-checked against the row summary; a mismatch aborts, so the
table can only be produced from a consistent rows/raw pair.

Usage: python3 scripts/rq1_group_a_table.py [--rows results/rq1_matching/rows/group_a_full.json]
Writes rows/group_a_table.json and prints markdown.
"""
import argparse, json, os
from collections import defaultdict

ROOT = "/home/yunzez/c2rust_testing"
LIB_ORDER = ["qsort", "urlparser", "quadtree", "genann", "cjson", "lil", "lodepng", "bzip2",
             "tulip", "optipng"]
DISPLAY = {"cjson": "cJSON", "tulip": "tulip"}


def leaf(n):
    return n.rsplit("::", 1)[-1]


def counts_from_raw(raw_dir, r):
    """Recount forced/deployment integers from the archived truth + matcher output."""
    truth = json.load(open(f"{raw_dir}/truth.json"))["truth"]
    mo = json.load(open(f"{raw_dir}/matcher_output.json"))
    got = {cc: rr for (cc, rr, _s, _k) in mo["forced"]}
    dgot = {cc: rr for (cc, rr, _s, _k) in mo["deployment"]}
    c = {
        "scorable": len(truth),
        "f_matched": len(got),
        "f_correct": sum(1 for k, v in truth.items() if leaf(got.get(k, "")) == leaf(v)),
        "d_matched": len(dgot),
        "d_correct": sum(1 for k, v in truth.items() if leaf(dgot.get(k, "")) == leaf(v)),
        "d_accepted_on_truth": sum(1 for k in truth if k in dgot),
        "d_ambiguous": len(mo["deployment_ambiguous"]),
    }
    f, d = r["forced"], r["deployment"]
    expect = {"scorable": r["scorable"], "f_matched": f["matched"], "f_correct": f["correct"],
              "d_matched": d["matched"], "d_correct": d["correct"], "d_ambiguous": d["ambiguous"]}
    bad = {k: (c[k], expect[k]) for k in expect if c[k] != expect[k]}
    if bad or abs(c["d_accepted_on_truth"] / c["scorable"] - d["coverage"]) > 5e-4:
        raise SystemExit(f"{r['_key']}: raw archive disagrees with row summary: {bad} "
                         f"coverage raw={c['d_accepted_on_truth']}/{c['scorable']} row={d['coverage']}")
    return c


def pool(rows):
    s = defaultdict(float)
    for r in rows:
        c = r["_counts"]
        s["pairs"] += c["scorable"]
        s["ambiguous_truth"] += r.get("ambiguous_truth", 0)
        for k in ("f_matched", "f_correct", "d_matched", "d_correct", "d_accepted_on_truth", "d_ambiguous"):
            s[k] += c[k]
    n = len(rows)
    out = {
        "artifacts": n, "libraries": len({r["_lib"] for r in rows}),
        "pairs": int(s["pairs"]), "ambiguous_truth": int(s["ambiguous_truth"]),
        "micro": {
            "precision": s["f_correct"] / s["f_matched"] if s["f_matched"] else None,
            "recall": s["f_correct"] / s["pairs"],
            "deploy_precision": s["d_correct"] / s["d_matched"] if s["d_matched"] else None,
            "coverage": s["d_accepted_on_truth"] / s["pairs"],
            "abstention": s["d_ambiguous"] / s["pairs"],
        },
        "macro": {
            "precision": sum(r["forced"]["precision"] for r in rows) / n,
            "recall": sum(r["forced"]["recall"] for r in rows) / n,
            "deploy_precision": sum(r["deployment"]["precision"] or 0 for r in rows) / n,
            "coverage": sum(r["deployment"]["coverage"] for r in rows) / n,
            "abstention": sum(r["deployment"]["abstention_rate"] for r in rows) / n,
        },
        "artifact_keys": sorted(r["_key"] for r in rows),
        "per_artifact_counts": {r["_key"]: r["_counts"] for r in rows},
    }
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--rows", default=f"{ROOT}/results/rq1_matching/rows/group_a_full.json")
    ap.add_argument("--out", default=f"{ROOT}/results/rq1_matching/rows/group_a_table.json")
    a = ap.parse_args()
    data = json.load(open(a.rows))
    rows, errors = [], {}
    for k, v in data.items():
        if k.startswith("_"):
            continue
        if "error" in v:
            errors[k] = v["error"]; continue
        v = dict(v); v["_key"] = k; v["_lib"] = k.split("__")[0]
        raw_dir = f"{ROOT}/{data['_meta']['raw_dir']}/{k}"
        if not os.path.isdir(raw_dir):
            raise SystemExit(f"{k}: no raw archive at {raw_dir}")
        v["_counts"] = counts_from_raw(raw_dir, v)
        rows.append(v)

    by_lib = defaultdict(list)
    for r in rows:
        by_lib[r["_lib"]].append(r)
    table = {}
    print("| library | split | artifacts | pairs | ambiguous | matcher P / R | deploy P@C | abstention |")
    print("|---|---|---:|---:|---:|---|---|---:|")
    for lib in LIB_ORDER + sorted(set(by_lib) - set(LIB_ORDER)):
        if lib not in by_lib:
            table[lib] = {"status": "TBD", "artifacts": 0, "pairs": 0}
            print(f"| {DISPLAY.get(lib, lib)} | | 0 | TBD | | TBD | TBD | |")
            continue
        p = pool(by_lib[lib]); p["split"] = by_lib[lib][0]["split"]
        table[lib] = p
        m = p["micro"]
        print(f"| {DISPLAY.get(lib, lib)} | {p['split']} | {p['artifacts']} | {p['pairs']} | "
              f"{p['ambiguous_truth']} | {m['precision']:.3f} / {m['recall']:.3f} | "
              f"{m['deploy_precision']:.3f}@{m['coverage']:.3f} | {m['abstention']:.3f} |")

    # Paper rule (tab:matching-accuracy caption, 2026-09-01): average first across the
    # available tool outputs of each library, then equally across libraries.
    paper = {}
    for lib in LIB_ORDER:
        sel = by_lib.get(lib, [])
        if not sel:
            continue
        n = len(sel)
        paper[lib] = {
            "tools": n, "pairs": sum(r["_counts"]["scorable"] for r in sel),
            "precision": sum(r["forced"]["precision"] for r in sel) / n,
            "recall": sum(r["forced"]["recall"] for r in sel) / n,
            "deploy_precision": sum(r["deployment"]["precision"] or 0 for r in sel) / n,
            "coverage": sum(r["deployment"]["coverage"] for r in sel) / n,
            "artifact_keys": sorted(r["_key"] for r in sel),
        }
    L = len(paper)
    paper_overall = {"libraries": L, "tool_outputs": sum(p["tools"] for p in paper.values()),
                     "pairs": sum(p["pairs"] for p in paper.values())}
    for k in ("precision", "recall", "deploy_precision", "coverage"):
        paper_overall[k] = sum(p[k] for p in paper.values()) / L
    print("\nPaper rule (mean over tools per library, then mean over libraries):")
    print("| library | tools | pairs | matcher P / R | deploy P@C |")
    print("|---|---:|---:|---|---|")
    for lib, p in paper.items():
        print(f"| {DISPLAY.get(lib, lib)} | {p['tools']} | {p['pairs']} | "
              f"{p['precision']:.3f} / {p['recall']:.3f} | {p['deploy_precision']:.3f}@{p['coverage']:.3f} |")
    o = paper_overall
    print(f"| **Overall** | {o['tool_outputs']} | {o['pairs']} | {o['precision']:.3f} / {o['recall']:.3f} | "
          f"{o['deploy_precision']:.3f}@{o['coverage']:.3f} |")

    aggr = {}
    for name, sel in (("eval", [r for r in rows if r["split"] == "eval"]),
                      ("dev", [r for r in rows if r["split"] == "dev"]),
                      ("all", rows)):
        if sel:
            aggr[name] = pool(sel)
    print()
    print("| set | artifacts | libraries | pairs | micro P / R | macro P / R | micro cov | micro abst | micro deploy P |")
    print("|---|---:|---:|---:|---|---|---:|---:|---:|")
    for name, p in aggr.items():
        m, M = p["micro"], p["macro"]
        print(f"| {name} | {p['artifacts']} | {p['libraries']} | {p['pairs']} | "
              f"{m['precision']:.3f} / {m['recall']:.3f} | {M['precision']:.3f} / {M['recall']:.3f} | "
              f"{m['coverage']:.3f} | {m['abstention']:.3f} | {m['deploy_precision']:.3f} |")
    if errors:
        print("\nerrors:", json.dumps(errors, indent=1))

    json.dump({"_meta": {"source_rows": a.rows.replace(ROOT + "/", ""),
                         "source_meta": data.get("_meta"),
                         "pooling": "see docstring of scripts/rq1_group_a_table.py"},
               "per_library": table, "aggregates": aggr,
               "paper_rule": {"per_library": paper, "overall": paper_overall,
                              "rule": "mean over tool outputs within a library, then mean over libraries; "
                                      "pairs are evidence volume only"},
               "errors": errors},
              open(a.out, "w"), indent=1)
    print(f"\nwrote {a.out}")


if __name__ == "__main__":
    main()
