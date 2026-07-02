#!/usr/bin/env python3

"""Generate a differential fuzz harness for a selected STU (entry function).

Given a c2rust pair (C source + translated .rs) and an entry function name, emit a
cargo-fuzz project that:
  - compiles the C source as an oracle, renaming every C function `f` -> `c_f`
    (sanitizer-coverage instrumented), so it does not collide with the #[no_mangle]
    Rust translation;
  - uses the translated .rs as the lib crate;
  - decodes the fuzz bytes into the entry's parameters (via a byte cursor), calls both
    `c_<entry>` and `translated::<entry>`, and panics on any divergence (return value or
    mutated/out buffers).

This is the harness side of the STU pipeline; used for G1 validation (on equivalent c2rust
output the false-divergence rate should be ~0). Supported parameter shapes: scalars,
`const T* + len` input buffers, `T* + len` in/out buffers, and `T*` out-scalars. Function
pointers / nested pointers are not supported (those boundaries are hard-gated by the STU
selector anyway).

Usage:
  python3 tools/stu_selector/gen_diff_harness.py --pair benchmark/pairs/intmath --entry intmath_eval
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402
import clang.cindex  # noqa: E402
from clang.cindex import CursorKind, Index, TypeKind  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]

# Generator capability stamp — recorded on every harvested dataset row so v1/v2 (built with
# different generator coverage) are never confused. Bump GEN_VERSION when adding a boundary shape.
GEN_VERSION = "0.5"
GEN_CAPABILITIES = (
    "scalar", "bounded_scalar", "input_buffer", "inout_buffer", "output_buffer", "out_scalar",
    "input_string", "input_fixed_array_buffer", "input_rectangular_pointer_table",
    "input_string_pointer_table", "input_struct", "inout_struct",
    "input_struct_array", "inout_struct_array", "output_array",
)

# c2rust type spelling -> (rust_ffi_type, byte_width). size_t -> usize to match translation.
SCALAR_MAP = {
    "size_t": ("usize", 8),
    "uint64_t": ("u64", 8), "int64_t": ("i64", 8),
    "uint32_t": ("u32", 4), "int32_t": ("i32", 4),
    "uint16_t": ("u16", 2), "int16_t": ("i16", 2),
    "uint8_t": ("u8", 1), "int8_t": ("i8", 1),
    "int": ("i32", 4), "unsigned int": ("u32", 4), "unsigned": ("u32", 4),
    "long": ("i64", 8), "unsigned long": ("u64", 8),
    "long long": ("i64", 8), "unsigned long long": ("u64", 8),
    "char": ("i8", 1), "signed char": ("i8", 1), "unsigned char": ("u8", 1),
    "_Bool": ("bool", 1),
    "float": ("f32", 4), "double": ("f64", 8),
}


def map_scalar(spelling: str) -> tuple[str, int] | None:
    s = spelling.replace("const ", "").strip()
    return SCALAR_MAP.get(s)


# C param names that are Rust keywords/reserved would produce invalid Rust in the harness.
RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "async", "await", "abstract", "become", "box", "do",
    "final", "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "gen",
}


def safe_name(name: str, idx: int) -> str:
    if not name:
        return f"arg{idx}"
    return f"{name}_" if name in RUST_KEYWORDS else name


MAX_TYPE_DEPTH = 12  # depth guard: recursive structs (struct Node { Node* next; }) would recurse
                     # forever -> a crash (RecursionError) is NOT a clean exclusion, so cap the
                     # descent and return a clean `unsupported` (recursive/too-deep) instead.


def describe_type(t, _depth: int = 0) -> dict:
    """Recursive, canonical-aware type descriptor (shared by ptr-to-array and future T**).

    Typedef/elaborated wrappers are peeled to reveal the structural kind, so `typedef size_t
    Edge[2]; const Edge *` is recognized as pointer->array. The scalar leaf prefers the ORIGINAL
    spelling (keeps size_t->usize) and falls back to the canonical one.

    `_depth` bounds the structural descent so recursive/self-referential records (a struct whose
    field points back at the struct) yield a clean UNSUPPORTED rather than a RecursionError crash.
    """
    if _depth > MAX_TYPE_DEPTH:
        return {"kind": "unsupported", "spelling": t.spelling, "reason": "recursive/too-deep type"}
    s = t
    seen = 0
    while s.kind in (TypeKind.TYPEDEF, TypeKind.ELABORATED) and seen < 8:
        s = s.get_canonical()
        seen += 1
    if s.kind == TypeKind.POINTER:
        pointee = s.get_pointee()
        return {"kind": "pointer", "const": pointee.is_const_qualified(),
                "inner": describe_type(pointee, _depth + 1)}
    if s.kind == TypeKind.CONSTANTARRAY:
        et = s.element_type
        return {"kind": "array", "extent": s.element_count,
                "const": et.is_const_qualified(), "elem": describe_type(et, _depth + 1)}
    if s.kind in (TypeKind.FUNCTIONPROTO, TypeKind.FUNCTIONNOPROTO):
        return {"kind": "function", "spelling": t.spelling}
    sc = map_scalar(t.spelling) or map_scalar(s.spelling)
    if sc:
        return {"kind": "scalar", "rust": sc[0], "width": sc[1]}
    if s.kind == TypeKind.RECORD:
        return _describe_record(t, s, _depth)
    return {"kind": "unsupported", "spelling": t.spelling}


def _rust_record_name(t) -> str:
    """The Rust type name c2rust gives this record: the C typedef/tag spelling, qualifiers stripped."""
    name = t.spelling
    for q in ("const ", "volatile ", "struct ", "union "):
        name = name.replace(q, "")
    return name.strip()


def _describe_record(t, s, _depth: int = 0) -> dict:
    """Struct/union descriptor: fields in declaration order + a POD verdict.

    POD (this increment) = every field is a scalar or a fixed array of scalars. Pointer / nested
    struct / function-pointer / union fields make the struct non-POD; we keep a precise reason so
    the boundary census can report WHY (a strong static feature for the validity model)."""
    decl = s.get_declaration()
    is_union = decl.kind == CursorKind.UNION_DECL
    fields = []
    pod = not is_union
    reason = "is a union" if is_union else None
    for f in decl.get_children():
        if f.kind != CursorKind.FIELD_DECL:
            continue
        fd = describe_type(f.type, _depth + 1)
        fields.append({"name": f.spelling, "desc": fd})
        if pod:
            if fd["kind"] == "scalar":
                pass
            elif fd["kind"] == "array" and fd["elem"]["kind"] == "scalar":
                pass
            elif fd["kind"] == "pointer":
                pod, reason = False, f"has pointer field '{f.spelling}'"
            elif fd["kind"] == "struct":
                pod, reason = False, f"has nested struct field '{f.spelling}'"
            elif fd["kind"] == "function":
                pod, reason = False, f"has function-pointer field '{f.spelling}'"
            else:
                pod, reason = False, f"has unsupported field '{f.spelling}' ({fd.get('kind')})"
    return {"kind": "struct", "name": _rust_record_name(t), "fields": fields,
            "pod": pod, "reason": reason}


def _param_from_descriptor(desc: dict, name: str) -> dict:
    """Map a type descriptor to a harness param. T** is recognized but deferred."""
    if desc["kind"] == "scalar":
        return {"kind": "scalar", "rust": desc["rust"], "w": desc["width"], "name": name}
    if desc["kind"] == "pointer":
        inner = desc["inner"]
        if inner["kind"] == "scalar":
            return {"kind": "ptr", "const": desc["const"],
                    "elem": inner["rust"], "elem_w": inner["width"], "name": name}
        if inner["kind"] == "array" and inner["elem"]["kind"] == "scalar":
            return {"kind": "ptr_array", "const": desc["const"] or inner["const"],
                    "elem": inner["elem"]["rust"], "elem_w": inner["elem"]["width"],
                    "inner_extent": inner["extent"], "name": name}
        if inner["kind"] == "struct":
            if not inner["pod"]:
                raise SystemExit(f"struct-invariant param {name}: {inner['name']} "
                                 f"{inner['reason']} (needs invariant reconstruction)")
            return {"kind": "ptr_struct", "const": desc["const"], "struct": inner, "name": name}
        if inner["kind"] == "function":
            raise SystemExit(f"callback parameter {name} deferred: function pointers "
                             f"(callback binding) not yet supported")
        if inner["kind"] == "pointer":
            inner2 = inner["inner"]
            if inner2["kind"] == "scalar":
                return {"kind": "ptr_ptr", "const": desc["const"], "inner_const": inner["const"],
                        "elem": inner2["rust"], "elem_w": inner2["width"], "name": name}
            raise SystemExit(f"unsupported: pointer-to-pointer-to-{inner2.get('kind')} param {name}")
        raise SystemExit(f"unsupported pointer target for {name}: {inner.get('kind')} {inner.get('spelling','')}")
    raise SystemExit(f"unsupported param type for {name}: {desc.get('spelling', desc['kind'])}")


def _param_usage(fn, names: set) -> dict:
    """Lightweight body analysis: for each parameter, is it the BASE of an array subscript
    (`p[..]`) and/or used in an INDEX position (`a[p]`). Distinguishes an output ARRAY (`dst[i]`,
    subscripted) from a single out-scalar (`*result`, not subscripted), and finds slice indices."""
    usage = {n: {"subscripted": False, "used_as_index": False} for n in names}

    def walk(node):
        if node.kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            ch = list(node.get_children())
            if len(ch) >= 2:
                base = {t.spelling for t in ch[0].get_tokens()}
                idxt = {t.spelling for t in ch[1].get_tokens()}
                for n in names:
                    if n in base:
                        usage[n]["subscripted"] = True
                    if n in idxt:
                        usage[n]["used_as_index"] = True
        for c in node.get_children():
            walk(c)
    walk(fn)
    return usage


def parse_entry_signature(cc_dir: Path, entry: str) -> tuple[list[dict], str, list[str]]:
    """Return (params, ret_rust_type, all_function_names) for the entry via libclang.
    Each param dict also carries body-usage flags (subscripted / used_as_index)."""
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    import os
    cwd0 = os.getcwd()
    params: list[dict] = []
    ret = "i32"
    all_fns: list[str] = []
    for cmd in cdb.getAllCompileCommands():
        src_abs = str((Path(cmd.directory) / cmd.filename).resolve())
        names = {cmd.filename, src_abs, Path(cmd.filename).name}
        args = cgmod._filter_compile_args(list(cmd.arguments), names)
        os.chdir(cmd.directory if Path(cmd.directory).exists() else cc_dir)
        try:
            tu = index.parse(src_abs, args=args)
        finally:
            os.chdir(cwd0)
        for cur in tu.cursor.walk_preorder():
            if cur.kind != CursorKind.FUNCTION_DECL or not cur.is_definition():
                continue
            f = cur.location.file.name if cur.location and cur.location.file else None
            if not f or f.startswith(("/usr/", "/lib/")):
                continue
            all_fns.append(cur.spelling)
            if cur.spelling != entry:
                continue
            arg_cursors = list(cur.get_arguments())
            usage = _param_usage(cur, {a.spelling for a in arg_cursors if a.spelling})
            for idx, a in enumerate(arg_cursors):
                pname = safe_name(a.spelling, idx)
                pd = _param_from_descriptor(describe_type(a.type), pname)
                u = usage.get(a.spelling, {})
                pd["subscripted"] = u.get("subscripted", False)
                pd["used_as_index"] = u.get("used_as_index", False)
                params.append(pd)
            rt = cur.result_type
            if rt.kind == TypeKind.VOID:
                ret = "void"
            else:
                sc = map_scalar(rt.spelling)
                ret = sc[0] if sc else "i32"
    return params, ret, sorted(set(all_fns))


def items_from_schema(schema: dict) -> list[dict]:
    """Reproduce classify()'s items from an explicit schema (schemas/<prog>.json).

    Roles come from the schema, not adjacency. The mapping is exact so that switching the
    generator from classify() to the schema is byte-identical (the schema was derived from
    classify()). output_buffer maps to the same io_buf decode as today (capacity used as the
    vector length) — the richer semantics are deferred to keep byte-identity.
    """
    items = []
    # A buffer derives its length from `.len()` (usize). When the C length param is NOT size_t
    # (real libs use `unsigned int` etc.), the call must cast to the param's actual Rust type.
    # Map every param name -> its declared rust type so buffer items can carry their len's type.
    param_rust = {p["name"]: p.get("rust") for p in schema["params"]}
    for p in schema["params"]:
        role = p["role"]
        if role == "scalar":
            it = {"kind": "scalar", "role": "scalar", "name": p["name"],
                  "rust": p["rust"], "w": p["width"], "decode": p.get("decode", "scalar")}
            if p.get("decode") == "bounded_scalar":
                it["max_value"] = p["max_value"]
                if "min_var" in p:
                    it["min_var"] = p["min_var"]
                else:
                    it["min_value"] = p["min_value"]
            items.append(it)
        elif role == "input_buffer":
            items.append({"kind": "ptr", "role": "in_buf", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "len_name": p["length_param"],
                          "len_rust": param_rust.get(p["length_param"]) or "usize"})
        elif role in ("inout_buffer", "output_buffer"):
            ln = p.get("length_param") or p.get("capacity_param")
            items.append({"kind": "ptr", "role": "io_buf", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "len_name": ln,
                          "len_rust": param_rust.get(ln) or "usize"})
        elif role == "out_scalar":
            items.append({"kind": "ptr", "role": "out_scalar", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"]})
        elif role == "output_array":
            items.append({"kind": "ptr", "role": "out_arr", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "cap": p["cap"]})
        elif role == "input_string":
            items.append({"kind": "ptr", "role": "in_str", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"]})
        elif role == "input_fixed_array_buffer":
            items.append({"kind": "ptr_array", "role": "in_arr", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "inner_extent": p["inner_extent"], "len_name": p["length_param"],
                          "len_rust": param_rust.get(p["length_param"]) or "usize"})
        elif role == "input_rectangular_pointer_table":
            items.append({"kind": "ptr_ptr", "role": "in_table", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "outer_name": p["outer_length_param"], "inner_name": p["inner_length_param"],
                          "outer_max": p["outer_max"], "inner_max": p["inner_max"]})
        elif role == "input_string_pointer_table":
            items.append({"kind": "ptr_ptr", "role": "in_str_table", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "len_name": p["length_param"], "count_max": p["count_max"]})
        elif role in ("input_struct", "inout_struct"):
            items.append({"kind": "ptr_struct",
                          "role": "in_struct" if role == "input_struct" else "io_struct",
                          "name": p["name"],
                          "struct": _struct_from_schema(p["struct_name"], p["fields"])})
        elif role in ("input_struct_array", "inout_struct_array"):
            items.append({"kind": "ptr_struct",
                          "role": "in_struct_arr" if role == "input_struct_array" else "io_struct_arr",
                          "name": p["name"], "len_name": p["length_param"],
                          "struct": _struct_from_schema(p["struct_name"], p["fields"])})
        # length / capacity params are consumed by their owning buffer (not standalone items)
    return items


def _infer_abi(params: list[dict]) -> list[dict]:
    """Synthesize an ABI-ordered schema-param list from inference (the --infer-schema fallback)."""
    its = classify(params)
    by = {p["name"]: p for p in params}
    buf_role, len_owner, table_meta = {}, {}, {}
    for it in its:
        if it["role"] == "in_buf":
            buf_role[it["name"]] = "input_buffer"; len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "io_buf":
            buf_role[it["name"]] = "inout_buffer"; len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "in_arr":
            buf_role[it["name"]] = "input_fixed_array_buffer"; len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "in_table":
            buf_role[it["name"]] = "input_rectangular_pointer_table"; table_meta[it["name"]] = it
            len_owner[it["outer_name"]] = (it["name"], "length")
            len_owner[it["inner_name"]] = (it["name"], "length")
        elif it["role"] == "in_str_table":
            buf_role[it["name"]] = "input_string_pointer_table"; table_meta[it["name"]] = it
            len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] in ("in_struct_arr", "io_struct_arr"):
            len_owner[it["len_name"]] = (it["name"], "length")
    out_scalars = {it["name"] for it in its if it["role"] == "out_scalar"}
    # A non-const out pointer that is SUBSCRIPTED (`dst[i]`) is an OUTPUT ARRAY, not a single
    # out-scalar (`*result`). If any exists, the function indexes a cap-sized array, so its bare
    # usize scalars are slice indices -> bound them to the cap (keeps a[lo..hi] in bounds: this is
    # both the output-array fix and the sliced-buffer fix, with no param grouping).
    OUT_ARR_CAP = 64
    out_arrays = {n for n in out_scalars if by[n].get("subscripted") and not by[n].get("const")}
    has_io_arr = bool(out_arrays)
    struct_items = {it["name"]: it for it in its
                    if it["role"] in ("in_struct", "io_struct", "in_struct_arr", "io_struct_arr")}
    abi = []
    prev_index = None  # for monotone slice indices (lo <= mid <= hi)
    for p in params:
        n = p["name"]
        if n in struct_items:
            it = struct_items[n]
            base = {"name": n, "struct_name": it["struct"]["name"],
                    "fields": _struct_fields_to_schema(it["struct"])}
            if it["role"] in ("in_struct_arr", "io_struct_arr"):
                base.update(role="input_struct_array" if it["role"] == "in_struct_arr" else "inout_struct_array",
                            decode="struct_array_vector", length_param=it["len_name"])
            else:
                base.update(role="input_struct" if it["role"] == "in_struct" else "inout_struct",
                            decode="struct_value")
            abi.append(base)
            continue
        if n in buf_role:
            it = next(i for i in its if i["name"] == n)
            if buf_role[n] == "input_rectangular_pointer_table":
                abi.append({"name": n, "role": buf_role[n], "decode": "rectangular_pointer_table",
                            "elem": it["elem"], "elem_width": it["elem_w"],
                            "outer_length_param": it["outer_name"], "inner_length_param": it["inner_name"],
                            "outer_max": 16, "inner_max": 16})
                continue
            if buf_role[n] == "input_string_pointer_table":
                abi.append({"name": n, "role": buf_role[n], "decode": "string_pointer_table",
                            "elem": it["elem"], "elem_width": it["elem_w"],
                            "length_param": it["len_name"], "count_max": 16,
                            "mutation": "backing_observable"})
                continue
            spec = {"name": n, "role": buf_role[n], "decode": "vector",
                    "elem": it["elem"], "elem_width": it["elem_w"], "length_param": it["len_name"]}
            if buf_role[n] == "input_fixed_array_buffer":
                spec["decode"] = "fixed_array_vector"
                spec["inner_extent"] = it["inner_extent"]
            abi.append(spec)
        elif n in len_owner:
            buf, kind = len_owner[n]
            abi.append({"name": n, "role": kind, "decode": "derived_from_buffer",
                        "of_buffer": buf, "rust": p["rust"], "width": p["w"]})
        elif n in out_scalars:
            it = next(i for i in its if i["name"] == n)
            if by[n].get("const"):
                abi.append({"name": n, "role": "input_string", "decode": "nul_string",
                            "elem": it["elem"], "elem_width": it["elem_w"]})
            elif n in out_arrays:
                abi.append({"name": n, "role": "output_array", "decode": "output_array_vector",
                            "elem": it["elem"], "elem_width": it["elem_w"], "cap": OUT_ARR_CAP})
            else:
                abi.append({"name": n, "role": "out_scalar", "decode": "out_scalar_zero",
                            "elem": it["elem"], "elem_width": it["elem_w"]})
        elif has_io_arr and p.get("rust") == "usize":
            # bare usize alongside an output array -> a slice index; bound it to the array cap AND
            # chain it to the previous index so lo <= mid <= hi (merge_runs assumes monotone bounds;
            # independent bounds let lo>mid drive a[lo+t] out of range).
            spec = {"name": n, "role": "scalar", "decode": "bounded_scalar",
                    "rust": "usize", "width": 8, "max_value": OUT_ARR_CAP}
            if prev_index is None:
                spec["min_value"] = 0
            else:
                spec["min_var"] = prev_index
            prev_index = n
            abi.append(spec)
        else:
            abi.append({"name": n, "role": "scalar", "decode": "scalar",
                        "rust": p["rust"], "width": p["w"]})
    return abi


def _infer_items_abi(params: list[dict]) -> tuple[list[dict], list[dict]]:
    """Inference path: derive the ABI (schema params) then build items the SAME way the schema path
    does (items_from_schema). This guarantees the decode (items) and the call/signature (abi) agree —
    e.g. a `const char*` with no length is input_string in BOTH, not out_scalar in one and
    input_string in the other (which produced `cannot find value _buf`)."""
    abi = _infer_abi(params)
    items = items_from_schema({"params": abi})
    return items, abi


def resolve(name: str, cc_dir: Path, entry: str, infer: bool = False, ignore_schema: bool = False):
    """(params, ret, all_fns, items, abi). Require + strongly validate the schema; --infer to fall back.

    items drive byte-decode (role-based, order preserved per buffer); abi is the schema params in
    ABI order and drives the call arguments + extern signature. ignore_schema forces inference even
    when a schemas/<name>.json exists (used by the boundary harvester, where the on-disk schema is
    keyed to the program's canonical entry, not the boundary being harvested).
    """
    params, ret, all_fns = parse_entry_signature(cc_dir, entry)
    if ignore_schema:
        items, abi = _infer_items_abi(params)
        return params, ret, all_fns, items, abi
    schema_path = ROOT / "schemas" / f"{name}.json"
    if schema_path.exists():
        import harness_schema as hs
        schema = json.loads(schema_path.read_text(encoding="utf-8"))
        errs = hs.validate(schema)
        if schema.get("entry") != entry:
            errs.append(f"schema entry {schema.get('entry')!r} != requested {entry!r}")
        if schema.get("program") != name:
            errs.append(f"schema program {schema.get('program')!r} != pair {name!r}")
        errs += hs.validate_against_signature(schema, params, ret)
        if errs:
            raise SystemExit(f"schema {schema_path} invalid: {errs}")
        return params, ret, all_fns, items_from_schema(schema), schema["params"]
    if infer:
        items, abi = _infer_items_abi(params)
        return params, ret, all_fns, items, abi
    raise SystemExit(f"no schema for '{name}' at {schema_path}; pass --infer-schema to fall back")


def classify(params: list[dict]) -> list[dict]:
    """Pair pointers with a following integer length param; tag in/out/out-scalar."""
    out = []
    i = 0
    while i < len(params):
        p = params[i]
        if p["kind"] == "ptr_ptr":
            n1 = params[i + 1] if i + 1 < len(params) else None
            n2 = params[i + 2] if i + 2 < len(params) else None
            if n1 and n1["kind"] == "scalar" and n2 and n2["kind"] == "scalar":
                # T** + (rows, cols) -> rectangular pointer table (e.g. int** matrix)
                out.append({**p, "role": "in_table", "outer_name": n1["name"], "inner_name": n2["name"]})
                i += 3
                continue
            if n1 and n1["kind"] == "scalar":
                # T** + ONE scalar count -> string-pointer table: a pointer table over
                # independent NUL-terminated backings (e.g. char** words + size_t count).
                # Distinct from the rectangular table: each backing has its own length.
                out.append({**p, "role": "in_str_table", "len_name": n1["name"], "count_max": 16})
                i += 2
                continue
            raise SystemExit(f"unsupported: T** param {p['name']} without dimensions")
        if p["kind"] == "ptr_struct":
            nxt = params[i + 1] if i + 1 < len(params) else None
            # PODStruct* followed by a size_t length -> array of structs (e.g. Slot* slots, size_t
            # cap). The size_t requirement disambiguates from a single struct + unrelated scalar
            # (e.g. op_add(VM* vm, uint8_t operand) stays a single struct).
            if nxt and nxt["kind"] == "scalar" and nxt["rust"] == "usize":
                role = "in_struct_arr" if p["const"] else "io_struct_arr"
                out.append({**p, "role": role, "len_name": nxt["name"]})
                i += 2
                continue
            # Otherwise a struct pointer is a standalone single in/out parameter.
            role = "in_struct" if p["const"] else "io_struct"
            out.append({**p, "role": role})
            i += 1
            continue
        if p["kind"] == "ptr_array":
            nxt = params[i + 1] if i + 1 < len(params) else None
            if nxt and nxt["kind"] == "scalar":
                if not p["const"]:
                    raise SystemExit(f"unsupported: mutable pointer-to-array param {p['name']}")
                out.append({**p, "role": "in_arr", "len_name": nxt["name"]})
                i += 2
                continue
            raise SystemExit(f"unsupported: pointer-to-array param {p['name']} without a length")
        if p["kind"] == "ptr":
            nxt = params[i + 1] if i + 1 < len(params) else None
            n2 = params[i + 2] if i + 2 < len(params) else None
            # Don't pair a buffer with a following scalar that is a SLICE index — but only in the
            # real sliced pattern (the index is followed by ANOTHER usize, e.g. merge_runs lo,mid,hi).
            # A lone `(T* a, size_t count)` still pairs even when count is used as `a[count-1]`
            # (count is the LENGTH; it bounds the loop) — not a regression for normal buffers.
            sliced_idx = (nxt and nxt.get("used_as_index") and n2
                          and n2["kind"] == "scalar" and n2.get("rust") == "usize")
            if nxt and nxt["kind"] == "scalar" and not sliced_idx:
                role = "in_buf" if p["const"] else "io_buf"
                out.append({**p, "role": role, "len_name": nxt["name"]})
                i += 2
                continue
            out.append({**p, "role": "out_scalar"})
        else:
            out.append({**p, "role": "scalar"})
        i += 1
    return out


def _struct_fields_to_schema(sd: dict) -> list[dict]:
    """Serialize a POD struct descriptor's fields into compact, JSON-friendly schema specs."""
    specs = []
    for f in sd["fields"]:
        fd = f["desc"]
        if fd["kind"] == "scalar":
            specs.append({"name": f["name"], "kind": "scalar",
                          "rust": fd["rust"], "width": fd["width"]})
        elif fd["kind"] == "array" and fd["elem"]["kind"] == "scalar":
            specs.append({"name": f["name"], "kind": "array", "elem": fd["elem"]["rust"],
                          "elem_width": fd["elem"]["width"], "extent": fd["extent"]})
        else:
            raise SystemExit(f"non-POD field {f['name']} cannot be serialized")
    return specs


def _struct_from_schema(struct_name: str, field_specs: list[dict]) -> dict:
    """Rebuild a struct descriptor (for codegen) from compact schema field specs."""
    fields = []
    for s in field_specs:
        if s["kind"] == "scalar":
            desc = {"kind": "scalar", "rust": s["rust"], "width": s["width"]}
        elif s["kind"] == "array":
            desc = {"kind": "array", "extent": s["extent"],
                    "elem": {"kind": "scalar", "rust": s["elem"], "width": s["elem_width"]}}
        else:
            raise SystemExit(f"bad struct field spec {s}")
        fields.append({"name": s["name"], "desc": desc})
    return {"kind": "struct", "name": struct_name, "fields": fields, "pod": True, "reason": None}


def _rust_field_decode(fd: dict) -> str:
    """A Rust expression that consumes bytes and yields one struct-field value."""
    if fd["kind"] == "scalar":
        return f"cur.take_{fd['rust']}()"
    if fd["kind"] == "array" and fd["elem"]["kind"] == "scalar":
        ext, el = fd["extent"], fd["elem"]["rust"]
        return (f"{{ let mut a = [0 as {el}; {ext}]; "
                f"for j in 0..{ext} {{ a[j] = cur.take_{el}(); }} a }}")
    raise SystemExit(f"unsupported struct field kind {fd.get('kind')}")


def _rust_struct_literal(sd: dict, prefix: str = "translated::") -> str:
    """Build a struct literal; fields emitted in DECLARATION order so byte-consume order is fixed
    (Rust evaluates struct-literal fields in source order)."""
    parts = [f"{f['name']}: {_rust_field_decode(f['desc'])}" for f in sd["fields"]]
    return f"{prefix}{sd['name']} {{ {', '.join(parts)} }}"


def _struct_field_cmp(name: str, sd: dict) -> str:
    """`a.f != b.f || ...` over all fields (scalars and [T;N] both impl PartialEq)."""
    return " || ".join(f"{name}_c.{f['name']} != {name}_r.{f['name']}" for f in sd["fields"])


def _struct_arr_cmp(name: str, sd: dict) -> str:
    """Element-wise, field-wise comparison of two struct arrays (c2rust structs don't derive
    PartialEq, so Vec<T> != Vec<T> is unavailable — compare lengths then each field per element)."""
    fields = " || ".join(f"a.{f['name']} != b.{f['name']}" for f in sd["fields"])
    return (f"{name}_c.len() != {name}_r.len() || "
            f"{name}_c.iter().zip({name}_r.iter()).any(|(a, b)| {fields})")


def _len_cast(it: dict) -> str:
    """Cast a buffer's `.len()` (usize) to the C length param's Rust type. Empty for size_t/usize
    lengths so existing entries stay byte-identical; ` as u32` etc. for real libs using `unsigned int`."""
    lr = it.get("len_rust") or "usize"
    return "" if lr == "usize" else f" as {lr}"


def _decode_and_post(items: list[dict]) -> tuple[list[str], list[str]]:
    """Storage + byte-decode + comparison, driven by item ROLE (not order).

    Decode text is identical to the pre-ABI-refactor generator (so the 12 migration entries stay
    byte-identical): each buffer emits its take + its derived-length binding right after it.
    """
    decode, post = [], []
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            if it.get("decode") == "bounded_scalar":
                hi, ty = it["max_value"], it["rust"]
                mn = it["min_var"] if it.get("min_var") else it["min_value"]
                decode.append(f"    let {n} = ({mn} as {ty}) + (cur.take_{ty}() "
                              f"% (({hi} - {mn} + 1) as {ty}));")
            else:
                decode.append(f"    let {n} = cur.take_{it['rust']}();")
        elif it["role"] == "in_buf":
            decode.append(f"    let {n}_buf: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_buf.len(){_len_cast(it)};")
        elif it["role"] == "io_buf":
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    let {it['len_name']} = {n}_c.len(){_len_cast(it)};")
            decode.append(f"    let mut {n}_r = {n}_c.clone();")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: buffer {n}"); }}')
        elif it["role"] == "out_scalar":
            decode.append(f"    let mut {n}_c: {it['elem']} = 0 as {it['elem']};")
            decode.append(f"    let mut {n}_r: {it['elem']} = 0 as {it['elem']};")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: out param {n}"); }}')
        elif it["role"] == "out_arr":
            # output / inout array sized to a fixed cap (>= any bounded index) so dst[i] / a[lo..hi]
            # stay in bounds; both sides start zeroed, compare the whole buffer after the call.
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = vec![0 as {it['elem']}; {it['cap']}];")
            decode.append(f"    let mut {n}_r: Vec<{it['elem']}> = vec![0 as {it['elem']}; {it['cap']}];")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: out array {n}"); }}')
        elif it["role"] == "in_str":
            decode.append(f"    let mut {n}_buf: Vec<{it['elem']}> = cur.take_vec_{it['elem']}();")
            decode.append(f"    {n}_buf.push(0 as {it['elem']});")
        elif it["role"] == "in_arr":
            ext = it["inner_extent"]
            decode.append(f"    let {n}_cnt = (cur.byte() as usize) % 64;")
            decode.append(f"    let mut {n}_buf: Vec<[{it['elem']}; {ext}]> = Vec::with_capacity({n}_cnt);")
            decode.append(f"    for _ in 0..{n}_cnt {{ let mut a = [0 as {it['elem']}; {ext}];"
                          f" for j in 0..{ext} {{ a[j] = cur.take_{it['elem']}(); }} {n}_buf.push(a); }}")
            decode.append(f"    let {it['len_name']} = {n}_buf.len(){_len_cast(it)};")
        elif it["role"] == "in_table":
            el, om, im = it["elem"], it["outer_max"], it["inner_max"]
            o, ix = it["outer_name"], it["inner_name"]
            decode.append(f"    let {o} = (cur.byte() as usize) % ({om} + 1);")
            decode.append(f"    let {ix} = (cur.byte() as usize) % ({im} + 1);")
            decode.append(f"    let {n}_data: Vec<Vec<{el}>> = (0..{o}).map(|_| "
                          f"(0..{ix}).map(|_| cur.take_{el}()).collect()).collect();")
            for side in ("c", "r"):
                decode.append(f"    let mut {n}_back_{side} = {n}_data.clone();")
                decode.append(f"    let mut {n}_tab_{side}: Vec<*mut {el}> = "
                              f"{n}_back_{side}.iter_mut().map(|row| row.as_mut_ptr()).collect();")
            post.append(f'    if {n}_back_c != {n}_back_r {{ panic!("divergence: table {n}"); }}')
        elif it["role"] == "in_str_table":
            el, cm, ln = it["elem"], it["count_max"], it["len_name"]
            decode.append(f"    let {ln} = (cur.byte() as usize) % ({cm} + 1);")
            decode.append(f"    let {n}_data: Vec<Vec<{el}>> = (0..{ln}).map(|_| "
                          f"{{ let mut s = cur.take_vec_{el}(); s.push(0 as {el}); s }}).collect();")
            for side in ("c", "r"):
                decode.append(f"    let mut {n}_back_{side} = {n}_data.clone();")
                decode.append(f"    let mut {n}_tab_{side}: Vec<*mut {el}> = "
                              f"{n}_back_{side}.iter_mut().map(|s| s.as_mut_ptr()).collect();")
            post.append(f'    if {n}_back_c != {n}_back_r {{ panic!("divergence: string table {n}"); }}')
        elif it["role"] == "in_struct":
            # const struct pointer: one decoded value shared by both sides (callee must not mutate).
            decode.append(f"    let {n}_val = {_rust_struct_literal(it['struct'])};")
        elif it["role"] == "io_struct":
            # mutable struct pointer: decode once, give each side its own copy (struct is Copy),
            # compare field-wise afterwards.
            decode.append(f"    let {n}_val = {_rust_struct_literal(it['struct'])};")
            decode.append(f"    let mut {n}_c = {n}_val;")
            decode.append(f"    let mut {n}_r = {n}_val;")
            post.append(f'    if {_struct_field_cmp(n, it["struct"])} '
                        f'{{ panic!("divergence: struct {n}"); }}')
        elif it["role"] in ("in_struct_arr", "io_struct_arr"):
            sd, ln = it["struct"], it["len_name"]
            el = f"translated::{sd['name']}"
            decode.append(f"    let {ln} = (cur.byte() as usize) % 64;")
            decode.append(f"    let {n}_data: Vec<{el}> = (0..{ln}).map(|_| "
                          f"{_rust_struct_literal(sd)}).collect();")
            if it["role"] == "io_struct_arr":
                decode.append(f"    let mut {n}_c = {n}_data.clone();")
                decode.append(f"    let mut {n}_r = {n}_data.clone();")
                post.append(f'    if {_struct_arr_cmp(n, sd)} '
                            f'{{ panic!("divergence: struct array {n}"); }}')
    return decode, post


def _call_and_decl(abi: list[dict]) -> tuple[list[str], list[str], list[str]]:
    """Call arguments and extern signature in STRICT schema (ABI) order.

    Iterating the schema params in declaration order is what makes a length param that PRECEDES
    its buffer (e.g. f(size_t n, T* buf, ...)) come out in the right ABI position.
    """
    def _is_slice(rty: str) -> bool:
        rty = (rty or "").replace(" ", "")
        return (rty.startswith("&[") or rty.startswith("&mut[") or "Box<[" in rty
                or rty.startswith("Vec<") or rty.startswith("&Vec<")
                or rty.startswith("Option<&mut[") or rty.startswith("Option<&["))

    # pass 1: a buffer rendered as a Rust slice FOLDS its length/capacity param (the slice
    # carries its own len), so that scalar must be DROPPED from the Rust call (idiomatic
    # translations like `f(&[u8], &mut [u8])` from C `f(const u8*, size_t, u8*, size_t)`).
    folded: set[str] = set()
    for p in abi:
        if p["role"] in ("input_buffer", "inout_buffer", "output_buffer") and _is_slice(p.get("rust_pty")):
            ln = p.get("length_param") or p.get("capacity_param")
            if ln:
                folded.add(ln)

    c_args, decl = [], []
    r_pairs: list[tuple[str, str]] = []  # (param_name, rust_call_expr); filtered for folding below
    for p in abi:
        role, n = p["role"], p["name"]
        rty = (p.get("rust_pty") or "").replace(" ", "")
        if role in ("scalar", "length", "capacity"):
            c_args.append(n); r_pairs.append((n, n)); decl.append(f"{n}: {p['rust']}")
        elif role == "input_buffer":
            c_args.append(f"{n}_buf.as_ptr()")
            decl.append(f"{n}: *const {p['elem']}")
            if "Box<[" in rty:
                r_pairs.append((n, f"&{n}_buf.clone().into_boxed_slice()"))
            elif rty.startswith("Option<&["):           # Option<&[T]>
                r_pairs.append((n, f"Some(&{n}_buf[..])"))
            elif rty.startswith("&[") or rty.startswith("&mut["):
                r_pairs.append((n, f"&{n}_buf[..]"))
            elif rty.startswith("Vec<"):
                r_pairs.append((n, f"{n}_buf.clone()"))
            elif rty.startswith("&Vec<"):
                r_pairs.append((n, f"&{n}_buf.clone()"))
            else:  # raw pointer / C-ABI (c2rust, gpt4o raw-ptr style) or unknown -> default
                r_pairs.append((n, f"{n}_buf.as_ptr()"))
        elif role in ("inout_buffer", "output_buffer"):
            c_args.append(f"{n}_c.as_mut_ptr()")
            decl.append(f"{n}: *mut {p['elem']}")
            if rty.startswith("Option<&mut["):
                r_pairs.append((n, f"Some(&mut {n}_r[..])"))
            elif rty.startswith("&mut[") or rty.startswith("&["):
                r_pairs.append((n, f"&mut {n}_r[..]"))
            else:
                r_pairs.append((n, f"{n}_r.as_mut_ptr()"))
        elif role == "out_scalar":
            c_args.append(f"&mut {n}_c"); r_pairs.append((n, f"&mut {n}_r"))
            decl.append(f"{n}: *mut {p['elem']}")
        elif role == "output_array":
            c_args.append(f"{n}_c.as_mut_ptr()")
            decl.append(f"{n}: *mut {p['elem']}")
            if rty.startswith("Option<&mut["):     # Option<&mut [T]>
                r_pairs.append((n, f"Some(&mut {n}_r[..])"))
            elif rty.startswith("&mut["):           # &mut [T]
                r_pairs.append((n, f"&mut {n}_r[..]"))
            else:                                    # raw pointer / default
                r_pairs.append((n, f"{n}_r.as_mut_ptr()"))
        elif role == "input_string":
            c_args.append(f"{n}_buf.as_ptr()"); r_pairs.append((n, f"{n}_buf.as_ptr()"))
            decl.append(f"{n}: *const {p['elem']}")
        elif role == "input_fixed_array_buffer":
            c_args.append(f"{n}_buf.as_ptr()"); r_pairs.append((n, f"{n}_buf.as_ptr()"))
            decl.append(f"{n}: *const [{p['elem']}; {p['inner_extent']}]")
        elif role in ("input_rectangular_pointer_table", "input_string_pointer_table"):
            c_args.append(f"{n}_tab_c.as_mut_ptr()"); r_pairs.append((n, f"{n}_tab_r.as_mut_ptr()"))
            decl.append(f"{n}: *mut *mut {p['elem']}")
        elif role == "input_struct":
            c_args.append(f"&{n}_val"); r_pairs.append((n, f"&{n}_val"))
            decl.append(f"{n}: *const translated::{p['struct_name']}")
        elif role == "inout_struct":
            c_args.append(f"&mut {n}_c"); r_pairs.append((n, f"&mut {n}_r"))
            decl.append(f"{n}: *mut translated::{p['struct_name']}")
        elif role == "input_struct_array":
            c_args.append(f"{n}_data.as_ptr()"); r_pairs.append((n, f"{n}_data.as_ptr()"))
            decl.append(f"{n}: *const translated::{p['struct_name']}")
        elif role == "inout_struct_array":
            c_args.append(f"{n}_c.as_mut_ptr()"); r_pairs.append((n, f"{n}_r.as_mut_ptr()"))
            decl.append(f"{n}: *mut translated::{p['struct_name']}")
    # drop folded length/capacity params from the Rust call (they live inside the slice now)
    r_args = [expr for (nm, expr) in r_pairs if nm not in folded]
    return c_args, r_args, decl


# In-loop UB-free fuzzing (--ub-free): UBSan flags for the C oracle (recover + minimal
# runtime so each check calls a no-arg `__ubsan_handle_*_minimal` that we override below to
# just set a flag and continue -- no print, no abort). Checks restricted to the UB classes
# the risk model cares about; the handler set in UBSHIM_C is a superset so linking is robust.
UB_SANITIZE_FLAGS = [
    "-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable",
    "-fsanitize-recover=all",
    "-fsanitize-minimal-runtime",
]

UBSHIM_C = '''/* in-loop UB-free gate: record (don't print/abort) UBSan minimal-runtime reports. */
volatile int c2r_ub_flag = 0;
void c2r_ub_reset(void) { c2r_ub_flag = 0; }
int  c2r_ub_get(void)   { return c2r_ub_flag; }
#define H(name) void __ubsan_handle_##name##_minimal(void) { c2r_ub_flag = 1; }
H(add_overflow) H(sub_overflow) H(mul_overflow) H(negate_overflow)
H(divrem_overflow) H(shift_out_of_bounds) H(out_of_bounds)
H(type_mismatch) H(builtin_unreachable) H(pointer_overflow)
H(load_invalid_value)
#undef H
'''


def gen_target(entry: str, items: list[dict], abi: list[dict], ret: str, crate: str,
               ub_free: bool = False, rust_entry: str | None = None,
               rust_ret: str | None = None) -> str:
    """Generate the fuzz_target source: decode from items, call/signature in ABI order.

    When ub_free, the C oracle is UBSan-instrumented (recover + minimal runtime, flag-based
    -- see UBSHIM_C). The harness resets the flag, runs C, and REJECTS the input (returns
    without comparing) if C hit UB. So a divergence is reported only on UB-free input -- the
    fuzzer's gradient points at real translation bugs, not UB artifacts (in-loop, vs the
    post-hoc per-artifact exclusion in classify_artifact.py)."""
    decode, post = _decode_and_post(items)
    c_args, r_args, decl = _call_and_decl(abi)
    extern_args = ", ".join(decl)
    extern_ret = "" if ret == "void" else f"-> {ret}"

    # decode-shape bridge: C `(.., T* out) -> count` (0 = failure sentinel) is folded by idiomatic
    # Rust into `(..) -> Option<(value, count)>`. The single out-param moves INTO the return tuple,
    # so drop it from the Rust call + its standalone post-compare; the return comparison (below)
    # normalises BOTH sides to (ok, value, consumed). Assumption (documented): tuple = (out-value,
    # return-count), None <=> C return 0. A wrong assumption would surface as spurious DIVERGENCE.
    out_scalars = [p["name"] for p in abi if p["role"] == "out_scalar"]
    decode_shape = bool(rust_ret and rust_ret.replace(" ", "").startswith("Option<(")
                        and len(out_scalars) == 1 and ret in _INT_TYPES)
    if decode_shape:
        osc = out_scalars[0]
        r_args = [a for a in r_args if a != f"&mut {osc}_r"]
        post = [l for l in post if f"{osc}_c != {osc}_r" not in l]

    call_c = f"c_{entry}({', '.join(c_args)})"
    call_r = f"translated::{rust_entry or entry}({', '.join(r_args)})"
    # UB-free gate: reset before C, reject (return) if C tripped UB, then call Rust.
    pre = "        c2r_ub_reset();\n" if ub_free else ""
    gate = "        if c2r_ub_get() != 0 { return; }  // C hit UB -> reject input\n" if ub_free else ""
    if ret == "void":
        body_call = f"{pre}        {call_c};\n{gate}        {call_r};"
        ret_cmp = ""
    elif decode_shape:
        osc = out_scalars[0]
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}        let r_ret = {call_r};"
        ret_cmp = (
            f"        let (c_ok, c_val, c_cons) = (c_ret != 0, {osc}_c, c_ret);\n"
            f"        let (r_ok, r_val, r_cons) = match r_ret {{ Some((v, c)) => (true, v, c), None => (false, 0, 0) }};\n"
            f'        if c_ok != r_ok {{ panic!("divergence: success/None mismatch"); }}\n'
            f'        if c_ok && (c_val != r_val || (c_cons as i128) != (r_cons as i128)) {{ panic!("divergence: decoded value/consumed"); }}'
        )
    else:
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}        let r_ret = {call_r};"
        # idiomatic translations may return a different-but-compatible integer width/signedness
        # (e.g. C `int`/i32 vs Rust `isize`); compare via i128 so the widths line up.
        if rust_ret and rust_ret != ret and ret in _INT_TYPES and rust_ret in _INT_TYPES:
            cmp = "(c_ret as i128) != (r_ret as i128)"
        else:
            cmp = "c_ret != r_ret"
        ret_cmp = f'        if {cmp} {{ panic!("divergence: return value"); }}'
    ub_externs = ["    fn c2r_ub_reset();", "    fn c2r_ub_get() -> i32;"] if ub_free else []

    return "\n".join([
        "#![no_main]",
        'use libfuzzer_sys::fuzz_target;',
        "struct Cur<'a> { d: &'a [u8], p: usize }",
        "impl<'a> Cur<'a> {",
        "    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }",
        "    fn byte(&mut self) -> u8 { let b = if self.p < self.d.len() { self.d[self.p] } else { 0 }; self.p += 1; b }",
        *[f"    fn take_{t}(&mut self) -> {t} {{ let mut v = [0u8; {w}]; for i in 0..{w} {{ v[i] = self.byte(); }} {t}::from_le_bytes(v) }}"
          for t, w in [("u8", 1), ("i8", 1), ("u16", 2), ("i16", 2), ("u32", 4), ("i32", 4),
                       ("u64", 8), ("i64", 8), ("usize", 8)]],
        *[f"    fn take_vec_{t}(&mut self) -> Vec<{t}> {{ let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_{t}()).collect() }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64"]],
        "}",
        "",
        "fn cd() -> i8 { 0 }  // silence unused on some shapes",
        "",
        f"use {crate} as translated;",
        "extern \"C\" {",
        f"    fn c_{entry}({extern_args}) {extern_ret};",
        *ub_externs,
        "}",
        "",
        "fuzz_target!(|data: &[u8]| {",
        "    let _ = cd();",
        "    let mut cur = Cur::new(data);",
        *decode,
        "    unsafe {",
        body_call,
        ret_cmp,
        *post,
        "    }",
        "});",
        "",
    ])


