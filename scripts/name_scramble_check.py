#!/usr/bin/env python3
"""Name-scramble self-check — HARD evidence the matcher uses NO function-name signal.

JSON-level scramble (plan §Hard artifact, layer 2): over the analyzer OUTPUTS (not the
source), rename every C function name -> c_0000.. and every Rust function name -> r_0000..,
consistently rewriting `functions[].name`, `raw_edges.from/to`, and the truth map. Names are
opaque node IDs only, so topology is preserved. Then re-run the matcher and compare the
matched pairs (translated back) to the un-scrambled run.

Expectation:
  - scrambled matched set == normal matched set (delta <= 1 pair; any diff is a score-tie
    broken by name string). If it differs more, something still leaks names -> investigate.
  - matcher CORRECTNESS on scrambled == normal (the matcher doesn't care about names).
  - NEGATIVE CONTROL: name-equality recall on the scrambled truth -> 0 (c_#### != r_####),
    proving the scramble actually destroyed the name channel.

Usage:
  name_scramble_check.py --truth-dir experiments/llm_transpiler/truth \
      --c-pairs benchmark/pairs --rust-out experiments/llm_transpiler/out [--json out.json]
"""
from __future__ import annotations
import argparse, json, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
import matcher  # noqa: E402
from eval_rq3_matcher import analyze  # noqa: E402

MAIN = dict(topo=True, feat="full", partial=True, abstain_eps=None)


def matched_set(c_data, r_data):
    res = matcher.match(c_data, r_data, **MAIN)
    return {(c, r) for (c, r, s, k) in res["matched"]}


def scramble(c_data, r_data, truth):
    cmap = {f["name"]: f"c_{i:04d}" for i, f in enumerate(c_data["functions"])}
    rmap = {f["name"]: f"r_{i:04d}" for i, f in enumerate(r_data["functions"])}

    def remap(data, m):
        nd = json.loads(json.dumps(data))
        for f in nd["functions"]:
            f["name"] = m.get(f["name"], f["name"])
        for e in nd.get("raw_edges", []):
            e["from"] = m.get(e["from"], e["from"])
            e["to"] = m.get(e["to"], e["to"])
        return nd

    cs, rs = remap(c_data, cmap), remap(r_data, rmap)
    ts = {cmap[c]: rmap[r] for c, r in truth.items() if c in cmap and r in rmap}
    return cs, rs, ts, cmap, rmap


def correctness(matched, truth):
    return sum(1 for (c, r) in matched if truth.get(c) == r)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--truth-dir", required=True)
    ap.add_argument("--c-pairs", required=True)
    ap.add_argument("--rust-out", required=True)
    ap.add_argument("--json", default=None)
    args = ap.parse_args()
    truth_dir, c_pairs, rust_out = Path(args.truth_dir), Path(args.c_pairs), Path(args.rust_out)
    tmp = Path(tempfile.mkdtemp())

    rows, tot_delta, tot_pairs = [], 0, 0
    neq_norm_tot = neq_scr_tot = scor_tot = 0
    print(f"{'program':16} {'pairs':>6} {'delta':>6} {'corr_norm':>10} {'corr_scr':>9} "
          f"{'neq_norm':>9} {'neq_scr':>8}")
    print("-" * 72)
    for tf in sorted(truth_dir.glob("*.json")):
        p = tf.stem
        cc, crate = c_pairs / p / "build", rust_out / p
        if not cc.exists() or not crate.exists():
            continue
        data, err = analyze(p, cc, crate, tmp)
        if err:
            print(f"{p:16} [{err}]"); continue
        c_data, r_data = data
        truth = json.loads(tf.read_text())
        normal = matched_set(c_data, r_data)
        cs, rs, ts, cmap, rmap = scramble(c_data, r_data, truth)
        inv_c = {v: k for k, v in cmap.items()}
        inv_r = {v: k for k, v in rmap.items()}
        scr_back = {(inv_c[c], inv_r[r]) for (c, r) in matched_set(cs, rs)}
        delta = len(normal ^ scr_back)
        corr_norm = correctness(normal, truth)
        corr_scr = correctness({(cmap[c], rmap[r]) for (c, r) in scr_back
                                if c in cmap and r in rmap}, ts)
        neq_norm = sum(1 for c in truth if truth[c] == c)
        neq_scr = sum(1 for c in ts if ts[c] == c)
        scor = len(truth)
        rows.append({"program": p, "pairs": len(normal), "delta": delta,
                     "corr_normal": corr_norm, "corr_scrambled": corr_scr,
                     "name_eq_normal": neq_norm, "name_eq_scrambled": neq_scr,
                     "scorable": scor})
        tot_delta += delta; tot_pairs += len(normal)
        neq_norm_tot += neq_norm; neq_scr_tot += neq_scr; scor_tot += scor
        print(f"{p:16} {len(normal):>6} {delta:>6} {corr_norm:>10} {corr_scr:>9} "
              f"{neq_norm:>9} {neq_scr:>8}")

    ok = tot_delta <= 1
    print("-" * 72)
    print(f"TOTAL pairs={tot_pairs} delta={tot_delta}  ->  "
          f"{'PASS (matcher ignores names)' if ok else 'FAIL (name leak!)'}")
    print(f"negative control: name-equality recall  normal={neq_norm_tot}/{scor_tot}="
          f"{neq_norm_tot/scor_tot:.3f}  scrambled={neq_scr_tot}/{scor_tot}="
          f"{neq_scr_tot/scor_tot:.3f}  (scrambled must be ~0)")
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(
            {"pass": ok, "total_pairs": tot_pairs, "total_delta": tot_delta,
             "name_eq_normal": round(neq_norm_tot / scor_tot, 3),
             "name_eq_scrambled": round(neq_scr_tot / scor_tot, 3),
             "programs": rows}, indent=1))
        print(f"wrote {args.json}")
    return 0 if ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
