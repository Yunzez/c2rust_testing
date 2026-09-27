# RQ1 group B — which renaming-translator outputs exist to analyze (2026-09-01)

> **SUPERSEDED (2026-09-02)** by [`group_b_status.md`](group_b_status.md), the canonical
> group-B status document. The inventory below is the 2026-09-01 snapshot (7 artifacts,
> dev/eval split). Five further cells were run on 2026-09-02 (PtrTrans urlparser/genann/lil,
> SACTOR urlparser/quadtree/tulip/lodepng); the N/A sub-classes and the dev/eval language here
> are no longer used. Kept for provenance only.

Group B of `tab:matching-accuracy` covers the two translators that do not preserve function
names: **PtrTrans** (FSE'26, gpt-5.1, Trans_PA) and **SACTOR** (gpt-5.1). Static matching does
not require the crate to compile, so the rule for the table is:

> a cell is scaffolded (and later labeled) if any parseable Rust output of that translator on
> that library exists on disk; it becomes **N/A** only after this file documents that no
> analyzable output exists, and says *why* (never produced vs. produced but not retained).

Every claim below points at the evidence that established it (translation-matrix note number
in `results/rq4_effectiveness/translation_matrix.md`, or a bug/evidence directory under
`results/rq4_effectiveness/bugs/`). Scaffolds are produced by `scripts/rq1_group_b_scaffold.py`
into `annotation/<tool>_<lib>/`.

## Summary

| library | split | PtrTrans | SACTOR |
|---|---|---|---|
| qsort | eval | **scaffolded** `annotation/ptrtrans_qsort/` (shipped, builds, map) | **scaffolded** `annotation/sactor_qsort/` (builds, map) |
| urlparser | eval | N/A — never run: C reference heap-overflows on every URL (UB gate, note 27) | N/A — never run, same gate (note 31) |
| quadtree | eval | **scaffolded** `annotation/ptrtrans_quadtree/` (shipped, builds, map) | N/A — refused pre-LLM (circular deps, note 33); 15 leaf-TU functions were translated but **not retained** |
| genann | eval | N/A — our run built a crate whose core is `unimplemented!()` stubs (note 24); artifact was scratch, **not retained** | **scaffolded** `annotation/sactor_genann/` (assembled, no map) |
| cJSON | dev | **scaffolded** `annotation/ptrtrans_cjson/` (our run, builds, no map retained) | N/A — refused pre-LLM (circular deps; matrix cell, `bugs/lil_sactor/README.md` names it as the first instance) |
| lil | dev | N/A — our run produced a non-compiling crate (116 syntax-level errors, note 25); artifact was scratch, **not retained** | N/A — refused pre-LLM, 0 LLM calls (note 34, `bugs/lil_sactor/circular_error.txt`) |
| lodepng | eval | **scaffolded** `annotation/ptrtrans_lodepng/` (shipped, does NOT build) | N/A — run 2 translated ~50 functions before the cost breaker (note 35); **not retained** |
| bzip2 | eval | **scaffolded** `annotation/ptrtrans_bzip2/` (shipped, does NOT build) | N/A — parse-fail before any LLM call (`bugs/bzip2_sactor/parser_errors.txt`) |
| tulip | eval | N/A — required PA pre-pass crashes / does not terminate, no LLM stage reached (`bugs/tulip_ptrtrans/`, note 39) | N/A — all 104 indicators were translated, all failed link-verification (note 37); the translated Rust was scratch, **not retained** |
| optipng | eval | N/A — PA pre-pass exceeds 2 h, no LLM stage reached (`bugs/optipng_ptrtrans/`) | N/A — parse-fail before any LLM call (`bugs/optipng_sactor/parser_errors.txt`) |

Scaffolded **and labeled**: **7 artifacts** — PtrTrans qsort, quadtree, bzip2, lodepng, cJSON;
SACTOR qsort, genann. Of these, 5 are held-out (`SPLIT.md`), cJSON is dev; 2 (bzip2, lodepng)
do not build. **Libraries with ≥1 group-B artifact: 6 of 10** (qsort, quadtree, genann,
cJSON, lodepng, bzip2); eval-split only: 5 of 8. urlparser, lil, tulip, optipng have none.

## N/A sub-classes (the distinction the table must keep)

1. **Never produced — translator failed before emitting Rust.** PtrTrans: urlparser (not run,
   UB gate), tulip, optipng (PA stage). SACTOR: urlparser (not run), cJSON, lil (circular-deps
   refusal at dependency analysis), bzip2, optipng (C parser). Nothing to analyze exists or ever
   existed; these are translator process failures, already counted in RQ4's matrix.
2. **Produced but not retained.** PtrTrans genann and lil (July-2026 runs kept in scratch dirs
   `ptrtrans_genann/`, `PA_trans_projects/lil/` that were cleaned); SACTOR quadtree (15 leaf
   functions), lodepng (~50 functions), tulip (104 indicators) — SACTOR writes per-function
   outputs into its result dir, which lived in the session scratchpad for those runs and was
   not copied into `results/`. These cells are analyzable in principle and are N/A only because
   of our archiving, not because of the tool. Re-running is a paid LLM run per cell
   (SACTOR tulip ≈ one full pass; PtrTrans genann/lil ≈ $10–20 each) and is the only way to
   fill them; this is recorded here rather than hidden behind "N/A".

Non-compiling is *not* a reason for N/A: PtrTrans bzip2 and lodepng are scaffolded and will
be reported on their own line (matching under degraded static analysis,
`topology_resolution.md`).

## What "pairs" means in group B

Group A pairs are name-equality pairs (C function ↔ identically-named Rust function). Group B
pairs are **manually labeled C→Rust correspondences**: one per C function with a non-`NONE`
truth. Within them the table must separate

- **real renamed pairs** — truth ≠ C name (e.g. `quickSort → quick_sort`), the rows that test
  name independence, and
- **same-name pairs** — truth = C name, which name equality also gets right.

Scaffold counts of *candidate* renames (tool claim ≠ C name, or no same-name Rust function),
to be confirmed by labeling: PtrTrans qsort 1/3, quadtree 0/24, bzip2 19/64, lodepng
126/235 claimed (63 of those are record-shift artifacts, see `mapping_audit`), cJSON 7/113
(no same-name function); SACTOR qsort 1/3, genann 0/15.

## Status (2026-09-01, evening)

**All 7 scaffolded artifacts are fully labeled** (`annotation/<case>/labels.json`, applied to
`sheet.csv`/`sheet.json` by `scripts/rq1_group_b_label.py`; scored by
`scripts/rq1_group_b_score.py` → `rows/group_b_full.json`). Labels were produced by Claude
(main session + subagents), every row carrying an evidence note (C line, Rust file:line,
rejected candidates); **`reviewed_by_user: false` on all seven** — the scorer prints a
warning until the user reviews them. Numbers are therefore *preliminary* until reviewed.

| case | rows | truth kinds | pairs (renamed) |
|---|---:|---|---:|
| ptrtrans_qsort | 3 | 3 fn | 3 (1) |
| sactor_qsort | 3 | 3 fn | 3 (1) |
| ptrtrans_quadtree | 24 | 17 fn, 7 NONE (`*_free`/`*_reset` elided; crate has no `impl Drop`) | 17 (0) |
| sactor_genann | 15 | 15 fn | 15 (0) |
| ptrtrans_cjson (dev) | 113 | 77 fn, 31 STUB, 5 NONE | 77 (1) |
| ptrtrans_bzip2 | 64 | 9 fn, 52 STUB, 3 NONE | 9 (2) |
| ptrtrans_lodepng | 235 | 8 fn, 214 STUB, 6 NONE, 6 AMBIGUOUS, 1 SPLIT | 8 (4) |

**Label vocabulary** (added `STUB:x` this round): Rust fn name | `NONE` | `SPLIT:a;b` |
`MERGED:x` | `STUB:x` | `AMBIGUOUS`. `STUB:x` = the translator emitted a signature-only
placeholder `x` for this C function (same name / doc cites the C signature; body is empty,
`0`, `None`, or `unimplemented!()`). A stub is not a translation, so it is **not a scorable
pair under the strict rule**; the scorer also reports a *lenient* variant (`STUB:x` counted
as truth `x`) on a separate line so the two readings can be compared. `AMBIGUOUS` in lodepng
= two duplicate empty stubs (CamelCase + snake_case) for the same C function.

**Fact established by labeling: the shipped PtrTrans bzip2 and lodepng crates are stub
shells, and a third of cJSON is.** bzip2: 52/64 C functions have only a placeholder (git log
= one "Initial translation" commit per function, bodies never filled); lodepng: 214/235, with
~8 functions carrying real logic, dozens of duplicate same-name definitions (the crate cannot
compile for that reason alone), and fragments under generic names (`translated_segment`,
`unnamed_loop_function`); cJSON (our run): 31/113 — the parse/print core. This is a property
of the translator output, not of the matcher, and the table must say so: the bzip2/lodepng
rows measure matching on 9 and 8 real pairs surrounded by 52 and 214 decoys.

**Analyzer limitation noted:** `lodepng_chunk_init` exists at `src/lodepng.rs:2014` but is
absent from the rust-analyzer inventory (parameter named `_r#type`), so it cannot be a matcher
target; labeled NONE with the note.
