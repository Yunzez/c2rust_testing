#!/usr/bin/env python3
"""RQ1b -- Mutation-injection RECALL, via the RQ2 pipeline (reused wholesale).

The recall half of the comparator claim (RQ2 = precision / no false positives on faithful
c2rust; this = sensitivity / we still catch real UB-free bugs). See
results/archive/mutation_recall_eval_plan.md.

Per mutation spec {id, program, pair, entry, rust_entry?, operator, find, replace}:
  1. copy the (faithful c2rust) pair to a temp dir;
  2. apply the Rust-side textual patch (find -> replace, must occur EXACTLY once);
  3. run the RQ2 pipeline on the mutated pair (eval_rq2_ubgate.run_boundary, unchanged):
     fuzz gate-OFF -> replay each artifact gate-ON + standalone full-UBSan evidence build;
  4. map the RQ2 verdict to a recall outcome:
       BUG_KEPT (UB_FREE_DIVERGENCE)     -> DETECTED_UB_FREE   (recall hit; carries UB-free evidence)
       TN                                -> NOT_DETECTED       (no divergence in budget; M2: equiv check)
       SUPPRESSED / GATE_MISS / MEMORY_UB-> UB_ARTIFACT        (divergence only on a UB input -> excluded)
       NEEDS_REVIEW / MIXED              -> NEEDS_REVIEW
       BUILD_FAIL / *excluded*           -> BUILD_FAIL         (non-compiling mutant -> excluded, not a miss)

Base = faithful c2rust so an injected divergence is PURELY the mutation (clean recall, matcher-
decoupled). Reuses eval_rq2_ubgate so "detected" == its UB_FREE_DIVERGENCE class exactly.

Usage:
  eval_mutation_recall.py --muts scripts/mut_m1.json --secs 25 --json results/ablations/attribution/mut_rows/m1.json
"""
from __future__ import annotations
import argparse, json, shutil, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import eval_rq2_ubgate as rq2  # reuse the whole RQ2 replay/evidence/classify pipeline

# RQ2 run_boundary verdict -> recall outcome
VERDICT_MAP = {
    "BUG_KEPT": "DETECTED_UB_FREE",
    "TN": "NOT_DETECTED",
    "SUPPRESSED": "UB_ARTIFACT",
    "GATE_MISS(hard-trap)": "UB_ARTIFACT",
    "MEMORY_UB(tier3)": "UB_ARTIFACT",
    "NEEDS_REVIEW": "NEEDS_REVIEW",
    "MIXED": "NEEDS_REVIEW",
    "BUILD_FAIL": "BUILD_FAIL",
    "TIMEOUT": "TIMEOUT",
    "OFF_NONZERO_NO_ARTIFACT": "BUILD_FAIL",
    "GEN_ON_FAIL": "GEN_FAIL",
}


def apply_patch(pair_dir: Path, find: str, replace: str):
    """Apply a unique find->replace to the pair's Rust translation. Return (ok, detail)."""
    rs = sorted((pair_dir / "translated").glob("*.rs"))
    if not rs:
        return False, "no .rs in translated/"
    hits = [f for f in rs if find in f.read_text()]
    if len(hits) == 0:
        return False, "find string not present"
    if len(hits) > 1:
        return False, f"find string in {len(hits)} files (ambiguous)"
    f = hits[0]
    txt = f.read_text()
    n = txt.count(find)
    if n != 1:
        return False, f"find string occurs {n}x in {f.name} (must be exactly 1)"
    f.write_text(txt.replace(find, replace, 1))
    return True, f.name


def dry_check(m, workdir):
    """Patch + standalone rustc metadata compile check (no fuzz). Validates find-uniqueness and
    that the mutant still compiles, cheaply, before committing to a fuzz campaign."""
    mid = m["id"]
    src_pair = ROOT / m["pair"]
    tmp_root = workdir / f"dry_{mid}"
    tmp_pair = tmp_root / src_pair.name
    if tmp_root.exists():
        shutil.rmtree(tmp_root)
    shutil.copytree(src_pair, tmp_pair)
    ok, detail = apply_patch(tmp_pair, m["find"], m["replace"])
    if not ok:
        shutil.rmtree(tmp_root, ignore_errors=True)
        return {"id": mid, "outcome": "PATCH_FAIL", "detail": detail}
    rs = sorted((tmp_pair / "translated").glob("*.rs"))[0]
    import subprocess
    r = subprocess.run(["rustc", f"+{rq2.TOOLCHAIN}", "--edition", "2021", "--crate-type", "lib",
                        "--emit=metadata", "-A", "warnings", "-o", str(tmp_root / "m.rmeta"), str(rs)],
                       text=True, capture_output=True)
    shutil.rmtree(tmp_root, ignore_errors=True)
    if r.returncode != 0:
        return {"id": mid, "outcome": "COMPILE_FAIL", "detail": r.stderr.strip().splitlines()[-1][:120]}
    return {"id": mid, "outcome": "OK", "detail": detail}


