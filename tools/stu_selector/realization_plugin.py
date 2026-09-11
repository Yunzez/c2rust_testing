#!/usr/bin/env python3
"""HarnessPlan resource-realization plugins: manifest loader, closed view vocabulary, plan record.

Design: `docs/construction_recipe_plugin_plan.md`.  Read it before changing anything here.

A resource-realization plugin is a TRUSTED, VERSIONED MANIFEST (TOML, no code).  It supplies the
one part of a logical resource the generic planner could not derive for a boundary it abstained on
-- the MATERIALIZATION (`stack T -> init(T*) -> target(T*) -> cleanup(T*)`) -- and SELECTS the
argument views from the emitter's existing closed vocabulary.  It never adds conversion code, never
touches comparison, and is a different namespace and kind from the comparator plugins that
`gen_diff_harness.load_plugins` reads (`--plugins`): those are CODE behind an ABI for OUTPUT
comparison; this is a DECLARATION for INPUT construction.

Division of labour:
  * this module   -- parse + structurally validate a manifest (closed vocabulary, closed key set,
                     identifier-only values), hash its content, and shape the plan-origin record;
  * harness_plan  -- `_plan_realization`: the semantic checks 1-10 of the plan (types, symbols,
                     signatures, determinism, ownership, BodyFacts field check), lowering into the
                     existing plan concepts (side-local storage, PRODUCER/FREE phases);
  * gen_diff_harness -- emits the realization exactly as it emits the producer bridge.

A structural error here (unknown view, wrong kind, unknown key) is a manifest error and aborts
before any planning; a compatibility failure is `construction unsupported: <reason>` in the plan.
"""

from __future__ import annotations

import hashlib
import re
from dataclasses import dataclass, field, asdict
from pathlib import Path

KIND = "harness-plan-resource-realization"
MANIFEST_VERSIONS = (1,)
LOGICAL_ROLES = ("initialized-resource",)
STORAGES = ("stack",)

# The CLOSED argument-view vocabulary.  Each Rust view names a shape the emitter already knows
# how to hand a side-local `let mut <v>: R` to (docs/construction_recipe_plugin_plan.md section 4):
#   pattern    -- what the translation's parameter type must normalise to (harness_plan._norm_ty:
#                 whitespace and module paths removed, aliases resolved), with {R} the resource's
#                 Rust type name;
#   expr       -- the argument expression, with {v} the side-local variable.
# A view not in this table is rejected at LOAD time; there is no escape hatch for user-provided
# conversion code.
RUST_VIEWS: dict[str, dict] = {
    "raw-mut":           {"pattern": r"\*mut{R}",          "expr": "core::ptr::addr_of_mut!({v}) as *mut _",
                          "owning": False},
    "raw-const":         {"pattern": r"\*const{R}",        "expr": "core::ptr::addr_of!({v}) as *const _",
                          "owning": False},
    "borrow":            {"pattern": r"&{R}",              "expr": "&{v}", "owning": False},
    "mut-borrow":        {"pattern": r"&mut{R}",           "expr": "&mut {v}", "owning": False},
    "option-borrow":     {"pattern": r"Option<&{R}>",      "expr": "Some(&{v})", "owning": False},
    "option-mut-borrow": {"pattern": r"Option<&mut{R}>",   "expr": "Some(&mut {v})", "owning": False},
}
# C has exactly two views of a side-local object: its address, mutable or const.
C_VIEWS: dict[str, dict] = {
    "raw-mut":   {"const": False},
    "raw-const": {"const": True},
}

_IDENT = re.compile(r"^[A-Za-z_]\w*$")
_C_PARAM_TYPE = re.compile(r"^\s*(const\s+)?(struct\s+)?([A-Za-z_]\w*)\s*\*\s*$")

_PLUGIN_KEYS = {"kind", "library", "name", "version", "description"}
_RESOURCE_KEYS = {"c_type", "logical_role", "c", "rust"}
_C_KEYS = {"parameter_type", "storage", "initializer", "initializer_view", "target_view",
           "cleanup", "cleanup_view"}
_RUST_KEYS = {"binding", "type", "storage", "initializer", "initializer_view", "target_view",
              "cleanup", "cleanup_view"}
_REQUIRES_KEYS = {"c_functions", "rust_functions"}


class RealizationManifestError(ValueError):
    """A manifest that cannot be accepted as a manifest (structure, vocabulary, kind)."""


@dataclass
class SideBinding:
    storage: str
    target_view: str
    initializer: str | None = None
    initializer_view: str | None = None
    cleanup: str | None = None
    cleanup_view: str | None = None
    parameter_type: str | None = None        # C only: `LodePNGState *`
    type: str | None = None                  # Rust only: `LodePNGState`
    binding: str | None = None               # Rust only: a label for one of several declared bindings


