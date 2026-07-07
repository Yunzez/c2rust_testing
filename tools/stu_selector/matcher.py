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
rewarded when its callees/callers correspond. Assignment: optimal 1-1 (Hungarian) on the
converged matrix. Ablation flags: `--no-topo` (per-function baseline), `--greedy`.

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


USE_CONSTS = True   # signal-C toggle (main() clears it for --no-consts ablation)
_CONST_W = 0.20     # weight of the type-tag term, applied post-propagation ONLY when both sides expose tags.
                    # 0.20 = lowest weight that gives cJSON its full gain (0.55) while NOT overriding lil's
                    # confident matches (0.35 regressed lil 0.984->0.969; 0.20-0.25 keep lil intact).
_INPUT_W = 0.12     # weight of the input-element-scalar term (separates typed-container ctor families
                    # e.g. int/float/double array whose only difference is the element type, which the
                    # return-type expansion otherwise drowns). Small: it fires broadly (many fns have inputs).
_SCALARS = {"i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64",
            "bool", "str", "char", "usize", "isize"}


def input_scalars(f) -> set:
    """The set of scalar element/param types on a function's INPUTS only (return type excluded).
    &[i32]->{i32}, const double*->{f64}. Cross-language via the analyzers' scalar normalization."""
    out = set()
    for inp in f["io"]["inputs"]:
        for tok in re.findall(r"[a-z0-9]+", inp.get("shape", "")):
            if tok in _SCALARS:
                out.add(tok)
    return out


def input_sim(fc, fr):
    """Jaccard of input scalar types. None (gate) when either side has no scalar inputs, so
    nullary / pointer-only functions keep their exact score."""
    a = input_scalars(fc)
    b = input_scalars(fr)
    if not a or not b:
        return None
    return jaccard(a, b)


def const_sim(fc, fr):
    """signal-C: jaccard of the TYPE-TAG / enum-variant token sets (analyzer `consts`).
    Returns None when EITHER side has no tags — the gate that makes this term invisible to
    functions that construct no variants (they keep exactly their pre-signal-C score, so a
    library of plain logic functions cannot regress). Discriminates the flat leaf
    constructors (cJSON `Create*`) that share io-shape and have no call topology."""
    a = fc.get("consts") or []
    b = fr.get("consts") or []
    if not a or not b:
        return None
    return jaccard(a, b)


def node_sim(fc, fr, feat="full") -> float:
    """Per-function similarity (NO topology) — the restart prior for propagation.
    feat="shape" = io-SHAPE-ONLY ablation baseline: uses ONLY the normalized parameter/
    return shape CATEGORIES (+ arity), EXCLUDING operators, metrics, topology, AND degree
    — the clean answer to "isn't the type signature already enough?". feat="full" = all
    per-function signals (shape + exact + metrics + operator histogram + arity + signal-C)."""
    exact = 1.0 if shape_sig(fc) == shape_sig(fr) else 0.0
    soft = jaccard(shape_tokens(fc), shape_tokens(fr))
    arity = 1.0 if len(fc["io"]["inputs"]) == len(fr["io"]["inputs"]) else 0.0
    if feat == "shape":
        return 0.70 * soft + 0.20 * exact + 0.10 * arity
    met = metric_sim(fc, fr)
    ops = op_sim(fc, fr)
    return 0.35 * soft + 0.15 * exact + 0.15 * met + 0.25 * ops + 0.10 * arity


def apply_consts(sim, c_data, r_data) -> dict:
    """Blend signal-C into the FINAL similarity matrix (post-propagation), so the type-tag
    discriminator votes at full assignment weight instead of being diluted to (1-alpha) as
    a restart prior — critical for flat leaf constructors whose topology term is a uniform
    ~1.0 (empty neighbor sets 'agree') that would otherwise drown it. Gated: only pairs
    where BOTH sides expose tags are adjusted; every other pair keeps its exact prior score,
    so a library of tag-less logic functions cannot regress."""
    if not USE_CONSTS:
        return sim
    cf = {f["name"]: f for f in c_data["functions"]}
    rf = {f["name"]: f for f in r_data["functions"]}
    out = dict(sim)
    for (c, r), s in sim.items():
        v = s
        cs = const_sim(cf[c], rf[r])
        if cs is not None:
            v = (1.0 - _CONST_W) * v + _CONST_W * cs
        isim = input_sim(cf[c], rf[r])
        if isim is not None:
            v = (1.0 - _INPUT_W) * v + _INPUT_W * isim
        out[(c, r)] = v
    return out


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
    """mean over a in A of (max over b in B of S[a,b]) — neighbor-set best-match.
    A/B are sets; iterate them SORTED so the float summation order is fixed. Without this,
    set iteration order varies with PYTHONHASHSEED and float non-associativity flips near-ties
    in homogeneous clusters (e.g. cJSON's 12 leaf constructors) => non-reproducible recall."""
    tot = 0.0
    for a in sorted(A):
        best = 0.0
        for b in sorted(B):
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


