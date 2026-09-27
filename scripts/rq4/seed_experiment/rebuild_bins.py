#!/usr/bin/env python3
"""Rebuild a cell's harness binaries with the CURRENT generator, without a campaign: plan_all + build_one per
planned boundary (exactly cell.py's build step), writing <out>/harnesses/<b>.bin, <out>/funnel.json (boundary,
built, generator hash) and <out>/plans.json. Used to re-replay archived corpora under a new oracle
(generator 0.9: NaN-equivalent float comparison) without re-fuzzing.
usage: rebuild_bins.py --pair P --lib L --tool T --out O --c-source F --shim F --defs F [--only a,b]"""
import argparse, json, re, shutil, sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import cell as CELL
sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
import c2r_funnel as F

def main():
    ap = argparse.ArgumentParser(); ap.add_argument("--pair", required=True); ap.add_argument("--lib", required=True)
    ap.add_argument("--tool", required=True); ap.add_argument("--out", required=True); ap.add_argument("--c-source")
    ap.add_argument("--shim"); ap.add_argument("--defs"); ap.add_argument("--plugins", action="append"); ap.add_argument("--only")
    a = ap.parse_args(); pair, out = Path(a.pair), Path(a.out); (out / "harnesses").mkdir(parents=True, exist_ok=True)
    CELL.GEN_HASH = CELL.generator_hash(); print(f"generator sources sha256[:16] = {CELL.GEN_HASH}", flush=True)
    plans = [p for p in F.plan_all(pair, out) if p["boundary"] not in F.pair_excludes(pair)]
    if a.only: plans = [p for p in plans if p["boundary"] in set(a.only.split(","))]
    planned = [p for p in plans if p["status"] == "planned"]
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}; private = set(defs.get("private", []))
    rs = next(iter(sorted((pair / "translated").glob("*.rs"))), None); rs_text = rs.read_text(errors="replace") if rs else ""
    rows, ok = [], 0
    for p in planned:
        b = p["boundary"]
        is_priv = b in private or (bool(rs_text) and not defs and re.search(rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(b)}\b', rs_text) is None)
        binp, err = CELL.build_one(a, pair, b, is_priv, out / "harnesses" / b, out / "target")
        rows.append({"boundary": b, "c_static": is_priv, "built": binp is not None, "error": err, "inputs": len(p["inputs"]), "generator": CELL.GEN_HASH})
        ok += binp is not None; print(f"  build {b:30s} {'OK' if binp else 'FAIL ' + (err or '')[:80]}", flush=True)
    (out / "funnel.json").write_text(json.dumps(rows, indent=1) + "\n")
    shutil.rmtree(out / "target", ignore_errors=True)
    print(f"built {ok}/{len(planned)}; REBUILD_DONE", flush=True); return 0

if __name__ == "__main__":
    raise SystemExit(main())
