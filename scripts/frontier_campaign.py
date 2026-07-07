#!/usr/bin/env python3
"""Frontier UB-free campaign (the loop-closer experiment).

Fuzz the boundaries the selector calls SAFE (frontier v2 members), with the in-loop
UB-free gate ON. By elimination, any divergence at a SAFE + UB-free boundary is a REAL
problem: either a genuine c2rust translation bug OR a selector false-safe. ZERO is also a
result (= empirical precision of the SAFE claim). C-UB is excluded (unsafe-origin), never a
fidelity bug. See memory differential-fuzzing-state / framing-and-related-work.

Usage: DUR=30 python3 scripts/frontier_campaign.py [prog ...]   (no args = all programs)
"""
import hashlib
import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools" / "stu_selector"
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(TOOLS))
import stu_frontier as sf  # noqa: E402

TOOLCHAIN = "nightly-2025-09-01"
DUR = int(os.environ.get("DUR", "30"))
SHARED = ROOT / "fuzz_gen" / "_shared_target"
ENV = dict(os.environ, PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""),
           CARGO_TARGET_DIR=str(SHARED))


def gen(pair: Path, entry: str, out: Path) -> tuple[bool, str]:
    cmd = ["python3", str(TOOLS / "gen_diff_harness.py"), "--pair", str(pair), "--entry", entry,
           "--infer-schema", "--expose-entry", "--ub-free", "--out", str(out)]
    r = subprocess.run(cmd, capture_output=True, text=True)
    return (r.returncode == 0), ((r.stderr or r.stdout).strip().splitlines() or ["gen failed"])[-1]


def build(out: Path, crate: str) -> tuple[bool, str]:
    r = subprocess.run(["cargo", f"+{TOOLCHAIN}", "fuzz", "build", f"{crate}_ft"],
                       cwd=out, capture_output=True, text=True, env=ENV)
    return (r.returncode == 0), "\n".join(r.stderr.strip().splitlines()[-3:])


def fuzz(out: Path, crate: str, tag: str) -> list[str]:
    art_dir = out / "fuzz" / "artifacts" / f"{crate}_ft"
    if art_dir.exists():
        for f in art_dir.glob("*"):
            f.unlink()
    logf = open(out / "run.log", "wb")
    p = subprocess.Popen(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", f"{crate}_ft"],
                         cwd=out, stdout=logf, stderr=subprocess.STDOUT, env=ENV, start_new_session=True)
    t0 = time.time()
    while time.time() - t0 < DUR and p.poll() is None:
        time.sleep(1)
    if p.poll() is None:
        try:
            os.killpg(os.getpgid(p.pid), signal.SIGKILL)
        except ProcessLookupError:
            pass
    subprocess.run(["pkill", "-9", "-f", tag], env=ENV)
    time.sleep(1)
    logf.close()
    return sorted(str(a) for a in art_dir.glob("*")) if art_dir.exists() else []


def classify(pair: Path, prog: str, entry: str, art: str) -> str:
    no_schema = not (ROOT / "schemas" / f"{prog}.json").exists()
    out = ROOT / "results" / "classified" / f"{prog}__{entry}" / (hashlib.sha256(Path(art).read_bytes()).hexdigest() + ".json")
    out.parent.mkdir(parents=True, exist_ok=True)
    cmd = ["python3", str(TOOLS / "classify_artifact.py"), "--pair", str(pair),
           "--entry", entry, "--artifact", art, "--out", str(out)]
    if no_schema:
        cmd.append("--ignore-schema")
    subprocess.run(cmd, capture_output=True, text=True, env=ENV)
    try:
        return json.loads(out.read_text())["label"]
    except Exception:
        return "CLASSIFY_ERROR"


def main() -> int:
    only = set(sys.argv[1:])
    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_") and (not only or p.name in only))
    print(f"frontier UB-free campaign | DUR={DUR}s | {len(pairs)} program(s)")
    rows = []
    for pair in pairs:
        info = sf.analyze(pair)
        if info.get("error"):
            continue
        for member in info.get("frontier_v2_members", []):
            prog = pair.name
            out = ROOT / "fuzz_gen" / f"fc_{prog}__{member}"
            tag = f"fc_{prog}__{member}"
            row = {"program": prog, "frontier_member": member, "label": ""}
            ok, msg = gen(pair, member, out)
            if not ok:
                row["label"] = "GEN_FAIL"; row["note"] = msg; rows.append(row)
                print(f"  [{prog}:{member}] GEN_FAIL: {msg}"); continue
            ok, msg = build(out, prog)
            if not ok:
                row["label"] = "BUILD_FAIL"; row["note"] = msg; rows.append(row)
                print(f"  [{prog}:{member}] BUILD_FAIL"); continue
            arts = fuzz(out, prog, tag)
            if not arts:
                row["label"] = "NO_DIVERGENCE"
            else:
                labels = sorted({classify(pair, prog, member, a) for a in arts})
                row["label"] = labels[0] if len(labels) == 1 else "MULTIPLE:" + ",".join(labels)
            rows.append(row)
            print(f"  [{prog}:{member}] -> {row['label']}")

    # summary by category
    from collections import Counter
    cats = Counter(r["label"] for r in rows)
    (ROOT / "results" / "ub_free_campaign_v1.json").write_text(json.dumps(rows, indent=2))
    md = ["# Frontier UB-free campaign (loop-closer: fuzz SAFE boundaries)\n",
          f"DUR={DUR}s/boundary, in-loop UB-free gate ON. A divergence at a SAFE + UB-free "
          "boundary = real c2rust bug OR selector false-safe. C_UB_CONFIRMED = excluded "
          "(unsafe-origin), NOT a fidelity bug.\n",
          f"**Summary:** {dict(cats)}\n",
          "| program | frontier_member (SAFE boundary) | result |",
          "|---|---|---|"]
    for r in rows:
        md.append(f"| {r['program']} | {r['frontier_member']} | {r['label']} |")
    (ROOT / "results" / "ub_free_campaign_v1.md").write_text("\n".join(md) + "\n")
    print(f"\nsummary: {dict(cats)}")
    print("wrote results/ub_free_campaign_v1.{json,md}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
