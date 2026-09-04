#!/usr/bin/env python3
"""UB-gate ablation: artifact-level Rust coverage under the three C2R_MODE settings.

All three measurements replay the SAME archived corpus through the SAME binary, so they share one
coverage map and one identity space; the only difference is which regions executed.

  gated      the validator's real coverage (UB-tripping inputs never reach Rust)
  nogate     ceiling for this corpus, C still executed
  rust-only  ceiling for this corpus, C never executed

Two self-checks are printed rather than assumed:
  * gated must reproduce the headline measurement taken before the mode switch existed;
  * nogate and rust-only must be IDENTICAL — the two sides use separate buffers, so executing C
    cannot change what Rust executes. A difference is interference between the sides, not noise.

usage: modes.py <linemap.json> <tests_coverage.json> <mode_exports_dir> <out.json> [expected_fns expected_regs]
"""
import json, re, sys
from collections import defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import analyse2 as A  # noqa: E402

MODES = ["gated", "nogate", "rust-only"]


def main(linemap, tests_json, mode_dir, out_json, exp_f=None, exp_r=None):
    scored, ranges = A.load_map(linemap)
    canon = Path(str(linemap).replace(".linemap.json", "")).read_text().split("\n")
    tf, tr, tnames = A.extract(tests_json, ranges, scored, canon)
    uni_f, cov_tf = A.sets(tf)
    uni_r, cov_tr = A.sets(tr)

    by_mode = {m: {"f": set(), "r": set(), "harnesses": [], "dropped": 0} for m in MODES}
    per_harness = defaultdict(dict)
    for jp in sorted(Path(mode_dir).glob("*__*.json")):
        entry, mode = re.match(r"(.+)__(gated|nogate|rust-only)\.json$", jp.name).groups()
        of, orr, _ = A.extract(jp, ranges, scored, canon)
        _, cf = A.sets(of); _, cr = A.sets(orr)
        by_mode[mode]["dropped"] += len(cf - uni_f) + len(cr - uni_r)
        cf &= uni_f; cr &= uni_r
        by_mode[mode]["f"] |= cf; by_mode[mode]["r"] |= cr
        by_mode[mode]["harnesses"].append(entry)
        per_harness[entry][mode] = {"functions": len(cf), "regions": len(cr)}

    res = {"denominator": {"functions": len(uni_f), "regions": len(uni_r)},
           "tests": {"functions": len(cov_tf), "regions": len(cov_tr),
                     "function_coverage": round(len(cov_tf) / len(uni_f), 6),
                     "region_coverage": round(len(cov_tr) / len(uni_r), 6)},
           "modes": {}, "per_harness": per_harness, "checks": {}}
    for m in MODES:
        b = by_mode[m]
        res["modes"][m] = {"harnesses": len(b["harnesses"]),
                           "functions": len(b["f"]), "regions": len(b["r"]),
                           "function_coverage": round(len(b["f"]) / len(uni_f), 6),
                           "region_coverage": round(len(b["r"]) / len(uni_r), 6),
                           "identities_outside_universe": b["dropped"]}

    g, ng, ro = by_mode["gated"], by_mode["nogate"], by_mode["rust-only"]
    res["gate_cost"] = {"functions_withheld": len(ng["f"] - g["f"]),
                        "regions_withheld": len(ng["r"] - g["r"]),
                        "functions_only_gated": len(g["f"] - ng["f"]),
                        "regions_only_gated": len(g["r"] - ng["r"])}
    res["checks"]["nogate_equals_rust_only"] = {
        "functions_equal": ng["f"] == ro["f"], "regions_equal": ng["r"] == ro["r"],
        "function_diff": len(ng["f"] ^ ro["f"]), "region_diff": len(ng["r"] ^ ro["r"])}
    if exp_f is not None:
        res["checks"]["gated_reproduces_headline"] = {
            "expected_functions": int(exp_f), "got_functions": len(g["f"]),
            "expected_regions": int(exp_r), "got_regions": len(g["r"]),
            "match": len(g["f"]) == int(exp_f) and len(g["r"]) == int(exp_r)}

    json.dump(res, open(out_json, "w"), indent=1)
    d = res["denominator"]
    print(f"denominator: {d['functions']} functions / {d['regions']} regions")
    print(f"tests side : {res['tests']['functions']:>4} fn ({res['tests']['function_coverage']:.3f})"
          f"  {res['tests']['regions']:>5} reg ({res['tests']['region_coverage']:.3f})\n")
    print(f"  {'mode':<11} {'harn':>4} {'fns':>5} {'fn cov':>8} {'regions':>8} {'reg cov':>8}")
    for m in MODES:
        x = res["modes"][m]
        print(f"  {m:<11} {x['harnesses']:>4} {x['functions']:>5} {x['function_coverage']:>8.3f} "
              f"{x['regions']:>8} {x['region_coverage']:>8.3f}")
    print(f"\ngate cost (nogate minus gated): {res['gate_cost']['functions_withheld']} functions, "
          f"{res['gate_cost']['regions_withheld']} regions")
    print(f"checks: {json.dumps(res['checks'])}")


if __name__ == "__main__":
    main(*sys.argv[1:7])
