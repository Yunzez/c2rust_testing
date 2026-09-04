#!/usr/bin/env python3
"""RQ1 group B — apply ground-truth labels to an annotation sheet.

Labels live in `results/rq1_matching/annotation/<case>/labels.json`:

    {"case": ..., "sheet_fingerprint": "<sheet.json fingerprint.id>",
     "labeler": ..., "method": ..., "date": ..., "reviewed_by_user": false,
     "labels": {"<c_function>": {"truth": "<rust fn>|NONE|SPLIT:a;b|MERGED:x|STUB:x|AMBIGUOUS",
                                  "note": "<evidence: C line, Rust file:line>"}}}

`STUB:x` = the translator emitted a signature-only placeholder x for the C function (name and
parameters correspond, body is a no-op / constant) — not a translation, so not a scorable pair
under the strict rule; the scorer reports a lenient variant that counts it as truth x.

`apply` copies truth / truth_note into sheet.csv and sheet.json (the other columns are left
untouched), refusing to run when the fingerprint disagrees with the sheet or when a label
names a Rust function that is not in the sheet's Rust inventory.  `check` only validates.

Usage: python3 scripts/rq1_group_b_label.py apply|check <case> [<case> ...]
"""
import csv, json, sys

ROOT = "/home/yunzez/c2rust_testing"
ANN = f"{ROOT}/results/rq1_matching/annotation"
SPECIAL = {"NONE", "AMBIGUOUS"}


def truth_targets(t):
    if t in SPECIAL:
        return []
    if t.startswith("SPLIT:"):
        return t[6:].split(";")
    if t.startswith("MERGED:"):
        return [t[7:]]
    if t.startswith("STUB:"):
        return [t[5:]]
    return [t]


def validate(case):
    sheet = json.load(open(f"{ANN}/{case}/sheet.json"))
    lab = json.load(open(f"{ANN}/{case}/labels.json"))
    problems = []
    if lab["sheet_fingerprint"] != sheet["fingerprint"]["id"]:
        problems.append(f"fingerprint mismatch labels={lab['sheet_fingerprint']} sheet={sheet['fingerprint']['id']}")
    inv = {r["rust_function"] for r in sheet["rust_inventory"]}
    rows = {r["c_function"] for r in sheet["rows"]}
    labels = lab["labels"]
    missing = sorted(rows - set(labels))
    extra = sorted(set(labels) - rows)
    if missing:
        problems.append(f"{len(missing)} unlabeled rows: {missing[:10]}")
    if extra:
        problems.append(f"labels for unknown C functions: {extra[:10]}")
    for c, l in labels.items():
        if not l.get("note"):
            problems.append(f"{c}: label without evidence note")
        for t in truth_targets(l["truth"]):
            if t not in inv:
                problems.append(f"{c}: truth target {t!r} not in Rust inventory")
    return sheet, lab, problems


def apply(case):
    sheet, lab, problems = validate(case)
    if problems:
        raise SystemExit(f"{case}: " + "; ".join(problems))
    labels = lab["labels"]
    for r in sheet["rows"]:
        r["truth"] = labels[r["c_function"]]["truth"]
        r["truth_note"] = labels[r["c_function"]]["note"]
    sheet["labels_meta"] = {k: v for k, v in lab.items() if k != "labels"}
    json.dump(sheet, open(f"{ANN}/{case}/sheet.json", "w"), indent=1)
    csv_path = f"{ANN}/{case}/sheet.csv"
    with open(csv_path) as f:
        rd = csv.DictReader(f); fields = rd.fieldnames; crows = list(rd)
    for r in crows:
        r["truth"] = labels[r["c_function"]]["truth"]
        r["truth_note"] = labels[r["c_function"]]["note"]
    with open(csv_path, "w", newline="") as f:
        w = csv.DictWriter(f, fieldnames=fields); w.writeheader(); w.writerows(crows)
    n = len(labels)
    kinds = {"NONE": 0, "AMBIGUOUS": 0, "SPLIT": 0, "MERGED": 0, "STUB": 0, "fn": 0}
    for l in labels.values():
        t = l["truth"]
        kinds["SPLIT" if t.startswith("SPLIT:") else "MERGED" if t.startswith("MERGED:")
              else "STUB" if t.startswith("STUB:") else t if t in SPECIAL else "fn"] += 1
    print(f"{case}: applied {n} labels {kinds} (labeler={lab['labeler']}, reviewed_by_user={lab['reviewed_by_user']})")


if __name__ == "__main__":
    cmd, cases = sys.argv[1], sys.argv[2:]
    for c in cases:
        if cmd == "apply":
            apply(c)
        else:
            _, _, p = validate(c)
            print(f"{c}: {'OK' if not p else p}")
