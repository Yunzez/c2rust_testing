#!/usr/bin/env python3
"""The cross-library RQ4 summary: one funnel table, one coverage table, one confirmation table,
assembled from every library's `cells.json` and the defect manifest -- so the five-library stop
("summarise funnel, coverage sets and defect clusters, then decide the figure") is one command.

usage: scripts/rq4/summary_all.py [--libs bzip2,genann,cjson,lil,tulip] [--out results/rq3_coverage/SUMMARY_ALL.md]

Rules carried from PROTOCOL.md: a tests column is shown only where the shipped suite passes
completely (acceptance baseline); raw region counts are per-translation identities and are never
compared across tools -- fractions and candidate counts are. Confirmation totals are the labelled
per-cell samples (200 per channel per boundary) unless a full confirmation exists.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RQ4 = ROOT / "results" / "rq3_coverage"
MANIFEST = ROOT / "results" / "rq4_effectiveness" / "defect_manifest.json"
TOOL_NAMES = {"c2rust": "c2rust", "laertes": "Laertes", "crown": "CROWN", "c2saferrust": "C2SaferRust",
              "sactor": "SACTOR", "ptrtrans": "PtrTrans"}
CONFIRMED = ("confirmed_divergence", "confirmed_termination")
# Defects whose FIRST evidence is an RQ4 cell (the manifest entry says so in `records`/`evidence`);
# every other id tied to a cell was catalogued earlier and re-found there.
NEW_IN_RQ4 = {"C8", "C9", "C10"}
NOISE = ("ub_associated", "ub_associated_termination", "ub_associated_value", "ub_gated",
         "out_of_contract_access", "instrument_only", "inconclusive", "inconclusive_wild_address",
         "not_reproducible")


def load_cells(lib: str) -> list[dict]:
    p = RQ4 / lib / "cells.json"
    return json.loads(p.read_text()) if p.exists() else []


def manifest_defects() -> list[dict]:
    """Defects plus held-out candidates (the latter tagged, so a cell whose confirmed divergences
    were classified as a candidate -- PtrTrans's stub -- does not read as unexplained)."""
    if not MANIFEST.exists():
        return []
    m = json.loads(MANIFEST.read_text())
    out = list(m.get("defects", []))
    for c in m.get("candidates", []):
        out.append(dict(c, id=f"{c['id']} (candidate)", family=c.get("status", "candidate")))
    return out


def defects_for(lib: str, tool: str, defects: list[dict]) -> list[str]:
    """Defect ids whose evidence names this RQ4 cell (found or re-found there)."""
    key = f"rq3_coverage/{lib}/{tool}"
    lib_names = {"cjson": ("cJSON",), "tulip": ("tulipindicators", "tulip")}
    out = []
    for d in defects:
        ev = (d.get("evidence") or "") + " " + (d.get("provenance_note") or "")
        if key in ev:
            out.append(d["id"])
    return out


def fmt_cov(x: dict, k: str) -> str:
    return f"{x[k]}/{x['universe']} ({x[k + '_cov']:.3f})"


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--libs", default="bzip2,genann,cjson,lil,tulip,qsort,urlparser,quadtree,lodepng,optipng")
    ap.add_argument("--out", default=str(RQ4 / "SUMMARY_ALL.md"))
    a = ap.parse_args()
    libs = [s.strip() for s in a.libs.split(",") if s.strip()]
    defects = manifest_defects()
    L: list[str] = []
    P = L.append
    P("# RQ4 — all libraries, one table each\n")
    P("Assembled by `scripts/rq4/summary_all.py` from every library's `cells.json` (itself from the "
      "cells' own files) and `results/rq4_effectiveness/defect_manifest.json`. Protocol: `PROTOCOL.md`. "
      "A library without a `cells.json` is listed as *in flight*.\n")

    # ---- 1. funnel
    P("## 1. Funnel per cell\n")
    P("| library | tool | matched | planned | built | executed | exported | corpus | term. cands | div. cands |")
    P("|---|---|---:|---:|---:|---:|---:|---:|---:|---:|")
    totals = dict(cells=0, matched=0, planned=0, built=0, executed=0, exported=0, corpus=0)
    for lib in libs:
        cells = load_cells(lib)
        if not cells:
            P(f"| {lib} | *in flight* | | | | | | | | |")
            continue
        for c in cells:
            g = lambda k: c.get(k, "—")
            P(f"| {lib} | {TOOL_NAMES.get(c['tool'], c['tool'])} | {g('matched')} | {g('planned')} | {g('built')} | "
              f"{g('executed')} | {g('exported')} | {g('corpus')} | {g('term_candidates')} | {g('div_candidates')} |")
            totals["cells"] += 1
            for k in ("matched", "planned", "built", "executed", "exported", "corpus"):
                totals[k] += c.get(k, 0) or 0
    P(f"\n{totals['cells']} cells: {totals['planned']} planned of {totals['matched']} matched boundaries, "
      f"{totals['built']} built, {totals['executed']} executed, {totals['exported']} with a coverage export; "
      f"{totals['corpus']} corpus inputs in total.\n")

    # ---- 2. coverage
    P("## 2. Coverage of the translation: shipped suite vs validator\n")
    P("Fractions are of the cell's universe (the passing suite's instrumented build where the suite is a "
      "baseline, else the translation's own rlib objects). `—` = the suite is not a baseline for that cell "
      "(TEST-UNAVAILABLE or TEST-FAILS): no tests column, never 0 %. Raw counts are per-translation identities "
      "and are not comparable across tools.\n")
    P("| library | tool | tests side | fn tests | fn ours | fn only-ours | reg tests | reg ours | reg only-ours |")
    P("|---|---|---|---:|---:|---:|---:|---:|---:|")
    for lib in libs:
        for c in load_cells(lib):
            if "function" not in c:          # a cell whose post-processing has not run (or died)
                P(f"| {lib} | {TOOL_NAMES.get(c['tool'], c['tool'])} | *incomplete* | | | | | | |")
                continue
            base = str(c.get("tests_side_mode", "")).startswith("measured") or c.get("tests") == "PASS" \
                or (c["function"].get("tests", 0) > 0)
            f, r = c["function"], c["region"]
            P(f"| {lib} | {TOOL_NAMES.get(c['tool'], c['tool'])} | {c.get('tests', '?')} | "
              f"{fmt_cov(f, 'tests') if base else '—'} | {fmt_cov(f, 'ours')} | {f['only_ours']} | "
              f"{fmt_cov(r, 'tests') if base else '—'} | {fmt_cov(r, 'ours')} | {r['only_ours']} |")
    P("")

    # ---- 3. confirmation
    P("## 3. Candidates, confirmation, defects\n")
    P("Confirmation totals are the cell's labelled sample (or its full confirmation where one exists). "
      "`confirmed` = `confirmed_divergence` + `confirmed_termination`; everything else is a recorded "
      "non-defect class. Defect ids are manifest rows whose evidence names this cell.\n")
    P("| library | tool | confirmed div. | confirmed term. | ub-associated | instrument-only / out-of-contract | inconclusive / not reproducible | defects (manifest) |")
    P("|---|---|---:|---:|---:|---:|---:|---|")
    grand = dict(cd=0, ct=0)
    for lib in libs:
        for c in load_cells(lib):
            t = (c.get("confirm") or {}).get("total") or {}
            cd, ct = t.get("confirmed_divergence", 0), t.get("confirmed_termination", 0)
            ub = sum(v for k, v in t.items() if k.startswith("ub_"))
            inst = t.get("instrument_only", 0) + t.get("out_of_contract_access", 0)
            inc = t.get("inconclusive", 0) + t.get("inconclusive_wild_address", 0) + t.get("not_reproducible", 0)
            ids = defects_for(lib, c["tool"], defects)
            grand["cd"] += cd
            grand["ct"] += ct
            P(f"| {lib} | {TOOL_NAMES.get(c['tool'], c['tool'])} | {cd} | {ct} | {ub} | {inst} | {inc} | "
              f"{', '.join(ids) if ids else ('—' if (cd or ct) == 0 else '*pending*')} |")
    P(f"\nConfirmed across all cells: {grand['cd']} value divergences, {grand['ct']} terminations, before "
      f"clustering (one site = one defect; a producer's crash counts once and blocks its dependants).\n")

    # ---- 4. manifest view
    P("## 4. Defects the RQ4 cells found or re-found\n")
    rows = [d for d in defects if "rq3_coverage/" in ((d.get("evidence") or "") + (d.get("provenance_note") or ""))
            and "(candidate)" not in d["id"]]
    P("| id | library × tool | family | found by |")
    P("|---|---|---|---|")
    for d in rows:
        P(f"| {d['id']} | {d['library']} × {d['tool']} | {d.get('family', '')} | "
          f"{'NEW — found by the RQ4 plan pipeline' if d['id'] in NEW_IN_RQ4 else 'catalogued earlier; re-found by the RQ4 cell'} |")
    P("")
    Path(a.out).write_text("\n".join(L) + "\n")
    print(f"wrote {a.out}: {totals['cells']} cells, {len(rows)} manifest rows tied to RQ4 cells")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
