"""t=0 corpora for each arm: cell.py's 64-byte default seed + the arm's seeds. usage: E plans.json"""
import json, pathlib, shutil, sys
E = pathlib.Path(sys.argv[1]); plans = json.load(open(sys.argv[2]))
for arm in ("base", "random", "grid"):
    for p in plans:
        if p["status"] != "planned": continue
        b = p["boundary"]; d = E / "t0" / arm / "corpus" / b; d.mkdir(parents=True, exist_ok=True)
        (d / "seed").write_bytes(bytes(range(64)))
        if arm != "base" and (E / "seeds" / arm / b).is_dir():
            for f in (E / "seeds" / arm / b).iterdir(): shutil.copy(f, d / f.name)
print("t0 corpora written")
