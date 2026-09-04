# RQ1 group B — canonical status (2026-09-02)

> **Presentation superseded 2026-09-02 (evening):** the paper no longer shows group A and group B as
> separate tables/aggregates. The binding presentation is one merged ten-library table
> (40 complete outputs, 4,202 pairs, 9 renamed, macro P/R 0.829/0.874, renamed correct 7/9) —
> see `rq1_reporting_decision_2026-09-02.md` and `scripts/rq1_merged_table.py`. The group-B-only
> primary table in §2 (7 libraries, 9 outputs, 0.516/0.739) is therefore **provenance, not a paper
> number**; rule 3's `Name-equality recall` column becomes prose (0/9 vs 7/9) and rule 5's
> group-A-only aggregate is no longer reported on its own. Everything else in this document
> (rules, labels, PARTIAL lines, per-cell runs, §5 review items) still holds.

**This is the one status document for group B of `tab:matching-accuracy`.** It supersedes
`group_b_availability.md` (2026-09-01 inventory) and the group-B / dev-vs-eval / deployment
parts of `rq1_assembled_v1.md`. Every number here is produced by
`scripts/rq1_group_b_score.py` from `rows/group_b_full.json`, which is built from
`annotation/<case>/sheet.json` + `labels.json`; every raw translator output and failure log is
under `raw/group_b_runs/<case>/` (2026-09-02 paid runs, each with a `RUN.md`) or the locations
named in `scripts/rq1_group_b_scaffold.py` (shipped / July artifacts).

## Reporting rules (binding, 2026-09-02; replace the 2026-09-01 rules)

1. **All ten libraries appear in the primary table.** cJSON and lil are ordinary libraries and
   enter the Overall; their earlier use as matcher-development data is recorded as provenance
   only. No dev-only / eval-only / held-out / micro-average aggregate is reported.
2. **Unit = library.** Within a library, average over its available tool outputs; across
   libraries, equal weight. Pairs are evidence volume only. Every aggregate states how many of
   the 10 libraries and how many tool outputs contribute.
3. **Group B columns:** Library | Tool symbols | Pairs (genuine renamed pairs) | Name-equality
   recall | Matcher precision / recall | Renamed correct.
