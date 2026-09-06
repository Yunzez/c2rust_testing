#!/usr/bin/env python3
"""The coverage UNIVERSE of a translation, exported from the instrumented rlib's own object files
rather than from a linked bin.

Why: a bin built with -C link-dead-code keeps every function of every archive member the linker
PULLS IN -- but a member is pulled only if some symbol in it is referenced. If the reference call
is small enough for rustc's automatic cross-crate inlining (PtrTrans's `cJSON_Version()`), the
bin never references the rlib at all and the universe collapses to two functions. `#[no_mangle]`
translations never showed this (exported symbols are not inlined away), which is why the bin route
looked right on c2rust / Laertes / CROWN. The rlib's codegen units contain a coverage record for
every function the crate compiled; exporting them with an unrelated profile yields all of them at
count 0 -- the universe, independent of what any bin referenced.

usage: rlib_universe.py <cargo target dir> <crate name> <profdata> <out.json> [<cargo json log>]

The rlib is taken from cargo's own `--message-format=json-render-diagnostics` output when that
log is given (the exact artifact of THIS build); the newest-by-mtime rlib is only a fallback and
is reported as such.
"""
import glob, json, os, subprocess, sys, tempfile

TC = os.path.expanduser("~/.rustup/toolchains/nightly-2025-09-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin")


def rlib_from_cargo_json(log: str, crate: str) -> str | None:
    """The lib artifact of this build, from cargo's JSON message stream."""
    for line in open(log, encoding="utf-8", errors="replace"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            m = json.loads(line)
        except ValueError:
            continue
        if m.get("reason") != "compiler-artifact" or m.get("target", {}).get("name") != crate:
            continue
        if "lib" not in m["target"].get("kind", []):
            continue
        for f in m.get("filenames", []):
            if f.endswith(".rlib"):
                return f
    return None


def main(target, crate, profdata, out, cargo_json=None):
    rlib = rlib_from_cargo_json(cargo_json, crate) if cargo_json and os.path.exists(cargo_json) else None
    how = "cargo json"
    if not rlib:
        rlibs = sorted(glob.glob(f"{target}/release/deps/lib{crate}-*.rlib"), key=os.path.getmtime)
        if not rlibs:
            sys.exit(f"no rlib for {crate} under {target}")
        rlib = rlibs[-1]
        how = "newest by mtime (FALLBACK: no cargo json artifact line)"
    with tempfile.TemporaryDirectory() as w:
        subprocess.run(["ar", "x", os.path.abspath(rlib)], cwd=w, check=True)
        objs = sorted(p for p in glob.glob(f"{w}/*.o") if ".rcgu.o" in p)
        if not objs:
            sys.exit(f"no codegen objects in {rlib}")
        cmd = [f"{TC}/llvm-cov", "export", objs[0]] + [x for o in objs[1:] for x in ("-object", o)] + [f"-instr-profile={profdata}"]
        r = subprocess.run(cmd, capture_output=True, text=True)
        if r.returncode != 0:
            sys.exit(r.stderr[:500])
        d = json.loads(r.stdout)
    for f in d["data"][0]["functions"]:
        f["count"] = 0
        for r_ in f.get("regions", []):
            r_[4] = 0
    d["_source"] = {"rlib": os.path.basename(rlib), "selected_by": how, "objects": len(objs), "note": "universe from the instrumented rlib objects, counts zeroed"}
    json.dump(d, open(out, "w"))
    print(f"{out}: {len(d['data'][0]['functions'])} functions from {len(objs)} object(s) of {os.path.basename(rlib)} [{how}]")


if __name__ == "__main__":
    main(*sys.argv[1:6])