def _dfcap_clean(callees, callers, n_funcs, cap) -> tuple:
    """Drop 'stopword' neighbors from the neighbor-sets: a node called by >cap fraction
    of the program (high in-degree, e.g. an assert helper `require` called by 89% of
    fns) carries no discriminative topology signal -- like 'the' in IR. Removed from
    callee-sets by in-degree, from caller-sets by out-degree. cap None/>=1 disables.
    Threshold 0.5 separates fake Rust-only hubs (require ~0.89) from real shared
    primitives (lil_to_string ~0.33) with a wide margin; validated by the lil gate."""
    if cap is None or cap >= 1.0 or n_funcs == 0:
        return callees, callers, []
    indeg = {n: len(callers[n]) for n in callers}
    outdeg = {n: len(callees[n]) for n in callees}
    drop_callee = {n for n, d in indeg.items() if d / n_funcs > cap}
    drop_caller = {n for n, d in outdeg.items() if d / n_funcs > cap}
    callees = {k: v - drop_callee for k, v in callees.items()}
    callers = {k: v - drop_caller for k, v in callers.items()}
    return callees, callers, sorted(drop_callee | drop_caller)


def propagate(c_data, r_data, alpha=0.7, iters=15, df_cap=0.5, feat="full") -> dict:
    """IsoRank-style propagation with neighbor-set best-match topology. Returns the
    converged similarity {(c,r): score}. df_cap removes hub 'stopword' neighbors that
    otherwise poison propagation (see _dfcap_clean)."""
    cn = [f["name"] for f in c_data["functions"]]
    rn = [f["name"] for f in r_data["functions"]]
    cf = {f["name"]: f for f in c_data["functions"]}
    rf = {f["name"]: f for f in r_data["functions"]}
    c_callees, c_callers = adjacency(c_data)
    r_callees, r_callers = adjacency(r_data)
    c_callees, c_callers, _ = _dfcap_clean(c_callees, c_callers, len(cn), df_cap)
    r_callees, r_callers, _ = _dfcap_clean(r_callees, r_callers, len(rn), df_cap)
    N = {(c, r): node_sim(cf[c], rf[r], feat) for c in cn for r in rn}
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


def _hungarian(cost) -> list:
    """Min-cost assignment, O(n^2 m), n<=m (e-maxx). Returns row->col."""
    INF = float("inf")
    n, m = len(cost), len(cost[0])
    u, v = [0.0] * (n + 1), [0.0] * (m + 1)
    p, way = [0] * (m + 1), [0] * (m + 1)
    for i in range(1, n + 1):
        p[0] = i
        j0 = 0
        minv = [INF] * (m + 1)
        used = [False] * (m + 1)
        while True:
            used[j0] = True
            i0, delta, j1 = p[j0], INF, -1
            for j in range(1, m + 1):
                if not used[j]:
                    cur = cost[i0 - 1][j - 1] - u[i0] - v[j]
                    if cur < minv[j]:
                        minv[j], way[j] = cur, j0
                    if minv[j] < delta:
                        delta, j1 = minv[j], j
            for j in range(m + 1):
                if used[j]:
                    u[p[j]] += delta
                    v[j] -= delta
                else:
                    minv[j] -= delta
            j0 = j1
            if p[j0] == 0:
                break
        while j0:
            j1 = way[j0]
            p[j0] = p[j1]
            j0 = j1
    res = [-1] * n
    for j in range(1, m + 1):
        if p[j] > 0:
            res[p[j] - 1] = j - 1
    return res


def node_matrix(c_data, r_data, feat="full") -> dict:
    """The per-function (no-topology) similarity prior, as {(c,r): node_sim}. Reused as
    the propagation restart vector and as the baseline for topo_delta diagnostics."""
    cf = {f["name"]: f for f in c_data["functions"]}
    rf = {f["name"]: f for f in r_data["functions"]}
    return {(c, r): node_sim(cf[c], rf[r], feat) for c in cf for r in rf}


