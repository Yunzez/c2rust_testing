#!/usr/bin/env python3
"""Harness Plan — the differential-harness generator's intermediate representation.

Binding design: `docs/harness_plan_architecture.md`.  Read it before changing anything here.

A Harness Plan is **generated, never written**.  Nobody hand-authors one and nobody patches one.
It is produced by one analysis over (C AST + function body):

    analyze_inputs(...)  -> InputPlan   how fuzz bytes become an input C accepts
    InputPlan            -> HarnessPlan (generator IR, like compiler IR)

There is no ObservationPlan.  What to compare is a FIXED comparison ladder owned by the code
emitters (termination -> scalar value -> pointer nullness -> known buffer contents -> user
plugin), identical for every boundary; see `docs/harness_oracle_plan.md`.  Memory safety is not
proved here either: confirmation replays every candidate with C under isolated ASan+UBSan, so the
index analysis below is a SIZING HEURISTIC, not a proof.

Rules this module implements (numbered as in the design doc):

  1. the plan is generated, never written                  -> there is no schema input here
  2. adapters are fixed generator code                     -> the plan only NAMES an adapter
  5. buffer bounds come from ONE global policy             -> `GeneratorPolicy`, below; there is
                                                              no per-boundary constant in this file
  7. a capacity is only as good as its source              -> never a caller's array declaration
  4. a boundary that cannot be planned FAILED HARNESS
     CONSTRUCTION                                          -> status="failed", with reasons

Every derived fact carries machine-generated `evidence` (file, line, column, source snippet, and
the name of the rule that fired) — acceptance item 7.

Usage:
  python3 tools/stu_selector/harness_plan.py --pair <pair_dir> --entry <fn> [--json out.json]
  python3 tools/stu_selector/harness_plan.py --pair <pair_dir> --all [--out-dir plans/]
"""

from __future__ import annotations

import argparse
import ctypes
import json
import os
import copy
import re
import sys
from dataclasses import dataclass, asdict, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import callgraph as cgmod          # noqa: E402
import gen_diff_harness as gdh     # noqa: E402
import clang.cindex                # noqa: E402
from clang.cindex import CursorKind, Index, TypeKind, BinaryOperator  # noqa: E402

PLAN_VERSION = 1


# ---------------------------------------------------------------------------
# THE global generator policy (rule 5).
#
# These are the ONLY constants in the harness-construction path.  They are properties of the
# generator, not of any library: no entry, no parameter and no library may introduce one of its
# own.  A per-boundary constant is a hand-written harness wearing a different hat.
# ---------------------------------------------------------------------------
@dataclass(frozen=True)
class GeneratorPolicy:
    # Largest number of BYTES the harness will ever hand to one buffer parameter.  Fuzz-decoded
    # buffer lengths are drawn from [0, max_buffer_bytes / elem_width].
    max_buffer_bytes: int = 1 << 20
    # Elements allocated for a pointer parameter whose required extent could NOT be derived.
    # The allocation makes the pointer valid and its extent known TO THE HARNESS; whether the
    # callee stays inside it is a separate, unproven obligation (see Safety).
    unproven_extent_elems: int = 4096
    # Clamp for a scalar parameter that only controls a loop trip count.  Purely a liveness
    # device: an unclamped i32 loop bound makes the target hang, which is not a divergence.
    max_trip: int = 1024
    # Rows in a pointer table (`char **strings` + count). One decoded byte picks the count, so
    # this must stay in [0, 255].
    max_table_rows: int = 16
    # Cap on every scalar handed to a PRODUCER (docs/producer_bridge_pilot.md section 4). The
    # producer's own allocation is an expression over its scalars (genann_init: weights ~
    # inputs*hidden + hidden*hidden*layers + ...), which the pilot does not prove against
    # max_buffer_bytes; one global cap keeps that allocation small (32 -> ~35k doubles) and, since
    # the target's array extents come from the same scalars, keeps those arrays small too.
    producer_scalar_max: int = 32

    def as_dict(self) -> dict:
        return asdict(self)


POLICY = GeneratorPolicy()


# ---------------------------------------------------------------------------
# Evidence
# ---------------------------------------------------------------------------
def _ev(rule: str, cur, note: str | None = None) -> dict:
    """Machine-generated evidence location for one derived fact."""
    loc = getattr(cur, "location", None)
    f = loc.file.name if (loc is not None and loc.file is not None) else None
    line = loc.line if loc is not None else 0
    col = loc.column if loc is not None else 0
    snippet = None
    if f and line:
        try:
            src = Path(f).read_text(encoding="utf-8", errors="replace").split("\n")
            if 0 < line <= len(src):
                snippet = src[line - 1].strip()[:160]
        except OSError:
            pass
    out = {"rule": rule, "file": (os.path.basename(f) if f else None),
           "line": line, "col": col, "snippet": snippet}
    if note:
        out["note"] = note
    return out


def _ev_policy(rule: str, field_name: str, note: str) -> dict:
    """Evidence for a fact that comes from the global policy rather than from the source."""
    return {"rule": rule, "file": "tools/stu_selector/harness_plan.py",
            "line": 0, "col": 0,
            "snippet": f"GeneratorPolicy.{field_name} = {getattr(POLICY, field_name)}",
            "note": note}


# ---------------------------------------------------------------------------
# Bound algebra.  An upper bound is one of
#   {"k":"const","v":n}
#   {"k":"param","p":name,"mul":m,"div":d,"add":a}   ==  floor(p*m/d) + a
#   {"k":"max","of":[...]}
#   {"k":"unknown","why":...}
# ---------------------------------------------------------------------------
def b_const(v: int) -> dict:
    return {"k": "const", "v": int(v)}


def b_param(p: str, mul: int = 1, div: int = 1, add: int = 0) -> dict:
    return {"k": "param", "p": p, "mul": mul, "div": div, "add": add}


def b_unknown(why: str) -> dict:
    return {"k": "unknown", "why": why}


def b_is_unknown(b: dict | None) -> bool:
    return b is None or b.get("k") == "unknown"


def b_add(b: dict | None, k: int) -> dict:
    if b_is_unknown(b):
        return b or b_unknown("?")
    if b["k"] == "const":
        return b_const(b["v"] + k)
    if b["k"] == "param":
        return b_param(b["p"], b["mul"], b["div"], b["add"] + k)
    if b["k"] == "max":
        return {"k": "max", "of": [b_add(x, k) for x in b["of"]]}
    return b_unknown("add")


def b_mulc(b: dict | None, m: int) -> dict:
    if b_is_unknown(b) or m <= 0:
        return b_unknown("mul")
    if b["k"] == "const":
        return b_const(b["v"] * m)
    if b["k"] == "param" and b["add"] == 0:
        return b_param(b["p"], b["mul"] * m, b["div"])
    if b["k"] == "max":
        return {"k": "max", "of": [b_mulc(x, m) for x in b["of"]]}
    return b_unknown("mul")