def run_mutation(m, secs, workdir, cleanup=True, seed=None):
    mid = m["id"]
    src_pair = ROOT / m["pair"]
    tmp_root = workdir / f"mut_{mid}"
    tmp_pair = tmp_root / src_pair.name
    if tmp_root.exists():
        shutil.rmtree(tmp_root)
    shutil.copytree(src_pair, tmp_pair)

    ok, detail = apply_patch(tmp_pair, m["find"], m["replace"])
    if not ok:
        return {"id": mid, "program": m["program"], "operator": m["operator"],
                "outcome": "PATCH_FAIL", "detail": detail}

    b = {"name": mid, "pair": str(tmp_pair.resolve()), "entry": m["entry"],
         "rust_entry": m.get("rust_entry"), "kind": m["operator"]}
    r = rq2.run_boundary(b, secs, tmp_root, seed=seed)
    if "error" in r:
        return {"id": mid, "program": m["program"], "operator": m["operator"],
                "outcome": "GEN_FAIL", "detail": r.get("detail", r["error"]), "rq2": r}

    verdict = r.get("verdict", "?")
    outcome = VERDICT_MAP.get(verdict, "NEEDS_REVIEW")
    # pull the UB-free evidence (decoded triggering input + C UBSan check) for DETECTED hits
    ev = None
    for art in r.get("artifacts", []):
        if art.get("class") == "UB_FREE_DIVERGENCE":
            ev = art.get("evidence")
            break
    row = {"id": mid, "program": m["program"], "operator": m["operator"],
           "entry": m["entry"], "site": m.get("site"), "outcome": outcome,
           "rq2_verdict": verdict, "n_artifacts": len(r.get("artifacts", [])),
           "detect_wall_s": r.get("detect_wall_s"), "seed": seed,
           "evidence": ev}
    if cleanup:  # drop the cargo builds; the row + evidence is all we keep (disk quota)
        shutil.rmtree(tmp_root, ignore_errors=True)
    return row


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--muts", required=True, help="JSON list of mutation specs")
    ap.add_argument("--secs", type=int, default=25)
    ap.add_argument("--workdir", default=None)
    ap.add_argument("--json", default=None)
    ap.add_argument("--keep-builds", action="store_true",
                    help="keep each mutant's cargo build dir (default: delete after classifying)")
    ap.add_argument("--dry-run", action="store_true",
                    help="only patch + rustc-metadata compile check each mutant (no fuzz); validates specs")
    ap.add_argument("--seed", type=int, default=None,
                    help="libFuzzer -seed for the gate-OFF campaign (reproducible runs / flakiness checks)")
    args = ap.parse_args()
    workdir = Path(args.workdir) if args.workdir else Path(
        "/tmp/claude-1000/-home-yunzez-c2rust-testing/1f18b0e9-85a1-4720-97e0-8c9d8d673339/scratchpad/mutrun")
    workdir.mkdir(parents=True, exist_ok=True)
    muts = json.loads(Path(args.muts).read_text())
    if args.dry_run:
        print(f"{'mutation':22} {'outcome':14} detail")
        print("-" * 70)
        bad = 0
        for m in muts:
            r = dry_check(m, workdir)
            if r["outcome"] != "OK":
                bad += 1
            print(f"{r['id']:22} {r['outcome']:14} {r.get('detail','')}")
        print("-" * 70)
        print(f"{len(muts)-bad}/{len(muts)} specs OK" + (f"  ({bad} need fixing)" if bad else ""))
        return 1 if bad else 0
    rows = []
    print(f"{'mutation':16} {'program':14} {'operator':22} {'outcome':18} {'rq2':18} {'wall_s':>7} ev")
    print("-" * 104)
    for m in muts:
        r = run_mutation(m, args.secs, workdir, cleanup=not args.keep_builds, seed=args.seed)
        rows.append(r)
        ev = r.get("evidence") or {}
        evs = (f"args={ev.get('args')} ub-free" if ev.get("is_ub") is False
               else "-" if not ev else f"is_ub={ev.get('is_ub')}")
        t = r.get("detect_wall_s")
        print(f"{r['id']:16} {r['program']:14} {r['operator']:22} "
              f"{r['outcome']:18} {str(r.get('rq2_verdict','-')):18} {t if t is not None else '-':>7} {evs}")
    n_det = sum(1 for r in rows if r["outcome"] == "DETECTED_UB_FREE")
    n_valid = sum(1 for r in rows if r["outcome"] in ("DETECTED_UB_FREE", "NOT_DETECTED"))
    print("-" * 96)
    print(f"detected UB-free: {n_det} / {n_valid} candidate-valid  "
          f"({len(rows)} total specs)")
    if args.json:
        Path(args.json).parent.mkdir(parents=True, exist_ok=True)
        Path(args.json).write_text(json.dumps(rows, indent=1))
        print(f"wrote {args.json}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