def _assign_hungarian(cn, rn, sim) -> list:
    """Forced optimal 1-1 assignment (every smaller-side node matched). Ablation control
    for --no-partial; the default path is _assign_partial."""
    swap = len(cn) > len(rn)
    rows, cols = (rn, cn) if swap else (cn, rn)
    cost = [[-sim[(c, r)] if not swap else -sim[(r, c)] for r in cols] for c in rows]
    assign = _hungarian(cost)
    mapping = []
    for i, j in enumerate(assign):
        if j < 0:
            continue
        c, r = (cols[j], rows[i]) if swap else (rows[i], cols[j])
        mapping.append((c, r, sim[(c, r)]))
    return mapping


def _assign_partial(cn, rn, sim, tau) -> tuple:
    """Partial bipartite matching with a per-node dummy outside-option: a C function is
    matched to a Rust function only when that beats NOT matching (worth -tau). Both
    C-only and Rust-only nodes may stay unmatched, so no surplus Rust node (LLM-added
    scaffolding/helpers) is force-consumed. tau is a LOW floor / outside option, NOT a
    quality gate: real matches can score ~0.10 while a mid-similarity pollutant scores
    ~0.43, so absolute score cannot separate them -- what fixes the bad assignment is
    relaxing the forced bijection, not the threshold value."""
    n, m = len(cn), len(rn)
    # cols = m real Rust + n dummy (one per C, so each can independently opt out)
    cost = [[-sim[(c, r)] for r in rn] + [-tau] * n for c in cn]
    assign = _hungarian(cost)  # n rows <= m+n cols
    matched, c_only, used_r = [], [], set()
    for i, j in enumerate(assign):
        if 0 <= j < m:
            matched.append((cn[i], rn[j], sim[(cn[i], rn[j])]))
            used_r.add(rn[j])
        else:  # routed to a dummy column -> deliberately unmatched
            c_only.append(cn[i])
    rust_only = [r for r in rn if r not in used_r]
    return matched, c_only, rust_only


def _two_sided_conf(sim, cn, rn) -> callable:
    """A confidence function for a pair (c,r): min of the C-side and R-side margins,
    i.e. how clearly r wins for c AND c wins for r. A many-way tie (tinyexpr builtins,
    where one Rust fn is near-equal best for many C fns) yields ~0 from the R-side -> low
    confidence -> ambiguous. A pair that is the clear mutual best (even a homogeneous
    cluster once topology has separated it, e.g. lil fnc_*) keeps a real margin. NOTE:
    this catches AMBIGUITY (many equal candidates), NOT confident-wrong (a clean 2-cycle
    swap like bignum to_int<->to_string has high two-sided margin) -- that needs signal-C."""
    c_top = {c: sorted(((sim[(c, r)], r) for r in rn), reverse=True)[:2] for c in cn}
    r_top = {r: sorted(((sim[(c, r)], c) for c in cn), reverse=True)[:2] for r in rn}

    def conf(c, r):
        v = sim[(c, r)]
        alt_c = c_top[c][0][0] if c_top[c][0][1] != r else (c_top[c][1][0] if len(c_top[c]) > 1 else 0.0)
        alt_r = r_top[r][0][0] if r_top[r][0][1] != c else (r_top[r][1][0] if len(r_top[r]) > 1 else 0.0)
        return min(v - alt_c, v - alt_r)
    return conf


def match(c_data, r_data, topo=True, alpha=0.7, iters=15, assign="hungarian",
          partial=True, tau=0.05, df_cap=0.5, abstain_eps=None, feat="full") -> dict:
    """Return {matched, ambiguous, c_only, rust_only, sim}. matched/ambiguous entries are
    (c, r, score, confidence). When abstain_eps is set, pairs with two-sided confidence
    below it are moved from `matched` to `ambiguous` (isolated, not guessed). Default None
    = accept all (no abstention) so baseline numbers are unchanged. feat="shape" =
    io-shape-only ablation (forces topo off at the call site)."""
    cn = [f["name"] for f in c_data["functions"]]
    rn = [f["name"] for f in r_data["functions"]]
    sim = propagate(c_data, r_data, alpha, iters, df_cap, feat) if topo else node_matrix(c_data, r_data, feat)
    sim = apply_consts(sim, c_data, r_data)
    if partial:
        pairs, c_only, rust_only = _assign_partial(cn, rn, sim, tau)
    elif assign == "hungarian":
        pairs = _assign_hungarian(cn, rn, sim)
        mc, mr = {c for c, _, _ in pairs}, {r for _, r, _ in pairs}
        c_only = [c for c in cn if c not in mc]
        rust_only = [r for r in rn if r not in mr]
    else:  # greedy ablation
        used_c, used_r, pairs = set(), set(), []
        for s, c, r in sorted(((s, c, r) for (c, r), s in sim.items()), reverse=True):
            if c in used_c or r in used_r:
                continue
            used_c.add(c); used_r.add(r)
            pairs.append((c, r, s))
        c_only = [c for c in cn if c not in used_c]
        rust_only = [r for r in rn if r not in used_r]
    conf = _two_sided_conf(sim, cn, rn)
    scored = [(c, r, s, conf(c, r)) for (c, r, s) in pairs]
    eps = float("-inf") if abstain_eps is None else abstain_eps
    matched = [t for t in scored if t[3] >= eps]
    ambiguous = [t for t in scored if t[3] < eps]
    return {"matched": matched, "ambiguous": ambiguous, "c_only": c_only,
            "rust_only": rust_only, "sim": sim}


