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


# ---- items_from_schema: roles drive the items; length param is consumed, not standalone ----
_SYNTH = {"params": [
    {"name": "buf", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1,
     "length_param": "n"},
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "buf",
     "rust": "usize", "width": 8},
    {"name": "k", "role": "scalar", "decode": "scalar", "rust": "i32", "width": 4}]}
_ITEMS = gdh.items_from_schema(_SYNTH)
check("items_from_schema: count (length consumed)", len(_ITEMS), 2)
check("items_from_schema: buffer -> in_buf",
      (_ITEMS[0]["role"], _ITEMS[0]["name"], _ITEMS[0]["len_name"]), ("in_buf", "buf", "n"))
check("items_from_schema: scalar role", (_ITEMS[1]["role"], _ITEMS[1]["name"]), ("scalar", "k"))
check("items_from_schema: output_buffer -> io_buf with capacity as len",
      gdh.items_from_schema({"params": [
          {"name": "d", "role": "output_buffer", "decode": "vector", "elem": "u8",
           "elem_width": 1, "capacity_param": "c", "written_length": "return"},
          {"name": "c", "role": "capacity", "decode": "derived_from_buffer", "of_buffer": "d",
           "rust": "usize", "width": 8}]})[0]["len_name"], "c")


# ---- ABI order: a length param that PRECEDES its buffer stays first in the call/signature ----
_ABI = [
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "buf",
     "rust": "usize", "width": 8},
    {"name": "buf", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1,
     "length_param": "n"},
    {"name": "mode", "role": "scalar", "decode": "scalar", "rust": "i32", "width": 4}]
_CA, _RA, _DECL = gdh._call_and_decl(_ABI)
check("ABI order: call args follow schema order (length first)",
      _CA, ["n", "buf_buf.as_ptr()", "mode"])
check("ABI order: extern decl follows schema order",
      _DECL, ["n: usize", "buf: *const u8", "mode: i32"])

# ---- validate_against_signature ----
_SIG_PARAMS = [
    {"kind": "ptr", "const": True, "elem": "u8", "elem_w": 1, "name": "buf"},
    {"kind": "scalar", "rust": "usize", "w": 8, "name": "n"}]
_SIG_SCHEMA = {"schema_version": 1, "entry": "f", "return": {"rust": "i32"}, "params": [
    {"name": "buf", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1,
     "length_param": "n"},
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "buf",
     "rust": "usize", "width": 8}]}
check("validate_against_signature: matching", hs.validate_against_signature(_SIG_SCHEMA, _SIG_PARAMS, "i32"), [])
_SIG_BAD = {"params": [
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "buf",
     "rust": "usize", "width": 8},
    {"name": "buf", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1,
     "length_param": "n"}]}
check("validate_against_signature: order mismatch flagged",
      any("mismatch" in e for e in hs.validate_against_signature(_SIG_BAD, _SIG_PARAMS, "i32")), True)

# counterexamples: each must produce a non-empty error (these previously slipped through)
import copy as _copy
_GP = [
    {"kind": "ptr", "const": True, "elem": "u8", "elem_w": 1, "name": "buf"},
    {"kind": "scalar", "rust": "usize", "w": 8, "name": "n"},
    {"kind": "scalar", "rust": "i32", "w": 4, "name": "k"}]
_GS = {"schema_version": 1, "entry": "f", "program": "p", "return": {"rust": "i32"}, "params": [
    {"name": "buf", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1, "length_param": "n"},
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "buf", "rust": "usize", "width": 8},
    {"name": "k", "role": "scalar", "decode": "scalar", "rust": "i32", "width": 4}]}
check("vas baseline clean", hs.validate_against_signature(_GS, _GP, "i32"), [])

def _vas_err(mut_ret="i32", **mut):
    s = _copy.deepcopy(_GS)
    for path, val in mut.items():
        idx, key = path.split("_", 1)
        s["params"][int(idx)][key] = val
    return hs.validate_against_signature(s, _GP, mut_ret)

check("vas: wrong return type flagged", any("return type" in e for e in _vas_err(mut_ret="u32")), True)
check("vas: wrong elem_width flagged", any("elem_width" in e for e in _vas_err(**{"0_elem_width": 2})), True)
check("vas: wrong scalar width flagged", any("width" in e for e in _vas_err(**{"2_width": 8})), True)
check("vas: const ptr as output_buffer flagged",
      any("mutable pointer" in e for e in _vas_err(**{"0_role": "output_buffer"})), True)

_OFB = _copy.deepcopy(_GS); _OFB["params"][1]["of_buffer"] = "k"
check("validate: of_buffer != referencing buffer flagged",
      any("referencing buffer" in e for e in hs.validate(_OFB)), True)

# length referenced by two buffers must be flagged (exactly-one-owner)
_TWO_OWNERS = {"schema_version": 1, "params": [
    {"name": "a", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1, "length_param": "n"},
    {"name": "b", "role": "input_buffer", "decode": "vector", "elem": "u8", "elem_width": 1, "length_param": "n"},
    {"name": "n", "role": "length", "decode": "derived_from_buffer", "of_buffer": "a", "rust": "usize", "width": 8}]}
check("validate: length owned by 2 buffers flagged",
      any("exactly 1" in e for e in hs.validate(_TWO_OWNERS)), True)


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
