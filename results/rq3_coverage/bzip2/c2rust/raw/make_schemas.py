#!/usr/bin/env python3
"""Author one explicit harness schema per bzip2 boundary.

Each schema is built from the libclang signature (so param order, names, element types and widths
are never hand-typed) plus a POLICY block that records what the C code actually requires.  Every
constant below is justified by a line of bzip2 1.0.8:

  BZ_MAX_ALPHA_SIZE 258, BZ_MAX_CODE_LEN 23           bzlib_private.h:115-116
  blockSize100k must be 1..9, workFactor 0..250        bzlib.c BZ2_bzCompressInit param check
  verbosity 0..4, small 0..1                           same
  blockSort takes mainSort only when nblock >= 10000    blocksort.c:1044
  base[length[i]+1] with UChar length -> index <= 256   huffman.c BZ2_hbCreateDecodeTables
  indexIntoF binary-searches cftab[0..255]              bzlib.c BZ2_indexIntoF
  fallbackSort needs fmap[nblock], eclass[nblock], bhtab[2+nblock/32]   blocksort.c

Every schema is checked against the signature by harness_schema.validate_against_signature before
it is written, so a wrong hand annotation fails loudly instead of generating a wrong harness.
"""
import json, sys
from pathlib import Path

SCR = Path("/tmp/claude-1000/-home-yunzez-c2rust-testing/"
           "6278f822-c4c5-451c-94c6-d3a713132b29/scratchpad/rq4_cov")
sys.path.insert(0, str(SCR / "rq4_gen"))
import gen_diff_harness as gdh          # noqa: E402
import harness_schema as hs             # noqa: E402

PAIR = SCR / "pair" / "bzip2_c2rust"
OUT = SCR / "schemas"
MiB = 1024 * 1024

# role/decode policy per entry, keyed by parameter name.
# "bounded" -> (min, max) or (min_var, max); "buf" -> input/inout buffer with a max_len;
# "outbuf" -> output buffer whose capacity is passed by pointer; "arr" -> fixed-cap output array.
POLICY = {
  "BZ2_bzBuffToBuffCompress": {
    "dest":          {"role": "output_buffer", "capacity_param": "destLen", "cap": 1179648},
    "destLen":       {"role": "capacity_ptr"},
    "source":        {"role": "inout_buffer", "length_param": "sourceLen", "max_len": 1 * MiB},
    "sourceLen":     {"role": "length"},
    "blockSize100k": {"bounded": (1, 9)},
    "verbosity":     {"bounded": (0, 4)},
    "workFactor":    {"bounded": (0, 250)},
  },
  "BZ2_bzBuffToBuffDecompress": {
    "dest":      {"role": "output_buffer", "capacity_param": "destLen", "cap": 4 * MiB},
    "destLen":   {"role": "capacity_ptr"},
    "source":    {"role": "inout_buffer", "length_param": "sourceLen", "max_len": 1 * MiB},
    "sourceLen": {"role": "length"},
    "small":     {"bounded": (0, 1)},
    "verbosity": {"bounded": (0, 4)},
  },
  # huffman.c — `length` carries alphaSize entries, so alphaSize IS its length; the code/limit/
  # base/perm tables are outputs sized to the maxima the C indexes with.
  "BZ2_hbAssignCodes": {
    "code":      {"role": "output_array", "cap": 258},
    "length":    {"role": "inout_buffer", "length_param": "alphaSize", "max_len": 258},
    "minLen":    {"bounded": (0, 23)},
    "maxLen":    {"bounded": (0, 23), "min_var": "minLen"},
    "alphaSize": {"role": "length"},
  },
  "BZ2_hbCreateDecodeTables": {
    # base is indexed as base[length[i]+1] and length is a UChar, so 257 slots are needed to stay
    # in bounds for ANY input, not just the 0..23 the compressor produces.
    "limit":     {"role": "output_array", "cap": 257},
    "base":      {"role": "output_array", "cap": 257},
    "perm":      {"role": "output_array", "cap": 258},
    "length":    {"role": "inout_buffer", "length_param": "alphaSize", "max_len": 258},
    "minLen":    {"bounded": (0, 23)},
    "maxLen":    {"bounded": (0, 23), "min_var": "minLen"},
    "alphaSize": {"role": "length"},
  },
  "BZ2_hbMakeCodeLengths": {
    "len":       {"role": "output_array", "cap": 258},
    "freq":      {"role": "inout_buffer", "length_param": "alphaSize", "max_len": 258},
    "alphaSize": {"role": "length"},
    "maxLen":    {"bounded": (1, 23)},
  },
  "BZ2_indexIntoF": {
    "indx":  {},
    "cftab": {"role": "output_array", "cap": 257},
  },
  # blocksort.c — eclass carries the block; fmap and bhtab are sized from it.
  "fallbackSort": {
    "fmap":   {"role": "output_array", "cap": 4096},
    "eclass": {"role": "inout_buffer", "length_param": "nblock", "max_len": 4096},
    "bhtab":  {"role": "output_array", "cap": 4096},
    "nblock": {"role": "length"},
    "verb":   {"bounded": (0, 4)},
  },
  "fallbackQSort3": {
    "fmap":   {"role": "output_array", "cap": 1024},
    "eclass": {"role": "output_array", "cap": 1024},
    "loSt":   {"bounded": (0, 1023)},
    "hiSt":   {"bounded": (0, 1023), "min_var": "loSt"},
  },
  "fallbackSimpleSort": {
    "fmap":   {"role": "output_array", "cap": 1024},
    "eclass": {"role": "output_array", "cap": 1024},
    "lo":     {"bounded": (0, 1023)},
    "hi":     {"bounded": (0, 1023), "min_var": "lo"},
  },
  "mmed3": {"a": {}, "b": {}, "c": {}},
}

