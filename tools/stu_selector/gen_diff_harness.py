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
import re
import shutil
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod  # noqa: E402
import contract_templates as ctpl  # noqa: E402
import clang.cindex  # noqa: E402
from clang.cindex import CursorKind, Index, TypeKind  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]

# Generator capability stamp — recorded on every harvested dataset row so v1/v2 (built with
# different generator coverage) are never confused. Bump GEN_VERSION when adding a boundary shape.
GEN_VERSION = "0.7"   # 2026-09-04: HARNESS PLAN path (--plan). The InputPlan is derived from the
                      # C AST + body by tools/stu_selector/harness_plan.py and lowered here; no
                      # schema file is read or written. Adds the `plan_array` adapter (a
                      # harness-owned allocation sized by a plan expression) and short/unsigned
                      # short to the scalar map. There is NO ObservationPlan: what to compare is
                      # the fixed ladder of docs/harness_oracle_plan.md, so a pointer return no
                      # longer fails construction -- it degrades to `pointer_nullness`. The
                      # generator's own local is `_c2r_m`, not `mode`, which is a common C
                      # parameter name (BZ2_bzopen(path, mode)) and was being shadowed.
                      # 0.6 (2026-09-03): output_buffer+capacity_ptr, max_len, per-entry schema,
                      # runtime C2R_MODE, canonical return type, C-global renaming, rem_euclid
GEN_CAPABILITIES = (
    "scalar", "bounded_scalar", "input_buffer", "inout_buffer", "output_buffer", "out_scalar",
    "input_string", "input_fixed_array_buffer", "input_rectangular_pointer_table",
    "input_string_pointer_table", "input_struct", "inout_struct",
    "input_struct_array", "inout_struct_array", "output_array",
    "output_buffer_with_capacity_ptr",   # RQ4
    "plan_array",                        # HarnessPlan lowering
    "buffer_table",                      # T** of constant-indexed rows (tulip inputs/outputs)
)