def expose_entry(rs_text: str, entry: str) -> tuple[str, bool]:
    """Make `entry` callable from the harness: if c2rust emitted it as a private (`static` C)
    `extern "C" fn`, prepend `#[no_mangle] pub`. No-op if it is already `pub`. Returns
    (text, changed). This is what lets the harvester test an internal boundary."""
    import re
    # already pub (c2rust extern "C" or plain idiomatic fn) -> no-op
    if re.search(rf'(?m)^\s*pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(entry)}\b', rs_text):
        return rs_text, False
    # c2rust `extern "C" fn` -> prepend `#[no_mangle] pub`
    pat = rf'(?m)^(\s*)((?:unsafe\s+)?extern\s+"C"\s+fn\s+{re.escape(entry)}\b)'
    new = re.sub(pat, r'\1#[no_mangle]\n\1pub \2', rs_text, count=1)
    if new != rs_text:
        return new, True
    # plain idiomatic `fn` (LLM-translated, not C-ABI) -> prepend `pub`
    pat2 = rf'(?m)^(\s*)((?:unsafe\s+)?fn\s+{re.escape(entry)}\b)'
    new = re.sub(pat2, r'\1pub \2', rs_text, count=1)
    return new, (new != rs_text)


def strip_static_c(c_text: str, entry: str) -> tuple[str, bool]:
    """Give the renamed C oracle symbol `c_<entry>` external linkage by dropping `static` from the
    entry's definition (a `static` C function isn't linkable even after the f->c_f #define rename)."""
    import re
    pat = rf'(?m)^(\s*)static(\s+[^\n;{{]*\b{re.escape(entry)}\s*\()'
    new = re.sub(pat, r'\1\2', c_text, count=1)
    return new, (new != c_text)