NOTE = {
  "BZ2_indexIntoF": "cftab has no length parameter, so it cannot be filled from the fuzz input; "
                    "both sides get the same zeroed 257-entry table. Memory-safe but the binary "
                    "search is then deterministic — a known limitation of the schema language.",
  "fallbackQSort3": "fmap/eclass have no length parameter, so they are zeroed fixed-cap arrays and "
                    "lo/hi are bounded to their capacity. Memory-safe; the real coverage of this "
                    "function comes transitively from the fallbackSort harness.",
  "fallbackSimpleSort": "same as fallbackQSort3.",
}


def build(entry):
    params, ret, _ = gdh.parse_entry_signature(PAIR / "build", entry)
    pol = POLICY[entry]
    # which buffer owns each length / capacity param (the validator requires exactly one)
    owner_of = {}
    for bname, q in pol.items():
        for ref in ("length_param", "capacity_param"):
            if ref in q:
                owner_of[q[ref]] = bname
    sp = []
    for p in params:
        q = pol.get(p["name"], {})
        e = {"name": p["name"]}
        if p["kind"] == "scalar":
            e.update(rust=p["rust"], width=p["w"])
            if q.get("role") == "length":
                e.update(role="length", decode="derived_from_buffer",
                         of_buffer=owner_of[p["name"]])
            elif "bounded" in q:
                lo, hi = q["bounded"]
                e.update(role="scalar", decode="bounded_scalar", min_value=lo, max_value=hi)
                if "min_var" in q:
                    e["min_var"] = q["min_var"]
            else:
                e.update(role="scalar", decode="scalar")
        else:
            e.update(elem=p["elem"], elem_width=p["elem_w"])
            role = q["role"]
            e["role"] = role
            e["decode"] = hs._ROLE_DECODE[role]
            if role == "capacity_ptr":
                e["of_buffer"] = owner_of[p["name"]]
            if role == "output_buffer":
                # the produced length is written back through the capacity pointer
                e["observable_length"] = {"kind": "capacity_ptr_writeback",
                                          "param": q["capacity_param"]}
            for k in ("capacity_param", "length_param", "cap", "max_len"):
                if k in q:
                    e[k] = q[k]
        sp.append(e)
    schema = {"schema_version": hs.SCHEMA_VERSION, "program": PAIR.name, "entry": entry,
              "params": sp, "return": {"rust": ret},
              "decode_scalars_first": True,
              "provenance": "hand-authored for the RQ4 coverage experiment; every bound is a "
                            "documented bzip2 1.0.8 requirement, see make_schemas.py",
              **({"note": NOTE[entry]} if entry in NOTE else {})}
    errs = hs.validate(schema) + hs.validate_against_signature(schema, params, ret)
    return schema, errs


if __name__ == "__main__":
    OUT.mkdir(exist_ok=True)
    bad = 0
    for entry in POLICY:
        schema, errs = build(entry)
        if errs:
            bad += 1
            print(f"{entry:28s} INVALID: {errs}")
            continue
        (OUT / f"{entry}.json").write_text(json.dumps(schema, indent=1))
        print(f"{entry:28s} ok")
    print(f"\n{len(POLICY) - bad}/{len(POLICY)} schemas written to {OUT}")
    sys.exit(1 if bad else 0)
