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
# T** rectangular pointer table (matrix int**)
_RT = {"schema_version": 1, "params": [
    {"name": "mat", "role": "input_rectangular_pointer_table", "decode": "rectangular_pointer_table",
     "elem": "i32", "elem_width": 4, "outer_length_param": "rows", "inner_length_param": "cols",
     "outer_max": 16, "inner_max": 16, "mutation": "backing_observable"},
    {"name": "rows", "role": "length", "decode": "derived_from_buffer", "of_buffer": "mat",
     "rust": "usize", "width": 8},
    {"name": "cols", "role": "length", "decode": "derived_from_buffer", "of_buffer": "mat",
     "rust": "usize", "width": 8}]}
_RI = gdh.items_from_schema(_RT)
check("items_from_schema: T** -> in_table with dims",
      (_RI[0]["role"], _RI[0]["outer_name"], _RI[0]["inner_name"], _RI[0]["outer_max"]),
      ("in_table", "rows", "cols", 16))
_RCA, _RRA, _RDECL = gdh._call_and_decl(_RT["params"])
check("T** decl is *mut *mut elem", _RDECL[0], "mat: *mut *mut i32")
check("T** call uses per-side table ptr in ABI order",
      (_RCA, _RRA), (["mat_tab_c.as_mut_ptr()", "rows", "cols"], ["mat_tab_r.as_mut_ptr()", "rows", "cols"]))
check("T** decode reads dims then data",
      any("(cur.byte() as usize) % (16 + 1)" in d for d in gdh._decode_and_post(_RI)[0]), True)
check("validate: rectangular table clean", hs.validate(_RT), [])
_RT_BAD = {"schema_version": 1, "params": [dict(_RT["params"][0], outer_max=None), _RT["params"][1], _RT["params"][2]]}
check("validate: rectangular table missing outer_max flagged",
      any("outer_max" in e for e in hs.validate(_RT_BAD)), True)
_RT_BIG = {"schema_version": 1, "params": [dict(_RT["params"][0], outer_max=300), _RT["params"][1], _RT["params"][2]]}
check("validate: dimension max>255 flagged (1-byte)",
      any("[0,255]" in e for e in hs.validate(_RT_BIG)), True)
_RT_NOMUT = {"schema_version": 1, "params": [{k: v for k, v in _RT["params"][0].items() if k != "mutation"},
                                             _RT["params"][1], _RT["params"][2]]}
check("validate: missing mutation contract flagged",
      any("backing_observable" in e for e in hs.validate(_RT_NOMUT)), True)

# char** string-pointer table (word_tokens char**) — DISTINCT from the rectangular table:
# ptr_ptr + ONE scalar count, each backing an independent NUL-terminated string.
_CPP = [{"kind": "ptr_ptr", "const": False, "inner_const": False, "elem": "i8", "elem_w": 1, "name": "words"},
        {"kind": "scalar", "rust": "usize", "w": 8, "name": "count"}]
_CC = gdh.classify(_CPP)
check("classify: char** + one scalar -> in_str_table",
      (_CC[0]["role"], _CC[0]["len_name"], _CC[0]["count_max"], len(_CC)), ("in_str_table", "count", 16, 1))
_ST = {"schema_version": 1, "params": [
    {"name": "words", "role": "input_string_pointer_table", "decode": "string_pointer_table",
     "elem": "i8", "elem_width": 1, "length_param": "count", "count_max": 16, "mutation": "backing_observable"},
    {"name": "count", "role": "length", "decode": "derived_from_buffer", "of_buffer": "words",
     "rust": "usize", "width": 8}]}
_SI = gdh.items_from_schema(_ST)
check("items_from_schema: char** -> in_str_table",
      (_SI[0]["role"], _SI[0]["len_name"], _SI[0]["count_max"], len(_SI)), ("in_str_table", "count", 16, 1))
_SCA, _SRA, _SDECL = gdh._call_and_decl(_ST["params"])
check("char** decl is *mut *mut elem", _SDECL[0], "words: *mut *mut i8")
check("char** call uses per-side table ptr in ABI order",
      (_SCA, _SRA), (["words_tab_c.as_mut_ptr()", "count"], ["words_tab_r.as_mut_ptr()", "count"]))
_SDEC, _SPOST = gdh._decode_and_post(_SI)
check("char** decode reads count then per-item NUL strings",
      any("(cur.byte() as usize) % (16 + 1)" in d for d in _SDEC)
      and any("take_vec_i8" in d and "push(0 as i8)" in d for d in _SDEC), True)
