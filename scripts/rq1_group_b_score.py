#!/usr/bin/env python3
"""RQ1 group B — score the matcher against manual ground truth on renaming translators.

Input: `results/rq1_matching/annotation/<case>/sheet.json` with `truth` filled by
`scripts/rq1_group_b_label.py apply`.  Output: `results/rq1_matching/rows/group_b_full.json`
(one row per artifact, same shape as group A rows) + a markdown table on stdout.

Scoring (mirrors scripts/rq1_group_a_table.py / rq1_name_preserving_full.py):
  scorable  = C functions whose truth names exactly one Rust function
              (NONE / MERGED / SPLIT / STUB / AMBIGUOUS rows are reported but excluded from the recall
              denominator; a forced or deployment match proposed for such a row still counts in
              the precision denominator, exactly like group A's unscorable C functions).
  forced    P = correct / all forced matches,      R = correct / scorable
  deployment P = correct / all accepted matches,  C = accepted-on-scorable / scorable,
              abstention = ambiguous / scorable
  name-eq baseline R = rows where a Rust fn with the identical leaf name exists AND equals truth.
  real-renamed subset = scorable rows whose truth name != C name (the rows group A cannot see);
              matcher forced/deployment recall is reported on that subset and on the same-name
              subset separately.
  tool-map audit = for rows with a tool claim, claim resolved == truth.
  STUB:x rows (translator emitted a signature-only placeholder x) are NOT pairs under the strict
              rule above; `lenient_with_stubs` re-scores the artifact with STUB:x read as truth x
              (name/parameter correspondence only) so both readings are on record.
Primary table (paper rule): per library, mean over its complete group-B tool outputs; overall =
equal-weight mean over libraries with >=1 output (denominator printed explicitly). Columns:
pairs (genuine renamed pairs) | name-eq recall | matcher P/R (forced) | renamed correct =
correctly matched genuine renamed pairs / all genuine renamed pairs (forced configuration; a
count ratio because renamed pairs are sparse). Deployment / abstention / coverage are kept per
artifact as SECONDARY numbers (operational policy), never in the primary table. Artifacts whose
labels.json carries `artifact_status: PARTIAL` (translator covered only part of the library) are
scored but excluded from the primary aggregate. No dev/eval-only or micro aggregate is produced.

Usage: python3 scripts/rq1_group_b_score.py [--cases a b ...]
"""
import argparse, json, os
from collections import defaultdict

ROOT = "/home/yunzez/c2rust_testing"
ANN = f"{ROOT}/results/rq1_matching/annotation"
OUT = f"{ROOT}/results/rq1_matching/rows/group_b_full.json"
LIB_ORDER = ["qsort", "urlparser", "quadtree", "genann", "cjson", "lil", "lodepng", "bzip2",
             "tulip", "optipng"]
TOOL_ORDER = ["ptrtrans", "sactor"]


def leaf(n):
    return n.rsplit("::", 1)[-1]


def single_target(t, stubs_count=False):
    """Rust fn name if the truth names exactly one Rust function, else None."""
    if not t or t in ("NONE", "AMBIGUOUS") or t.startswith(("SPLIT:", "MERGED:")):
        return None
    if t.startswith("STUB:"):
        return t[5:] if stubs_count else None
    return t


