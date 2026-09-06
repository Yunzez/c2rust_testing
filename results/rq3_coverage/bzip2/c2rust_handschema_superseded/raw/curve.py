#!/usr/bin/env python3
"""Artifact-level coverage curve, and the final cell result, for one (library, tool) pair.

Input: one llvm-cov export per (harness, checkpoint), named `<entry>@<m>min.json`.
At each checkpoint the artifact's coverage is the UNION over harnesses of the function and region
identities covered — never a sum or an average of per-harness percentages.  The denominator is the
tests-side identity set (that build carries -C link-dead-code, so it is complete).

usage: curve.py <linemap.json> <tests_coverage.json> <ours_dir> <out_dir> <corpus_root>
"""
import json, re, sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import analyse2 as A  # noqa: E402


def main(linemap, tests_json, ours_dir, out_dir, corpus_root):
    scored, ranges = A.load_map(linemap)
    canon = Path(str(linemap).replace(".linemap.json", "")).read_text().split("\n")
    out_dir = Path(out_dir); out_dir.mkdir(parents=True, exist_ok=True)

    tf, tr, tnames = A.extract(tests_json, ranges, scored, canon)
    uni_f, cov_tf = A.sets(tf)
    uni_r, cov_tr = A.sets(tr)

    per_ckpt = defaultdict(lambda: {"f": set(), "r": set(), "harnesses": [], "dropped_f": 0,
                                    "dropped_r": 0, "inputs": 0})
    per_harness_final = []
    for jp in sorted(Path(ours_dir).glob("*@*min.json")):
        entry, minute = re.match(r"(.+)@(\d+)min\.json$", jp.name).groups()
        minute = int(minute)
        of, orr, _ = A.extract(jp, ranges, scored, canon)
        _, cf = A.sets(of); _, cr = A.sets(orr)
        b = per_ckpt[minute]
        b["dropped_f"] += len(cf - uni_f); b["dropped_r"] += len(cr - uni_r)
        cf &= uni_f; cr &= uni_r
        b["f"] |= cf; b["r"] |= cr
        n = len(list((Path(corpus_root) / f"{entry}@{minute}min").iterdir()))
        b["harnesses"].append(entry); b["inputs"] += n
        if minute == 60:
            per_harness_final.append({
                "harness": entry, "functions_covered": len(cf), "regions_covered": len(cr),
                **A.gate_stats(jp, entry, n)})

    curve = []
    for m in sorted(per_ckpt):
        b = per_ckpt[m]
        curve.append({"minute": m, "harnesses": len(b["harnesses"]), "corpus_inputs": b["inputs"],
                      "functions_covered": len(b["f"]), "regions_covered": len(b["r"]),
                      "function_coverage": round(len(b["f"]) / len(uni_f), 6),
                      "region_coverage": round(len(b["r"]) / len(uni_r), 6),
                      "identities_outside_universe": {"functions": b["dropped_f"],
                                                      "regions": b["dropped_r"]}})

    final = max(per_ckpt)
    cov_of, cov_or = per_ckpt[final]["f"], per_ckpt[final]["r"]
    res = {"scope_files": scored, "checkpoint_minutes": sorted(per_ckpt),
           "harnesses_unioned": len(per_harness_final), "curve": curve,
           "per_harness_final": per_harness_final}
    for label, uni, ct, co in (("function", uni_f, cov_tf, cov_of),
                               ("region", uni_r, cov_tr, cov_or)):
        both, only_t, only_o = ct & co, ct - co, co - ct
        total = len(uni)
        res[label] = {"total_in_scope": total, "covered_tests": len(ct), "covered_ours": len(co),
                      "covered_both": len(both), "only_tests": len(only_t),
                      "only_ours": len(only_o), "union": len(ct | co),
                      "covered_by_neither": total - len(ct | co),
                      "tests_coverage": round(len(ct) / total, 6),
                      "ours_coverage": round(len(co) / total, 6),
                      "growth": round((len(co) - len(ct)) / total, 6),
                      "sanity": {
                          "both_plus_only_tests_eq_covered_tests": len(both) + len(only_t) == len(ct),
                          "both_plus_only_ours_eq_covered_ours": len(both) + len(only_o) == len(co),
                          "both_plus_onlys_eq_union": len(both) + len(only_t) + len(only_o) == len(ct | co),
                          "all_reported_in_scope": all(i[0] in [f"{m}.rs" for m in scored] for i in uni),
                          "covered_le_denominator": len(ct) <= total and len(co) <= total}}
        if label == "function":
            for nm, s in (("covered_by_both.txt", both), ("only_tests.txt", only_t),
                          ("only_ours.txt", only_o), ("covered_by_neither.txt", uni - (ct | co))):
                (out_dir / nm).write_text("".join(
                    f"{a}\tline {b}\t{A.demangle(tnames.get((a, b), '?'))}\n" for a, b in sorted(s)))

    json.dump(res, open(out_dir / "result.json", "w"), indent=1)
    print("coverage curve (artifact-level union over harnesses):")
    print(f"  {'min':>4} {'harn':>5} {'inputs':>7} {'fns':>5} {'fn cov':>8} {'regions':>8} {'reg cov':>8}")
    for c in curve:
        print(f"  {c['minute']:>4} {c['harnesses']:>5} {c['corpus_inputs']:>7} "
              f"{c['functions_covered']:>5} {c['function_coverage']:>8.3f} "
              f"{c['regions_covered']:>8} {c['region_coverage']:>8.3f}")
    print()
    print(json.dumps({k: res[k] for k in ("function", "region")}, indent=1))
    print("\nper harness at the final checkpoint:")
    for h in res["per_harness_final"]:
        print(f"  {h['harness']:28s} fn {h['functions_covered']:3d}  reg {h['regions_covered']:5d}  "
              f"replayed {h['inputs_replayed']}  reached-Rust {h['reached_rust']}  "
              f"gate-excluded {h['ub_gate_excluded']}")


if __name__ == "__main__":
    main(*sys.argv[1:6])