_INT_TYPES = {"i8", "i16", "i32", "i64", "i128", "isize",
              "u8", "u16", "u32", "u64", "u128", "usize"}


def parse_rust_ret_type(rs_text: str, entry: str) -> str | None:
    """Return type spelling of `entry` in the translation (None if no `-> T`, i.e. unit)."""
    import re
    m = re.search(rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(entry)}\s*\([^;{{]*?\)\s*->\s*([^{{]+?)\s*\{{', rs_text, re.S)
    return m.group(1).strip() if m else None


def parse_rust_param_types(rs_text: str, entry: str) -> list[str]:
    """Extract the Rust translation's per-parameter TYPE strings for `entry`, in declaration order.

    For name-preserving idiomatic translations the params line up 1:1 with the C signature, so this
    lets the harness marshal each C-ABI value into the idiomatic Rust type the translation expects
    (e.g. C `const i32*` + len  ->  Rust `&Box<[i32]>` / `&[i32]` / `Vec<i32>`). Returns [] if the
    signature can't be found (caller falls back to the C-ABI form)."""
    import re
    m = re.search(rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(entry)}\s*\(([^;{{]*?)\)\s*(?:->|\{{)', rs_text, re.S)
    if not m:
        return []
    inner = m.group(1).strip()
    if not inner:
        return []
    # split on top-level commas (respect <> and [] nesting)
    parts, depth, cur = [], 0, ""
    for ch in inner:
        if ch in "<[(": depth += 1
        elif ch in ">])": depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur); cur = ""
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    types = []
    for part in parts:
        # `name: type`  (ignore `mut`, `self`)
        if ":" in part:
            types.append(part.split(":", 1)[1].strip())
        else:
            types.append(part.strip())
    return types


def main() -> int:
    ap = argparse.ArgumentParser(description="Generate a differential fuzz harness for an STU")
    ap.add_argument("--pair", required=True, help="benchmark/pairs/<name>")
    ap.add_argument("--entry", required=True)
    ap.add_argument("--out", default=None, help="output project dir (default fuzz_gen/<name>)")
    ap.add_argument("--infer-schema", action="store_true",
                    help="fall back to signature inference when no schema exists (default: require schema)")
    ap.add_argument("--ignore-schema", action="store_true",
                    help="force inference even if schemas/<name>.json exists (harvesting non-entry boundaries)")
    ap.add_argument("--expose-entry", action="store_true",
                    help="make a private (static) entry callable by prepending #[no_mangle] pub")
    ap.add_argument("--rust-entry", default=None,
                    help="name of the matched function in the Rust translation when it differs from "
                    "the C entry (renamed translations); the harness calls translated::<rust-entry> "
                    "while the C oracle keeps c_<entry>. Defaults to <entry>.")
    ap.add_argument("--ub-free", action="store_true",
                    help="in-loop UB-free gate: UBSan-instrument the C oracle and reject "
                    "(skip comparison on) inputs where C hits UB, so divergences are reported "
                    "only on UB-free input (vs post-hoc per-artifact exclusion)")
    args = ap.parse_args()

    pair = Path(args.pair)
    name = pair.name
    crate = name.replace("-", "_")
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"))
    c_src = next((pair / "source").glob("*.c"))

    params, ret, all_fns, items, abi = resolve(
        name, cc, args.entry, infer=args.infer_schema, ignore_schema=args.ignore_schema)

    out = Path(args.out) if args.out else (ROOT / "fuzz_gen" / name)
    out.mkdir(parents=True, exist_ok=True)
    (out / "src").mkdir(exist_ok=True)
    (out / "c").mkdir(exist_ok=True)
    (out / "fuzz" / "fuzz_targets").mkdir(parents=True, exist_ok=True)

    rust_entry = args.rust_entry or args.entry
    c_text = c_src.read_text()
    rs_text = rs.read_text()
    # idiomatic bridge: record each Rust param type so _call_and_decl can marshal C-ABI data into
    # the idiomatic shape. Two alignments: (a) 1:1 with the C ABI (no folding); (b) Rust has FEWER
    # params because length/capacity scalars were FOLDED into slices -> align to the non-len/cap
    # "core" params (e.g. C `(const u8*, size_t, u8*, size_t)` -> Rust `(&[u8], &mut [u8])`).
    rust_ptys = parse_rust_param_types(rs_text, rust_entry)
    rust_ret = parse_rust_ret_type(rs_text, rust_entry)
    # Align the Rust param types to the C ABI params. Idiomatic translations fold scalars away:
    # length/capacity fold into slices, and (decode shape) an out-param folds into the return tuple.
    # Try progressively-reduced ABI subsets and use the first whose arity matches the Rust signature.
    if rust_ptys:
        for cand in (abi,
                     [p for p in abi if p["role"] not in ("length", "capacity")],
                     [p for p in abi if p["role"] not in ("length", "capacity", "out_scalar")]):
            if len(rust_ptys) == len(cand):
                for p, t in zip(cand, rust_ptys):
                    p["rust_pty"] = t
                break
    if args.expose_entry:
        rs_text, _ = expose_entry(rs_text, rust_entry)
        c_text, _ = strip_static_c(c_text, args.entry)
    (out / "c" / c_src.name).write_text(c_text, encoding="utf-8")
    # Real libs split into .c + sibling .h (authored corpus is self-contained single-TU).
    # Copy the headers next to the .c so `#include "foo.h"` resolves from the harness c/ dir.
    for h in (pair / "source").glob("*.h"):
        (out / "c" / h.name).write_text(h.read_text(), encoding="utf-8")
    (out / "src" / "lib.rs").write_text(rs_text, encoding="utf-8")

    (out / "Cargo.toml").write_text(
        f'[package]\nname = "{crate}"\nversion = "0.1.0"\nedition = "2021"\n\n'
        f'[build-dependencies]\ncc = "1"\n\n[dependencies]\n', encoding="utf-8")

    defines = "\n".join(f'        .define("{fn}", "c_{fn}")' for fn in all_fns)
    # --ub-free: instrument the oracle with UBSan (recover) and compile the flag shim in.
    if args.ub_free:
        (out / "c" / "ubshim.c").write_text(UBSHIM_C, encoding="utf-8")
    ub_flags = "".join(f'\n        .flag("{f}")' for f in UB_SANITIZE_FLAGS) if args.ub_free else ""
    ub_file = f'\n    build.file("c/ubshim.c");' if args.ub_free else ""
    (out / "build.rs").write_text(f'''fn main() {{
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp"){ub_flags}.warnings(false);
    build
{defines};
    build.file("c/{c_src.name}");{ub_file}
    build.compile("c_oracle");
    let rd = std::process::Command::new("clang").arg("--print-resource-dir").output().unwrap();
    let rd = String::from_utf8(rd.stdout).unwrap().trim().to_string();
    let lib_dir = std::path::Path::new(&rd).join("lib").join("linux");
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| "x86_64".into());
    println!("cargo:rustc-link-search=native={{}}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=clang_rt.profile-{{}}", arch);
}}
''', encoding="utf-8")

    (out / "fuzz" / "Cargo.toml").write_text(f'''[package]
name = "{crate}-fuzz"
version = "0.0.0"
publish = false
edition = "2021"

[package.metadata]
cargo-fuzz = true

[dependencies]
libfuzzer-sys = "0.4"

[dependencies.{crate}]
path = ".."

[workspace]
members = ["."]

[profile.release]
debug = 1

[[bin]]
name = "{crate}_ft"
path = "fuzz_targets/{crate}_ft.rs"
test = false
doc = false
''', encoding="utf-8")

    (out / "fuzz" / "fuzz_targets" / f"{crate}_ft.rs").write_text(
        gen_target(args.entry, items, abi, ret, crate, ub_free=args.ub_free, rust_entry=rust_entry,
                   rust_ret=rust_ret), encoding="utf-8")

    print(f"generated harness at {out}")
    print(f"  entry: {args.entry} -> {ret}")
    print(f"  abi roles: {[(p['name'], p['role']) for p in abi]}")
    if args.ub_free:
        print("  in-loop UB-free gate: ON (C UBSan-instrumented; UB inputs rejected)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
