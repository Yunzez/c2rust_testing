#!/usr/bin/env python3
"""Name-independent C<->Rust function matcher (the project's core novelty).

Inputs: the C-side (c_analyzer.py) and Rust-side (analyzer) per-function JSON.
Pairs functions using STRUCTURAL signals only — names are NEVER used for matching:
  - io shape (input shapes + output shape): exact match + soft token Jaccard
  - comparable metrics (cyclomatic, stmts, nodes, loops, max_loop_depth, derefs, allocs)
  - operator histogram (cosine) — separates structurally-identical, different-op twins
  - call-graph TOPOLOGY: IsoRank-style propagation where neighbor agreement uses a
    neighbor-set best-match (mean-of-max, symmetrized), NOT a cartesian average.
Per-function signals are the restart prior N; topology propagates them so a pair is
rewarded when its callees/callers correspond. Assignment: greedy 1-1 on the converged
matrix (`--no-topo` falls back to the per-function baseline for ablation).

Validation: names ARE present in faithful c2rust output, so the name-equal pairing is the
ground truth. We match WITHOUT names, then score against it -> precision/recall/accuracy.

Usage: matcher.py --c c.json --rust rust.json
"""
import argparse
import json
import math
import re
from collections import Counter
from pathlib import Path

COMPARABLE = ["cyclomatic", "stmts", "nodes", "loops", "max_loop_depth", "derefs", "allocs"]
_TOK = re.compile(r"[A-Za-z0-9_]+|\*|&|\{|\}|\[|\]|<|>|;")


def shape_sig(f) -> tuple:
    return tuple(i["shape"] for i in f["io"]["inputs"]) + ("->", f["io"]["output"]["shape"])


def shape_tokens(f) -> list:
    return _TOK.findall("|".join(shape_sig(f)))


def jaccard(a, b) -> float:
    ca, cb = Counter(a), Counter(b)
    inter = sum((ca & cb).values())
    union = sum((ca | cb).values())
    return inter / union if union else 1.0


def degrees(data) -> tuple:
    names = {f["name"] for f in data["functions"]}
    out, inn = Counter(), Counter()
    for e in data["raw_edges"]:
        if e["from"] in names:
            out[e["from"]] += 1
        if e["to"] in names:
            inn[e["to"]] += 1
    return out, inn


def metric_sim(fc, fr) -> float:
    mc, mr = fc.get("metrics"), fr.get("metrics")
    if not mc or not mr:
        return 0.0
    acc = 0.0
    for k in COMPARABLE:
        a, b = mc.get(k, 0), mr.get(k, 0)
        acc += abs(a - b) / (1 + max(a, b))
    return 1 - acc / len(COMPARABLE)


def op_sim(fc, fr) -> float:
    """Cosine similarity of the operator histograms (the / vs % discriminator)."""
    a, b = fc.get("ops", {}), fr.get("ops", {})
    if not a and not b:
        return 1.0
    keys = set(a) | set(b)
    dot = sum(a.get(k, 0) * b.get(k, 0) for k in keys)
    na = math.sqrt(sum(v * v for v in a.values()))
    nb = math.sqrt(sum(v * v for v in b.values()))
    return dot / (na * nb) if na and nb else 0.0


def deg_sim(nc, nr, dc, dr) -> float:
    oc, ic = dc[0][nc], dc[1][nc]
    orr, ir = dr[0][nr], dr[1][nr]
    denom = 1 + oc + orr + ic + ir
    return 1 - (abs(oc - orr) + abs(ic - ir)) / denom


def score(fc, fr, dc, dr) -> float:
    exact = 1.0 if shape_sig(fc) == shape_sig(fr) else 0.0
    soft = jaccard(shape_tokens(fc), shape_tokens(fr))
    met = metric_sim(fc, fr)
    ops = op_sim(fc, fr)
    deg = deg_sim(fc["name"], fr["name"], dc, dr)
    # arity mismatch is a hard-ish penalty
    arity = 1.0 if len(fc["io"]["inputs"]) == len(fr["io"]["inputs"]) else 0.0
    return 0.30 * soft + 0.15 * exact + 0.15 * met + 0.20 * ops + 0.10 * deg + 0.10 * arity


def node_sim(fc, fr) -> float:
    """Per-function similarity (NO topology) — the restart prior for propagation."""
    exact = 1.0 if shape_sig(fc) == shape_sig(fr) else 0.0
    soft = jaccard(shape_tokens(fc), shape_tokens(fr))
    met = metric_sim(fc, fr)
    ops = op_sim(fc, fr)
    arity = 1.0 if len(fc["io"]["inputs"]) == len(fr["io"]["inputs"]) else 0.0
    return 0.35 * soft + 0.15 * exact + 0.15 * met + 0.25 * ops + 0.10 * arity