@dataclass
class RealizationManifest:
    path: str
    kind: str
    library: str
    name: str
    version: int
    c_type: str
    logical_role: str
    c: SideBinding
    rust: list                               # [SideBinding]: one per declared translated representation
    requires: dict = field(default_factory=dict)
    content_hash: str = ""
    description: str = ""

    @property
    def ident(self) -> str:
        return f"{self.name}@{self.version}"

    def identity(self) -> dict:
        return {"name": self.name, "version": self.version, "library": self.library,
                "kind": self.kind, "manifest": self.path, "content_hash": self.content_hash}


def _err(path: Path, msg: str) -> RealizationManifestError:
    return RealizationManifestError(f"realization plugin {path}: {msg}")


def _check_keys(path: Path, table: dict, allowed: set, where: str) -> None:
    extra = sorted(set(table) - allowed)
    if extra:
        raise _err(path, f"[{where}] has unknown key(s) {extra}; the manifest is a closed "
                         f"declaration (allowed: {sorted(allowed)})")


def _ident(path: Path, val, where: str) -> str:
    if not isinstance(val, str) or not _IDENT.match(val):
        raise _err(path, f"{where} must be a plain identifier, got {val!r} (no source snippets are "
                         f"interpolated from a manifest)")
    return val


def _side(path: Path, d: dict, side: str) -> SideBinding:
    allowed = _C_KEYS if side == "c" else _RUST_KEYS
    _check_keys(path, d, allowed, f"resource.{side}")
    views = C_VIEWS if side == "c" else RUST_VIEWS
    storage = d.get("storage", "stack")
    if storage not in STORAGES:
        raise _err(path, f"[resource.{side}] storage {storage!r} is not one of {STORAGES}")
    if "target_view" not in d:
        raise _err(path, f"[resource.{side}] has no target_view")
    for k in ("initializer_view", "target_view", "cleanup_view"):
        if k in d and d[k] not in views:
            raise _err(path, f"[resource.{side}] {k} = {d[k]!r} is not in the closed "
                             f"{'C' if side == 'c' else 'Rust'} view vocabulary {sorted(views)}; "
                             f"a plugin selects a view, it never supplies conversion code")
    for hook in ("initializer", "cleanup"):
        if hook in d:
            _ident(path, d[hook], f"[resource.{side}] {hook}")
            if f"{hook}_view" not in d:
                raise _err(path, f"[resource.{side}] {hook} = {d[hook]!r} has no {hook}_view")
        elif f"{hook}_view" in d:
            raise _err(path, f"[resource.{side}] {hook}_view given without {hook}")
    sb = SideBinding(storage=storage, target_view=d["target_view"],
                     initializer=d.get("initializer"), initializer_view=d.get("initializer_view"),
                     cleanup=d.get("cleanup"), cleanup_view=d.get("cleanup_view"))
    if side == "c":
        pt = d.get("parameter_type")
        if not isinstance(pt, str) or not _C_PARAM_TYPE.match(pt):
            raise _err(path, f"[resource.c] parameter_type must be `T *` (optionally const/struct), "
                             f"got {pt!r}")
        sb.parameter_type = re.sub(r"\s+", " ", pt.strip())
    else:
        sb.type = _ident(path, d.get("type"), "[resource.rust] type")
        if "binding" in d:
            if not isinstance(d["binding"], str) or not re.match(r"^[A-Za-z0-9_.-]+$", d["binding"]):
                raise _err(path, f"[resource.rust] binding {d['binding']!r} must be [A-Za-z0-9_.-]+")
            sb.binding = d["binding"]
    return sb


