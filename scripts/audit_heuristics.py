#!/usr/bin/env python3

"""Semi-automated label audit for the boundary dataset.

The v1 dataset was audited by 4 independent agents, which overturned 17% of labels. Hand-spawning
agents every harvest does not scale. This encodes the PATTERNS those agents found as fast heuristics
so that, per harvest, most labels are auto-classified with high confidence and only a small FLAGGED
subset needs a human/agent spot-check.

Per labeled boundary it derives the inferred parameter ROLES (from the generator), reads the UBSan
signal from the classification evidence, and assigns:

  validity_v2 in {valid, invalid_intrinsic_ub, invalid_isolation_invariant, excluded_generator,
                  weak_exclude}  + audit_auto = {verdict, confidence, flag}

Heuristics (conservative — when unsure, FLAG rather than guess):
  INVALID + UBSan "overflow"/"divide"  + no struct input        -> invalid_intrinsic_ub      (auto)
  INVALID + UBSan "out of bounds"/"index" + struct input        -> invalid_isolation_invariant(auto)
  INVALID + C_CRASH (no sanitizer signal)                       -> FLAG (mis-model / crash risk)
  INVALID + out-of-bounds but NO struct input                   -> FLAG (possible array-as-single)
  VALID   + very low semantic work (few C stmts) or pure reader -> FLAG (weak/trivial positive)
  VALID   + a non-length size_t param (cursor index risk)       -> FLAG (cursor mis-infer)
  VALID   otherwise (real work, exercised)                      -> valid                     (auto)

Usage:
  python3 scripts/audit_heuristics.py --in dataset/boundaries_v2.jsonl --out dataset/boundaries_v2.jsonl
  python3 scripts/audit_heuristics.py --in dataset/boundaries.jsonl --validate   # check vs v1 agent verdicts
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TOOLS = ROOT / "tools" / "stu_selector"
sys.path.insert(0, str(TOOLS))
import gen_diff_harness as gdh  # noqa: E402

_STRUCT_ROLES = {"in_struct", "io_struct", "in_struct_arr", "io_struct_arr"}
_LOW_STMT = 4  # at/below this many C statements, a "no divergence" positive is suspect (trivial)


def derive_roles(pair: str, fn: str) -> set | None:
    cc = ROOT / "benchmark" / "pairs" / pair / "build"
    try:
        _, _, _, items, _ = gdh.resolve(pair, cc, fn, ignore_schema=True)
        return {it["role"] for it in items}
    except SystemExit:
        return None
    except Exception:  # noqa: BLE001
        return None


def ubsan_signal(row: dict) -> str:
    """Concatenated UBSan sanitizer messages from the classification evidence, lowercased."""
    msgs = []
    for art in row.get("artifact_labels", []):
        p = art.get("result")
        if not p or not Path(p).exists():
            continue
        try:
            r = json.loads(Path(p).read_text())
            msgs += r.get("c_run", {}).get("sanitizers", []) or []
        except Exception:  # noqa: BLE001
            pass
    return " ; ".join(msgs).lower()


def n_size_t_params(row: dict) -> int:
    return int(row.get("n_pointer_params", 0))  # proxy; refined below via roles


def classify_boundary(row: dict, roles: set | None, ub: str) -> dict:
    """Return {validity_v2, verdict, confidence, flag}."""
    label = row.get("label", "")
    has_struct = bool(roles and (roles & _STRUCT_ROLES))
    oob = ("out of bounds" in ub) or ("index" in ub) or ("out-of-bounds" in ub)
    # intrinsic scalar UB phrasings UBSan emits (overflow / divide / negate-INT_MIN / shift-too-large)
    intrinsic = any(k in ub for k in (
        "overflow", "divide", "division", "cannot be represented", "negation",
        "shift exponent", "too large for", "out of range for"))

    if row["validity"] == "invalid":
        if label == "C_UB_CONFIRMED" and intrinsic and not has_struct:
            return _v("invalid_intrinsic_ub", "intrinsic arithmetic UB (overflow/divide), scalar/buffer input", "high")
        if label == "C_UB_CONFIRMED" and oob and has_struct:
            return _v("invalid_isolation_invariant", "OOB from a trusted struct field (isolation invariant)", "high")
        if label == "C_UB_CONFIRMED" and oob and not has_struct:
            return _flag("invalid", "OOB on a non-struct input — possible array-as-single mis-model", "low")
        if label == "C_CRASH":
            return _flag("invalid", "C_CRASH with no UBSan signal — mis-model / crash risk", "low")
        if label == "RUST_PANIC":
            return _flag("invalid", "Rust panic with no C UB — needs review", "low")
        # invalid but unclear
        return _flag("invalid", f"invalid via {label}, evidence unclear", "low")

    if row["validity"] == "valid":
        cstmts = int(row.get("c_stmts", 0))
        if cstmts <= _LOW_STMT:
            return _flag("valid", f"low semantic work (c_stmts={cstmts}) — possible trivial/weak positive", "low")
        if roles is not None:
            # a buffer whose length is its own len is fine; a lone size_t scalar alongside a buffer
            # is a cursor-index risk (the harness can't tell length from index).
            if "scalar" in roles and any(b in roles for b in ("in_buf", "io_buf", "in_str")):
                return _flag("valid", "buffer + free scalar — possible cursor/index mis-infer", "low")
        return _v("valid", "real work, exercised, no divergence", "medium")

    return _v(row["validity"], "non-binary label carried through", "n/a")


def _v(vv, verdict, conf):
    return {"validity_v2": vv, "verdict": verdict, "confidence": conf, "flag": False}


def _flag(vv, verdict, conf):
    return {"validity_v2": vv, "verdict": verdict, "confidence": conf, "flag": True}


def main() -> int:
    ap = argparse.ArgumentParser(description="Semi-automated boundary label audit")
    ap.add_argument("--in", dest="inp", required=True)
    ap.add_argument("--out", default=None, help="write annotated dataset here (default: no write)")
    ap.add_argument("--validate", action="store_true",
                    help="compare auto-verdicts to an existing validity_v2 (agent ground truth)")
    args = ap.parse_args()

    rows = [json.loads(l) for l in Path(args.inp).read_text().splitlines() if l.strip()]
    labeled = [r for r in rows if r.get("validity") in ("valid", "invalid")]
    flagged, auto = [], 0
    agree = disagree = 0
    for r in rows:
        if r.get("validity") not in ("valid", "invalid"):
            continue
        roles = derive_roles(r["pair"], r["fn"])
        ub = ubsan_signal(r)
        a = classify_boundary(r, roles, ub)
        if args.validate and "validity_v2" in r:
            # Map agent buckets to the heuristic's binary-ish comparison.
            gt = r["validity_v2"]
            # treat weak_exclude/excluded_generator as "flag" expectation
            gt_flag = gt in ("weak_exclude", "excluded_generator")
            if gt_flag == a["flag"]:
                agree += 1
            else:
                disagree += 1
                print(f"  MISMATCH {r['pair']}__{r['fn']}: agent={gt} heuristic={'FLAG' if a['flag'] else a['validity_v2']} ({a['verdict']})")
        r["audit_auto"] = a
        r["validity_v2_auto"] = a["validity_v2"]
        if a["flag"]:
            flagged.append(f"{r['pair']}__{r['fn']}  [{r['label']}]  {a['verdict']}")
        else:
            auto += 1

    if args.out:
        Path(args.out).write_text("".join(json.dumps(r) + "\n" for r in rows), encoding="utf-8")

    print(f"\nlabeled: {len(labeled)} | auto-classified: {auto} | FLAGGED for spot-check: {len(flagged)}")
    for f in flagged:
        print("  FLAG:", f)
    if args.validate:
        tot = agree + disagree
        print(f"\nvalidation vs agent ground truth: {agree}/{tot} agree on flag-vs-auto "
              f"({100*agree//max(tot,1)}%)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
