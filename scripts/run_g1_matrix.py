#!/usr/bin/env python3

"""P2: structured G1 support matrix over every benchmark ENTRY.

For each program it records the full pipeline state, not just CLEAN/DIVERGENCE:

  generator_supported -> generated -> built -> fuzzed -> artifact_count -> classifier_label

Entries the generator cannot handle (nested pointers, callbacks) are recorded as
UNSUPPORTED_SIGNATURE — they are neither "clean" nor a selector failure, just out of the
generator's current coverage. Every produced artifact is run through the P1 classifier
(classify_artifact.py) so each row carries an evidence-backed conservative label.

A shared CARGO_TARGET_DIR caches the LibAFL build across programs (first build slow, rest fast).

Output: results/g1_matrix.json + results/g1_matrix.md

Usage: DUR=30 python3 scripts/run_g1_matrix.py
"""

from __future__ import annotations

import json
import os
import signal
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools" / "stu_selector"
sys.path.insert(0, str(TOOLS))
import frontier as fr  # noqa: E402  (for // ENTRY parsing)

TOOLCHAIN = "nightly-2025-09-01"
DUR = int(os.environ.get("DUR", "30"))
SHARED_TARGET = ROOT / "fuzz_gen" / "_shared_target"
ENV = dict(os.environ,
           PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""),
           CARGO_TARGET_DIR=str(SHARED_TARGET))


def entries_from_raw() -> dict[str, str | None]:
    out = {}
    for c in (ROOT / "benchmark" / "raw").rglob("*.c"):
        out[c.stem] = fr.parse_entry(c)
    return out


def gen(pair: Path, entry: str) -> tuple[bool, str]:
    r = subprocess.run(
        ["python3", str(TOOLS / "gen_diff_harness.py"), "--pair", str(pair), "--entry", entry],
        capture_output=True, text=True)
    if r.returncode == 0:
        return True, ""
    msg = (r.stderr or r.stdout).strip().splitlines()[-1] if (r.stderr or r.stdout).strip() else "gen failed"
    return False, msg


def build(prog: str) -> tuple[bool, str]:
    cd = ROOT / "fuzz_gen" / prog
    r = subprocess.run(["cargo", f"+{TOOLCHAIN}", "fuzz", "build", f"{prog}_ft"],
                       cwd=cd, capture_output=True, text=True, env=ENV)
    if r.returncode == 0:
        return True, ""
    return False, "\n".join(r.stderr.strip().splitlines()[-3:])


def fuzz(prog: str) -> int:
    cd = ROOT / "fuzz_gen" / prog
    art_dir = cd / "fuzz" / "artifacts" / f"{prog}_ft"
    if art_dir.exists():
        for f in art_dir.glob("*"):
            f.unlink()
    p = subprocess.Popen(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", f"{prog}_ft"],
                         cwd=cd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                         env=ENV, start_new_session=True)
    t0 = time.time()
    while time.time() - t0 < DUR:
        if p.poll() is not None:
            break
        time.sleep(1)
    # stop the (runaway) LibAFL fuzzer: kill the process group + backstop pkill by unique name.
    try:
        os.killpg(os.getpgid(p.pid), signal.SIGKILL)
    except ProcessLookupError:
        pass
    subprocess.run(["pkill", "-9", "-f", f"fuzz_gen/{prog}"], env=ENV)
    time.sleep(1)
    return len(list(art_dir.glob("*"))) if art_dir.exists() else 0


def classify(pair: Path, prog: str, entry: str) -> str:
    art_dir = ROOT / "fuzz_gen" / prog / "fuzz" / "artifacts" / f"{prog}_ft"
    arts = sorted(art_dir.glob("*")) if art_dir.exists() else []
    if not arts:
        return "CLEAN"
    out = ROOT / "results" / "classified" / f"{prog}.json"
    out.parent.mkdir(parents=True, exist_ok=True)
    r = subprocess.run(["python3", str(TOOLS / "classify_artifact.py"),
                        "--pair", str(pair), "--entry", entry,
                        "--artifact", str(arts[0]), "--out", str(out)],
                       capture_output=True, text=True, env=ENV)
    try:
        return json.loads(out.read_text())["label"]
    except Exception:
        return "CLASSIFY_ERROR"


def main() -> int:
    entries = entries_from_raw()
    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_"))
    rows = []
    for pair in pairs:
        prog = pair.name
        entry = entries.get(prog)
        row = {"program": prog, "entry": entry, "generator_supported": False,
               "generated": False, "built": False, "fuzzed": False,
               "artifact_count": 0, "label": "", "note": ""}
        if not entry:
            row["label"] = "NO_ENTRY"
            rows.append(row); print(f"[{prog}] NO_ENTRY"); continue

        ok, msg = gen(pair, entry)
        if not ok:
            row["generator_supported"] = False
            row["label"] = "UNSUPPORTED_SIGNATURE" if "unsupported" in msg.lower() else "GEN_FAIL"
            row["note"] = msg
            rows.append(row); print(f"[{prog}] {row['label']}: {msg}"); continue
        row["generator_supported"] = True
        row["generated"] = True

        ok, msg = build(prog)
        if not ok:
            row["label"] = "BUILD_FAIL"; row["note"] = msg
            rows.append(row); print(f"[{prog}] BUILD_FAIL"); continue
        row["built"] = True

        row["artifact_count"] = fuzz(prog)
        row["fuzzed"] = True
        row["label"] = classify(pair, prog, entry)
        rows.append(row)
        print(f"[{prog}] built ok, artifacts={row['artifact_count']}, label={row['label']}")

    (ROOT / "results" / "g1_matrix.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")

    cols = ["program", "entry", "generator_supported", "built", "artifact_count", "label"]
    md = ["# G1 support matrix\n", f"DUR={DUR}s per program; classifier = classify_artifact.py\n",
          "| " + " | ".join(cols) + " |", "|" + "|".join("---" for _ in cols) + "|"]
    for r in rows:
        md.append("| " + " | ".join(str(r[c]) for c in cols) + " |")
    (ROOT / "results" / "g1_matrix.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("\nwrote results/g1_matrix.json and results/g1_matrix.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
