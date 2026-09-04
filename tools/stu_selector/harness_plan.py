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


class BodyAnalyzer:
    """Derives, from the entry's own body: rejection guards, index bounds and loop bounds.

    Nothing here looks at a CALL SITE.  Rule 7: a caller's local array declaration is that
    caller's fact, never the boundary's contract.
    """

    def __init__(self, fn_cursor, param_names: set[str]):
        self.fn = fn_cursor
        self.params = param_names
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
            self._record_write(kids[0], loop_depth, in_for_control)
            self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
            self._scan(kids[1], loop_depth, write_targets, in_for_control)
            return
        if kind == CursorKind.COMPOUND_ASSIGNMENT_OPERATOR and len(kids) == 2:
            self._record_dep(kids[0], kids[1])
            self._record_write(kids[0], loop_depth, in_for_control)
            self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
            self._scan(kids[1], loop_depth, write_targets, in_for_control)
            return
        if kind == CursorKind.UNARY_OPERATOR and kids:
            toks = [t.spelling for t in node.get_tokens()]
            if "++" in toks or "--" in toks:
                self._record_write(kids[0], loop_depth, in_for_control)
                self._scan(kids[0], loop_depth, write_targets | {_key(kids[0])}, in_for_control)
                return
            if toks and toks[0] == "*":
                self._record_deref(kids[0], _key(node) in write_targets, node)

        if kind == CursorKind.ARRAY_SUBSCRIPT_EXPR and len(kids) == 2 and self.pass_no == 2:
            base, idx = kids
            name, ref = _ref_name(base)
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

        for c in kids:
            self._scan(c, loop_depth, write_targets, in_for_control)

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
        n, _ = _ref_name(target)
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
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
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
            if (cur.kind == CursorKind.FUNCTION_DECL and cur.is_definition()
                    and cur.spelling == entry):
                f = cur.location.file.name if cur.location and cur.location.file else ""
                if not f.startswith(("/usr/", "/lib/")):
                    return cur, tu          # keep `tu` alive: cursors borrow from it
    return None, None


_EFFECTFUL_CACHE: dict[str, set] = {}


def effectful_functions(cc_dir: Path) -> set:
    """Functions in this translation unit that transitively reach an effectful call.

    The effect is what matters, and it is rarely in the entry's own body: `BZ2_bzopen` does not
    call `fopen`, it calls `bzopen_or_bzdopen`, which does. Without the transitive step the rule
    would catch the private helper and wave the two public entry points through.
    """
    key = str(cc_dir)
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
            if cur.kind != CursorKind.FUNCTION_DECL or not cur.is_definition():
                continue
            callees = edges.setdefault(cur.spelling, set())
            for n in cur.walk_preorder():
                if n.kind == CursorKind.CALL_EXPR and n.spelling:
                    callees.add(n.spelling)
    eff = set(EFFECTFUL_CALLS)
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
    ty = re.sub(r"&\s*'\w+\s*", "&", ty or "")
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
                one_elem: bool = False) -> tuple[str | None, str | None]:
    """(bridge name, reason it is missing).  `rust_ty` None means the C ABI form."""
    if rust_ty is None:
        return "c_abi", None                 # no Rust signature parsed: raw C-ABI call
    r = _norm_ty(rust_ty, aliases)

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
        if elem and pointee not in (elem, "c_void", "core::ffi::c_void") and adapter != "null_pointer":
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
    if rust_types is not None and len(rust_types) == len(params):
        by_pos = {p["name"]: rust_types[i] for i, p in enumerate(params)}
    folded = set()
    for s in plan.specs:
        ty = by_pos.get(s.param)
        one = (s.detail.get("alloc_elems") == 1
               or (s.detail.get("extent") or {}).get("v") == 1)
        b, why = rust_bridge(s.c_decoder, ty, s.detail.get("elem"),
                             by_name.get(s.param, {}).get("rust"), aliases, one)
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
        if p["kind"] == "ptr_ptr" and not _table_count(p, params, subscripted_names(facts)):
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
        if not req[pn]:
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
        if pn in length_of or req.get(pn):
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
            plan.specs.append(InputSpec(n, "struct_value",
                                        {"struct": p["struct"]["name"]}, e["evidence"]))
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


