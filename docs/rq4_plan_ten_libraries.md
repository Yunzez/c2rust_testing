# RQ4 — extending the campaign to all ten libraries (plan, 2026-09-07)

*Five libraries are done (19 cells: bzip2 ×4, genann ×5, cJSON ×2, lil ×4, tulip ×4;
`results/rq3_coverage/SUMMARY_ALL.md`). This page is the plan for the other five — qsort, urlparser,
quadtree, lodepng, optipng — under the same protocol (`results/rq3_coverage/PROTOCOL.md`), the same
runbook (`docs/rq4_runbook.md`) and the same frozen generator. Nothing here changes what a cell
measures; it only says which artifacts exist, where each came from, what the tests side is, and in
what order they run.*

## 0. What counts as a cell

A cell is one (library × translation) whose translation **builds** — the same rule as the first
five. Translation-process failures (`✗` in `results/rq4_effectiveness/translation_matrix.md`) are
not cells: SACTOR on quadtree/lodepng/optipng, PtrTrans on lodepng/optipng, CROWN on optipng.
E1's *UB-gate exclusions* (urlparser × Laertes/CROWN) **are** cells here: RQ4 measures reach of the
translation, and the UB gate acts per input inside confirmation (`ub_associated*`, never promoted),
so a translation whose C reference is UB on its main path still has a measurable reach and a
measurable construction funnel. urlparser × SACTOR / PtrTrans have no artifact (never run) — not cells.

| library | c2rust | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans | new cells |
|---|---|---|---|---|---|---|---:|
| qsort | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | **6** |
| urlparser | ✓ | ✓ | ✓ | ✓ | no artifact | no artifact | **4** |
| quadtree | ✓ | — | — | ✓ | ✗ circular | ✓ | **3** |
| lodepng | ✓ | — | — | ✓ | ✗ translate | ✗ compile | **2** |
| optipng | ✓ | ✓ | ✓ | ✗ analyse | ✗ parse | ✗ PA | **3** |

18 new cells → **37 cells over ten libraries** (E3 measured 33 of these; urlparser's four are new
to any table). Every c2rust cell stays the negative control (0 confirmed expected).

## 1. Sources and translations per cell (provenance to record in `artifact_hashes.json`)

Rule from the runbook: the pair's C is **what the translator consumed**, hashed per cell. Where two
copies of a library exist in the tree they are different revisions (quadtree below), so the copy is
chosen per tool, never per library.

### qsort (30 LOC, 3 functions; no suite)

