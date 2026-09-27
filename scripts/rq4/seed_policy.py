#!/usr/bin/env python3
"""The frozen global sampling policy (docs/seeding_policy_plan.md section 3) over the partial Seed IR:
plan-guided seeds for every boundary with a placeable policy field, the fallback (cell.py's 64-byte default
seed) for the rest, and a manifest that says which and why.

  seed_policy.py --plans plans.json --pair PAIR --out DIR [--prng-seed 7] [--max-seeds 16] [--max-bytes 2097152]

Values (never per application):
  bounded scalar          {min, min+1, mid, max-1, max} deduplicated
  unbounded signed int    {0, 1, 2, 3, 5, 10, 20, -1}
  unbounded unsigned int  {0, 1, 2, 3, 5, 10, 20}
  unbounded float         {0.0, 0.5, 1.0, 2.0, 5.0, -1.0}
  numeric array, const extent <= 4, placeable: all elements = v for each grid value v (the unbounded set of the
  element type); for extent >= 2 additionally one seed per position with that element = the next grid value.
Seed k assigns every targeted scalar field its k-th value (cycling), so each seed is one joint assignment; the
array variants follow. Filler bytes from PRNG key "<prng-seed>:<boundary>:<k>"; bytes after the first opaque
node are 256 filler bytes. Writes DIR/<boundary>/seed_<k> and DIR/manifest.json."""
import argparse, hashlib, json, pathlib, random, struct, sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / "tools" / "stu_selector"))
import seed_ir as I

SIGNED = [0, 1, 2, 3, 5, 10, 20, -1]; UNSIGNED = [0, 1, 2, 3, 5, 10, 20]; FLOAT = [0.0, 0.5, 1.0, 2.0, 5.0, -1.0]
DEFAULT_SEED = bytes(range(64))


def values_for(nd) -> list:
    if isinstance(nd, I.Scalar):
        if nd.float: return FLOAT
        if nd.bounds:
            lo, hi = nd.bounds; vs = [lo, lo + 1, (lo + hi) // 2, hi - 1, hi]
            out = []
            for v in vs:
                if lo <= v <= hi and v not in out: out.append(v)
            return out
        return SIGNED if nd.signed else UNSIGNED
    if isinstance(nd, I.Repeat):
        return FLOAT if nd.float else (SIGNED if nd.signed else UNSIGNED)
    return []


def policy_fields(nodes):
    pre = I.placeable(nodes)
    sc = [n for n in pre if isinstance(n, I.Scalar)]
    ar = [n for n in pre if isinstance(n, I.Repeat) and n.count.kind == "const" and 1 <= n.count.value <= 4 and n.elem in I._ELEM]
    return sc, ar


def assignment_sets(sc, ar, max_seeds):
    sets = []
    K = max([len(values_for(n)) for n in sc + ar] or [0])
    for k in range(K):
        a = {n.name: values_for(n)[k % len(values_for(n))] for n in sc}
        for n in ar: a[n.name] = [values_for(n)[k % len(values_for(n))]] * n.count.value
        sets.append(a)
    for n in ar:                                     # vary one position at a time (no Cartesian product)
        vs = values_for(n)
        if n.count.value >= 2:
            for pos in range(n.count.value):
                a = {m.name: values_for(m)[0] for m in sc}
                for m in ar: a[m.name] = [values_for(m)[0]] * m.count.value
                row = [vs[0]] * n.count.value; row[pos] = vs[1 % len(vs)]; a[n.name] = row; sets.append(a)
    # canonical order, cap
    sets = [s for s in sets]
    if len(sets) > max_seeds:
        sets = sorted(sets, key=lambda s: json.dumps(s, sort_keys=True))[:max_seeds]
    return sets


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--plans", required=True); ap.add_argument("--pair", required=True); ap.add_argument("--out", required=True)
    ap.add_argument("--prng-seed", type=int, default=7); ap.add_argument("--max-seeds", type=int, default=16); ap.add_argument("--max-bytes", type=int, default=2 << 20)
    a = ap.parse_args()
    plans = json.load(open(a.plans)); aliases = I.load_aliases(a.pair); out = pathlib.Path(a.out); out.mkdir(parents=True, exist_ok=True)
    man = {"policy": "docs/seeding_policy_plan.md section 3 (frozen 2026-09-10)", "plans_sha256": hashlib.sha256(open(a.plans, "rb").read()).hexdigest(),
           "prng_seed": a.prng_seed, "max_seeds": a.max_seeds, "max_bytes": a.max_bytes, "boundaries": {}}
    ng = nf = 0
    for e in plans:
        b = e["boundary"]
        if e.get("status") != "planned": continue
        nodes = I.lower(e, aliases, I.param_order(a.pair, b)); sc, ar = policy_fields(nodes)
        first_opaque = next((n for n in nodes if isinstance(n, I.Opaque)), None)
        if not (sc or ar):
            man["boundaries"][b] = {"kind": "fallback", "why": (f"first opaque: {first_opaque.name}: {first_opaque.why}" if first_opaque else "no policy field") + ("; no placeable scalar or small array" if first_opaque else ""), "ir": I.describe(nodes)}
            nf += 1; continue
        sets = assignment_sets(sc, ar, a.max_seeds); seeds = []; total = 0; d = out / b; d.mkdir(exist_ok=True)
        for k, asg in enumerate(sets):
            rng = random.Random(f"{a.prng_seed}:{b}:{k}")
            data, layout = I.encode(nodes, asg, rng)
            if total + len(data) > a.max_bytes: man["boundaries"][b] = man["boundaries"].get(b) or {}; break
            (d / f"seed_{k}").write_bytes(data); total += len(data)
            seeds.append({"k": k, "assignments": asg,
                          "length": len(data), "sha256": hashlib.sha256(data).hexdigest()[:16], "layout": layout})
        man["boundaries"][b] = {"kind": "plan-guided", "fields": [n.name for n in sc + ar], "first_opaque": (first_opaque.name if first_opaque else None),
                                "seeds": seeds, "dropped_by_cap": max(0, len(sets) - len(seeds)), "ir": I.describe(nodes)}
        ng += 1
    man["summary"] = {"plan_guided": ng, "fallback": nf}
    (out / "manifest.json").write_text(json.dumps(man, indent=1) + "\n")
    print(f"plan-guided {ng}, fallback {nf} -> {out/'manifest.json'}")


if __name__ == "__main__":
    main()
