#!/usr/bin/env python3
"""Apply the 2026-08-09 honesty corrections to results/rq4_effectiveness/reach_census.md.

Run against the committed (HEAD) version of the doc. Splits the file at the
results-table heading BY POSITION FIRST, then edits the prose half, then appends
a freshly generated table — so no inserted cross-reference can collide with the
split anchor.
"""
import glob
import json
import sys

P = "results/rq4_effectiveness/reach_census.md"
SPLIT = "## E3 results table (COMPLETE 2026-07-18)"

s = open(P).read()
if SPLIT not in s:
    sys.exit(f"split anchor not found — is {P} at its committed state?")
head = s[:s.index(SPLIT)]

SUBS = [
    ("- **`ours`** = median per-function entry count after a **uniform 1-hour coverage-guided libFuzzer run**\n"
     "  per cell, obtained by replaying the grown corpus through a `-C instrument-coverage` build and reading\n"
     "  each function's `count` via `llvm-cov export`. Reported as a **lower bound** (corpus-replay counts the\n"
     "  saved corpus, not every one of the run's executions — see caveat).",
     "- **`ours`** = median per-function entry count after a coverage-guided libFuzzer run per cell, obtained\n"
     "  by replaying the grown corpus through a `-C instrument-coverage` build and reading each function's\n"
     "  `count` via `llvm-cov export`. Reported as a **lower bound** (corpus-replay counts the saved corpus,\n"
     "  not every one of the run's executions — see caveat).\n"
     "  **Two medians are reported per cell and they must not be conflated:**\n"
     "  - `median_all` — median over **all** censused functions, unreached ones counted as 0. **This is the\n"
     "    honest headline number**; it is what a claim of the form \"ours ≫ theirs\" has to be measured on.\n"
     "  - `median_reached` — median over functions with count > 0, reported **alongside the reach fraction\n"
     "    `n_reached / n_functions`**, and only meaningful together with it.\n\n"
     "  In **9 of 33 cells `median_all` is 0 — identical to `theirs`** (bzip2 ×3, cJSON ×2, lodepng ×2,\n"
     "  optipng ×2). Those cells say *\"our fuzzing reaches part of the API surface very deeply and never\n"
     "  reaches the rest\"* — a real finding, **not** a depth advantage. Multi-API-surface libraries (a PNG\n"
     "  codec's encoder half, bzip2's file-I/O layer) are where this bites."),

    ("  literally **`0 / 1487`**. The point needs no statistics: **their validation ran the function zero\n"
     "  times; ours ran it thousands.**",
     "  literally **`0 / 1487`**. This is a statement about the *acceptance criterion*, not a measured contrast\n"
     "  between two testing campaigns: `theirs = 0` holds **by construction** of `cargo check`.\n\n"
     "**Baseline honesty — `theirs = 0` is a definition, not a measurement.** A reviewer will (correctly) read\n"
     "\"median 27,043 vs 0\" as a strawman if `0` is left to stand alone, because nobody claims `cargo check`\n"
     "executes code. The defensible framing is narrow: *the tools ship translations whose acceptance evidence\n"
     "contains zero executions of the translated function*. The comparison that would carry real weight — and\n"
     "which E3 does **not** currently make — is against **the library's own shipped test suite** run on the\n"
     "translated Rust (bzip2, cJSON, lodepng, lil all ship tests). Until that is measured, E3 claims *\"the\n"
     "acceptance criterion carries no execution evidence\"*, **not** *\"we test N× harder than they do.\"*"),

    ("**What E3 claims (non-tautological):** current C→Rust translation is accepted with **near-zero\n"
     "per-function execution evidence**, so bugs survive into shipped output; a uniform 1-hour coverage-guided\n"
     "fuzz exercises every function O(10³–10⁵)× deeper, which is *why* the E1 bugs sit in functions the tools\n"
     "had passed. It does **NOT** claim \"X% correct\" (depth bounds *scrutiny*, not correctness) and does **not**\n"
     "showcase the matcher (plain fuzzing reaches the same code — depth-vs-their-evidence is the point).",
     "**What E3 claims:** C→Rust translations are accepted with **near-zero per-function execution evidence** —\n"
     "the tools' acceptance criteria execute the translated code 0 times — while coverage-guided fuzzing reaches\n"
     "a substantial part of the same API surface O(10³–10⁸)× deep. It is a statement about how little evidence\n"
     "the acceptance criterion carries; it is the paper's credibility backstop, not its headline.\n\n"
     "**What E3 does NOT claim:**\n"
     "- **Not** \"X% correct\" — depth bounds *scrutiny*, not correctness.\n"
     "- **Not** a matcher demonstration — plain fuzzing reaches the same code.\n"
     "- **Not a causal explanation of the E1 bugs.** An earlier revision said the depth gap is \"*why* the E1\n"
     "  bugs sit in functions the tools had passed.\" That inference does not hold: `theirs = 0` is constant\n"
     "  across every cell — the 20 bug cells *and* the 16 certificate cells alike — and a constant cannot\n"
     "  explain the variation between them. The honest statement is weaker: the acceptance criterion supplies\n"
     "  no per-function execution evidence **either way**, which is what makes silent defects survivable in\n"
     "  principle; it does not predict *which* cells contain them."),

    ("| `theirs / ours` | median per-fn execution count under the tool's own acceptance **/** under our 1-h fuzz (min in the per-cell detail); `ours` is a lower bound |",
     "| `theirs / ours` | median per-fn execution count under the tool's own acceptance **/** under our fuzz run. `median_all` (zeros included) is the headline; `median_reached` only ever with its reach fraction; `ours` is a lower bound |"),

    ("## The matrix (fillable state derived from E1 final)",
     "## The matrix (fillable state derived from E1 final) — ⚠️ HISTORICAL PLANNING TABLE\n\n"
     "> **This is the 2026-07-10 *work plan*, kept for provenance. Every `∅` below has since been run.**\n"
     "> **The actual results are in the final section of this document — read that one instead.**"),

    ("   run's full execution stream (qsort: 36-file corpus → quickSort 1487, but the 1-h run itself executed",
     "   run's full execution stream (qsort: 36-file corpus → quickSort 1487, but the run itself executed"),

    ("3. **Uniform 1-h budget per cell** — fixes corpus size across cells so counts are comparable within the\n"
     "   protocol; fast libraries (qsort ~63 k exec/s) simply pile up more, which only widens the gap.",
     "3. **The execution budget is NOT uniform across cells, so counts are NOT comparable cell-to-cell.**\n"
     "   Cells ran with libFuzzer `-runs=N` (not `-max_total_time`, which `_Exit()`s and skips the profraw\n"
     "   flush), and `N` was set per cell to fit the library's speed and the wall-clock available: **N ranges\n"
     "   from 4,000 (optipng×Laertes) to 2,000,000 (qsort×PtrTrans, genann×c2rust) — a 500× spread.** Each\n"
     "   cell records its own `runs_budget`.\n"
     "   - **Valid:** `ours` vs `theirs` *within* a cell, and tool-vs-tool *within one library* where N was\n"
     "     held fixed — which it was for the four **tulip** cells (all 30,000) and **genann** (10⁶, c2rust 2×10⁶).\n"
     "   - **Invalid:** raw magnitudes *across* libraries.\n"
     "   An earlier revision claimed a \"uniform 1-hour budget per cell\" and used it to justify cross-cell\n"
     "   comparability. That claim was **false** — the protocol migrated from time-based to runs-based partway\n"
     "   through the campaign (for the flush reason above) and the prose was never updated. Cross-cell\n"
     "   magnitude claims that depended on it have been removed."),

    ("   bound. Framing: on a hard-fault cell our 1-h fuzz *crashed the translation the tool shipped*",
     "   bound. Framing: on a hard-fault cell our fuzz *crashed the translation the tool shipped*"),
]

