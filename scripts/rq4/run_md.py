#!/usr/bin/env python3
"""Assemble the NUMERIC sections of a cell's RUN.md from what the pipeline wrote, so the four
cells of a library are transcribed by one program instead of four hands. Prose (deviations,
what is not established, interpretation) is written by a person and appended after `<!-- prose -->`.

Reads: funnel.json, plans.json, analysis/result.json, analysis@<cp>s/result.json (optional),
divergences/summary.json (optional), confirm_sample/summary.json or confirm/summary.json (optional),
snapshots.json (optional), recollect*.json (optional).

usage: scripts/rq4/run_md.py --cell <dir> --lib bzip2 --tool c2rust --tests-side <json> [--out RUN.md]
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path


def load(p: Path):
    return json.loads(p.read_text()) if p.exists() else None


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--cell", required=True)
    ap.add_argument("--lib", required=True)
    ap.add_argument("--tool", required=True)
    ap.add_argument("--tests-side")
    ap.add_argument("--out")
    a = ap.parse_args()
    cell = Path(a.cell)
    funnel = load(cell / "funnel.json") or []
    plans = load(cell / "plans.json") or []
    res = load(cell / "analysis" / "result.json")
    rep = load(cell / "divergences" / "summary.json") or []
    # the full confirmation (confirm/) when it adjudicated anything, else the labelled sample: a
    # cell with nothing to confirm in full (bzip2 x c2rust) has confirm/summary.json with an
    # empty total, and that must not hide the sample's tally
    conf = load(cell / "confirm" / "summary.json")
    if not (conf or {}).get("total"):
        conf = load(cell / "confirm_sample" / "summary.json") or conf
    snaps = load(cell / "snapshots.json") or {}
    ts = ((load(Path(a.tests_side)) if a.tests_side else None) or {}).get(a.tool) or {}
    L = []
    P = L.append

    P(f"# {a.lib} × {a.tool} — RQ4 cell (plan pipeline)\n")
    P("Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose "
      "after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.\n")

    # 1. funnel
    planned = [p for p in plans if p.get("status") == "planned"]
    built = [r for r in funnel if r.get("built")]
    exported = [r for r in built if str(r.get("coverage", "")).startswith(("batch", "per-input"))]
    P("## 1. Funnel\n")
    P("| stage | n |\n|---|---:|")
    P(f"| matched boundaries (frozen RQ1 output) | {len(plans) or '—'} |")
    P(f"| planned (complete HarnessPlan, lossless bridge for every parameter) | {len(planned) or len(funnel)} |")
    P(f"| built | {len(built)} |")
    P(f"| executed (corpus > 0) | {sum(1 for r in built if (r.get('corpus') or 0) > 0)} |")
    P(f"| coverage exported | {len(exported)} |\n")
    fails = [(p["boundary"], (p.get("failures") or ["?"])[0]) for p in plans if p.get("status") != "planned"]
    if fails:
        from collections import Counter
        c = Counter(f[1].split(" -- ")[-1][:90] for f in fails)
        P("Plan failures, by the generator's own reason:\n")
        for why, n in c.most_common():
            P(f"- **{n}** × {why}")
        P("")
    nb = [r for r in funnel if not r.get("built")]
    if nb:
        P("Planned but not built:\n")
        for r in nb:
            P(f"- `{r['boundary']}`: {(r.get('error') or '')[:120]}")
        P("")

    # 2. per-boundary table
    P("## 2. Per boundary\n")
    P("| boundary | C static | corpus | term. candidates | div. replay | coverage mode |\n|---|---|---:|---:|---|---|")
    rep_by = {r["boundary"]: r for r in rep}
    for r in funnel:
        if not r.get("built"):
            continue
        rr = rep_by.get(r["boundary"])
        div = ", ".join(f"{k} {v}" for k, v in sorted((rr or {}).get("tally", {}).items())) if rr else "—"
        P(f"| `{r['boundary']}` | {'yes' if r.get('c_static') else 'no'} | {r.get('corpus', 0)} | "
          f"{r.get('artifacts', 0)} | {div} | {r.get('coverage')} |")
    P("")

    # 3. tests side
    P("## 3. Tests side\n")
    if ts:
        P(f"Status **{ts.get('status')}**"
          + (f", {ts.get('passed')}/{(ts.get('passed') or 0) + (ts.get('failed') or 0)} passed" if ts.get("passed") is not None else "")
          + f". {ts.get('observation', '')}\n")
    if res:
        P(f"Mode used for the partition: **{res.get('tests_side')}**.\n")

    # 4. coverage
    P("## 3a. Campaign parameters, preflight, generator\n")
    pf = cell / "preflight" / "preflight.json"
    if pf.exists():
        _pf = json.loads(pf.read_text())
        _fl = [b for b, r in _pf["boundaries"].items() if r.get("flag")]
        P(f"Preflight ({_pf['seconds']} s test run + empty-input probe before the campaign): "
          f"{len(_pf['boundaries'])} harnesses, {len(_fl)} crash-all "
          f"({', '.join(f'`{b}`' + (' accepted' if _pf['boundaries'][b].get('accepted') else ' FLAGGED') for b in _fl) or 'none'}).\n")
    _gens = sorted({r.get("generator", "unrecorded") for r in funnel})
    if _gens:
        P(f"Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, "
          f"flatten_translation.py) that built the harnesses: {', '.join(f'`{g}`' for g in _gens)}"
          + (" — more than one: see deviations." if len(_gens) > 1 else "") + "\n")
    cp = cell / "campaign_params.json"
    if cp.exists():
        _cp = json.loads(cp.read_text())
        P("libFuzzer parameters: " + ", ".join(f"`{k}={v}`" for k, v in _cp.items()) + "\n")

    if res:
        P("## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))\n")
        P("| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
        for k in ("function", "region"):
            x = res[k]
            P(f"| {k}s | {x['total_in_scope']} | {x['covered_tests']} | {x['covered_ours']} | {x['covered_both']} | "
              f"{x['only_tests']} | {x['only_ours']} | {x['covered_by_neither']} | {x['tests_coverage']:.3f} | {x['ours_coverage']:.3f} |")
        san = {k: all(res[k].get("sanity", {}).values()) for k in ("function", "region")}
        P(f"\nSanity checks: function {'pass' if san['function'] else 'FAIL'}, region {'pass' if san['region'] else 'FAIL'}. "
          f"Harnesses unioned: {res['harnesses_unioned']}. Identities outside the universe (excluded, never added): "
          f"{res['ours_identities_outside_universe']['functions']} fn / {res['ours_identities_outside_universe']['regions']} reg.\n")
        # budget cross-check
        rows = []
        for cp in (300, 1800):
            b = load(cell / f"analysis@{cp}s" / "result.json")
            if b:
                rows.append((cp, b))
        if rows:
            P("### Budget cross-check from the same campaign (hard-linked snapshots)\n")
            P("| budget | fn ours | reg ours | reg only-ours |\n|---:|---:|---:|---:|")
            for cp, b in rows:
                P(f"| {cp} s | {b['function']['covered_ours']} ({b['function']['ours_coverage']:.3f}) | "
                  f"{b['region']['covered_ours']} ({b['region']['ours_coverage']:.3f}) | {b['region']['only_ours']} |")
            P(f"| 3600 s | {res['function']['covered_ours']} ({res['function']['ours_coverage']:.3f}) | "
              f"{res['region']['covered_ours']} ({res['region']['ours_coverage']:.3f}) | {res['region']['only_ours']} |\n")
        if snaps:
            P("### Corpus growth at the checkpoints (inputs)\n")
            P("| boundary | " + " | ".join(f"{cp} s" for cp in sorted(snaps, key=int)) + " | 3600 s |")
            P("|---|" + "---:|" * (len(snaps) + 1))
            final = {r["boundary"]: r.get("corpus", 0) for r in funnel}
            for b in sorted(final):
                if final[b] <= 1:
                    continue
                P(f"| `{b}` | " + " | ".join(str(snaps[cp].get(b, "")) for cp in sorted(snaps, key=int)) + f" | {final[b]} |")
            P("")

    # 5. replay + confirmation
    if rep:
        agg: dict[str, int] = {}
        for r in rep:
            for k, v in r["tally"].items():
                agg[k] = agg.get(k, 0) + v
        P("## 5. Combined replay of the coverage corpus (protocol §4 step 6)\n")
        P("Outcome tally over every saved corpus input, C reference beside the translation, ladder on:\n")
        P("| outcome | inputs |\n|---|---:|")
        for k, v in sorted(agg.items(), key=lambda kv: -kv[1]):
            P(f"| {k} | {v} |")
        P("")
    if conf:
        P(f"## 6. Confirmation ({conf.get('mode')}"
          + (f", first {conf.get('sample_per_boundary')} artifacts per boundary — a labelled SAMPLE, not the cell's adjudication" if conf.get("sample_per_boundary") else "")
          + ")\n")
        P("| boundary | adjudicated / total | verdicts | clusters |\n|---|---:|---|---:|")
        for b in conf.get("boundaries", []):
            if b.get("error"):
                P(f"| `{b['boundary']}` | — | BUILD/RUN ERROR: {b['error'][:80]} | — |")
                continue
            P(f"| `{b['boundary']}` | {b['adjudicated']} / {b['candidates_total']} | "
              + ", ".join(f"{k} {v}" for k, v in sorted(b['tally'].items())) + f" | {len(b.get('clusters', []))} |")
        P(f"\nTotal: " + ", ".join(f"{k} {v}" for k, v in sorted((conf.get('total') or {}).items())) + "\n")

    P("<!-- prose -->\n")
    text = "\n".join(L)
    if a.out:
        # Regenerating the numbers must never eat the hand-written part: keep whatever already
        # follows the marker in the existing file.
        old = Path(a.out).read_text() if Path(a.out).exists() else ""
        if "<!-- prose -->" in old:
            # the prose is whatever follows the LAST marker; a stale numbers section that ended up
            # after a marker (lil cells: the file was regenerated twice and the copy doubled) is
            # not prose and is dropped
            tail = old.rsplit("<!-- prose -->", 1)[1]
            if "## 1. Funnel" not in tail:
                text = text + tail
        Path(a.out).write_text(text)
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
