#!/usr/bin/env python3
"""RQ3 evaluation: name-independent matcher precision/recall on RENAMED translations.

One ROW per data source (provenance / renaming regime), micro-averaged over its programs:
  precision = correct_matched / matched        (of what we paired, how many right)
  recall    = correct_matched / truth_pairs     (of all true pairs, how many recovered)
  coverage  = matched / truth_pairs
  name_eq   = recall of the NAME-EQUALITY baseline (predict identity c->c) = what Fluorine/
              RustAssure rely on; under genuine renaming this is ~0.
  no_topo   = recall with --no-topo (call-graph propagation ablation).

Usage:
  rq3_eval.py --source "raw-LLM (gpt-5-mini)" \
      --truth-dir experiments/llm_transpiler/truth \
      --c-pairs   benchmark/pairs \
      --rust-out  experiments/llm_transpiler/out \
      [--json results/rq3_rows/rawllm.json]
"""
from __future__ import annotations
import argparse, json, re, subprocess, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STU = ROOT / "tools" / "stu_selector"
ANALYZER = STU / "analyzer" / "target" / "release" / "analyzer"
PY = sys.executable

RE_MATCHED = re.compile(r"matched:\s*(\d+)")
RE_CORRECT = re.compile(r"CORRECT \(\w+\):\s*(\d+)\s*\|\s*accuracy:\s*(\d+)/(\d+)")


def run(cmd):
    return subprocess.run(cmd, text=True, capture_output=True)


def matcher_scores(c_json: Path, r_json: Path, truth: Path, no_topo: bool):
    cmd = [PY, str(STU / "matcher.py"), "--c", str(c_json), "--rust", str(r_json), "--truth", str(truth)]
    if no_topo:
        cmd.append("--no-topo")
    out = run(cmd).stdout
    m_matched = RE_MATCHED.search(out)
    m_correct = RE_CORRECT.search(out)
    if not (m_matched and m_correct):
        return None
    matched = int(m_matched.group(1))
    correct = int(m_correct.group(1))
    scorable = int(m_correct.group(3))
    return {"matched": matched, "correct": correct, "scorable": scorable}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", required=True, help="provenance label (the table row name)")
    ap.add_argument("--truth-dir", required=True)
    ap.add_argument("--c-pairs", required=True, help="dir with <pair>/build compile_commands")
    ap.add_argument("--rust-out", required=True, help="dir with <pair> rust crates")
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    truth_dir, c_pairs, rust_out = Path(args.truth_dir), Path(args.c_pairs), Path(args.rust_out)
    agg = {"correct": 0, "matched": 0, "scorable": 0, "name_eq": 0, "no_topo_correct": 0}
    per_pair, tmp = [], Path(tempfile.mkdtemp())
    print(f"{'pair':16} {'truth':>6} {'matched':>8} {'correct':>8} {'no_topo':>8} {'name_eq':>8}")
    print("-" * 60)
    for tf in sorted(truth_dir.glob("*.json")):
        p = tf.stem
        cc, crate = c_pairs / p / "build", rust_out / p
        if not cc.exists() or not crate.exists():
            print(f"{p:16} [skip: missing C build or rust crate]"); continue
        truth = json.loads(tf.read_text())
        name_eq = sum(1 for c, r in truth.items() if c == r)   # identity-mapping baseline
        c_json, r_json = tmp / f"{p}.c.json", tmp / f"{p}.rust.json"
        rc = run([PY, str(STU / "c_analyzer.py"), "--compile-commands", str(cc), "--enable-metrics"])
        if rc.returncode != 0:
            print(f"{p:16} [c_analyzer failed]"); continue
        c_json.write_text(rc.stdout)
        rr = run([str(ANALYZER), str(crate), "--enable-metrics"])
        if rr.returncode != 0 or '"load_failed"' in rr.stdout:
            print(f"{p:16} [rust analyzer failed]"); continue
        r_json.write_text(rr.stdout)
        s = matcher_scores(c_json, r_json, tf, no_topo=False)
        st = matcher_scores(c_json, r_json, tf, no_topo=True)
        if not s:
            print(f"{p:16} [matcher parse failed]"); continue
        nt = st["correct"] if st else 0
        per_pair.append({"pair": p, **s, "no_topo_correct": nt, "name_eq": name_eq})
        for k in ("correct", "matched", "scorable"):
            agg[k] += s[k]
        agg["name_eq"] += name_eq
        agg["no_topo_correct"] += nt
        print(f"{p:16} {s['scorable']:>6} {s['matched']:>8} {s['correct']:>8} {nt:>8} {name_eq:>8}")

    S, C, M = agg["scorable"], agg["correct"], agg["matched"]
    row = {
        "source": args.source,
        "programs": len(per_pair),
        "truth_pairs": S,
        "precision": round(C / M, 3) if M else 0.0,
        "recall": round(C / S, 3) if S else 0.0,
        "coverage": round(M / S, 3) if S else 0.0,
        "recall_no_topo": round(agg["no_topo_correct"] / S, 3) if S else 0.0,
        "recall_name_eq": round(agg["name_eq"] / S, 3) if S else 0.0,
    }
    print("\n=== RQ3 ROW (micro-averaged) ===")
    print(f"source={row['source']}  programs={row['programs']}  truth_pairs={row['truth_pairs']}")
    print(f"precision={row['precision']}  recall={row['recall']}  coverage={row['coverage']}")
    print(f"recall(--no-topo ablation)={row['recall_no_topo']}  recall(name-equality baseline)={row['recall_name_eq']}")
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps({"row": row, "per_pair": per_pair}, indent=1))
        print(f"wrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
