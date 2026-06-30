#!/usr/bin/env python3
"""RQ3 matcher evaluation (DoD runner) — one regime (data source) per invocation.

Imports matcher.match() directly (no stdout parsing) so it can report, per program and
micro/macro-aggregated: matched / extra_accepted / ambiguous / correct and
precision / recall / coverage, across FIVE methods (the ablation ladder) + the headline
main config.

Methods (see results/rq3_eval_plan.md Table 2):
  name_eq      identity c->c from truth (no matcher) — recall only (what name-based oracles get)
  shape_only   io-SHAPE-ONLY node sim, forced (--shape-only): "are signatures enough?" baseline
  node_only    full per-function sim, NO topology, forced (--no-topo)
  full_forced  full + topology, forced 1-1 (every C matched)
  main         full + topology, partial (deployment, no abstention) — Table 1 headline
  full_abstain full + topology, partial + abstention (deployment, isolates ambiguous)

Metrics (plan §Metrics):
  scorable          = |truth|
  matched           = accepted pairs
  extra_accepted    = accepted pairs whose C is NOT in truth
  accepted_on_truth = matched - extra_accepted
  correct           = accepted pairs with truth[c] == r
  precision = correct / matched ; recall = correct / scorable ; coverage = accepted_on_truth / scorable

Usage:
  eval_rq3_matcher.py --source "raw-LLM (gpt-5-mini)" \
      --truth-dir experiments/llm_transpiler/truth \
      --c-pairs   benchmark/pairs --rust-out experiments/llm_transpiler/out \
      --json results/rq3_rows/rawllm_gpt5mini.v2.json
"""
from __future__ import annotations
import argparse, json, subprocess, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
STU = ROOT / "tools" / "stu_selector"
ANALYZER = STU / "analyzer" / "target" / "release" / "analyzer"
sys.path.insert(0, str(STU))
import matcher  # noqa: E402

ABSTAIN_EPS = 0.01
CFGS = {
    "shape_only":   dict(topo=False, feat="shape", partial=False, abstain_eps=None),
    "node_only":    dict(topo=False, feat="full",  partial=False, abstain_eps=None),
    "full_forced":  dict(topo=True,  feat="full",  partial=False, abstain_eps=None),
    "main":         dict(topo=True,  feat="full",  partial=True,  abstain_eps=None),
    "full_abstain": dict(topo=True,  feat="full",  partial=True,  abstain_eps=ABSTAIN_EPS),
}
METHOD_ORDER = ["name_eq", "shape_only", "node_only", "full_forced", "main", "full_abstain"]


def rdiv(a, b):
    return round(a / b, 3) if b else None


def metrics(res, truth, scorable):
    matched = res["matched"]
    correct = sum(1 for (c, r, s, k) in matched if truth.get(c) == r)
    extra = sum(1 for (c, r, s, k) in matched if c not in truth)
    return {
        "scorable": scorable, "matched": len(matched), "extra_accepted": extra,
        "ambiguous": len(res["ambiguous"]), "correct": correct,
        "precision": rdiv(correct, len(matched)),
        "recall": rdiv(correct, scorable),
        "coverage": rdiv(len(matched) - extra, scorable),
    }


def run_methods(c_data, r_data, truth):
    scorable = len(truth)
    out = {}
    neq = sum(1 for c in truth if truth[c] == c)   # identity-mapping baseline
    out["name_eq"] = {"scorable": scorable, "matched": None, "extra_accepted": None,
                      "ambiguous": None, "correct": neq,
                      "precision": None, "recall": rdiv(neq, scorable), "coverage": None}
    for name, cfg in CFGS.items():
        out[name] = metrics(matcher.match(c_data, r_data, **cfg), truth, scorable)
    return out


def scramble_rust(r_data, truth):
    """Mechanically rename every Rust function -> r_0000.. over the analyzer output
    (functions[].name + raw_edges), and remap the (independent) truth values to match.
    Turns a names-kept corpus (e.g. SACTOR, truth = its function_name_map) into a REAL
    renaming regime with INDEPENDENT truth — name-equality collapses to ~0, the matcher
    must recover by structure. Names are opaque node IDs so topology is preserved."""
    rmap = {f["name"]: f"r_{i:04d}" for i, f in enumerate(r_data["functions"])}
    nd = json.loads(json.dumps(r_data))
    for f in nd["functions"]:
        f["name"] = rmap.get(f["name"], f["name"])
    for e in nd.get("raw_edges", []):
        e["from"] = rmap.get(e["from"], e["from"])
        e["to"] = rmap.get(e["to"], e["to"])
    ts = {c: rmap.get(r, r) for c, r in truth.items()}
    return nd, ts


