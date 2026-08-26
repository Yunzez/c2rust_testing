#!/usr/bin/env python3
"""Seeded generator (NOT coverage-guided): random.seed(42), 200 records covering all 104 indicators and k=0..4,
plus 2 canonical records: 000_noargs (0xFF) and 001_sma5 (sma, k=1, value 5)."""
import random, os, sys, hashlib
out = sys.argv[1]; os.makedirs(out, exist_ok=True)
NAMES = [l.strip() for l in open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "indicators.txt")) if l.strip()]
random.seed(42)
recs = [bytes([0xFF]), bytes([NAMES.index("sma"), 1, 4])]  # 4 % 20 + 1 = 5
for i in range(200):
    idx = i % len(NAMES) if i < len(NAMES) else random.randrange(len(NAMES))
    k = random.randrange(5)
    recs.append(bytes([idx, k] + [random.randrange(256) for _ in range(k)]))
for i, r in enumerate(recs):
    open(os.path.join(out, f"{i:03d}_{hashlib.sha1(r).hexdigest()[:10]}"), "wb").write(r)
print(len(recs))