def diagnose(c_data, r_data, sim, node, truth) -> None:
    """Per-C rank diagnostics (the user's point 3): best/second/margin, and (with truth)
    the true match's rank + topo_delta = sim_topo - sim_node (how far propagation moved
    THIS pair -- negative on the true pair = topology poisoning). Plus per-side hubs
    (point 5): high in-degree (df) nodes that propagation tends to over-attract."""
    cn = [f["name"] for f in c_data["functions"]]
    rn = [f["name"] for f in r_data["functions"]]
    has_t = bool(truth)
    hdr = f"{'C func':22s} {'best_rust':20s} {'best':>6} {'2nd':>6} {'margin':>6}"
    if has_t:
        hdr += f" | {'true_rust':20s} {'true':>6} {'rank':>4} {'dTtrue':>7} {'dTbest':>7}"
    print(hdr)
    print("-" * len(hdr))
    for c in cn:
        scored = sorted(((sim[(c, r)], r) for r in rn), reverse=True)
        best_s, best_r = scored[0] if scored else (0.0, "-")
        second_s = scored[1][0] if len(scored) > 1 else 0.0
        line = f"{c:22s} {best_r:20s} {best_s:6.3f} {second_s:6.3f} {best_s - second_s:6.3f}"
        if has_t:
            tr = truth.get(c)
            if tr in rn:
                trank = next(i for i, (s, r) in enumerate(scored) if r == tr)
                d_true = sim[(c, tr)] - node[(c, tr)]
                d_best = best_s - node[(c, best_r)]
                line += (f" | {tr:20s} {sim[(c, tr)]:6.3f} {trank:4d} "
                         f"{d_true:+7.3f} {d_best:+7.3f}")
            else:
                line += f" | {str(tr):20s} {'--':>6} {'--':>4} {'--':>7} {'--':>7}"
        print(line)
    _, c_in = adjacency(c_data)
    _, r_in = adjacency(r_data)
    top = lambda inn: sorted(((len(v), k) for k, v in inn.items()), reverse=True)[:3]
    print(f"\nhubs (in-degree df)  C: {top(c_in)}   Rust: {top(r_in)}")


