#!/usr/bin/env python3

"""Boundary harvester — Stage B: validity labels via build + fuzz + classify.

Stage A (harvest_census.py) decided which matched-function boundaries are signature-constructible.
This stage attaches a VALIDITY label to each constructible boundary by actually building a
differential harness, fuzzing it briefly, and classifying any artifact with the existing
evidence-backed pipeline (classify_artifact.py). The label answers the boundary-validity question
the learned model will predict: does a SOUND differential harness exist here, or does a naive one
produce divergences/crashes that are NOT translation bugs?

For each constructible boundary (from dataset/boundaries_static.jsonl):
  gen --ignore-schema --expose-entry  ->  build  ->  fuzz DUR s  ->  classify each artifact

Summary label (per boundary), then a coarse VALIDITY for the model:
  NO_DIVERGENCE_OBSERVED            full-duration fuzz, no artifact            -> valid    (positive)
  C_UB_CONFIRMED         C UBSan fired on the same input           -> invalid  (negative)
  C_CRASH / RUST_PANIC   crash / panic, no isolated translation bug-> invalid  (negative)
  HARNESS_DIVERGENCE     outputs differ, cause not isolated        -> review   (set aside)
  NON_REPRODUCIBLE       replay outcomes varied                    -> review
  BUILD_FAIL / GEN_FAIL / FUZZER_EXITED_EARLY                      -> excluded (pipeline issue)

Output: dataset/boundaries.jsonl (features + validity + evidence + telemetry), results/stage_b_v1.md.
The per-boundary classifier JSONs land under results/classified/<pair>__<fn>/.

NOTE the deliberate conservatism: NO_DIVERGENCE_OBSERVED is "no divergence in DUR seconds", NOT proof of
equivalence — it is a weak positive and the label auditor should sanity-check that the fuzzer
actually exercised the boundary (executions > 0, the function isn't a trivial early-return).

Usage: DUR=20 python3 scripts/harvest_stage_b.py [--limit N] [--only pair__fn,...]
"""

from __future__ import annotations

import argparse
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
TOOLCHAIN = "nightly-2025-09-01"
DUR = int(os.environ.get("DUR", "20"))
SHARED_TARGET = ROOT / "fuzz_gen" / "_shared_target"
HARVEST = ROOT / "fuzz_gen" / "_harvest"
# UB-free differential testing (boundary-validity rule 4) is handled at the ORACLE level, NOT by
# restricting inputs or asserting overflow semantics: the fuzzer explores the FULL input space; a
# divergence whose C side triggers UB (UBSan/ASan, ANY class) is EXCLUDED as non-evidence (see
# scripts/ub_free_rescan.py / classify). We deliberately do NOT force -Coverflow-checks=off: that would
# ASSERT "C signed overflow == wrapping", which can HIDE a real divergence where the C binary exploits UB
# (e.g. at -O2) differently from wrapping. Strict exclude (option 1) makes no such assertion.
ENV = dict(os.environ,
           PATH=os.path.expanduser("~/.cargo/bin") + ":" + os.environ.get("PATH", ""),
           CARGO_TARGET_DIR=str(SHARED_TARGET))
_EXEC_RE = re.compile(r"executions?\s*[:=]?\s*(\d+)")

# summary label -> (validity bucket for the model). NOTE: NO_DIVERGENCE_OBSERVED is a WEAK positive
# ("fuzzing found nothing in DUR s"), NOT proof of equivalence — never call it VALID/equivalent.
VALIDITY = {
    "NO_DIVERGENCE_OBSERVED": "valid",
    "C_UB_CONFIRMED": "invalid", "C_CRASH": "invalid", "RUST_PANIC": "invalid",
    "HARNESS_DIVERGENCE": "review", "NON_REPRODUCIBLE": "review", "UNKNOWN": "review",
    "C_CRASH_NO_ARTIFACT": "invalid",  # ASan abort, no replayable artifact (recovered from log)
    "BUILD_FAIL": "excluded", "GEN_FAIL": "excluded", "FUZZER_EXITED_EARLY": "excluded",
}


sys.path.insert(0, str(TOOLS))
import gen_diff_harness as _gdh  # noqa: E402  (for the generator capability stamp)


def load_constructible(static_path: Path) -> list[dict]:
    rows = [json.loads(l) for l in static_path.read_text().splitlines() if l.strip()]
    return [r for r in rows if r.get("constructible")]


def gen(pair: Path, fn: str, out: Path) -> tuple[bool, str]:
    r = subprocess.run(
        ["python3", str(TOOLS / "gen_diff_harness.py"), "--pair", str(pair), "--entry", fn,
         "--ignore-schema", "--expose-entry", "--out", str(out)],
        capture_output=True, text=True, env=ENV)
    if r.returncode == 0:
        return True, ""
    msg = (r.stderr or r.stdout).strip()
    return False, (msg.splitlines()[-1] if msg else "gen failed")


