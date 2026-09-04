#!/usr/bin/env python3
"""RQ1 merged table (decision of 2026-09-02, user + advisor): one ten-library table pooling the
name-preserving artifacts (group A, `rows/group_a_table.json`, truth = hidden-name equality)
and the renaming-translator artifacts (group B, `rows/group_b_full.json`, truth = manual
labels). Aggregation = existing library-level macro rule: mean over COMPLETE tool outputs
within a library, then equal weight over the ten libraries. PARTIAL group-B outputs are
listed with a superscript p but excluded from row scores and Overall. Forced configuration,
strict implemented-function rule (STUBs are not pairs). No micro-average, no dev/eval split,
no deployment / abstention / P@C / coverage columns.

Writes results/rq1_matching/rows/merged_table.json and prints the markdown table.
"""
import json, os, sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
ROWS = f"{ROOT}/results/rq1_matching/rows"
LIBS = ["qsort", "urlparser", "quadtree", "genann", "cjson", "lil", "lodepng", "bzip2", "tulip", "optipng"]
SYM = {"c2rust": "∘", "laertes": "△", "crown": "◇", "c2saferrust": "•", "ptrtrans": "★", "sactor": "×"}


def main():
    a = json.load(open(f"{ROWS}/group_a_table.json"))
    b = json.load(open(f"{ROWS}/group_b_full.json"))
    arts = {l: [] for l in LIBS}
    partial = {l: [] for l in LIBS}
    for lib, d in a["per_library"].items():
        for key, c in d["per_artifact_counts"].items():
            tool = key.split("__")[1]
            p = c["f_correct"] / c["f_matched"] if c["f_matched"] else 0.0
            arts[lib].append(dict(key=key, tool=tool, group="A", pairs=c["scorable"], precision=p,
                                  recall=c["f_correct"] / c["scorable"], renamed=0, renamed_correct=0))
    for r in b["rows"].values():
        if r["artifact_status"] != "COMPLETE":
            partial[r["lib"]].append(r["tool"])
            continue
        rr = r["real_renamed"]
        arts[r["lib"]].append(dict(key=r["case"], tool=r["tool"], group="B", pairs=r["scorable"],
                                   precision=r["forced"]["precision"], recall=r["forced"]["recall"],
                                   renamed=rr["pairs"], renamed_correct=rr.get("forced_correct", 0) if rr["pairs"] else 0))
    out = {"_meta": {"sources": ["rows/group_a_table.json", "rows/group_b_full.json"],
                     "rule": "mean over COMPLETE outputs within library, equal weight over 10 libraries; forced; strict",
                     "group_a_meta": a["_meta"], "group_b_meta": b.get("_meta")},
           "per_library": {}, "overall": {}}
    P, R = [], []
    lines = ["| Library | Tools | Pairs (renamed) | Matcher P / R | Renamed correct |", "|---|---|---:|---|---:|"]
    tot = dict(outputs=0, pairs=0, renamed=0, renamed_correct=0)
    for l in LIBS:
        x = arts[l]
        n = len(x)
        pairs = sum(t["pairs"] for t in x); rp = sum(t["renamed"] for t in x); rc = sum(t["renamed_correct"] for t in x)
        p = sum(t["precision"] for t in x) / n; r = sum(t["recall"] for t in x) / n
        P.append(p); R.append(r)
        for k, v in zip(tot, (n, pairs, rp, rc)):
            tot[k] += v
        syms = " ".join(SYM[t["tool"]] for t in x) + "".join(f" {SYM[t]}ᵖ" for t in partial[l])
        out["per_library"][l] = dict(outputs=n, artifact_keys=[t["key"] for t in x], partial_tools=partial[l],
                                     pairs=pairs, renamed_pairs=rp, renamed_correct=rc, precision=p, recall=r)
        lines.append(f"| {l} | {syms} | {pairs} ({rp}) | {p:.3f} / {r:.3f} | {f'{rc}/{rp}' if rp else '—'} |")
    out["overall"] = dict(libraries=len(LIBS), **tot, precision=sum(P) / len(P), recall=sum(R) / len(R))
    o = out["overall"]
    lines.append(f"| **Overall ({o['libraries']} libraries, {o['outputs']} complete outputs)** | | **{o['pairs']} ({o['renamed']})** | "
                 f"**{o['precision']:.3f} / {o['recall']:.3f}** | **{o['renamed_correct']}/{o['renamed']}** |")
    print("\n".join(lines))
    print("\nᵖ = PARTIAL output (labeled + scored separately in group_b_status.md §3; not in row scores or Overall).")
    print("Name equality recovers 0/%d genuinely renamed pairs by construction; the matcher recovers %d/%d." % (o["renamed"], o["renamed_correct"], o["renamed"]))
    json.dump(out, open(f"{ROWS}/merged_table.json", "w"), indent=1)
    print(f"wrote {ROWS}/merged_table.json")


if __name__ == "__main__":
    sys.exit(main())
