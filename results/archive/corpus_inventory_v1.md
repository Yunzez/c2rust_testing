# Differential corpus inventory (v1, 2026-06-28)

Version-aligned (C, translation) material available **without cloning or spending tokens**, by
combining CROWN's shipped `c-code/` (original C) + `benchmark/` (its aligned c2rust output) with the
C2SaferRust `laertes_benchmarks` (which adds the LLM lane). "Aligned" = the C compiled as oracle is the
same source the Rust translation was derived from, so a divergence is a real bug, not version drift.

## Lanes per program

| program | C_LOC | C | c2rust | CROWN | LLM (C2SaferRust) | Laertes |
|---|--:|:--:|:--:|:--:|:--:|:--:|
| avl | 304 | y | y | y(run) | – | – |
| binn | 3744 | y | y | y(run) | – | – |
| brotli | 21510 | y | y | y(run) | – | – |
| bst | 158 | y | y | y(run) | – | – |
| buffer | 605 | y | y | y(run) | – | – |
| **bzip2** | 7344 | y | y | y(run) | **y** | y |
| **genann** | 951 | y | y | y(run) | **y** | y |
| heman | 8593 | y | y | y(run) | – | – |
| ht | 970 | y | y | y(run) | – | – |
| libcsv | 1027 | y | y | y(run) | – | – |
| libtree | 1837 | y | y | y(run) | – | – |
| libzahl | 2861 | y | y | y(run) | – | – |
| **lil** | 3197 | y | y | y(run) | **y** | y |
| lodepng | 6658 | y | y | y(run) | – | – |
| quadtree | 443 | y | y | y(run) | – | – |
| rgba | 532 | y | y | y(run) | – | – |
| robotfindskitten | 595 | y | y | y(run) | – | – |
| **urlparser** | 70 | y | y | y(run) | **y** | y |
| **tulipindicators** | ~15k | y | y | y(built) | **y** | y |
| optipng | ? | (laertes) | y | – | y | y |
| snudown | ? | (laertes) | y | – | y | y |
| qsort | 90 | y | y | – | y | y |
| grabc | ? | (laertes) | y | – | y | y |
| xzoom | ? | (laertes) | y | – | y | y |

- **y(run)**: CROWN-lifted Rust is producible from `crown/benchmark/<p>` via `crown/run.sh` (rule-based,
  deterministic). **y(built)**: tulip CROWN staticlib already built + wired.

## Totals

- **19 programs** with C + c2rust + CROWN, version-aligned (the 18 CROWN c-code libs minus header-only
  `json.h`, plus tulip). Range 158 LOC (bst) → 21.5k LOC (brotli) — substantial, real programs.
- **5 programs** with the full 4-way incl. the LLM lane: bzip2, genann, lil, urlparser, tulip.
- **+5 laertes-only** with the LLM lane (optipng/snudown/qsort/grabc/xzoom; CROWN absent, C alignment
  per-program). Union ≈ **24 distinct programs**.
- This is the **same benchmark prior work (Laertes/CROWN/C2SaferRust) evaluated on** → numbers are
  directly comparable; using it is a strength, not a self-rolled target.

## The real bottleneck: per-program harnesses, not material

tulip needed only ONE harness because all 104 indicators share a uniform ABI. These 19 programs each
expose a different API → each needs its own differential harness. Mitigations:

1. **c2rust + CROWN lanes preserve names** (`#[no_mangle]`), so they pair by symbol name and a harness
   can be **auto-generated from each C function's signature** (no matcher needed). This is the cheap,
   high-coverage path — covers the 19 aligned programs for C-vs-c2rust and C-vs-CROWN fidelity.
2. **LLM lane (C2SaferRust) mostly preserves names too** here (it rewrites bodies, keeps `extern "C"`),
   so the 5 four-way programs are also name-pairable — the bug-rich hunt.
3. The **matcher (C1)** is for the renamed/idiomatic case (raw LLM, SACTOR, human ports) — a separate,
   forward-looking track, not needed for this corpus.

## CROWN lane PRODUCED (2026-06-28)

Ran `crown/run.sh` over CROWN's own 20 benchmarks → `crown/results/<p>/` (CROWN-lifted Rust, in-place).
All 20 lifted (each has `analysis_results/{ownership,mutability,fatness,statistics}.json`); spot-checked
buffer/genann/libcsv/rgba/bst all **compile** under nightly-2023-01-26 and show real lifts (genann
403→297 raw ptrs, bst 28→21). So we now hold, version-aligned and compiling, for ~19 programs:
**C (`crown/c-code/`) + c2rust (`crown/benchmark/`) + CROWN (`crown/results/`)**.

Decision (user, 2026-06-28): use CROWN's OWN bundled c2rust (the `benchmark/` form) for the CROWN lane —
do NOT force CROWN onto CRUST-bench. Reason: CROWN's rewriter is tightly coupled to its exact c2rust
version. We built old c2rust v0.18 in docker (`crown/oldc2rust/`, image `oldc2rust:0.18`, libc::-form
confirmed) and CROWN's *analysis* runs on it, but its *rewrite* produces non-compiling code (26 errors)
while CROWN's own benchmark/buffer lifts+compiles cleanly — proven by control. Goal was "have CROWN
output"; achieved via run.sh. docker old-c2rust kept for possible future use.

## Next step

Build a harness generator keyed on C signatures (reuse `gen_diff_harness.py` patterns + the tulip
robust-driver trap handling), pilot on a small (urlparser/bst/avl), a medium (genann/libcsv), and a
large/stateful (lil/bzip2) program, then scale across the 19. Hunt LLM bugs on the 5 four-way programs.