def build(out: Path, crate: str) -> tuple[bool, str]:
    r = subprocess.run(["cargo", f"+{TOOLCHAIN}", "fuzz", "build", f"{crate}_ft"],
                       cwd=out, capture_output=True, text=True, env=ENV)
    return (r.returncode == 0), "\n".join(r.stderr.strip().splitlines()[-3:])


def run_fuzz(out: Path, crate: str, tag: str) -> dict:
    art_dir = out / "fuzz" / "artifacts" / f"{crate}_ft"
    if art_dir.exists():
        for f in art_dir.glob("*"):
            f.unlink()
    log_path = out / "run.log"
    logf = open(log_path, "wb")
    p = subprocess.Popen(["cargo", f"+{TOOLCHAIN}", "fuzz", "run", f"{crate}_ft"],
                         cwd=out, stdout=logf, stderr=subprocess.STDOUT, env=ENV,
                         start_new_session=True)
    t0 = time.time()
    terminated_by_timeout = False
    exit_code = None
    while True:
        if p.poll() is not None:
            exit_code = p.returncode
            break
        if time.time() - t0 >= DUR:
            terminated_by_timeout = True
            try:
                os.killpg(os.getpgid(p.pid), signal.SIGKILL)
            except ProcessLookupError:
                pass
            subprocess.run(["pkill", "-9", "-f", f"_harvest/{tag}"], env=ENV)
            break
        time.sleep(1)
    elapsed = round(time.time() - t0, 1)
    logf.close()
    log_txt = log_path.read_text(errors="ignore") if log_path.exists() else ""
    execs = max((int(x) for x in _EXEC_RE.findall(log_txt)), default=None)
    arts = sorted(art_dir.glob("*")) if art_dir.exists() else []
    # An ASan abort (SIGABRT) bypasses libFuzzer's artifact writer, so a real crash can exit early
    # with NO saved artifact. Detect it from the log so it is never mislabeled FUZZER_EXITED_EARLY.
    cm = re.search(r"(?:AddressSanitizer|ERROR: libFuzzer|UndefinedBehaviorSanitizer)[^\n]*", log_txt)
    crash_summary = cm.group(0).strip() if cm else None
    return {"artifact_count": len(arts), "artifacts": [str(a) for a in arts],
            "elapsed_seconds": elapsed, "exit_code": exit_code,
            "terminated_by_timeout": terminated_by_timeout, "executions": execs,
            "crash_summary": crash_summary}


def classify_all(pair: Path, fn: str, out: Path, tag: str, artifacts: list[str]) -> list[dict]:
    out_dir = ROOT / "results" / "classified" / tag
    out_dir.mkdir(parents=True, exist_ok=True)
    results = []
    for art in artifacts:
        sha = hashlib.sha256(Path(art).read_bytes()).hexdigest()
        rj = out_dir / f"{sha}.json"
        subprocess.run(
            ["python3", str(TOOLS / "classify_artifact.py"), "--pair", str(pair), "--entry", fn,
             "--ignore-schema", "--crate-dir", str(out), "--artifact", art, "--out", str(rj)],
            capture_output=True, text=True, env=ENV)
        try:
            label = json.loads(rj.read_text())["label"]
        except Exception:
            label = "CLASSIFY_ERROR"
        results.append({"sha256": sha, "label": label, "result": str(rj)})
    return results


def summary_label(run: dict, artifact_labels: list[str]) -> str:
    if artifact_labels:
        uniq = sorted(set(artifact_labels))
        return uniq[0] if len(uniq) == 1 else "MULTIPLE:" + ",".join(uniq)
    if run.get("terminated_by_timeout"):
        return "NO_DIVERGENCE_OBSERVED"
    # Early exit WITH a sanitizer signal in the log = a real crash, just no saved artifact to replay.
    if run.get("crash_summary"):
        return "C_CRASH_NO_ARTIFACT"
    return "FUZZER_EXITED_EARLY"


