#!/usr/bin/env python3

"""Boundary-specific SEMANTIC-RISK features (libclang AST).

The generic complexity/signature features only weakly separate valid from invalid
(results/feature_analysis_v1.md). These features target the two negative MECHANISMS directly:

  intrinsic_ub risk  — does the function do UB-prone arithmetic on unconstrained input, and is it
                       guarded?  (div/mod, shift, signed reduction, negate; nonzero / width guards)
  isolation_invariant risk — does it index an array by a STRUCT FIELD / parameter that the caller
                       must keep in range, and is the index bounds-checked?  (field-as-array-index,
                       struct has index-like field, internal/static helper)

Token + AST based, deliberately simple and language-local (no whole-program dataflow). The point is
features that EXPLAIN validity, validated by their separation (AUC), not a perfect analysis.

Usage (per pair): imported by scripts/enrich_features.py; or
  python3 tools/stu_selector/risk_features.py --pair benchmark/pairs/safe_stats
"""

from __future__ import annotations

import os
from pathlib import Path

import callgraph as cgmod  # noqa: E402
import clang.cindex  # noqa: E402
from clang.cindex import CompilationDatabase, CursorKind, Index, TypeKind  # noqa: E402

# struct field names that typically hold an index / size the code then trusts as an array bound
_INDEX_FIELDS = ("len", "length", "cap", "capacity", "size", "count", "sp", "head", "tail",
                 "top", "pos", "idx", "index", "nbins", "gap_start", "gap_end", "cursor", "offset")
_CMP = ("<", ">", "<=", ">=", "==", "!=")
_COMPOUND = ("+=", "-=", "*=")


def _contains_member_or_param(node) -> bool:
    """True if the subtree references a struct field (MEMBER_REF) or a plain variable (DECL_REF) —
    i.e. the array index is data-dependent, not a constant/loop-local literal."""
    if node.kind in (CursorKind.MEMBER_REF_EXPR, CursorKind.DECL_REF_EXPR):
        return True
    return any(_contains_member_or_param(c) for c in node.get_children())


def _has_member_ref(node) -> bool:
    if node.kind == CursorKind.MEMBER_REF_EXPR:
        return True
    return any(_has_member_ref(c) for c in node.get_children())


def _has_kind(node, kind) -> bool:
    if node.kind == kind:
        return True
    return any(_has_kind(c, kind) for c in node.get_children())


