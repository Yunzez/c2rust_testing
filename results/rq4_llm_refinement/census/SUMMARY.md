# Seeds-only census over the archived RQ4 cells — plan-guided seeds under one frozen global policy

*Step 4 of `docs/seeding_policy_plan.md`. Numbers: [`CENSUS.md`](CENSUS.md) / `census.json`; per cell:
`cells/<lib>_<tool>/` (policy manifest with the plan-guided / fallback split and every seed's layout, the rebuild
funnel, both t=0 results). Scripts: `scripts/rq4/seed_experiment/census_cell.sh`, `census_table.py`; the Seed IR and
policy: `tools/stu_selector/seed_ir.py`, `scripts/rq4/seed_policy.py` (branch `seed-ir`, generator 0.9.1).*

## What was measured

For every archived cell (33 non-tulip cells and tulip × c2rust as the cross-check; the other three tulip cells were
not run — their grid-rule ablation is already complete): rebuild the harnesses with the current generator, lower
each rebuilt boundary's InputPlan to the partial Seed IR, emit plan-guided seeds where a policy field is placeable
(bounded / unbounded scalars, small numeric arrays before the first opaque node) and the 64-byte default seed
elsewhere, and measure region coverage of the seeds alone — no fuzzing — for (a) the default seed and (b) default +
plan-guided seeds, against the cell's archived universe. The archived 3 600 s campaign sits beside them.
Two quantities matter: **seed effect** = (b) − (a), and the pre-registered **rerun criterion**: (b) ≥ archived
campaign + 5 percentage points. The difference (b) − campaign is a criterion, not a coverage regression.

## Result

Only **tulip** qualifies. tulip × c2rust: default seed 0.342, plan-guided seeds alone **0.909**, archived campaign
0.344 (+56.7 pp seed effect, +56.5 pp over the campaign) — the global policy reproduces the tulip grid ablation
(0.904 seeds-only under the hand grid) with no tulip-specific values: `size` gets its bounded boundary values and the
float set {0, 0.5, 1, 2, 5, −1} lands on the legal periods. Every other cell has a seed effect between 0 and
+2.5 pp, and in 20 of 29 the plan-guided seeds add nothing at all. Three situations, each explained by the plan:

1. **Fuzzing saturated, legal-domain seeds decisive — tulip only.** Ordinary fuzzing cannot get past a
   `(int)options[k]` guard by mutation; a seed that lands in the domain opens the whole body.
2. **Fuzzing reaches the code by itself; seeds are not the lever — bzip2, lil.** bzip2's campaign reaches 0.807 from
   the shipped `sample{1,2,3}` inputs of PROTOCOL §3 plus an hour of fuzzing; bounded scalars add +2.5 pp. lil reaches
   0.82–0.87 by fuzzing its script text; only 5 of 51 boundaries even have a placeable scalar.
3. **Scalars are not the bottleneck — lodepng, optipng, quadtree, genann, cJSON, urlparser, qsort.** Either the
   placeable fields do not gate anything (lodepng +0.6, optipng ≤ +1.1, qsort already at 0.98 from the default seed),
   or nothing is placeable because the API takes its object or string first (genann 5/10, quadtree 3/17, cJSON 3–4,
   urlparser 1 per cell: seed effect exactly 0, and on urlparser and quadtree × PtrTrans the campaign itself never
   left the default seed's reach).

**Why lodepng and optipng do not respond to samples either.** Applying PROTOCOL §3 (shipped sample inputs) to them
was considered and rejected on the plans: every PNG-consuming entry point failed harness construction — lodepng's
`lodepng_decode*`, `inspect`, `load_file`, `zlib_decompress`, `inflate` on the `out: T**` output pointer or a
struct-invariant `LodePNGState`; optipng's `png_read_*`, `opng_read_*`, `png_get_IHDR` on the opaque
`png_struct_def` or `FILE*`. The planned boundaries that take a buffer are byte-level helpers (adler32, crc32,
filterScanline, compress/uncompress). bzip2 differs precisely because `BZ2_bzBuffToBuffDecompress` is a planned
format consumer with a plain buffer. Their low coverage is a construction limit (`T**` outputs, opaque state
structs), the fourth cause category, and needs generator capabilities, not seeds.

## Decision

Per the pre-registered rule, no application other than tulip is rerun; tulip's seeded runs already exist as the
grid-rule ablation (`../tulip/`). Deterministic plan-guided seeding is a **complement for derivable narrow scalar
domains**, not a general coverage technique, and stays a component analysis beside the automatic baseline — the
37 archived cells are unchanged.

## Caveats and things found on the way

- The census rebuilt harnesses with generator 0.9.1 (0.9's NaN-equivalent oracle + a test-only flag; emitted
  campaign code identical to 0.9 by the golden regression); the archived campaigns used 0.8. Build sets match the
  archives (`built census / archived` column) except bzip2 × Laertes/CROWN/C2SaferRust, where one boundary
  (`BZ2_hbCreateDecodeTables`) no longer builds: a planner drift (`length` now lowered as a capacity pointer while
  `alphaSize` still derives from it), recorded in `docs/rq4_runbook.md`, counted as unbuilt here.
- Three generator-side regressions surfaced and were fixed on `seed-ir`: bzip2's `static __inline__` definitions had
  stopped linking (`fixups` ran `strip_static_c` before the inline-aware pattern); cJSON × c2rust rebuilt only 7/39
  harnesses because the 2026-09-06 "re-export the entry module's types" fixup emitted `pub use crate::cJSON::cJSON;`
  for a single-file translation whose struct already sits at the crate root (E0432; now guarded on a declared
  module, 39/39 rebuilt, seed effect +0.0 pp); and the Seed IR initially took
  the plan's input order for the decode order (plans.json lists inputs in analysis order; the harness decodes in C
  declaration order) — caught by the decode-dump round-trip test on `quadtree_insert`.
- Seeds are lowered from the plans the rebuilt harnesses were generated from, never from the archived plans.
- Cells were run in up to three parallel lanes (build-bound; no fuzzing); one lane's cells were corrupted by
  editing the driver script while it ran (bash reads scripts by offset) and were rerun from scratch.