for a, b in SUBS:
    if a not in head:
        sys.exit("ANCHOR MISS:\n" + a[:120])
    head = head.replace(a, b, 1)

# ---- regenerate the results table from the cell JSONs ----
LIBS = ["qsort", "urlparser", "quadtree", "genann", "cjson", "lil", "lodepng", "bzip2", "tulip", "optipng"]
TOOLS = ["c2rust", "laertes", "c2saferrust", "crown", "sactor", "ptrtrans"]
HDR = {"c2rust": "c2rust", "laertes": "Laertes", "c2saferrust": "C2SaferRust",
       "crown": "CROWN", "sactor": "SACTOR", "ptrtrans": "PtrTrans"}

cells = {}
for f in glob.glob("results/rq4_effectiveness/reach_cells/*.json"):
    d = json.load(open(f))
    cells[(d.get("library"), d.get("tool"))] = d


def fmt(d):
    if d is None:
        return "—"
    if d.get("metric") == "crash-all":
        return "**CRASH-ALL**"
    ma, mr = d.get("median_all"), d.get("median_reached")
    rf = d.get("reach_frac") or "?"
    fl = " *(fl)*" if d.get("metric") == "corpus-replay-floor" else ""
    b = d.get("runs_budget")
    bs = f"<br><sub>N={b:,}</sub>" if b else ""
    if d.get("median_all_src") == "unavailable":
        return f"n/a · {mr:,.0f} r [{rf}]{fl}{bs}"
    head_num = "**0**" if ma == 0 else f"{ma:,.0f}"
    return f"{head_num} · {mr:,.0f} r [{rf}]{fl}{bs}"


rows = "\n".join("| **" + lib + "** | " + " | ".join(fmt(cells.get((lib, t))) for t in TOOLS) + " |"
                 for lib in LIBS)