| tool | translation | C consumed |
|---|---|---|
| c2rust | `tools/frameworks/c2saferrust/laertes_benchmarks/qsort/qsort.rs` | `laertes_benchmarks/qsort/qsort.c` |
| Laertes | `laertes_benchmarks/qsort_laertes/qsort.rs` (+ its `laertes_rt`) | same |
| C2SaferRust | `laertes_benchmarks/qsort_WIP/qsort.rs` (`swap(&mut i32,&mut i32)`, `quickSort(&mut [i32], usize, usize)`) | same |
| CROWN | `fuzz/qsort_crown_e3lite/src/lib.rs` (our C → c2rust → CROWN run, E1 #20; workspace gone, this is the kept copy) | same |
| SACTOR | `fuzz/qsort_sactor_e3lite/src/lib.rs` (`quick_sort(&mut [i32], i32, i32)`; renamed) | same |
| PtrTrans | `results/rq4_effectiveness/bugs/qsort_ptrtrans/translated_qsort.rs` (= `fuzz/qsort_ptrtrans_e3/src/lib.rs`; `Option<&mut [i32]>`, `quick_sort` renamed) | `original_qsort.c` there = the same file |

Tests side: qsort ships no test target → `TEST-UNAVAILABLE` ×6, universe from the rlib.
Renames (`quickSort → quick_sort`, SACTOR and PtrTrans) come from the RQ1 ground truth
(`results/rq1_matching/annotation/{sactor,ptrtrans}_qsort/labels.json`, matcher and manual agree)
and are passed to the pipeline as `translated/renames.json` — the first time RQ4 consumes a matcher
map. A reshaped signature the frozen bridge cannot construct (`Option<&mut [i32]>`) is recorded as
*construction unsupported*, not patched (user decision 2026-09-05).

### urlparser (url.h ≈ 560 lines, header-only; test.c is the shipped suite)

| tool | translation | C consumed |
|---|---|---|
| c2rust | `laertes_benchmarks/urlparser/test.rs` | `laertes_benchmarks/urlparser/{url.h,test.c}` |
| Laertes | `laertes_benchmarks/urlparser_laertes/test.rs` | same |
| C2SaferRust | `laertes_benchmarks/urlparser_WIP/test.rs` | same |
| CROWN | `tools/frameworks/crown/results/urlparser/src/test.rs` (`main` commented out → adapter calls `main_0`) | `crown/c-code/urlparser/url.h` (check = same) |

The translation unit **is** the test program (`test.c` includes `url.h`), so the scored module
carries the driver's `main_0` too; the RUN.md says so, and the driver's regions are reported
separately (function-level exclusion in the cell table is by name: `main`, `main_0`). Tests side:
`make test` = run `test.rs`'s main → adapter as for genann (`test::main()` / `src::test::main_0()`).
Expected: the C reference heap-overflows in `url_parse → get_part` on every normal URL
(E1 #27) → every boundary through `url_parse` is `ub_associated`; UB-free boundaries
(`url_is_protocol`, `url_is_ssh`, the `url_get_*` family where the C side stays clean) are the
ones that can confirm. E1's C2SaferRust `url_is_ssh` UTF-8 panic (bug #2) is the re-find target.

### quadtree (4 C files, 24 functions; test.c is the shipped suite)

| tool | translation | C consumed |
|---|---|---|
| c2rust | `tools/c2rust_crustbench/out/quadtree/src/*.rs` + `test.rs` (`fuzz/quadtree_c2rust_e3` is a copy) | `tools/c2rust_crustbench/out/quadtree/src/*.c` — the **newer upstream** revision (`insert_` returns 0/1/2, `node_contains_` uses `<=`) |
| CROWN | `tools/frameworks/crown/results/quadtree/src/{src/*.rs,test.rs}` | `crown/c-code/quadtree-0.1.0/src/*.c` (0.1.0 revision) |
| PtrTrans | `tools/frameworks/ptrtrans_rebuild/PtrTrans-C2Rust/dataset/Trans_C-Rust-KG/quadtree/src/*.rs` (the variant the E1 cell used, verified byte-equal to `fuzz/quadtree_ptrtrans_e3`) | `PtrTrans-C2Rust/dataset/crown_dataset/quadtree/src/*.c` (0.1.0, reformatted) |

Two C revisions, chosen per tool — the lil provenance lesson (E1 #15). Amalgamation
`quadtree_all.c` = `#include` of point.c, bounds.c, node.c, quadtree.c (no static collisions).
Tests side: `make test` → `test.c`; driver `test.rs` present for c2rust and CROWN → measured;
PtrTrans has none → `TEST-UNAVAILABLE`. PtrTrans's `Option<Box<T>>` / `Option<&T>` reshaping is
the same shape cJSON × PtrTrans could not construct; expect a small built count and say why.

### lodepng (6 658-line single TU; C++ unit test only)

| tool | translation | C consumed |
|---|---|---|
| c2rust | `fuzz/lodepng_c2rust_e3/src/src/lodepng.rs` (c2rust of the file below; the kept copy) | `tools/frameworks/crown/c-code/lodepng/lodepng.c` @ `997936f` |
| CROWN | `tools/frameworks/crown/results/lodepng/src/lodepng.rs` | same |

Tests side: `lodepng_unittest.cpp` is C++ and was never transpiled → `TEST-UNAVAILABLE` ×2. ~300
functions: the first planning job of lodepng's size after tulip; `-max_len` stays 4096 (PNG
inputs are byte buffers; the planner's length-carrying buffers are the inputs).

### optipng (52-file build set ≈ 50 kLOC: libpng 1.6.34 + zlib 1.2.11 + pngxtern/pnmio/gifread/minitiff/opngreduc)

| tool | translation | C consumed |
|---|---|---|
| c2rust | `laertes_benchmarks/optipng/src/**.rs` (58 modules incl. `zlib/test/{example,minigzip}.rs`, `libpng/pngtest.rs` — drivers, unscored) | `tools/frameworks/optipng-0.7.7/src/**` with `pnglibconf.h.optipng` (PROVENANCE.txt) |
| Laertes | `laertes_benchmarks/optipng_laertes/src/**.rs` | same |
| C2SaferRust | `laertes_benchmarks/optipng_WIP/src/**.rs` | same |

Amalgamation `optipng_all.c` over the 52 TUs the Rust tree has modules for; two static-variable
collisions (`options` in optim.c/optipng.c, `err_png_ptr` in pngxrgif.c/pngxrtif.c) are handled
with `#define`/`#undef` around the second include — the sources are untouched. Tests side:
`make test` = `bitset_test`, `ratio_test`, gifread/minitiff self-tests; none transpiled →
`TEST-UNAVAILABLE` ×3 (zlib's `example.rs` is zlib's own test, not optipng's target — noted, not
used). Cost: each harness compiles the whole translation; plan first, build only planned
boundaries, and expect the three cells to be the longest of the study.

## 2. Order and gates

1. **qsort ×6** — smallest; exercises renames and two reshaped-signature translators through the
   pipeline before anything expensive. Gate: the c2rust cell plans 3/3 and confirms 0.
2. **quadtree ×3** — small, two C revisions, one PtrTrans reshaping. Gate: tests side PASS on
   c2rust and CROWN before the campaign (adapter must build).
3. **urlparser ×4** — the UB-reference library; the RUN.md must show the `ub_associated` share.
4. **lodepng ×2** — big single TU.
5. **optipng ×3** — biggest; last, so a quota death costs nothing else.

Per library, unchanged from the runbook: pairs → tests side (or denominator) → plan the whole
library and read the failure reasons → build ONE harness from the negative-control pair → serial
chain (`run_<lib>_serial.sh`, detached, free-space and file-count guards) with **preflight** →
post (`post_<lib>.sh`) → `cell_table.py` → `SUMMARY.md` → manifest for `confirmed_*` only →
`summary_all.py` / `paper_tables.py` with the library added to their lists.

## 3. Pipeline changes this plan needs (plumbing, not capability)

- `translated/renames.json` (`{"C name": "Rust name"}`) read by `harness_plan.py --all`,
  `c2r_funnel.py` and `cell.py`, passed as `--rust-entry`. Source of truth is the RQ1 label
  file; the generator already accepts `--rust-entry`. Absent file = identity, so the 19 finished
  cells are unaffected (their generator hash changes only if these files change).
- `tests_side_quadtree.sh`, `tests_side_urlparser.sh` on the genann template (driver = `test`).
- `scripts/rq4/make_pair.py`: source copy + `compile_commands.json` + flatten, so eighteen pairs
  are built one way and the C hash is recorded at build time.
- `summary_all.py` / `paper_tables.py` `LIBS` lists gain the five libraries; `NEW_IN_RQ4` gains
  whatever the manifest gains.

## 4. What is pre-registered for these cells (PROTOCOL amendment 2026-09-07)

Budget 3 600 s, seed 42, snapshots 60/300/600/1 800 s, `-max_len 4096` (no buffer-table rows
here), preflight 60 s + empty-input probe, one campaign one corpus, sample confirmation 200 per
channel. Tests-side rule table: see the amendment in `PROTOCOL.md` §2. Nothing about a cell's
budget or universe is decided after its numbers are seen.