def score_case(case, stubs_count=False):
    sh = json.load(open(f"{ANN}/{case}/sheet.json"))
    rows = sh["rows"]
    if any(not r["truth"] for r in rows):
        raise SystemExit(f"{case}: unlabeled rows present — run rq1_group_b_label.py apply first")
    truth = {r["c_function"]: single_target(r["truth"], stubs_count) for r in rows}
    kinds = defaultdict(int)
    for r in rows:
        t = r["truth"]
        kinds["SPLIT" if t.startswith("SPLIT:") else "MERGED" if t.startswith("MERGED:")
              else "STUB" if t.startswith("STUB:") else t if t in ("NONE", "AMBIGUOUS") else "single"] += 1
    scorable = [r for r in rows if truth[r["c_function"]]]
    n = len(scorable)
    # raw matcher lists (forced / deployment) — cross-checked against the per-row columns
    forced = {c: leaf(rr) for c, rr, _s, _k in sh["matcher_forced_raw"]}
    deploy = {c: leaf(rr) for c, rr, _s, _k in sh["matcher_deploy_raw"]}
    amb = {c for c, *_ in sh["matcher_deploy_ambiguous"]}
    for r in rows:
        c = r["c_function"]
        if forced.get(c, "") != (r["matcher_forced"] or ""):
            raise SystemExit(f"{case}/{c}: forced raw {forced.get(c)} != row {r['matcher_forced']}")
        col = r["matcher_deploy"]
        exp = deploy.get(c, "ABSTAIN" if c in amb else "NONE")
        if col != exp:
            raise SystemExit(f"{case}/{c}: deploy raw {exp} != row {col}")
    f_correct = sum(1 for r in scorable if forced.get(r["c_function"]) == truth[r["c_function"]])
    d_correct = sum(1 for r in scorable if deploy.get(r["c_function"]) == truth[r["c_function"]])
    d_acc = sum(1 for r in scorable if r["c_function"] in deploy)
    d_amb = sum(1 for r in scorable if r["c_function"] in amb)
    # name-equality baseline and renamed / same-name split
    name_eq_correct = sum(1 for r in scorable if r["name_eq"] and leaf(r["name_eq"]) == truth[r["c_function"]])
    renamed = [r for r in scorable if truth[r["c_function"]] != r["c_function"]]
    same = [r for r in scorable if truth[r["c_function"]] == r["c_function"]]

    def sub(sel):
        m = len(sel)
        if not m:
            return {"pairs": 0}
        fc = sum(1 for r in sel if forced.get(r["c_function"]) == truth[r["c_function"]])
        dc = sum(1 for r in sel if deploy.get(r["c_function"]) == truth[r["c_function"]])
        da = sum(1 for r in sel if r["c_function"] in deploy)
        return {"pairs": m, "forced_correct": fc, "forced_recall": fc / m,
                "deploy_correct": dc, "deploy_accepted": da,
                "deploy_precision": dc / da if da else None, "coverage": da / m,
                "name_eq_recall": sum(1 for r in sel if r["name_eq"] and leaf(r["name_eq"]) == truth[r["c_function"]]) / m}

    claimed_all = [r for r in rows if r["tool_claim_status"] == "CLAIMED"]
    claimed = [r for r in claimed_all if truth[r["c_function"]]]   # audit only where a pair exists
    claim_ok = sum(1 for r in claimed if leaf(r["tool_claim_resolved"] or r["tool_claim"]) == truth[r["c_function"]])
    tool, lib = case.split("_", 1)
    fm, dm = len(forced), len(deploy)
    out = {
        "case": case, "tool": tool, "lib": lib, "split": sh["split"], "builds": sh["builds"],
        "artifact_status": (sh.get("labels_meta") or {}).get("artifact_status", "COMPLETE"),
        "c_functions": len(rows), "rust_functions": len(sh["rust_inventory"]),
        "rust_functions_dropped": sh["rust_functions_dropped"],
        "truth_kinds": dict(kinds), "scorable": n,
        "labels_meta": sh.get("labels_meta"),
        "forced": {"matched": fm, "correct": f_correct,
                   "precision": f_correct / fm if fm else None, "recall": f_correct / n if n else None},
        "deployment": {"matched": dm, "correct": d_correct, "accepted_on_truth": d_acc, "ambiguous": d_amb,
                       "precision": d_correct / dm if dm else None, "recall": d_correct / n if n else None,
                       "coverage": d_acc / n if n else None, "abstention_rate": d_amb / n if n else None},
        "name_eq_baseline": {"correct": name_eq_correct, "recall": name_eq_correct / n if n else None},
        "real_renamed": sub(renamed), "same_name": sub(same),
        "tool_map_audit": {"claimed_total": len(claimed_all), "claimed_on_pairs": len(claimed),
                           "claim_equals_truth": claim_ok,
                           "claim_precision": claim_ok / len(claimed) if claimed else None},
        "fingerprint": sh["fingerprint"],
    }
    if kinds.get("STUB") and not stubs_count:
        len_ = score_case(case, stubs_count=True)
        out["lenient_with_stubs"] = {k: len_[k] for k in ("scorable", "forced", "deployment", "name_eq_baseline",
                                                          "real_renamed", "same_name", "tool_map_audit")}
        out["lenient_with_stubs"]["note"] = "STUB:x read as truth x (signature-only placeholders count as pairs); NOT the primary number"
    return out


