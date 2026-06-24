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

# inclusive [min, max] domain per Rust scalar type, for validating bounded_scalar ranges.
TYPE_RANGE = {
    "u8": (0, 2**8 - 1), "u16": (0, 2**16 - 1), "u32": (0, 2**32 - 1),
    "u64": (0, 2**64 - 1), "usize": (0, 2**64 - 1),
    "i8": (-2**7, 2**7 - 1), "i16": (-2**15, 2**15 - 1),
    "i32": (-2**31, 2**31 - 1), "i64": (-2**63, 2**63 - 1),
}

ROLES = {"scalar", "input_buffer", "inout_buffer", "output_buffer", "out_scalar",
         "input_string", "output_array", "input_fixed_array_buffer", "input_rectangular_pointer_table",
         "input_string_pointer_table", "input_struct", "inout_struct",
         "input_struct_array", "inout_struct_array", "length", "capacity"}
DECODES = {"scalar", "bounded_scalar", "vector", "derived_from_buffer", "out_scalar_zero",
           "nul_string", "output_array_vector", "fixed_array_vector", "rectangular_pointer_table",
           "string_pointer_table", "struct_value", "struct_array_vector"}

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
        if it["role"] == "in_table":
            buf_role[it["name"]] = "input_rectangular_pointer_table"
            len_owner[it["outer_name"]] = (it["name"], "length")
            len_owner[it["inner_name"]] = (it["name"], "length")
        elif it["role"] == "in_str_table":
            buf_role[it["name"]] = "input_string_pointer_table"
            len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "in_arr":
            buf_role[it["name"]] = "input_fixed_array_buffer"
            len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "in_buf":
            buf_role[it["name"]] = "input_buffer"
            len_owner[it["len_name"]] = (it["name"], "length")
        elif it["role"] == "io_buf":
            if _looks_output(it["name"], it["len_name"]):
                buf_role[it["name"]] = "output_buffer"
                len_owner[it["len_name"]] = (it["name"], "capacity")
            else:
                buf_role[it["name"]] = "inout_buffer"
                len_owner[it["len_name"]] = (it["name"], "length")
    for it in items:
        if it["role"] in ("in_struct_arr", "io_struct_arr"):
            len_owner[it["len_name"]] = (it["name"], "length")
    out_scalars = {it["name"] for it in items if it["role"] == "out_scalar"}
    struct_items = {it["name"]: it for it in items
                    if it["role"] in ("in_struct", "io_struct", "in_struct_arr", "io_struct_arr")}

    sparams = []
    for p in params:
        n = p["name"]
        if n in struct_items:
            it = struct_items[n]
            spec = {"name": n, "struct_name": it["struct"]["name"],
                    "fields": gdh._struct_fields_to_schema(it["struct"])}
            if it["role"] in ("in_struct_arr", "io_struct_arr"):
                spec["role"] = "input_struct_array" if it["role"] == "in_struct_arr" else "inout_struct_array"
                spec["decode"] = "struct_array_vector"
                spec["length_param"] = it["len_name"]
            else:
                spec["role"] = "input_struct" if it["role"] == "in_struct" else "inout_struct"
                spec["decode"] = "struct_value"
            sparams.append(spec)
        elif n in buf_role:
            it = next(i for i in items if i["name"] == n)
            role = buf_role[n]
            spec = {"name": n, "role": role, "decode": "vector",
                    "elem": it["elem"], "elem_width": it["elem_w"]}
            if role == "input_rectangular_pointer_table":
                spec["decode"] = "rectangular_pointer_table"
                spec["outer_length_param"] = it["outer_name"]
                spec["inner_length_param"] = it["inner_name"]
                # dimension bounds are not inferable; default to 16 — REVIEW (rows×cols allocation).
                # Each dimension is one decoded byte, so max must stay in [0,255].
                spec["outer_max"] = 16
                spec["inner_max"] = 16
                # v1 contract: callee must not rewrite the row-pointer table (we compare backing
                # contents, not pointer addresses). REVIEW if a callee reorders/replaces rows.
                spec["mutation"] = "backing_observable"
            elif role == "input_string_pointer_table":
                spec["decode"] = "string_pointer_table"
                spec["length_param"] = it["len_name"]
                # count is one decoded byte, so its max must stay in [0,255]; default 16 — REVIEW.
                spec["count_max"] = 16
                # Each backing is an independent NUL-terminated string (per-item length). v1
                # contract: callee must not rewrite the pointer table; we compare backing contents.
                spec["mutation"] = "backing_observable"
            elif role == "input_fixed_array_buffer":
                spec["decode"] = "fixed_array_vector"
                spec["inner_extent"] = it["inner_extent"]
                spec["length_param"] = it["len_name"]
            elif role == "output_buffer":
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
        if p.get("role") in ("input_buffer", "inout_buffer", "input_fixed_array_buffer") \
                and "length_param" not in p:
            errs.append(f"{p['name']}: {p['role']} missing length_param")
        if p.get("role") == "input_fixed_array_buffer" and not isinstance(p.get("inner_extent"), int):
            errs.append(f"{p['name']}: input_fixed_array_buffer missing inner_extent")
        if p.get("role") == "input_rectangular_pointer_table":
            for req in ("outer_length_param", "inner_length_param"):
                if p.get(req) not in names:
                    errs.append(f"{p['name']}: rectangular table {req} unresolved")
            # v1: each dimension is decoded from ONE byte, so its max must fit [0, 255].
            for dim in ("outer_max", "inner_max"):
                v = p.get(dim)
                if not isinstance(v, int) or not (0 <= v <= 255):
                    errs.append(f"{p['name']}: rectangular table {dim} must be an int in [0,255] (1-byte dimension)")
            # Mutation contract: v1 only supports a callee that does not rewrite the row-pointer
            # table (we compare backing contents, not pointer topology / addresses).
            if p.get("mutation") != "backing_observable":
                errs.append(f"{p['name']}: rectangular table requires mutation=\"backing_observable\" "
                            f"(row-pointer rewriting not supported in v1)")
        if p.get("role") == "input_string_pointer_table":
            if p.get("length_param") not in names:
                errs.append(f"{p['name']}: string table length_param unresolved")
            # count is decoded from ONE byte, so its max must fit [0, 255].
            cm = p.get("count_max")
            if not isinstance(cm, int) or not (0 <= cm <= 255):
                errs.append(f"{p['name']}: string table count_max must be an int in [0,255] (1-byte count)")
            # Same v1 mutation contract as the rectangular table: backing contents are observable,
            # the pointer table itself must not be rewritten by the callee.
            if p.get("mutation") != "backing_observable":
                errs.append(f"{p['name']}: string table requires mutation=\"backing_observable\" "
                            f"(pointer-table rewriting not supported in v1)")
        if p.get("role") in ("input_struct", "inout_struct", "input_struct_array", "inout_struct_array"):
            if not p.get("struct_name"):
                errs.append(f"{p['name']}: struct role missing struct_name")
            fs = p.get("fields")
            if not isinstance(fs, list) or not fs:
                errs.append(f"{p['name']}: struct role needs a non-empty fields list")
            else:
                for fspec in fs:
                    if fspec.get("kind") not in ("scalar", "array"):
                        errs.append(f"{p['name']}: struct field {fspec.get('name')} bad kind {fspec.get('kind')}")
            if p.get("role") in ("input_struct_array", "inout_struct_array") and "length_param" not in p:
                errs.append(f"{p['name']}: struct array missing length_param")
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
        for ref in ("length_param", "capacity_param", "outer_length_param", "inner_length_param"):
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
    "output_buffer": "vector", "out_scalar": "out_scalar_zero", "input_string": "nul_string", "output_array": "output_array_vector",
    "input_fixed_array_buffer": "fixed_array_vector",
    "input_rectangular_pointer_table": "rectangular_pointer_table",
    "input_string_pointer_table": "string_pointer_table",
    "input_struct": "struct_value", "inout_struct": "struct_value",
    "input_struct_array": "struct_array_vector", "inout_struct_array": "struct_array_vector",
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
        _dec = s.get("decode")
        _ok = (_ROLE_DECODE.get(s.get("role")) == _dec
               or (s.get("role") == "scalar" and _dec == "bounded_scalar"))
        if not _ok:
            errs.append(f"{s['name']}: role {s.get('role')} incompatible with decode {_dec}")
        if _dec == "bounded_scalar":
            lo, hi, rng = s.get("min_value"), s.get("max_value"), TYPE_RANGE.get(s.get("rust"))
            if not isinstance(lo, int) or not isinstance(hi, int) or lo > hi:
                errs.append(f"{s['name']}: bounded_scalar needs integer min_value <= max_value")
            elif rng and not (rng[0] <= lo <= hi <= rng[1]):
                errs.append(f"{s['name']}: bounded range [{lo},{hi}] outside {s.get('rust')} {rng}")
            elif rng and (hi - lo) >= (rng[1] - rng[0]):
                errs.append(f"{s['name']}: bounded span ({hi}-{lo}+1) not representable in {s.get('rust')}")
        if p["kind"] == "ptr_struct":
            single = "inout_struct" if not p["const"] else "input_struct"
            arr = "inout_struct_array" if not p["const"] else "input_struct_array"
            if s.get("role") not in (single, arr):
                errs.append(f"{s['name']}: struct pointer (const={p['const']}) should be {single}/{arr}, got {s.get('role')}")
            elif s.get("struct_name") != p["struct"]["name"]:
                errs.append(f"{s['name']}: struct_name {s.get('struct_name')} != signature {p['struct']['name']}")
            continue
        if p["kind"] == "ptr_ptr":
            if s["role"] not in ("input_rectangular_pointer_table", "input_string_pointer_table"):
                errs.append(f"{s['name']}: T** param has role {s['role']}")
                continue
            if s.get("elem") != p["elem"]:
                errs.append(f"{s['name']}: elem {s.get('elem')} != signature {p['elem']}")
            if s.get("elem_width") != p["elem_w"]:
                errs.append(f"{s['name']}: elem_width {s.get('elem_width')} != signature {p['elem_w']}")
            continue
        if p["kind"] == "ptr_array":
            if s["role"] != "input_fixed_array_buffer":
                errs.append(f"{s['name']}: pointer-to-array param has role {s['role']}")
                continue
            if s.get("elem") != p["elem"]:
                errs.append(f"{s['name']}: elem {s.get('elem')} != signature {p['elem']}")
            if s.get("elem_width") != p["elem_w"]:
                errs.append(f"{s['name']}: elem_width {s.get('elem_width')} != signature {p['elem_w']}")
            if s.get("inner_extent") != p["inner_extent"]:
                errs.append(f"{s['name']}: inner_extent {s.get('inner_extent')} != signature {p['inner_extent']}")
            if not p["const"]:
                errs.append(f"{s['name']}: input_fixed_array_buffer requires a const pointer")
        elif p["kind"] == "ptr":
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