4. **Renamed correct** = correctly matched genuine renamed pairs / all genuine renamed pairs,
   **forced** configuration. Deployment / abstention / P@C / coverage never appear in the
   primary table (they are kept in the scorer's secondary block only).
5. **Group A is unchanged** (10 libraries, 31 artifacts, macro P/R 0.938 / 0.939).
6. **Non-building Rust is eligible.** A cell is N/A only when no analyzable Rust was emitted
   or retained. **PARTIAL** cells (the translator stopped or crashed before covering the
   library, or emitted Rust for only part of it) are labeled and scored on their own lines and
   are **never mixed into the primary aggregate**.
7. **Truth distinctions are kept:** genuine single-function pair | `STUB:x` | `NONE` |
   `SPLIT` / `MERGED` | `AMBIGUOUS`. STUBs are not genuine pairs (strict rule; the lenient
   variant is secondary). Tool-map record shifts and placeholder names are not renames.
8. **All labels are preliminary** (Claude-labeled, `reviewed_by_user: false` in every
   `labels.json`); the scorer warns until a human flips the flag. Every genuine renamed pair,
   every matcher error and every STUB-vs-genuine decision has an evidence note in its
   `labels.json`.

Tool symbols: ∘ c2rust, △ Laertes, ◇ CROWN, • C2SaferRust, ★ PtrTrans, × SACTOR.

## 1. Availability matrix (10 libraries × 2 renaming translators)

Classes: **AVAILABLE** (analyzable Rust for the library, labeled + scored, in the primary
table) · **PARTIAL** (analyzable Rust for part of the library; labeled + scored on a separate
line) · **PRE-OUTPUT FAILURE** (the shipped configuration failed before emitting any Rust;
verified from logs) · **PRODUCED BUT LOST** (Rust existed in July 2026 but was not archived
and has not been reproduced) · **NOT RUN**.

| library | PtrTrans ★ (Trans_PA, gpt-5.1) | SACTOR × (gpt-5.1) |
|---|---|---|
| qsort | **AVAILABLE** — shipped artifact (`PA_trans_projects/qsort`), builds, map | **AVAILABLE** — July run, builds, map |
| urlparser | **AVAILABLE** — 2026-09-02 run, builds, 18 fn / 2 STUB / 1 NONE (`raw/group_b_runs/ptrtrans_urlparser/RUN.md`) | **PARTIAL** — 2026-09-02 run, 7/22 fns (idiomatic phase), non-building; `URL_SCHEMES` global failed 5× and blocked 13 dependents (`raw/group_b_runs/sactor_urlparser/RUN.md`) |
| quadtree | **AVAILABLE** — shipped artifact, builds, map | **PARTIAL** — 2026-09-02 rerun (July output lost), 12/24 fns (leaf TUs, unidiomatic phase, SACTOR-verified), `quadtree.c` refused: circular dependencies (`raw/group_b_runs/sactor_quadtree/RUN.md`) |
| genann | **AVAILABLE** — 2026-09-02 run (July output lost), builds, 11 fn / 3 STUB / 1 NONE (`raw/group_b_runs/ptrtrans_genann/RUN.md`) | **AVAILABLE** — July run, builds, no map |
| cJSON | **AVAILABLE** — July run (our own), builds, no map retained | **PRE-OUTPUT FAILURE** — circular-dependency refusal at dependency analysis, 0 LLM calls (`results/rq4_effectiveness/certificates/cjson_matrix.md`, `bugs/lil_sactor/README.md`) |
| lil | **PARTIAL** — 2026-09-02 run (July output lost); translator crashed at unit 95/131 (807k-token prompt > 272k limit); partial crate builds, 76 fn / 14 STUB / 38 NONE (`raw/group_b_runs/ptrtrans_lil/RUN.md`) | **PRE-OUTPUT FAILURE** — circular-dependency refusal, 0 LLM calls (`bugs/lil_sactor/circular_error.txt`, `batch_summary.json`) |
| lodepng | **AVAILABLE** — shipped artifact, does not build, 8 fn / 214 STUB | **PARTIAL** — 2026-09-02 run (July output lost), 74/235 fns (unidiomatic phase; 53 SACTOR-verified, 20 failed 6/6 on SACTOR's duplicate-`LodePNGColorType` scaffold, 1 link failure on a `static` C helper aborted the TU; 161 never attempted), non-building (`raw/group_b_runs/sactor_lodepng/RUN.md`) |
| bzip2 | **AVAILABLE** — shipped artifact, does not build, 9 fn / 52 STUB | **PARTIAL** — 2026-09-02 Gate 0/1 run (supersedes the July "C-parser failure": the `BZALLOC`/`BZFREE` call spelling is a neutral input-side rewrite, after which the parser passes), 32/64 fns (unidiomatic phase; 27 SACTOR-verified, 3 failed 6/6 on SACTOR's `Debug`/`size_t` scaffold conflicts, 2 link failures on `static` C helpers aborted `bzlib.c`/`compress.c`; 32 never attempted), non-building (`raw/group_b_runs/sactor_bzip2/RUN.md`) |
| tulip | **PRE-OUTPUT FAILURE** — PA pre-pass: `pa_struct` stoul crash on `ti_indicator_info`, `pa_func` > 1 h; no LLM stage (`bugs/tulip_ptrtrans/README.md`) | **PARTIAL** — 2026-09-02 run (July output lost), 69/269 fns (one per TU, unidiomatic, log-recovered), none verified (harness link failure on the `ti_indicators[]` table), non-building (`raw/group_b_runs/sactor_tulip/RUN.md`) |
| optipng | **PRE-OUTPUT FAILURE** — PA pre-pass: `pa_struct` > 2 h, no LLM stage (`bugs/optipng_ptrtrans/README.md`) | **PRE-OUTPUT FAILURE** — 2026-09-02 Gate 0 ($0, 3 dry runs; supersedes the July "`ZALLOC` deflate.c:277" note): after 17 indirect-call spellings in 12 files were neutralised, the final wall is `__builtin_va_start` in 4 variadic functions (`gzprintf`, `opng_snprintf_impl`, `error`, `app_printf`), which SACTOR's resolver treats as undefined project functions; no neutral rewrite exists, 0 LLM calls (`raw/group_b_runs/sactor_optipng/RUN.md`) |

NOT RUN: none. PRODUCED BUT LOST: none remaining (all five July-lost cells were rerun).
All pre-output failures are "failed to emit analyzable Rust under the shipped configuration";
nothing was patched, no cost breaker raised, no LLM-only/ablation configuration used. The
2026-09-02 PtrTrans runs carry three input-side adaptations documented in their `RUN.md`
(genann: four `unused` attribute uses removed because PtrTrans's macro step breaks on them;
lil: compile_commands reshaped; urlparser: none); the SACTOR runs carry the July harness
drivers and, for quadtree, the three parser-level source spellings documented in
`bugs/quadtree_sactor/README.md`. The 2026-09-02 Gate 0/1 attempts on bzip2 and optipng
(user-accepted protocol: $0 dummy-key dry runs, neutral input-side spellings only, then at most
one ≈$1–2 paid run) add the rewrites listed in their `RUN.md` (bzip2: `BZALLOC`/`BZFREE`
spelling, `_POSIX_C_SOURCE`, the two table TUs relocated verbatim into `bzlib.c`; optipng:
13 `(*(x->f))(…)` → `x->f(…)` sites, `ZALLOC`/`ZFREE`, a `(isspace)(…)` macro, one
array-subscript callee, `HAVE_UNISTD_H`, `_XOPEN_SOURCE`). None changes program semantics.

## 2. Primary table — group B, per library (strict rule; forced configuration)

`rows/group_b_full.json` → `paper_rule.per_library`. Within a library: mean over its
AVAILABLE tool outputs; Overall: equal weight over libraries. PARTIAL outputs are **not**
included (§3).

| library | tools | pairs (genuine renamed) | name-eq R | matcher P / R | renamed correct |
|---|---|---:|---:|---|---:|
| qsort | ★ × | 6 (2) | 0.667 | 0.667 / 0.667 | 1/2 |
| urlparser | ★ | 18 (0) | 1.000 | 0.800 / 0.889 | — |
| quadtree | ★ | 17 (0) | 1.000 | 0.882 / 0.882 | — |
| genann | ★ × | 26 (0) | 1.000 | 0.714 / 0.773 | — |
| cJSON | ★ | 77 (1) | 0.987 | 0.398 / 0.558 | 1/1 |
| lil | — (★ PARTIAL only) | N/A | N/A | N/A | N/A |
| lodepng | ★ | 8 (4) | 0.500 | 0.021 / 0.625 | 3/4 |
| bzip2 | ★ | 9 (2) | 0.778 | 0.130 / 0.778 | 2/2 |
| tulip | — (× PARTIAL only) | N/A | N/A | N/A | N/A |
| optipng | — | N/A | N/A | N/A | N/A |
| **Overall (7 of 10 libraries, 9 tool outputs)** | | **161 (9)** | **0.847** | **0.516 / 0.739** | **7/9** |

Per tool output (strict): qsort★ 3(1) 1.000/1.000 1/1 · qsort× 3(1) 0.333/0.333 0/1 ·
urlparser★ 18 0.800/0.889 · quadtree★ 17 0.882/0.882 · genann★ 11 0.429/0.545 ·
genann× 15 1.000/1.000 · cJSON★ 77(1) 0.398/0.558 1/1 · lodepng★ 8(4) 0.021/0.625 3/4 ·
bzip2★ 9(2) 0.130/0.778 2/2. Lenient (STUB:x = x) lines and deployment/abstention columns
stay in the scorer's secondary block and are not primary evidence.

## 3. PARTIAL outputs — scored separately, never in the primary aggregate

| cell | coverage | builds | pairs | name-eq R | matcher P / R | renamed correct | why partial |
|---|---|---|---:|---:|---|---:|---|
| lil ★ | 76 fn / 14 STUB / 38 NONE of 128 | yes | 76 (0) | 1.000 | 0.656 / 0.776 | — | crash at unit 95/131 (807k-token prompt); 12 stubs = `fnc_store` + the 11-function parse/eval SCC after 5 repair rounds; 2 more constant-`None` bodies labeled STUB by body reading |
| urlparser × | 7 fn / 15 NONE of 22 | no | 7 (0) | 1.000 | 0.143 / 0.143 | — | `URL_SCHEMES` static of C strings rejected by rustc (E0277) 5×; 13 dependents never attempted |
| quadtree × | 12 fn / 12 NONE of 24 | no | 12 (0) | 1.000 | 1.000 / 1.000 | — | `quadtree.c` refused (circular deps `insert_` ↔ `split_node_`); leaf TUs verified |
| tulip × | 69 fn / 200 NONE of 269 | no | 69 (0) | 1.000 | 0.014 / 0.014 | — | one function per TU before the harness link failure; 37 TUs never reached the LLM (same-directory `simple1.h` not on SACTOR's include path); zero call edges in the artifact, so the matcher has no non-name signal |
| bzip2 × | 32 fn / 32 NONE of 64 | no | 32 (0) | 1.000 | 0.875 / 0.875 | — | SACTOR aborted `bzlib.c` at function 20/41 (`flush_RL`) and `compress.c` at 5/9 (`bsPutUInt32`): both cdylibs reference `static` C helpers (`add_pair_to_block`/`init_RL`, `bsW`) → link failure (reproduced, `relink_evidence/`); 3 of the 32 failed 6/6 on SACTOR's `Debug`/`size_t` scaffold conflicts but carry full bodies (genuine, unverified); 32 never attempted (blocksort/decompress stopped by batch-order dependency checks); no idiomatic phase, no tool map. Matcher errors = 4 crossed pairs (each one miss + one false proposal: `unRLE_obuf_to_output_FAST`←`_SMALL`, `BZ2_bzDecompressEnd`←`_Init`, `myfeof`←`BZ2_bzwrite`, `makeMaps_e`←`generateMTFValues`) |
| lodepng × | 74 fn / 161 NONE of 235 | no | 74 (0) | 1.000 | 0.811 / 0.811 | — | SACTOR aborted the TU at function 74 (`lodepng_gtofl` cdylib references the `static` C helper `lodepng_addofl` → link failure); 20 of the 74 failed 6/6 on SACTOR's own duplicate `LodePNGColorType` scaffold but carry full bodies (genuine, unverified); 161 never attempted; no idiomatic phase, no tool map |

All 2026-09-02 outputs, PtrTrans and SACTOR alike, **kept every C function name**; the only
genuine renamed pairs in group B are the 9 from the July/shipped artifacts (qsort ★×, cJSON ★,
lodepng ★, bzip2 ★). Six PARTIAL cells in total (lil ★; urlparser, quadtree, tulip, lodepng,
bzip2 ×), 270 same-name pairs of PARTIAL evidence, none in the primary aggregate.

## 4. Provenance and cost of the 2026-09-02 runs

Key: `key.env` (OpenAI), budget ≈ $100. PtrTrans-C2Rust `b20d5bb` + 3 path patches
(`raw/group_b_runs/_tool_patches/`), `--translate_mode Trans_PA --model_name gpt-5.1`.
SACTOR `577c3d2` + 3 patches, gpt-5.1 via LiteLLM, `sactor translate --type bin …
--continue-run-when-incomplete`. Costs are estimates from logged prompt volume unless a
`llm_stat*.json` exists (tools do not report billing).

| cell | class | archive | est. cost |
|---|---|---|---:|
| urlparser ★ | AVAILABLE | `raw/group_b_runs/ptrtrans_urlparser/` | ≈ $4 |
| urlparser × | PARTIAL | `raw/group_b_runs/sactor_urlparser/` (4 runs, 2 aborted on our harness) | ≈ $4 |
| genann ★ | AVAILABLE | `raw/group_b_runs/ptrtrans_genann/` | ≈ $2 |
| lil ★ | PARTIAL | `raw/group_b_runs/ptrtrans_lil/` | ≈ $3–6 |
| tulip × | PARTIAL | `raw/group_b_runs/sactor_tulip/` (run 1 + run 2, both full passes) | ≈ $10 |
| quadtree × | PARTIAL | `raw/group_b_runs/sactor_quadtree/` | ≈ $1 |
| lodepng × | PARTIAL | `raw/group_b_runs/sactor_lodepng/` (40 min, ended by SACTOR's own link error; external 90-min cap never fired) | ≈ $5 |
| bzip2 × | PARTIAL | `raw/group_b_runs/sactor_bzip2/` (Gate 0: 2 dry runs $0; Gate 1: run 1 killed at the first link failures ≈5 prompts, run 1b crashed in SACTOR's AST pass ≈10 prompts, run 2 full pass 13 min / 61 prompts, ended by SACTOR's own link errors; external 25-min cap never fired) | ≈ $2–2.5 |
| optipng × | PRE-OUTPUT FAILURE | `raw/group_b_runs/sactor_optipng/` (Gate 0 only: 3 dry runs, 0 LLM calls) | $0 |
| **total** | | | **≈ $32–38** of the ≈ $100 budget (all estimates; tools do not report billing) |

Reused without reproduction (logs sufficient): tulip ★, optipng ★ (PA stage, $0), cJSON ×,
lil × (0 LLM calls). bzip2 × and optipng × were re-attempted on 2026-09-02 under the Gate 0/1
protocol (rows above); their July notes are superseded.

## 5. Labels requiring independent review (all 15 cases are unreviewed)

Highest value first:
1. The **9 genuine renamed pairs** (qsort ★ `partition`-family, qsort ×, cJSON ★, lodepng ★ ×4,
   bzip2 ★ ×2) — they decide "renamed correct".
2. STUB-vs-genuine calls made by body reading rather than tool tag: ptrtrans_urlparser
   `url_get_path` (LLM-authored constant-None body); ptrtrans_lil `lil_to_string`,
   `lil_list_get` (STUB despite `No_Fix_Compile_Success`) and `find_cmd`, `lil_find_local_var`
   (kept genuine though every path returns `None`); ptrtrans_genann `genann_run` (kept genuine
   though it panics unconditionally).
3. sactor_tulip: the labeler chose **provenance** over the README's literal "behaviorally
   interchangeable ⇒ AMBIGUOUS" rule (the `ti_X_start` bodies fall into ~6 clone classes);
   under the literal rule the cell collapses to AMBIGUOUS and becomes unscorable.
4. lodepng ★ 6 AMBIGUOUS + 1 SPLIT rows; a sample of the 214 lodepng / 52 bzip2 / 31 cJSON STUB
   rows.
5. sactor_urlparser `url_free` (idiomatic `{}` body labeled genuine as an ownership drop).
6. sactor_lodepng: the 20 fail-6/6 bodies and `lodepng_gtofl` labeled genuine by body reading
   although SACTOR never verified them (tool-side E0428 / link failure); the 6 forced-matcher
   errors are two same-body swaps (`lodepng_chunk_ancillary`↔`_safetocopy`, `isGrayICCProfile`↔
   `isRGBICCProfile`) plus 4 mis-assignments, and 8 proposals landed on NONE rows.
7. sactor_bzip2: `BZ2_bzflush` labeled genuine although its Rust body is a bare `0` (the C
   body is `return 0;` with a "do nothing now" comment); the 5 SACTOR-unverified last attempts
   (`BZ2_bzCompressInit`, `BZ2_bzReadGetUnused`, `BZ2_bzerror`, `flush_RL`, `bsPutUInt32`)
   labeled genuine by body reading; benign added null guards in 6 bodies kept genuine.

## 6. Cells still impossible under the shipped configurations

PtrTrans: tulip, optipng (PA pre-pass never terminates / crashes before any LLM call).
SACTOR: cJSON, lil (circular-dependency refusal), optipng (resolver rejects the compiler
builtin `__builtin_va_start` behind 4 variadic functions; Rust cannot define them, so no
input-side rewrite exists). Also structurally capped: SACTOR quadtree (core TU refused),
SACTOR tulip (per-function harness cannot link against the fn-pointer table; 37 TUs fail
SACTOR's own preprocessing), SACTOR urlparser (C-string static), SACTOR bzip2 and lodepng
(per-function link closure cannot see `static` C helpers → TU abort; scaffold conflicts),
PtrTrans lil (prompt growth past the model's context limit). Filling any of these would
require patching the translator, un-`static`-ing library helpers, a cost/prompt-size guard,
or an ablation/LLM-only configuration — all excluded by the reporting rules.

Published-artifact survey (2026-09-02, item 3 of the user's plan): no published PtrTrans or
SACTOR translation of a real library contains correspondences that are both genuinely renamed
and non-stub. Upstream PtrTrans ships only quadtree (identity names); SACTOR ships 8 toy
examples whose only renames are camelCase→snake_case (2–3 functions each; our qsort ×
`quickSort→quick_sort` already covers that class); CROWN (13 further projects), Laertes and
C2SaferRust (coreutils + 3) outputs are c2rust-derived and name-preserving; CRUST-Bench's
Rust side is `unimplemented!()` by design. Nothing there would add renamed evidence, so no
further budget was spent on it.