def fmt(x, d=3):
    return "—" if x is None else f"{x:.{d}f}"


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--cases", nargs="*")
    a = ap.parse_args()
    cases = a.cases or sorted(d for d in os.listdir(ANN) if os.path.exists(f"{ANN}/{d}/labels.json"))
    rows = {c: score_case(c) for c in cases}
    unreviewed = [c for c, r in rows.items() if not (r["labels_meta"] or {}).get("reviewed_by_user")]
    # PARTIAL artifacts (translator stopped before covering the library, e.g. cost breaker / refusal
    # after a leaf subset) are scored but never enter the primary aggregate.
    partial = {c: r for c, r in rows.items() if r["artifact_status"] == "PARTIAL"}
    complete = {c: r for c, r in rows.items() if c not in partial}

    print("## Secondary — per artifact (deployment / abstention are operational policy, not in the primary table)")
    print("| artifact | status | provenance | builds | C fns | Rust fns | truth kinds | pairs | name-eq R | matcher P / R | renamed correct | deploy P@C | abst | tool-map claim P |")
    print("|---|---|---|---|---:|---:|---|---:|---:|---|---|---|---:|---|")
    for c, r in rows.items():
        f, d, rn = r["forced"], r["deployment"], r["real_renamed"]
        rn_s = f"{rn['forced_correct']}/{rn['pairs']}" if rn["pairs"] else "0/0"
        print(f"| {c} | {r['artifact_status']} | {r['split']} | {'yes' if r['builds'] else 'no'} | {r['c_functions']} | {r['rust_functions']} | "
              f"{r['truth_kinds']} | {r['scorable']} | {fmt(r['name_eq_baseline']['recall'])} | "
              f"{fmt(f['precision'])} / {fmt(f['recall'])} | {rn_s} | {fmt(d['precision'])}@{fmt(d['coverage'])} | "
              f"{fmt(d['abstention_rate'])} | "
              f"{fmt(r['tool_map_audit']['claim_precision'])} ({r['tool_map_audit']['claimed_on_pairs']}/{r['tool_map_audit']['claimed_total']}) |")
        if "lenient_with_stubs" in r:
            l = r["lenient_with_stubs"]; f, d, rn = l["forced"], l["deployment"], l["real_renamed"]
            print(f"| ↳ {c} lenient (STUB:x = x, not primary) | | | | | | | {l['scorable']} | {fmt(l['name_eq_baseline']['recall'])} | "
                  f"{fmt(f['precision'])} / {fmt(f['recall'])} | {rn.get('forced_correct', 0)}/{rn['pairs']} | "
                  f"{fmt(d['precision'])}@{fmt(d['coverage'])} | {fmt(d['abstention_rate'])} | |")

    # Primary (paper rule): mean over complete tool outputs within a library, then equal weight over
    # libraries with data.  Renamed-correct is a count ratio (correct / genuine renamed pairs, forced
    # configuration), pooled within the library and summed overall, because renamed pairs are sparse.
    by_lib = defaultdict(list)
    for r in complete.values():
        by_lib[r["lib"]].append(r)
    paper = {}
    for lib in LIB_ORDER:
        sel = by_lib.get(lib)
        if not sel:
            continue
        n = len(sel)
        paper[lib] = {"tools": n, "tool_list": [r["tool"] for r in sel], "provenance": sel[0]["split"],
                      "pairs": sum(r["scorable"] for r in sel),
                      "renamed_pairs": sum(r["real_renamed"]["pairs"] for r in sel),
                      "renamed_correct": sum(r["real_renamed"].get("forced_correct", 0) for r in sel),
                      "precision": sum(r["forced"]["precision"] or 0 for r in sel) / n,
                      "recall": sum(r["forced"]["recall"] or 0 for r in sel) / n,
                      "name_eq_recall": sum(r["name_eq_baseline"]["recall"] or 0 for r in sel) / n,
                      "artifact_keys": [r["case"] for r in sel]}
    L = len(paper)
    overall = {"libraries_with_group_b_data": L, "libraries_in_corpus": len(LIB_ORDER),
               "denominator_note": f"equal-weight mean over the {L} of {len(LIB_ORDER)} libraries with >=1 complete labeled group-B output: {sorted(paper)}; partial artifacts excluded: {sorted(partial)}",
               "tool_outputs": sum(p["tools"] for p in paper.values()),
               "pairs": sum(p["pairs"] for p in paper.values()),
               "renamed_pairs": sum(p["renamed_pairs"] for p in paper.values()),
               "renamed_correct": sum(p["renamed_correct"] for p in paper.values())}
    for k in ("precision", "recall", "name_eq_recall"):
        overall[k] = sum(p[k] for p in paper.values()) / L if L else None
    print("\n## Primary — per library (mean over tool outputs within a library, equal weight over libraries)")
    print("| library | tools | pairs (genuine renamed) | name-eq R | matcher P / R (forced) | renamed correct |")
    print("|---|---|---:|---:|---|---:|")
    for lib, p in paper.items():
        print(f"| {lib} | {' '.join(p['tool_list'])} | {p['pairs']} ({p['renamed_pairs']}) | {fmt(p['name_eq_recall'])} | "
              f"{fmt(p['precision'])} / {fmt(p['recall'])} | {p['renamed_correct']}/{p['renamed_pairs']} |")
    o = overall
    print(f"| **Overall ({L}/{len(LIB_ORDER)} libraries, {o['tool_outputs']} tool outputs)** | | {o['pairs']} ({o['renamed_pairs']}) | "
          f"{fmt(o['name_eq_recall'])} | {fmt(o['precision'])} / {fmt(o['recall'])} | {o['renamed_correct']}/{o['renamed_pairs']} |")
    if partial:
        print(f"\nPARTIAL artifacts scored above but excluded from the primary aggregate: {sorted(partial)}")
    if unreviewed:
        print(f"\nWARNING: labels NOT user-reviewed for: {unreviewed}")

    json.dump({"_meta": {"scorer": "scripts/rq1_group_b_score.py", "annotation_dir": "results/rq1_matching/annotation",
                         "labels_unreviewed_by_user": unreviewed, "partial_artifacts": sorted(partial),
                         "scoring": "see docstring of scripts/rq1_group_b_score.py"},
               "rows": rows,
               "paper_rule": {"per_library": paper, "overall": overall,
                              "rule": "mean over complete group-B tool outputs within a library, then equal weight over libraries with data; "
                                      "pairs are evidence volume only; renamed_correct is a count ratio (forced configuration)"}},
              open(OUT, "w"), indent=1)
    print(f"\nwrote {OUT}")


if __name__ == "__main__":
    main()
