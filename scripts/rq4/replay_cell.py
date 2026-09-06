#!/usr/bin/env python3
"""PROTOCOL.md section 4, step 6: combined replay of the campaign's own corpus.

`cell.py` runs the campaign Rust-only (coverage is a property of the translation alone) and keeps
the crash artifacts that campaign produced. Those artifacts are TERMINATIONS. The value-divergence
channel -- the S3 / S10 / S11 / S12 shape, where both sides return and disagree -- only exists
when the C reference runs beside the translation on the same input. So every saved corpus input
is replayed once more here in `C2R_MODE=combined` with the comparison ladder on, and every input
whose outcome is not `normal` becomes a candidate under `divergences/<boundary>/`.

Same corpus, same budget: candidates from this step are candidates ON the coverage corpus, which
is what keeps the coverage measurement and the divergence search one experiment.

`ub-gated` inputs (the in-loop noise filter fired: the C side hit a UB check) are counted and
listed, not promoted -- confirmation re-adjudicates everything under ASan + full UBSan anyway.

This is a replay of a few hundred inputs per boundary, not a campaign; it still runs AFTER the
cells, never beside one.

usage: scripts/rq4/replay_cell.py --cell <dir from cell.py> [--timeout 20] [--only a,b]
"""
from __future__ import annotations

import argparse
import json
import shutil
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import c2r_campaign as C  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--timeout", type=float, default=20.0)
    ap.add_argument("--only")
    a = ap.parse_args()
    cell = Path(a.cell)
    only = {s.strip() for s in a.only.split(",")} if a.only else None
    funnel = json.loads((cell / "funnel.json").read_text())
    out = cell / "divergences"
    out.mkdir(exist_ok=True)
    work = cell / "replay_work"
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir()

    report = []
    for row in funnel:
        b = row["boundary"]
        if only and b not in only:
            continue
        binp = cell / "harnesses" / f"{b}.bin"
        corpus = cell / "corpus" / b
        if not (binp.exists() and corpus.is_dir()):
            continue
        t0 = time.time()
        tally: dict[str, int] = {}
        kept = out / b
        shutil.rmtree(kept, ignore_errors=True)
        kept.mkdir()
        inputs = sorted(p for p in corpus.iterdir() if p.is_file())
        details = []
        for i, inp in enumerate(inputs):
            r = C.run_once(binp, "combined", inp, a.timeout, work, f"{b}_{i:04d}")
            k = r["outcome"]
            tally[k] = tally.get(k, 0) + 1
            if k != "normal":
                shutil.copy(inp, kept / f"{k}-{inp.name}")
                details.append({"input": inp.name, "outcome": k, "reported": r["reported"],
                                "phase": r["phase"], "sanitizer": r["sanitizer"],
                                "top_frames": r["top_frames"][:2],
                                "detail": (r["stderr_tail"].split("C2R_OUTCOME")[-1][:160]
                                           if "C2R_OUTCOME" in r["stderr_tail"] else "")})
        (kept / "_outcomes.json").write_text(json.dumps(details, indent=1) + "\n")
        report.append({"boundary": b, "inputs": len(inputs), "tally": tally,
                       "candidates": sum(v for k, v in tally.items()
                                         if k not in ("normal", "ub-gated")),
                       "seconds": round(time.time() - t0)})
        print(f"{b:28s} {len(inputs):5d} inputs  {json.dumps(tally)}  {round(time.time()-t0)}s",
              flush=True)
    shutil.rmtree(work, ignore_errors=True)
    (out / "summary.json").write_text(json.dumps(report, indent=1) + "\n")
    agg: dict[str, int] = {}
    for r in report:
        for k, v in r["tally"].items():
            agg[k] = agg.get(k, 0) + v
    print(f"\nTOTAL {json.dumps(agg)}")
    print("REPLAY_DONE")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