def adjacency(data) -> tuple:
    """name -> (set of local callees, set of local callers), from raw_edges."""
    names = {f["name"] for f in data["functions"]}
    callees = {n: set() for n in names}
    callers = {n: set() for n in names}
    for e in data["raw_edges"]:
        a, b = e["from"], e["to"]
        if a in names and b in names:
            callees[a].add(b)
            callers[b].add(a)
    return callees, callers


def _dir(A, B, S, c_first) -> float:
    """mean over a in A of (max over b in B of S[a,b]) — neighbor-set best-match."""
    tot = 0.0
    for a in A:
        best = 0.0
        for b in B:
            v = S[(a, b)] if c_first else S[(b, a)]
            if v > best:
                best = v
        tot += best
    return tot / len(A)


def _setsim(ac, br, S) -> float:
    """Symmetrized neighbor-set similarity (each side's neighbors find their best
    correspondent). 1.0 if both empty (agree on no neighbors), 0.0 if exactly one
    empty (connectivity mismatch). NOT a cartesian average."""
    if not ac and not br:
        return 1.0
    if not ac or not br:
        return 0.0
    return 0.5 * _dir(ac, br, S, True) + 0.5 * _dir(br, ac, S, False)


def propagate(c_data, r_data, alpha=0.7, iters=15) -> dict:
    """IsoRank-style propagation with neighbor-set best-match topology. Returns the
    converged similarity {(c,r): score}."""
    cn = [f["name"] for f in c_data["functions"]]
    rn = [f["name"] for f in r_data["functions"]]
    cf = {f["name"]: f for f in c_data["functions"]}
    rf = {f["name"]: f for f in r_data["functions"]}
    c_callees, c_callers = adjacency(c_data)
    r_callees, r_callers = adjacency(r_data)
    N = {(c, r): node_sim(cf[c], rf[r]) for c in cn for r in rn}
    S = dict(N)
    for _ in range(iters):
        new = {}
        for c in cn:
            for r in rn:
                terms = []
                if c_callees[c] or r_callees[r]:
                    terms.append(_setsim(c_callees[c], r_callees[r], S))
                if c_callers[c] or r_callers[r]:
                    terms.append(_setsim(c_callers[c], r_callers[r], S))
                if not terms:  # isolated on both sides -> no topology signal
                    new[(c, r)] = N[(c, r)]
                else:
                    topo = sum(terms) / len(terms)
                    new[(c, r)] = (1 - alpha) * N[(c, r)] + alpha * topo
        S = new
    return S


def match(c_data, r_data, topo=True, alpha=0.7, iters=15) -> list:
    if topo:
        S = propagate(c_data, r_data, alpha, iters)
        pairs = [(s, c, r) for (c, r), s in S.items()]
    else:
        dc, dr = degrees(c_data), degrees(r_data)
        pairs = [(score(fc, fr, dc, dr), fc["name"], fr["name"])
                 for fc in c_data["functions"] for fr in r_data["functions"]]
    pairs.sort(reverse=True)
    used_c, used_r, mapping = set(), set(), []
    for s, c, r in pairs:
        if c in used_c or r in used_r:
            continue
        used_c.add(c)
        used_r.add(r)
        mapping.append((c, r, s))
    return mapping


def main() -> int:
    ap = argparse.ArgumentParser(description="Name-independent C<->Rust function matcher")
    ap.add_argument("--c", required=True)
    ap.add_argument("--rust", required=True)
    ap.add_argument("--no-topo", action="store_true",
                    help="disable call-graph topology propagation (per-function baseline)")
    ap.add_argument("--alpha", type=float, default=0.7, help="topology weight (0..1)")
    ap.add_argument("--iters", type=int, default=15, help="propagation iterations")
    ap.add_argument("-v", "--verbose", action="store_true")
    args = ap.parse_args()
    c_data = json.loads(Path(args.c).read_text())
    r_data = json.loads(Path(args.rust).read_text())

    mapping = match(c_data, r_data, topo=not args.no_topo, alpha=args.alpha, iters=args.iters)
    # ground truth = name equality (valid for faithful, name-preserving c2rust)
    r_names = {f["name"] for f in r_data["functions"]}
    gt_pairs = [(c, r, s) for (c, r, s) in mapping if c in r_names]
    correct = sum(1 for (c, r, s) in mapping if c == r)
    n_c = len(c_data["functions"])
    n_pred = len(mapping)

    print(f"C functions: {n_c} | Rust functions: {len(r_data['functions'])}")
    print(f"predicted pairs: {n_pred} | CORRECT (name-equal): {correct}")
    print(f"accuracy: {correct}/{n_c} = {100 * correct // max(n_c, 1)}%")
    if args.verbose:
        for c, r, s in sorted(mapping, key=lambda x: -x[2]):
            mark = "OK " if c == r else "XX "
            print(f"  [{mark}] {c:24s} -> {r:24s}  score={s:.3f}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
