#!/usr/bin/env python3
"""Diff two harness runs. Exit 1 (and flag) if any cell's recall dropped vs baseline."""
import json, sys
base = json.load(open(sys.argv[1]))
after = json.load(open(sys.argv[2]))
keys = sorted(set(base) | set(after))
drops, ups, same = [], [], 0
for k in keys:
    b = base.get(k, {}).get("recall")
    a = after.get(k, {}).get("recall")
    if b is None or a is None:
        print(f"  ??? {k:22s} base={b} after={a}")
        continue
    if a < b - 1e-6:
        drops.append((k, b, a))
    elif a > b + 1e-6:
        ups.append((k, b, a))
    else:
        same += 1
print(f"unchanged={same}  improved={len(ups)}  REGRESSED={len(drops)}")
if ups:
    print("--- improved ---")
    for k, b, a in ups: print(f"  ↑ {k:22s} {b:.4f} -> {a:.4f}")
if drops:
    print("--- REGRESSIONS (must be zero) ---")
    for k, b, a in drops: print(f"  ↓ {k:22s} {b:.4f} -> {a:.4f}")
    sys.exit(1)
print("OK: no regressions")