def parse_manifest(path: str | Path, text: str | None = None) -> RealizationManifest:
    """Parse and structurally validate one manifest.  Raises RealizationManifestError."""
    import tomllib
    p = Path(path)
    raw = p.read_bytes() if text is None else text.encode("utf-8")
    try:
        doc = tomllib.loads(raw.decode("utf-8"))
    except (tomllib.TOMLDecodeError, UnicodeDecodeError) as e:
        raise _err(p, f"not valid TOML: {e}")
    _check_keys(p, doc, {"plugin", "resource", "requires"}, "manifest")
    pl = doc.get("plugin")
    if not isinstance(pl, dict):
        raise _err(p, "missing [plugin] table")
    kind = pl.get("kind")
    if kind != KIND:
        raise _err(p, f"[plugin] kind {kind!r} is not {KIND!r}" + (
            " (a comparator plugin manifest belongs to --plugins, not --realization-plugins)"
            if kind is None and "c_source" in pl else ""))
    _check_keys(p, pl, _PLUGIN_KEYS, "plugin")
    for k in ("library", "name"):
        if not isinstance(pl.get(k), str) or not pl[k].strip():
            raise _err(p, f"[plugin] {k} is required")
    if not re.match(r"^[A-Za-z0-9_.-]+$", pl["name"]):
        raise _err(p, f"[plugin] name {pl['name']!r} must be [A-Za-z0-9_.-]+")
    ver = pl.get("version")
    if not isinstance(ver, int) or ver not in MANIFEST_VERSIONS:
        raise _err(p, f"[plugin] version {ver!r} is not one of the supported manifest versions "
                      f"{MANIFEST_VERSIONS}")
    res = doc.get("resource")
    if not isinstance(res, dict):
        raise _err(p, "missing [resource] table")
    _check_keys(p, res, _RESOURCE_KEYS, "resource")
    c_type = _ident(p, res.get("c_type"), "[resource] c_type")
    role = res.get("logical_role")
    if role not in LOGICAL_ROLES:
        raise _err(p, f"[resource] logical_role {role!r} is not one of {LOGICAL_ROLES}")
    if not isinstance(res.get("c"), dict):
        raise _err(p, "[resource.c] is required")
    # One translated representation per binding: a single `[resource.rust]` table (the plan's
    # section-4 form) or several `[[resource.rust]]` tables, each a complete binding the planner
    # verifies on its own; the ones that do not fit a translation are recorded as rejected.
    rb = res.get("rust")
    if isinstance(rb, dict):
        rb = [rb]
    if not isinstance(rb, list) or not rb or not all(isinstance(x, dict) for x in rb):
        raise _err(p, "[resource.rust] (a table, or an array of tables) is required")
    c = _side(p, res["c"], "c")
    r = [_side(p, x, "rust") for x in rb]
    labels = [x.binding for x in r]
    if len(r) > 1 and (any(l is None for l in labels) or len(set(labels)) != len(labels)):
        raise _err(p, "several [[resource.rust]] bindings must each carry a distinct `binding` label")
    if _C_PARAM_TYPE.match(c.parameter_type).group(3) != c_type:
        raise _err(p, f"[resource.c] parameter_type {c.parameter_type!r} does not name c_type "
                      f"{c_type!r}")
    req = doc.get("requires") or {}
    if not isinstance(req, dict):
        raise _err(p, "[requires] must be a table")
    _check_keys(p, req, _REQUIRES_KEYS, "requires")
    for k in _REQUIRES_KEYS:
        v = req.get(k, [])
        if not isinstance(v, list):
            raise _err(p, f"[requires] {k} must be a list of identifiers")
        for x in v:
            _ident(p, x, f"[requires] {k} entry")
    # Every lifecycle function the manifest binds must also be in [requires]: the requirement list
    # is the auditable statement of what the plugin needs from each side.
    for side, sbs, key in (("c", [c], "c_functions"), ("rust", r, "rust_functions")):
        for sb in sbs:
            for hook in (sb.initializer, sb.cleanup):
                if hook and hook not in req.get(key, []):
                    raise _err(p, f"[resource.{side}] binds {hook!r} but [requires] {key} does not list it")
    return RealizationManifest(
        path=str(p), kind=kind, library=pl["library"], name=pl["name"], version=ver,
        c_type=c_type, logical_role=role, c=c, rust=r,
        requires={k: list(req.get(k, [])) for k in _REQUIRES_KEYS},
        content_hash=hashlib.sha256(raw).hexdigest(),
        description=str(pl.get("description", "")))


def load_realization_plugins(paths: list[str] | None) -> list[RealizationManifest]:
    """Load every manifest; a structural error is fatal (the user named the file explicitly)."""
    out: list[RealizationManifest] = []
    for p in (paths or []):
        m = parse_manifest(p)
        if any(o.name == m.name and o.version == m.version for o in out):
            raise _err(Path(p), f"duplicate plugin {m.ident}")
        out.append(m)
    return out


def rust_view_pattern(view: str, rust_type: str) -> re.Pattern:
    """Regex a NORMALISED Rust parameter type must fully match to be this view of `rust_type`."""
    return re.compile(RUST_VIEWS[view]["pattern"].format(R=re.escape(rust_type)))


def rust_view_expr(view: str, var: str) -> str:
    """The argument expression handing the side-local `var` to a function under `view`."""
    return RUST_VIEWS[view]["expr"].format(v=var)


def c_view_is_const(view: str) -> bool:
    return C_VIEWS[view]["const"]


def origin_record(m: RealizationManifest, resource: dict, views: dict, hooks: dict,
                  validation: list[dict], assumptions: list[str],
                  rejected: list[dict]) -> dict:
    """The plan-origin record persisted in the HarnessPlan (plan section 5)."""
    return {
        "origin": "plugin",
        "plugin": m.identity(),
        "logical_role": m.logical_role,
        "resource": resource,        # C type/role/layout and Rust type/storage
        "hooks": hooks,              # initializer / cleanup on each side, with source locations
        "views": views,              # owner + initializer, target, cleanup views per side
        "validation": validation,    # every check, with its evidence
        "assumptions": assumptions,  # what stays the plugin author's obligation
        "rejected_alternatives": rejected,
        "manifest": {"c": asdict(m.c), "rust": [asdict(b) for b in m.rust], "requires": m.requires},
    }
