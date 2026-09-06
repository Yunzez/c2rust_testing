#!/usr/bin/env python3
"""One row per (library, tool) cell: the funnel, the campaign, the four-set coverage, the tests
side, and the candidate counts -- read from what cell.py / replay_cell.py / confirm_cell.py /
c2r_coverage.py wrote. Prints Markdown; nothing here is computed, only collected.

usage: scripts/rq4/cell_table.py --lib bzip2 --cells <dir1> <dir2> ... [--tests-side <json>]
       (each cell dir must carry funnel.json and analysis/result.json)
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path


def load(p: Path):
    return json.loads(p.read_text()) if p.exists() else None


def row(cell: Path, tests_side: dict | None) -> dict:
    tool = cell.name.split("_")[-1]
    funnel = load(cell / "funnel.json") or []
    res = load(cell / "analysis" / "result.json")
    plans = load(cell / "plans.json") or []
    rep = load(cell / "divergences" / "summary.json") or []
    conf = load(cell / "confirm" / "summary.json") or load(cell / "confirm_sample" / "summary.json")
    built = [r for r in funnel if r.get("built")]
    executed = [r for r in built if (r.get("corpus") or 0) > 0]
    exported = [r for r in built if str(r.get("coverage", "")).startswith(("batch", "per-input"))]
    ts = (tests_side or {}).get(tool) or {}
    out = {
        "tool": tool,
        "matched": len(plans) or None,
        "planned": sum(1 for p in plans if p.get("status") == "planned") or len(funnel),
        "built": len(built), "executed": len(executed), "exported": len(exported),
        "corpus": sum(r.get("corpus") or 0 for r in funnel),
        "term_candidates": sum(r.get("artifacts") or 0 for r in funnel),
        "div_candidates": sum(r.get("candidates") or 0 for r in rep) if rep else None,
        "tests": (f"{ts.get('status')} {ts.get('passed')}/{(ts.get('passed') or 0)+(ts.get('failed') or 0)}"
                  if ts.get("passed") is not None else ts.get("status")),
    }
    if res:
        for k in ("function", "region"):
            x = res[k]
            out[k] = {"universe": x["total_in_scope"], "tests": x["covered_tests"],
                      "ours": x["covered_ours"], "both": x["covered_both"],
                      "only_tests": x["only_tests"], "only_ours": x["only_ours"],
                      "neither": x["covered_by_neither"],
                      "tests_cov": x["tests_coverage"], "ours_cov": x["ours_coverage"],
                      "sanity": all(x.get("sanity", {}).values()) if x.get("sanity") else None}
        out["tests_side_mode"] = res.get("tests_side")
    if conf:
        out["confirm"] = {"mode": conf.get("mode"), "total": conf.get("total")}
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--lib", required=True)
    ap.add_argument("--cells", nargs="+", required=True)
    ap.add_argument("--tests-side")
    ap.add_argument("--json")
    a = ap.parse_args()
    ts = load(Path(a.tests_side)) if a.tests_side else None
    rows = [row(Path(c), ts) for c in a.cells]
    if a.json:
        Path(a.json).write_text(json.dumps(rows, indent=1) + "\n")

    print(f"## {a.lib} — plan pipeline, one row per tool\n")
    print("| tool | tests side | matched | planned | built | executed | exported | corpus | "
          "term. cands | div. cands | fn universe | fn tests | fn ours | fn only-ours | "
          "reg universe | reg tests | reg ours | reg only-ours | sanity |")
    print("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|")
    for r in rows:
        f, g = r.get("function") or {}, r.get("region") or {}
        def cov(x, k):
            return f"{x[k]} ({x[k + '_cov']:.3f})" if x else "—"
        print(f"| {r['tool']} | {r['tests']} | {r['matched'] or '—'} | {r['planned']} | {r['built']} | "
              f"{r['executed']} | {r['exported']} | {r['corpus']} | {r['term_candidates']} | "
              f"{r['div_candidates'] if r['div_candidates'] is not None else '—'} | "
              f"{f.get('universe', '—')} | {cov(f, 'tests') if f and f['tests'] else '—'} | {cov(f, 'ours')} | "
              f"{f.get('only_ours', '—')} | {g.get('universe', '—')} | "
              f"{cov(g, 'tests') if g and g['tests'] else '—'} | {cov(g, 'ours')} | {g.get('only_ours', '—')} | "
              f"{'ok' if f.get('sanity') and g.get('sanity') else ('—' if not f else 'FAIL')} |")
    print("\n`—` in a tests column = TEST-FAILS / TEST-ADAPTER-FAILS: universe from a link-dead-code "
          "denominator, partition collapses to Ours / Neither (PROTOCOL.md §2). Never 0 %.")
    print("Raw region counts are per-translation identities and are NOT comparable across tools; "
          "compare the fractions and the candidate counts.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
