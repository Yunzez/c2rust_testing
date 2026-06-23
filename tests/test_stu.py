#!/usr/bin/env python3
"""Unit tests for the STU pipeline decision logic (no fuzzing / no network).

Run: python3 tests/test_stu.py
Covers: gen_diff_harness.safe_name, classify_artifact.classify decision table,
run_g1_matrix.run_label (incl. the fuzzer-exited-early path).
"""

import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools" / "stu_selector"))
sys.path.insert(0, str(ROOT / "scripts"))

import gen_diff_harness as gdh          # noqa: E402
import classify_artifact as ca          # noqa: E402
import run_g1_matrix as mx              # noqa: E402
import harness_schema as hs             # noqa: E402

CASES = []


def check(name, got, want):
    ok = got == want
    CASES.append((name, ok, got, want))


# ---- safe_name ----
check("safe_name keyword 'in'", gdh.safe_name("in", 0), "in_")
check("safe_name keyword 'fn'", gdh.safe_name("fn", 0), "fn_")
check("safe_name keyword 'type'", gdh.safe_name("type", 3), "type_")
check("safe_name normal", gdh.safe_name("buf", 1), "buf")
check("safe_name empty -> argN", gdh.safe_name("", 2), "arg2")


# ---- classify decision table ----
def cls(diff, c, rust):
    return ca.classify(diff, c, rust)[0]

check("C_UB (overflow + rust panic)",
      cls({"available": True, "reproducible": True, "outcome": "rust_panic"},
          {"sanitizers": ["signed integer overflow"]}, {"panicked": True}),
      "C_UB_CONFIRMED")
check("NON_REPRODUCIBLE",
      cls({"available": True, "reproducible": False, "outcome": "rust_panic", "outcomes": ["rust_panic", "clean"]},
          {"sanitizers": []}, {}),
      "NON_REPRODUCIBLE")
check("RUST_PANIC (panic, no UB)",
      cls({"available": True, "reproducible": True, "outcome": "rust_panic"},
          {"sanitizers": []}, {"panicked": True}),
      "RUST_PANIC")
check("HARNESS_DIVERGENCE (values differ, no UB)",
      cls({"available": True, "reproducible": True, "outcome": "harness_divergence"},
          {"sanitizers": []}, {"panicked": False}),
      "HARNESS_DIVERGENCE")
check("C_CRASH (asan)",
      cls({"available": True, "reproducible": True, "outcome": "crash"},
          {"sanitizers": [], "asan_crash": True}, {}),
      "C_CRASH")
check("C_CRASH (diff crash, no sanitizer signal)",
      cls({"available": True, "reproducible": True, "outcome": "crash"},
          {"sanitizers": [], "asan_crash": False}, {}),
      "C_CRASH")
check("UNKNOWN (no diff binary, nothing)",
      cls({"available": False}, {"sanitizers": []}, {}),
      "UNKNOWN")
# classifier must never auto-emit a translation-bug label
for nm, lbl in [("c_ub", "C_UB_CONFIRMED")]:
    pass
auto = {cls({"available": True, "reproducible": True, "outcome": o}, {"sanitizers": s}, {"panicked": p})
        for o in ("rust_panic", "harness_divergence", "crash")
        for s in ([], ["signed integer overflow"]) for p in (True, False)}
check("never auto TRANSLATION_BUG_*",
      any(x.startswith("TRANSLATION_BUG") for x in auto), False)


# ---- run_label (runner decision incl. abnormal exit) ----
check("run_label single artifact",
      mx.run_label({"terminated_by_timeout": True}, ["C_UB_CONFIRMED"]), "C_UB_CONFIRMED")
check("run_label multiple artifacts",
      mx.run_label({"terminated_by_timeout": True}, ["C_UB_CONFIRMED", "RUST_PANIC"]),
      "MULTIPLE:C_UB_CONFIRMED,RUST_PANIC")
check("run_label full run, no artifact -> NO_DIVERGENCE_OBSERVED",
      mx.run_label({"terminated_by_timeout": True}, []), "NO_DIVERGENCE_OBSERVED")
check("run_label early exit, no artifact -> FUZZER_EXITED_EARLY",
      mx.run_label({"terminated_by_timeout": False}, []), "FUZZER_EXITED_EARLY")


# ---- harness_schema.validate ----
_GOOD = {"schema_version": 1, "program": "p", "entry": "e", "return": {"rust": "i32"},
         "params": [
             {"name": "src", "role": "input_buffer", "decode": "vector", "elem": "u8",
              "elem_width": 1, "length_param": "len"},
             {"name": "len", "role": "length", "decode": "derived_from_buffer", "of_buffer": "src",
              "rust": "usize", "width": 8}]}
check("validate good schema", hs.validate(_GOOD), [])
_BADROLE = {"schema_version": 1, "params": [{"name": "x", "role": "frobnicate", "decode": "scalar"}]}
check("validate bad role flagged", any("bad role" in e for e in hs.validate(_BADROLE)), True)
_DANGLING = {"schema_version": 1, "params": [
    {"name": "src", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1,
     "length_param": "nope"}]}
check("validate dangling length_param flagged",
      any("unknown param" in e for e in hs.validate(_DANGLING)), True)

# ---- persisted schemas carry the reviewed roles ----
SCHEMAS = ROOT / "schemas"
if (SCHEMAS / "rle_codec.json").exists():
    def role_of(prog, pname):
        s = hs.load(SCHEMAS / f"{prog}.json")
        return next(p["role"] for p in s["params"] if p["name"] == pname)
    check("rle_codec dst = output_buffer", role_of("rle_codec", "dst"), "output_buffer")
    check("rle_codec dst_cap = capacity", role_of("rle_codec", "dst_cap"), "capacity")
    check("mergesort a = inout_buffer", role_of("mergesort_search", "a"), "inout_buffer")
    check("rpn_eval result = out_scalar", role_of("rpn_eval", "result"), "out_scalar")
    check("intmath op = scalar", role_of("intmath", "op"), "scalar")


def main():
    failed = [c for c in CASES if not c[1]]
    for name, ok, got, want in CASES:
        print(f"  {'PASS' if ok else 'FAIL'}  {name}" + ("" if ok else f"  got={got!r} want={want!r}"))
    print(f"\n{len(CASES) - len(failed)}/{len(CASES)} passed")
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