def main() -> int:
    ap = argparse.ArgumentParser(description="Name-independent C<->Rust function matcher")
    ap.add_argument("--c", required=True)
    ap.add_argument("--rust", required=True)
    ap.add_argument("--no-topo", action="store_true",
                    help="disable call-graph topology propagation (per-function baseline)")
    ap.add_argument("--shape-only", action="store_true",
                    help="ablation: io-SHAPE-ONLY node similarity (normalized param/return "
                    "categories + arity; NO operators/metrics/topology/degree). Forces topo off.")
    ap.add_argument("--alpha", type=float, default=0.7, help="topology weight (0..1)")
    ap.add_argument("--iters", type=int, default=15, help="propagation iterations")
    ap.add_argument("--greedy", action="store_true",
                    help="greedy assignment instead of Hungarian (ablation; with --no-partial)")
    ap.add_argument("--no-partial", action="store_true",
                    help="force full assignment (no dummy outside-option) -- ablation")
    ap.add_argument("--tau", type=float, default=0.05,
                    help="partial-matching outside-option floor (LOW; not a quality gate)")
    ap.add_argument("--df-cap", type=float, default=0.5,
                    help="topology hub-stopword cap: drop neighbors called by >cap "
                    "fraction of the program (1.0 disables)")
    ap.add_argument("--truth", help="hand-labeled ground-truth JSON {c_name: rust_name} "
                    "for renamed translations (LLM track); default = name equality")
    ap.add_argument("--diag", action="store_true",
                    help="print per-C rank diagnostics (best/2nd/margin, true rank, topo_delta, hubs)")
    ap.add_argument("--abstain-eps", type=float, default=None,
                    help="abstention: move pairs with two-sided confidence < eps from "
                    "matched to ambiguous (isolate, don't guess). Default off (accept all).")
    ap.add_argument("--no-consts", action="store_true",
                    help="disable signal-C (type-tag/enum-variant term) -- ablation control")
    ap.add_argument("-v", "--verbose", action="store_true")
    ap.add_argument("--emit-pairs", default=None,
                    help="write the matched C->Rust correspondence as JSON {c_name: rust_name} "
                    "(for the end-to-end driver to drive gen_diff_harness --rust-entry)")
    args = ap.parse_args()
    globals()["USE_CONSTS"] = not args.no_consts
    c_data = json.loads(Path(args.c).read_text())
    r_data = json.loads(Path(args.rust).read_text())

    feat = "shape" if args.shape_only else "full"
    topo = (not args.no_topo) and not args.shape_only  # shape-only excludes topology
    res = match(c_data, r_data, topo=topo, alpha=args.alpha, iters=args.iters,
                assign="greedy" if args.greedy else "hungarian",
                partial=not args.no_partial, tau=args.tau, df_cap=args.df_cap,
                abstain_eps=args.abstain_eps, feat=feat)
    matched, ambiguous = res["matched"], res["ambiguous"]
    c_only, rust_only = res["c_only"], res["rust_only"]

    # ground truth: name equality (faithful, name-preserving c2rust) OR a hand-labeled
    # map (LLM track, where the translator renames so name-equality no longer holds).
    truth = json.loads(Path(args.truth).read_text()) if args.truth else None
    if truth is not None:
        is_correct = lambda c, r: truth.get(c) == r
        label = "labeled"
        scorable = sum(1 for f in c_data["functions"] if f["name"] in truth)
    else:
        is_correct = lambda c, r: c == r
        label = "name-equal"
        scorable = len(c_data["functions"])
    correct = sum(1 for (c, r, s, k) in matched if is_correct(c, r))

    print(f"C functions: {len(c_data['functions'])} | Rust functions: {len(r_data['functions'])}")
    scaf = r_data.get("excluded_scaffolding")
    if scaf:
        items = [f"{e['name']}[{e['reason']}]" if isinstance(e, dict) else str(e) for e in scaf]
        print(f"excluded_scaffolding (Rust, {len(scaf)}): {items}")
    print(f"matched: {len(matched)} | ambiguous: {len(ambiguous)} | "
          f"c_only_unmatched: {len(c_only)} | rust_only_unmatched: {len(rust_only)}")
    # CORRECT line kept stable for the regression gate (counts accepted/matched only).
    print(f"CORRECT ({label}): {correct} | accuracy: {correct}/{scorable} = "
          f"{100 * correct // max(scorable, 1)}%")
    if args.abstain_eps is not None:
        amb_wrong = sum(1 for (c, r, s, k) in ambiguous if not is_correct(c, r))
        prec = 100 * correct // max(len(matched), 1)
        cov = 100 * len(matched) // max(scorable, 1)
        print(f"abstain eps={args.abstain_eps}: accepted-precision {correct}/{len(matched)} "
              f"= {prec}% | coverage {len(matched)}/{scorable} = {cov}% | "
              f"ambiguous {len(ambiguous)} ({amb_wrong} would-be-wrong isolated)")
    if args.verbose:
        for c, r, s, k in sorted(matched, key=lambda x: -x[2]):
            print(f"  [{'OK ' if is_correct(c, r) else 'XX '}] {c:24s} -> {r:24s}  score={s:.3f} conf={k:+.3f}")
        for c, r, s, k in sorted(ambiguous, key=lambda x: -x[2]):
            print(f"  [AMB] {c:24s} -> {r:24s}  score={s:.3f} conf={k:+.3f}")
        if c_only:
            print(f"  c_only:    {c_only}")
        if rust_only:
            print(f"  rust_only: {rust_only}")
    if args.emit_pairs:
        Path(args.emit_pairs).write_text(
            json.dumps({c: r for (c, r, s, k) in matched}, indent=1), encoding="utf-8")
        print(f"wrote {len(matched)} pairs -> {args.emit_pairs}")
    if args.diag:
        print()
        diagnose(c_data, r_data, res["sim"], node_matrix(c_data, r_data), truth)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
