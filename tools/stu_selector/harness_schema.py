#!/usr/bin/env python3

"""P3 step 1: an explicit, persisted harness schema for each STU entry.

Today the harness generator infers parameter roles by *adjacency* at generation time (a pointer
followed by an integer is a buffer+length). That is fragile. This module makes the roles an
explicit, reviewable, version-controlled artifact: one `schemas/<program>.json` per entry that
the generator (P3 step 2) will read instead of re-inferring.

Design constraints (project-lead review):
- Roles come from the schema, not adjacency.
- Separate the four things the old code conflated: input buffer, output buffer, capacity, and
  logical output length.
- The schema is DERIVED from the current classify() so that the step-2 migration can be proven
  byte-identical: every param carries an explicit `decode` kind that mirrors exactly what the
  current generator does (scalar take_<ty>; buffer take_vec_<elem>; length/capacity derived from
  the buffer's decoded length; out-scalar zero-initialised). The richer role labels
  (output_buffer vs inout_buffer, capacity vs length, written_length) are METADATA for future
  semantic improvements and intentionally do NOT change the decode bytes yet.

This step only defines + derives + persists + validates schemas. It does NOT rewire the
generator (that is step 2) and does NOT add nested-pointer support.

Usage:
  python3 tools/stu_selector/harness_schema.py --all          # (re)derive for every generatable entry
  python3 tools/stu_selector/harness_schema.py --pair benchmark/pairs/rle_codec --entry rle_encode
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import gen_diff_harness as gdh  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
SCHEMA_DIR = ROOT / "schemas"
SCHEMA_VERSION = 1

ROLES = {"scalar", "input_buffer", "inout_buffer", "output_buffer", "out_scalar",
         "input_string", "length", "capacity"}
DECODES = {"scalar", "vector", "derived_from_buffer", "out_scalar_zero", "nul_string"}

# name hints used only to DRAFT output- vs inout-buffer; provenance flags this for human review.
_OUT_HINTS = ("dst", "out", "output", "result", "res")


def _looks_output(buf_name: str, len_name: str) -> bool:
    b = buf_name.lower()
    return any(h in b for h in _OUT_HINTS) or "cap" in len_name.lower()


def derive(cc_dir: Path, entry: str) -> dict:
    """Build a draft schema from the current libclang signature + classify() grouping.

    Faithful to current decode behavior so the step-2 generator migration is byte-identical.
    """
    params, ret, _ = gdh.parse_entry_signature(cc_dir, entry)
    items = gdh.classify(params)

    # Resolve buffer roles and remember each length/capacity param's owning buffer.
    buf_role: dict[str, str] = {}          # buffer name -> input/inout/output_buffer
    len_owner: dict[str, tuple[str, str]] = {}  # int param -> (buffer name, "length"|"capacity")
    by_name = {p["name"]: p for p in params}
    for it in items:
        if it["role"] == "in_buf":
            buf_role[it["name"]] = "input_buffer"
            len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "io_buf":
            if _looks_output(it["name"], it["len_name"]):
                buf_role[it["name"]] = "output_buffer"
                len_owner[it["len_name"]] = (it["name"], "capacity")
            else:
                buf_role[it["name"]] = "inout_buffer"
                len_owner[it["len_name"]] = (it["name"], "length")
    out_scalars = {it["name"] for it in items if it["role"] == "out_scalar"}

    sparams = []
    for p in params:
        n = p["name"]
        if n in buf_role:
            it = next(i for i in items if i["name"] == n)
            role = buf_role[n]
            spec = {"name": n, "role": role, "decode": "vector",
                    "elem": it["elem"], "elem_width": it["elem_w"]}
            if role == "output_buffer":
                spec["capacity_param"] = it["len_name"]
                # How much of the output buffer is meaningful is NOT inferable from the signature
                # (the return value might be a status code, not a written length). Default unknown;
                # a human sets {"kind": "return_value"} or {"kind": "nul_terminated"} after review.
                spec["observable_length"] = {"kind": "unknown"}
            else:
                spec["length_param"] = it["len_name"]
            sparams.append(spec)
        elif n in len_owner:
            buf, kind = len_owner[n]
            sparams.append({"name": n, "role": kind, "decode": "derived_from_buffer",
                            "of_buffer": buf, "rust": p["rust"], "width": p["w"]})
        elif n in out_scalars:
            it = next(i for i in items if i["name"] == n)
            # A *const* pointer with no length is an input string (NUL-terminated), not an output
            # scalar. Adjacency inference got this wrong (it ignored const); the schema fixes it.
            if by_name[n].get("const"):
                sparams.append({"name": n, "role": "input_string", "decode": "nul_string",
                                "elem": it["elem"], "elem_width": it["elem_w"]})
            else:
                sparams.append({"name": n, "role": "out_scalar", "decode": "out_scalar_zero",
                                "elem": it["elem"], "elem_width": it["elem_w"]})
        else:  # plain scalar
            sparams.append({"name": n, "role": "scalar", "decode": "scalar",
                            "rust": p["rust"], "width": p["w"]})

    return {
        "schema_version": SCHEMA_VERSION,
        "program": cc_dir.parent.name,
        "entry": entry,
        "provenance": "auto-derived from libclang signature + classify(); REVIEW roles "
                      "(output_buffer vs inout_buffer, capacity vs length, written_length) "
                      "before trusting — decode kinds mirror the pre-migration generator exactly.",
        "return": {"rust": ret},
        "params": sparams,
    }


def validate(schema: dict) -> list[str]:
    """Return a list of problems; empty means valid."""
    errs = []
    if schema.get("schema_version") != SCHEMA_VERSION:
        errs.append(f"bad schema_version: {schema.get('schema_version')}")
    names = {p["name"] for p in schema.get("params", [])}
    for p in schema.get("params", []):
        if p.get("role") not in ROLES:
            errs.append(f"{p.get('name')}: bad role {p.get('role')}")
        if p.get("decode") not in DECODES:
            errs.append(f"{p.get('name')}: bad decode {p.get('decode')}")
        # cross-references must resolve
        for ref in ("length_param", "capacity_param", "of_buffer"):
            if ref in p and p[ref] not in names:
                errs.append(f"{p['name']}: {ref} -> unknown param {p[ref]}")
        if p.get("role") in ("input_buffer", "inout_buffer") and "length_param" not in p:
            errs.append(f"{p['name']}: {p['role']} missing length_param")
        if p.get("role") == "output_buffer" and "capacity_param" not in p:
            errs.append(f"{p['name']}: output_buffer missing capacity_param")
    # observable_length kind must be known
    for p in schema.get("params", []):
        if p.get("role") == "output_buffer":
            ol = p.get("observable_length", {})
            if ol.get("kind") not in ("unknown", "return_value", "nul_terminated"):
                errs.append(f"{p['name']}: bad observable_length {ol}")
    # every length/capacity must be referenced by EXACTLY one buffer, and its of_buffer must name
    # that same buffer (not merely an existing param).
    owners: dict[str, list[str]] = {}
    for p in schema.get("params", []):
        for ref in ("length_param", "capacity_param"):
            if ref in p:
                owners.setdefault(p[ref], []).append(p["name"])
    for p in schema.get("params", []):
        if p.get("role") in ("length", "capacity"):
            owning = owners.get(p["name"], [])
            if len(owning) != 1:
                errs.append(f"{p['name']}: {p['role']} referenced by {len(owning)} buffers (must be exactly 1)")
            elif p.get("of_buffer") != owning[0]:
                errs.append(f"{p['name']}: of_buffer {p.get('of_buffer')!r} != referencing buffer {owning[0]!r}")
            if p.get("of_buffer") not in names:
                errs.append(f"{p['name']}: of_buffer unresolved")
    return errs


# role -> the single legal decode kind
_ROLE_DECODE = {
    "scalar": "scalar", "input_buffer": "vector", "inout_buffer": "vector",
    "output_buffer": "vector", "out_scalar": "out_scalar_zero", "input_string": "nul_string",
    "length": "derived_from_buffer", "capacity": "derived_from_buffer",
}


def validate_against_signature(schema: dict, params: list[dict], ret: str) -> list[str]:
    """Fail-closed cross-check of a schema against the libclang signature.

    Verifies param count, order, names, scalar/element types, and role/decode legality so a
    stale or hand-edited schema cannot silently generate a wrong harness.
    """
    errs = []
    sp = schema.get("params", [])
    sret = (schema.get("return") or {}).get("rust")
    if sret != ret:
        errs.append(f"return type {sret!r} != signature {ret!r}")
    if len(sp) != len(params):
        return errs + [f"param count mismatch: schema {len(sp)} vs signature {len(params)}"]
    const_roles = ("input_buffer", "input_string")
    mut_roles = ("output_buffer", "inout_buffer", "out_scalar")
    for s, p in zip(sp, params):
        if s["name"] != p["name"]:
            errs.append(f"order/name mismatch: schema {s['name']} vs signature {p['name']}")
            continue
        if _ROLE_DECODE.get(s.get("role")) != s.get("decode"):
            errs.append(f"{s['name']}: role {s.get('role')} incompatible with decode {s.get('decode')}")
        if p["kind"] == "ptr":
            if s["role"] not in const_roles + mut_roles:
                errs.append(f"{s['name']}: pointer param has non-pointer role {s['role']}")
                continue
            if s.get("elem") != p["elem"]:
                errs.append(f"{s['name']}: elem {s.get('elem')} != signature {p['elem']}")
            if s.get("elem_width") != p["elem_w"]:
                errs.append(f"{s['name']}: elem_width {s.get('elem_width')} != signature {p['elem_w']}")
            if s["role"] in const_roles and not p["const"]:
                errs.append(f"{s['name']}: {s['role']} requires a const pointer")
            if s["role"] in mut_roles and p["const"]:
                errs.append(f"{s['name']}: {s['role']} requires a mutable pointer")
        else:
            if s["role"] not in ("scalar", "length", "capacity"):
                errs.append(f"{s['name']}: scalar param has non-scalar role {s['role']}")
                continue
            if s.get("rust") != p["rust"]:
                errs.append(f"{s['name']}: rust {s.get('rust')} != signature {p['rust']}")
            if s.get("width") != p["w"]:
                errs.append(f"{s['name']}: width {s.get('width')} != signature {p['w']}")
    return errs


def load(path: Path) -> dict:
    return json.loads(Path(path).read_text(encoding="utf-8"))


def save(schema: dict, path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(schema, indent=2), encoding="utf-8")


def _generatable_pairs() -> list[tuple[Path, str]]:
    import frontier as fr
    out = []
    entries = {c.stem: fr.parse_entry(c) for c in (ROOT / "benchmark" / "raw").rglob("*.c")}
    for pair in sorted((ROOT / "benchmark" / "pairs").iterdir()):
        if not pair.is_dir() or pair.name.startswith("_"):
            continue
        entry = entries.get(pair.name)
        if not entry:
            continue
        try:
            gdh.parse_entry_signature(pair / "build", entry)  # raises on unsupported signature
        except SystemExit:
            continue
        out.append((pair, entry))
    return out


def main() -> int:
    ap = argparse.ArgumentParser(description="Derive + persist explicit harness schemas")
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--all", action="store_true", help="every generatable entry")
    g.add_argument("--pair", help="single pair dir")
    ap.add_argument("--entry")
    args = ap.parse_args()

    targets = _generatable_pairs() if args.all else [(Path(args.pair), args.entry)]
    bad = 0
    for pair, entry in targets:
        schema = derive(pair / "build", entry)
        errs = validate(schema)
        out = SCHEMA_DIR / f"{pair.name}.json"
        save(schema, out)
        status = "OK" if not errs else f"INVALID: {errs}"
        if errs:
            bad += 1
        roles = ", ".join(f"{p['name']}:{p['role']}" for p in schema["params"])
        print(f"[{pair.name}] {status}  ({roles})")
    print(f"\nwrote {len(targets)} schema(s) to {SCHEMA_DIR}/" + (f"; {bad} invalid" if bad else ""))
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