# Human POLICY annotations that derive() cannot reproduce from the signature alone; these must
# survive a re-derive. derive() produces STRUCTURE (roles, decode kinds, elem types); a human sets
# POLICY (bounded ranges, what the observable output length is, tuned dimension caps, NUL-termination).
_POLICY_FIELDS = ("observable_length", "mutation", "outer_max", "inner_max", "count_max",
                  "nul_terminated")


def merge_overrides(fresh: dict, existing: dict) -> dict:
    """Re-apply human policy annotations from an existing schema onto a freshly-derived one,
    matched by param name. Without this, `--all` silently CLOBBERS hand annotations (e.g. graph_dfs
    `n` bounded_scalar [0,64] -> plain scalar; kv_config nul_terminated; rle/leb observable_length).
    Re-derive becomes IDEMPOTENT on a hand-annotated schema."""
    old = {p["name"]: p for p in existing.get("params", [])}
    for p in fresh.get("params", []):
        o = old.get(p["name"])
        if not o:
            continue
        # bounded_scalar is a manual decode override on a scalar param.
        if o.get("decode") == "bounded_scalar" and p.get("role") == "scalar":
            p["decode"] = "bounded_scalar"
            for k in ("min_value", "max_value"):
                if k in o:
                    p[k] = o[k]
        for k in _POLICY_FIELDS:
            if k in o:
                p[k] = o[k]
    # preserve a human-curated provenance note if one was set
    if existing.get("provenance") and existing["provenance"] != fresh.get("provenance"):
        fresh.setdefault("provenance_original", fresh.get("provenance"))
        fresh["provenance"] = existing["provenance"]
    return fresh