def b_divc(b: dict | None, d: int) -> dict:
    if b_is_unknown(b) or d <= 0:
        return b_unknown("div")
    if b["k"] == "const":
        return b_const(b["v"] // d)
    if b["k"] == "param" and b["add"] == 0:
        return b_param(b["p"], b["mul"], b["div"] * d)
    if b["k"] == "max":
        return {"k": "max", "of": [b_divc(x, d) for x in b["of"]]}
    return b_unknown("div")


def b_max(bs: list[dict]) -> dict:
    """Least upper bound of a set of bounds, with the obvious simplifications."""
    flat: list[dict] = []
    for b in bs:
        if b_is_unknown(b):
            return b
        flat.extend(b["of"] if b["k"] == "max" else [b])
    best_const: int | None = None
    byform: dict[tuple, int] = {}
    for b in flat:
        if b["k"] == "const":
            best_const = b["v"] if best_const is None else max(best_const, b["v"])
        else:
            key = (b["p"], b["mul"], b["div"])
            byform[key] = max(byform.get(key, b["add"]), b["add"])
    out = [b_param(p, m, d, a) for (p, m, d), a in sorted(byform.items())]
    if best_const is not None:
        out.append(b_const(best_const))
    if not out:
        return b_unknown("empty")
    return out[0] if len(out) == 1 else {"k": "max", "of": out}


def _nonneg(b: dict | None) -> bool:
    return b is not None and b["k"] == "const" and b["v"] >= 0


def b_render(b: dict | None) -> str:
    if b is None:
        return "?"
    if b["k"] == "const":
        return str(b["v"])
    if b["k"] == "unknown":
        return f"unknown({b.get('why', '')})"
    if b["k"] == "max":
        return "max(" + ", ".join(b_render(x) for x in b["of"]) + ")"
    s = b["p"]
    if b["mul"] != 1:
        s = f"{s}*{b['mul']}"
    if b["div"] != 1:
        s = f"{s}/{b['div']}"
    if b["add"]:
        s = f"{s}{b['add']:+d}"
    return s


def b_params(b: dict | None) -> set[str]:
    if b is None or b["k"] in ("const", "unknown"):
        return set()
    if b["k"] == "param":
        return {b["p"]}
    return set().union(*(b_params(x) for x in b["of"])) if b["of"] else set()


def b_subst(b: dict | None, mapping: dict) -> dict:
    """Rewrite a callee-relative bound into the caller's parameters: every callee parameter it
    mentions is replaced by the caller's argument bound (a caller parameter or a constant); a
    callee parameter whose argument is anything else makes the bound unknown."""
    if b_is_unknown(b):
        return b or b_unknown("?")
    if b["k"] == "const":
        return b
    if b["k"] == "param":
        m = mapping.get(b["p"])
        if m is None:
            return b_unknown(f"callee bound depends on {b['p']}, whose argument is not a boundary parameter")
        if m["k"] == "const":
            return b_const((m["v"] * b["mul"]) // b["div"] + b["add"])
        if b["mul"] == 1 and b["div"] == 1:
            return b_param(m["p"], m["mul"], m["div"], m["add"] + b["add"])
        if m["add"] == 0:
            return b_param(m["p"], m["mul"] * b["mul"], m["div"] * b["div"], b["add"])
        return b_unknown("callee bound with a scaled, offset argument")
    if b["k"] == "max":
        return b_max([b_subst(x, mapping) for x in b["of"]])
    return b_unknown("subst")


def b_eval_max(b: dict | None, caps: dict[str, int]) -> int | None:
    """Numeric upper bound, given a numeric cap for every parameter mentioned."""
    if b is None or b["k"] == "unknown":
        return None
    if b["k"] == "const":
        return b["v"]
    if b["k"] == "max":
        vs = [b_eval_max(x, caps) for x in b["of"]]
        return None if any(v is None for v in vs) else max(vs)
    if b["p"] not in caps:
        return None
    return (caps[b["p"]] * b["mul"]) // b["div"] + b["add"]


# ---------------------------------------------------------------------------
# libclang helpers
# ---------------------------------------------------------------------------
_PEEL = {CursorKind.UNEXPOSED_EXPR, CursorKind.PAREN_EXPR, CursorKind.CSTYLE_CAST_EXPR}


def _peel(cur):
    while cur is not None and cur.kind in _PEEL:
        ch = list(cur.get_children())
        if not ch:
            return cur
        cur = ch[-1] if cur.kind == CursorKind.CSTYLE_CAST_EXPR else ch[0]
    return cur


_EVAL_READY = False


def _eval_int(cur) -> int | None:
    """Constant-fold an expression with libclang's own evaluator.

    Token scraping is not enough: a macro-expanded literal (`BZ_MAX_CODE_LEN`) has an extent
    inside the expansion, so `get_tokens()` returns the macro name or nothing at all, and the
    index bound silently degrades to `unknown`.  `clang_Cursor_Evaluate` folds it properly.
    """
    global _EVAL_READY
    lib = clang.cindex.conf.lib
    if not _EVAL_READY:
        try:
            lib.clang_Cursor_Evaluate.argtypes = [clang.cindex.Cursor]
            lib.clang_Cursor_Evaluate.restype = ctypes.c_void_p
            lib.clang_EvalResult_getKind.argtypes = [ctypes.c_void_p]
            lib.clang_EvalResult_getKind.restype = ctypes.c_int
            lib.clang_EvalResult_getAsLongLong.argtypes = [ctypes.c_void_p]
            lib.clang_EvalResult_getAsLongLong.restype = ctypes.c_longlong
            lib.clang_EvalResult_dispose.argtypes = [ctypes.c_void_p]
            _EVAL_READY = True
        except Exception:
            return None
    try:
        r = lib.clang_Cursor_Evaluate(cur)
    except Exception:
        return None
    if not r:
        return None
    try:
        return lib.clang_EvalResult_getAsLongLong(r) if lib.clang_EvalResult_getKind(r) == 1 \
            else None
    finally:
        lib.clang_EvalResult_dispose(r)


def _int_literal(cur) -> int | None:
    c = _peel(cur)
    if c is None:
        return None
    if c.kind in (CursorKind.INTEGER_LITERAL, CursorKind.UNARY_OPERATOR,
                  CursorKind.BINARY_OPERATOR, CursorKind.PAREN_EXPR,
                  CursorKind.DECL_REF_EXPR, CursorKind.CHARACTER_LITERAL):
        v = _eval_int(c)
        if v is not None:
            # A DECL_REF_EXPR only folds when it names an enum constant or a const int; a
            # reference to a mutable variable must NOT be folded to its initialiser.
            if c.kind != CursorKind.DECL_REF_EXPR or _is_constant_decl(c):
                return v
    if c.kind == CursorKind.INTEGER_LITERAL:
        toks = [t.spelling for t in c.get_tokens()]
        if toks:
            try:
                return int(toks[0].rstrip("uUlL"), 0)
            except ValueError:
                return None
    if c.kind == CursorKind.UNARY_OPERATOR:
        toks = [t.spelling for t in c.get_tokens()]
        ch = list(c.get_children())
        if toks and toks[0] == "-" and ch:
            v = _int_literal(ch[0])
            return None if v is None else -v
    return None


def _is_constant_decl(cur) -> bool:
    r = cur.referenced
    if r is None:
        return False
    if r.kind == CursorKind.ENUM_CONSTANT_DECL:
        return True
    return r.kind == CursorKind.VAR_DECL and r.type is not None and r.type.is_const_qualified()


def _is_null(cur) -> bool:
    c = _peel(cur)
    if c is None:
        return False
    toks = [t.spelling for t in c.get_tokens()]
    if toks and toks[0] in ("NULL", "nullptr"):
        return True
    return _int_literal(c) == 0 and c.type is not None and c.type.kind == TypeKind.POINTER


def _ref_name(cur) -> tuple[str | None, object]:
    """(referenced declaration spelling, referenced cursor) for a DECL_REF_EXPR after peeling."""
    c = _peel(cur)
    if c is not None and c.kind == CursorKind.DECL_REF_EXPR:
        r = c.referenced
        return ((r.spelling if r is not None else None), r)
    return (None, None)


_SIGNED = {"i8": 7, "i16": 15, "i32": 31, "i64": 63}
_UNSIGNED = {"u8": 8, "u16": 16, "u32": 32, "u64": 64, "usize": 64}


def _type_max(t) -> int | None:
    """Largest value a scalar of this C type can hold (used for `a[tab[i]]` index bounds)."""
    d = gdh.describe_type(t)
    if d.get("kind") != "scalar":
        return None
    r = d["rust"]
    if r in _UNSIGNED:
        return (1 << _UNSIGNED[r]) - 1
    if r in _SIGNED:
        return (1 << _SIGNED[r]) - 1
    return None


# ---------------------------------------------------------------------------
# Body facts
# ---------------------------------------------------------------------------
@dataclass
class Subscript:
    base: str
    written: bool
    index_bound: dict
    index_lower: dict
    deps: list
    ev: dict


@dataclass
class BodyFacts:
    guards: dict = field(default_factory=dict)        # param -> {"min","max","evidence"}
    subscripts: list = field(default_factory=list)    # Subscript
    derefs: dict = field(default_factory=dict)        # param -> {"written": bool, "ev": ...}
    escapes: dict = field(default_factory=dict)       # param -> ev (passed to a call)
    escape_callees: dict = field(default_factory=dict)   # param -> {callee names}
    loop_bound_params: dict = field(default_factory=dict)   # param -> ev
    unresolved: list = field(default_factory=list)
    rows: dict = field(default_factory=dict)          # T** param -> {row k: pseudo pointer name}
    advanced: set = field(default_factory=set)        # pointers the body increments or reassigns


class BodyAnalyzer:
    """Derives, from the entry's own body: rejection guards, index bounds and loop bounds.

    Nothing here looks at a CALL SITE.  Rule 7: a caller's local array declaration is that
    caller's fact, never the boundary's contract.
    """

    # Callee summaries (2026-09-07). The boundary's body may hand a parameter to another function
    # of the same TU that does the dereferencing (`quickSort(arr, low, high)` reads `arr[high]`
    # only inside `partition`). Those accesses are the boundary's own obligations, so the callee's
    # facts about its parameter are carried back onto the caller's argument when the argument is a
    # plain boundary parameter or a constant -- bounded depth, no recursion, memoised per callee.
    # This is still "from the body": a call is a statement of the body. Rule 7 stands unchanged:
    # nothing here looks at a CALL SITE *of the boundary* (what callers pass in).
    CALLEE_DEPTH = 2

    def __init__(self, fn_cursor, param_names: set[str], table_params: set[str] | None = None,
                 depth: int = 0, call_chain: frozenset | None = None):
        self.fn = fn_cursor
        self.params = set(param_names)
        self.depth = depth
        self.call_chain = (call_chain or frozenset()) | {fn_cursor.spelling}
        # T** parameters whose rows may be named by locals (`input = inputs[0]`); each named row
        # becomes a pseudo pointer parameter `inputs__row0` with an extent of its own.
        self.tables = set(table_params or ())
        self.row_alias: dict[str, str] = {}
        self.assigns: dict[str, list[tuple[str, int]]] = {}   # local -> [(kind, loop_depth)]
        self.scope: list[dict] = []          # lexical stack of {loop var: (upper, lower)}
        self.deps: dict[str, set] = {}       # local -> names its value can depend on
        self.pass_no = 1
        self.facts = BodyFacts()
        self._memo: dict[str, dict] = {}
        self._active: set[str] = set()

    # -- entry point ------------------------------------------------------
    def run(self) -> BodyFacts:
        body = None
        for c in self.fn.get_children():
            if c.kind == CursorKind.COMPOUND_STMT:
                body = c
        if body is None:
            self.facts.unresolved.append("entry has no visible body")
            return self.facts
        self._collect_guards(body)
        # Pass 1 records every assignment, so pass 2 can tell a loop induction variable (bounded
        # by its own for-condition) from a variable the body also assigns for other reasons --
        # regardless of which comes first in the source.
        self.pass_no = 1
        self._scan(body, loop_depth=0, write_targets=set())
        self.pass_no = 2
        self.scope = []
        self._scan(body, loop_depth=0, write_targets=set())
        return self.facts

    # -- pointer tables ---------------------------------------------------
    def _row_of(self, expr):
        """(table param, k) when `expr` is `P[k]`: P a T** parameter, k a non-negative constant."""
        e = _peel(expr)
        if e is None or e.kind != CursorKind.ARRAY_SUBSCRIPT_EXPR:
            return None
        kids = list(e.get_children())
        if len(kids) != 2:
            return None
        base, _ = _ref_name(kids[0])
        k = _int_literal(kids[1])
        if base in self.tables and k is not None and k >= 0:
            return base, k
        return None

    def _row_pseudo(self, tab: str, k: int) -> str:
        name = f"{tab}__row{k}"
        self.facts.rows.setdefault(tab, {})[k] = name
        self.params.add(name)
        return name

    def _alias(self, name):
        return self.row_alias.get(name, name) if name else name

    def _note_pointer_write(self, lhs):
        """`p++`, `p += k`, `p = ...` on a pointer parameter or a row alias: its derefs and
        subscripts are relative to a moving base, so they do not bound its extent (pass 1)."""
        if self.pass_no != 1:
            return
        l = _peel(lhs)
        if l is None or l.kind != CursorKind.DECL_REF_EXPR:
            return
        n, r = _ref_name(l)
        if not n:
            return
        if n in self.row_alias:
            self.facts.advanced.add(self.row_alias[n])
        elif (n in self.params and r is not None and r.kind == CursorKind.PARM_DECL
              and r.type is not None and r.type.kind == TypeKind.POINTER):
            self.facts.advanced.add(n)

    # -- rejection guards -------------------------------------------------
    def _collect_guards(self, body):
        """Leading `if (C) return <pure>;` statements: the accepted domain is NOT C.

        Only statements before the first side effect are considered, so the guard really is a
        rejection guard and narrowing the domain cannot hide a divergence.
        """
        for st in body.get_children():
            if st.kind == CursorKind.DECL_STMT:
                if self._has_side_effect(st):
                    return
                continue
            if st.kind != CursorKind.IF_STMT:
                return
            ch = list(st.get_children())
            if len(ch) < 2:
                return
            cond, then = ch[0], ch[1]
            if len(ch) > 2:            # has an else -> not a plain rejection guard
                return
            if not self._is_pure_return(then) or self._has_side_effect(cond):
                return
            self._apply_negation(cond, st)

    def _is_pure_return(self, stmt) -> bool:
        s = stmt
        if s.kind == CursorKind.COMPOUND_STMT:
            kids = list(s.get_children())
            if len(kids) != 1:
                return False
            s = kids[0]
        if s.kind != CursorKind.RETURN_STMT:
            return False
        return not self._has_side_effect(s)

    def _has_side_effect(self, cur) -> bool:
        for n in cur.walk_preorder():
            if n.kind in (CursorKind.CALL_EXPR, CursorKind.COMPOUND_ASSIGNMENT_OPERATOR):
                return True
            if n.kind == CursorKind.UNARY_OPERATOR:
                toks = [t.spelling for t in n.get_tokens()]
                if "++" in toks or "--" in toks:
                    return True
            if n.kind == CursorKind.BINARY_OPERATOR and _binop(n) in _ASSIGN_OPS:
                return True
        return False

    def _apply_negation(self, cond, stmt):
        """accepted = NOT cond.  Handles the `A || B || ...` rejection form."""
        for d in _disjuncts(cond):
            self._negate_atom(d, stmt)

    def _negate_atom(self, atom, stmt):
        a = _peel(atom)
        if a is None:
            return
        op = _binop(a)
        if op == BinaryOperator.LAnd:
            # NOT (x != K1 && x != K2 ...)  ==  x in {K1, K2, ...}
            conj = _conjuncts(a)
            names, lits = set(), []
            for c in conj:
                cc = _peel(c)
                if _binop(cc) != BinaryOperator.NE:
                    return
                n, k = _cmp_param_literal(cc, self.params)
                if n is None or k is None:
                    return
                names.add(n)
                lits.append(k)
            if len(names) == 1 and lits and max(lits) - min(lits) == len(set(lits)) - 1:
                self._narrow(names.pop(), min(lits), max(lits), stmt,
                             "guard_negated_inequality_set")
            return
        if op not in _CMP_OPS:
            return
        name, lit = _cmp_param_literal(a, self.params)
        if name is None:
            return
        if lit is None:                       # `p == NULL` / `p != NULL`
            return
        # accepted domain is the negation of the rejecting comparison
        if op == BinaryOperator.LT:      self._narrow(name, lit, None, stmt, "guard_negated_lt")
        elif op == BinaryOperator.LE:    self._narrow(name, lit + 1, None, stmt, "guard_negated_le")
        elif op == BinaryOperator.GT:    self._narrow(name, None, lit, stmt, "guard_negated_gt")
        elif op == BinaryOperator.GE:    self._narrow(name, None, lit - 1, stmt, "guard_negated_ge")
        elif op == BinaryOperator.NE:    self._narrow(name, lit, lit, stmt, "guard_negated_ne")

    def _narrow(self, name, lo, hi, stmt, rule):
        g = self.facts.guards.setdefault(name, {"min": None, "max": None, "evidence": []})
        if lo is not None:
            g["min"] = lo if g["min"] is None else max(g["min"], lo)
        if hi is not None:
            g["max"] = hi if g["max"] is None else min(g["max"], hi)
        g["evidence"].append(_ev(rule, stmt,
                                 f"accepted domain of {name} narrowed to "
                                 f"[{g['min']}, {g['max']}] by the entry's own rejection guard"))

    # -- structural scan --------------------------------------------------
    def _scan(self, node, loop_depth: int, write_targets: set, in_for_control: bool = False):
        kind = node.kind
        kids = list(node.get_children())

        if kind == CursorKind.VAR_DECL and kids and self.pass_no == 1 and kids[-1].kind.is_expression():
            init = kids[-1]
            row = self._row_of(init)
            if row is not None:
                self.row_alias[node.spelling] = self._row_pseudo(*row)
            # `T v = <init>` makes v depend on every name in <init>, exactly like `v = <init>`
            names = set()
            for m in init.walk_preorder():
                if m.kind == CursorKind.DECL_REF_EXPR:
                    nm, mr = _ref_name(m)
                    if nm and (nm in self.params or (mr is not None and mr.kind == CursorKind.VAR_DECL)):
                        names.add(nm)
            self.deps.setdefault(node.spelling, set()).update(names)

        if kind == CursorKind.FOR_STMT:
            binding = self._for_stmt(node, loop_depth) if self.pass_no == 2 else {}
            self.scope.append(binding)
            try:
                for c in kids[:-1]:
                    self._scan(c, loop_depth + 1, write_targets, in_for_control=True)
                if kids:
                    self._scan(kids[-1], loop_depth + 1, write_targets)
            finally:
                self.scope.pop()
            return
        if kind in (CursorKind.WHILE_STMT, CursorKind.DO_STMT):
            for c in kids:
                self._scan(c, loop_depth + 1, write_targets)
            return

        if kind == CursorKind.BINARY_OPERATOR and _binop(node) in _ASSIGN_OPS and len(kids) == 2:
            self._record_dep(kids[0], kids[1])
            self._note_pointer_write(kids[0])
            self._record_write(kids[0], loop_depth, in_for_control)
            self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
            self._scan(kids[1], loop_depth, write_targets, in_for_control)
            return
        if kind == CursorKind.COMPOUND_ASSIGNMENT_OPERATOR and len(kids) == 2:
            self._record_dep(kids[0], kids[1])
            self._note_pointer_write(kids[0])
            self._record_write(kids[0], loop_depth, in_for_control)
            self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
            self._scan(kids[1], loop_depth, write_targets, in_for_control)
            return
        if kind == CursorKind.UNARY_OPERATOR and kids:
            toks = [t.spelling for t in node.get_tokens()]
            if "++" in toks or "--" in toks:
                self._note_pointer_write(kids[0])
                self._record_write(kids[0], loop_depth, in_for_control)
                self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
                return
            if toks and toks[0] == "*":
                self._record_deref(kids[0], _key(node) in write_targets, node)

        if kind == CursorKind.ARRAY_SUBSCRIPT_EXPR and len(kids) == 2 and self.pass_no == 2:
            base, idx = kids
            name, ref = _ref_name(base)
            name = self._alias(name)                 # `input[i]` with `input = inputs[0]`
            if name is None:
                row = self._row_of(base)             # `outputs[1][i]`: the row itself is the base
                if row is not None:
                    name = self._row_pseudo(*row)
            if name in self.params:
                bnd, _ = self._bound_of(idx, 0)
                low = self._lower_of(idx, 0)
                self.facts.subscripts.append(
                    Subscript(base=name, written=_key(node) in write_targets,
                              index_bound=bnd, index_lower=low,
                              deps=sorted(self._deps_of(idx)),
                              ev=_ev("index_bound_from_body", node,
                                     f"{name}[{b_render(bnd)}] "
                                     f"{'written' if _key(node) in write_targets else 'read'}")))

        if kind == CursorKind.CALL_EXPR and self.pass_no == 2:
            callee = node.spelling or ""
            for a in kids:
                n, _ = _ref_name(a)
                if n in self.params:
                    self.facts.escapes.setdefault(
                        n, _ev("param_escapes_into_call", node,
                               f"{n} is passed to {callee or 'another function'}"))
                    self.facts.escape_callees.setdefault(n, set()).add(callee)
            self._absorb_callee(node, callee)

        for c in kids:
            self._scan(c, loop_depth, write_targets, in_for_control)

    def _callee_definition(self, callee: str):
        if not callee:
            return None
        tu = self.fn.translation_unit
        key = str(tu.spelling)
        idx = _TU_DEFS.get(key)
        if idx is None:                     # one walk per TU, not one per call expression
            idx = {}
            for cur in tu.cursor.walk_preorder():
                if cur.kind == CursorKind.FUNCTION_DECL and cur.is_definition():
                    f = cur.location.file.name if cur.location and cur.location.file else ""
                    if f and not f.startswith(("/usr/", "/lib/")) and cur.spelling not in idx:
                        idx[cur.spelling] = cur
            _TU_DEFS[key] = idx
        return idx.get(callee)

    def _absorb_callee(self, node, callee: str):
        """Carry the callee's facts about its parameters onto the arguments the boundary passes."""
        if self.depth >= self.CALLEE_DEPTH or not callee or callee in self.call_chain:
            return
        args = list(node.get_arguments())
        mapping: dict[str, dict] = {}       # callee param -> bound of the caller's argument
        arg_name: dict[str, str] = {}       # callee param -> caller parameter name
        cur = self._callee_definition(callee)
        if cur is None:
            return
        cparams = [a.spelling for a in cur.get_arguments()]
        if len(cparams) != len(args):
            return
        for q, a in zip(cparams, args):
            n, _ = _ref_name(a)
            if n in self.params:
                mapping[q] = b_param(n)
                arg_name[q] = n
            else:
                lit = _int_literal(a)
                if lit is not None:
                    mapping[q] = b_const(lit)
        if not arg_name:
            return
        key = (str(self.fn.translation_unit.spelling), callee, self.depth + 1)
        cf = _CALLEE_FACTS.get(key)
        if cf is None:
            try:
                cf = BodyAnalyzer(cur, set(cparams), depth=self.depth + 1,
                                  call_chain=self.call_chain).run()
            except Exception as e:   # a callee the analyser cannot read is no fact, not a crash
                cf = BodyFacts(unresolved=[f"callee {callee}: {e}"])
            _CALLEE_FACTS[key] = cf
        for q, n in arg_name.items():
            d = cf.derefs.get(q)
            if d:
                mine = self.facts.derefs.setdefault(n, {"written": False, "read": False, "ev": None})
                mine["written"] = mine["written"] or d["written"]
                mine["read"] = mine["read"] or d["read"]
                if mine["ev"] is None:
                    mine["ev"] = _ev("pointer_dereferenced_in_callee", node,
                                     f"{n} is passed to {callee}({q}), which "
                                     f"{'writes' if d['written'] else 'reads'} *{q}")
            for s in cf.subscripts:
                if s.base != q:
                    continue
                self.facts.subscripts.append(Subscript(
                    base=n, written=s.written,
                    index_bound=b_subst(s.index_bound, mapping),
                    index_lower=b_subst(s.index_lower, mapping),
                    deps=sorted({arg_name[x] for x in s.deps if x in arg_name}),
                    ev=_ev("index_bound_via_callee", node,
                           f"{n} is passed to {callee}({q}), which "
                           f"{'writes' if s.written else 'reads'} {q}[{b_render(s.index_bound)}] "
                           f"(bound rewritten to the boundary's parameters)")))
            if q in cf.loop_bound_params:
                self.facts.loop_bound_params.setdefault(
                    n, _ev("loop_trip_count_controlled_by_param_via_callee", node,
                           f"{n} is passed to {callee}({q}), where {q} controls a loop"))
            if q in cf.advanced:
                self.facts.advanced.add(n)
            for c2 in cf.escape_callees.get(q, ()):
                self.facts.escape_callees.setdefault(n, set()).add(c2)
            g = cf.guards.get(q)
            if g and n not in self.facts.guards:
                gg = dict(g)
                gg["evidence"] = list(g.get("evidence", [])) + [
                    _ev("rejection_guard_via_callee", node,
                        f"{n} is passed to {callee}({q}), which rejects it outside this range")]
                self.facts.guards[n] = gg

    def _record_dep(self, lhs, rhs):
        """`v = <expr>` makes v depend on every name in <expr> (pass 1 only)."""
        if self.pass_no != 1:
            return
        l = _peel(lhs)
        if l is None:
            return
        if l.kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            lk = list(l.get_children())
            if not lk:
                return
            l = _peel(lk[0])          # `arr[i] = e` taints arr with everything in e and in i
        if l is None or l.kind != CursorKind.DECL_REF_EXPR:
            return
        n, r = _ref_name(l)
        if not n or r is None or r.kind != CursorKind.VAR_DECL:
            return
        names = set()
        for m in rhs.walk_preorder():
            if m.kind == CursorKind.DECL_REF_EXPR:
                nm, mr = _ref_name(m)
                if nm and (nm in self.params or (mr is not None
                                                 and mr.kind == CursorKind.VAR_DECL)):
                    names.add(nm)
        self.deps.setdefault(n, set()).update(names)

    def _deps_of(self, expr) -> set:
        """Parameters an index expression can transitively depend on."""
        seed = set()
        for m in expr.walk_preorder():
            if m.kind == CursorKind.DECL_REF_EXPR:
                nm, _ = _ref_name(m)
                if nm:
                    seed.add(nm)
        seen, work, out = set(), list(seed), set()
        while work:
            n = work.pop()
            if n in seen:
                continue
            seen.add(n)
            if n in self.params:
                out.add(n)
            work.extend(self.deps.get(n, ()))
        return out

    def _record_write(self, lhs, loop_depth, in_for_control: bool = False):
        l = _peel(lhs)
        if l is None or self.pass_no != 1:
            if l is not None and self.pass_no == 2 and l.kind == CursorKind.UNARY_OPERATOR:
                kids = list(l.get_children())
                toks = [tk.spelling for tk in l.get_tokens()]
                if toks and toks[0] == "*" and kids:
                    self._record_deref(kids[0], True, l)
            return
        if l.kind == CursorKind.DECL_REF_EXPR:
            n, r = _ref_name(l)
            if n and r is not None and r.kind == CursorKind.VAR_DECL:
                self.assigns.setdefault(n, []).append(
                    ("loopvar" if in_for_control else "plain", loop_depth))
        elif l.kind == CursorKind.UNARY_OPERATOR:
            kids = list(l.get_children())
            toks = [t.spelling for t in l.get_tokens()]
            if toks and toks[0] == "*" and kids:
                self._record_deref(kids[0], True, l)

    def _record_deref(self, target, written: bool, node):
        if self.pass_no != 2:
            return
        t = _peel(target)
        if t is not None and t.kind == CursorKind.UNARY_OPERATOR:        # `*p++`, `*--p`
            toks = [tk.spelling for tk in t.get_tokens()]
            ch = list(t.get_children())
            if ("++" in toks or "--" in toks) and ch:
                target = ch[0]
        n, _ = _ref_name(target)
        n = self._alias(n)
        if n in self.params:
            d = self.facts.derefs.setdefault(n, {"written": False, "read": False, "ev": None})
            d["written"] = d["written"] or written
            d["read"] = d["read"] or not written
            if d["ev"] is None:
                d["ev"] = _ev("pointer_dereferenced_in_body", node,
                              f"*{n} is {'written' if written else 'read'}")

    def _for_stmt(self, node, loop_depth) -> dict:
        """Bindings contributed by `for (v = INIT; v < LIMIT; v++)`, valid only inside this loop."""
        binding: dict = {}
        kids = list(node.get_children())
        ctrl = kids[:-1]
        # Every parameter mentioned in a loop's control expressions is a trip-count control,
        # whether or not an induction bound could be derived from it (liveness, rule: policy clamp).
        for c2 in ctrl:
            for pp in _params_in(c2, self.params):
                self.facts.loop_bound_params.setdefault(
                    pp, _ev("loop_trip_count_controlled_by_param", node,
                            f"{pp} appears in this loop's control expressions"))
        for c in ctrl:
            for cc in _conjuncts(c):
                if cc is None or cc.kind != CursorKind.BINARY_OPERATOR:
                    continue
                op = _binop(cc)
                if op not in (BinaryOperator.LT, BinaryOperator.LE):
                    continue
                ck = list(cc.get_children())
                if len(ck) != 2:
                    continue
                n, r = _ref_name(ck[0])
                if not n or r is None or r.kind != CursorKind.VAR_DECL:
                    continue
                if any(k == "plain" for k, _ in self.assigns.get(n, [])):
                    continue      # the body assigns it for other reasons: not an induction var
                up, _ = self._bound_of(ck[1], 0)
                if op == BinaryOperator.LT:
                    up = b_add(up, -1)
                low = b_unknown(f"no for-init for {n}")
                for c2 in ctrl:
                    c2p = _peel(c2)
                    if (c2p is not None and c2p.kind == CursorKind.BINARY_OPERATOR
                            and _binop(c2p) == BinaryOperator.Assign):
                        ik = list(c2p.get_children())
                        if len(ik) == 2 and _ref_name(ik[0])[0] == n:
                            low = self._lower_of(ik[1], 0)
                binding[n] = (up, low)
        return binding

    # -- bound evaluation -------------------------------------------------
    def _bound_of(self, expr, depth: int) -> tuple[dict, bool]:
        """(upper bound, index_may_be_negative)."""
        if depth > 12:
            return b_unknown("expression too deep"), True
        e = _peel(expr)
        if e is None:
            return b_unknown("empty"), True

        lit = _int_literal(e)
        if lit is not None:
            return b_const(lit), lit < 0

        if e.kind == CursorKind.DECL_REF_EXPR:
            n, r = _ref_name(e)
            if n in self.params:
                signed = (gdh.describe_type(e.type).get("rust", "") in _SIGNED)
                return b_param(n), signed
            if r is not None and r.kind == CursorKind.VAR_DECL:
                return self._scoped(n, 0), False
            return b_unknown(f"reference to {n}"), True

        if e.kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            # `a[tab[i]]`: the index is a loaded value, bounded by its own type.
            m = _type_max(e.type)
            return (b_const(m), gdh.describe_type(e.type).get("rust", "") in _SIGNED) \
                if m is not None else (b_unknown("loaded index of non-scalar type"), True)

        if e.kind == CursorKind.BINARY_OPERATOR:
            kids = list(e.get_children())
            if len(kids) != 2:
                return b_unknown("binary arity"), True
            op = _binop(e)
            lb, lneg = self._bound_of(kids[0], depth + 1)
            rlit = _int_literal(kids[1])
            llit = _int_literal(kids[0])
            if op == BinaryOperator.Add:
                if rlit is not None:
                    return b_add(lb, rlit), lneg
                if llit is not None:
                    rb, rneg = self._bound_of(kids[1], depth + 1)
                    return b_add(rb, llit), rneg
                return b_unknown("sum of two non-constants"), True
            if op == BinaryOperator.Sub and rlit is not None:
                return b_add(lb, -rlit), True
            if op == BinaryOperator.Mul and rlit is not None and rlit > 0:
                return b_mulc(lb, rlit), lneg
            if op == BinaryOperator.Div and rlit is not None and rlit > 0:
                return b_divc(lb, rlit), lneg
            if op == BinaryOperator.Shr and rlit is not None and rlit >= 0:
                return b_divc(lb, 1 << rlit), lneg
            if op == BinaryOperator.Shl and rlit is not None and rlit >= 0:
                return b_mulc(lb, 1 << rlit), lneg
            if op == BinaryOperator.And and rlit is not None and rlit >= 0:
                return b_const(rlit), False
            if op == BinaryOperator.Rem and rlit is not None and rlit > 0:
                return b_const(rlit - 1), True
            return b_unknown(f"operator {op}"), True

        if e.kind == CursorKind.CONDITIONAL_OPERATOR:
            kids = list(e.get_children())
            if len(kids) == 3:
                a, an = self._bound_of(kids[1], depth + 1)
                b, bn = self._bound_of(kids[2], depth + 1)
                return b_max([a, b]), an or bn

        return b_unknown(f"{e.kind.name.lower()}"), True

    def _lower_of(self, expr, depth: int) -> dict:
        """Lower bound of an index expression (mirror of _bound_of; used for the >= 0 obligation)."""
        if depth > 12:
            return b_unknown("expression too deep")
        e = _peel(expr)
        if e is None:
            return b_unknown("empty")
        lit = _int_literal(e)
        if lit is not None:
            return b_const(lit)
        if e.kind == CursorKind.DECL_REF_EXPR:
            n, r = _ref_name(e)
            if n in self.params:
                return b_param(n)
            if r is not None and r.kind == CursorKind.VAR_DECL:
                return self._scoped(n, 1)
            return b_unknown(f"lower bound of {n}")
        if e.kind == CursorKind.ARRAY_SUBSCRIPT_EXPR:
            d = gdh.describe_type(e.type)
            if d.get("kind") == "scalar":
                r = d["rust"]
                if r in _UNSIGNED:
                    return b_const(0)
                if r in _SIGNED:
                    return b_const(-(1 << _SIGNED[r]) - 1)
            return b_unknown("loaded index")
        if e.kind == CursorKind.BINARY_OPERATOR:
            kids = list(e.get_children())
            if len(kids) == 2:
                op = _binop(e)
                rlit = _int_literal(kids[1])
                lo = self._lower_of(kids[0], depth + 1)
                if op == BinaryOperator.Add and rlit is not None:
                    return b_add(lo, rlit)
                if op == BinaryOperator.Sub and rlit is not None:
                    return b_add(lo, -rlit)
                if op in (BinaryOperator.Shr, BinaryOperator.Div, BinaryOperator.Rem,
                          BinaryOperator.And) and rlit is not None and rlit > 0:
                    return b_const(0) if not b_is_unknown(lo) and _nonneg(lo) else b_unknown("shift")
        return b_unknown(f"{e.kind.name.lower()}")

    def _scoped(self, name: str, which: int) -> dict:
        """Innermost lexical bound for a loop induction variable (0 = upper, 1 = lower)."""
        for frame in reversed(self.scope):
            if name in frame:
                return frame[name][which]
        return b_unknown(f"{name} is not a loop induction variable in scope")


_CMP_OPS = {BinaryOperator.LT, BinaryOperator.LE, BinaryOperator.GT,
            BinaryOperator.GE, BinaryOperator.EQ, BinaryOperator.NE}
_ASSIGN_OPS = {BinaryOperator.Assign, BinaryOperator.AddAssign, BinaryOperator.SubAssign,
               BinaryOperator.MulAssign, BinaryOperator.DivAssign, BinaryOperator.RemAssign,
               BinaryOperator.AndAssign, BinaryOperator.OrAssign, BinaryOperator.XorAssign,
               BinaryOperator.ShlAssign, BinaryOperator.ShrAssign}


def _binop(cur):
    try:
        return cur.binary_operator
    except Exception:
        return BinaryOperator.Invalid


def _key(cur) -> tuple:
    c = _peel(cur)
    e = c.extent
    return (e.start.offset, e.end.offset, c.kind)


def _disjuncts(cur) -> list:
    c = _peel(cur)
    if c is not None and c.kind == CursorKind.BINARY_OPERATOR and _binop(c) == BinaryOperator.LOr:
        out = []
        for k in c.get_children():
            out.extend(_disjuncts(k))
        return out
    return [c]


def _conjuncts(cur) -> list:
    c = _peel(cur)
    if c is not None and c.kind == CursorKind.BINARY_OPERATOR and _binop(c) == BinaryOperator.LAnd:
        out = []
        for k in c.get_children():
            out.extend(_conjuncts(k))
        return out
    return [c]


def _cmp_param_literal(cur, params: set[str]) -> tuple[str | None, int | None]:
    """For `p OP K` (or `K OP p`, normalised away) return (param, literal)."""
    kids = list(cur.get_children())
    if len(kids) != 2:
        return (None, None)
    ln, _ = _ref_name(kids[0])
    rn, _ = _ref_name(kids[1])
    if ln in params:
        if _is_null(kids[1]):
            return (ln, None)
        return (ln, _int_literal(kids[1]))
    if rn in params:
        return (None, None)      # reversed form: not normalised, so not claimed
    return (None, None)


def _params_in(cur, params: set[str]) -> set[str]:
    out = set()
    if cur is None:
        return out
    for n in cur.walk_preorder():
        if n.kind == CursorKind.DECL_REF_EXPR:
            nm, _ = _ref_name(n)
            if nm in params:
                out.add(nm)
    return out


# ---------------------------------------------------------------------------
# Signature + body acquisition
# ---------------------------------------------------------------------------
def entry_cursor(cc_dir: Path, entry: str):
    """The definition cursor of `entry`, from the pair's own compilation database."""
    hit = gdh.definition_index(cc_dir).get(entry)
    return hit if hit else (None, None)      # the index keeps every TU alive: cursors borrow from it


_EFFECTFUL_CACHE: dict[str, set] = {}


def effectful_functions(cc_dir: Path) -> set:
    """Functions in this translation unit that transitively reach an effectful call.

    The effect is what matters, and it is rarely in the entry's own body: `BZ2_bzopen` does not
    call `fopen`, it calls `bzopen_or_bzdopen`, which does. Without the transitive step the rule
    would catch the private helper and wave the two public entry points through.
    """
    return _reaching_functions(cc_dir, EFFECTFUL_CALLS)


# Randomness a producer may reach (docs/producer_bridge_pilot.md section 3): the same fixpoint
# finds `genann_init -> genann_randomize -> rand`, which a body-only scan misses.
RANDOM_CALLS = {"rand", "random", "drand48", "lrand48", "mrand48", "rand_r", "srand", "srandom"}


def random_functions(cc_dir: Path) -> set:
    return _reaching_functions(cc_dir, RANDOM_CALLS)


def freeing_functions(cc_dir: Path) -> set:
    """Functions that transitively call free(): the destructor candidates of section 2."""
    return _reaching_functions(cc_dir, {"free"})


_CALLGRAPH_CACHE: dict = {}


def callgraph(cc_dir: Path) -> dict:
    """{function: set(callees)} over every definition in the pair's translation unit(s), parsed once."""
    key = str(cc_dir)
    if key not in _CALLGRAPH_CACHE:
        _reaching_functions(cc_dir, set())          # populates the cache as a side effect
    return _CALLGRAPH_CACHE[key]


def reachable_functions(cc_dir: Path, f: str) -> set:
    """Transitive callees of `f` inside the TU: a derivable proxy for how much of the library a
    producer's object can carry (cJSON_Parse reaches the whole parser; cJSON_CreateString reaches two
    helpers). Used only to ORDER producer candidates."""
    edges = callgraph(cc_dir)
    seen, todo = set(), [f]
    while todo:
        g = todo.pop()
        for h in edges.get(g, ()):
            if h in edges and h not in seen:
                seen.add(h)
                todo.append(h)
    return seen


def _reaching_functions(cc_dir: Path, sources: set) -> set:
    key = (str(cc_dir), tuple(sorted(sources)))
    if key in _EFFECTFUL_CACHE:
        return _EFFECTFUL_CACHE[key]
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    cwd0 = os.getcwd()
    edges: dict[str, set] = {}
    for cmd in cdb.getAllCompileCommands():
        src_abs = str((Path(cmd.directory) / cmd.filename).resolve())
        args = cgmod._filter_compile_args(list(cmd.arguments),
                                          {cmd.filename, src_abs, Path(cmd.filename).name})
        os.chdir(cmd.directory if Path(cmd.directory).exists() else cc_dir)
        try:
            tu = index.parse(src_abs, args=args)
        finally:
            os.chdir(cwd0)
        for cur in tu.cursor.walk_preorder():
            # A file-scope function pointer initialised to one of the sources is that source under
            # another name: cJSON's `static void (*cJSON_free)(void *) = free;` makes every
            # `cJSON_free(p)` a call to free(). Aliases live in the same edge map as functions.
            if cur.kind == CursorKind.VAR_DECL and cur.semantic_parent is not None \
                    and cur.semantic_parent.kind == CursorKind.TRANSLATION_UNIT:
                refs = {n.spelling for n in cur.walk_preorder()
                        if n.kind == CursorKind.DECL_REF_EXPR and n.spelling}
                if refs:
                    edges.setdefault(cur.spelling, set()).update(refs)
                continue
            if cur.kind != CursorKind.FUNCTION_DECL or not cur.is_definition():
                continue
            callees = edges.setdefault(cur.spelling, set())
            for n in cur.walk_preorder():
                if n.kind == CursorKind.CALL_EXPR and n.spelling:
                    callees.add(n.spelling)
    _CALLGRAPH_CACHE[str(cc_dir)] = edges
    eff = set(sources)
    changed = True
    while changed:                      # fixpoint over the call graph
        changed = False
        for f, cs in edges.items():
            if f not in eff and (cs & eff):
                eff.add(f)
                changed = True
    _EFFECTFUL_CACHE[key] = eff
    return eff


# ---------------------------------------------------------------------------
# RustBridge
#
# One C-shaped canonical input, materialized twice: once as C arguments, once as Rust arguments.
# The bridge is the second materialization.  It is a CLOSED set -- a Rust parameter shape with no
# bridge is a harness-construction failure, because an input that cannot be reproduced losslessly
# on the Rust side is not the same logical input.
#
# Type category, width and signedness are checked HERE, inside the bridge.  They are not a
# comparison stage and not a reported result: either the C-shaped value survives the trip or the
# boundary does not run.
# ---------------------------------------------------------------------------
_INT_RUST = set(_SIGNED) | set(_UNSIGNED)
_FLOAT_RUST = {"f32", "f64"}

# A translator writes C ABI types as aliases, so the bridge has to resolve them before it can
# check anything: c2rust emits `Int32`, which is `std::os::raw::c_int`, which is `i32`.
_C_ABI_ALIASES = {
    "c_char": "i8", "c_schar": "i8", "c_uchar": "u8",
    "c_short": "i16", "c_ushort": "u16", "c_int": "i32", "c_uint": "u32",
    "c_long": "i64", "c_ulong": "u64", "c_longlong": "i64", "c_ulonglong": "u64",
    "c_float": "f32", "c_double": "f64", "size_t": "usize", "ssize_t": "isize",
}
# A translation may write a GLOBAL path (`::core::ffi::c_int`), so the leading `::` has to go too
# or the resolved leaf comes out as `::i32` and matches nothing.
_GLOBAL_COLONS = re.compile(r"(?<![A-Za-z0-9_>])::")
_ALIAS_PATH = re.compile(r"\b(?:std::os::raw|core::ffi|std::ffi|libc)::")
_IDENT = re.compile(r"\b[A-Za-z_][A-Za-z0-9_]*\b")


def rust_type_aliases(rs_text: str) -> dict:
    """`pub type Int32 = std::os::raw::c_int;` chains, resolved to primitives.

    A multi-module translation also re-exports its own aliases (`pub type Int32 =
    crate::blocksort::Int32;`), so module paths are stripped and a self-referential re-export is
    skipped rather than overwriting the real definition.
    """
    raw: dict[str, str] = {}
    for name, rhs in re.findall(r"(?m)^\s*pub\s+type\s+([A-Za-z_]\w*)\s*=\s*([^;]+);",
                                rs_text or ""):
        leaf = _resolve_leaf(rhs)
        if leaf == name or name in raw:
            continue                    # a re-export of itself, or a later duplicate
        raw[name] = leaf
    out = {}
    for name in raw:
        seen, cur = set(), name
        for _ in range(8):
            if cur in _INT_RUST or cur in _FLOAT_RUST or cur == "bool":
                break
            nxt = raw.get(cur)
            if nxt is None or cur in seen:
                break
            seen.add(cur)
            cur = _resolve_leaf(nxt)
        if cur in _INT_RUST or cur in _FLOAT_RUST or cur == "bool":
            out[name] = cur
    # Pointer typedefs: c2rust keeps `pub type lil_t = *mut _lil_t;` where Laertes writes the
    # pointer out at every use. Same shape, one spelling -- resolve the alias so a producer that
    # "returns lil_t" is recognised as returning a raw pointer to _lil_t. Only a bare
    # `*mut/*const Ident` right-hand side qualifies (fn-pointer and Option aliases stay opaque).
    for name, rhs in re.findall(r"(?m)^\s*pub\s+type\s+([A-Za-z_]\w*)\s*=\s*([^;]+);",
                                rs_text or ""):
        m = re.fullmatch(r"\s*\*\s*(mut|const)\s+((?:[A-Za-z_]\w*::)*)([A-Za-z_]\w*)\s*", rhs)
        if m and name not in out and m.group(3) != name:
            out[name] = f"*{m.group(1)} {m.group(3)}"
    return out


_MODULE_PATH = re.compile(r"\b(?:crate|self|super)(?:::[A-Za-z_]\w*)*::")


def _resolve_leaf(ty: str) -> str:
    ty = _GLOBAL_COLONS.sub("", (ty or "").strip())
    ty = _ALIAS_PATH.sub("", ty)
    ty = _MODULE_PATH.sub("", ty)
    ty = re.sub(r"\s+", "", ty)
    return _C_ABI_ALIASES.get(ty, ty)


def _norm_ty(ty: str, aliases: dict | None = None) -> str:
    """Lifetimes and module paths stripped, aliases resolved, whitespace removed -- in that order.

    Order matters: removing the whitespace first welds `*mut c_char` into `*mutc_char`, and the
    alias substitution then finds no `c_char` token to resolve.
    """
    ty = re.sub(r"/\*.*?\*/", "", ty or "")          # CROWN annotates: `*mut /* owning */ T`
    ty = re.sub(r"&\s*'\w+\s*", "&", ty)
    ty = _GLOBAL_COLONS.sub("", ty)
    ty = _ALIAS_PATH.sub("", ty)
    ty = _MODULE_PATH.sub("", ty)
    if aliases is None:
        aliases = {}

    def sub(m):
        n = m.group(0)
        return aliases.get(n) or _C_ABI_ALIASES.get(n, n)

    for _ in range(4):
        new = _IDENT.sub(sub, ty)
        if new == ty:
            break
        ty = new
    return re.sub(r"\s+", "", ty)


def rust_bridge(adapter: str, rust_ty: str | None, elem: str | None,
                c_rust: str | None, aliases: dict | None = None,
                one_elem: bool = False, writes: bool = False,
                owner: tuple | None = None) -> tuple[str | None, str | None]:
    """(bridge name, reason it is missing).  `rust_ty` None means the C ABI form."""
    if rust_ty is None:
        return "c_abi", None                 # no Rust signature parsed: raw C-ABI call
    r = _norm_ty(rust_ty, aliases)

    if adapter == "buffer_table":
        # The table keeps its C shape in every translation this bridges: a pointer to pointers of
        # the same element type. A reshaped table (`&[&[f64]]`, `Vec<Vec<f64>>`) is refused.
        m = re.fullmatch(r"\*(?:mut|const)\*(mut|const)(\w+)", r)
        if m and m.group(2) == _resolve_leaf(elem or ""):
            if writes and m.group(1) == "const":
                return None, (f"buffer table rows are written by the callee but the Rust "
                              f"parameter is {rust_ty} (const rows)")
            return "c_abi", None
        return None, (f"buffer table of {elem} has Rust type {rust_ty}, which is not a pointer to "
                      f"pointers of the same element type (reshaped table; no bridge)")

    if adapter == "produced_object":
        # The object comes from the translation's OWN producer, so it already has the Rust type
        # the target wants; the bridge only has to know whether to pass it as a raw pointer or
        # reborrow it as a reference. Anything else (a Box, an owned struct) is a reshaped
        # ownership model the pilot does not bridge.
        if owner and owner[0] == "opt_box":
            # Nullable owned object: the harness holds the box for the whole call and lends the
            # target a BORROWED view of it. A target that takes the box by value would move the
            # owner out of the harness -- the object could not then be compared or freed on a
            # claimed schedule -- so it stays a construction failure (family rule 6).
            orx = re.escape(owner[1])
            m = re.fullmatch(rf"Option<&(mut)?{orx}(?:<[^<>]*>)?>", r)
            if m:
                return ("opt_box_opt_ref_mut" if m.group(1) else "opt_box_opt_ref"), None
            m = re.fullmatch(rf"&(mut)?{orx}(?:<[^<>]*>)?", r)
            if m:
                return ("opt_box_ref_mut" if m.group(1) else "opt_box_ref"), None
            return None, (f"the produced object is owned as Option<Box<{owner[1]}>> in Rust and the "
                          f"target takes {rust_ty}: the nullable-owned-object bridge lends a borrowed "
                          f"view (Option<&T>, Option<&mut T>, &T, &mut T) and never transfers the box")
        name = _resolve_leaf(elem or "")
        if re.fullmatch(rf"\*(?:mut|const){re.escape(name)}", r):
            return "c_abi", None
        if re.fullmatch(rf"&(?:mut)?{re.escape(name)}", r):
            return "ref_obj", None
        if re.fullmatch(rf"Option<&(?:mut)?{re.escape(name)}>", r):
            return "opt_ref_obj", None
        return None, (f"produced object of type {elem} is passed as {rust_ty} in Rust, which is "
                      f"neither a raw pointer, a reference, nor an Option of one (pilot bridges only those)")

    if adapter == "input_string_pointer_table":
        m = re.fullmatch(r"\*(?:mut|const)\*(?:mut|const)(\w+)", r)
        if m and m.group(1) == _resolve_leaf(elem or ""):
            return "c_abi", None
        return None, (f"string pointer table of {elem} has Rust type {rust_ty}, which is not a "
                      f"pointer to a pointer to the same element type")

    if r.startswith("*"):
        # Raw pointer: check the pointee category/width/signedness, and that a buffer the callee
        # writes is not handed over as `*const`.
        mut = r.startswith("*mut")
        pointee = r[4:] if mut else r[6:]
        writes = adapter in ("inout_buffer", "output_buffer", "inout_array", "output_array",
                             "capacity_ptr", "out_scalar")
        if writes and not mut:
            return None, (f"{adapter} is written by the callee but the Rust parameter is "
                          f"{rust_ty} (const)")
        # `size_t*` is `*mut u64` in c2rust's spelling (c_ulong) and `*mut usize` in an idiomatic
        # one: same width, same signedness on this target -- one logical buffer either way
        _same = {"usize": {"usize", "u64"}, "u64": {"usize", "u64"}, "isize": {"isize", "i64"}, "i64": {"isize", "i64"}}
        if elem and pointee not in ({elem, "c_void", "core::ffi::c_void"} | _same.get(elem, set())) \
                and adapter != "null_pointer":
            return None, (f"pointer element type is {elem} in C and {pointee} in Rust: the same "
                          f"logical buffer cannot be passed")
        return "c_abi", None

    if adapter in ("scalar", "bounded_scalar", "length"):
        if r in _INT_RUST or r in _FLOAT_RUST:
            if c_rust and r != c_rust:
                # a different-but-compatible width/signedness; the call site casts through i128
                if (r in _INT_RUST) == (c_rust in _INT_RUST):
                    return "scalar_cast", None
                return None, (f"scalar parameter is {c_rust} in C and {rust_ty} in Rust: the "
                              f"category differs, so the same logical value cannot be passed")
            return "scalar_copy", None
        if r == "bool" and c_rust in _INT_RUST:
            return None, "scalar parameter lifted to bool: the C domain is wider than bool"
        return None, f"scalar parameter has Rust type {rust_ty}, which is not a scalar"

    if adapter == "null_pointer":
        return None, f"void* parameter has Rust type {rust_ty}; only a raw pointer is lossless"

    elem = _resolve_leaf(elem or "")
    const_shapes = {f"&[{elem}]": "slice", f"&Vec<{elem}>": "vec_ref", f"Vec<{elem}>": "vec",
                    f"Box<[{elem}]>": "boxed_slice", f"&Box<[{elem}]>": "boxed_slice_ref",
                    f"Option<&[{elem}]>": "option_slice"}
    mut_shapes = {f"&mut[{elem}]": "mut_slice", f"Option<&mut[{elem}]>": "option_mut_slice",
                  f"&mut Vec<{elem}>": "mut_vec_ref"}
    mut_shapes = {_norm_ty(k, aliases): v for k, v in mut_shapes.items()}
    const_shapes = {_norm_ty(k, aliases): v for k, v in const_shapes.items()}

    if adapter in ("inout_buffer", "output_buffer", "inout_array", "output_array",
                   "input_array", "input_buffer") and one_elem:
        # exactly one element: a lifter writes this as a scalar reference, never a slice
        if r in (_norm_ty(f"&mut {elem}", aliases), _norm_ty(f"Option<&mut {elem}>", aliases),
                 _norm_ty(f"&{elem}", aliases), _norm_ty(f"Option<&{elem}>", aliases)):
            return "mut_ref_one", None

    if adapter in ("input_buffer", "input_array", "input_string"):
        if r in const_shapes:
            return const_shapes[r], None
        if r in mut_shapes:
            return mut_shapes[r], None
        return None, (f"input buffer of {elem} has Rust type {rust_ty}, which is not a raw "
                      f"pointer, a slice, a Vec or a Box<[T]> of the same element type")

    if adapter in ("inout_buffer", "output_buffer", "inout_array", "output_array"):
        if r in mut_shapes:
            return mut_shapes[r], None
        return None, (f"mutable buffer of {elem} has Rust type {rust_ty}, which is not a raw "
                      f"pointer or a mutable slice of the same element type")

    if adapter in ("capacity_ptr", "out_scalar"):
        if r in (_norm_ty(f"&mut {elem}", aliases), _norm_ty(f"Option<&mut {elem}>", aliases)):
            return "mut_ref", None
        return None, (f"out-scalar of {elem} has Rust type {rust_ty}, which is not a raw pointer "
                      f"or a mutable reference to the same type")

    if adapter == "struct_value":
        return ("c_abi", None) if r.startswith("&") else \
               (None, f"struct parameter has Rust type {rust_ty}")

    return None, f"no bridge for adapter {adapter!r} with Rust type {rust_ty}"


def apply_rust_bridges(plan: InputPlan, rust_types: list[str] | None,
                       params: list[dict], aliases: dict | None = None) -> list[str]:
    """Attach a `rust_bridge` to every InputSpec; return the construction failures."""
    fails: list[str] = []
    # A buffer bridged to a slice CARRIES its length, so the length parameter is folded out of the
    # Rust call and has no bridge of its own.
    by_name = {p["name"]: p for p in params}
    by_pos = {}
    if rust_types is not None and len(rust_types) != len(params):
        # A reshaped signature (C2SaferRust: `(dest: &mut Vec<u8>, source: &[u8], ..)` for a
        # seven-parameter C function) cannot be bridged positionally. Before this check every
        # parameter silently fell through to `c_abi` -- the plan said `planned`, and the build
        # said `E0061: takes 5 arguments but 7 were supplied`. A boundary that cannot be planned
        # FAILED harness construction; it is not a plan with a hopeful bridge.
        why = (f"Rust signature has {len(rust_types)} parameters, C has {len(params)}: "
               f"reshaped API, no positional bridge")
        for s in plan.specs:
            s.rust_type, s.rust_bridge = None, None
        return [f"{s.param}: no lossless Rust bridge -- {why}" for s in plan.specs[:1]] \
            or [f"(signature): {why}"]
    if rust_types is not None:
        by_pos = {p["name"]: rust_types[i] for i, p in enumerate(params)}
    folded = set()
    for s in plan.specs:
        ty = by_pos.get(s.param)
        if s.c_decoder == "realized_resource":
            # The view was SELECTED by the manifest and VERIFIED against this very parameter type
            # in _realize_with (check 2); the bridge only records it.
            s.rust_type = ty
            s.rust_bridge = "view:" + s.detail["target_view"]["rust"]
            continue
        one = (s.detail.get("alloc_elems") == 1
               or (s.detail.get("extent") or {}).get("v") == 1)
        _own = ((s.detail.get("producer_owner"), s.detail.get("producer_rust_owner"))
                if s.detail.get("producer_owner") else None)
        b, why = rust_bridge(s.c_decoder, ty, s.detail.get("elem"),
                             by_name.get(s.param, {}).get("rust"), aliases, one,
                             writes=bool(s.detail.get("written")), owner=_own)
        s.rust_type = ty
        s.rust_bridge = b
        if b is None:
            fails.append(f"{s.param}: no lossless Rust bridge -- {why}")
        elif b in ("slice", "mut_slice", "option_slice", "option_mut_slice", "vec", "vec_ref",
                   "boxed_slice", "boxed_slice_ref", "mut_vec_ref"):
            ln = s.detail.get("length_param") or s.detail.get("capacity_param")
            if ln:
                folded.add(ln)
    for s in plan.specs:
        if s.param in folded:
            s.rust_bridge = "folded_into_slice"
    return fails


# ---------------------------------------------------------------------------
# InputPlan / HarnessPlan
# ---------------------------------------------------------------------------
@dataclass
class InputSpec:
    """One parameter of the C-shaped canonical input, and how it reaches the Rust side.

    `c_decoder` says how the fuzz bytes become a value C accepts; `rust_bridge` says how that same
    value is materialized a second time as a Rust argument. Two independent allocations, one
    logical input.
    """
    param: str
    c_decoder: str
    detail: dict = field(default_factory=dict)
    evidence: list = field(default_factory=list)
    rust_type: str | None = None
    rust_bridge: str | None = None


@dataclass
class InputPlan:
    specs: list = field(default_factory=list)
    failures: list = field(default_factory=list)
    extents: dict = field(default_factory=dict)     # param -> {"bound","source","alloc_elems"}
    liveness: list = field(default_factory=list)


# A parameter that flows into one of these is not a value the harness can construct: it NAMES
# external state, and the call has an effect the harness cannot undo (creating or truncating a
# file, replacing the process, touching the network).  This is deliberately about EFFECTS, not
# about `fopen`: `BZ2_bzopen(path, mode)` really consumes "a file with certain contents", and
# fuzzing its NAME only ever explores fopen failing.  Contrast cJSON, whose `const char*` IS the
# value under test -- that is why this rule costs cJSON nothing.
EFFECTFUL_CALLS = {
    "fopen", "freopen", "fdopen", "open", "openat", "creat", "popen", "tmpfile",
    "unlink", "remove", "rename", "link", "symlink", "truncate", "ftruncate",
    "mkdir", "rmdir", "chmod", "chown",
    "system", "exec", "execl", "execle", "execlp", "execv", "execvp", "execvpe",
    "socket", "connect", "bind", "listen", "accept",
}

_LEN_SUFFIXES = ("len", "length", "size", "count", "cap", "capacity", "n", "num", "sz", "nb")


def _name_pairs(buf: str, num: str) -> bool:
    """Uniform (not per-library) name relation between a buffer and its length parameter."""
    b, n = buf.lower(), num.lower()
    # The generator escapes a C parameter whose name is a Rust keyword (`in` -> `in_`,
    # gdh.safe_name); the relation is between the C NAMES, so the escape is undone here or
    # lodepng's `(const unsigned char* in, size_t insize)` never pairs and `insize` is decoded as
    # a free scalar the callee trusts past the buffer.
    if b.endswith("_") and b[:-1] in gdh.RUST_KEYWORDS:
        b = b[:-1]
    if n.startswith(b) and n[len(b):].lstrip("_") in _LEN_SUFFIXES:
        return True
    if n.endswith(b) and n[: -len(b)].rstrip("_") in _LEN_SUFFIXES:
        return True
    return n in _LEN_SUFFIXES


def subscripted_names(facts: BodyFacts) -> set:
    return {s.base for s in facts.subscripts}


def _table_count(p: dict, params: list[dict], subs: set) -> str | None:
    """The count parameter of a `T** + count` string table, or None if this T** is not one.

    The discriminator is the BODY, not adjacency: a string table is INDEXED (`strings[i]`), while
    an out-parameter is dereferenced and written (`*return_parse_end = end`). Both shapes have an
    adjacent integer, so adjacency alone would confuse them.
    """
    if p["kind"] != "ptr_ptr" or p["name"] not in subs:
        return None
    names = [q["name"] for q in params]
    i = names.index(p["name"])
    for j in (i + 1, i - 1):
        if 0 <= j < len(params) and params[j]["kind"] == "scalar":
            return params[j]["name"]
    return None


def analyze_inputs(params: list[dict], facts: BodyFacts, policy: GeneratorPolicy,
                   effectful: set | None = None) -> InputPlan:
    """fuzz bytes -> logical values -> a C representation AND a Rust representation."""
    plan = InputPlan()
    names = [p["name"] for p in params]
    by_name = {p["name"]: p for p in params}
    ptrs = [p for p in params if p["kind"] in ("ptr", "ptr_array", "ptr_struct", "ptr_ptr")]
    _subs = subscripted_names(facts)
    scalars = [p for p in params if p["kind"] == "scalar"]

    # ---- T** buffer tables (tulip: `TI_REAL const *const *inputs`, `TI_REAL *const *outputs`) ----
    # A T** indexed ONLY by constants is a table of rows; row k is a pointer parameter of its own
    # (`inputs__row0`), whose extent the body derives like any other pointer's -- through the
    # local that names it (`input = inputs[0]`) or a nested subscript (`outputs[1][i]`). The row
    # count is a fact of the body (max constant index + 1), never a caller's table declaration.
    tables: dict[str, int] = {}
    pseudo_rows: dict[str, tuple[str, int]] = {}
    for p in params:
        n = p["name"]
        # a `char** + count` is the string table of old (cJSON_CreateStringArray); a table of any
        # other element type indexed by constants is a buffer table, adjacency notwithstanding
        if p["kind"] != "ptr_ptr" or n in facts.derefs or n in facts.advanced \
                or p.get("elem") not in (_INT_RUST | _FLOAT_RUST) \
                or (p.get("elem") in ("i8", "u8") and _table_count(p, params, _subs)):
            continue
        own = [s for s in facts.subscripts if s.base == n]
        if not own or any(s.written or s.index_bound.get("k") != "const" for s in own):
            continue
        rows = max(s.index_bound["v"] for s in own) + 1
        if rows < 1 or rows > policy.max_table_rows:
            continue
        tables[n] = rows
        for k in range(rows):
            name = f"{n}__row{k}"
            pseudo_rows[name] = (n, k)
            ptrs.append({"kind": "ptr", "const": bool(p.get("inner_const")), "elem": p["elem"],
                         "elem_w": p["elem_w"], "name": name, "_row_of": (n, k)})

    for p in params:
        eff = facts.escape_callees.get(p["name"], set()) & (effectful or EFFECTFUL_CALLS)
        if eff:
            plan.failures.append(
                f"{p['name']}: environment input -- it flows into {sorted(eff)[0]}(), whose effect "
                f"the harness cannot undo. What the boundary consumes is external state, not this "
                f"value; needs an environment adapter")
        if p["kind"] == "void_ptr" and facts.derefs.get(p["name"], {}).get("read"):
            plan.failures.append(f"{p['name']}: void* is dereferenced by the entry, so NULL is "
                                 f"not a valid value and no other value can be constructed")
        if p["kind"] == "ptr_ptr" and p["name"] not in tables \
                and not _table_count(p, params, subscripted_names(facts)):
            plan.failures.append(
                f"{p['name']}: T** is not a constructible input here -- it is dereferenced and "
                f"written rather than indexed, so it is an OUT pointer whose value is an interior "
                f"pointer into another argument, not a table of strings")
        if p["kind"] == "ptr_struct" and not p["struct"].get("pod"):
            plan.failures.append(f"{p['name']}: struct {p['struct']['name']} is not POD")

    # ---- required extent of each pointer parameter, from the body only ----
    req: dict[str, list[dict]] = {p["name"]: [] for p in ptrs}
    written: dict[str, bool] = {p["name"]: False for p in ptrs}
    read: dict[str, bool] = {p["name"]: False for p in ptrs}
    subscripted: dict[str, bool] = {p["name"]: False for p in ptrs}
    ev_by_param: dict[str, list] = {p["name"]: [] for p in ptrs}
    lowers: dict[str, list] = {p["name"]: [] for p in ptrs}
    idx_deps: dict[str, set] = {p["name"]: set() for p in ptrs}
    for s in facts.subscripts:
        if s.base not in req:
            continue
        subscripted[s.base] = True
        written[s.base] = written[s.base] or s.written
        read[s.base] = read[s.base] or not s.written
        lowers[s.base].append(s.index_lower)
        idx_deps[s.base].update(s.deps)
        req[s.base].append(b_add(s.index_bound, 1))
        if len(ev_by_param[s.base]) < 6:
            ev_by_param[s.base].append(s.ev)
    for n, d in facts.derefs.items():
        if n in req:
            written[n] = written[n] or d["written"]
            read[n] = read[n] or d["read"]
            req[n].append(b_const(1))
            lowers[n].append(b_const(0))
            ev_by_param[n].append(d["ev"])
    # A pointer the body ADVANCES (`*out++ = v`, `p += k`) is dereferenced relative to a moving
    # base: neither its derefs nor its subscripts bound the extent. Before this rule `*out++` in
    # a loop counted as extent 1 -- an under-allocation the harness itself would have caused.
    # The extent is unknown; the policy allocation applies, and the loop-trip clamp (max_trip)
    # is what keeps the writes inside it -- recorded as the unproven obligation it is.
    for n in facts.advanced:
        if n in req:
            req[n] = [b_unknown("pointer advanced in the body")]
            lowers[n] = [b_const(0)]
            ev_by_param[n].append(_ev("pointer_advanced_in_body", _NoLoc(),
                                      f"{n} is incremented or reassigned in the body, so its "
                                      f"accesses do not bound its extent: policy allocation, with "
                                      f"the loop-trip parameters clamped by the policy"))

    # ---- scalar caps: guards, index roles, loop-trip policy ----
    caps: dict[str, int] = {}
    scalar_spec: dict[str, dict] = {}
    for p in scalars:
        n = p["name"]
        lo, hi, ev, src = None, None, [], []
        g = facts.guards.get(n)
        if g:
            lo, hi = g["min"], g["max"]
            ev.extend(g["evidence"])
            src.append("rejection_guard")
        if n in facts.loop_bound_params and hi is None:
            hi = policy.max_trip
            lo = 0 if lo is None else lo
            ev.append(facts.loop_bound_params[n])
            ev.append(_ev_policy("policy_trip_clamp", "max_trip",
                                 f"{n} controls a loop trip count and no guard bounds it; "
                                 f"clamped by the global policy (liveness only)"))
            src.append("policy_trip_clamp")
        scalar_spec[n] = {"min": lo, "max": hi, "evidence": ev, "sources": src}

    # a scalar that appears in a pointer's required extent must fit the allocation
    for n, bs in req.items():
        for b in bs:
            for pn in b_params(b):
                if pn in scalar_spec:
                    scalar_spec[pn].setdefault("bounds_an_extent", []).append(n)

    # RULE 6, the harness's own obligation.  Two clamps, both derived, neither per-boundary:
    #
    #  (a) EXTENT FITS ALLOCATION.  When a pointer's required extent is a proven expression over
    #      parameters (`max(i1+8, nblock)`), the harness allocates the policy size and clamps
    #      every parameter in that expression so the requirement is met.  Without this the
    #      "proven" extent would be evaluated against a cap nothing actually enforces.
    #  (b) INDEX CLAMPED TO ALLOCATION.  When the extent could not be proven at all, every
    #      parameter that can transitively influence an index into the buffer is clamped to the
    #      allocation.  This is what replaces a hand-written `"bounded": (0, 1023)`.
    #
    # In both cases the bound is the harness's own allocation, which comes from the global policy
    # -- never from a caller's array declaration (rule 7).
    def _clamp(pname: str, hi: int, rule: str, elems: int, buf: str, why: str):
        if pname not in scalar_spec or hi < 0:
            return
        sp = scalar_spec[pname]
        sp["max"] = hi if sp["max"] is None else min(sp["max"], hi)
        sp["min"] = 0 if sp["min"] is None else max(sp["min"], 0)
        if rule not in sp["sources"]:
            sp["sources"].append(rule)
            sp["evidence"].append(_ev_policy(rule, "unproven_extent_elems",
                                             f"{pname} {why} {buf}; the harness allocates {elems} "
                                             f"elements, so {pname} <= {hi}"))

    for pt in ptrs:
        n = pt["name"]
        if not req.get(n):
            continue
        ew = pt.get("elem_w") or 1
        alloc = min(policy.unproven_extent_elems, max(1, policy.max_buffer_bytes // ew))
        b = b_max(req[n])
        terms = [] if b_is_unknown(b) else (b["of"] if b["k"] == "max" else [b])
        if terms and all(x["k"] in ("const", "param") for x in terms):
            for x in terms:
                if x["k"] != "param" or x["p"] not in scalar_spec:
                    continue
                _clamp(x["p"], ((alloc - x["add"]) * x["div"]) // x["mul"],
                       "extent_fits_allocation", alloc, n,
                       "appears in the proven required extent of")
        else:
            for dep in sorted(idx_deps.get(n, ())):
                _clamp(dep, alloc - 1, "index_clamped_to_allocation", alloc, n,
                       "can influence an index into")

    # ---- pointer/length pairing ----
    length_of: dict[str, str] = {}     # buffer -> length param
    pair_src: dict[str, str] = {}
    scalar_names = {s["name"] for s in scalars}
    for pn in list(req):
        if not req[pn] or pn in pseudo_rows:     # a table row never defines a length parameter
            continue
        b = b_max(req[pn])
        # ONLY an extent that is exactly one parameter, with no additive or multiplicative term
        # and no competing max() branch, is a parameter-carried length.  `max(23, maxLen+1)` is a
        # proven extent but it is NOT "maxLen elements", and treating it as one under-allocates.
        if b["k"] == "param" and b["mul"] == 1 and b["div"] == 1 and b["add"] == 0 \
                and b["p"] in scalar_names:
            length_of[pn] = b["p"]
            pair_src[pn] = "proven_index_bound"
    for p in ptrs:                     # heuristic fallback: adjacency + uniform name relation
        pn = p["name"]
        # a pointer the body ADVANCES (`*buf++`, zlib's adler32/crc32) has an unknown required
        # extent, which is exactly what the adjacent `len` says: the heuristic applies to it too
        if pn in length_of or p.get("_row_of") or (req.get(pn) and not b_is_unknown(b_max(req[pn]))):
            continue
        i = names.index(pn)
        for j in (i + 1, i - 1):
            if not (0 <= j < len(names)):
                continue
            q = by_name[names[j]]
            is_int_val = q["kind"] == "scalar"
            is_int_ptr = q["kind"] == "ptr" and q.get("elem") in (_SIGNED | _UNSIGNED)
            if (is_int_val or is_int_ptr) and _name_pairs(pn, q["name"]):
                length_of[pn] = q["name"]
                pair_src[pn] = "heuristic_name_and_adjacency"
                break

    # A length parameter shared by several buffers is DEFINED by exactly one of them: the first
    # buffer that is read (its contents come from the fuzz input).  Write-only buffers sharing the
    # same length are allocated to it; they never define it.
    length_src: dict[str, str] = {}
    for ln in set(length_of.values()):
        sharers = [b for b in length_of if length_of[b] == ln]
        readers = [b for b in sharers if read.get(b) and not written.get(b)]
        length_src[ln] = (readers or sharers)[0]
    len_params = set(length_of.values())
    cap_ptr_params = {v for v in length_of.values()
                      if by_name[v]["kind"] == "ptr"}

    # ---- concrete allocation for every pointer ----
    for p in ptrs:
        n = p["name"]
        ew = p.get("elem_w") or 1
        policy_elems = min(policy.unproven_extent_elems, max(1, policy.max_buffer_bytes // ew))
        if length_of.get(n) and pair_src[n] == "proven_index_bound":
            source, bound = "proven_index_bound", b_param(length_of[n])
            alloc = None
        elif length_of.get(n):
            source, bound, alloc = "parameter_carried_length", b_param(length_of[n]), None
        else:
            b = b_max(req[n]) if req.get(n) else b_unknown("pointer is never dereferenced")
            # Only parameters with an ENFORCED maximum may be substituted; a parameter with no
            # derived bound makes the extent unknown rather than silently assuming one.
            caps_for_eval = {s: sp["max"] for s, sp in scalar_spec.items()
                             if sp["max"] is not None}
            v = b_eval_max(b, caps_for_eval)
            if v is not None and 0 < v <= policy_elems:
                source, bound, alloc = "proven_extent_in_boundary", b, v
            elif v is not None and v > policy_elems:
                # The extent is proven but larger than the policy will allocate (a `u32` load used
                # as an index proves only 2**32).  The harness cannot satisfy it, so the extent is
                # the policy's and staying inside it becomes an unproven obligation.
                source, bound, alloc = "policy_allocation_capped", b, policy_elems
            else:
                source, bound, alloc = "policy_allocation", b, policy_elems
        plan.extents[n] = {"bound": bound, "source": source, "alloc_elems": alloc,
                           "elem": p.get("elem"), "elem_width": ew,
                           "written": written.get(n, False),
                           "subscripted": subscripted.get(n, False),
                           "index_lowers": lowers.get(n, []),
                           "read": read.get(n, False),
                           "escapes": n in facts.escapes,
                           "evidence": ev_by_param.get(n, [])}

    # An index whose lower bound is a parameter is non-negative exactly when that parameter's
    # derived minimum is; evaluate it here, where the scalar domains are known.
    mins = {n: (sp["min"] if sp["min"] is not None else None) for n, sp in scalar_spec.items()}
    for n, e in plan.extents.items():
        neg = False
        for lo in e.pop("index_lowers", []):
            if lo is None or lo.get("k") == "unknown":
                neg = True
            elif lo["k"] == "const":
                neg = neg or lo["v"] < 0
            elif lo["k"] == "param":
                m = mins.get(lo["p"])
                neg = neg or m is None or (m * lo["mul"]) // lo["div"] + lo["add"] < 0
            else:
                neg = True
        e["may_index_negative"] = neg

    # ---- adapters ----
    for p in params:
        n = p["name"]
        if p["kind"] == "ptr_ptr" and n in tables:
            row_specs = []
            for k in range(tables[n]):
                e = plan.extents[f"{n}__row{k}"]
                if e["written"] and e["read"]:
                    ad = "inout_array"
                elif e["written"]:
                    ad = "output_array"
                else:
                    ad = "input_array"
                row_specs.append({"row": k, "adapter": ad, "extent": e["bound"],
                                  "extent_source": e["source"], "alloc_elems": e["alloc_elems"],
                                  "fills_from_fuzz": bool(e["read"]), "written": bool(e["written"]),
                                  "evidence": e["evidence"]})
            plan.specs.append(InputSpec(
                n, "buffer_table",
                {"elem": p["elem"], "elem_width": p["elem_w"],
                 "inner_const": bool(p.get("inner_const")), "rows": tables[n],
                 "row_specs": row_specs, "written": any(r["written"] for r in row_specs)},
                [_ev("t_star_star_indexed_by_constants_is_a_buffer_table", _NoLoc(),
                     f"{n} is indexed only by the constants 0..{tables[n] - 1}; each row is a "
                     f"buffer whose extent is derived from the body like any pointer parameter "
                     f"(a local initialised from {n}[k] names row k)")]))
            continue
        if p["kind"] == "ptr_ptr":
            cnt = _table_count(p, params, _subs)
            if cnt is None:
                continue                     # already reported as a construction failure
            plan.specs.append(InputSpec(
                n, "input_string_pointer_table",
                {"elem": p["elem"], "elem_width": p["elem_w"], "length_param": cnt,
                 "count_max": policy.max_table_rows},
                [_ev("t_star_star_indexed_with_a_count_is_a_string_table", _NoLoc(),
                     f"{n} is indexed in the body and {cnt} is its adjacent count; each row is an "
                     f"independent NUL-terminated string"),
                 _ev_policy("policy_table_rows", "max_table_rows",
                            f"{n}'s row count is bounded by the global policy")]))
            continue
        if p["kind"] == "void_ptr":
            # A void* CAN carry a real input -- it is usually an opaque handle the caller owns.
            # The type carries no shape, so NULL is the only value the harness can construct, and
            # that is COMPLETE only when the entry never uses the parameter. When the entry does
            # use it, NULL is safe but leaves the other branch unreachable, and reaching it needs
            # an object the harness does not own (an operation-sequence capability).
            used = (n in facts.escapes) or bool(facts.derefs.get(n))
            ev = [_ev("void_pointer_has_no_shape", _NoLoc(),
                      "void* carries no shape, so NULL is the only constructible value")]
            if n in facts.escapes:
                ev.append(facts.escapes[n])
            plan.specs.append(InputSpec(
                n, "null_pointer",
                {"input_strength": "partial(null-only)" if used else "complete",
                 "reason": (f"the entry uses {n}, so NULL leaves the other path unreachable"
                            if used else f"the entry never uses {n}, so NULL is the whole input")},
                ev))
            continue
        if p["kind"] == "scalar":
            if n in len_params:
                owner = length_src[n]
                plan.specs.append(InputSpec(n, "length", {"of_buffer": owner},
                                            [_ev("length_of_buffer_not_decoded_independently",
                                                 _NoLoc(), f"{n} is the element count of {owner}")]))
                continue
            sp = scalar_spec[n]
            if sp["min"] is None and sp["max"] is None:
                plan.specs.append(InputSpec(n, "scalar", {}, []))
            else:
                plan.specs.append(InputSpec(n, "bounded_scalar",
                                            {"min": sp["min"] if sp["min"] is not None else None,
                                             "max": sp["max"],
                                             "derivation": sp["sources"]}, sp["evidence"]))
            continue
        if p["kind"] not in ("ptr", "ptr_array", "ptr_struct"):
            continue
        e = plan.extents[n]
        max_elems = max(1, policy.max_buffer_bytes // (p.get("elem_w") or 1))
        if n in cap_ptr_params:
            owner = length_src[n]
            plan.specs.append(InputSpec(n, "capacity_ptr",
                                        {"of_buffer": owner, "elem": p.get("elem"),
                                         "elem_width": p.get("elem_w")},
                                        [_ev("capacity_passed_by_pointer", _NoLoc(),
                                             f"{n} carries {owner}'s capacity in and its "
                                             f"written length out")]))
            continue
        if p["kind"] == "ptr_struct":
            # a POD struct behind a pointer: decoded field by field (input_struct / inout_struct in
            # the generator). A boundary that FREES it (quadtree_point_free) would free the
            # harness's own value: no heap-owned struct adapter exists, so that is a construction
            # failure, stated -- not a crash on both sides counted as agreement.
            freed = bool(facts.escape_callees.get(n, set()) & {"free", "realloc"})
            if freed:
                plan.failures.append(f"{n}: the boundary passes this POD struct pointer to free(); "
                                     f"the harness owns the value and has no heap-owned struct adapter")
            plan.specs.append(InputSpec(n, "struct_value",
                                        {"struct": p["struct"]["name"], "written": bool(e["written"]),
                                         "freed": freed}, e["evidence"]))
            continue
        ln = length_of.get(n)
        # A length parameter may size several buffers of different element widths (`alphaSize`
        # sizes both a u8 `length` and an i32 `code`).  Cap it by the WIDEST of them, so the
        # global byte budget holds for every buffer it sizes -- not just for this one.
        widest = max([q.get("elem_w") or 1 for q in ptrs if length_of.get(q["name"]) == ln]
                     or [p.get("elem_w") or 1]) if ln else (p.get("elem_w") or 1)
        max_elems = max(1, policy.max_buffer_bytes // widest)  # noqa: F841 (rebound per buffer)
        if ln:
            if by_name[ln]["kind"] == "ptr":
                adapter = "output_buffer"        # its capacity is passed in/out by pointer
            elif e["written"] and not e["read"]:
                adapter = "output_buffer"
            elif e["written"] or (e["escapes"] and not p.get("const")):
                adapter = "inout_buffer"
            else:
                adapter = "input_buffer"
        elif (p.get("const") and p.get("elem") in ("i8", "u8")
              and b_is_unknown(e["bound"]) and not e["written"]):
            # C convention: a `const char*` with no length parameter and no provable extent is a
            # NUL-terminated string. Neither side may write it, so both share one allocation.
            plan.specs.append(InputSpec(
                n, "input_string", {"elem": p["elem"], "elem_width": p["elem_w"],
                                    "max_elems": max_elems},
                [_ev("const_char_star_without_length_is_a_string", _NoLoc(),
                     f"{n} is a const char* with no length parameter and no provable extent")]))
            continue
        else:
            if e["written"] and e["read"]:
                adapter = "inout_array"
            elif e["written"]:
                adapter = "output_array"
            else:
                adapter = "input_array"
        detail = {"elem": p.get("elem"), "elem_width": p.get("elem_w"),
                  "extent": e["bound"], "extent_source": e["source"]}
        if ln:
            detail["capacity_param" if adapter == "output_buffer" else "length_param"] = ln
            detail["fills_from_fuzz"] = (n == length_src.get(ln) and adapter != "output_buffer")
            if n == length_src.get(ln):
                detail["max_elems"] = max_elems
                detail["max_elems_evidence"] = _ev_policy(
                    "policy_buffer_bound", "max_buffer_bytes",
                    f"{n}'s element count is fuzz-decoded and capped by the global policy; it "
                    f"defines {ln}")
        else:
            detail["alloc_elems"] = e["alloc_elems"]
            # A buffer the callee READS must carry fuzz-controlled contents; zeroing it would
            # make every path through it deterministic.
            detail["fills_from_fuzz"] = bool(e["read"])
        plan.specs.append(InputSpec(n, adapter, detail, e["evidence"]))

    scalar_names_all = {s["name"] for s in scalars}
    for n, ev in facts.loop_bound_params.items():
        if n not in scalar_names_all:
            continue
        plan.liveness.append({"param": n, "obligation": "controls a loop trip count",
                              "resolution": "clamped by GeneratorPolicy.max_trip", "evidence": ev})
    return plan


class _NoLoc:
    """Evidence anchor for a fact derived from the plan itself rather than from a source line."""
    location = None


@dataclass
class HarnessPlan:
    boundary: str
    status: str
    inputs: list
    liveness: list
    failures: list
    policy: dict
    plan_version: int = PLAN_VERSION
    # Plan-origin record when a resource-realization plugin supplied a materialization
    # (docs/construction_recipe_plugin_plan.md section 5); None for every automatically planned
    # boundary, and then OMITTED from the serialised plan (`plan_dict`) so no-plugin output is
    # byte-identical to before the extension point existed.
    origin: dict | None = None


def plan_dict(plan: "HarnessPlan") -> dict:
    d = asdict(plan)
    if d.get("origin") is None:
        d.pop("origin", None)
    return d


# Accepted resource-realization manifests for this run (`--realization-plugins`), consulted by
# build_plan ONLY after the generic producer search abstained (plan check 10). Empty = the
# extension point is never consulted and planning is unchanged.
_REALIZATIONS: list = []


def set_realizations(manifests: list) -> None:
    _REALIZATIONS[:] = list(manifests or [])


_SIG_CACHE: dict = {}
_PRODUCER_PLAN_CACHE: dict = {}
_DESTRUCTOR_CACHE: dict = {}


def _sig(cc_dir: Path, f: str, with_return_desc: bool = True, allow_nonpod: bool = True):
    """parse_entry_signature, memoised: planning cJSON's 71 boundaries re-parsed the TU thousands of
    times (every candidate producer, every destructor candidate, for every target)."""
    key = (str(cc_dir), f, with_return_desc, allow_nonpod)
    if key not in _SIG_CACHE:
        try:
            _SIG_CACHE[key] = ("ok", gdh.parse_entry_signature(cc_dir, f, with_return_desc=with_return_desc,
                                                              allow_nonpod=allow_nonpod))
        except SystemExit as e:
            _SIG_CACHE[key] = ("err", str(e))
    tag, val = _SIG_CACHE[key]
    if tag == "err":
        raise SystemExit(val)
    return val


# C name -> Rust name for translators that rename functions (SACTOR / PtrTrans `quickSort ->
# quick_sort`). Loaded from `<pair>/translated/renames.json`; the map is the matcher's (RQ1) output
# for the pair, never guessed here. Absent file = identity, so name-preserving pairs are unchanged.
_RENAMES: dict[str, str] = {}


def load_renames(pair: Path) -> dict[str, str]:
    p = Path(pair) / "translated" / "renames.json"
    return json.loads(p.read_text()) if p.exists() else {}


def _rn(name: str) -> str:
    return _RENAMES.get(name, name)


def _rust_fn_exists(rs_text: str | None, name: str) -> bool:
    return bool(rs_text) and re.search(
        rf'(?m)^\s*(?:pub\s+)?(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(_rn(name))}\s*[<(]',
        rs_text) is not None


_DRIVER_CACHE: dict = {}
_CALLEE_FACTS: dict = {}     # (TU, callee, depth) -> BodyFacts, one analysis per callee per plan run
_TU_DEFS: dict = {}          # TU spelling -> {function: definition cursor}


def _first_of(cur, kind):
    for n in cur.walk_preorder():
        if n.kind == kind and n.spelling:
            return n.spelling
    return None


def driver_evidence(cc_dir: Path, target: str) -> dict:
    """{producer: n}: in the pair's shipped drivers (`<pair>/drivers/*.c` -- test.c, example*.c,
    tests/main.c -- compiled with the pair's own flags, never linked into the harness), how often the
    value passed to `target` is the result of `producer`, either directly (`target(producer(..))`)
    or through a local assigned from it. A call-graph fact of the artifact, not a schema; used only
    to ORDER candidates that already passed rules 1-4. Empty when the pair ships no drivers."""
    key = (str(cc_dir), target)
    if key in _DRIVER_CACHE:
        return _DRIVER_CACHE[key]
    out: dict = {}
    drivers = sorted((Path(cc_dir).parent / "drivers").glob("*.c"))
    if not drivers:
        _DRIVER_CACHE[key] = out
        return out
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    cmd = next(iter(cdb.getAllCompileCommands()))
    args = cgmod._filter_compile_args(list(cmd.arguments),
                                      {cmd.filename, str((Path(cmd.directory) / cmd.filename).resolve()),
                                       Path(cmd.filename).name})
    index = Index.create()
    for d in drivers:
        try:
            tu = index.parse(str(d), args=args)
        except Exception:
            continue
        for fn in tu.cursor.walk_preorder():
            if fn.kind != CursorKind.FUNCTION_DECL or not fn.is_definition():
                continue
            assigned: dict = {}
            for n in fn.walk_preorder():
                if n.kind == CursorKind.VAR_DECL:
                    for k in n.get_children():
                        c = _first_of(k, CursorKind.CALL_EXPR)
                        if c:
                            assigned[n.spelling] = c
                elif n.kind == CursorKind.BINARY_OPERATOR:
                    kids = list(n.get_children())
                    if len(kids) == 2 and kids[0].kind == CursorKind.DECL_REF_EXPR:
                        c = _first_of(kids[1], CursorKind.CALL_EXPR)
                        if c:
                            assigned[kids[0].spelling] = c
                elif n.kind == CursorKind.CALL_EXPR and n.spelling == target:
                    for a in list(n.get_children())[1:]:
                        c = _first_of(a, CursorKind.CALL_EXPR)
                        if c and c != target:
                            out[c] = out.get(c, 0) + 1
                            continue
                        ref = _first_of(a, CursorKind.DECL_REF_EXPR)
                        if ref in assigned:
                            out[assigned[ref]] = out.get(assigned[ref], 0) + 1
    _DRIVER_CACHE[key] = out
    return out


# The `nullable owned object` bridge family (2026-09-07). C's owning `T*` reaches Rust either as a
# raw pointer (the mechanical lift) or as `Option<Box<R>>` -- CROWN and PtrTrans both produce the
# latter for quadtree. The two shapes differ in WHO owns the object during the call: with a raw
# pointer the callee sees the same address the harness holds; with a box the HARNESS owns it and
# lends the target a borrowed view. Everything else about the sequence is unchanged.
_OWNED_BOX = re.compile(r"Option<Box<([A-Za-z_]\w*)(?:<[^<>]*>)?>>")


def _producer_owner(rret: str, tname: str) -> tuple[str | None, str | None]:
    """(owner kind, Rust type name of the owned object) for a producer's normalised return type.

    `raw`     -- `*mut T` / `*const T`; the object's Rust type is the C type's name.
    `opt_box` -- `Option<Box<R>>`; R is returned so the TARGET's borrowed view can be required to
                 name the same R. The C signature already says both sides mean one object, so the
                 Rust-side obligation is consistency, not a C-to-Rust type-name map (PtrTrans
                 renames `quadtree_node_t` to `QuadtreeNode`).
    """
    if re.fullmatch(rf"\*(?:mut|const){re.escape(tname)}", rret):
        return "raw", tname
    m = _OWNED_BOX.fullmatch(rret)
    if m:
        return "opt_box", m.group(1)
    return None, None


def _plan_producer(cc_dir: Path, param: dict, entry: str, policy: GeneratorPolicy,
                   rust_text: str | None, rust_aliases: dict | None):
    """docs/producer_bridge_pilot.md sections 2-4: pick the producer for a `T*` parameter whose
    struct carries pointers, or say exactly why there is none. Returns (InputSpec | None, reason | None)."""
    st = param["struct"]
    tname, cname = st["name"], st.get("c_name", st["name"])
    base_reason = f"struct-invariant param {param['name']}: {tname} {st.get('reason')} (needs invariant reconstruction)"
    if rust_text is None:
        return None, base_reason + "; no Rust text to verify a producer against"
    rand_fns = random_functions(cc_dir)
    free_fns = freeing_functions(cc_dir)
    considered, alternatives, viable = [], [], []
    for f in _all_entries(cc_dir):
        if f == entry:
            continue
        try:
            fparams, fret, _fns, fdesc = _sig(cc_dir, f)
        except SystemExit as e:
            alternatives.append({"fn": f, "excluded": f"signature: {e}"[:120]})
            continue
        # rule 2: canonical pointee type is T, const ignored; mutability must satisfy the target
        if fdesc.get("kind") != "pointer" or fdesc.get("inner", {}).get("kind") != "struct" \
                or fdesc["inner"].get("name") != tname:
            continue                                   # returns something else: not a candidate
        considered.append(f)
        if fdesc.get("const") and not param.get("const"):
            alternatives.append({"fn": f, "excluded": "returns const T* but the target takes T*"})
            continue
        # rule 3: depth one -- a producer may not itself take T*
        if any(q.get("kind") == "ptr_struct_nonpod" for q in fparams):
            alternatives.append({"fn": f, "excluded": "takes a struct-with-pointers itself (pilot is depth 1)"})
            continue
        # rule 1: exists on both sides, and the Rust side hands the object back as a raw pointer
        # (the pilot's only ownership shape: what `*mut T` from C becomes on a faithful lift)
        if not _rust_fn_exists(rust_text, f):
            alternatives.append({"fn": f, "excluded": "not present in the Rust translation"})
            continue
        # the harness calls `translated::<producer>`: a C `static` producer that the translation
        # keeps private (lil's `real_trim`) is not callable, so it is not a candidate -- the next
        # viable producer (the public API's own constructor) is ranked instead
        if not re.search(rf'(?m)^\s*(?:#\[no_mangle\]\s*)?pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{re.escape(_rn(f))}\b', rust_text or ""):
            alternatives.append({"fn": f, "excluded": "not public in the translation (C static kept private); the harness cannot call it"})
            continue
        rret = _norm_ty(gdh.parse_rust_ret_type(rust_text, _rn(f)) or "", rust_aliases)
        owner_kind, owner_rust = _producer_owner(rret, tname)
        if owner_kind is None:
            alternatives.append({"fn": f, "excluded": f"returns {rret or 'nothing'} in Rust; the bridge owns a "
                                                      f"produced object either as a raw pointer to {tname} or as "
                                                      f"Option<Box<T>> (nullable owned object), and this is neither"})
            continue
        # rule 4: every parameter plannable by the existing InputPlan (scalars, strings, buffers --
        # the cJSON generalisation: `cJSON_Parse(const char*)` is a producer, not just scalar-only ones)
        frt = gdh.parse_rust_param_types(rust_text, _rn(f))
        pk = (str(cc_dir), f)
        if pk not in _PRODUCER_PLAN_CACHE:
            _PRODUCER_PLAN_CACHE[pk] = build_plan(cc_dir, f, policy, rust_types=(frt or None),
                                                  rust_aliases=rust_aliases, rust_text=rust_text,
                                                  allow_producer=False)
        fplan = copy.deepcopy(_PRODUCER_PLAN_CACHE[pk])     # the caps below mutate it
        if fplan.status != "planned":
            alternatives.append({"fn": f, "excluded": "its own plan fails: " + "; ".join(fplan.failures)[:160]})
            continue
        # determinism (section 3)
        seed_reset = "none"
        if f in rand_fns:
            if re.search(r'\b(?:libc::)?s?rand\s*\(', rust_text) or re.search(r'fn\s+s?rand\s*\(', rust_text):
                seed_reset = "libc"
            else:
                alternatives.append({"fn": f, "excluded": "reaches rand() in C but the Rust side has no libc randomness to re-seed"})
                continue
        # producer scalars: the producer's own guards, then the global producer cap (section 4);
        # then the producer's inputs are lowered by the SAME lowering every boundary gets
        fq = {q["name"]: q for q in fparams}
        for s in fplan.inputs:
            d = s.setdefault("detail", {}) if isinstance(s.get("detail"), dict) else {}
            s["detail"] = d
            if s["c_decoder"] in ("scalar", "bounded_scalar"):
                hi = d.get("max")
                d["max"] = policy.producer_scalar_max if hi is None else min(int(hi), policy.producer_scalar_max)
                d["min"] = max(int(d.get("min") or 0), 0)
                d.setdefault("evidence", []).append(_ev_policy(
                    "policy_producer_scalar_cap", "producer_scalar_max",
                    f"{s['param']} is a producer scalar; the produced object's allocation and the "
                    f"target's array extents are expressions over it, so it is capped at {policy.producer_scalar_max}"))
                s["c_decoder"] = "bounded_scalar"
        try:
            lowered = lower_to_schema(fplan, fparams, f, "ptr", policy)["params"]
        except LoweringError as e:
            alternatives.append({"fn": f, "excluded": f"its inputs have no lowering: {e}"[:160]})
            continue
        surface = sum(1 for s in fplan.inputs if s["c_decoder"] != "null_pointer")
        viable.append({"fn": f, "nparams": len(fparams), "surface": surface,
                       "owner": owner_kind, "owner_rust": owner_rust,
                       "reach": len(reachable_functions(cc_dir, f)),
                       "driver": driver_evidence(cc_dir, entry).get(f, 0),
                       "lowered": lowered, "frt": frt or None, "seed_reset": seed_reset})
    if not viable:
        # The abstention names the extension point (docs/construction_recipe_plugin_plan.md
        # section 5): a resource-realization plugin may declare the in-place initializer the
        # library establishes this resource with; build_plan consults it after this return.
        why = "; ".join(f"{x['fn']}: {x['excluded']}" for x in alternatives if x["fn"] in considered)
        return None, base_reason + f"; no producer returns {tname}*" + (f" ({why})" if why else "") \
            + "; no in-place initializer declared"
    # Ordering (docs/producer_bridge_pilot.md section 2, cJSON generalisation): a producer whose
    # object carries fuzz-controlled state first (cJSON_Parse's string over CreateObject's nothing),
    # then the one the shipped drivers feed to this target, then the fewest parameters. Every
    # candidate is a legal sequence; the order decides what gets explored, and it is recorded.
    # Order: the producer whose body reaches the most of the library (cJSON_Parse reaches the whole
    # parser and can build any node; cJSON_CreateString reaches two helpers), then the one with
    # fuzz-controlled inputs, then the one the shipped drivers feed to this target, then the
    # fewest parameters. All derivable from the artifact; every candidate is a legal sequence.
    # A RAW-pointer producer outranks a boxed one for the same type, before every other key: it is
    # the mechanical shape, it keeps the harness's sequence identical to the C one, and fixing the
    # order this way proves that adding the boxed family cannot move a producer choice that was
    # already made (CROWN's quadtree_node_t has both: `quadtree_node_with_bounds` raw and
    # `quadtree_node_new` boxed).
    viable.sort(key=lambda v: (v["owner"] != "raw", -v["reach"], -v["surface"], -v["driver"],
                               v["nparams"], v["fn"]))
    chosen = viable[0]
    for v in viable[1:]:
        alternatives.append({"fn": v["fn"], "excluded": f"ranked below {chosen['fn']}: reaches {v['reach']} fns "
                                                       f"(vs {chosen['reach']}), fuzz surface {v['surface']} "
                                                       f"(vs {chosen['surface']}), driver evidence {v['driver']} "
                                                       f"(vs {chosen['driver']}), params {v['nparams']} (vs {chosen['nparams']})"})
    f = chosen["fn"]
    # destructor (section 2): takes exactly T*, returns void, reaches free() -- one per type
    dk = (str(cc_dir), tname)
    if dk not in _DESTRUCTOR_CACHE:
        _DESTRUCTOR_CACHE[dk] = None
        for g in _all_entries(cc_dir):
            if g not in free_fns:
                continue
            try:
                gparams, gret, _x, gdesc = _sig(cc_dir, g)
            except SystemExit:
                continue
            if gret == "void" and len(gparams) == 1 and gparams[0].get("kind") == "ptr_struct_nonpod" \
                    and gparams[0]["struct"].get("name") == tname and _rust_fn_exists(rust_text, g):
                _DESTRUCTOR_CACHE[dk] = g
                break
    destructor = _DESTRUCTOR_CACHE[dk] if _DESTRUCTOR_CACHE[dk] not in (entry, f) else None
    # The target IS the type's destructor (cJSON_Delete, genann_free): after it the object is gone
    # on both sides, so nothing may read it -- no post-state canonicalisation, no second free;
    # only how the two sides terminated is compared. NOTE the destructor rule itself is
    # "takes exactly T*, returns void, reaches free()" -- an assumption checked by hand on
    # genann_free and cJSON_Delete, not an ownership inference (docs/producer_bridge_pilot.md).
    consumed = (entry == _DESTRUCTOR_CACHE[dk])
    # Cleanup of a BOXED owner (family rule 5: move at most once, never a double free). The C side
    # always calls its own destructor when there is one. On the Rust side the box is moved into a
    # CONSUMING destructor if the translation has one; otherwise the box's own Drop is the analogue
    # of the C free and the destructor is not called at all -- calling a borrowing destructor and
    # then dropping could free the same allocation twice.
    cleanup = "rust_destructor"
    if chosen["owner"] == "opt_box":
        d0 = ""
        if destructor:
            drt = gdh.parse_rust_param_types(rust_text, _rn(destructor)) or []
            d0 = _norm_ty(drt[0], rust_aliases) if drt else ""
        orx = re.escape(chosen["owner_rust"])
        if re.fullmatch(rf"Option<Box<{orx}(?:<[^<>]*>)?>>", d0):
            cleanup = "rust_consuming_option_box"
        elif re.fullmatch(rf"Box<{orx}(?:<[^<>]*>)?>", d0):
            cleanup = "rust_consuming_box"
        else:
            cleanup = "drop"
    detail = {"producer": f, "producer_lowered": chosen["lowered"],
              "producer_owner": chosen["owner"], "producer_rust_owner": chosen["owner_rust"],
              "cleanup": cleanup,
              "consumed_by_target": consumed,
              "producer_rust_types": chosen["frt"],
              "destructor": destructor,
              "lifecycle": "init -> target -> free" if destructor else "not claimed (no destructor found)",
              "seed_reset": chosen["seed_reset"], "seed": 42,
              "struct": tname, "c_struct": cname, "const": bool(param.get("const")),
              "elem": tname,
              "producer_evidence": ("sole candidate under rules 1-4" if len(viable) == 1 else
                                    f"ranked first of {len(viable)}: reaches {chosen['reach']} fns, fuzz surface "
                                    f"{chosen['surface']}, driver evidence {chosen['driver']}, params {chosen['nparams']}"),
              "producer_alternatives": alternatives}
    spec = InputSpec(param=param["name"], c_decoder="produced_object", detail=detail,
                     evidence=[_ev_policy("producer_bridge", "producer_scalar_max",
                                          f"{param['name']} is built by {f}() on each side; "
                                          f"see docs/producer_bridge_pilot.md")])
    return spec, None


# ---------------------------------------------------------------------------
# Resource-realization plugins (docs/construction_recipe_plugin_plan.md).
#
# A sibling materialization of the producer bridge: the resource is not RETURNED by a producer
# but ESTABLISHED IN PLACE by an initializer over side-local storage the harness owns
# (`stack T -> init(T*) -> target(T*) -> cleanup(T*)`).  The manifest (realization_plugin.py)
# names the type, the lifecycle functions and the argument views; everything below is the
# planner's own verification of that declaration against the C AST and the translation, and the
# lowering into the SAME plan concepts the producer bridge uses (PRODUCER / FREE phases, the C UB
# gate around the C-side calls).  Nothing here interpolates manifest text into generated code:
# the emitter receives identifiers that were checked against the translation and view NAMES from
# the closed vocabulary.
# ---------------------------------------------------------------------------
# The harness's C-side storage for a realized resource is a `[u64; N]` (8-byte aligned).
_REALIZED_STORAGE_ALIGN = 8


def _c_param_layout(cc_dir: Path, entry: str, idx: int) -> tuple[int, int, dict] | None:
    """(sizeof, alignof, evidence) of the pointee of the entry's idx-th parameter, from the C AST."""
    cur, _tu = entry_cursor(cc_dir, entry)
    if cur is None:
        return None
    args = list(cur.get_arguments())
    if idx >= len(args):
        return None
    t = args[idx].type
    seen = 0
    while t.kind in (TypeKind.TYPEDEF, TypeKind.ELABORATED) and seen < 8:
        t = t.get_canonical()
        seen += 1
    if t.kind != TypeKind.POINTER:
        return None
    pointee = t.get_pointee()
    size, align = pointee.get_size(), pointee.get_align()
    if size <= 0 or align <= 0:
        return None
    return size, align, _ev("c_struct_layout_from_ast", args[idx],
                            f"sizeof({pointee.spelling}) = {size}, alignof = {align} (libclang)")


_MEMBER_CACHE: dict = {}


def _member_access(fn_cursor, pname: str, cc_dir: Path | None = None, depth: int = 0) -> dict:
    """{top-level field: {read, written, address_taken, partial_write, partial_address, ev}} for the
    accesses `pname->field...` in the function's body, plus `all_established` when the body zeroes
    the whole object (memset/bzero). Conservative: a write or address-of that reaches only a
    SUB-field (`p->a.b = x`, `&p->a.b`) is recorded as partial and does not establish `a`.
    Depth-1 callee summaries: a callee that receives `pname` itself contributes its own accesses."""
    key = (str(fn_cursor.translation_unit.spelling), fn_cursor.spelling, pname, depth)
    if key in _MEMBER_CACHE:
        return _MEMBER_CACHE[key]
    out: dict = {}

    def note(field: str, ctx: str, node, top: bool):
        d = out.setdefault(field, {"read": False, "written": False, "address_taken": False,
                                   "partial_write": False, "partial_address": False, "ev": None})
        if ctx == "read":
            d["read"] = True
        elif ctx == "write":
            d["written" if top else "partial_write"] = True
        elif ctx == "addr":
            d["address_taken" if top else "partial_address"] = True
        if d["ev"] is None:
            d["ev"] = _ev("member_access_in_body", node,
                          f"{pname}->{field} {ctx}{'' if top else ' (sub-field)'}")

    def is_param(expr) -> bool:
        e = _peel(expr)
        if e is not None and e.kind == CursorKind.UNARY_OPERATOR:      # `(*p).f`
            toks = [t.spelling for t in e.get_tokens()]
            ch = list(e.get_children())
            if toks and toks[0] == "*" and ch:
                e = _peel(ch[0])
        n, r = _ref_name(e) if e is not None else (None, None)
        return n == pname and r is not None and r.kind == CursorKind.PARM_DECL

    def walk(node, ctx: str, top: bool):
        kind = node.kind
        kids = list(node.get_children())
        if kind == CursorKind.MEMBER_REF_EXPR and kids:
            if is_param(kids[0]):
                note(node.spelling, ctx, node, top)
            else:
                walk(kids[0], ctx, False)
            return
        if kind == CursorKind.BINARY_OPERATOR and _binop(node) in _ASSIGN_OPS and len(kids) == 2:
            walk(kids[0], "write", True)
            walk(kids[1], "read", True)
            return
        if kind == CursorKind.COMPOUND_ASSIGNMENT_OPERATOR and len(kids) == 2:
            walk(kids[0], "read", True)          # `p->f += x` reads f before it writes it
            walk(kids[0], "write", True)
            walk(kids[1], "read", True)
            return
        if kind == CursorKind.UNARY_OPERATOR and kids:
            toks = [t.spelling for t in node.get_tokens()]
            if toks and toks[0] == "&":
                walk(kids[0], "addr", True)
                return
            if "++" in toks or "--" in toks:
                walk(kids[0], "read", True)
                walk(kids[0], "write", True)
                return
        if kind == CursorKind.ARRAY_SUBSCRIPT_EXPR and len(kids) == 2:
            walk(kids[0], ctx, False)
            walk(kids[1], "read", True)
            return
        if kind == CursorKind.CALL_EXPR:
            callee = node.spelling or ""
            args = list(node.get_arguments())
            if callee in ("memset", "bzero") and args and is_param(args[0]):
                out["all_established"] = _ev("whole_object_zeroed_in_body", node,
                                             f"{callee}({pname}, ..) establishes every byte")
            elif depth < 1 and cc_dir is not None and callee:
                cdef = gdh.definition_index(cc_dir).get(callee)
                if cdef is not None:
                    cparams = [a.spelling for a in cdef[0].get_arguments()]
                    for q, a in zip(cparams, args):
                        if is_param(a):
                            sub = _member_access(cdef[0], q, cc_dir, depth + 1)
                            for f2, d2 in sub.items():
                                if f2 == "all_established":
                                    out.setdefault("all_established", d2)
                                    continue
                                d = out.setdefault(f2, {"read": False, "written": False,
                                                        "address_taken": False, "partial_write": False,
                                                        "partial_address": False, "ev": None})
                                for k in ("read", "written", "address_taken", "partial_write",
                                          "partial_address"):
                                    d[k] = d[k] or d2[k]
                                if d["ev"] is None:
                                    d["ev"] = _ev("member_access_via_callee", node,
                                                  f"{pname} is passed to {callee}({q}), which "
                                                  f"accesses {q}->{f2}")
        for c in kids:
            if kind in _PEEL:
                walk(c, ctx, top)
            else:
                walk(c, "read", True)

    body = next((c for c in fn_cursor.get_children() if c.kind == CursorKind.COMPOUND_STMT), None)
    if body is not None:
        walk(body, "read", True)
    _MEMBER_CACHE[key] = out
    return out


def _split_top_level(s: str) -> list[str]:
    """Split on commas outside <> [] (), treating `->` as an arrow, not a closing bracket."""
    parts, depth, cur, i = [], 0, "", 0
    while i < len(s):
        ch = s[i]
        if ch == "-" and s[i + 1:i + 2] == ">":
            cur += "->"
            i += 2
            continue
        if ch in "<[(":
            depth += 1
        elif ch in ">])":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
        i += 1
    if cur.strip():
        parts.append(cur)
    return [p.strip() for p in parts if p.strip()]


def _rust_struct_fields(rust_text: str, name: str) -> list[tuple[str, str]] | None:
    """[(field, type)] of `pub struct/union name { .. }` in the translation, or None."""
    m = re.search(rf'(?m)^\s*pub\s+(?:struct|union)\s+{re.escape(name)}\s*(?:<[^>{{]*>)?\s*\{{',
                  rust_text or "")
    if not m:
        return None
    i, depth, start = m.end(), 1, m.end()
    while i < len(rust_text) and depth:
        if rust_text[i] == "{":
            depth += 1
        elif rust_text[i] == "}":
            depth -= 1
        i += 1
    body = rust_text[start:i - 1]
    body = re.sub(r"(?m)^\s*#\[[^\]]*\]\s*$", "", body)          # per-field attributes
    body = re.sub(r"//[^\n]*", "", body)
    out = []
    for part in _split_top_level(body):
        part = re.sub(r"^\s*pub(?:\([^)]*\))?\s+", "", part)
        if ":" not in part:
            continue
        fname, fty = part.split(":", 1)
        out.append((fname.strip(), fty.strip()))
    return out


def _rust_zeroable(rust_text: str, ty: str, aliases: dict | None, seen: set | None = None) -> str | None:
    """Reason all-zero bytes are NOT known to be a valid value of `ty` (None when they are).

    The harness's side-local Rust storage is `core::mem::zeroed()`; that is sound only for a type
    every field of which accepts zero: integers, floats, bool, raw pointers, `Option<_>` (None),
    arrays of those, and translation-defined structs/unions of those.  A `Box`, a reference, a
    `Vec`/`String`, a `NonNull` or a bare fn pointer has no zero value, so such a translated
    representation is refused rather than fabricated."""
    seen = seen if seen is not None else set()
    t = _norm_ty(ty, aliases)
    if t in _INT_RUST or t in _FLOAT_RUST or t in ("bool", "()", "c_void"):
        return None
    if t.startswith("*mut") or t.startswith("*const"):
        return None
    if t.startswith("Option<") or t.startswith("Option::<"):
        return None
    m = re.fullmatch(r"\[(.+);([^;\]]+)\]", t)
    if m:
        return _rust_zeroable(rust_text, m.group(1), aliases, seen)
    if t.startswith(("Box<", "&", "Vec<", "String", "NonNull<", "fn(", "unsafefn", "unsafeextern",
                     "extern", "Rc<", "Arc<", "(")):
        return f"{ty}: no all-zero value (owning/borrowing/non-nullable representation)"
    if not re.fullmatch(r"[A-Za-z_]\w*", t):
        return f"{ty}: unrecognised type shape"
    if t in seen:
        return None
    fields = _rust_struct_fields(rust_text, t)
    if fields is None:
        return f"{ty}: not a struct/union defined in the translation"
    seen.add(t)
    for fname, fty in fields:
        r = _rust_zeroable(rust_text, fty, aliases, seen)
        if r:
            return f"{t}.{fname} -> {r}"
    return None


_RUST_PUB_FN = r'(?m)^\s*(?:#\[no_mangle\]\s*)?pub\s+(?:unsafe\s+)?(?:extern\s+"C"\s+)?fn\s+{name}\b'


def _plan_realization(cc_dir: Path, param: dict, pidx: int, entry: str, policy: GeneratorPolicy,
                      rust_text: str | None, rust_aliases: dict | None,
                      rust_types: list[str] | None, facts: BodyFacts, generic_reason: str):
    """Consult the accepted resource-realization manifests for `param` (a `T*` whose struct carries
    pointers) AFTER the generic producer search abstained. Returns (InputSpec, None, origin) or
    (None, "construction unsupported: ..", None). Checks are numbered as in
    docs/construction_recipe_plugin_plan.md section 5."""
    import realization_plugin as rp
    st = param["struct"]
    tname, cname = st["name"], st.get("c_name", st["name"])
    rejected: list[dict] = [{"kind": "generic-producer", "reason": generic_reason}]
    applicable = [m for m in _REALIZATIONS if m.c_type == tname]
    for m in _REALIZATIONS:
        if m.c_type != tname:
            rejected.append({"kind": "plugin", "plugin": m.ident,
                             "reason": f"declares resource {m.c_type}, not {tname}"})
    if not applicable:
        return None, None, None
    reasons: list[str] = []
    todo = [(m, rb) for m in applicable for rb in m.rust]
    for k, (m, rb) in enumerate(todo):
        label = f"{m.ident}" + (f"[{rb.binding}]" if rb.binding else "")
        try:
            spec, origin = _realize_with(cc_dir, m, rb, param, pidx, entry, policy, rust_text,
                                         rust_aliases, rust_types, facts, list(rejected))
            for m2, rb2 in todo[k + 1:]:       # declared but never tried: recorded, not silently dropped
                origin["rejected_alternatives"].append({
                    "kind": "plugin-binding",
                    "plugin": f"{m2.ident}" + (f"[{rb2.binding}]" if rb2.binding else ""),
                    "reason": f"not tried: the earlier declared binding {label} fits this translation"})
            return spec, None, origin
        except _Unsupported as e:
            reasons.append(f"{label}: {e}")
            rejected.append({"kind": "plugin-binding", "plugin": label, "reason": str(e)})
    return None, "construction unsupported: " + " | ".join(reasons), None


class _Unsupported(Exception):
    pass


def _realize_with(cc_dir: Path, m, rb, param: dict, pidx: int, entry: str, policy: GeneratorPolicy,
                  rust_text: str | None, rust_aliases: dict | None, rust_types: list[str] | None,
                  facts: BodyFacts, rejected: list[dict]):
    """One manifest x one declared Rust binding `rb`; raises _Unsupported with the failing check."""
    import realization_plugin as rp
    st = param["struct"]
    tname, cname = st["name"], st.get("c_name", st["name"])
    pname = param["name"]
    validation: list[dict] = []
    assumptions: list[str] = []

    def ok(check: int, what: str, ev=None):
        validation.append({"check": check, "what": what, "ok": True,
                           **({"evidence": ev} if ev is not None else {})})

    def fail(check: int, why: str):
        validation.append({"check": check, "what": why, "ok": False})
        raise _Unsupported(f"check {check}: {why}")

    if rust_text is None:
        fail(2, "no Rust text to verify the translated representation against")
    rn_rust = _rn  # renames map (matcher output for renaming translators)

    # --- check 1: the C target parameter resolves to the declared C type and role -------------
    if m.c_type != tname:
        fail(1, f"manifest resource {m.c_type} is not the parameter's type {tname}")
    # The target must not be the lifecycle function itself: the harness would call c_<name> both as the
    # initializer/cleanup and as the target and define the extern twice (E0428 on lodepng_state_init in
    # the 2026-09-11 pilot); a lifecycle function is exercised through the targets it serves.
    if entry in (m.c.initializer, m.c.cleanup):
        fail(1, f"the target {entry} is the realization's own lifecycle function; it cannot be both the "
                f"initializer/cleanup and the target of one harness")
    c_target_view = m.c.target_view
    if rp.c_view_is_const(c_target_view) != bool(param.get("const")):
        fail(1, f"C target takes {'const ' if param.get('const') else ''}{cname}* but the manifest "
                f"declares target_view {c_target_view!r}")
    layout = _c_param_layout(cc_dir, entry, pidx)
    if layout is None:
        fail(1, f"could not derive sizeof/alignof of {cname} from the C AST")
    c_size, c_align, layout_ev = layout
    if c_align > _REALIZED_STORAGE_ALIGN:
        fail(1, f"{cname} needs {c_align}-byte alignment; the harness's side-local storage is "
                f"{_REALIZED_STORAGE_ALIGN}-byte aligned")
    ok(1, f"C target parameter {pname} is {cname}* ({'const' if param.get('const') else 'mutable'}); "
          f"sizeof={c_size} alignof={c_align}; view {c_target_view}", layout_ev)

    # --- check 2: the Rust target parameter resolves to the declared Rust type and view --------
    if rust_types is None or pidx >= len(rust_types):
        fail(2, f"the translation's signature of {rn_rust(entry)} has no parameter at position {pidx}")
    r_target_ty = _norm_ty(rust_types[pidx], rust_aliases)
    r_view = rb.target_view
    if not rp.rust_view_pattern(r_view, rb.type).fullmatch(r_target_ty):
        fail(2, f"Rust target parameter {pidx} is {rust_types[pidx]!r}, which is not the declared "
                f"view {r_view!r} of {rb.type}")
    zero_why = _rust_zeroable(rust_text, rb.type, rust_aliases)
    if zero_why:
        fail(2, f"Rust storage for {rb.type} cannot be zero-established: {zero_why}")
    ok(2, f"Rust target parameter {pidx} is {rust_types[pidx]} = view {r_view} of {rb.type}; "
          f"all-zero bytes are a valid {rb.type} (every field is scalar / raw pointer / Option / "
          f"array / translation struct of those)")

    # --- check 3: initializer and cleanup exist on both sides ---------------------------------
    if not m.c.initializer or not rb.initializer:
        fail(3, f"the manifest declares no in-place initializer for {tname} on "
                f"{'C' if not m.c.initializer else 'Rust'}: the plugin supplies the materialization, "
                f"not the views")
    if bool(m.c.cleanup) != bool(rb.cleanup):
        fail(3, "cleanup is declared on one side only; the lifecycle must be claimed on both or neither")
    hooks: dict = {}
    defs = gdh.definition_index(cc_dir)
    for role, cfn, rfn in (("initializer", m.c.initializer, rb.initializer),
                           ("cleanup", m.c.cleanup, rb.cleanup)):
        if not cfn:
            continue
        if cfn not in defs:
            fail(3, f"C {role} {cfn} is not defined in the pair's translation unit")
        if not _rust_fn_exists(rust_text, rfn):
            fail(3, f"Rust {role} {rfn} is not present in the translation")
        if not re.search(_RUST_PUB_FN.format(name=re.escape(rn_rust(rfn))), rust_text):
            fail(3, f"Rust {role} {rfn} is not public in the translation; the harness cannot call it")
        hooks[role] = {"c": cfn, "rust": rn_rust(rfn),
                       "c_location": _ev("lifecycle_function_defined", defs[cfn][0], f"{cfn} definition")}
    for cf in m.requires.get("c_functions", []):
        if cf not in defs:
            fail(3, f"[requires] C function {cf} is not defined in the pair's translation unit")
    for rf in m.requires.get("rust_functions", []):
        if not _rust_fn_exists(rust_text, rf):
            fail(3, f"[requires] Rust function {rf} is not present in the translation")
    ok(3, "initializer" + (" and cleanup" if m.c.cleanup else "") + " defined on both sides; "
          f"[requires] satisfied: C {m.requires.get('c_functions')}, Rust {m.requires.get('rust_functions')}")

    # --- check 4 (+6): the normalised signatures accept the declared side-specific views ------
    # v1 realizes SINGLE-argument lifecycle functions returning void: `init(T*)`. Every input of
    # the initializer is then constructible by definition (check 6), and nothing about the call
    # is compared, so a value-returning initializer is refused rather than left unobserved.
    for role, cview, rview in (("initializer", m.c.initializer_view, rb.initializer_view),
                               ("cleanup", m.c.cleanup_view, rb.cleanup_view)):
        h = hooks.get(role)
        if not h:
            continue
        try:
            hp_, hret, _f, _d = _sig(cc_dir, h["c"])
        except SystemExit as e:
            fail(4, f"C {role} {h['c']}: signature: {e}")
        if hret != "void":
            fail(4, f"C {role} {h['c']} returns {hret}; v1 realizes void lifecycle functions only")
        if len(hp_) != 1:
            fail(4, f"C {role} {h['c']} takes {len(hp_)} parameters; v1 realizes `{role}({cname}*)` only")
        q = hp_[0]
        if q.get("kind") not in ("ptr_struct_nonpod", "ptr_struct") or q["struct"].get("name") != tname:
            fail(4, f"C {role} {h['c']} does not take {cname}*")
        if q.get("const") and not rp.c_view_is_const(cview):
            fail(4, f"C {role} {h['c']} takes const {cname}* but the manifest declares view {cview!r}")
        if rp.c_view_is_const(cview) and not q.get("const"):
            fail(4, f"C {role} {h['c']} takes {cname}* (mutable) but the manifest declares view {cview!r}")
        rts = gdh.parse_rust_param_types(rust_text, h["rust"])
        if len(rts) != 1:
            fail(4, f"Rust {role} {h['rust']} takes {len(rts)} parameters; v1 realizes one-argument "
                    f"lifecycle functions only")
        rt = _norm_ty(rts[0], rust_aliases)
        if not rp.rust_view_pattern(rview, rb.type).fullmatch(rt):
            fail(4, f"Rust {role} {h['rust']} takes {rts[0]!r}, which is not the declared view "
                    f"{rview!r} of {rb.type}")
        rret = gdh.parse_rust_ret_type(rust_text, h["rust"])
        if rret not in (None, "()"):
            fail(4, f"Rust {role} {h['rust']} returns {rret}; v1 realizes void lifecycle functions only")
        h["c_view"], h["rust_view"] = cview, rview
        h["c_signature"] = f"void {h['c']}({'const ' if q.get('const') else ''}{cname}*)"
        h["rust_signature"] = f"fn {h['rust']}({rts[0]})"
        ok(4, f"{role}: C `{h['c_signature']}` accepts view {cview}; Rust `{h['rust_signature']}` "
              f"accepts view {rview}")
    ok(6, "the initializer's only input is the resource itself (v1: single-argument lifecycle "
          "functions), so its inputs are constructible by construction")

    # --- check 5: the target's view neither clones, leaks nor narrows ------------------------
    if rp.RUST_VIEWS[r_view]["owning"]:
        fail(5, f"view {r_view} would move the resource into the target")
    ok(5, f"target view {r_view} lends the side-local object (non-owning); C passes its address; "
          f"no narrowing: the resource is not decoded from fuzz bytes")

    # --- check 7: deterministic initialization (or the producer bridge's seed-reset rule) -----
    rand_fns, eff_fns = random_functions(cc_dir), effectful_functions(cc_dir)
    seed_reset = "none"
    for role, h in hooks.items():
        if h["c"] in eff_fns:
            fail(7, f"C {role} {h['c']} reaches an effectful call (file/process/network)")
        if h["c"] in rand_fns:
            if re.search(r'\b(?:libc::)?s?rand\s*\(', rust_text) or re.search(r'fn\s+s?rand\s*\(', rust_text):
                seed_reset = "libc"
            else:
                fail(7, f"C {role} {h['c']} reaches rand() but the Rust side has no libc randomness to re-seed")
    ok(7, "initializer" + (" and cleanup" if m.c.cleanup else "") + " reach no randomness and no "
          "effectful call" if seed_reset == "none" else
          "initializer reaches rand(); both sides re-seed with the producer bridge's libc rule")

    # --- check 8: cleanup cannot consume or free the resource before the target call ---------
    # The storage is the harness's own (stack): a lifecycle function or the target that hands the
    # resource POINTER to free()/realloc() would free memory nobody allocated. Cleanup is emitted
    # only after the target, in the FREE phase, by construction of the lowering.
    consumed = (entry == m.c.cleanup)
    freed = facts.escape_callees.get(pname, set()) & {"free", "realloc"}
    if freed:
        fail(8, f"the target passes {pname} itself to {sorted(freed)}; a side-local resource cannot be freed")
    for role, h in hooks.items():
        cur = defs[h["c"]][0]
        cp = [a.spelling for a in cur.get_arguments()]
        try:
            hf = BodyAnalyzer(cur, set(cp)).run()
        except Exception as e:
            fail(8, f"C {role} {h['c']}: body analysis failed: {e}")
        f2 = hf.escape_callees.get(cp[0], set()) & {"free", "realloc"}
        if f2:
            fail(8, f"C {role} {h['c']} passes the resource pointer itself to {sorted(f2)}")
    ok(8, "neither the target nor a lifecycle function passes the resource pointer to free()/realloc()"
          + (f"; the target IS the cleanup, so no second cleanup is emitted" if consumed else
             "; cleanup is emitted after the target (FREE phase)"))
    assumptions.append("plugin-owned: the cleanup releases only members the initializer/target "
                       "allocated, never the resource's own storage (depth-1 escape check only)")

    # --- check 9: conservative BodyFacts field check --------------------------------------
    tcur = entry_cursor(cc_dir, entry)[0]
    icur = defs[m.c.initializer][0]
    ip = [a.spelling for a in icur.get_arguments()][0]
    t_acc = _member_access(tcur, pname, cc_dir)
    i_acc = _member_access(icur, ip, cc_dir)
    target_fields = sorted(f for f in t_acc if f != "all_established")
    if "all_established" in i_acc:
        established = set(target_fields)
        via_callee: list[str] = []
    else:
        established = {f for f, d in i_acc.items() if f != "all_established" and (d["written"] or d["address_taken"])}
        via_callee = sorted(f for f, d in i_acc.items() if f != "all_established"
                            and d["address_taken"] and not d["written"])
    unestablished = [f for f in target_fields if f not in established]
    if unestablished:
        fail(9, f"the target reads {pname}->{{{', '.join(unestablished)}}} but {m.c.initializer} "
                f"neither writes nor hands out the address of {'it' if len(unestablished) == 1 else 'them'}")
    ok(9, f"target accesses {pname}->{{{', '.join(target_fields)}}}; every one is written by "
          f"{m.c.initializer} or has its address passed to a callee there"
          + (f" (via callee: {', '.join(via_callee)})" if via_callee else ""),
       [d["ev"] for f, d in sorted(t_acc.items()) if f != "all_established" and d.get("ev")])
    assumptions.append(f"plugin-owned: {m.c.initializer} establishes the {tname} invariant"
                       + (f"; fields {', '.join(via_callee)} are established by the callees it passes "
                          f"their addresses to" if via_callee else "")
                       + "; storage is zero-filled on both sides before the initializer runs, so an "
                         "unestablished field reads as zero (defined), and this check guards the "
                         "invariant, not definedness")

    # --- check 10: the plugin does not override a usable generic plan ------------------------
    ok(10, "consulted only after the generic producer search abstained: " + rejected[0]["reason"])

    resource = {"c": {"type": cname, "parameter_type": m.c.parameter_type, "storage": m.c.storage,
                      "size": c_size, "align": c_align},
                "rust": {"type": rb.type, "storage": rb.storage,
                         "zero_established": True}}
    views = {"owner": "harness (side-local)",
             "c": {"initializer": m.c.initializer_view, "target": c_target_view,
                   "cleanup": m.c.cleanup_view},
             "rust": {"initializer": rb.initializer_view, "target": r_view,
                      "cleanup": rb.cleanup_view}}
    origin = rp.origin_record(m, resource, views, hooks, validation, assumptions, rejected)
    origin["binding"] = rb.binding
    detail = {"realization": "in_place_initializer",
              "plugin": m.identity(),
              "struct": tname, "c_struct": cname, "rust_struct": rb.type,
              "c_size": c_size, "c_align": c_align, "storage": "stack",
              "initializer": {"c": hooks["initializer"]["c"], "rust": hooks["initializer"]["rust"],
                              "c_view": m.c.initializer_view, "rust_view": rb.initializer_view},
              "cleanup": ({"c": hooks["cleanup"]["c"], "rust": hooks["cleanup"]["rust"],
                           "c_view": m.c.cleanup_view, "rust_view": rb.cleanup_view}
                          if "cleanup" in hooks else None),
              "target_view": {"c": c_target_view, "rust": r_view},
              "consumed_by_target": consumed,
              "lifecycle": ("init -> target -> cleanup" if "cleanup" in hooks and not consumed else
                            "init -> target (target is the cleanup)" if consumed else
                            "not claimed (no cleanup declared)"),
              "seed_reset": seed_reset, "seed": 42,
              "const": bool(param.get("const")), "elem": tname}
    spec = InputSpec(param=pname, c_decoder="realized_resource", detail=detail,
                     evidence=[{"rule": "resource_realization_plugin", "file": m.path, "line": 0, "col": 0,
                                "snippet": f"{m.ident} sha256:{m.content_hash[:16]}",
                                "note": f"{pname} is established in place by {m.c.initializer}() on each "
                                        f"side; see docs/construction_recipe_plugin_plan.md"}, layout_ev])
    return spec, origin


def build_plan(cc_dir: Path, entry: str, policy: GeneratorPolicy = POLICY,
               rust_types: list[str] | None = None,
               rust_aliases: dict | None = None,
               rust_text: str | None = None,
               allow_producer: bool = True) -> HarnessPlan:
    """InputPlan -> HarnessPlan.  No schema is read; nothing is hand-written.

    The return value is NOT a construction gate.  What can be compared about it is decided by the
    fixed ladder in the emitters: void -> nothing, scalar -> value, pointer -> nullness (or a user
    plugin).  A boundary is rejected only when its INPUT cannot be constructed.
    """
    failures: list[str] = []

    def _fail(reason: str) -> HarnessPlan:
        return HarnessPlan(entry, "failed", [], [], [reason], policy.as_dict())

    # a renaming translator's map travels with the pair (also when the generator drives this
    # function directly with --plan, where main() below never runs)
    if not _RENAMES:
        _RENAMES.update(load_renames(Path(cc_dir).parent))
    try:
        params, ret, _fns, ret_desc = gdh.parse_entry_signature(cc_dir, entry, with_return_desc=True,
                                                              allow_nonpod=allow_producer)
    except SystemExit as e:
        return _fail(f"signature: {e}")
    if not params and entry not in _fns:
        return _fail("signature: the entry was not found in the pair's translation unit")
    # A boundary exists only if BOTH sides define it. With `--all` the C side enumerates every
    # function of the TU, including ones the translator's input never had (a C-source version newer
    # than the one it consumed); those planned as C-ABI and then failed at build with E0425.
    if rust_text is not None and not _rust_fn_exists(rust_text, entry):
        return _fail(f"signature: {entry} is not present in the Rust translation (no boundary)")
    if not params:
        # No arguments is not a construction failure: there IS no input to construct. Both sides
        # are called once and compared -- a single deterministic execution decides the boundary.
        return HarnessPlan(entry, "planned", [], [], [], policy.as_dict())

    cur, _tu = entry_cursor(cc_dir, entry)
    if cur is None:
        return _fail("body: no definition found in the pair's compilation database")
    facts = BodyAnalyzer(cur, {p["name"] for p in params},
                         {p["name"] for p in params
                          if p["kind"] == "ptr_ptr" and p.get("elem") in (_INT_RUST | _FLOAT_RUST)}).run()

    iplan = analyze_inputs(params, facts, policy, effectful_functions(cc_dir))
    failures += iplan.failures + facts.unresolved
    # A `T*` whose struct carries pointers is not decodable from bytes; the pilot builds it with
    # the library's own producer (docs/producer_bridge_pilot.md) or fails with the reason.
    n_produced = 0
    origin = None
    for pidx, p in enumerate(params):
        if p.get("kind") == "ptr_struct_nonpod":
            spec, why = _plan_producer(cc_dir, p, entry, policy, rust_text, rust_aliases)
            if spec is None and _REALIZATIONS and allow_producer:
                # Extension point (docs/construction_recipe_plugin_plan.md): a resource-realization
                # plugin may declare the in-place initializer; it is consulted only here, after the
                # generic search abstained, and validated against the C AST and the translation.
                spec, why2, origin2 = _plan_realization(cc_dir, p, pidx, entry, policy, rust_text,
                                                        rust_aliases, rust_types, facts, why)
                if spec is not None:
                    origin = origin2
                elif why2:
                    why = why.replace("; no in-place initializer declared", "") + "; " + why2
            if spec is None:
                failures.append(f"signature: {why}")
            else:
                iplan.specs.append(spec)
                n_produced += 1
    if n_produced >= 2:
        # `cJSON_AddItemToArray(array, item)`: after the call `item` belongs to `array`, and freeing
        # both is a double free -- a harness bug that would masquerade as a crash. Ownership
        # transfer between two produced objects is not derived in this increment, so the boundary
        # is refused rather than guessed (docs/producer_bridge_pilot.md, cJSON generalisation).
        failures.append(f"signature: {n_produced} produced objects in one call; ownership transfer "
                        f"between them cannot be ruled out, so the lifecycle cannot be claimed")
    # Second materialization: the same C-shaped input as Rust arguments. A parameter shape with no
    # lossless bridge is a construction failure -- inputs are not allowed to be approximate.
    failures += apply_rust_bridges(iplan, rust_types, params, rust_aliases)

    DECODABLE = set(_SIGNED) | set(_UNSIGNED) | {"f32", "f64", "bool"}
    for p in params:
        for key in ("rust", "elem"):
            t = p.get(key)
            if t is not None and t not in DECODABLE:
                failures.append(f"decode: {p['name']} has type {t!r}, which the byte cursor "
                                f"cannot construct")

    status = "failed" if failures else "planned"
    return HarnessPlan(
        boundary=entry, status=status,
        inputs=[asdict(s) for s in iplan.specs],
        liveness=iplan.liveness,
        failures=failures,
        policy=policy.as_dict(),
        origin=(origin if status == "planned" else None),
    )


# ---------------------------------------------------------------------------
# Lowering: HarnessPlan -> the generator's internal parameter list
#
# This is the ONLY consumer of a plan.  The result is not a file and not a user-facing format:
# it is the argument the code emitters already take.  Nothing here may invent a fact the plan
# does not carry -- if a lowering has no adapter for a planned entry, that is a generator gap and
# it raises, rather than substituting a guess.
# ---------------------------------------------------------------------------
class LoweringError(RuntimeError):
    pass


def _rust_extent_expr(bound: dict, alloc: int | None, cap: int) -> str:
    """A usize expression for an allocation size, clamped to the policy so a decoded parameter
    can never turn into a multi-gigabyte allocation."""
    if alloc is not None:
        return str(int(alloc))
    if bound is None or bound.get("k") == "unknown":
        raise LoweringError("no extent")
    if bound["k"] == "const":
        return str(max(0, min(int(bound["v"]), cap)))
    if bound["k"] == "param":
        e = f"({bound['p']} as i64)"
        if bound["mul"] != 1:
            e = f"({e} * {bound['mul']})"
        if bound["div"] != 1:
            e = f"({e} / {bound['div']})"
        if bound["add"]:
            e = f"({e} + {bound['add']})"
        return f"(({e}).max(0) as usize).min({cap})"
    if bound["k"] == "max":
        parts = [_rust_extent_expr(x, None, cap) for x in bound["of"]]
        out = parts[0]
        for q in parts[1:]:
            out = f"({out}).max({q})"
        return out
    raise LoweringError(f"extent kind {bound['k']}")


def lower_to_schema(plan: HarnessPlan, params: list[dict], program: str, ret_rust: str,
                    policy: GeneratorPolicy = POLICY) -> dict:
    """HarnessPlan -> {"params": [...]} in the generator's ABI order."""
    if plan.status != "planned":
        raise LoweringError(f"{plan.boundary}: plan status is {plan.status}")
    by_param = {i["param"]: i for i in plan.inputs}
    by_name = {p["name"]: p for p in params}
    cap_ptr = {i["param"] for i in plan.inputs if i["c_decoder"] == "capacity_ptr"}

    out = []
    for p in params:                      # strict declaration order == ABI order
        n = p["name"]
        spec = by_param.get(n)
        if spec is None:
            raise LoweringError(f"{plan.boundary}: parameter {n} has no planned adapter")
        a, d = spec["c_decoder"], spec["detail"]
        byte_cap = max(1, policy.max_buffer_bytes // (p.get("elem_w") or 1))
        elems_cap = min(policy.unproven_extent_elems, byte_cap)
        if a == "null_pointer":
            out.append({"name": n, "role": "null_pointer", "decode": "null"})
        elif a == "struct_value":
            if d.get("freed"):
                raise LoweringError(f"{plan.boundary}: {n} is freed by the boundary (no heap-owned struct adapter)")
            out.append({"name": n, "role": "inout_struct" if d.get("written") else "input_struct",
                        "struct_name": p["struct"]["name"],
                        "fields": gdh._struct_fields_to_schema(p["struct"])})
        elif a == "produced_object":
            # The producer's scalars are lowered exactly like any bounded scalar; the generator
            # namespaces them under the object's name so `genann_run(ann, inputs)` and
            # `genann_init(inputs, ...)` cannot collide.
            # The producer's inputs were lowered by lower_to_schema when the producer was chosen;
            # the Rust parameter types ride along by position so the generator can bridge them
            # (a `&str` for `const char*`, a slice for a buffer) exactly as for any target.
            pp = [dict(q) for q in d["producer_lowered"]]
            prt = d.get("producer_rust_types") or []
            if len(prt) == len(pp):
                for q, t in zip(pp, prt):
                    q["rust_pty"] = re.sub(r"&\s*'\w+\s*", "&", t)
            out.append({"name": n, "role": "produced_object", "decode": "producer_call",
                        "owner": d.get("producer_owner", "raw"),
                        "owner_rust": d.get("producer_rust_owner"),
                        "cleanup": d.get("cleanup", "rust_destructor"),
                        "producer": d["producer"], "producer_params": pp,
                        "destructor": d.get("destructor"), "consumed": bool(d.get("consumed_by_target")),
                        "seed_reset": d.get("seed_reset", "none"),
                        "seed": int(d.get("seed", 42)), "const": bool(d.get("const")),
                        "struct": d["struct"], "c_struct": d.get("c_struct", d["struct"]),
                        "rust_pty": spec.get("rust_type"), "bridge": spec.get("rust_bridge")})
        elif a == "realized_resource":
            # Resource-realization plugin (docs/construction_recipe_plugin_plan.md): side-local
            # zero-filled storage on each side, established by the library's own in-place
            # initializer under the manifest's view, handed to the target under the target view,
            # released by the cleanup after the target. Identifiers were verified against the
            # translation; views are names from the closed vocabulary.
            out.append({"name": n, "role": "realized_resource", "decode": "in_place_initializer",
                        "struct": d["struct"], "c_struct": d.get("c_struct", d["struct"]),
                        "rust_struct": d["rust_struct"],
                        "c_size": int(d["c_size"]), "c_align": int(d["c_align"]),
                        "initializer": dict(d["initializer"]),
                        "cleanup": (dict(d["cleanup"]) if d.get("cleanup") else None),
                        "target_view": dict(d["target_view"]),
                        "consumed": bool(d.get("consumed_by_target")),
                        "seed_reset": d.get("seed_reset", "none"), "seed": int(d.get("seed", 42)),
                        "const": bool(d.get("const")),
                        "plugin": dict(d.get("plugin") or {}),
                        "rust_pty": spec.get("rust_type"), "bridge": spec.get("rust_bridge")})
        elif a == "scalar":
            out.append({"name": n, "role": "scalar", "decode": "scalar",
                        "rust": p["rust"], "width": p["w"]})
        elif a == "bounded_scalar" and d.get("max") is None:
            # A one-sided rejection guard (`if (inputs < 1) return 0;`) bounds nothing the decoder
            # can use: values below the bound are legal inputs the callee rejects on both sides.
            # Lower as a full-range scalar rather than inventing an upper bound (genann_init).
            out.append({"name": n, "role": "scalar", "decode": "scalar",
                        "rust": p["rust"], "width": p["w"]})
        elif a == "bounded_scalar":
            out.append({"name": n, "role": "scalar", "decode": "bounded_scalar",
                        "rust": p["rust"], "width": p["w"],
                        "min_value": int(d.get("min") or 0), "max_value": int(d["max"])})
        elif a == "length":
            out.append({"name": n, "role": "length", "decode": "derived_from_buffer",
                        "of_buffer": d["of_buffer"], "rust": p["rust"], "width": p["w"]})
        elif a == "input_string_pointer_table":
            out.append({"name": n, "role": "input_string_pointer_table",
                        "decode": "string_pointer_table",
                        "elem": p["elem"], "elem_width": p["elem_w"],
                        "length_param": d["length_param"], "count_max": int(d["count_max"]),
                        "inner_const": bool(p.get("inner_const")),
                        "mutation": "backing_observable"})
        elif a == "input_string":
            out.append({"name": n, "role": "input_string", "decode": "nul_string",
                        "elem": p["elem"], "elem_width": p["elem_w"],
                        "max_len": int(d.get("max_elems") or byte_cap)})
        elif a == "capacity_ptr":
            out.append({"name": n, "role": "capacity_ptr", "decode": "capacity_ptr_inout",
                        "elem": p["elem"], "elem_width": p["elem_w"]})
        elif a in ("input_buffer", "inout_buffer"):
            # A non-const C buffer gets a SEPARATE allocation per side even when the C body never
            # writes it: the observation plan compares it, and one shared allocation could not
            # tell a write by the translation from a write by the original.
            role = "input_buffer" if (a == "input_buffer" and p.get("const")) else "inout_buffer"
            it = {"name": n, "role": role, "decode": "vector",
                  "elem": p["elem"], "elem_width": p["elem_w"],
                  "length_param": d["length_param"]}
            if d.get("max_elems"):
                it["max_len"] = int(d["max_elems"])
            out.append(it)
        elif a == "output_buffer" and d.get("capacity_param") in cap_ptr:
            out.append({"name": n, "role": "output_buffer", "decode": "vector",
                        "elem": p["elem"], "elem_width": p["elem_w"],
                        "capacity_param": d["capacity_param"],
                        "cap": int(d.get("max_elems") or elems_cap),
                        "observable_length": {"kind": "capacity_ptr_writeback"}})
        elif a in ("output_buffer", "output_array", "input_array", "inout_array"):
            # One uniform adapter: an allocation the harness owns, sized by the plan, filled from
            # the fuzz input exactly when the callee reads it.
            try:
                elems = _rust_extent_expr(d.get("extent"), d.get("alloc_elems"), byte_cap)
            except LoweringError as e:
                raise LoweringError(f"{plan.boundary}: {n}: {e}")
            if a == "output_buffer":      # capacity is a plain scalar parameter
                elems = _rust_extent_expr(b_param(d["capacity_param"]), None, byte_cap)
            out.append({"name": n, "role": "plan_array", "decode": "plan_array",
                        "elem": p["elem"], "elem_width": p["elem_w"],
                        "elems": elems, "fill": "fuzz" if d.get("fills_from_fuzz") else "zero",
                        "const": bool(p.get("const")),
                        "one_elem": spec.get("rust_bridge") == "mut_ref_one"})
        elif a == "buffer_table":
            rows = []
            for r in d["row_specs"]:
                try:
                    elems = _rust_extent_expr(r.get("extent"), r.get("alloc_elems"), byte_cap)
                except LoweringError as e:
                    raise LoweringError(f"{plan.boundary}: {n} row {r['row']}: {e}")
                rows.append({"elems": elems, "fill": "fuzz" if r.get("fills_from_fuzz") else "zero",
                             "written": bool(r.get("written"))})
            out.append({"name": n, "role": "buffer_table", "decode": "buffer_table",
                        "elem": p["elem"], "elem_width": p["elem_w"],
                        "inner_const": bool(p.get("inner_const")), "rows": rows})
        else:
            raise LoweringError(f"{plan.boundary}: no lowering for adapter {a!r} on {n}")
    return {"schema_version": 1, "program": program, "entry": plan.boundary,
            "provenance": "lowered from a generated HarnessPlan; not hand-written",
            "decode_scalars_first": True, "return": {"rust": ret_rust}, "params": out}


def plan_and_lower(cc_dir: Path, entry: str, program: str,
                   policy: GeneratorPolicy = POLICY,
                   rust_types: list[str] | None = None,
                   rust_aliases: dict | None = None,
                   rust_text: str | None = None) -> tuple[HarnessPlan, dict | None]:
    plan = build_plan(cc_dir, entry, policy, rust_types, rust_aliases, rust_text=rust_text)
    if plan.status != "planned":
        return plan, None
    params, ret, _fns = gdh.parse_entry_signature(cc_dir, entry, allow_nonpod=True)
    return plan, lower_to_schema(plan, params, program, ret, policy)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------
_ENTRIES_CACHE: dict = {}


def _all_entries(cc_dir: Path) -> list[str]:
    if str(cc_dir) in _ENTRIES_CACHE:
        return _ENTRIES_CACHE[str(cc_dir)]
    _ENTRIES_CACHE[str(cc_dir)] = _all_entries_uncached(cc_dir)
    return _ENTRIES_CACHE[str(cc_dir)]


def _all_entries_uncached(cc_dir: Path) -> list[str]:
    return sorted(gdh.definition_index(cc_dir))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--pair", required=True, help="pair dir containing build/compile_commands.json")
    ap.add_argument("--entry")
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--entries", help="comma-separated entry list")
    ap.add_argument("--rust", help="the translated .rs (defaults to <pair>/translated/*.rs); the "
                                   "RustBridge is derived from its signatures")
    ap.add_argument("--rust-entry", help="name of the entry in the Rust translation, if renamed")
    ap.add_argument("--no-rust", action="store_true",
                    help="skip bridge derivation (C-ABI assumed) -- diagnostics only")
    ap.add_argument("--realization-plugins", action="append", default=None,
                    help="resource-realization plugin manifest (plugins/<lib>-harness-plan/plugin.toml, "
                         "kind harness-plan-resource-realization); repeatable. Consulted ONLY for a "
                         "boundary the generic planner abstains on with `no producer returns T*; no "
                         "in-place initializer declared`; the plan then records origin=plugin. "
                         "Absent: planning is unchanged. Not the comparator --plugins namespace.")
    ap.add_argument("--out-dir")
    ap.add_argument("--json")
    a = ap.parse_args()

    cc = Path(a.pair) / "build"
    _RENAMES.update(load_renames(Path(a.pair)))
    if a.realization_plugins:
        import realization_plugin as rp
        try:
            set_realizations(rp.load_realization_plugins(a.realization_plugins))
        except rp.RealizationManifestError as e:
            raise SystemExit(str(e))
    if a.all:
        entries = _all_entries(cc)
    elif a.entries:
        entries = [e.strip() for e in a.entries.split(",") if e.strip()]
    elif a.entry:
        entries = [a.entry]
    else:
        ap.error("one of --entry / --entries / --all is required")

    rs_text = None
    if not a.no_rust:
        rp = Path(a.rust) if a.rust else next(iter(sorted((Path(a.pair) / "translated").glob("*.rs"))), None)
        if rp is not None:
            rs_text = rp.read_text(encoding="utf-8", errors="replace")

    outdir = Path(a.out_dir) if a.out_dir else None
    if outdir:
        outdir.mkdir(parents=True, exist_ok=True)
    plans = []
    aliases = rust_type_aliases(rs_text) if rs_text else None
    for e in entries:
        rt = gdh.parse_rust_param_types(rs_text, a.rust_entry or _rn(e)) if rs_text else None
        p = build_plan(cc, e, rust_types=(rt or None), rust_aliases=aliases, rust_text=rs_text)
        plans.append(p)
        if outdir:
            (outdir / f"{e}.plan.json").write_text(json.dumps(plan_dict(p), indent=1) + "\n")
        mark = "OK " if p.status == "planned" else "FAIL"
        print(f"{mark} {e:30s} inputs={len(p.inputs):2d}"
              + (f"  origin=plugin {p.origin['plugin']['name']}@{p.origin['plugin']['version']}" if p.origin else ""))
        for f in p.failures:
            print(f"       - {f}")
    if a.json:
        Path(a.json).write_text(json.dumps([plan_dict(p) for p in plans], indent=1) + "\n")
    ok = sum(1 for p in plans if p.status == "planned")
    print(f"\nplanned {ok} / {len(plans)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
