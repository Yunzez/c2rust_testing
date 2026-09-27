"""Per-boundary region coverage of the boundary's own function, straight from each arm's llvm-cov export.
usage: compare_fn.py ARMDIR[:label] ... -- boundary ..."""
import json, sys, pathlib
args = sys.argv[1:]; sep = args.index("--"); arms, bounds = args[:sep], args[sep+1:]
def cov(armdir, b):
    f = pathlib.Path(armdir) / "ours" / f"{b}.json"
    if not f.exists(): return None
    d = json.load(open(f))
    best = None
    for fn in d["data"][0]["functions"]:
        n = fn["name"]
        # the boundary's own symbol: mangled name contains the exact identifier, not a longer one
        if (b in n) and not any(x in n for x in ("fuzz", "Cur", "rust_fuzzer")) and \
           (b + "17h" in n or n.endswith(b) or f"{len(b)}{b}17h" in n):
            regs = fn["regions"]; c = sum(1 for r in regs if r[4] > 0)
            best = (c, len(regs), fn.get("count"))
    return best
corp = lambda armdir, b: len(list((pathlib.Path(armdir) / "corpus" / b).glob("*"))) if (pathlib.Path(armdir) / "corpus" / b).is_dir() else None
print(f"{'boundary':22s}" + "".join(f"{a.split(':')[-1]:>26s}" for a in arms))
for b in bounds:
    row = f"{b:22s}"
    for a in arms:
        d = a.split(":")[0]; c = cov(d, b)
        row += f"{'—' if c is None else f'{c[0]}/{c[1]} regs (corp {corp(d,b)})':>26s}"
    print(row)