check("char** post compares the two backings", any("string table words" in p for p in _SPOST), True)
check("validate: string table clean", hs.validate(_ST), [])
_ST_BIG = {"schema_version": 1, "params": [dict(_ST["params"][0], count_max=300), _ST["params"][1]]}
check("validate: string table count_max>255 flagged (1-byte)",
      any("[0,255]" in e for e in hs.validate(_ST_BIG)), True)
_ST_NOMUT = {"schema_version": 1, "params": [{k: v for k, v in _ST["params"][0].items() if k != "mutation"},
                                             _ST["params"][1]]}
check("validate: string table missing mutation contract flagged",
      any("backing_observable" in e for e in hs.validate(_ST_NOMUT)), True)
check("validate_against_signature: char** accepts input_string_pointer_table",
      hs.validate_against_signature(
          {"return": {"rust": "u64"}, "params": _ST["params"]},
          [{"kind": "ptr_ptr", "const": False, "inner_const": False, "elem": "i8", "elem_w": 1, "name": "words"},
           {"kind": "scalar", "rust": "usize", "w": 8, "name": "count"}], "u64"), [])

# POD struct-by-pointer (opcode_dispatch VM = {int32 stack[64]; int32 sp}) — construct by value.
_STSCH = {"schema_version": 1, "params": [
    {"name": "vm", "role": "inout_struct", "decode": "struct_value", "struct_name": "VM",
     "fields": [{"name": "stack", "kind": "array", "elem": "i32", "elem_width": 4, "extent": 4},
                {"name": "sp", "kind": "scalar", "rust": "i32", "width": 4}]}]}
_STI = gdh.items_from_schema(_STSCH)
check("items_from_schema: struct -> io_struct with name+fields",
      (_STI[0]["role"], _STI[0]["struct"]["name"], len(_STI[0]["struct"]["fields"])),
      ("io_struct", "VM", 2))
_STCA, _STRA, _STDECL = gdh._call_and_decl(_STSCH["params"])
check("struct decl is *mut translated::<Name>", _STDECL[0], "vm: *mut translated::VM")
check("struct call uses per-side &mut in ABI order",
      (_STCA, _STRA), (["&mut vm_c"], ["&mut vm_r"]))
_STDEC, _STPOST = gdh._decode_and_post(_STI)
check("struct decode builds literal (fields in decl order) + two copies",
      any("translated::VM { stack:" in d for d in _STDEC)
      and any("let mut vm_c = vm_val;" in d for d in _STDEC)
      and any("let mut vm_r = vm_val;" in d for d in _STDEC), True)
check("struct post compares fields (arrays + scalars impl PartialEq)",
      any("vm_c.stack != vm_r.stack || vm_c.sp != vm_r.sp" in p for p in _STPOST), True)
# real signature: POD struct pointer -> ptr_struct, classify -> io_struct
if (ROOT / "benchmark" / "pairs" / "opcode_dispatch" / "build").exists():
    _vp, _, _ = gdh.parse_entry_signature(ROOT / "benchmark" / "pairs" / "opcode_dispatch" / "build", "vm_pop")
    check("vm_pop param is ptr_struct (VM, pod)",
          (_vp[0]["kind"], _vp[0]["struct"]["name"], _vp[0]["struct"]["pod"]), ("ptr_struct", "VM", True))
    check("classify: POD struct pointer (mutable) -> io_struct", gdh.classify(_vp)[0]["role"], "io_struct")
    # derived struct schema validates and reproduces the inferred decode byte-for-byte
    _dsch = hs.derive(ROOT / "benchmark" / "pairs" / "opcode_dispatch" / "build", "vm_pop")
    _vp2, _ret2, _ = gdh.parse_entry_signature(ROOT / "benchmark" / "pairs" / "opcode_dispatch" / "build", "vm_pop")
    check("derive: struct schema role + name", (_dsch["params"][0]["role"], _dsch["params"][0]["struct_name"]),
          ("inout_struct", "VM"))
    check("derive: struct schema validates", hs.validate(_dsch), [])
    check("derive: struct schema matches signature", hs.validate_against_signature(_dsch, _vp2, _ret2), [])
    check("struct: schema-driven decode == inferred decode (byte-identical)",
          gdh._decode_and_post(gdh.items_from_schema(_dsch)) == gdh._decode_and_post(gdh.classify(_vp2)), True)

