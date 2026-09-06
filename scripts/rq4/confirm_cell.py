#!/usr/bin/env python3
"""Confirmation for one RQ4 cell: adjudicate every candidate the campaign produced.

Runs AFTER the campaign, never beside it: the four replays per candidate would compete with a
running campaign for CPU and put the cells under different conditions.

For each boundary that has candidates:
  1. build the CONFIRMATION harness  (--c-sanitize: ASan + full UBSan on the C side)
  2. build the NO-SANITIZER harness  (--sanitizer none: decides program-vs-instrument for traps)
  3. reuse the DISCOVERY binary the cell already kept (<harnesses>/<entry>.bin)
  4. scripts/c2r_campaign.py confirm  --bin --asan-bin --nosan-bin

Candidates are inputs, not defects: one root cause yields thousands (c2rust bzip2: 26 814 on a
single boundary). `--sample N` adjudicates the first N per boundary, sorted by artifact name --
libFuzzer names are content hashes, so that is an unbiased draw -- and writes the tally under
`confirm_sample/`. It is a pipeline check and a preview, LABELLED as a sample; the full run writes
under `confirm/` and is the only thing a RUN.md may quote as the cell's adjudication.

usage:
  scripts/rq4/confirm_cell.py --cell <dir from cell.py> --pair <dir> [--c-source bzip2lib.c]
                              [--shim <file>] [--defs <json>] [--plugins <toml>]
                              [--sample N] [--only a,b] [--timeout 20]
"""
from __future__ import annotations

import argparse
import json
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "scripts" / "rq4"))
import cell as CELL  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--pair", required=True)
    ap.add_argument("--plugins", action="append")
    ap.add_argument("--c-source")
    ap.add_argument("--shim")
    ap.add_argument("--defs")
    ap.add_argument("--sample", type=int, help="adjudicate only the first N artifacts per boundary")
    ap.add_argument("--only")
    ap.add_argument("--timeout", type=float, default=20.0)
    a = ap.parse_args()

    cell, pair = Path(a.cell), Path(a.pair)
    funnel = json.loads((cell / "funnel.json").read_text())
    defs = json.loads(Path(a.defs).read_text()) if a.defs else {}
    tag = "confirm_sample" if a.sample else "confirm"
    out = cell / tag
    out.mkdir(exist_ok=True)
    target = cell / "target_confirm"
    only = {s.strip() for s in a.only.split(",")} if a.only else None

    summary = []
    for row in funnel:
        b = row["boundary"]
        if only and b not in only:
            continue
        cand_dir = cell / "candidates" / b
        cands = sorted(p for p in cand_dir.glob("*") if p.is_file() and p.suffix != ".log") \
            if cand_dir.is_dir() else []
        # The value-divergence channel: inputs the combined replay (replay_cell.py, protocol step 6)
        # flagged on the coverage corpus. `ub-gated-*` were rejected by the in-loop noise filter and
        # are not candidates; `_outcomes.json` is the replay's own record, not an input.
        div_dir = cell / "divergences" / b
        divs = sorted(p for p in div_dir.glob("*") if p.is_file() and not p.name.startswith("_")
                      and not p.name.startswith("ub-gated-")) if div_dir.is_dir() else []
        cands = cands + divs
        if not cands:
            continue
        dis = cell / "harnesses" / f"{b}.bin"
        if not dis.exists():
            summary.append({"boundary": b, "error": "discovery binary missing"})
            print(f"{b}: discovery binary missing", flush=True)
            continue
        t0 = time.time()
        san, e1 = CELL.build_one(a, pair, b, row["c_static"], cell / "harnesses" / f"{b}_san",
                                 target, sanitize=True)
        nos, e2 = CELL.build_one(a, pair, b, row["c_static"], cell / "harnesses" / f"{b}_nosan",
                                 target, nosan=True)
        shutil.rmtree(target, ignore_errors=True)
        if not san or not nos:
            summary.append({"boundary": b, "error": (e1 or e2 or "")[:300]})
            print(f"{b}: BUILD FAILED {(e1 or e2 or '')[:120]}", flush=True)
            continue

        # Sample PER CHANNEL: artifact names sort `crash-*` before `divergence-*`, so a single
        # prefix would never reach the value-divergence inputs on a boundary with thousands of
        # terminations (CROWN fallbackSort: 5109 crashes, 32 bhtab divergences, sample saw 0).
        terms = [c for c in cands if not c.name.startswith(("divergence-", "panic-", "signal-", "timeout-", "nonzero-exit-"))]
        divs_ = [c for c in cands if c not in terms]
        use = (terms[:a.sample] + divs_[:a.sample]) if a.sample else cands
        cdir = out / f"{b}_cands"
        shutil.rmtree(cdir, ignore_errors=True)
        cdir.mkdir()
        for c in use:
            (cdir / c.name).symlink_to(c.resolve())
        v = out / f"{b}_verdicts"
        r = subprocess.run([sys.executable, str(ROOT / "scripts/c2r_campaign.py"), "confirm",
                            "--bin", str(dis), "--asan-bin", str(san), "--nosan-bin", str(nos),
                            "--candidates", str(cdir), "--out", str(v),
                            "--timeout", str(a.timeout)],
                           capture_output=True, text=True, errors="replace", timeout=12 * 3600)
        v.mkdir(exist_ok=True)
        (v / "confirm.log").write_text(r.stdout + r.stderr)
        # the replays/ tree is one file per (candidate x phase); keep the verdicts, drop the bulk
        shutil.rmtree(v / "replays", ignore_errors=True)
        tally, clusters = {}, []
        if (v / "verdicts.json").exists():
            for x in json.loads((v / "verdicts.json").read_text()):
                tally[x["verdict"]] = tally.get(x["verdict"], 0) + 1
            clusters = json.loads((v / "clusters.json").read_text())
        summary.append({"boundary": b, "candidates_total": len(cands), "adjudicated": len(use),
                        "tally": tally, "clusters": clusters[:12],
                        "seconds": round(time.time() - t0)})
        # verdicts are the record; the two adjudication binaries (2 x ~19 MB with debug info) and
        # their harness copies are rebuildable and are not kept per boundary (byte quota)
        for junk in (san, nos, cell / "harnesses" / f"{b}_san", cell / "harnesses" / f"{b}_nosan"):
            shutil.rmtree(junk, ignore_errors=True) if Path(junk).is_dir() else Path(junk).unlink(missing_ok=True)
        print(f"{b:28s} {len(use):6d}/{len(cands):<6d} {json.dumps(tally)}  "
              f"{len(clusters)} clusters  {round(time.time()-t0)}s", flush=True)

    agg: dict[str, int] = {}
    for s in summary:
        for k, n in (s.get("tally") or {}).items():
            agg[k] = agg.get(k, 0) + n
    (out / "summary.json").write_text(json.dumps(
        {"mode": tag, "sample_per_boundary": a.sample, "boundaries": summary, "total": agg},
        indent=1) + "\n")
    print(f"\nTOTAL ({tag}) {json.dumps(agg)}")
    print("CONFIRM_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