# c2rust type spelling -> (rust_ffi_type, byte_width). size_t -> usize to match translation.
SCALAR_MAP = {
    "size_t": ("usize", 8),
    "uint64_t": ("u64", 8), "int64_t": ("i64", 8),
    "uint32_t": ("u32", 4), "int32_t": ("i32", 4),
    "uint16_t": ("u16", 2), "int16_t": ("i16", 2),
    "uint8_t": ("u8", 1), "int8_t": ("i8", 1),
    "int": ("i32", 4), "unsigned int": ("u32", 4), "unsigned": ("u32", 4),
    "short": ("i16", 2), "short int": ("i16", 2),
    "unsigned short": ("u16", 2), "unsigned short int": ("u16", 2),
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
    # An opaque / incomplete (forward-declared) struct has NO visible fields -> the POD check above
    # passes vacuously, which is wrong: it can't be constructed or field-compared (e.g. OpenSSL's
    # EVP_PKEY/BIGNUM, or any handle behind a `struct foo;`). Treat a zero-field record as non-POD.
    if pod and not fields:
        pod, reason = False, "opaque/incomplete (no visible fields)"
    # c_name = the spelling usable in a C declaration (keeps the `struct`/`union` keyword for
    # non-typedef'd tags, drops only cv-qualifiers). Used by the out-of-process C oracle to declare
    # the struct; the Rust `name` (keyword stripped) is what c2rust calls the type.
    c_name = t.spelling
    for q in ("const ", "volatile "):
        c_name = c_name.replace(q, "")
    return {"kind": "struct", "name": _rust_record_name(t), "c_name": c_name.strip(),
            "fields": fields, "pod": pod, "reason": reason}


def _param_from_descriptor(desc: dict, name: str, allow_nonpod: bool = False) -> dict:
    """Map a type descriptor to a harness param. T** is recognized but deferred.

    `allow_nonpod`: the HarnessPlan path asks for a struct-with-pointers parameter to come back as
    `ptr_struct_nonpod` so the planner can try the producer bridge
    (docs/producer_bridge_pilot.md); every other caller keeps the construction failure."""
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
                if allow_nonpod:
                    return {"kind": "ptr_struct_nonpod", "const": desc["const"], "struct": inner,
                            "name": name}
                raise SystemExit(f"struct-invariant param {name}: {inner['name']} "
                                 f"{inner['reason']} (needs invariant reconstruction)")
            return {"kind": "ptr_struct", "const": desc["const"], "struct": inner, "name": name}
        if inner["kind"] == "unsupported" and inner.get("spelling", "").replace("const ", "") == "void":
            # `void*` carries no shape, so there is nothing to decode -- but NULL is the one
            # pointer value that is universally valid to pass: malloc-style opaque slots ignore
            # it and `free(NULL)` is a no-op. Both sides get NULL, which is a real comparison of
            # how each handles it, and it cannot be a wild pointer.
            return {"kind": "void_ptr", "const": desc["const"], "name": name}
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


def parse_entry_signature(cc_dir: Path, entry: str, with_return_desc: bool = False,
                          allow_nonpod: bool = False):
    # with_return_desc=True adds a 4th element: the structural descriptor of the RETURN type,
    # which eligibility needs in order to decide whether the return value can be compared at
    # all. Default stays a 3-tuple so existing callers are unaffected.
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
    ret_desc: dict = {"kind": "void"}
    all_fns: list[str] = []
    # RQ4 FIX 2 (2026-09-03): file-scope VARIABLES with external linkage must be renamed in the
    # oracle too. c2rust emits translated globals as #[no_mangle] statics, so an un-renamed C
    # global and its translation collapse onto ONE storage location at link time -- the linker
    # either rejects it (duplicate symbol) or, worse, silently makes the differential compare a
    # value against itself. Real libraries have such tables (bzip2: BZ2_crc32Table, BZ2_rNums);
    # the micro-benchmark corpus this generator was built on has none.
    all_globals: list[str] = []
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
            if cur.kind == CursorKind.VAR_DECL and cur.is_definition():
                f = cur.location.file.name if cur.location and cur.location.file else None
                if (f and not f.startswith(("/usr/", "/lib/"))
                        and cur.semantic_parent is not None
                        and cur.semantic_parent.kind == CursorKind.TRANSLATION_UNIT
                        and cur.linkage == clang.cindex.LinkageKind.EXTERNAL):
                    all_globals.append(cur.spelling)
                continue
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
                pd = _param_from_descriptor(describe_type(a.type), pname, allow_nonpod)
                u = usage.get(a.spelling, {})
                pd["subscripted"] = u.get("subscripted", False)
                pd["used_as_index"] = u.get("used_as_index", False)
                params.append(pd)
            rt = cur.result_type
            if rt.kind == TypeKind.VOID:
                ret = "void"
                ret_desc = {"kind": "void"}
            else:
                # RQ4 FIX 1 (2026-09-03): resolve typedefs, exactly as describe_type already does
                # for PARAMETERS. Upstream mapped only rt.spelling and silently fell back to i32,
                # so a `UChar`-returning C function got an `-> i32` extern declaration: on x86-64
                # SysV the upper 24 bits of eax are unspecified for a byte return, so the oracle's
                # return value was read as garbage. Now bzip2's UChar/Int32/UInt32 returns map
                # through the same canonical path the parameters use.
                d = describe_type(rt)
                ret_desc = d
                if d["kind"] == "scalar":
                    ret = d["rust"]
                elif d["kind"] == "pointer":
                    # A pointer return is never a scalar. The sentinel keeps it out of the scalar
                    # comparison path; what it means is decided by the return contract template.
                    ret = "ptr"
                else:
                    sc = map_scalar(rt.spelling)
                    ret = sc[0] if sc else "i32"
    _fns = sorted(set(all_fns)) + [f"@var:{g}" for g in sorted(set(all_globals))]
    return (params, ret, _fns, ret_desc) if with_return_desc else (params, ret, _fns)



# ---------------------------------------------------------------------------
# Contract templates
#
# Eligibility asks whether a boundary's parameters AND its return value AND every mutable output
# match a SUPPORTED CONTRACT, not merely whether a pointer appears. A pointer with no declared
# contract is rejected HERE, with a reason, instead of being accepted and then failing to compile
# or - worse - being compared as a raw address. Two heap pointers from two allocators are never
# equal, so an undeclared pointer return is not a build accident: it is an unsupported contract.
#
# Parameter templates are the existing roles (scalar / bounded_scalar / input_buffer /
# inout_buffer / output_buffer+capacity_ptr / out_scalar / output_array / input_string /
# fixed-array / pointer tables / POD struct). Return templates are declared below.
# ---------------------------------------------------------------------------
def load_plugins(paths: list[str] | None) -> list[dict]:
    """Comparator plugins: user-supplied CODE behind a stable ABI, not a declaration.

    docs/harness_oracle_plan.md section 5. A plugin extends OUTPUT comparison only; it never
    touches the InputPlan. It is untrusted code linked into the harness.
    """
    import tomllib
    out = []
    for path in (paths or []):
        f = Path(path)
        d = tomllib.loads(f.read_text(encoding="utf-8"))["plugin"]
        for key in ("c_type", "c_source", "rust_source"):
            if key not in d:
                raise SystemExit(f"plugin {path}: missing required key {key!r}")
        d["_dir"] = f.parent
        for key in ("c_source", "rust_source"):
            if d.get(key):
                q = f.parent / d[key]
                if not q.exists():
                    raise SystemExit(f"plugin {path}: {key} {q} does not exist")
                d["_" + key] = q
        d.setdefault("max_bytes", 1 << 20)
        out.append(d)
    return out


def plugin_compat(pl: dict, rust_text: str) -> str | None:
    """Why this plugin cannot be linked against THIS translation, or None if it can.

    A comparator's Rust half reads the translated struct's fields by name and calls the
    translation's destructor; the manifest lists both ([plugin.requires]). A translation that
    renames a field (PtrTrans `type_` for c2rust's `type_0`) or ships no destructor cannot host
    it -- and by the comparison ladder that is a DEGRADATION to the next rung (pointer
    nullness), never a build failure. Without a `requires` table the plugin is assumed compatible
    (the pre-manifest behaviour).
    """
    req = pl.get("requires") or {}
    if not req:
        return None
    st = req.get("rust_struct") or pl["c_type"]
    m = re.search(rf'(?ms)^\s*pub\s+struct\s+{re.escape(st)}\b[^{{]*\{{(.*?)^\s*\}}', rust_text or "")
    if not m:
        return f"no `pub struct {st}` in the translation"
    fields = set(re.findall(r'(?m)^\s*pub\s+([A-Za-z_]\w*)\s*:', m.group(1)))
    miss = [f for f in req.get("rust_fields", []) if f not in fields]
    if miss:
        return f"struct {st} lacks field(s) {miss} the comparator reads"
    for fn in req.get("rust_fns", []):
        if not re.search(rf'(?m)^\s*(?:#\[no_mangle\]\s*)?pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(fn)}\b', rust_text or ""):
            return f"no public fn {fn} in the translation"
    return None


_PLUGINS_OK: list | None = None      # plugins compatible with the translation being generated for
_PLUGINS_DEGRADED: dict = {}         # library -> reason, for the verdict and the log


def _plugins(args) -> list[dict]:
    return _PLUGINS_OK if _PLUGINS_OK is not None else load_plugins(args.plugins)


def _match_plugin(ret_desc: dict, plugins: list[dict]) -> dict | None:
    inner = ret_desc.get("inner") or {}
    names = {inner.get("name"), inner.get("c_name"), inner.get("spelling")}
    names = {n.replace("struct ", "").strip() for n in names if n}
    for pl in plugins:
        if pl["c_type"].replace("*", "").replace("struct ", "").strip() in names:
            return pl
    return None


RETURN_TEMPLATES = {
    "void":              "nothing to compare",
    "scalar":            "compare the value",
    "interior_pointer":  "compare nullness and (returned_ptr - base_of_named_input)",
    "structured_object": "compare a canonical extraction of the pointed-to object",
    "opaque_handle":     "do not compare the handle; compare the observations of an operation sequence",
    "pointer_nullness":  "compare NULL vs non-NULL only (never the address)",
    "comparator_plugin": "compare the user comparator's canonical bytes for the pointed-to object",
}


def return_contract(ret_desc: dict, ret_rust: str, schema: dict | None,
                    plugins: list[dict] | None = None, rust_sig_ret: str | None = None) -> dict:
    """Classify the C return type into a comparison contract, or say why there is none.

    A schema may declare `return: {kind: ...}` to select a pointer template; without one, any
    pointer return is unsupported.
    """
    declared = ((schema or {}).get("return") or {}).get("kind")
    if ret_desc.get("kind") == "void" or ret_rust == "void":
        return {"template": "void", "compare": "none"}
    if ret_desc.get("kind") == "scalar":
        return {"template": "scalar", "compare": "value", "rust": ret_desc["rust"]}
    if ret_desc.get("kind") == "pointer":
        pl = _match_plugin(ret_desc, plugins or [])
        if pl is not None:
            # Rung 5: a registered comparator turns a partial oracle into a full one.
            return {"template": "comparator_plugin", "compare": RETURN_TEMPLATES["comparator_plugin"],
                    # NOT "full": what is compared is the object state the PLUGIN declares, not
                    # all program semantics.
                    "oracle_strength": "structured-state", "plugin": pl}
        if declared == "interior_pointer":
            base = ((schema or {}).get("return") or {}).get("base")
            byrole = {q["name"]: q.get("role") for q in (schema or {}).get("params", [])}
            if base not in byrole:
                return {"template": None, "reason":
                        f"interior_pointer declares base {base!r}, which is not a parameter"}
            if byrole[base] not in ("input_buffer", "inout_buffer", "output_buffer", "input_string"):
                return {"template": None, "reason":
                        f"interior_pointer base {base!r} has role {byrole[base]!r}; the base must be "
                        f"a buffer or string parameter so an offset is defined"}
            return {"template": "interior_pointer", "compare": RETURN_TEMPLATES["interior_pointer"],
                    "base": base, "elem": (ret_desc.get("inner") or {}).get("rust", "i8"),
                    "const": ret_desc.get("const", True), "declared": True}
        if declared == "structured_object":
            r = (schema or {}).get("return") or {}
            miss = [k for k in ("type", "fields", "child", "next", "header", "free") if k not in r]
            if miss:
                return {"template": None, "reason":
                        f"structured_object is missing {miss}; the object graph must be declared "
                        f"(type, fields, child, next, header, free) so BOTH extractors can be "
                        f"generated without calling the library's own printer"}
            for f in r["fields"]:
                if f.get("kind") not in ("int", "double", "cstring"):
                    return {"template": None, "reason":
                            f"structured_object field {f.get('name')!r} has unsupported kind "
                            f"{f.get('kind')!r} (int | double | cstring)"}
            out = dict(r); out.update({"template": "structured_object",
                                       "compare": RETURN_TEMPLATES["structured_object"],
                                       "declared": True})
            return out
        if declared == "opaque_handle":
            return {"template": declared, "compare": RETURN_TEMPLATES[declared], "declared": True}
        # No declared contract: the boundary is NOT rejected.  A raw pointer cannot be compared
        # across two allocators, so the ladder degrades to rung 3 -- nullness, never the address.
        # docs/harness_oracle_plan.md: inputs must be exact, outputs may be partial.
        inner = (ret_desc.get("inner") or {}).get("kind", "?")
        # The Rust side must be able to ANSWER the nullness question. A reshaped translation
        # (C2SaferRust `BZ2_bzlibVersion() -> &str` for `const char*`) returns a reference, which
        # is never null, or an Option; casting either to `*const c_void` does not compile, and
        # before this check the plan said `planned` and rustc said E0606.
        # `ret_rust` here is the C-side mapping, and for a pointer it is the sentinel "ptr"; the
        # decision needs the TRANSLATION's own return type, parsed from the .rs and passed in as
        # `rust_sig_ret` (None when no signature was found: keep the raw-pointer reading).
        rr = (rust_sig_ret or "").replace(" ", "")
        if not rr or rr.startswith("*"):
            rust_null = "(r_ret as *const core::ffi::c_void).is_null()"
        elif rr.startswith("Option<"):
            rust_null = "r_ret.is_none()"
        elif rr.startswith("&"):
            rust_null = "{ let _ = &r_ret; false }"   # a reference is never null
        else:
            return {"template": None, "reason":
                    f"C returns a pointer but the Rust return type is {ret_rust}: neither a raw "
                    f"pointer, a reference, nor an Option, so nullness has no Rust-side reading"}
        return {"template": "pointer_nullness", "compare": RETURN_TEMPLATES["pointer_nullness"],
                "oracle_strength": "partial(nullness)", "rust_null": rust_null,
                "note": f"pointer return (to {inner}) has no canonical comparator; register a "
                        f"comparator plugin to compare the pointed-to object"}
    return {"template": None, "reason":
            f"return type {ret_desc.get('spelling', ret_desc.get('kind'))} is neither void, a "
            f"scalar, nor a pointer with a declared contract"}


def eligibility(name: str, cc_dir: Path, entry: str, infer: bool = True,
                schema_path: Path | None = None) -> dict:
    """Single eligibility verdict for one boundary: parameters, return value and comparator.

    Returns {"supported": bool, "reason": str|None, "param_templates": [...],
             "return_contract": {...}} . A rejection is reported as unsupported:<reason>; nothing
    is deferred to the build.
    """
    import json as _json
    schema = None
    if schema_path and Path(schema_path).exists():
        schema = _json.loads(Path(schema_path).read_text(encoding="utf-8"))
    try:
        params, ret, _fns, ret_desc = parse_entry_signature(cc_dir, entry, with_return_desc=True)
    except SystemExit as e:
        return {"supported": False, "reason": f"signature: {e}"}
    if not params:
        return {"supported": False,
                "reason": "no parameters: the entry was not found in the C translation unit, or it "
                          "takes no arguments, so no logical input can be constructed"}
    try:
        items = items_from_schema(schema) if schema else classify(params)
        _abi = schema["params"] if schema else _infer_abi(params)
    except SystemExit as e:
        return {"supported": False, "reason": f"parameter: {e}"}
    # Every decoded item needs a cursor method; a type the byte cursor cannot produce is an
    # unsupported contract, not a build accident. (float/double were accepted by SCALAR_MAP long
    # before the cursor could emit them.)
    DECODABLE = {"u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "usize", "f32", "f64", "bool"}
    for it in items:
        for key in ("rust", "elem"):
            ty = it.get(key)
            if ty is not None and ty not in DECODABLE:
                return {"supported": False,
                        "reason": f"decode: parameter {it['name']!r} has type {ty!r}, which the "
                                  f"byte cursor cannot construct"}
    rc = return_contract(ret_desc, ret, schema)
    if rc.get("template") is None:
        return {"supported": False, "reason": f"return: {rc['reason']}",
                "param_templates": [(i["name"], i["role"]) for i in items]}
    return {"supported": True, "reason": None,
            "param_templates": [(i["name"], i["role"]) for i in items],
            "return_contract": rc}

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
    param_role = {p["name"]: p.get("role") for p in schema["params"]}
    param_elem = {p["name"]: p.get("elem") for p in schema["params"]}
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
            it = {"kind": "ptr", "role": "in_buf", "name": p["name"],
                  "elem": p["elem"], "elem_w": p["elem_width"], "len_name": p["length_param"],
                  "len_rust": param_rust.get(p["length_param"]) or "usize"}
            if "max_len" in p:
                it["max_len"] = p["max_len"]
            items.append(it)
        elif role == "output_buffer":
            # RQ4: a real output buffer -- allocated to `cap`, with its capacity handed to the
            # callee. When the capacity param is itself a pointer (`capacity_ptr`) the callee both
            # READS it as the capacity and WRITES the produced length back, so it is an in/out
            # scalar seeded with the allocation size, and the observable output is the prefix
            # [0, written_length).
            cp = p["capacity_param"]
            if param_role.get(cp) == "capacity_ptr":
                items.append({"kind": "ptr", "role": "out_buf_cap", "name": p["name"],
                              "elem": p["elem"], "elem_w": p["elem_width"], "cap": p["cap"],
                              "cap_name": cp, "cap_rust": param_elem.get(cp) or "u32"})
            else:
                items.append({"kind": "ptr", "role": "io_buf", "name": p["name"],
                              "elem": p["elem"], "elem_w": p["elem_width"], "len_name": cp,
                              "len_rust": param_rust.get(cp) or "usize"})
        elif role == "inout_buffer":
            ln = p.get("length_param") or p.get("capacity_param")
            it = {"kind": "ptr", "role": "io_buf", "name": p["name"],
                  "elem": p["elem"], "elem_w": p["elem_width"], "len_name": ln,
                  "len_rust": param_rust.get(ln) or "usize"}
            if "max_len" in p:
                it["max_len"] = p["max_len"]
            items.append(it)
        elif role == "out_scalar":
            items.append({"kind": "ptr", "role": "out_scalar", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"]})
        elif role == "output_array":
            items.append({"kind": "ptr", "role": "out_arr", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"], "cap": p["cap"]})
        elif role == "null_pointer":
            items.append({"kind": "void_ptr", "role": "null_ptr", "name": p["name"]})
        elif role == "produced_object":
            # docs/producer_bridge_pilot.md: the object is built on each side by the library's
            # own producer from scalars decoded here; the producer's scalars are namespaced under
            # the object so `genann_run(ann, inputs)` and `genann_init(inputs, ..)` cannot collide.
            # The producer's lowered parameters are ordinary schema params (scalars, strings,
            # buffers); they get the same items/decode/call treatment as a target's, under the
            # object's namespace. Cross-references (a buffer's length param) are renamed with them.
            pre = p["name"] + "__"
            pabi = []
            for q in p["producer_params"]:
                q2 = dict(q, name=pre + q["name"])
                for k in ("length_param", "capacity_param", "of_buffer"):
                    if q2.get(k):
                        q2[k] = pre + q2[k]
                pabi.append(q2)
            items.append({"kind": "produced", "role": "produced", "name": p["name"],
                          "producer": p["producer"], "destructor": p.get("destructor"),
                          "consumed": bool(p.get("consumed")),
                          "seed_reset": p.get("seed_reset", "none"), "seed": int(p.get("seed", 42)),
                          "const": bool(p.get("const")), "struct": p["struct"],
                          "params_abi": pabi,
                          "params_items": items_from_schema({"params": pabi})})
        elif role == "plan_array":
            # HarnessPlan lowering: an allocation the harness owns, sized by a plan expression
            # (a constant, or a usize expression over already-decoded parameters), filled from the
            # fuzz input exactly when the callee reads it.  Each side gets its own copy.
            items.append({"kind": "ptr", "role": "plan_arr", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "elems": p["elems"], "fill": p.get("fill", "zero"),
                          "const": bool(p.get("const"))})
        elif role == "buffer_table":
            # T** whose rows are named by constants: each row is a plan array of its own, and
            # the table handed to the callee is a Vec of the rows' pointers (one per side).
            items.append({"kind": "ptr_ptr", "role": "buf_table", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "inner_const": bool(p.get("inner_const")), "rows": p["rows"]})
        elif role == "input_string":
            it = {"kind": "ptr", "role": "in_str", "name": p["name"],
                  "elem": p["elem"], "elem_w": p["elem_width"]}
            if "max_len" in p:
                it["max_len"] = p["max_len"]
            items.append(it)
        elif role == "input_fixed_array_buffer":
            items.append({"kind": "ptr_array", "role": "in_arr", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "inner_extent": p["inner_extent"], "len_name": p["length_param"],
                          "len_rust": param_rust.get(p["length_param"]) or "usize",
                          # `const char **` needs a table of `*const T`; `char **` a table of
                          # `*mut T`. Using the wrong one is a hard type error at the call.
                          "inner_const": bool(p.get("inner_const"))})
        elif role == "input_rectangular_pointer_table":
            items.append({"kind": "ptr_ptr", "role": "in_table", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "outer_name": p["outer_length_param"], "inner_name": p["inner_length_param"],
                          "outer_max": p["outer_max"], "inner_max": p["inner_max"]})
        elif role == "input_string_pointer_table":
            items.append({"kind": "ptr_ptr", "role": "in_str_table", "name": p["name"],
                          "elem": p["elem"], "elem_w": p["elem_width"],
                          "len_name": p["length_param"], "count_max": p["count_max"],
                          # the count is not always size_t (cJSON declares `int count`), so the
                          # decoded value has to be cast to the parameter's real Rust type
                          "len_rust": param_rust.get(p["length_param"]) or "usize",
                          # `const char **` needs a table of `*const T`; `char **` a table of
                          # `*mut T`. Using the wrong one is a hard type error at the call.
                          "inner_const": bool(p.get("inner_const"))})
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
        # length / capacity / capacity_ptr params are consumed by their owning buffer
    if schema.get("decode_scalars_first"):
        # RQ4: decode every scalar before any buffer, so a scalar's byte offset does not move when
        # a (possibly megabyte-sized) buffer changes length. Call order is unaffected: the call is
        # built from `abi`, in strict declaration order.
        buf_roles = ("in_buf", "io_buf", "out_buf_cap", "in_str", "in_arr", "in_table",
                     "in_str_table", "in_struct_arr", "io_struct_arr")
        # plan arrays go LAST: their size may be an expression over a length a buffer defines.
        items.sort(key=lambda i: 2 if i["role"] in ("plan_arr", "buf_table") else
                                 (1 if i["role"] in buf_roles else 0))
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
        if p["kind"] == "void_ptr":
            abi.append({"name": n, "role": "null_pointer", "decode": "null"})
            continue
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


def resolve(name: str, cc_dir: Path, entry: str, infer: bool = False, ignore_schema: bool = False,
            schema_path: Path | None = None):
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
    # RQ4: --schema selects an explicit file, because schemas/<program>.json is keyed to a
    # single entry and a library needs one schema per boundary.
    schema_path = Path(schema_path) if schema_path else (ROOT / "schemas" / f"{name}.json")
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
        if p["kind"] == "void_ptr":
            out.append({**p, "role": "null_ptr"})
            i += 1
            continue
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
    # Two variable-length inputs cannot both take the REST of the fuzz bytes: the first would
    # starve the second (cJSON_GetObjectItem: the producer's JSON text ate everything, the key was
    # always empty). Every rest-taking input but the last, in decode order and through produced
    # objects, is length-delimited instead. Marked once, at the top level.
    if not any(i.get("_lenpref") is not None for i in items):
        def _rest_takers(its):
            out = []
            for i in its:
                if i.get("role") == "produced":
                    out += _rest_takers(i["params_items"])
                elif i.get("role") in ("in_str", "in_buf", "io_buf") and (i["role"] == "in_str" or "max_len" in i):
                    out.append(i)
            return out
        rt = _rest_takers(items)
        for i in rt[:-1]:
            i["_lenpref"] = True
        for i in rt[-1:]:
            i["_lenpref"] = False
    for it in items:
        n = it["name"]
        if it["role"] == "scalar":
            if it.get("decode") == "bounded_scalar":
                hi, ty = it["max_value"], it["rust"]
                mn = it["min_var"] if it.get("min_var") else it["min_value"]
                # RQ4: rem_euclid, not %. Rust's % keeps the sign of the dividend, so a signed
                # bounded_scalar produced values in [min-(span-1), max] -- e.g. bzip2's
                # blockSize100k, declared 1..9, actually ranged -7..9 and half of every campaign
                # was spent on inputs the callee rejects outright.
                decode.append(f"    let {n} = ({mn} as {ty}) + (cur.take_{ty}()"
                              f".rem_euclid((({hi}) as {ty}) - ({mn} as {ty}) + (1 as {ty})));")   # `+ 1` is an integer literal: E0277 on a bounded f64 (lil_alloc_double)
            else:
                decode.append(f"    let {n} = cur.take_{it['rust']}();")
        elif it["role"] == "in_buf":
            take = (f"cur.take_len_{it['elem']}({it['max_len']})" if it.get("_lenpref")
                    else f"cur.take_rest_{it['elem']}({it['max_len']})" if "max_len" in it
                    else f"cur.take_vec_{it['elem']}()")
            decode.append(f"    let mut {n}_buf: Vec<{it['elem']}> = {take};")
            decode.append(f"    {n}_buf.reserve(1); unsafe {{ *{n}_buf.as_mut_ptr().add({n}_buf.len()) = 0 as {it['elem']}; }}  // sentinel past len: a length-0 buffer is still a valid, NUL-terminated pointer (lil_parse: codelen 0 => strlen(code)); an empty Vec's as_ptr() is dangling")
            decode.append(f"    let {it['len_name']} = {n}_buf.len(){_len_cast(it)};")
        elif it["role"] == "io_buf":
            take = (f"cur.take_len_{it['elem']}({it['max_len']})" if it.get("_lenpref")
                    else f"cur.take_rest_{it['elem']}({it['max_len']})" if "max_len" in it
                    else f"cur.take_vec_{it['elem']}()")
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = {take};")
            decode.append(f"    let {it['len_name']} = {n}_c.len(){_len_cast(it)};")
            decode.append(f"    let mut {n}_r = {n}_c.clone();")
            for _side in ("c", "r"):        # sentinel past len on BOTH copies (see in_buf)
                decode.append(f"    {n}_{_side}.reserve(1); unsafe {{ *{n}_{_side}.as_mut_ptr().add({n}_{_side}.len()) = 0 as {it['elem']}; }}")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: buffer {n}"); }}')
        elif it["role"] == "out_buf_cap":
            # RQ4: output buffer whose capacity is passed by pointer. Both sides get their own
            # zeroed allocation of `cap` elements and their own capacity cell seeded with `cap`.
            # After the call the callee has overwritten the cell with the length it produced, so
            # the observable output is that length plus the prefix it names -- comparing the whole
            # allocation would compare bytes the callee never wrote.
            c, cn, cr = it["cap"], it["cap_name"], it["cap_rust"]
            decode.append(f"    let mut {n}_c: Vec<{it['elem']}> = vec![0 as {it['elem']}; {c}];")
            decode.append(f"    let mut {n}_r: Vec<{it['elem']}> = vec![0 as {it['elem']}; {c}];")
            decode.append(f"    let mut {cn}_c: {cr} = {c} as {cr};")
            decode.append(f"    let mut {cn}_r: {cr} = {c} as {cr};")
            post.append(f'    if {cn}_c != {cn}_r {{ panic!("divergence: written length {cn}"); }}')
            post.append(f"    {{ let _n = ({cn}_c as usize).min({c});")
            post.append(f'      if {n}_c[.._n] != {n}_r[.._n] {{ panic!("divergence: output buffer {n}"); }} }}')
        elif it["role"] == "produced":
            # the producer's inputs are decoded here exactly like a target's (namespaced); the two
            # objects are built in the body, one per side. Their post-call comparison, when a
            # comparator plugin knows the type, is emitted in the body too, not here.
            d2, _p2 = _decode_and_post(list(it["params_items"]))
            decode.extend(d2)
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
        elif it["role"] == "plan_arr":
            e = it["elem"]
            decode.append(f"    let {n}_n: usize = {it['elems']};")
            if it["fill"] == "fuzz":
                decode.append(f"    let mut {n}_c: Vec<{e}> = "
                              f"(0..{n}_n).map(|_| cur.take_{e}()).collect();")
            else:
                decode.append(f"    let mut {n}_c: Vec<{e}> = vec![0 as {e}; {n}_n];")
            decode.append(f"    let mut {n}_r: Vec<{e}> = {n}_c.clone();")
            post.append(f'    if {n}_c != {n}_r {{ panic!("divergence: array {n}"); }}')
        elif it["role"] == "buf_table":
            e = it["elem"]
            _pk = "const" if it.get("inner_const") else "mut"
            _as = "as_ptr" if it.get("inner_const") else "as_mut_ptr"
            for k, row in enumerate(it["rows"]):
                rn = f"{n}__{k}"
                decode.append(f"    let {rn}_n: usize = {row['elems']};")
                if row["fill"] == "fuzz":
                    decode.append(f"    let mut {rn}_c: Vec<{e}> = "
                                  f"(0..{rn}_n).map(|_| cur.take_{e}()).collect();")
                else:
                    decode.append(f"    let mut {rn}_c: Vec<{e}> = vec![0 as {e}; {rn}_n];")
                decode.append(f"    let mut {rn}_r: Vec<{e}> = {rn}_c.clone();")
                if row["written"]:
                    if e in ("f32", "f64"):
                        # bit-for-bit: `!=` on floats calls NaN != NaN, which would report a
                        # divergence both sides produced identically
                        post.append(f'    if {rn}_c.len() != {rn}_r.len() || {rn}_c.iter().zip({rn}_r.iter())'
                                    f'.any(|(x, y)| x.to_bits() != y.to_bits()) '
                                    f'{{ panic!("divergence: table {n} row {k}"); }}')
                    else:
                        post.append(f'    if {rn}_c != {rn}_r {{ panic!("divergence: table {n} row {k}"); }}')
            for side in ("c", "r"):
                decode.append(f"    let {n}_tab_{side}: Vec<*{_pk} {e}> = vec!["
                              + ", ".join(f"{n}__{k}_{side}.{_as}()" for k in range(len(it["rows"])))
                              + "];")
        elif it["role"] == "in_str":
            # take_vec_* caps at 63 bytes, which is far too short for a real parser input; the
            # plan's policy bound is used when it has one.
            _take = (f"cur.take_len_{it['elem']}({it.get('max_len', 65535)})" if it.get("_lenpref")
                     else f"cur.take_rest_{it['elem']}({it['max_len']})" if "max_len" in it
                     else f"cur.take_vec_{it['elem']}()")
            decode.append(f"    let mut {n}_buf: Vec<{it['elem']}> = {_take};")
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
            _lr = it.get("len_rust") or "usize"
            decode.append(f"    let {ln} = ((cur.byte() as usize) % ({cm} + 1)) as {_lr};")
            _pk = "const" if it.get("inner_const") else "mut"
            _as = "as_ptr" if it.get("inner_const") else "as_mut_ptr"
            decode.append(f"    let {n}_data: Vec<Vec<{el}>> = (0..{ln}).map(|_| "
                          f"{{ let mut s = cur.take_vec_{el}(); s.push(0 as {el}); s }}).collect();")
            for side in ("c", "r"):
                decode.append(f"    let mut {n}_back_{side} = {n}_data.clone();")
                decode.append(f"    let mut {n}_tab_{side}: Vec<*{_pk} {el}> = "
                              f"{n}_back_{side}.iter_mut().map(|s| s.{_as}()).collect();")
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



def _base_exprs(abi: list[dict], base: str) -> tuple[str, str, str, str]:
    """(c_base, r_base, c_len, r_len) expressions for the buffer an interior pointer points into.

    Shared inputs (input_buffer / input_string) give both sides the SAME allocation, so the two
    offsets are taken against one base; mutable buffers give each side its own copy, so each offset
    is taken against that side's own base. Comparing offsets, never addresses, is the whole point.
    """
    role = next((p["role"] for p in abi if p["name"] == base), None)
    if role in ("input_buffer", "input_string"):
        return (f"{base}_buf.as_ptr()", f"{base}_buf.as_ptr()",
                f"{base}_buf.len()", f"{base}_buf.len()")
    if role in ("inout_buffer", "output_buffer"):
        return (f"{base}_c.as_ptr()", f"{base}_r.as_ptr()", f"{base}_c.len()", f"{base}_r.len()")
    raise SystemExit(f"interior_pointer base {base!r} has unusable role {role!r}")


# Pointer typedefs of the translation (`pub type lil_t = *mut _lil_t;`, c2rust keeps them where
# Laertes/CROWN spell the pointer out). Set once per generation from the .rs text; the decisions
# below read the TRANSLATION's parameter/return types and must see the pointer, not its name.
# Resolved at the point of use so plans.json keeps the translation's own spelling.
_RUST_ALIASES: dict = {}


def _ptr_alias(rty: str | None) -> str:
    t = (rty or "").strip()
    return _RUST_ALIASES.get(t, t)


def hp_norm(rty: str) -> str:
    """The translation's scalar type resolved to a primitive (aliases + C-ABI names), for casts."""
    import harness_plan as _hp
    return _hp._norm_ty(rty, _RUST_ALIASES)


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
        rty = _ptr_alias(p.get("rust_pty")).replace(" ", "")
        if role in ("scalar", "length", "capacity"):
            # The C-ABI decl uses the generator's own mapping (`size_t` -> usize); a faithful
            # translation may spell the same C type differently (c2rust: `size_t` = c_ulong = u64).
            # Same width, different name: the plan's `scalar_cast` bridge, materialised here.
            _rt = hp_norm(rty) if rty else ""
            cast = f"{n} as {_rt}" if (_rt in _INT_TYPES or _rt in ("f32", "f64")) and _rt != p["rust"] else n
            c_args.append(n); r_pairs.append((n, cast)); decl.append(f"{n}: {p['rust']}")
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
        elif role in ("out_scalar", "capacity_ptr"):
            c_args.append(f"&mut {n}_c")
            decl.append(f"{n}: *mut {p['elem']}")
            # Safety lifters raise a raw out-pointer to Option<&mut T> (Laertes and CROWN both do
            # this to bzip2's `unsigned int* destLen`). The generator already bridges the buffer
            # form Option<&mut [T]>; the scalar form needs the same wrapper or the harness will not
            # compile against the lifted signature.
            if rty.startswith("Option<&mut"):
                r_pairs.append((n, f"Some(&mut {n}_r)"))
            else:
                r_pairs.append((n, f"&mut {n}_r"))
        elif role == "output_array":
            c_args.append(f"{n}_c.as_mut_ptr()")
            decl.append(f"{n}: *mut {p['elem']}")
            if rty.startswith("Option<&mut["):     # Option<&mut [T]>
                r_pairs.append((n, f"Some(&mut {n}_r[..])"))
            elif rty.startswith("&mut["):           # &mut [T]
                r_pairs.append((n, f"&mut {n}_r[..]"))
            else:                                    # raw pointer / default
                r_pairs.append((n, f"{n}_r.as_mut_ptr()"))
        elif role == "null_pointer":
            c_args.append("core::ptr::null_mut()")
            r_pairs.append((n, "core::ptr::null_mut()"))
            decl.append(f"{n}: *mut core::ffi::c_void")
        elif role == "produced_object":
            cst = "const" if p.get("const") else "mut"
            c_args.append(f"{n}_c as *{cst} core::ffi::c_void")
            decl.append(f"{n}: *{cst} core::ffi::c_void")
            # Mutability follows the RUST parameter, not the C one: CROWN lifts `const genann*`
            # to `Option<&mut genann>` because the function writes ann->output. The object is
            # ours (a raw pointer from the producer), so either reborrow is available.
            br, rty = p.get("bridge"), _ptr_alias(p.get("rust_pty")).replace(" ", "")
            want_mut = "&mut" in rty or rty.startswith("*mut")
            if br == "ref_obj":
                r_pairs.append((n, f"&mut *{n}_r" if want_mut else f"&*{n}_r"))
            elif br == "opt_ref_obj":
                r_pairs.append((n, f"Some(&mut *{n}_r)" if want_mut else f"Some(&*{n}_r)"))
            else:
                r_pairs.append((n, f"{n}_r" if want_mut else f"{n}_r as *const _"))
        elif role == "plan_array" and p.get("one_elem") and (
                rty.startswith("&mut") or rty.startswith("Option<&mut") or rty.startswith("&")
                or rty.startswith("Option<&")):
            # a one-element allocation the translation renders as a scalar reference
            c_args.append(f"{n}_c.as_mut_ptr()")
            decl.append(f"{n}: *mut {p['elem']}")
            r_pairs.append((n, f"Some(&mut {n}_r[0])" if rty.startswith("Option<")
                            else f"&mut {n}_r[0]"))
        elif role == "plan_array":
            if p.get("const"):
                c_args.append(f"{n}_c.as_ptr()")
                decl.append(f"{n}: *const {p['elem']}")
                r_pairs.append((n, f"&{n}_r[..]" if rty.startswith("&[") else f"{n}_r.as_ptr()"))
            else:
                c_args.append(f"{n}_c.as_mut_ptr()")
                decl.append(f"{n}: *mut {p['elem']}")
                if rty.startswith("Option<&mut["):
                    r_pairs.append((n, f"Some(&mut {n}_r[..])"))
                elif rty.startswith("&mut["):
                    r_pairs.append((n, f"&mut {n}_r[..]"))
                else:
                    r_pairs.append((n, f"{n}_r.as_mut_ptr()"))
        elif role == "input_string":
            c_args.append(f"{n}_buf.as_ptr()"); r_pairs.append((n, f"{n}_buf.as_ptr()"))
            decl.append(f"{n}: *const {p['elem']}")
        elif role == "input_fixed_array_buffer":
            c_args.append(f"{n}_buf.as_ptr()"); r_pairs.append((n, f"{n}_buf.as_ptr()"))
            decl.append(f"{n}: *const [{p['elem']}; {p['inner_extent']}]")
        elif role == "buffer_table":
            _pk = "const" if p.get("inner_const") else "mut"
            c_args.append(f"{n}_tab_c.as_ptr()"); r_pairs.append((n, f"{n}_tab_r.as_ptr()"))
            decl.append(f"{n}: *const *{_pk} {p['elem']}")
        elif role in ("input_rectangular_pointer_table", "input_string_pointer_table"):
            c_args.append(f"{n}_tab_c.as_mut_ptr()"); r_pairs.append((n, f"{n}_tab_r.as_mut_ptr()"))
            _pk = "const" if p.get("inner_const") else "mut"
            decl.append(f"{n}: *mut *{_pk} {p['elem']}")
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
    # float-cast-overflow is NOT optional: casting an out-of-range double to int is UB in C and
    # the two sides disagree by construction -- x86-64 `cvttsd2si` yields INT_MIN, Rust's `as i32`
    # saturates to i32::MAX. cJSON's `item->valueint = (int)n` (cJSON.c:112) hits it on any number
    # literal above 2^31, and without this flag the gate lets it through and the difference is
    # misreported as a translation defect.
    "-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable,"
    "float-cast-overflow,pointer-overflow,return,vla-bound",
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
/* float_cast_overflow is the one that matters for a differential oracle: the two sides disagree
   BY CONSTRUCTION on an out-of-range double->int cast (x86-64 gives INT_MIN, Rust saturates), so
   without this handler the gate lets the input through and a UB-associated value difference is
   misreported as a translation defect. */
H(float_cast_overflow) H(missing_return) H(vla_bound_not_positive)
H(nonnull_arg) H(invalid_builtin) H(alignment_assumption)
#undef H
'''


def clang_major() -> int | None:
    """Major version of the `clang` on PATH (the compiler build.rs pins), or None if absent."""
    import re as _re
    import subprocess as _sp
    try:
        out = _sp.run(["clang", "--version"], capture_output=True, text=True, timeout=10).stdout
    except (OSError, _sp.TimeoutExpired):
        return None
    m = _re.search(r"clang version (\d+)", out)
    return int(m.group(1)) if m else None


def ub_sanitize_flags() -> list[str]:
    """UB_SANITIZE_FLAGS, plus `-fno-sanitize-link-runtime` on clang >= 21.

    clang >= 21 ships `__ubsan_handle_*_minimal` in libclang_rt.ubsan_minimal and links that
    runtime whenever -fsanitize-minimal-runtime reaches a driver link, so UBSHIM_C's handlers
    collide with it (duplicate-symbol link error; hit by the 2026-08-25 pilots). With
    -fno-sanitize-link-runtime the shim IS the runtime. Older clang only pulls its archive
    runtime for handlers the shim lacks (none), so it never collided; keep its flag set
    unchanged. build.rs compiles with -c (rustc links, no runtime pulled) so the flag is inert
    there -- it matters for anything that reuses this flag set on a clang-driver link (probes)."""
    major = clang_major()
    if major is not None and major >= 21:
        return UB_SANITIZE_FLAGS + ["-fno-sanitize-link-runtime"]
    return list(UB_SANITIZE_FLAGS)


def gen_target_rust_only(entry: str, items: list[dict], abi: list[dict], ret: str, crate: str,
                         rust_entry: str | None = None) -> str:
    """E3 depth harness: pure Rust, NO C oracle, NO differential compare. Decode fuzz bytes into
    the entry's params and call ONLY translated::<entry>. Used to measure per-function hit-depth
    (llvm-cov counts the Rust crate); correctness/divergence is E1's job, not E3's."""
    decode, _post = _decode_and_post(items)
    _c_args, r_args, _decl = _call_and_decl(abi)
    call_r = f"translated::{rust_entry or entry}({', '.join(r_args)})"
    body = f"        {call_r};" if ret == "void" else f"        let _ = {call_r};"
    return "\n".join([
        "#![no_main]",
        "#![allow(unused, unused_mut, non_snake_case)]",
        'use libfuzzer_sys::fuzz_target;',
        "struct Cur<'a> { d: &'a [u8], p: usize }",
        "impl<'a> Cur<'a> {",
        "    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }",
        "    fn byte(&mut self) -> u8 { let b = if self.p < self.d.len() { self.d[self.p] } else { 0 }; self.p += 1; b }",
        *[f"    fn take_{t}(&mut self) -> {t} {{ let mut v = [0u8; {w}]; for i in 0..{w} {{ v[i] = self.byte(); }} {t}::from_le_bytes(v) }}"
          for t, w in [("u8", 1), ("i8", 1), ("u16", 2), ("i16", 2), ("u32", 4), ("i32", 4),
                       ("u64", 8), ("i64", 8), ("usize", 8), ("f32", 4), ("f64", 8)]],
        *[f"    fn take_vec_{t}(&mut self) -> Vec<{t}> {{ let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_{t}()).collect() }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]],
        *[f"    fn take_rest_{t}(&mut self, max: usize) -> Vec<{t}> {{ let mut v = Vec::new(); "
          f"while self.p < self.d.len() && v.len() < max {{ v.push(self.take_{t}()); }} v }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]],
        "}",
        "",
        f"use {crate} as translated;",
        "",
        "fuzz_target!(|data: &[u8]| {",
        "    let mut cur = Cur::new(data);",
        *decode,  # full decode kept; _r depends on _c (clone), _c side simply goes unused
        "    unsafe {",
        body,
        "    }",
        "});",
        "",
    ])


def gen_target(entry: str, items: list[dict], abi: list[dict], ret: str, crate: str,
               ub_free: bool = False, rust_entry: str | None = None,
               rust_ret: str | None = None, ret_contract: dict | None = None,
               plugins: list[dict] | None = None) -> str:
    """Generate the fuzz_target source: decode from items, call/signature in ABI order.

    When ub_free, the C oracle is UBSan-instrumented (recover + minimal runtime, flag-based
    -- see UBSHIM_C). The harness resets the flag, runs C, and REJECTS the input (returns
    without comparing) if C hit UB. So a divergence is reported only on UB-free input -- the
    fuzzer's gradient points at real translation bugs, not UB artifacts (in-loop, vs the
    post-hoc per-artifact exclusion in classify_artifact.py)."""
    decode, post = _decode_and_post(items)
    c_args, r_args, decl = _call_and_decl(abi)
    extern_args = ", ".join(decl)
    if ret == "void":
        extern_ret = ""
    elif ret == "ptr":
        rcp = ret_contract or {}
        _cst = "const" if rcp.get("const", True) else "mut"
        extern_ret = f"-> *{_cst} {rcp.get('elem') or 'core::ffi::c_void'}"
    else:
        extern_ret = f"-> {ret}"

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

    # Producer bridge (docs/producer_bridge_pilot.md section 5): the sequence
    # producer -> target -> destructor, one object per side, phase-marked, NULL-aware.
    _produced = [it for it in items if it.get("role") == "produced"]
    _prod_c, _prod_r, _null_c, _null_r, _null_cmp, _free_c, _free_r, _prod_externs = [], [], [], [], [], [], [], []
    _obj_cmp_after, _obj_plugin = [], None
    for it in _produced:
        n, pr, ds = it["name"], it["producer"], it.get("destructor")
        # the producer's call is assembled by the same routine as the target's, so a `&str` or
        # slice parameter of an idiomatic producer is bridged exactly as it would be for a target
        pc, pr_args, pdecl = _call_and_decl(it["params_abi"])
        _prod_externs.append(f"    fn c_{pr}({', '.join(pdecl)}) -> *mut core::ffi::c_void;")
        if ds:
            _prod_externs.append(f"    fn c_{ds}(p: *mut core::ffi::c_void);")
        seed = ([f"        srand({it['seed']} as core::ffi::c_uint);"] if it["seed_reset"] == "libc" else [])
        # The in-loop UB gate must cover the PRODUCER's C call too: cJSON_Parse's `(int)double`
        # cast is UB on out-of-range numbers, and without this the two objects differed before the
        # target ran and were reported as a producer divergence on the faithful translation.
        _prod_c += ["        c2r_phase(C2R_PH_PRODUCER);", *seed,
                    *(["        c2r_ub_reset();"] if ub_free else []),
                    f"        let {n}_c = c_{pr}({', '.join(pc)});",
                    *(['        if _c2r_m == C2R_GATED && c2r_ub_get() != 0 { c2r_outcome("ub-gated", ""); return; }  // C producer hit UB -> reject']
                      if ub_free else [])]
        _prod_r += ["        c2r_phase(C2R_PH_PRODUCER);", *seed, f"        let {n}_r = translated::{pr}({', '.join(pr_args)});"]
        _null_c.append(f'        if {n}_c.is_null() {{ c2r_outcome("normal", ""); return; }}  // producer rejected the input')
        _null_r.append(f'        if {n}_r.is_null() {{ c2r_outcome("normal", ""); return; }}')
        _null_cmp += [f'        if {n}_c.is_null() != {n}_r.is_null() {{ c2r_div("producer {pr} nullness"); }}',
                      f'        if {n}_c.is_null() {{ c2r_outcome("normal", ""); return; }}']
        # A comparator plugin that knows the produced type turns the sequence-level oracle into an
        # attributed one: the two objects are canonicalised right after the producers (a difference
        # there belongs to the producer) and again after the target (a difference there is the
        # target's effect on the object). cJSON has such a plugin; genann does not.
        pl = _match_plugin({"inner": {"name": it["struct"]}}, plugins or [])
        if pl is not None:
            _obj_plugin = pl
            cap = int(pl.get("max_bytes", 1 << 20))
            def _canon(tag, what):
                return [f"        {{ let mut _ob = vec![0u8; {cap}];",
                        f"          let _on = c2r_canon({n}_c as *const core::ffi::c_void, _ob.as_mut_ptr() as *mut i8, _ob.len());",
                        f"          let _or = c2r_canon_rust({n}_r as *const core::ffi::c_void);",
                        f"          if _on > _ob.len() || _or.len() > {cap} {{ c2r_div(\"canonical form exceeded the plugin buffer ({tag})\"); }}",
                        f"          else if _ob[.._on] != _or[..] {{ c2r_div(\"{what}\"); }} }}"]
            _null_cmp += _canon("producer", f"producer {pr} state of {n}")
            if not it.get("consumed"):     # the target freed it: reading it back would be OUR use-after-free
                _obj_cmp_after += _canon("after", f"produced object {n} state after {entry}")
        if ds:
            _free_c += ["        c2r_phase(C2R_PH_FREE);", f"        if !{n}_c.is_null() {{ c_{ds}({n}_c); }}"]
            _free_r += ["        c2r_phase(C2R_PH_FREE);", f"        if !{n}_r.is_null() {{ translated::{ds}({n}_r); }}"]
    if any(it["seed_reset"] == "libc" for it in _produced):
        _prod_externs.append("    fn srand(seed: core::ffi::c_uint);")
    _ind = lambda ls: [l.replace("        ", "            ", 1) for l in ls]
    # UB-free gate: reset before C, reject (return) if C tripped UB, then call Rust.
    pre = "        c2r_ub_reset();\n" if ub_free else ""
    gate = ('        if _c2r_m == C2R_GATED && c2r_ub_get() != 0 '
            '{ c2r_outcome("ub-gated", ""); return; }  // C hit UB -> reject\n'
            if ub_free else "")
    if ret == "void":
        body_call = f"{pre}        {call_c};\n{gate}"
        rust_call = f"        {call_r};"
        ret_cmp = ""
    elif decode_shape:
        osc = out_scalars[0]
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        ret_cmp = (
            f"        let (c_ok, c_val, c_cons) = (c_ret != 0, {osc}_c, c_ret);\n"
            f"        let (r_ok, r_val, r_cons) = match r_ret {{ Some((v, c)) => (true, v, c), None => (false, 0, 0) }};\n"
            f'        if c_ok != r_ok {{ panic!("divergence: success/None mismatch"); }}\n'
            f'        if c_ok && (c_val != r_val || (c_cons as i128) != (r_cons as i128)) {{ panic!("divergence: decoded value/consumed"); }}'
        )
    elif (ret_contract or {}).get("template") == "structured_object":
        rc = ret_contract
        cap = int((rc.get("limits") or {}).get("extract_cap", 1 << 20))
        fc, fr = rc["free"]["c"], rc["free"]["rust"]
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        ret_cmp = (
            "        // Compare CANONICAL EXTRACTIONS, never addresses and never the library's own\n"
            "        // printer: a printer is translated code too, so a defect it also mis-handles\n"
            "        // would be hidden (cf. S8, cJSON valuestring lost on the success path).\n"
            f"        let mut _cbuf = vec![0u8; {cap}];\n"
            f"        let _cn = c2r_extract(c_ret as *const core::ffi::c_void,\n"
            f"                              _cbuf.as_mut_ptr() as *mut i8, _cbuf.len());\n"
            f"        let _rs = c2r_r_extract(r_ret as *const _);\n"
            f"        if _cn > _cbuf.len() || _rs.len() > {cap} {{\n"
            '            c2r_div("structured object extraction exceeded the buffer (raise limits)");\n'
            "        } else if _cbuf[.._cn] != _rs[..] {\n"
            '            c2r_div("structured object canonical extraction");\n'
            "        }\n"
            "        // free is an OBSERVATION step, not cleanup: a normal return, a Rust panic, an\n"
            "        // abort or a sanitizer failure here are all outcomes of the boundary. A panic is\n"
            "        // caught and reported; an abort or sanitizer report terminates and is attributed\n"
            "        // by the saved crash input and its stack.\n"
            f"        c_{fc}(c_ret as *mut core::ffi::c_void);\n"
            "        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {\n"
            f"            translated::{fr}(r_ret)\n"
            "        })).is_err() {\n"
            '            c2r_div("panic while freeing the Rust object");\n'
            "        }")
    elif (ret_contract or {}).get("template") == "comparator_plugin":
        pl = ret_contract["plugin"]
        cap = int(pl.get("max_bytes", 1 << 20))
        free = pl.get("free") or {}
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        _free_lines = []
        # With a produced object in play the boundary may return a pointer INTO it
        # (cJSON_GetObjectItem); freeing that would corrupt the parent the destructor frees later.
        # Fresh returns then leak (detect_leaks=0) rather than risk a double free.
        if free.get("c") and free.get("rust") and not _produced:
            _free_lines = [
                "        // Releasing what the boundary allocated is CONTRACT, not comparison: a",
                "        // campaign without it is dominated by RSS growth. A panic while freeing",
                "        // is itself an outcome of the boundary, so it is reported, not swallowed.",
                f"        c_{free['c']}(c_ret as *mut core::ffi::c_void);",
                "        if std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {",
                f"            translated::{free['rust']}(r_ret)",
                "        })).is_err() {",
                '            c2r_div("panic while freeing the object returned by the boundary");',
                "        }"]
        ret_cmp = "\n".join([
            "        // Rung 5: the user comparator's canonical bytes. Never an address, and never",
            "        // the library's own printer -- a printer is translated code too, so a defect",
            "        // it also mis-handles would be invisible.",
            f"        let mut _cbuf = vec![0u8; {cap}];",
            "        let _cn = c2r_canon(c_ret as *const core::ffi::c_void,",
            "                            _cbuf.as_mut_ptr() as *mut i8, _cbuf.len());",
            "        let _rs = c2r_canon_rust(r_ret as *const core::ffi::c_void);",
            f"        if _cn > _cbuf.len() || _rs.len() > {cap} {{",
            '            c2r_div("canonical form exceeded the plugin buffer (raise max_bytes)");',
            "        } else if _cbuf[.._cn] != _rs[..] {",
            '            c2r_div("canonical object comparison");',
            "        }"] + _free_lines)
    elif (ret_contract or {}).get("template") == "pointer_nullness":
        # Rung 3 of the fixed ladder. Two allocators never agree on an address, so only nullness
        # is comparable; everything past this rung needs a comparator plugin.
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        rn = (ret_contract or {}).get("rust_null") or "(r_ret as *const core::ffi::c_void).is_null()"
        ret_cmp = (
            "        let c_null = (c_ret as *const core::ffi::c_void).is_null();\n"
            f"        let r_null = {rn};\n"
            '        if c_null != r_null { c2r_div("returned pointer nullness"); }')
    elif (ret_contract or {}).get("template") == "interior_pointer":
        # The returned pointer is compared as NULLNESS + OFFSET FROM ITS DECLARED BASE, never as an
        # address: two runs allocate at different addresses, so an address comparison is meaningless
        # and an equality test on raw pointers would be either always-false or accidentally true.
        base = ret_contract["base"]
        cb, rb, cl, rl = _base_exprs(abi, base)
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        ret_cmp = (
            f"        let c_i = c2r_interior(c_ret as *const u8, {cb} as *const u8, {cl});\n"
            f"        let r_i = c2r_interior(r_ret as *const u8, {rb} as *const u8, {rl});\n"
            f"        if c_i == C2rInterior::OutOfRange || r_i == C2rInterior::OutOfRange {{\n"
            f'            c2r_div("interior pointer outside the declared base buffer {base}");\n'
            f"        }} else if c_i != r_i {{\n"
            f'            c2r_div("interior pointer offset into {base}");\n'
            f"        }}")
    else:
        body_call = f"{pre}        let c_ret = {call_c};\n{gate}"
        rust_call = f"        let r_ret = {call_r};"
        # idiomatic translations may return a different-but-compatible integer width/signedness
        # (e.g. C `int`/i32 vs Rust `isize`); compare via i128 so the widths line up.
        # `rust_ret` may be a typedef of the translation (c2rust: `size_t` = u64 for the
        # generator's usize); resolve it before deciding, or the two spellings compare as
        # different types and rustc rejects `c_ret != r_ret` (lil_list_size).
        _rr = hp_norm(rust_ret) if rust_ret else rust_ret
        if _rr and _rr != ret and ret in _INT_TYPES and _rr in _INT_TYPES:
            cmp = "(c_ret as i128) != (r_ret as i128)"
        else:
            cmp = "c_ret != r_ret"
        ret_cmp = f'        if {cmp} {{ panic!("divergence: return value"); }}'
    ub_externs = ["    fn c2r_ub_reset();", "    fn c2r_ub_get() -> i32;"] if ub_free else []
    _pl = (ret_contract or {}).get("template") == "comparator_plugin"
    if _pl:
        _p = ret_contract["plugin"]
        ub_externs += ["    fn c2r_canon(obj: *const core::ffi::c_void, out: *mut i8, "
                       "cap: usize) -> usize;"]
        _f = _p.get("free") or {}
        if _f.get("c"):
            ub_externs += [f"    fn c_{_f['c']}(p: *mut core::ffi::c_void);"]
    _so = (ret_contract or {}).get("template") == "structured_object"
    if _so:
        ub_externs += [
            "    fn c2r_extract(p: *const core::ffi::c_void, out: *mut i8, cap: usize) -> usize;",
            f"    fn c_{ret_contract['free']['c']}(p: *mut core::ffi::c_void);",
        ]
    _extractor = ctpl.rust_extractor_lines(ret_contract, crate) if _so else []
    if _pl:
        # the plugin's Rust half, verbatim. It is written against `translated`, so it names no
        # crate -- but it reads the translated struct's fields BY NAME, so it is reusable only
        # across translations whose layout stays compatible with the C header's.
        _extractor = (Path(ret_contract["plugin"]["_rust_source"])
                      .read_text(encoding="utf-8").split("\n"))
    _rust_only_free = ([f"            translated::{ret_contract['free']['rust']}(r_ret);"]
                       if _so and not _produced else [])
    if _obj_plugin is not None and not _pl:
        # the produced object's type has a comparator plugin even though the return does not:
        # link the plugin's two halves for the object-state comparison
        ub_externs += ["    fn c2r_canon(obj: *const core::ffi::c_void, out: *mut i8, "
                       "cap: usize) -> usize;"]
        _extractor = Path(_obj_plugin["_rust_source"]).read_text(encoding="utf-8").split("\n")
    # emitted ONLY for a boundary that actually uses the interior-pointer contract, so every other
    # generated target is byte-identical to before this template existed
    _interior_helper = ([
        "// interior-pointer contract: [base, base+len] is IN contract, one-past-end included,",
        "// which C permits and parsers rely on. NULL is a distinct outcome. Anything else is out",
        "// of contract and is reported rather than silently compared.",
        "#[derive(PartialEq, Eq, Debug)]",
        "enum C2rInterior { Null, At(usize), OnePastEnd, OutOfRange }",
        "fn c2r_interior(p: *const u8, base: *const u8, len: usize) -> C2rInterior {",
        "    if p.is_null() { return C2rInterior::Null; }",
        "    let (a, b) = (p as usize, base as usize);",
        "    if a < b || a > b.wrapping_add(len) { return C2rInterior::OutOfRange; }",
        "    if a == b.wrapping_add(len) { return C2rInterior::OnePastEnd; }",
        "    C2rInterior::At(a - b)",
        "}",
        "",
    ] if (ret_contract or {}).get("template") == "interior_pointer" else [])

    return "\n".join([
        "#![no_main]",
        'use libfuzzer_sys::fuzz_target;',
        "struct Cur<'a> { d: &'a [u8], p: usize }",
        "impl<'a> Cur<'a> {",
        "    fn new(d: &'a [u8]) -> Self { Cur { d, p: 0 } }",
        "    fn byte(&mut self) -> u8 { let b = if self.p < self.d.len() { self.d[self.p] } else { 0 }; self.p += 1; b }",
        *[f"    fn take_{t}(&mut self) -> {t} {{ let mut v = [0u8; {w}]; for i in 0..{w} {{ v[i] = self.byte(); }} {t}::from_le_bytes(v) }}"
          for t, w in [("u8", 1), ("i8", 1), ("u16", 2), ("i16", 2), ("u32", 4), ("i32", 4),
                       ("u64", 8), ("i64", 8), ("usize", 8), ("f32", 4), ("f64", 8)]],
        *[f"    fn take_vec_{t}(&mut self) -> Vec<{t}> {{ let n = (self.byte() as usize) % 64; (0..n).map(|_| self.take_{t}()).collect() }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]],
        *[f"    fn take_rest_{t}(&mut self, max: usize) -> Vec<{t}> {{ let mut v = Vec::new(); "
          f"while self.p < self.d.len() && v.len() < max {{ v.push(self.take_{t}()); }} v }}"
          for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]],
        # length-delimited take, emitted only when a target has more than one rest-taking input
        # (so every earlier generated target stays byte-identical): a u16 prefix picks the length
        *([f"    fn take_len_{t}(&mut self, max: usize) -> Vec<{t}> {{ let n = (self.take_u16() as usize) % (max.min(65535) + 1); "
           f"let mut v = Vec::new(); while self.p < self.d.len() && v.len() < n {{ v.push(self.take_{t}()); }} v }}"
           for t in ["u8", "i8", "u16", "i16", "u32", "i32", "u64", "i64", "f32", "f64"]]
          if any("take_len_" in l for l in decode) else []),
        "}",
        "",
        "// RQ4: C reference and UB gate are selected at RUN TIME so every mode shares one binary,",
        "// one coverage map and one set of identities. C2R_MODE=gated|nogate|rust-only.",
        "const C2R_GATED: u8 = 0; const C2R_NOGATE: u8 = 1; const C2R_RUST_ONLY: u8 = 2;",
        "const C2R_COVERAGE: u8 = 3; const C2R_C_ONLY: u8 = 4;",
        "fn c2r_mode() -> u8 {",
        "    static M: std::sync::OnceLock<u8> = std::sync::OnceLock::new();",
        "    *M.get_or_init(|| match std::env::var(\"C2R_MODE\").as_deref() {",
        "        Ok(\"nogate\") => C2R_NOGATE,",
        "        Ok(\"rust-only\") => C2R_RUST_ONLY,",
        "        Ok(\"coverage\") => C2R_COVERAGE,",
        "        // Confirmation phase A: run ONLY C, so a sanitizer report is unambiguously C's.",
        "        // Sanitizing both sides at once makes the report unattributable.",
        "        Ok(\"c-only\") => C2R_C_ONLY,",
        "        // `combined` is the both-sides replay and is the default. `gated` stays as an",
        "        // alias: the in-loop UB gate used to be the main line and no longer is.",
        "        Ok(\"combined\") | Ok(\"gated\") => C2R_GATED,",
        "        _ => C2R_GATED,",
        "    })",
        "}",
        "",
        "// ---- termination rung (docs/harness_oracle_plan.md, rung 1) -------------------------",
        "// The outcome vocabulary is FIXED and is the contract with the confirmation driver:",
        "//     normal | divergence | panic | signal | nonzero-exit | timeout",
        "// The last three are observed by the driver from the process result; the first three are",
        "// reported from here. `phase` says how far the execution got, which is what turns an",
        "// abort into an attribution: phase>=2 means C already returned normally, so a panic or a",
        "// crash at phase 3 is the translation diverging on TERMINATION, not a C-side failure.",
        "const C2R_PH_DECODE: u8 = 0; const C2R_PH_C: u8 = 1; const C2R_PH_C_DONE: u8 = 2;",
        "const C2R_PH_RUST: u8 = 3; const C2R_PH_RUST_DONE: u8 = 4; const C2R_PH_COMPARED: u8 = 5;",
        *(["// producer bridge: the step of the init -> target -> free sequence an outcome happened in",
           "const C2R_PH_PRODUCER: u8 = 6; const C2R_PH_FREE: u8 = 7;"] if _produced else []),
        "static C2R_PHASE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);",
        "fn c2r_phase(p: u8) { C2R_PHASE.store(p, std::sync::atomic::Ordering::Relaxed); }",
        "fn c2r_outcome_file() -> Option<&\'static str> {",
        "    static F: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();",
        "    F.get_or_init(|| std::env::var(\"C2R_OUTCOME_FILE\").ok()).as_deref()",
        "}",
        "fn c2r_outcome(kind: &str, detail: &str) {",
        "    let ph = C2R_PHASE.load(std::sync::atomic::Ordering::Relaxed);",
        "    let d: String = detail.chars().map(|c| if c == \'\\n\' { \' \' } else { c }).take(200).collect();",
        "    let line = format!(\"C2R_OUTCOME kind={kind} phase={ph} detail={d}\\n\");",
        "    // `normal` is never printed: in discovery that would be one write per execution.",
        "    if kind != \"normal\" { eprint!(\"{line}\"); }",
        "    if let Some(f) = c2r_outcome_file() {",
        "        use std::io::Write;",
        "        if let Ok(mut h) = std::fs::OpenOptions::new().create(true).append(true).open(f) {",
        "            let _ = h.write_all(line.as_bytes());",
        "        }",
        "    }",
        "}",
        "// libfuzzer-sys installs a panic hook that ABORTS before unwinding, so catch_unwind cannot",
        "// see a panic. Chaining a hook in front of it records the outcome and then lets libFuzzer",
        "// do exactly what it did before.",
        "fn c2r_install_panic_hook() {",
        "    static ONCE: std::sync::Once = std::sync::Once::new();",
        "    ONCE.call_once(|| {",
        "        let prev = std::panic::take_hook();",
        "        std::panic::set_hook(Box::new(move |info| {",
        "            c2r_outcome(\"panic\", &format!(\"{info}\"));",
        "            prev(info);",
        "        }));",
        "    });",
        "}",
        "// A divergence is the ORACLE, so by default it panics and libFuzzer records the finding.",
        "// In `coverage` mode the same comparison runs but a mismatch is only counted: a coverage",
        "// replay must not abort, or an artifact whose defect lies on the main path yields NO",
        "// coverage at all (observed on bzip2 x Laertes and x CROWN, whose compress corpora are",
        "// almost entirely divergence-triggering inputs).",
        "fn c2r_div(what: &str) {",
        "    if c2r_mode() == C2R_COVERAGE {",
        "        static N: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);",
        "        if N.fetch_add(1, std::sync::atomic::Ordering::Relaxed) == 0 {",
        "            eprintln!(\"c2r: divergence recorded, not panicking (coverage mode): {what}\");",
        "        }",
        "        return;",
        "    }",
        "    // Report and abort DETERMINISTICALLY rather than unwinding. A divergence is a terminal",
        "    // observation, so there is nothing to unwind to, and unwinding out of this helper was",
        "    // observed to hang the single-input replay path (libFuzzer\'s fork mode was unaffected,",
        "    // which is why the campaigns still recorded every event). SIGABRT is what libFuzzer\'s",
        "    // deadly-signal handler expects.",
        "    c2r_outcome(\"divergence\", what);",
        "    std::process::abort();",
        "}",
        "",
        *_interior_helper,
        *_extractor,
        "fn cd() -> i8 { 0 }  // silence unused on some shapes",
        "",
        f"use {crate} as translated;",
        "extern \"C\" {",
        f"    fn c_{entry}({extern_args}) {extern_ret};",
        *ub_externs,
        # the destructor may already be declared as the return contract's `free` (cJSON_Delete)
        *[l for l in _prod_externs if l not in ub_externs],
        "}",
        "",
        "fuzz_target!(|data: &[u8]| {",
        "    let _ = cd();",
        "    c2r_install_panic_hook();",
        "    c2r_phase(C2R_PH_DECODE);",
        "    let mut cur = Cur::new(data);",
        *decode,
        "    let _c2r_m = c2r_mode();",
        "    unsafe {",
        "        if _c2r_m == C2R_C_ONLY {",
        "            // confirmation phase A: C alone, so any sanitizer report is C's",
        *_ind(_prod_c), *_ind(_null_c),
        "            c2r_phase(C2R_PH_C);",
        *[l.replace("        ", "            ", 1)
          for l in body_call.split("\n") if l and "C hit UB -> reject" not in l],
        "            c2r_phase(C2R_PH_C_DONE);",
        *_ind(_free_c),
        "        } else if _c2r_m == C2R_RUST_ONLY {",
        "            // no C reference, so nothing can be compared; throughput bound only",
        *_ind(_prod_r), *_ind(_null_r),
        "            c2r_phase(C2R_PH_RUST);",
        rust_call.replace("        ", "            ", 1),
        "            c2r_phase(C2R_PH_RUST_DONE);",
        # Releasing what the boundary allocated is part of the CONTRACT, not part of the
        # comparison: without this, rust-only replay leaks every returned object and
        # LeakSanitizer fails the run, which is how the cJSON_Create* coverage replays died.
        *_rust_only_free,
        *_ind(_free_r),
        "        } else {",
        *_ind(_prod_c), *_ind(_prod_r), *_ind(_null_cmp),
        "            c2r_phase(C2R_PH_C);",
        *[l.replace("        ", "            ", 1) for l in body_call.split("\n") if l],
        "            c2r_phase(C2R_PH_C_DONE);",
        rust_call.replace("        ", "            ", 1).replace(
            "            ", "            c2r_phase(C2R_PH_RUST);\n            ", 1),
        "            c2r_phase(C2R_PH_RUST_DONE);",
        *[l.replace("        ", "            ", 1) for l in ret_cmp.split("\n") if l],
        *[l.replace("        ", "            ", 1) for l in post],
        *_ind(_obj_cmp_after),
        *_ind(_free_c), *_ind(_free_r),
        "        }",
        "    }",
        "    c2r_phase(C2R_PH_COMPARED);",
        "    c2r_outcome(\"normal\", \"\");",
        "});",
        "",
    ]).replace('panic!("divergence: ', 'c2r_div("')


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
    pat = rf'(?m)^(\s*)static(\s+[^\n;{{]*\b{re.escape(entry)}\s*(?:<[^>()]*>)?\s*\()'
    # every occurrence, not the first: a forward declaration (`static const char *parse_array(..);`
    # before the definition, as cJSON writes them) left the DEFINITION static, and C rejects a
    # static definition after a non-static declaration.
    new = re.sub(pat, r'\1\2', c_text)
    return new, (new != c_text)


_INT_TYPES = {"i8", "i16", "i32", "i64", "i128", "isize",
              "u8", "u16", "u32", "u64", "u128", "usize"}


def parse_rust_ret_type(rs_text: str, entry: str) -> str | None:
    """Return type spelling of `entry` in the translation (None if no `-> T`, i.e. unit)."""
    import re
    m = re.search(rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(entry)}\s*(?:<[^>()]*>)?\s*\([^;{{]*?\)\s*->\s*([^{{]+?)\s*\{{', rs_text, re.S)
    return m.group(1).strip() if m else None


def parse_rust_param_types(rs_text: str, entry: str) -> list[str]:
    """Extract the Rust translation's per-parameter TYPE strings for `entry`, in declaration order.

    For name-preserving idiomatic translations the params line up 1:1 with the C signature, so this
    lets the harness marshal each C-ABI value into the idiomatic Rust type the translation expects
    (e.g. C `const i32*` + len  ->  Rust `&Box<[i32]>` / `&[i32]` / `Vec<i32>`). Returns [] if the
    signature can't be found (caller falls back to the C-ABI form)."""
    import re
    m = re.search(rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(entry)}\s*(?:<[^>()]*>)?\s*\(([^;{{]*?)\)\s*(?:->|\{{)', rs_text, re.S)
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
    ap.add_argument("--c-source", default=None,
                    help="which .c in pair/source/ is the oracle translation unit (required when a "
                         "pair ships an amalgamation plus its siblings)")
    ap.add_argument("--schema", default=None,
                    help="explicit schema file (schemas/<program>.json is keyed to ONE entry; a "
                         "library needs one schema per boundary)")
    ap.add_argument("--plan", action="store_true",
                    help="HARNESS PLAN path (docs/harness_plan_architecture.md): derive the "
                         "InputPlan/ObservationPlan from the C AST + body, lower the resulting "
                         "HarnessPlan, and build from that. No schema file is read or written. "
                         "A boundary whose plan is incomplete is a harness-construction FAILURE.")
    ap.add_argument("--plan-json", default=None,
                    help="with --plan, also write the generated plan here for audit")
    ap.add_argument("--ignore-schema", action="store_true",
                    help="force inference even if schemas/<name>.json exists (harvesting non-entry boundaries)")
    ap.add_argument("--expose-entry", action="store_true",
                    help="make a private (static) entry callable by prepending #[no_mangle] pub")
    ap.add_argument("--rust-entry", default=None,
                    help="name of the matched function in the Rust translation when it differs from "
                    "the C entry (renamed translations); the harness calls translated::<rust-entry> "
                    "while the C oracle keeps c_<entry>. Defaults to <entry>.")
    ap.add_argument("--plugins", action="append", default=None,
                    help="comparator plugin manifest (plugins/<lib>/plugin.toml); repeatable. "
                         "A plugin extends OUTPUT comparison only and never touches the InputPlan; "
                         "a boundary whose return type it covers gets oracle_strength=structured-state.")
    ap.add_argument("--c-sanitize", "--c-asan", dest="c_sanitize", action="store_true",
                    help="CONFIRMATION build: compile the C oracle with -fsanitize=address,undefined "
                         "so BOTH a C-side memory error and value-level UB are detected. ASan alone "
                         "is not enough -- an out-of-range double->int cast is UB but not a memory "
                         "error, so an ASan-only replay reports C `clean` and a UB-associated "
                         "difference is misadjudicated as a translation defect. The Rust side "
                         "already links the ASan runtime, so both share one shadow map. Pair with "
                         "C2R_MODE=c-only to attribute the report to C.")
    ap.add_argument("--ub-free", action="store_true",
                    help="in-loop UB-free gate: UBSan-instrument the C oracle and reject "
                    "(skip comparison on) inputs where C hits UB, so divergences are reported "
                    "only on UB-free input (vs post-hoc per-artifact exclusion)")
    ap.add_argument("--rust-only", action="store_true",
                    help="E3 depth mode: emit a PURE-RUST harness (no C oracle, no build.rs, no "
                    "differential compare) that only drives translated::<entry>. For per-function "
                    "hit-depth measurement; correctness is E1's job.")
    args = ap.parse_args()

    pair = Path(args.pair)
    name = pair.name
    crate = name.replace("-", "_")
    cc = pair / "build"
    rs = next((pair / "translated").glob("*.rs"))
    # 2026-09-03: `next(glob(...))` is filesystem-order dependent. With a single-file pair that is
    # harmless, but a real library is given to the generator as an amalgamation plus its siblings,
    # and the arbitrary pick silently compiled the wrong translation unit as the oracle (observed:
    # `undefined symbol: c_mmed3`). Selection is now sorted, and --c-source names it explicitly.
    _cs = sorted((pair / "source").glob("*.c"))
    if args.c_source:
        _m = [c for c in _cs if c.name == args.c_source]
        if not _m:
            raise SystemExit(f"--c-source {args.c_source} not in {pair/'source'}: "
                             f"{[c.name for c in _cs]}")
        c_src = _m[0]
    else:
        if len(_cs) > 1:
            print(f"  note: {len(_cs)} .c files in the pair; using {_cs[0].name} "
                  f"(pass --c-source to choose)")
        c_src = _cs[0]

    # Eligibility is decided BEFORE anything is written: parameters, return value and comparator
    # must all match a supported contract template. A boundary that does not is reported here with
    # its reason, never generated and then left to fail at build time.
    if args.plan:
        import harness_plan as hp
        # The RustBridge needs the translated signature: a parameter shape the bridge cannot
        # reproduce losslessly is a construction failure, decided HERE rather than at build time.
        _rs_text = rs.read_text(encoding="utf-8", errors="replace")
        global _RUST_ALIASES, _PLUGINS_OK, _PLUGINS_DEGRADED
        _RUST_ALIASES = hp.rust_type_aliases(_rs_text)
        _PLUGINS_OK, _PLUGINS_DEGRADED = [], {}
        for _pl in load_plugins(args.plugins):
            _why = plugin_compat(_pl, _rs_text)
            if _why:
                _PLUGINS_DEGRADED[_pl.get("library") or _pl["c_type"]] = _why
                print(f"  plugin {_pl.get('library') or _pl['c_type']}: incompatible with this translation "
                      f"({_why}); the ladder degrades to pointer nullness")
            else:
                _PLUGINS_OK.append(_pl)
        _rt = parse_rust_param_types(_rs_text, args.rust_entry or args.entry)
        plan, lowered = hp.plan_and_lower(cc, args.entry, name, rust_types=(_rt or None),
                                          rust_aliases=hp.rust_type_aliases(_rs_text),
                                          rust_text=_rs_text)
        if args.plan_json:
            from dataclasses import asdict as _asdict
            Path(args.plan_json).write_text(json.dumps(_asdict(plan), indent=1) + "\n")
        if lowered is None:
            print(f"harness construction failed: {'; '.join(plan.failures)}")
            return 2
        params, ret, all_fns = parse_entry_signature(cc, args.entry, allow_nonpod=True)
        items, abi = items_from_schema(lowered), lowered["params"]
        seen = [q["name"] for q in abi]
        assert seen == [q["name"] for q in params], f"lowering lost ABI order: {seen}"
        print(f"  plan: {len(plan.inputs)} inputs; bridges "
              + ", ".join(f"{i['param']}={i['rust_bridge']}" for i in plan.inputs))
        # The return value is not a construction gate: the fixed ladder decides what about it is
        # comparable (void -> nothing, scalar -> value, pointer -> nullness, or a plugin).
        _rd = parse_entry_signature(cc, args.entry, with_return_desc=True, allow_nonpod=True)[3]
        verdict = {"supported": True,
                   "return_contract": return_contract(
                       _rd, ret, None, _plugins(args),
                       rust_sig_ret=_ptr_alias(parse_rust_ret_type(_rs_text, args.rust_entry or args.entry)))}
        if _PLUGINS_DEGRADED:
            verdict["plugin_degraded"] = dict(_PLUGINS_DEGRADED)
        _rc0 = verdict["return_contract"]
        if _rc0.get("template") is None:
            # A return the ladder cannot read is a construction failure, not a silent fall-through
            # to `c_ret != r_ret` (which, for two pointers, compares ADDRESSES and diverges always).
            print(f"harness construction failed: return -- {_rc0.get('reason')}")
            return 2
        # termination-only : nothing but how the two runs ended is comparable
        # partial(nullness) : a returned pointer, compared as NULL vs non-NULL only
        # observable-state  : return scalars plus every buffer the harness itself owns
        # structured-state  : observable-state, plus the object state a comparator plugin declares
        _strength = _rc0.get("oracle_strength") or (
            "termination-only" if _rc0["template"] == "void" and not plan.inputs
            else "observable-state")
        print(f"  oracle: return={_rc0['template']} (oracle_strength={_strength})")
    else:
        verdict = eligibility(name, cc, args.entry, infer=args.infer_schema,
                              schema_path=args.schema)
        if not verdict["supported"]:
            print(f"unsupported: {verdict['reason']}")
            return 2

        params, ret, all_fns, items, abi = resolve(
            name, cc, args.entry, infer=args.infer_schema, ignore_schema=args.ignore_schema,
            schema_path=args.schema)

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
                    # Normalise away explicit lifetimes before storing: safety lifters annotate the
                    # lifted types (`Option<&'a1 mut u32>`, `&'a mut [u8]`), and every downstream
                    # shape test here matches on the bare form, so an annotated type silently took
                    # the raw-pointer fallback and the harness failed to compile.
                    p["rust_pty"] = re.sub(r"&\s*'\w+\s*", "&", t)
                break
    if args.expose_entry:
        rs_text, _ = expose_entry(rs_text, rust_entry)
        if not args.rust_only:
            c_text, _ = strip_static_c(c_text, args.entry)
    (out / "src" / "lib.rs").write_text(rs_text, encoding="utf-8")

    if args.rust_only:
        # E3 depth: pure-Rust crate, NO C oracle, NO build.rs.
        (out / "Cargo.toml").write_text(
            f'[package]\nname = "{crate}"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\n',
            encoding="utf-8")
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
            gen_target_rust_only(args.entry, items, abi, ret, crate, rust_entry=rust_entry),
            encoding="utf-8")
        print(f"generated RUST-ONLY (E3 depth) harness at {out}")
        print(f"  entry: translated::{rust_entry}  abi roles: {[(p['name'], p['role']) for p in abi]}")
        return 0

    (out / "c" / c_src.name).write_text(c_text, encoding="utf-8")
    # Real libs split into .c + sibling .h (authored corpus is self-contained single-TU).
    # Copy the headers next to the .c so `#include "foo.h"` resolves from the harness c/ dir.
    for h in (pair / "source").glob("*.h"):
        (out / "c" / h.name).write_text(h.read_text(), encoding="utf-8")

    # A translation written against the `libc` crate (CROWN's genann: 890 uses) needs it as a
    # dependency of the harness crate the translation is copied into; one written against
    # std::os::raw (c2rust, Laertes, CROWN's bzip2) does not, and gets the same Cargo.toml as before.
    _needs_libc = bool(re.search(r"(?m)\blibc::|^\s*(?:pub\s+)?use\s+libc\b|extern\s+crate\s+libc\b", rs_text))
    (out / "Cargo.toml").write_text(
        f'[package]\nname = "{crate}"\nversion = "0.1.0"\nedition = "2021"\n\n'
        f'[build-dependencies]\ncc = "1"\n\n[dependencies]\n'
        + ('libc = "0.2"\n' if _needs_libc else ''), encoding="utf-8")

    # RQ4 FIX 2: all_fns now also carries "@var:<name>" entries for file-scope C globals.
    _rename = [f.removeprefix("@var:") for f in all_fns]
    defines = "\n".join(f'        .define("{fn}", "c_{fn}")' for fn in _rename)
    # --ub-free: instrument the oracle with UBSan (recover) and compile the flag shim in.
    if args.ub_free:
        (out / "c" / "ubshim.c").write_text(UBSHIM_C, encoding="utf-8")
    _rcv = verdict.get("return_contract") or {}
    _so_file = ""
    # a comparator plugin is linked when the RETURN is its type, or when a PRODUCED OBJECT is
    _pobj_pl = None
    for _q in abi:
        if _q.get("role") == "produced_object":
            _pobj_pl = _match_plugin({"inner": {"name": _q["struct"]}}, _plugins(args))
            if _pobj_pl:
                break
    if _rcv.get("template") == "comparator_plugin" or _pobj_pl is not None:
        _p = _rcv["plugin"] if _rcv.get("template") == "comparator_plugin" else _pobj_pl
        # The plugin's C half is compiled INTO the oracle, so the generator's function renaming
        # applies to it: a call to `cJSON_Delete` in the plugin becomes `c_cJSON_Delete`, i.e. the
        # C side's own function. That is what makes one plugin serve both sides.
        shutil.copy(_p["_c_source"], out / "c" / "c2r_plugin.c")
        # `header` names a header of the LIBRARY, not a file the plugin ships: the generator has
        # already copied every *.h from the pair's source into c/.
        if _p.get("header") and not (out / "c" / Path(_p["header"]).name).exists():
            raise SystemExit(f"plugin {_p['library']}: header {_p['header']} is not among the "
                             f"pair's headers, so the plugin's C half cannot compile")
        _so_file = '\n    build.file("c/c2r_plugin.c");'
        print(f"  comparator plugin: {_p['library']} for `{_p['c_type']}` "
              f"(oracle_strength=structured-state, max_bytes={_p['max_bytes']})")
    if _rcv.get("template") == "structured_object":
        (out / "c" / "c2r_extract.c").write_text(ctpl.c_extractor_source(_rcv), encoding="utf-8")
        _so_file = '\n    build.file("c/c2r_extract.c");'
        print(f"  structured_object: canonical extractors generated for {_rcv['type']} "
              f"({len(_rcv['fields'])} fields, child={_rcv['child']}, next={_rcv['next']})")
    _cflags = list(ub_sanitize_flags()) if args.ub_free else []
    if args.c_sanitize:
        # CONFIRMATION build: C-DEFINEDNESS checking, which is the adjudication oracle.
        #
        # ASan alone is not enough. An out-of-range double->int cast (cJSON.c:112,
        # `item->valueint = (int)n`) is UB in C but is NOT a memory error, so an ASan-only replay
        # reports C `clean` and the resulting value difference is misadjudicated as a translation
        # defect. Full UBSan is what catches it, `float-cast-overflow` explicitly.
        #
        # The in-loop UBSan-minimal instrumentation is dropped here: clang rejects
        # `-fsanitize-minimal-runtime` alongside `-fsanitize=address`, and the in-loop gate is a
        # DISCOVERY-side noise filter, not the adjudicator. ubshim.c is still compiled so the
        # harness's c2r_ub_* externs resolve; with no minimal instrumentation the gate never trips
        # and every input reaches the C call, which is exactly what a C-only replay wants.
        _cflags = ["-fsanitize=address,undefined",
                   "-fsanitize=float-cast-overflow,pointer-overflow,return,vla-bound",
                   "-fno-sanitize-recover=all"]
    ub_flags = "".join(f'\n        .flag("{f}")' for f in _cflags)
    ub_file = f'\n    build.file("c/ubshim.c");' if args.ub_free else ""
    (out / "build.rs").write_text(f'''fn main() {{
    let mut build = cc::Build::new();
    build.compiler("clang").flag("-O1").flag("-g")
        .flag("-fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp"){ub_flags}.warnings(false);
    build
{defines};
    build.file("c/{c_src.name}");{ub_file}{_so_file}
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
                   rust_ret=rust_ret, ret_contract=verdict.get("return_contract"),
                   plugins=_plugins(args)), encoding="utf-8")

    print(f"generated harness at {out}")
    print(f"  entry: {args.entry} -> {ret}")
    print(f"  abi roles: {[(p['name'], p['role']) for p in abi]}")
    if args.ub_free:
        print(f"  in-loop UB-free gate: ON (C UBSan-instrumented; UB inputs rejected; "
              f"clang major {clang_major()}, flags {ub_sanitize_flags()})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
