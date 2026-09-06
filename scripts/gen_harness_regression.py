#!/usr/bin/env python3
"""Golden-output regression for the differential-harness generator.

Every schema in schemas/ is regenerated and the two files the generator authors -- the fuzz target
and build.rs -- are hashed and compared against a checked-in manifest. Any change to
tools/stu_selector/gen_diff_harness.py or harness_schema.py that alters generated code for an
existing boundary fails here instead of silently changing a frozen experiment.

  python3 scripts/gen_harness_regression.py            # check
  python3 scripts/gen_harness_regression.py --update   # re-freeze after an INTENDED change

Added 2026-09-03 with generator 0.6, after a round of fixes that had to be impact-audited by hand.
Re-frozen 2026-09-03 when C2R_MODE gained the `coverage` setting (an intended change: every
generated target now routes divergence reports through c2r_div so a coverage replay does not abort).
"""
import argparse, hashlib, json, subprocess, sys, tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GEN = ROOT / "tools" / "stu_selector" / "gen_diff_harness.py"
GOLDEN = ROOT / "scripts" / "gen_harness_golden.json"


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()[:16] if path.exists() else None


def build_manifest():
    sys.path.insert(0, str(GEN.parent))
    import gen_diff_harness as gdh
    out = {"gen_version": gdh.GEN_VERSION, "entries": {}}
    for sp in sorted((ROOT / "schemas").glob("*.json")):
        s = json.loads(sp.read_text())
        prog, entry = s["program"], s["entry"]
        with tempfile.TemporaryDirectory() as td:
            r = subprocess.run(
                [sys.executable, str(GEN), "--pair", str(ROOT / "benchmark" / "pairs" / prog),
                 "--entry", entry, "--ub-free", "--out", td],
                capture_output=True, text=True, cwd=str(ROOT))
            if r.returncode != 0:
                out["entries"][f"{prog}/{entry}"] = {"error": (r.stderr or r.stdout).strip()[-200:]}
                continue
            crate = prog.replace("-", "_")
            out["entries"][f"{prog}/{entry}"] = {
                "fuzz_target": digest(Path(td) / "fuzz" / "fuzz_targets" / f"{crate}_ft.rs"),
                "build_rs": digest(Path(td) / "build.rs")}
    # HarnessPlan path (no schema): the RQ4 pairs under benchmark/pairs/rq4. One case per
    # generator feature that has no schema equivalent -- the plan lowering itself, the
    # reshaped-return bridge, the producer bridge in its three parameter shapes.
    for case in PLAN_CASES:
        pair, entry, c_src = case[:3]
        plugin = case[3] if len(case) > 3 else None
        pdir = ROOT / "benchmark" / "pairs" / "rq4" / pair
        if not pdir.exists():
            out["entries"][f"plan:{pair}/{entry}"] = {"error": "pair not in repo"}
            continue
        with tempfile.TemporaryDirectory() as td:
            r = subprocess.run(
                [sys.executable, str(GEN), "--pair", str(pdir), "--entry", entry, "--rust-entry", entry,
                 "--plan", "--ub-free", "--c-source", c_src, "--out", td]
                + (["--plugins", str(ROOT / plugin)] if plugin else []),
                capture_output=True, text=True, cwd=str(ROOT))
            if r.returncode != 0:
                out["entries"][f"plan:{pair}/{entry}"] = {"error": (r.stderr or r.stdout).strip()[-200:]}
                continue
            out["entries"][f"plan:{pair}/{entry}"] = {
                "fuzz_target": digest(Path(td) / "fuzz" / "fuzz_targets" / f"{pair}_ft.rs"),
                "build_rs": digest(Path(td) / "build.rs")}
    return out


PLAN_CASES = [
    ("bzip2_c2rust", "BZ2_bzBuffToBuffCompress", "bzip2lib.c"),      # plan lowering, plan_array + bounded scalars
    ("bzip2_c2saferrust", "BZ2_bzlibVersion", "bzip2lib.c"),         # reshaped return: `&str` for `const char*`
    ("genann_c2rust", "genann_run", "genann.c"),                     # producer bridge, raw-pointer target
    ("genann_crown", "genann_run", "genann.c"),                      # producer bridge, Option<&mut T> target
    ("genann_sactor", "genann_act_sigmoid_cached", "genann.c"),      # producer bridge + scalar return (the #32 site)
    ("cjson_c2rust", "cJSON_GetObjectItem", "cJSON.c", "plugins/cjson/plugin.toml"),  # string producer, two rest-takers -> take_len, plugin object state
    ("lil_c2rust", "lil_parse", "lil.c"),                            # producer through a POINTER TYPEDEF (`lil_t` = *mut _lil_t) + size_t -> u64 scalar cast
    ("lil_c2rust", "lil_append_char", "lil.c"),                      # producer `lil_alloc_double(f64)`: bounded float scalar, producer must be pub
    ("tulip_c2rust", "ti_sma", "tulip.c"),                           # buffer tables: input row policy-allocated (period-bounded loop), output row pointer-ADVANCED (*output++)
    ("tulip_c2rust", "ti_ad", "tulip.c"),                            # buffer tables: 4 input rows + 1 output row, all proven `size`
    ("tulip_c2rust", "ti_bbands", "tulip.c"),                        # buffer tables: 3 output rows incl. an inout row; bitwise f64 row compare
    ("cjson_c2rust", "cJSON_Delete", "cJSON.c", "plugins/cjson/plugin.toml"),         # target IS the destructor: no post-state canon, no second free
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--update", action="store_true")
    a = ap.parse_args()
    cur = build_manifest()
    if a.update or not GOLDEN.exists():
        GOLDEN.write_text(json.dumps(cur, indent=1) + "\n")
        print(f"wrote {GOLDEN.relative_to(ROOT)} for generator {cur['gen_version']}: "
              f"{len(cur['entries'])} entries")
        return 0
    old = json.loads(GOLDEN.read_text())
    bad = []
    for k, v in cur["entries"].items():
        if old["entries"].get(k) != v:
            bad.append(f"{k}: golden {old['entries'].get(k)} -> now {v}")
    for k in old["entries"]:
        if k not in cur["entries"]:
            bad.append(f"{k}: missing from this run")
    if old["gen_version"] != cur["gen_version"]:
        print(f"note: GEN_VERSION {old['gen_version']} -> {cur['gen_version']}")
    if bad:
        print(f"REGRESSION: {len(bad)} of {len(cur['entries'])} entries changed")
        for b in bad:
            print("  " + b)
        return 1
    print(f"ok: {len(cur['entries'])} entries unchanged (generator {cur['gen_version']})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
