#!/usr/bin/env python3

"""P2 (+P2.1): structured G1 support matrix over every benchmark ENTRY.

Records the full pipeline state per program — never collapses to bare CLEAN/DIVERGENCE:

  generator_supported -> generated -> built -> fuzzed
    -> run telemetry (elapsed_seconds, exit_code, terminated_by_timeout, executions, run_log)
    -> per-artifact conservative classification (classify_artifact.py)
    -> summary label

Key conservatism (project-lead review):
  * A run that ENDS EARLY (fuzzer died) with no artifact is FUZZER_EXITED_EARLY, NOT clean.
  * Only a full-duration run with no artifact is NO_DIVERGENCE_OBSERVED (NOT "CLEAN"/"equivalent" —
    30 s of fuzzing proves nothing about equivalence).
  * EVERY artifact is classified (G2/G3 will yield several per run); results saved under
    results/classified/<program>/<sha256>.json; the matrix records artifact_labels + root_cause_count.
  * Unsupported signatures (nested ptr / callback / ptr-to-array) -> UNSUPPORTED_SIGNATURE.

A shared CARGO_TARGET_DIR caches the LibAFL build across programs.

Usage: DUR=30 python3 scripts/run_g1_matrix.py
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import signal
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools" / "stu_selector"
sys.path.insert(0, str(TOOLS))
import frontier as fr  # noqa: E402

TOOLCHAIN = "nightly-2025-09-01"
DUR = int(os.environ.get("DUR", "30"))
SHARED_TARGET = ROOT / "fuzz_gen" / "_shared_target"
ENV = dict(os.environ,
           PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""),
           CARGO_TARGET_DIR=str(SHARED_TARGET))
_EXEC_RE = re.compile(r"(?:executions?|#)\s*[:=]?\s*(\d+)")


def entries_from_raw() -> dict[str, str | None]:
    return {c.stem: fr.parse_entry(c) for c in (ROOT / "benchmark" / "raw").rglob("*.c")}


def gen(pair: Path, entry: str) -> tuple[bool, str]:
    r = subprocess.run(
        ["python3", str(TOOLS / "gen_diff_harness.py"), "--pair", str(pair), "--entry", entry],
        capture_output=True, text=True)
    if r.returncode == 0:
        return True, ""
    out = (r.stderr or r.stdout).strip()
    return False, (out.splitlines()[-1] if out else "gen failed")


def build(prog: str) -> tuple[bool, str]:
    cd = ROOT / "fuzz_gen" / prog
    r = subprocess.run(["cargo", f"+{TOOLCHAIN}", "fuzz", "build", f"{prog}_ft"],
                       cwd=cd, capture_output=True, text=True, env=ENV)
    return (r.returncode == 0), "\n".join(r.stderr.strip().splitlines()[-3:])


def run_fuzz(prog: str) -> dict:
    """Fuzz for DUR seconds; capture telemetry so a crashed fuzzer is never read as 'clean'."""
    cd = ROOT / "fuzz_gen" / prog
    art_dir = cd / "fuzz" / "artifacts" / f"{prog}_ft"
    if art_dir.exists():
        for f in art_dir.glob("*"):
            f.unlink()
    log_path = cd / "run.log"
    logf = open(log_path, "wb")
    p = subprocess.Popen(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", f"{prog}_ft"],
                         cwd=cd, stdout=logf, stderr=subprocess.STDOUT,
                         env=ENV, start_new_session=True)
    t0 = time.time()
    terminated_by_timeout = False
    exit_code = None
    while True:
        if p.poll() is not None:
            exit_code = p.returncode          # fuzzer ended on its own (abnormal for LibAFL)
            break
        if time.time() - t0 >= DUR:
            terminated_by_timeout = True
            try:
                os.killpg(os.getpgid(p.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass
            subprocess.run(["pkill", "-9", "-f", f"fuzz_gen/{prog}"], env=ENV)
            break
        time.sleep(1)
    elapsed = round(time.time() - t0, 1)
    logf.close()
    log_txt = log_path.read_text(errors="ignore") if log_path.exists() else ""
    execs = None
    ms = _EXEC_RE.findall(log_txt)
    if ms:
        execs = max(int(x) for x in ms)
    arts = sorted(art_dir.glob("*")) if art_dir.exists() else []
    return {
        "artifact_count": len(arts), "artifacts": [str(a) for a in arts],
        "elapsed_seconds": elapsed, "exit_code": exit_code,
        "terminated_by_timeout": terminated_by_timeout, "executions": execs,
        "run_log": str(log_path),
    }


def classify_all(pair: Path, prog: str, entry: str, artifacts: list[str]) -> list[dict]:
    out_dir = ROOT / "results" / "classified" / prog
    out_dir.mkdir(parents=True, exist_ok=True)
    results = []
    for art in artifacts:
        sha = hashlib.sha256(Path(art).read_bytes()).hexdigest()
        out = out_dir / f"{sha}.json"
        subprocess.run(["python3", str(TOOLS / "classify_artifact.py"),
                        "--pair", str(pair), "--entry", entry, "--artifact", art, "--out", str(out)],
                       capture_output=True, text=True, env=ENV)
        try:
            label = json.loads(out.read_text())["label"]
        except Exception:
            label = "CLASSIFY_ERROR"
        results.append({"sha256": sha, "label": label, "result": str(out)})
    return results


def run_label(run: dict, artifact_labels: list[str]) -> str:
    """Pure decision: map run telemetry + per-artifact labels to a summary label."""
    if artifact_labels:
        uniq = sorted(set(artifact_labels))
        return uniq[0] if len(uniq) == 1 else "MULTIPLE:" + ",".join(uniq)
    if run.get("terminated_by_timeout"):
        return "NO_DIVERGENCE_OBSERVED"
    return "FUZZER_EXITED_EARLY"


def main() -> int:
    entries = entries_from_raw()
    pairs = sorted(p for p in (ROOT / "benchmark" / "pairs").iterdir()
                   if p.is_dir() and not p.name.startswith("_"))
    rows = []
    for pair in pairs:
        prog = pair.name
        entry = entries.get(prog)
        row = {"program": prog, "entry": entry, "generator_supported": False, "built": False,
               "fuzzed": False, "elapsed_seconds": None, "exit_code": None,
               "terminated_by_timeout": None, "executions": None, "artifact_count": 0,
               "artifact_labels": [], "root_cause_count": 0, "label": "", "note": ""}
        if not entry:
            row["label"] = "NO_ENTRY"; rows.append(row); print(f"[{prog}] NO_ENTRY"); continue

        ok, msg = gen(pair, entry)
        if not ok:
            row["label"] = "UNSUPPORTED_SIGNATURE" if "unsupported" in msg.lower() else "GEN_FAIL"
            row["note"] = msg; rows.append(row); print(f"[{prog}] {row['label']}: {msg}"); continue
        row["generator_supported"] = True

        ok, msg = build(prog)
        if not ok:
            row["label"] = "BUILD_FAIL"; row["note"] = msg; rows.append(row)
            print(f"[{prog}] BUILD_FAIL"); continue
        row["built"] = True

        run = run_fuzz(prog)
        row.update({k: run[k] for k in ("elapsed_seconds", "exit_code", "terminated_by_timeout",
                                        "executions", "artifact_count")})
        row["fuzzed"] = True
        classified = classify_all(pair, prog, entry, run["artifacts"])
        labels = [c["label"] for c in classified]
        row["artifact_labels"] = classified
        row["root_cause_count"] = len(set(labels))
        row["label"] = run_label(run, labels)
        rows.append(row)
        print(f"[{prog}] built, {run['artifact_count']} artifact(s), "
              f"elapsed={run['elapsed_seconds']}s timeout={run['terminated_by_timeout']} "
              f"-> {row['label']}")

    (ROOT / "results" / "g1_matrix.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
    cols = ["program", "entry", "generator_supported", "built", "elapsed_seconds",
            "terminated_by_timeout", "artifact_count", "root_cause_count", "label"]
    md = ["# G1 support matrix\n",
          f"DUR={DUR}s/program; shared LibAFL build; each artifact classified by classify_artifact.py.",
          "Labels: NO_DIVERGENCE_OBSERVED (full run, no artifact) / FUZZER_EXITED_EARLY / "
          "UNSUPPORTED_SIGNATURE / BUILD_FAIL / per-artifact classifier labels.\n",
          "| " + " | ".join(cols) + " |", "|" + "|".join("---" for _ in cols) + "|"]
    for r in rows:
        md.append("| " + " | ".join(str(r[c]) for c in cols) + " |")
    (ROOT / "results" / "g1_matrix.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print("\nwrote results/g1_matrix.json and results/g1_matrix.md")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
