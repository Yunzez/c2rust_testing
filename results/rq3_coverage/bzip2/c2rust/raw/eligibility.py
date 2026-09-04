#!/usr/bin/env python3
"""Stage 3 of the method — harness eligibility — applied to every matched pair of one cell.

Eligibility is NOT re-decided here.  It is read off the frozen generator: an entry is eligible iff
tools/stu_selector/gen_diff_harness.py `resolve(..., infer=True)` produces items/abi for it, i.e.
iff the current automatic generator can construct one logical input for both sides and compare the
outputs.  Every rejection carries the generator's own message as the reason.

usage: eligibility.py <pair_dir> <matcher_output.json> <out.json>
"""
import json, sys, io, contextlib
from pathlib import Path

REPO = Path("/home/yunzez/c2rust_testing")
# the frozen generator plus the two documented RQ4 fixes (canonical return type, C-global renaming)
pathlib_gen = Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov/rq4_gen")
sys.path.insert(0, str(pathlib_gen))
import gen_diff_harness as gdh   # noqa: E402

def main(pair_dir, matcher_json, out_json):
    pair = Path(pair_dir)
    name = pair.name
    cc = pair / "build"
    m = json.load(open(matcher_json))
    pairs = [(c, r) for c, r, *_ in m["forced"]]
    deployment = {c for c, r, *_ in m["deployment"]}

    rows = []
    for c_fn, r_fn in sorted(pairs):
        row = {"c_entry": c_fn, "rust_entry": r_fn, "in_deployment_set": c_fn in deployment}
        buf = io.StringIO()
        try:
            with contextlib.redirect_stdout(buf), contextlib.redirect_stderr(buf):
                params, ret, all_fns, items, abi = gdh.resolve(name, cc, c_fn, infer=True)
        except SystemExit as e:
            row["eligible"] = False
            row["reason"] = str(e)
        except Exception as e:  # libclang / descriptor failures are also ineligibility
            row["eligible"] = False
            row["reason"] = f"{type(e).__name__}: {e}"
        else:
            if not params and not abi:
                row["eligible"] = False
                row["reason"] = ("entry not found in the C translation unit, or it takes no "
                                 "parameters, so no logical input can be constructed")
            else:
                row["eligible"] = True
                row["ret"] = ret
                row["abi_roles"] = [(p["name"], p["role"]) for p in abi]
                row["reason"] = None
        rows.append(row)
        flag = "ELIGIBLE" if row["eligible"] else "rejected"
        print(f"{c_fn:32s} {flag:9s} {row.get('reason') or row.get('abi_roles')}")

    out = {
        "cell": name,
        "generator": {"module": "tools/stu_selector/gen_diff_harness.py",
                      "GEN_VERSION": gdh.GEN_VERSION,
                      "GEN_CAPABILITIES": list(gdh.GEN_CAPABILITIES)},
        "matched_pairs": len(pairs),
        "eligible": sum(1 for r in rows if r["eligible"]),
        "rows": rows,
    }
    json.dump(out, open(out_json, "w"), indent=1)
    print(f"\nmatched {out['matched_pairs']}  eligible {out['eligible']}")

if __name__ == "__main__":
    main(*sys.argv[1:4])
