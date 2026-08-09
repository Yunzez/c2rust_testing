#!/usr/bin/env python3
"""Regenerate results/e3_matrix.html from the corrected cell JSONs.

Reuses the existing CSS block verbatim (lines up to and including </style>) so
the visual identity is unchanged; regenerates every number, the tiles, the
findings and the caveats from results/rq3_cells/*.json.
"""
import glob
import json
import re

SRC = "results/e3_matrix.html"
LIBS = [("qsort", "sorting · 3 fn"), ("quadtree", "spatial tree · 24 fn"),
        ("genann", "neural net · ~20 fn"), ("cjson", "JSON parser · 58 fn"),
        ("lil", "interpreter · 145 fn"), ("lodepng", "PNG codec · 235 fn"),
        ("bzip2", "compressor · 64 fn"), ("tulip", "indicators · ~100 fn"),
        ("optipng", "PNG optimizer · ~400 fn"), ("urlparser", "URL parsing · 21 fn")]
TOOLS = ["c2rust", "laertes", "c2saferrust", "crown", "sactor", "ptrtrans"]
HDR = ["c2rust", "Laertes", "C2SaferRust", "CROWN", "SACTOR", "PtrTrans"]
# cells corroborating a confirmed E1 bug
STAR = {("qsort", "c2saferrust"), ("qsort", "ptrtrans"), ("cjson", "ptrtrans"),
        ("genann", "sactor"), ("lil", "c2saferrust"), ("bzip2", "laertes"),
        ("bzip2", "c2saferrust"), ("bzip2", "crown"), ("tulip", "c2saferrust"),
        ("optipng", "laertes"), ("optipng", "c2saferrust")}

cells = {}
for f in glob.glob("results/rq3_cells/*.json"):
    d = json.load(open(f))
    cells[(d.get("library"), d.get("tool"))] = d


def si(v):
    if v is None:
        return "n/a"
    v = float(v)
    for lim, suf in ((1e9, "B"), (1e6, "M"), (1e3, "k")):
        if v >= lim:
            s = f"{v/lim:.1f}".rstrip("0").rstrip(".")
            return s + suf
    return f"{v:.0f}"


def td(lib, tool):
    d = cells.get((lib, tool))
    star = ' <span class="star">★</span>' if (lib, tool) in STAR else ""
    if d is None:
        return '<td><span class="cell c-na">—</span></td>'
    if d.get("metric") == "crash-all":
        return f'<td><span class="cell c-crash">CRASH-ALL{star}</span></td>'
    ma, mr = d.get("median_all"), d.get("median_reached")
    rf = d.get("reach_frac") or "?"
    n = d.get("runs_budget")
    fl = "floor " if d.get("metric") == "corpus-replay-floor" else ""
    nb = f" · N={si(n)}" if n else ""
    if d.get("median_all_src") == "unavailable":
        return (f'<td><span class="cell c-deep">{si(mr)}<span class="rr">'
                f'reached-only · {fl}{rf}{nb}</span></span></td>')
    if ma == 0:
        return (f'<td><span class="cell c-zero">0<span class="rr">'
                f'reached {si(mr)} · {fl}{rf}{nb}</span></span></td>')
    cls = "c-floor" if fl else "c-deep"
    return (f'<td><span class="cell {cls}">{si(ma)}{star}<span class="rr">'
            f'reached {si(mr)} · {fl}{rf}{nb}</span></span></td>')


rows = []
for lib, role in LIBS:
    tds = "".join(td(lib, t) for t in TOOLS)
    rows.append(f'          <tr>\n            <td class="lib">{lib}'
                f'<span class="role">{role}</span></td>\n            {tds}\n          </tr>')

n_cells = len(cells)
n_zero = sum(1 for d in cells.values() if d.get("median_all") == 0)
n_crash = sum(1 for d in cells.values() if d.get("metric") == "crash-all")

css = open(SRC).read()
css = css[:css.index("</style>") + len("</style>")]
# an honest-zero cell needs its own colour; append one rule to the existing sheet
css = css.replace("</style>", """  .c-zero{color:var(--crash);background:var(--na-bg);border:1px dashed var(--line)}
</style>""")