def derive_merged(cc_dir: Path, entry: str, schema_path: Path) -> dict:
    """derive() then re-apply policy from the on-disk schema (if any) so annotations survive."""
    fresh = derive(cc_dir, entry)
    if schema_path.exists():
        fresh = merge_overrides(fresh, load(schema_path))
    return fresh


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
    ap.add_argument("--check", action="store_true",
                    help="do NOT write; verify each EXISTING on-disk schema equals derive()+overrides "
                         "(detects drift / accidental clobber) and exit nonzero on mismatch. Pairs "
                         "with no schema are skipped — they are harvested via --ignore-schema, not "
                         "schema-managed.")
    args = ap.parse_args()

    targets = _generatable_pairs() if args.all else [(Path(args.pair), args.entry)]
    bad = 0
    drift = 0
    checked = skipped = 0
    for pair, entry in targets:
        out = SCHEMA_DIR / f"{pair.name}.json"
        # --check verifies the schemas we MAINTAIN; a pair with no on-disk schema is not drift,
        # it is simply inference-harvested (not under schema management).
        if args.check and not out.exists():
            skipped += 1
            continue
        schema = derive_merged(pair / "build", entry, out)  # re-applies human policy annotations
        errs = validate(schema)
        if errs:
            bad += 1
        roles = ", ".join(f"{p['name']}:{p['role']}" for p in schema["params"])
        if args.check:
            checked += 1
            if load(out) != schema:
                drift += 1
                print(f"[{pair.name}] DRIFT: on-disk schema != derive()+overrides")
            else:
                print(f"[{pair.name}] OK (in sync)  ({roles})")
            continue
        save(schema, out)
        status = "OK" if not errs else f"INVALID: {errs}"
        print(f"[{pair.name}] {status}  ({roles})")
    if args.check:
        print(f"\nchecked {checked} existing schema(s); {drift} drift, {bad} invalid; "
              f"{skipped} pair(s) skipped (no schema, inference-harvested)")
        return 1 if (drift or bad) else 0
    print(f"\nwrote {len(targets)} schema(s) to {SCHEMA_DIR}/" + (f"; {bad} invalid" if bad else ""))
    return 1 if bad else 0


if __name__ == "__main__":
    raise SystemExit(main())