def build_plan(cc_dir: Path, entry: str, policy: GeneratorPolicy = POLICY,
               rust_types: list[str] | None = None,
               rust_aliases: dict | None = None) -> HarnessPlan:
    """InputPlan -> HarnessPlan.  No schema is read; nothing is hand-written.

    The return value is NOT a construction gate.  What can be compared about it is decided by the
    fixed ladder in the emitters: void -> nothing, scalar -> value, pointer -> nullness (or a user
    plugin).  A boundary is rejected only when its INPUT cannot be constructed.
    """
    failures: list[str] = []

    def _fail(reason: str) -> HarnessPlan:
        return HarnessPlan(entry, "failed", [], [], [reason], policy.as_dict())

    try:
        params, ret, _fns, ret_desc = gdh.parse_entry_signature(cc_dir, entry, with_return_desc=True)
    except SystemExit as e:
        return _fail(f"signature: {e}")
    if not params and entry not in _fns:
        return _fail("signature: the entry was not found in the pair's translation unit")
    if not params:
        # No arguments is not a construction failure: there IS no input to construct. Both sides
        # are called once and compared -- a single deterministic execution decides the boundary.
        return HarnessPlan(entry, "planned", [], [], [], policy.as_dict())

    cur, _tu = entry_cursor(cc_dir, entry)
    if cur is None:
        return _fail("body: no definition found in the pair's compilation database")
    facts = BodyAnalyzer(cur, {p["name"] for p in params}).run()

    iplan = analyze_inputs(params, facts, policy, effectful_functions(cc_dir))
    failures += iplan.failures + facts.unresolved
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
        elif a == "scalar":
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
        else:
            raise LoweringError(f"{plan.boundary}: no lowering for adapter {a!r} on {n}")
    return {"schema_version": 1, "program": program, "entry": plan.boundary,
            "provenance": "lowered from a generated HarnessPlan; not hand-written",
            "decode_scalars_first": True, "return": {"rust": ret_rust}, "params": out}


def plan_and_lower(cc_dir: Path, entry: str, program: str,
                   policy: GeneratorPolicy = POLICY,
                   rust_types: list[str] | None = None,
                   rust_aliases: dict | None = None) -> tuple[HarnessPlan, dict | None]:
    plan = build_plan(cc_dir, entry, policy, rust_types, rust_aliases)
    if plan.status != "planned":
        return plan, None
    params, ret, _fns = gdh.parse_entry_signature(cc_dir, entry)
    return plan, lower_to_schema(plan, params, program, ret, policy)


# ---------------------------------------------------------------------------
# CLI
# ---------------------------------------------------------------------------
def _all_entries(cc_dir: Path) -> list[str]:
    cgmod._configure_libclang()
    from clang.cindex import CompilationDatabase
    cdb = CompilationDatabase.fromDirectory(str(cc_dir))
    index = Index.create()
    cwd0, out = os.getcwd(), []
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
            if cur.kind == CursorKind.FUNCTION_DECL and cur.is_definition():
                f = cur.location.file.name if cur.location and cur.location.file else ""
                if f and not f.startswith(("/usr/", "/lib/")):
                    out.append(cur.spelling)
    return sorted(set(out))


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
    ap.add_argument("--out-dir")
    ap.add_argument("--json")
    a = ap.parse_args()

    cc = Path(a.pair) / "build"
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
        rt = gdh.parse_rust_param_types(rs_text, a.rust_entry or e) if rs_text else None
        p = build_plan(cc, e, rust_types=(rt or None), rust_aliases=aliases)
        plans.append(p)
        if outdir:
            (outdir / f"{e}.plan.json").write_text(json.dumps(asdict(p), indent=1) + "\n")
        mark = "OK " if p.status == "planned" else "FAIL"
        print(f"{mark} {e:30s} inputs={len(p.inputs):2d}")
        for f in p.failures:
            print(f"       - {f}")
    if a.json:
        Path(a.json).write_text(json.dumps([asdict(p) for p in plans], indent=1) + "\n")
    ok = sum(1 for p in plans if p.status == "planned")
    print(f"\nplanned {ok} / {len(plans)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