html = css + f"""

<div class="wrap">
  <p class="eyebrow">E3 · Table 3 · depth of the bug hunt · medians corrected 2026-08-09</p>
  <h1>Not how much we cover — <span class="dim">how deep we hit</span></h1>
  <p class="thesis">Plain fuzzing covers the same code, so breadth showcases nothing. E3 measures
    <b>depth</b>: how many times our differential fuzzer drives each translated Rust function. The tools
    accept a translation when it <b>compiles</b> — their acceptance criterion executes it <b>0 times</b>.
    But the honest median counts the functions our fuzzing <i>never reached</i> as well, and in
    <b>{n_zero} of {n_cells} cells that median is also 0</b>. This page shows both numbers, because only
    one of them is a depth result.</p>
  <div class="meta">
    <span>10 libraries × 6 translators</span><span>{n_cells} runnable cells — table complete</span>
    <span>headline = median over ALL functions</span><span>budgets 4k–2M (500× spread)</span>
    <span>cargo-fuzz · -C instrument-coverage</span>
  </div>

  <div class="tiles">
    <div class="tile"><div class="n">{n_cells}</div><div class="k">Cells measured</div><div class="sub">every runnable translation · fillable table done</div></div>
    <div class="tile crash"><div class="n">{n_zero}</div><div class="k">Cells whose honest median is 0</div><div class="sub">same value as <i>theirs</i> — reach, not depth</div></div>
    <div class="tile tgt"><div class="n">173<small>/224</small></div><div class="k">Cleanest comparison</div><div class="sub">tulip · 4 tools · fixed N=30,000 · ~27k</div></div>
    <div class="tile crash"><div class="n">{n_crash}</div><div class="k">CRASH-ALL cells</div><div class="sub">C2SaferRust faults on all/valid input</div></div>
  </div>

  <hr class="rule">

  <section>
    <h2>The matrix — one cell per tool-translation</h2>
    <p class="sub" style="color:var(--ink-3);font-size:13px;margin:2px 0 14px">Big number = median over
      <b>all</b> censused functions (unreached counted as 0). Small line = the reached-only median, the
      reach fraction, and that cell's <code class="inl">-runs</code> budget. <b>Budgets differ by up to
      500×, so magnitudes are not comparable between libraries</b> — only within a library at fixed N.</p>
    <div class="scroll">
      <table class="mx">
        <thead><tr>
          <th class="lib">library<span class="role">domain · ~#fn</span></th>
          {"".join(f"<th>{h}</th>" for h in HDR)}
        </tr></thead>
        <tbody>
{chr(10).join(rows)}
        </tbody>
      </table>
    </div>
    <div class="legend">
      <span><span class="sw" style="background:var(--deep)"></span><b>27k</b> median exec/fn — ran to depth</span>
      <span><span class="sw" style="background:var(--floor)"></span><b>floor</b> crash-cell corpus-replay lower bound</span>
      <span><span class="sw" style="background:var(--na-bg);border:1px dashed var(--line)"></span><b>0</b> honest median 0 — fuzzing reached &lt;½ the functions</span>
      <span><span class="sw" style="background:var(--crash)"></span><b>CRASH-ALL</b> faults on all/valid input</span>
      <span><span class="sw" style="background:var(--na)"></span><b>—</b> no runnable artifact</span>
      <span><span class="star">★</span> corroborates a confirmed E1 bug</span>
    </div>
  </section>

  <section>
    <h2>What the table says</h2>
    <div class="two">
      <div class="card spot">
        <h3><span class="m">◆ headline</span> The safety paradox — qsort</h3>
        <p>On one 3-function quicksort, <b>CROWN</b> (a dedicated safety-lifter) kept it 100%
        <code class="inl">*mut i32</code> while <b>SACTOR</b> (an LLM) produced fully safe
        <code class="inl">&amp;mut [i32]</code> — at the same order of depth (~40–49M), both at full reach.
        Safety cost no execution depth. <a href="qsort_safety_paradox.html">Read the case study →</a></p>
      </div>
      <div class="card">
        <h3><span class="m">◆</span> CRASH-ALL trio corroborates E1</h3>
        <p>lil / bzip2 / optipng × <b>C2SaferRust</b> fault on all/valid input where the c2rust base runs
        to full depth. A qualitative outcome, so it is <b>unaffected by the median correction</b> — a
        second, independent witness for each of those three E1 bugs.</p>
      </div>
      <div class="card">
        <h3><span class="m">◆</span> tulip vs lil — risk lives in pointers</h3>
        <p>The one methodologically clean cross-tool comparison here: all four tools on <b>tulip</b> at a
        <b>fixed</b> N=30,000, same reach (173/224), same ~27k median. A <i>negative</i> result. The
        pointer- and recursion-heavy interpreter <b>lil</b> gives 3 crash-floors + 1 crash-all instead.</p>
      </div>
      <div class="card">
        <h3><span class="m">◆</span> Where the fuzzer never goes</h3>
        <p>In <b>{n_zero} cells</b> the honest median is 0: lodepng's entire <b>encoder</b> half (25/75,
        23/75), optipng's non-PNG readers (150/374, 33/374), bzip2's file-I/O layer (16/35), cJSON
        (3/37, 6/121). Multi-API-surface libraries need <b>more harnesses</b>, not more fuzz-hours.</p>
      </div>
    </div>
  </section>

  <section>
    <h2>What this table does <i>not</i> claim</h2>
    <div class="callout">Three corrections applied 2026-08-09. <b>(1)</b> The budget is <b>not</b> uniform
      — an earlier revision claimed a "uniform 1-hour budget per cell" and used it to justify cross-cell
      comparison; actual budgets run 4,000–2,000,000. <b>(2)</b> The median was taken over <b>reached</b>
      functions only, which hid the {n_zero} cells above. <b>(3)</b> The depth gap is <b>not</b> a causal
      explanation of the E1 bugs: <code class="inl">theirs = 0</code> is constant across bug cells and
      certificate cells alike, and a constant cannot explain the difference between them.</div>
    <p style="font-size:13px;color:var(--ink-3)"><b>The <code class="inl">theirs = 0</code> baseline is
      definitional, not measured</b> — nobody claims <code class="inl">cargo check</code> executes code.
      The comparison that would carry real weight, and which E3 does not yet make, is against each
      <b>library's own shipped test suite</b> run on the translated Rust (bzip2, cJSON, lodepng and lil
      all ship tests). Until then E3 claims only that the acceptance criterion carries no per-function
      execution evidence. Two cells (<code class="inl">qsort×SACTOR</code>,
      <code class="inl">quadtree×PtrTrans</code>) show a reached-only median because their per-function
      census was not retained; both have reach &gt;½, so the two medians differ by less than one order
      statistic.</p>
  </section>

  <footer>E3 · per-function hit-depth · {n_cells} cells · rows = E1 corpus · Rust-side ·
    results/rq3_master_table.md · medians corrected 2026-08-09 · dataset-v2.1</footer>
</div>
"""

open(SRC, "w").write(html)
print(f"wrote {SRC}: {n_cells} cells, {n_zero} honest-zero, {n_crash} crash-all")