def _risk(fn) -> dict:
    # Exclude COMMENT tokens — a comment like "/* b == 0 -> UB */" must not trip the guard checks.
    toks = [t.spelling for t in fn.get_tokens()
            if t.kind != clang.cindex.TokenKind.COMMENT]
    joined = " ".join(toks)

    # --- intrinsic-UB-risk (token-level operator presence) ---
    r_div_mod = toks.count("/") + toks.count("%")
    r_shift = toks.count("<<") + toks.count(">>")
    r_compound_arith = sum(toks.count(op) for op in _COMPOUND)   # signed reduction / accumulation
    r_compares = sum(toks.count(op) for op in _CMP)              # guard proxy
    r_nonzero_guard = int(("!= 0" in joined) or ("== 0" in joined) or ("!=0" in joined))
    r_width_guard = int(any(g in joined for g in ("< 32", "< 64", "& 31", "& 63", "% 32", "% 64"))
                        or "& 0x1f" in joined or "& 0x3f" in joined)
    # uses signed integer arithmetic at all (overflow is only UB for signed)
    r_signed = int(any(t in ("int", "int32_t", "int64_t", "long", "int16_t", "int8_t") for t in toks))

    # --- isolation-invariant-risk (AST) ---
    r_field_index = 0          # array subscript whose index references a struct field (s->buf[s->head])
    r_unmasked_field_index = 0 # ... AND the index is NOT masked/bounded (% or & ) => trusts the field
    r_datadep_index = 0        # array subscript whose index is data-dependent (field or variable)
    # --- intrinsic-UB-risk (AST; operator spelling, NOT compound-assign forms) ---
    # External libs surfaced two overflow forms the token-level compound set missed: a signed
    # multiply-accumulate written as `n = n*10 + d` (atoi) and unary negation of INT_MIN (`-a`, llabs).
    r_mul = 0                  # binary `*` (signed multiply can overflow)
    r_negate = 0               # unary `-` (negation of INT_MIN is UB)
    # input-bounding guard (used by the SELECTOR, not the risk score): a value compared against a
    # constant (clamp / range check, e.g. `pct < 0`, `pct > 100`). A cheap, interpretable proxy for
    # "this function constrains a value's domain" -> can shield a risky callee at a higher boundary.
    r_input_clamp = 0
    _CMP_OPS = ("<", ">", "<=", ">=")
    subscript_bases: set[str] = set()  # param/var names used as a subscript BASE (p[i]) -> written/read buffer

    def _binop_op(node):
        ch = list(node.get_children())
        if len(ch) != 2:
            return None
        end0 = ch[0].extent.end.offset
        for t in node.get_tokens():
            if t.extent.start.offset >= end0:
                return t.spelling
        return None

    def walk(node):
        nonlocal r_field_index, r_unmasked_field_index, r_datadep_index, r_mul, r_negate, r_input_clamp
        if node.kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            ch = list(node.get_children())
            if len(ch) >= 2:
                base, idx = ch[0], ch[1]
                btoks = [t.spelling for t in base.get_tokens()]
                if btoks:
                    subscript_bases.add(btoks[0])
                if _has_member_ref(idx):
                    r_field_index += 1
                    itoks = [t.spelling for t in idx.get_tokens()]
                    if "%" not in itoks and "&" not in itoks:  # not masked -> trusts the field
                        r_unmasked_field_index += 1
                if _contains_member_or_param(idx):
                    r_datadep_index += 1
        elif node.kind == CursorKind.BINARY_OPERATOR:
            op = _binop_op(node)
            if op == "*":
                r_mul += 1
            if op in _CMP_OPS:
                ch = list(node.get_children())
                if len(ch) == 2 and (
                    (_has_kind(ch[0], CursorKind.DECL_REF_EXPR) and _has_kind(ch[1], CursorKind.INTEGER_LITERAL))
                    or (_has_kind(ch[1], CursorKind.DECL_REF_EXPR) and _has_kind(ch[0], CursorKind.INTEGER_LITERAL))):
                    r_input_clamp += 1
        elif node.kind == CursorKind.UNARY_OPERATOR:
            t0 = next(iter(node.get_tokens()), None)
            if t0 is not None and t0.spelling == "-":  # prefix unary minus (deref `*p`/`++`/`--` excluded)
                r_negate += 1
        for c in node.get_children():
            walk(c)
    walk(fn)

    # output-size precondition (isolation sub-mechanism): a non-const pointer param that is written
    # through a subscript but is NOT followed by a size/capacity scalar -> caller must size it
    # (e.g. base64_encode's `out`; a well-formed `(dst, dst_cap)` does NOT fire).
    r_unsized_output = 0
    pargs = list(fn.get_arguments())
    for i, a in enumerate(pargs):
        t = a.type
        if t.kind != TypeKind.POINTER:
            continue
        pt = t.get_pointee()
        if pt.is_const_qualified():
            continue
        if a.spelling not in subscript_bases:
            continue
        nxt = pargs[i + 1] if i + 1 < len(pargs) else None
        nxt_is_size = bool(nxt) and any(k in (nxt.spelling or "").lower() for k in _INDEX_FIELDS)
        if not nxt_is_size:
            r_unsized_output = 1

    # struct-pointer parameter + whether its record has an index-like field
    r_struct_ptr = 0
    r_struct_index_field = 0
    for a in fn.get_arguments():
        t = a.type
        if t.kind == TypeKind.POINTER:
            pt = t.get_pointee().get_canonical()
            if pt.kind == TypeKind.RECORD:
                r_struct_ptr = 1
                decl = pt.get_declaration()
                fields = [f.spelling.lower() for f in decl.get_children()
                          if f.kind == CursorKind.FIELD_DECL]
                if any(any(k == fld or k in fld for k in _INDEX_FIELDS) for fld in fields):
                    r_struct_index_field = 1

    # static (internal) helper => relies on a caller-established invariant
    r_internal = int(fn.storage_class == clang.cindex.StorageClass.STATIC)

    return {
        "rf_div_mod": r_div_mod, "rf_shift": r_shift, "rf_compound_arith": r_compound_arith,
        "rf_mul": r_mul, "rf_negate": r_negate,
        "rf_compares": r_compares, "rf_nonzero_guard": r_nonzero_guard,
        "rf_width_guard": r_width_guard, "rf_signed": r_signed,
        "rf_field_index": r_field_index, "rf_unmasked_field_index": r_unmasked_field_index,
        "rf_datadep_index": r_datadep_index, "rf_unsized_output": r_unsized_output,
        "rf_input_clamp": r_input_clamp,
        "rf_struct_ptr": r_struct_ptr, "rf_struct_index_field": r_struct_index_field,
        "rf_internal": r_internal,
    }


def risk_features_for_pair(cc_dir: Path) -> dict[str, dict]:
    cgmod._configure_libclang()
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    out: dict[str, dict] = {}
    cwd0 = os.getcwd()
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
            if not f or f.startswith(("/usr/", "/lib/", "/opt/")) or not cur.spelling:
                continue
            if cur.spelling not in out:
                out[cur.spelling] = _risk(cur)
    return out


def main() -> int:
    import argparse
    import json
    ap = argparse.ArgumentParser()
    ap.add_argument("--pair", required=True)
    a = ap.parse_args()
    print(json.dumps(risk_features_for_pair(Path(a.pair) / "build"), indent=2))
    return 0


if __name__ == "__main__":
    import sys
    sys.path.insert(0, str(Path(__file__).resolve().parent))
    raise SystemExit(main())
