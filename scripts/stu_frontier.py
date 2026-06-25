#!/usr/bin/env python3

"""Layer 2, Step 2 — STU frontier selector v1 (hard-threshold, bottom-up antichain).

Reuses the existing plumbing (mapping / callgraph / census constructibility / risk_features). NO
model training and NO weighted objective (deferred until G3 gives a calibration curve, per
docs/roadmap_frontier.md). The risk score is a FIXED, interpretable static rule built from the two
negative mechanisms we already characterized:

    risk(f) = BLOCKED  if f is not constructible (can't even build a differential harness)
            = RISKY    if f has an UNGUARDED signed UB op  (intrinsic-UB mechanism), OR
                          an UNMASKED struct-field index    (isolation-invariant mechanism)
            = OK       otherwise
    threshold T: only OK passes.

Region rule (over the C SCC DAG, bottom-up):
    valid_region(node) = every member is mapped(C<->Rust) AND risk == OK
                         AND every DAG-successor (callee SCC) is valid_region
Rust-only helpers do not appear as C nodes, so they are absorbed implicitly (reported as info).
Frontier = the MAXIMAL valid_region antichain (highest trustworthy regions).

It also computes the Step-3 comparison vs root / all-constructible / leaf-only, on metrics that need
NO fuzzing: #harness, covered funcs, and risk-exposed harnesses (a harness whose reachable region
contains a RISKY/BLOCKED node — i.e. a likely false-divergence source).

Usage: python3 scripts/stu_frontier.py [--only prog,prog]
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
sys.path.insert(0, str(ROOT / "scripts"))
import mapping as mapmod          # noqa: E402
import risk_features as rfmod     # noqa: E402
import harvest_census as census   # noqa: E402


def risk_class(rf: dict, constructible: bool) -> tuple[str, str]:
    if not constructible:
        return "BLOCKED", "not constructible"
    ub = (rf.get("rf_div_mod", 0) + rf.get("rf_shift", 0) + rf.get("rf_compound_arith", 0)
          + rf.get("rf_mul", 0) + rf.get("rf_negate", 0)) > 0
    guarded = bool(rf.get("rf_nonzero_guard", 0)) or bool(rf.get("rf_width_guard", 0))
    if ub and rf.get("rf_signed", 0) and not guarded:
        return "RISKY", "unguarded signed UB op (intrinsic-UB risk)"
    if rf.get("rf_unmasked_field_index", 0):
        return "RISKY", "unmasked struct-field index (isolation risk)"
    return "OK", ""


def reachable(start: int, succ: dict[int, list[int]]) -> set[int]:
    seen, stack = set(), [start]
    while stack:
        n = stack.pop()
        if n in seen:
            continue
        seen.add(n)
        stack.extend(succ.get(n, []))
    return seen


def analyze(pair: Path) -> dict:
    name = pair.name
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"), None)
    if not cc.exists() or rs is None:
        return {"program": name, "error": "no build/translated"}
    try:
        c = mapmod.build_c_graph(cc)
        r = mapmod.build_rust_graph(rs, mapmod._DEFAULT_RUST_BIN)
        amap = mapmod.align(c, r, rs)
        sigs = census.collect_signatures(cc)
        rfs = rfmod.risk_features_for_pair(cc)
    except Exception as e:  # noqa: BLE001
        return {"program": name, "error": f"{type(e).__name__}: {e}"}

    matched = {m["name"] for m in amap["matched"]}
    # per-function risk
    fn_risk, fn_reason = {}, {}
    for f in c["functions"]:
        n = f["name"]
        sig = sigs.get(n)
        constructible = sig is not None and census.constructibility(sig)[0] == "SUPPORTED"
        cls, why = risk_class(rfs.get(n, {}), constructible)
        if n not in matched:
            cls, why = "BLOCKED", "not mapped C<->Rust"
        fn_risk[n], fn_reason[n] = cls, why

    # SCC DAG
    sccs = c["sccs"]
    members = {rec["id"]: rec["members"] for rec in sccs}
    succ: dict[int, list[int]] = {rec["id"]: [] for rec in sccs}
    indeg = {rec["id"]: 0 for rec in sccs}
    for a, b in c["scc_dag_edges"]:
        succ[a].append(b)
        indeg[b] += 1
    # Selectability — corrected risk PROPAGATION (the v1 conceptual fix):
    #   * constructibility/mapping is a STANDALONE-node property. A non-constructible helper does NOT
    #     invalidate a constructible PARENT that builds it internally, so BLOCKED does NOT propagate up
    #     — it only disqualifies the node itself from being a frontier leaf.
    #   * a RISKY node (intrinsic-UB / isolation mechanism) DOES propagate: fuzzing any ancestor can
    #     trigger the false divergence, so every region reaching it is untrustworthy.
    constructible_mapped = {sid: all(fn_risk[m] != "BLOCKED" for m in mem)
                            for sid, mem in members.items()}

    def has_risky(sid: int) -> bool:
        return any(fn_risk[m] == "RISKY" for s in reachable(sid, succ) for m in members[s])

    def selectable(sid: int) -> bool:
        return constructible_mapped[sid] and not has_risky(sid)

    # matched-function coverage of a region = matched funcs reachable from it (incl self)
    def cover(sid: int) -> set[str]:
        out = set()
        for s in reachable(sid, succ):
            out |= {m for m in members[s] if m in matched}
        return out

    def region_has_risk(sid: int) -> bool:  # likely false-divergence source = a reachable RISKY node
        return has_risky(sid)

    # STU frontier = maximal valid_region antichain, top-down from roots
    roots = [sid for sid in members if indeg[sid] == 0]
    leaves = [sid for sid in members if not succ[sid]]
    selected: list[int] = []
    sink_reasons: list[str] = []
    visited: set[int] = set()
    def descend(sid: int):
        if sid in visited:
            return
        visited.add(sid)
        if selectable(sid):
            selected.append(sid)
            return
        # not a frontier STU here — record why, then look lower for cleaner regions
        head = members[sid][0]
        risky = [(m, fn_reason[m]) for s in reachable(sid, succ)
                 for m in members[s] if fn_risk[m] == "RISKY"]
        if risky:
            m, why = risky[0]
            sink_reasons.append(f"{head}: sunk past RISKY {m} ({why})")
        elif not constructible_mapped[sid]:
            sink_reasons.append(f"{head}: not constructible as a standalone boundary")
        for s in succ[sid]:
            descend(s)
    for sid in sorted(roots):
        descend(sid)

    # --- selector v2: "guarded rise" ---
    # A RISKY callee can be tolerated at a HIGHER boundary if the call to it is shielded by an input
    # clamp (cheap proxy: a direct caller bounds a value against a constant, rf_input_clamp). This lets
    # the frontier RISE to the boundary that constrains the risky callee's input domain instead of
    # sinking/collapsing (v1). KNOWN LIMITATION: "direct-caller clamp" is coarse — it does not verify
    # the clamp actually bounds the specific value reaching the risky op (the precise version is
    # interprocedural range/precondition analysis, deferred; G3 measures it empirically).
    fn_clamp = {f["name"]: rfs.get(f["name"], {}).get("rf_input_clamp", 0) > 0 for f in c["functions"]}
    callers: dict[str, set] = {f["name"]: set() for f in c["functions"]}
    for e in c["edges"]:
        if e["to"] in callers:
            callers[e["to"]].add(e["from"])

    def shielded(r: str) -> bool:
        return any(fn_clamp.get(cl) for cl in callers.get(r, ()))

    def own_risky(sid: int) -> bool:
        return any(fn_risk[m] == "RISKY" for m in members[sid])

    def has_unshielded_risky(sid: int) -> bool:
        return any(fn_risk[m] == "RISKY" and not shielded(m)
                   for s in reachable(sid, succ) for m in members[s])

    def selectable_v2(sid: int) -> bool:  # rise unless an UNSHIELDED risky is reachable / it is itself risky
        return constructible_mapped[sid] and not own_risky(sid) and not has_unshielded_risky(sid)

    selected_v2: list[int] = []
    visited2: set[int] = set()
    def descend2(sid: int):
        if sid in visited2:
            return
        visited2.add(sid)
        if selectable_v2(sid):
            selected_v2.append(sid)
            return
        for s in succ[sid]:
            descend2(s)
    for sid in sorted(roots):
        descend2(sid)

    def strat(sel_ids: list[int]) -> dict:
        cov: set[str] = set()
        for sid in sel_ids:
            cov |= cover(sid)
        exposed = sum(1 for sid in sel_ids if region_has_risk(sid))
        return {"harness": len(sel_ids), "covered": len(cov), "risk_exposed": exposed}

    all_constructible = [sid for sid in members if constructible_mapped[sid]]
    return {
        "program": name, "error": None,
        "n_funcs": len(c["functions"]), "matched": len(matched),
        "rust_only": amap["summary"]["rust_only_count"],
        "frontier": strat(selected),
        "frontier_v2": strat(selected_v2),
        "root": strat(roots),
        "all_constructible": strat(all_constructible),
        "leaf": strat(leaves),
        "frontier_members": [members[s][0] for s in selected],
        "frontier_v2_members": [members[s][0] for s in selected_v2],
        "sink_reasons": sink_reasons,
    }


def main() -> int:
    ap = argparse.ArgumentParser(description="STU frontier selector v1")
    ap.add_argument("--only", default=None, help="comma list of program names")
    args = ap.parse_args()
    only = set(args.only.split(",")) if args.only else None
    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_")
                   and (only is None or p.name in only))
    rows = [analyze(p) for p in pairs]
    ok = [r for r in rows if not r["error"]]
    bad = [r for r in rows if r["error"]]
    # keep programs where the strategies actually differ (a real frontier choice)
    interesting = [r for r in ok if r["frontier"] != r["root"] or r["frontier"] != r["all_constructible"]]
    interesting.sort(key=lambda r: -r["n_funcs"])

    def cell(s):  # "harness/covered (exposed)"
        return f"{s['harness']}/{s['covered']} ({s['risk_exposed']})"

    md = ["# STU frontier selector — strategy comparison (Layer 2, Steps 2-4)\n",
          "Hard-threshold bottom-up antichain; fixed interpretable risk (no model, no training). "
          "Cells are **#harness / covered-funcs (risk-exposed)** — computable without fuzzing. "
          "**v1** = sink below RISKY (collapses where risk is central). **v2** = *guarded rise*: tolerate "
          "a RISKY callee when its call is shielded by an input clamp, so the frontier rises to the "
          "constraining boundary instead of collapsing. (v2 risk-exposed counts reachable RISKY even when "
          "shielded — a static over-count G3 corrects empirically.)\n",
          "| program | funcs | root | all-constructible | leaf-only | frontier **v1** | frontier **v2** |",
          "|---|--:|---|---|---|---|---|"]
    for r in interesting:
        md.append(f"| {r['program']} | {r['n_funcs']} | {cell(r['root'])} | "
                  f"{cell(r['all_constructible'])} | {cell(r['leaf'])} | {cell(r['frontier'])} | "
                  f"**{cell(r['frontier_v2'])}** |")
    # detail for the deep programs
    md += ["\n## Frontier detail (deep programs)\n"]
    for r in interesting:
        if r["n_funcs"] < 8:
            continue
        md.append(f"### {r['program']} ({r['n_funcs']} funcs, {r['matched']} matched, "
                  f"{r['rust_only']} rust-only helpers)")
        md.append(f"- v1 selected: `{', '.join(r['frontier_members']) or '(none)'}`")
        md.append(f"- v2 selected (guarded rise): `{', '.join(r['frontier_v2_members']) or '(none)'}`")
        for s in r["sink_reasons"][:8]:
            md.append(f"  - {s}")
    if bad:
        md += ["\n## Failures\n"] + [f"- `{r['program']}`: {r['error']}" for r in bad]
    out = ROOT / "results" / "stu_frontier_v1.md"
    out.write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"wrote {out}")
    print(f"\n{len(ok)}/{len(rows)} analyzed; {len(interesting)} with a real strategy difference"
          + (f"; {len(bad)} FAILED" if bad else ""))
    print(f"\n{'program':14} {'funcs':>5}  root           all            leaf           v1             v2")
    for r in interesting[:14]:
        print(f"  {r['program']:12} {r['n_funcs']:5}  {cell(r['root']):14} "
              f"{cell(r['all_constructible']):14} {cell(r['leaf']):14} {cell(r['frontier']):14} {cell(r['frontier_v2'])}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
