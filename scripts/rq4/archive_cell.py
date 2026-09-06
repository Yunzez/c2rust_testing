#!/usr/bin/env python3
"""Archive one finished cell into results/rq3_coverage/<lib>/<tool>/ (PROTOCOL.md section 11).

What goes into git, and why:
  funnel.json, plans.json, analysis/ (result.json + four identity lists), analysis@<cp>s/result.json,
  snapshots.json, recollect*.json, tests-side pointer, artifact_hashes.json      -- the cell's numbers
  divergences/summary.json + per-boundary _outcomes.json + the divergence INPUTS -- small, and they
                                                                                   are the evidence
  confirm*/summary.json + per-boundary verdicts.json / clusters.json             -- adjudication
  harnesses/<b>/coverage_cmd.log, the generated fuzz target + build.rs per harness -- reproducibility
  corpus.tar.gz                                                                 -- the ONE corpus (§4)
What stays out: crash artifacts by the ten-thousand (their sha256 list is kept, plus the first 20
per boundary as `candidates_sample/`), harness binaries, cargo target dirs.

usage: scripts/rq4/archive_cell.py --cell <dir> --lib bzip2 --tool c2rust --pair <dir> [--dest results/rq3_coverage]
"""
from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
from pathlib import Path


def sha(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--lib", required=True)
    ap.add_argument("--tool", required=True)
    ap.add_argument("--pair", required=True)
    ap.add_argument("--dest", default="results/rq3_coverage")
    a = ap.parse_args()
    cell, pair = Path(a.cell), Path(a.pair)
    dest = Path(a.dest) / a.lib / a.tool
    dest.mkdir(parents=True, exist_ok=True)
    # Re-archiving replaces the cell's own outputs; raw/ and the tests-side files are not ours to touch.
    for stale in list(dest.glob("confirm*")) + [dest / d for d in
                  ("analysis", "divergences", "harnesses", "candidates_sample", "fuzz_logs")] \
                 + list(dest.glob("analysis@*")) + [dest / "candidates_manifest.json"]:
        if stale.is_dir():
            shutil.rmtree(stale, ignore_errors=True)
        elif stale.exists():
            stale.unlink()

    def cp(src: Path, rel: str | None = None):
        if src.exists():
            d = dest / (rel or src.name)
            d.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy(src, d)

    for name in ("funnel.json", "plans.json", "snapshots.json", "plan.log"):
        cp(cell / name)
    for f in cell.glob("recollect*.json"):
        cp(f)
    for d in sorted(cell.glob("analysis*")):
        if d.is_dir():
            for f in d.iterdir():
                cp(f, f"{d.name}/{f.name}")
    # divergences: summary, per-boundary outcomes, and the inputs themselves (small)
    dv = cell / "divergences"
    if dv.is_dir():
        for f in dv.rglob("*"):
            if f.is_file():
                cp(f, f"divergences/{f.relative_to(dv)}")
    # confirmation: summaries + verdicts/clusters, never the replays/ trees. A verdict row carries
    # a 1.5 KB stderr tail; over 8 500 rows that was 48 MB for one cell. The tail is kept only on
    # rows that PROMOTE something (confirmed_*), and the file is gzipped. confirm.log is the
    # per-candidate replay narration; verdicts.json is its content, so it is not archived.
    import gzip
    for cdir in sorted(cell.glob("confirm*")):
        if cdir.is_dir():
            cp(cdir / "summary.json", f"{cdir.name}/summary.json")
            for v in cdir.glob("*_verdicts"):
                cp(v / "clusters.json", f"{cdir.name}/{v.name}/clusters.json")
                vj = v / "verdicts.json"
                if vj.exists():
                    rows = json.loads(vj.read_text())
                    # every input a confirmed_* verdict rests on is evidence: keep it whole
                    b = v.name[: -len("_verdicts")]
                    for r in rows:
                        if str(r.get("verdict", "")).startswith("confirmed_"):
                            for src in (cell / "candidates" / b / r["artifact"],
                                        cell / "divergences" / b / r["artifact"]):
                                if src.exists():
                                    cp(src, f"confirmed_inputs/{b}/{r['artifact']}")
                                    break
                    for r in rows:
                        if not str(r.get("verdict", "")).startswith("confirmed_"):
                            for side in ("c_only", "rust_only", "combined", "rust_no_sanitizer"):
                                if isinstance(r.get(side), dict):
                                    r[side].pop("stderr_tail", None)
                    d = dest / cdir.name / v.name / "verdicts.json.gz"
                    d.parent.mkdir(parents=True, exist_ok=True)
                    with gzip.open(d, "wt") as fh:
                        json.dump(rows, fh)
    # harnesses: what the generator wrote, the coverage command log, never binaries
    hs = cell / "harnesses"
    for h in sorted(p for p in hs.iterdir() if p.is_dir()):
        for f in list(h.glob("fuzz/fuzz_targets/*.rs")) + [h / "build.rs", h / "coverage_cmd.log",
                                                            h / "Cargo.toml"]:
            cp(f, f"harnesses/{h.name}/{f.name}")
    # candidates: sha256 manifest of everything, first 20 inputs per boundary kept
    cands = cell / "candidates"
    manifest = {}
    if cands.is_dir():
        for b in sorted(p for p in cands.iterdir() if p.is_dir()):
            files = sorted(p for p in b.iterdir() if p.is_file())
            manifest[b.name] = {"count": len(files), "sha256": [sha(p) for p in files]}
            for p in files[:20]:
                cp(p, f"candidates_sample/{b.name}/{p.name}")
        # fork-mode parent logs run to ~10 MB each over an hour (185 MB per cell); keep the head
        # (flags, seed, seed-input count) and the tail (the final counters), not the heartbeat.
        for lg in cands.glob("*.fuzz.log"):
            lines = lg.read_text(errors="replace").splitlines()
            keep = lines[:12] + (["... [%d lines elided] ..." % (len(lines) - 212)] if len(lines) > 212 else []) + lines[-200:]
            d = dest / "fuzz_logs" / lg.name
            d.parent.mkdir(parents=True, exist_ok=True)
            d.write_text("\n".join(keep) + "\n")
    with gzip.open(dest / "candidates_manifest.json.gz", "wt") as fh:
        json.dump(manifest, fh)
    # the one corpus
    subprocess.run(["tar", "czf", str(dest / "corpus.tar.gz"), "-C", str(cell), "corpus"], check=True)
    # the raw llvm-cov exports the partition was computed from (final + snapshot re-collects)
    ours_dirs = [d.name for d in cell.glob("ours*") if d.is_dir() and not d.name.startswith("ours_mix")]
    subprocess.run(["tar", "czf", str(dest / "harness_exports.tar.gz"), "-C", str(cell), *ours_dirs],
                   check=True)
    # provenance hashes
    hashes = {"pair": {}, "cell": {}}
    for f in sorted((pair / "source").glob("*")):
        hashes["pair"][f"source/{f.name}"] = sha(f)
    for f in sorted((pair / "translated").glob("*")):
        hashes["pair"][f"translated/{f.name}"] = sha(f)
    for f in sorted(hs.glob("*.bin")):
        hashes["cell"][f"harnesses/{f.name}"] = sha(f)
    hashes["cell"]["corpus.tar.gz"] = sha(dest / "corpus.tar.gz")
    (dest / "artifact_hashes.json").write_text(json.dumps(hashes, indent=1) + "\n")
    total = sum(f.stat().st_size for f in dest.rglob("*") if f.is_file())
    print(f"archived {a.lib}/{a.tool} -> {dest}  ({total/1e6:.1f} MB, corpus {((dest/'corpus.tar.gz').stat().st_size)/1e6:.1f} MB)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