# struct-ARRAY buffer (hash_table Slot* slots + size_t cap) — POD struct ptr + size_t length => array.
# Distinct from a single struct ptr (vm_pop) and from a struct ptr + non-usize scalar (op_add operand).
_SASCH = {"schema_version": 1, "params": [
    {"name": "slots", "role": "inout_struct_array", "decode": "struct_array_vector", "struct_name": "Slot",
     "length_param": "cap", "fields": [{"name": "key", "kind": "scalar", "rust": "i32", "width": 4},
                                       {"name": "value", "kind": "scalar", "rust": "i32", "width": 4},
                                       {"name": "used", "kind": "scalar", "rust": "u8", "width": 1}]},
    {"name": "cap", "role": "length", "decode": "derived_from_buffer", "of_buffer": "slots",
     "rust": "usize", "width": 8}]}
_SAI = gdh.items_from_schema(_SASCH)
check("items_from_schema: struct-array -> io_struct_arr with len + struct",
      (_SAI[0]["role"], _SAI[0]["len_name"], _SAI[0]["struct"]["name"]), ("io_struct_arr", "cap", "Slot"))
_SACA, _SARA, _SADECL = gdh._call_and_decl(_SASCH["params"])
check("struct-array decl is *mut translated::<Name>", _SADECL[0], "slots: *mut translated::Slot")
check("struct-array call uses per-side as_mut_ptr in ABI order",
      (_SACA, _SARA), (["slots_c.as_mut_ptr()", "cap"], ["slots_r.as_mut_ptr()", "cap"]))
_SADEC, _SAPOST = gdh._decode_and_post(_SAI)
check("struct-array decode builds Vec<translated::Slot> sized by len",
      any("Vec<translated::Slot> = (0..cap).map(|_| translated::Slot {" in d for d in _SADEC), True)
check("struct-array post compares element-wise (no PartialEq on structs)",
      any("slots_c.iter().zip(slots_r.iter()).any(|(a, b)| a.key != b.key" in p for p in _SAPOST), True)
check("validate: struct-array clean", hs.validate(_SASCH), [])
check("validate: struct-array missing length_param flagged",
      any("length_param" in e for e in hs.validate({"schema_version": 1, "params": [
          {k: v for k, v in _SASCH["params"][0].items() if k != "length_param"}, _SASCH["params"][1]]})), True)
# real signature: Slot* + size_t cap -> io_struct_arr; the size_t disambiguates from single-struct
if (ROOT / "benchmark" / "pairs" / "hash_table" / "build").exists():
    _hp, _, _ = gdh.parse_entry_signature(ROOT / "benchmark" / "pairs" / "hash_table" / "build", "ht_insert_into")
    _hc = gdh.classify(_hp)
    check("classify: PODStruct* + size_t -> io_struct_arr (Slot[])",
          (_hc[0]["role"], _hc[0]["len_name"], _hc[0]["struct"]["name"]), ("io_struct_arr", "cap", "Slot"))

# real signature: invariant-bearing struct (DynArray{int* data; ...}) is hard-gated with a precise reason
if (ROOT / "benchmark" / "pairs" / "dynamic_array" / "build").exists():
    try:
        gdh.parse_entry_signature(ROOT / "benchmark" / "pairs" / "dynamic_array" / "build", "da_push")
        _da_err = "(no error)"
    except SystemExit as e:
        _da_err = str(e)
    check("non-POD struct (pointer field) gives a precise struct-invariant gate",
          "struct-invariant" in _da_err and "pointer field" in _da_err, True)

# bounded scalar
_BS = {"params": [{"name": "n", "role": "scalar", "decode": "bounded_scalar",
                   "rust": "usize", "width": 8, "min_value": 0, "max_value": 64}]}
_BI = gdh.items_from_schema(_BS)
check("items_from_schema: bounded scalar carries bounds",
      (_BI[0]["decode"], _BI[0]["min_value"], _BI[0]["max_value"]), ("bounded_scalar", 0, 64))
_BD, _ = gdh._decode_and_post(_BI)
check("bounded scalar decode maps into [min,max]",
      "(0 as usize) + (cur.take_usize() % ((64 - 0 + 1) as usize))" in _BD[0], True)
_BS_BAD = {"params": [{"name": "n", "role": "scalar", "decode": "bounded_scalar",
                       "rust": "usize", "width": 8}]}
check("validate_against_signature: bounded_scalar without bounds flagged",
      any("bounded_scalar needs" in e for e in hs.validate_against_signature(
          _BS_BAD, [{"kind": "scalar", "rust": "usize", "w": 8, "name": "n"}], "i32")), True)
if (ROOT / "schemas" / "graph_dfs.json").exists():
    _np = next(p for p in hs.load(ROOT / "schemas" / "graph_dfs.json")["params"] if p["name"] == "n")
    check("graph_dfs n is bounded_scalar", _np.get("decode"), "bounded_scalar")