n_filled = len(cells)
n_zero = sum(1 for d in cells.values() if d.get("median_all") == 0)

table = f"""## E3 results table (COMPLETE 2026-07-18; medians corrected 2026-08-09)

**Cell format: `median_all` · `median_reached` r `[reached/total]`, plus that cell's `-runs` budget.**

- **`median_all`** (bold when 0) — median per-function execution count over **all** censused functions,
  unreached counted as 0. **The headline number.**
- **`median_reached` r** — median over reached functions only (the previous convention). Not
  interpretable without the bracket beside it.
- `[reached/total]` — how much of the censused API surface the fuzzing actually executed.
- `N=` — that cell's `-runs` budget. **Budgets differ by up to 500×; magnitudes are NOT comparable
  between libraries.** Comparable within a library where N is fixed (tulip all 30,000; genann 10⁶).
- *(fl)* = crash-cell corpus-replay floor · **CRASH-ALL** = faults on all/valid input (a finding, not a
  number) · `—` = no artifact (per E1) · `n/a` = `per_fn` not retained, so `median_all` needs a re-run;
  reach is >½ there, so it differs from `median_reached` by less than one order statistic.

| library | {" | ".join(HDR[t] for t in TOOLS)} |
|---|{"---|" * len(TOOLS)}
{rows}

**{n_filled} cells filled — fillable table COMPLETE** (every runnable translation measured).

### The honest reading

**In {n_zero} of {n_filled} cells `median_all` is 0 — the same value as `theirs`.** There, our fuzzing
drives *part* of the API surface very deep and never reaches the rest: bzip2 ×3 (16/35, file-I/O layer
unreached), cJSON ×2 (3/37, 6/121), lodepng ×2 (25/75, 23/75 — the entire encoder half unreached),
optipng ×2 (150/374, 33/374). That is a real finding about **where fuzzing goes**, and it is not a depth
advantage. Reporting only the reached-median, as the previous revision did, concealed it.

The cells carrying a genuine depth result are the high-reach ones: **tulip** (173/224 across all four
tools at fixed N=30,000, median ~27k — the cleanest tool-vs-tool comparison here), **genann** (7/11,
~0.8–1.6M), **qsort** (3/3), **quadtree** (16–18/19).

### Cross-cutting findings
- **CRASH-ALL trio** (lil/bzip2/optipng × C2SaferRust): C2SaferRust's idiomatic rewrites fault on
  all/valid input where the c2rust base runs to full depth — each corroborates an E1 bug *through
  execution depth* (lil c:1 = NonNull-from-null in hm_destroy; optipng c:1 s:2 = deadly signal on every
  PNG; bzip2 c:1 s:1). **Unaffected by the median correction** — these are qualitative outcomes.
- **qsort — the safety paradox**: on the same 3-function quicksort, **CROWN** (a dedicated safety-lifter)
  kept it **100% unsafe** (fatness analysis can't prove `arr.offset(j)` bounds → 0 pointers lifted,
  output ≡ c2rust), while **SACTOR** (LLM) produced **fully safe** `&mut [i32]` + `split_at_mut`. Both
  run at the same order of depth (~40–49M; 3/3 and 6/8 reached) as raw-pointer c2rust — **safety cost no
  execution depth.** Same library, same reach, opposite safety outcomes.
- **tulip = uniform full depth**: all 4 tools reach 173/224 at a **fixed** N=30,000, median ~27k. The one
  methodologically clean cross-tool comparison in the table — and a *negative* result: a well-behaved
  numeric library every tool translates soundly. Contrast with **lil** (3 crash-floor + 1 crash-all).
- **PtrTrans idiomatic never crashes where the base does**: quadtree×PtrTrans runs to full depth without
  faulting while c2rust's translation of the same library is a crash cell. *Stated qualitatively on
  purpose* — the previous revision compared 127M against 178,753, but those are a `total-exec` and a
  `corpus-replay-floor` number at different budgets (10⁵ vs 10⁶), so the ratio meant nothing. The
  crash-vs-no-crash contrast is the finding and needs no ratio.

### Open items
1. **`theirs = 0` is definitional.** Supplement it with the libraries' **own shipped test suites** run
   against the translated Rust (bzip2, cJSON, lodepng, lil all ship tests).
2. **Two cells (`qsort×SACTOR`, `quadtree×PtrTrans`) report `n/a` for `median_all`** — `recensus_mangled.py`
   did not retain `per_fn` and the `/tmp` llvm-cov caches were reclaimed. Re-run to close.
3. **Budgets are not uniform.** Either re-run at fixed N, or keep per-cell N visible (done) and make no
   cross-library magnitude claims (done).
"""

open(P, "w").write(head + table)
print(f"OK — {n_filled} cells, {n_zero} with honest median 0")
