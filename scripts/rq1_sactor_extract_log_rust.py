#!/usr/bin/env python3
"""Extract the Rust that SACTOR emitted for functions whose verification never passed.

SACTOR writes `translated_code_unidiomatic/functions/<fn>.rs` only after a function passes its
harness; when the harness cannot even be linked (tulip: `ti_indicators[]` fn-pointer table is
outside SACTOR's link closure, so every TU fails at "Failed to link project-level harness"), the
translation exists only in the structured log as consecutive records

    unidiomatic_translator:1358  "Translated function <name>:"
    unidiomatic_translator:1359  "<rust code>"

This script pulls those pairs out of `result/logs/sactor-*.jsonl` and writes one `.rs` per record
(`<out>/<name>.rs`, `<name>__attempt<k>.rs` when a function was translated more than once) plus an
`index.json` {name: [{"attempt", "file", "timestamp", "verdict"}]}.  The verdict is taken from the
next ERROR / "compiled successfully" style record that mentions the function or the batch item.

Usage: rq1_sactor_extract_log_rust.py <jsonl-log> <out-dir>
"""
import json, os, sys
from collections import defaultdict


def main(log, out):
    os.makedirs(out, exist_ok=True)
    recs = [json.loads(l) for l in open(log)]
    idx = defaultdict(list)
    for i, r in enumerate(recs):
        m = r.get("message", "")
        if not (r.get("lineno") == 1358 and m.startswith("Translated function ")):
            continue
        name = m[len("Translated function "):].rstrip(":").strip()
        nxt = recs[i + 1] if i + 1 < len(recs) else {}
        if nxt.get("lineno") != 1359:
            continue
        code = nxt["message"]
        # verdict: the first later record that mentions this function or a link/compile failure
        verdict = ""
        for r2 in recs[i + 2:i + 60]:
            m2 = r2.get("message", "")
            if r2.get("level") == "ERROR" or "Failed to link" in m2:
                verdict = m2.split("\n")[0][:200]
                break
            if "compiled successfully" in m2 and "Rust code" in m2:
                verdict = verdict or "rust compiled"
        k = len(idx[name]) + 1
        fn = f"{name}.rs" if k == 1 else f"{name}__attempt{k}.rs"
        with open(os.path.join(out, fn), "w") as f:
            f.write(f"// SACTOR unidiomatic translation of `{name}` (extracted from {os.path.basename(log)} "
                    f"at {r['timestamp']}; attempt {k}). Verification verdict: {verdict or 'n/a'}\n")
            f.write(code.rstrip("\n") + "\n")
        idx[name].append({"attempt": k, "file": fn, "timestamp": r["timestamp"], "verdict": verdict})
    json.dump(idx, open(os.path.join(out, "index.json"), "w"), indent=1, sort_keys=True)
    print(f"{sum(len(v) for v in idx.values())} translations for {len(idx)} functions -> {out}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
