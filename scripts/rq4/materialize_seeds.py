#!/usr/bin/env python3
"""Materialise initial-corpus seeds for a cell's boundaries from its archived plans.json, in the
byte layout the generated harness decodes (gen_diff_harness.py: scalars first, then buffer_table /
plan_arr items in declaration order; the cursor yields 0 past the end of the input).

Two arms are produced from ONE deterministic pseudorandom byte string per (boundary, k):
  random : the byte string as is (length-matched control);
  grid   : the same bytes with every element of the boundary's fuzz-filled `options` array
           overwritten by grid value k (all option indices get the same value).
So the paired seeds differ only at the option bytes.  Boundaries whose plan has no fuzz-filled
`options` get no seeds in either arm (the intervention is undefined there) and are listed in the
manifest with the reason.  Layouts using an input kind this script does not model
(input_string, struct values, length-prefixed buffers) are refused explicitly, never guessed.

  materialize_seeds.py --plans plans.json --out DIR --grid 1,2,3,5,10,20 [--prng-seed 7]
writes DIR/random/<boundary>/seed_k, DIR/grid/<boundary>/seed_k and DIR/manifest.json.
"""
import argparse, json, pathlib, random, struct, hashlib

WIDTH = {"c_int": 4, "i32": 4, "c_uint": 4, "u32": 4, "f32": 4, "c_float": 4,
         "c_long": 8, "i64": 8, "u64": 8, "usize": 8, "f64": 8, "c_double": 8,
         "c_short": 2, "i16": 2, "u16": 2, "c_char": 1, "u8": 1, "i8": 1, "c_uchar": 1}
ELEM_FMT = {"f64": "<d", "f32": "<f", "i32": "<i", "u32": "<I", "i64": "<q", "u64": "<Q",
            "u8": "<B", "i8": "<b", "i16": "<h", "u16": "<H"}


def width_of(rust_type: str) -> int:
    t = rust_type.split("::")[-1]
    if t not in WIDTH:
        raise ValueError(f"unknown scalar width for {rust_type}")
    return WIDTH[t]


def layout(entry: dict) -> list[dict]:
    """Ordered byte fields of one boundary's input: [{name, kind, offset, nbytes, ...}]."""
    rank0, rank2 = [], []
    for i in entry["inputs"]:
        d, det, n = i["c_decoder"], i.get("detail", {}), i["param"]
        if d in ("scalar", "bounded_scalar"):
            rank0.append({"name": n, "kind": d, "nbytes": width_of(i["rust_type"]),
                          "min": det.get("min"), "max": det.get("max"), "rust": i["rust_type"]})
        elif d == "input_array":
            w = det["elem_width"]
            if det.get("fills_from_fuzz"):
                rank2.append({"name": n, "kind": "array", "elem": det["elem"], "elem_w": w,
                              "elems": det["alloc_elems"], "nbytes": det["alloc_elems"] * w})
            else:
                rank2.append({"name": n, "kind": "array_zero", "elems": det["alloc_elems"], "nbytes": 0})
        elif d == "buffer_table":
            w = det["elem_width"]
            for r in det["row_specs"]:
                nb = r["alloc_elems"] * w if r.get("fills_from_fuzz") else 0
                rank2.append({"name": f"{n}[{r['row']}]", "kind": "row" if nb else "row_zero",
                              "elem": det["elem"], "elem_w": w, "elems": r["alloc_elems"], "nbytes": nb})
        elif d in ("output_scalar", "out_scalar"):
            continue  # consumes nothing
        else:
            raise ValueError(f"unsupported input kind {d} for {n}")
    fields, off = [], 0
    for f in rank0 + rank2:  # stable: declaration order within each rank
        f["offset"] = off
        off += f["nbytes"]
        fields.append(f)
    return fields


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--plans", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--grid", default="1,2,3,5,10,20")
    ap.add_argument("--prng-seed", type=int, default=7)
    ap.add_argument("--option-param", default="options")
    a = ap.parse_args()
    grid = [float(x) for x in a.grid.split(",")]
    plans = json.load(open(a.plans))
    out = pathlib.Path(a.out)
    manifest = {"plans_sha256": hashlib.sha256(open(a.plans, "rb").read()).hexdigest(),
                "grid": grid, "prng_seed": a.prng_seed, "seeded": {}, "skipped": {}}
    for e in plans:
        b = e["boundary"]
        if e["status"] != "planned":
            manifest["skipped"][b] = "not planned"
            continue
        try:
            fields = layout(e)
        except ValueError as ex:
            manifest["skipped"][b] = str(ex)
            continue
        opt = [f for f in fields if f["name"] == a.option_param and f["kind"] == "array"]
        if not opt:
            manifest["skipped"][b] = f"no fuzz-filled `{a.option_param}` array in the plan"
            continue
        opt = opt[0]
        total = sum(f["nbytes"] for f in fields)
        fmt = ELEM_FMT[opt["elem"]]
        seeds = []
        for k, v in enumerate(grid):
            rng = random.Random(f"{a.prng_seed}:{b}:{k}")
            base = bytearray(rng.getrandbits(8) for _ in range(total))
            hinted = bytearray(base)
            for j in range(opt["elems"]):
                o = opt["offset"] + j * opt["elem_w"]
                hinted[o:o + opt["elem_w"]] = struct.pack(fmt, v if fmt in ("<d", "<f") else int(v))
            for arm, data in (("random", base), ("grid", hinted)):
                d = out / arm / b
                d.mkdir(parents=True, exist_ok=True)
                (d / f"seed_{k}").write_bytes(data)
            seeds.append({"k": k, "value": v, "length": total,
                          "random_sha256": hashlib.sha256(base).hexdigest()[:16],
                          "grid_sha256": hashlib.sha256(hinted).hexdigest()[:16]})
        manifest["seeded"][b] = {"length": total, "options_offset": opt["offset"],
                                 "options_elems": opt["elems"], "fields": fields, "seeds": seeds}
    out.mkdir(parents=True, exist_ok=True)
    (out / "manifest.json").write_text(json.dumps(manifest, indent=1) + "\n")
    print(f"seeded {len(manifest['seeded'])} boundaries x {len(grid)} seeds per arm; "
          f"skipped {len(manifest['skipped'])}; manifest {out/'manifest.json'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