def main() -> int:
    ap = argparse.ArgumentParser(description="Stage B: validity labels for constructible boundaries")
    ap.add_argument("--limit", type=int, default=None, help="only the first N boundaries")
    ap.add_argument("--only", default=None, help="comma list of pair__fn tags to run")
    ap.add_argument("--static", default=str(ROOT / "dataset" / "boundaries_static.jsonl"),
                    help="constructibility census input (Stage A output)")
    ap.add_argument("--out", default=str(ROOT / "dataset" / "boundaries.jsonl"),
                    help="labeled dataset output (use a v2 path to avoid clobbering frozen v1)")
    args = ap.parse_args()

    HARVEST.mkdir(parents=True, exist_ok=True)
    boundaries = load_constructible(Path(args.static))
    gen_stamp = {"version": _gdh.GEN_VERSION, "capabilities": list(_gdh.GEN_CAPABILITIES)}
    only = set(args.only.split(",")) if args.only else None
    if only:
        boundaries = [b for b in boundaries if f"{b['pair']}__{b['fn']}" in only]
    if args.limit:
        boundaries = boundaries[:args.limit]

    rows = []
    for i, b in enumerate(boundaries, 1):
        pair_name, fn = b["pair"], b["fn"]
        tag = f"{pair_name}__{fn}"
        pair = ROOT / "benchmark" / "pairs" / pair_name
        crate = pair_name.replace("-", "_")
        out = HARVEST / tag
        row = {k: b[k] for k in b}  # carry every static feature + census field
        row["dur_seconds"] = DUR
        row["generator"] = gen_stamp  # version + capabilities at harvest time (v1/v2 disambiguation)
        # STU boundaries may legitimately fall on INTERNAL (static) functions, but they must never
        # be reported as public-API equivalence: tag scope so public/internal are reported apart.
        row["boundary_scope"] = "public" if b.get("pub_in_rust") else "internal"
        row["exposed_static"] = bool(b.get("needs_expose"))

        ok, msg = gen(pair, fn, out)
        if not ok:
            row.update(label="GEN_FAIL", validity="excluded", note=msg)
            rows.append(row); print(f"[{i}/{len(boundaries)}] {tag}: GEN_FAIL ({msg})"); continue
        ok, msg = build(out, crate)
        if not ok:
            row.update(label="BUILD_FAIL", validity="excluded", note=msg)
            rows.append(row); print(f"[{i}/{len(boundaries)}] {tag}: BUILD_FAIL"); continue

        run = run_fuzz(out, crate, tag)
        classified = classify_all(pair, fn, out, tag, run["artifacts"])
        labels = [c["label"] for c in classified]
        label = summary_label(run, labels)
        row.update(label=label, validity=VALIDITY.get(label.split(":")[0], "review"),
                   executions=run["executions"], elapsed_seconds=run["elapsed_seconds"],
                   terminated_by_timeout=run["terminated_by_timeout"],
                   artifact_count=run["artifact_count"], artifact_labels=classified,
                   crash_summary=run.get("crash_summary"))
        rows.append(row)
        print(f"[{i}/{len(boundaries)}] {tag}: {label} -> {row['validity']} "
              f"(exec={run['executions']}, arts={run['artifact_count']})")

    out_path = Path(args.out)
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")

    by_validity: dict[str, int] = {}
    by_label: dict[str, int] = {}
    scope_validity: dict[tuple, int] = {}
    for r in rows:
        by_validity[r["validity"]] = by_validity.get(r["validity"], 0) + 1
        by_label[r["label"]] = by_label.get(r["label"], 0) + 1
        sk = (r.get("boundary_scope", "?"), r["validity"])
        scope_validity[sk] = scope_validity.get(sk, 0) + 1
    n = len(rows)
    _tag = "v2" if "_v2" in out_path.stem else "v1"
    md = [f"# Boundary dataset {_tag} — Stage B (RAW harvest labels)\n",
          f"DUR={DUR}s/boundary; shared LibAFL cache; each artifact classified by classify_artifact.py "
          f"(--ignore-schema, exposed crate). Raw dataset: `dataset/{out_path.name}` "
          f"(generator {_gdh.GEN_VERSION}).\n",
          f"> These are RAW harvest labels. The AUDITED label is `validity_v2` "
          f"(see `scripts/audit_heuristics.py` + the audit report); do not train on these directly.\n",
          f"## {n} constructible boundaries labeled\n", "| validity | n |", "|---|---|"]
    for k in ("valid", "invalid", "review", "excluded"):
        if k in by_validity:
            md.append(f"| {k} | {by_validity[k]} |")
    # public vs internal reported SEPARATELY: an internal (exposed-static) boundary is a valid STU
    # learning target but is NOT public-API equivalence.
    md += ["\n## By boundary scope (public API vs exposed internal)\n",
           "| scope | valid | invalid | review | excluded |", "|---|---|---|---|---|"]
    for scope in ("public", "internal"):
        cells = [str(scope_validity.get((scope, v), 0)) for v in ("valid", "invalid", "review", "excluded")]
        md.append(f"| {scope} | " + " | ".join(cells) + " |")
    md += ["\n## Summary labels\n", "| label | n |", "|---|---|"]
    for k in sorted(by_label, key=lambda k: -by_label[k]):
        md.append(f"| {k} | {by_label[k]} |")
    md += ["\n> NO_DIVERGENCE_OBSERVED = no divergence in DUR s — a **weak positive**, not proof of equivalence. "
           "The label auditor verifies the fuzzer actually exercised each boundary and that each "
           "`invalid` is a real boundary property (UB/invariant), not a harness-inference artifact.\n"]
    (ROOT / "results" / f"stage_b_{_tag}.md").write_text("\n".join(md) + "\n", encoding="utf-8")
    print(f"\nwrote {n} rows to {out_path}; validity={by_validity}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