def run(cmd):
    return subprocess.run(cmd, text=True, capture_output=True)


def analyze(p, cc, crate, tmp):
    c_json, r_json = tmp / f"{p}.c.json", tmp / f"{p}.rust.json"
    rc = run([sys.executable, str(STU / "c_analyzer.py"), "--compile-commands", str(cc), "--enable-metrics"])
    if rc.returncode != 0:
        return None, "c_analyzer failed"
    rr = run([str(ANALYZER), str(crate), "--enable-metrics"])
    if rr.returncode != 0 or '"load_failed"' in rr.stdout:
        return None, "rust analyzer failed"
    c_json.write_text(rc.stdout); r_json.write_text(rr.stdout)
    return (json.loads(rc.stdout), json.loads(rr.stdout)), None


def aggregate(programs):
    """micro = pooled counts re-divided; macro = unweighted mean of per-program rates."""
    micro, macro = {}, {}
    for m in METHOD_ORDER:
        rows = [pr["methods"][m] for pr in programs if m in pr["methods"]]
        if not rows:
            continue
        S = sum(r["scorable"] for r in rows)
        C = sum(r["correct"] for r in rows)
        Mt = sum((r["matched"] or 0) for r in rows)
        Ex = sum((r["extra_accepted"] or 0) for r in rows)
        Am = sum((r["ambiguous"] or 0) for r in rows)
        has_match = rows[0]["matched"] is not None
        micro[m] = {
            "scorable": S, "matched": Mt if has_match else None,
            "extra_accepted": Ex if has_match else None,
            "ambiguous": Am if has_match else None, "correct": C,
            "precision": rdiv(C, Mt) if has_match else None,
            "recall": rdiv(C, S),
            "coverage": rdiv(Mt - Ex, S) if has_match else None,
        }
        def mean(key):
            vals = [r[key] for r in rows if r.get(key) is not None]
            return round(sum(vals) / len(vals), 3) if vals else None
        macro[m] = {"precision": mean("precision"), "recall": mean("recall"),
                    "coverage": mean("coverage")}
    return micro, macro


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--source", required=True, help="data-source / regime label (the table row)")
    ap.add_argument("--regime", default=None, help="regime tag (defaults to --source)")
    ap.add_argument("--truth-dir", required=True)
    ap.add_argument("--c-pairs", required=True, help="dir with <pair>/build compile_commands")
    ap.add_argument("--rust-out", required=True, help="dir with <pair> rust crates")
    ap.add_argument("--json", default=None)
    ap.add_argument("--scramble-rust", action="store_true",
                    help="mechanically rename Rust fns -> r_#### and remap truth (independent-"
                    "truth REAL rename regime, e.g. for SACTOR whose map = ground truth)")
    args = ap.parse_args()

    truth_dir, c_pairs, rust_out = Path(args.truth_dir), Path(args.c_pairs), Path(args.rust_out)
    tmp = Path(tempfile.mkdtemp())
    programs = []
    for tf in sorted(truth_dir.glob("*.json")):
        p = tf.stem
        cc, crate = c_pairs / p / "build", rust_out / p
        if not cc.exists() or not crate.exists():
            print(f"{p:16} [skip: missing C build or rust crate]"); continue
        data, err = analyze(p, cc, crate, tmp)
        if err:
            print(f"{p:16} [{err}]"); continue
        c_data, r_data = data
        truth = json.loads(tf.read_text())
        if args.scramble_rust:
            r_data, truth = scramble_rust(r_data, truth)
        methods = run_methods(c_data, r_data, truth)
        programs.append({"program": p, "regime": args.regime or args.source,
                         "scorable": len(truth), "methods": methods})
        mm = methods["main"]
        print(f"{p:16} scorable={mm['scorable']:>3} matched={mm['matched']:>3} "
              f"correct={mm['correct']:>3} extra={mm['extra_accepted']:>2} "
              f"P={mm['precision']} R={mm['recall']}")

    micro, macro = aggregate(programs)
    print(f"\n=== {args.source}  ({len(programs)} programs) — micro recall by method ===")
    for m in METHOD_ORDER:
        if m in micro:
            x = micro[m]
            print(f"  {m:14} recall={x['recall']}  precision={x['precision']}  "
                  f"coverage={x['coverage']}  (correct {x['correct']}/{x['scorable']})")
    blob = {"source": args.source, "regime": args.regime or args.source,
            "programs": programs, "micro": micro, "macro": macro}
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(blob, indent=1))
        print(f"\nwrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
