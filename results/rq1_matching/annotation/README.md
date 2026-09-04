# RQ1 group B — manual-labeling scaffolds for the renaming translators (PtrTrans, SACTOR)

Generated 2026-09-01 by `scripts/rq1_group_b_scaffold.py --all`. Which translator × library
cells have analyzable output at all, and why the others are N/A, is documented in
`../group_b_availability.md` — read that first. **No `truth` label has been written yet for
any artifact**; nothing below is a Matching-Accuracy result.

Each directory holds `sheet.csv` (one row per C function; fill `truth` / `truth_note`),
`sheet.json` (rows + raw matcher output + fingerprint: matcher/analyzer commits, artifact
and C-source hashes), `rust_inventory.csv` (every Rust function with who claims it) and a
`README.md` with the artifact facts and the labeling rules. Raw analyzer / matcher output
per case is archived in `../raw/group_b/<case>/`. The labeling rules are identical across
all seven; read one.

Duplicate leaf names are detected, never silently resolved: a Rust leaf defined twice makes
`name_eq` = `DUPLICATE` and a tool claim on it `tool_claim_defined_in_rust` = `DUPLICATE`.
(No artifact currently has any.)

## Buildable translations

| artifact | split | C fns | Rust fns | tool map | claim ≠ C name | no same-name Rust fn | elided | matcher abstains | rows to decide | status |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---|
| [`ptrtrans_qsort/`](ptrtrans_qsort/) | eval | 3 | 3 | yes (3 claims) | 1 (`quickSort → quick_sort`) | 1 | 0 | 1 (`partition`) | 1 | unlabeled |
| [`sactor_qsort/`](sactor_qsort/) | eval | 3 | 3 (+`main` dropped) | yes (`function_name_map.json`) | 1 (`quickSort → quick_sort`) | 1 | 0 | 2 | 2 | unlabeled |
| [`ptrtrans_quadtree/`](ptrtrans_quadtree/) | eval | 24 | 17 | yes (16 claims) | 0 | 7 | 7 (`*_free`, `*_reset` → `Drop`) | 6 | 3 | unlabeled |
| [`sactor_genann/`](sactor_genann/) | eval | 15 | 15 (+`main` dropped) | none | — | 0 | 0 | 2 | 0 conflicts (15 single-source) | unlabeled |
| [`ptrtrans_cjson/`](ptrtrans_cjson/) | **dev** | 113 | 108 | none retained | — | 7 | — | 83 | 48 | unlabeled |

## Non-buildable translations (call graph 1–2 % resolved — see `../topology_resolution.md`)

| artifact | split | C fns | Rust fns | tool map | claim ≠ C name | no same-name Rust fn | elided | matcher abstains | rows to decide | status |
|---|---|---:|---:|---|---:|---:|---:|---:|---:|---|
| [`ptrtrans_bzip2/`](ptrtrans_bzip2/) | eval | 64 | 61 | yes (61 claims) | 19 (all camelCase → snake_case) | 22 | 3 | 48 | 35 | unlabeled |
| [`ptrtrans_lodepng/`](ptrtrans_lodepng/) | eval | 235 | 252 | yes (226 claims, 2 undefined) | 126 (63 record-shifted, 21 placeholder/libc, rest snake-case) | 36 | 3 | 234 | 205 | unlabeled — **do not label all 235 up front** |

"rows to decide" = rows whose `prior` is `CONFLICT` (the three sources disagree) plus, for
tools without a map, rows where only the matcher proposes. Any P/R/abstention/coverage
computed from the two lower rows is a result about matching *under degraded static
analysis* and is reported on a separate line from the upper rows.

## Two things the scaffolds already show (facts about the inputs, not accuracy)

- **Forced ≠ deployment on qsort.** PtrTrans qsort: forced mode assigns all 3; deployment
  (eps = 0.01) abstains on `partition`, so the deployment coverage is 2/3. The earlier
  "matcher 3/3 on qsort×PtrTrans" was forced mode; the paper must report both.
- **SACTOR's idiomatic qsort scrambles topology.** SACTOR nests helpers inside functions
  (`partition` carries its own `do_swap`; `quick_sort` recurses through an inner fn), so the
  C edge `partition → swap` has no Rust counterpart and the matcher's forced assignment
  swaps `partition`/`quick_sort` (scores 0.497 / 0.291); deployment keeps
  `partition → quick_sort` at confidence 0.047 > eps. Whether that is a matcher error is
  decided by the label, but the scaffold makes it the first row to read.

## Labeling order

1. qsort, both tools (1 + 2 rows), quadtree (3 rows) — establishes the conventions on
   artifacts where the matcher, name equality and the tool map mostly agree.
2. bzip2 (35 rows) — mostly matcher-vs-map conflicts; the map itself is clean (no record
   shift), so the tool claim is a strong prior there.
3. lodepng — only after 1–2, and only the rows needed for the map re-audit; the map is
   out of register over long stretches, so the tool claim is *not* a usable prior.
4. genann (SACTOR) and cJSON (dev) — names are preserved almost everywhere; the labels
   mainly confirm same-name pairs and settle the 7 cJSON functions with no same-name Rust fn.

`truth` vocabulary (from each artifact README): a Rust function name; `NONE` (no Rust
counterpart); `SPLIT:a;b`; `MERGED:x`; `AMBIGUOUS`. A claim that names no defined Rust
function is never a truth. `Compile_Failed` functions still count. The map audit is
four-way: correct / confirmed wrong (sub-class: record-shifted) / ambiguous /
missing-abstained, with ELIDED reported separately.