# CLOBBER GUARD: derive() alone re-derives a PLAIN scalar (loses the manual bound); derive_merged()
# must re-apply the human annotation from the on-disk schema so `--all` is non-destructive.
_fresh = {"params": [{"name": "n", "role": "scalar", "decode": "scalar", "rust": "usize", "width": 8}]}
_exist = {"params": [{"name": "n", "role": "scalar", "decode": "bounded_scalar", "rust": "usize",
                      "width": 8, "min_value": 0, "max_value": 64}]}
_merged = hs.merge_overrides(_fresh, _exist)
check("merge_overrides re-applies bounded_scalar annotation",
      (_merged["params"][0]["decode"], _merged["params"][0]["min_value"], _merged["params"][0]["max_value"]),
      ("bounded_scalar", 0, 64))
check("merge_overrides carries observable_length/mutation policy",
      hs.merge_overrides({"params": [{"name": "d", "role": "output_buffer"}]},
                         {"params": [{"name": "d", "role": "output_buffer",
                                      "observable_length": {"kind": "return_value"}}]}
                         )["params"][0]["observable_length"]["kind"], "return_value")
if (ROOT / "benchmark" / "pairs" / "graph_dfs" / "build").exists():
    _gp = ROOT / "benchmark" / "pairs" / "graph_dfs"
    _gentry = hs.load(ROOT / "schemas" / "graph_dfs.json")["entry"]
    _raw_n = next(p for p in hs.derive(_gp / "build", _gentry)["params"] if p["name"] == "n")
    check("raw derive() WOULD clobber bounded n (so merge is load-bearing)", _raw_n.get("decode"), "scalar")
    _dm_n = next(p for p in hs.derive_merged(_gp / "build", _gentry,
                                             ROOT / "schemas" / "graph_dfs.json")["params"] if p["name"] == "n")
    check("derive_merged preserves bounded n (--all no longer clobbers)", _dm_n.get("decode"), "bounded_scalar")

# bounded scalar type-range validation (item 1)
def _bs_vas(rust, w, lo, hi):
    sch = {"return": {"rust": "i32"}, "params": [
        {"name": "x", "role": "scalar", "decode": "bounded_scalar", "rust": rust, "width": w,
         "min_value": lo, "max_value": hi}]}
    return hs.validate_against_signature(sch, [{"kind": "scalar", "rust": rust, "w": w, "name": "x"}], "i32")
check("bounded: usize negative lower bound flagged", any("outside" in e for e in _bs_vas("usize", 8, -1, 10)), True)
check("bounded: u8 max 300 flagged", any("outside" in e for e in _bs_vas("u8", 1, 0, 300)), True)
check("bounded: u64 full-range span flagged", any("not representable" in e for e in _bs_vas("u64", 8, 0, 2**64 - 1)), True)
check("bounded: valid [0,64] usize clean", _bs_vas("usize", 8, 0, 64), [])

# function-pointer (callback) deferral (item 4): describe_type expresses kind=function
if (ROOT / "benchmark" / "pairs" / "array_map_reduce" / "build").exists():
    try:
        gdh.parse_entry_signature(ROOT / "benchmark" / "pairs" / "array_map_reduce" / "build", "map_then_reduce")
        _cb_err = "(no error)"
    except SystemExit as e:
        _cb_err = str(e)
    check("callback param gives a clear deferral message", "callback" in _cb_err.lower(), True)

# ptr-to-array (input_fixed_array_buffer)
_ARR = {"params": [
    {"name": "edges", "role": "input_fixed_array_buffer", "decode": "fixed_array_vector",
     "elem": "usize", "elem_width": 8, "inner_extent": 2, "length_param": "m"},
    {"name": "m", "role": "length", "decode": "derived_from_buffer", "of_buffer": "edges",
     "rust": "usize", "width": 8}]}
_AI = gdh.items_from_schema(_ARR)
check("items_from_schema: ptr-to-array -> in_arr with inner_extent",
      (_AI[0]["role"], _AI[0]["inner_extent"], _AI[0]["len_name"]), ("in_arr", 2, "m"))
_ACA, _, _ADECL = gdh._call_and_decl(_ARR["params"])
check("ptr-to-array decl is *const [elem; extent]", _ADECL[0], "edges: *const [usize; 2]")
check("ptr-to-array call passes the buffer ptr in ABI order", _ACA, ["edges_buf.as_ptr()", "m"])
if (ROOT / "schemas" / "graph_dfs.json").exists():
    _gd = hs.load(ROOT / "schemas" / "graph_dfs.json")
    _ep = next(p for p in _gd["params"] if p["name"] == "edges")
    check("graph_dfs edges = input_fixed_array_buffer (extent 2)",
          (_ep["role"], _ep["inner_extent"]), ("input_fixed_array_buffer", 2))

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
