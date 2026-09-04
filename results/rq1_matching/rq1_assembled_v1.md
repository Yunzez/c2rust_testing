# RQ1 — one question, one main table

> **PARTLY SUPERSEDED (2026-09-02).** The group-B section, the *dev/eval split* rule (rule 6),
> the *micro-average* and *eval-only* aggregates, and the *deployment P@C* columns are
> **superseded by [`group_b_status.md`](group_b_status.md)** and its binding reporting rules
> (all ten libraries in the primary table, library = unit, no dev/eval or micro-average
> aggregate, no deployment/abstention columns in the primary table). Group A numbers (10
> libraries, 31 artifacts, paper rule 0.938 / 0.939) are unchanged and remain authoritative here.

*Draft v5, 2026-09-01. Group A measured for all ten libraries (31 artifacts) on the frozen
split; group B inventoried across PtrTrans **and** SACTOR, scaffolded (7 artifacts) and now
**fully labeled and scored — preliminary, labels not yet user-reviewed**. Every number is
marked **measured**, **preliminary**, or **not yet run**; nothing is carried over from prose.
Group A aggregates trace to `rows/group_a_table.json` → `rows/group_a_full.json` →
`raw/group_a/<artifact>/`; group B values trace to `rows/group_b_full.json` →
`annotation/<case>/sheet.json` (+ `labels.json`) → `raw/group_b/<case>/`.*

> **RQ1. How accurately does our matcher recover true C–Rust function correspondences?**

## Reporting rules (binding)

1. **Every row reports precision, recall, abstention rate, and accepted coverage.** Recall
   alone hides mismatches, and in differential testing a confident wrong pair is more
   dangerous than an abstention: it manufactures false divergences at a boundary nobody
   flagged, whereas an abstention only leaves a boundary untested.
2. **Both micro and macro.** Micro pools all correspondences; macro weights each artifact
   equally. Every aggregate states **how many artifacts and how many libraries** it covers.
3. **No outcome-driven exclusion.** tulip's homogeneous-cluster collapse (below) and `lil`'s
   LLM ignoring the rename instruction are experimental results, not noise. A number computed
   after dropping an artifact is never the headline; it may appear, labeled, as explanation.
4. **Cell counts are artifact counts, not Cartesian products.** 10 libraries × 4
   name-preserving tools would be 40; we have 31 artifacts (§Group A). Say the second thing.
5. **Only real shipped-translator artifacts are primary evidence.** raw-LLM translation and
   mechanical identifier scrambling are *controlled stress tests*, reported separately.
6. **Development and evaluation data are library-disjoint and frozen** (`SPLIT.md`):
   cJSON, lil and the `benchmark/pairs` micro corpus are development data; qsort, genann,
   urlparser, quadtree, lodepng, bzip2, tulip, optipng are evaluation data.
7. **Forced mode and deployment mode are different configurations and are always reported
   as such.** Forced = `matcher.match(topo=True)` (must answer for every C function);
   deployment = the same with `abstain_eps=0.01`. "3/3 on qsort×PtrTrans" is a forced-mode
   statement; deployment accepts 2/3 and abstains on `partition`.
8. **Two kinds of N/A.** A group-B cell is N/A only after `group_b_availability.md` records
   that no analyzable Rust exists, and says whether it was *never produced* or *produced but
   not retained*. Non-compiling output is analyzable and is never N/A.

---

## Table 1 (main) — `tab:matching-accuracy`, per library, paper row order

Pairs are correspondence pairs, not artifacts (rule 4). **Aggregation rule (paper caption,
2026-09-01): within a library, mean over that library's available tool outputs; across
libraries, equal weight; pairs are evidence volume only.** Group A: truth = name equality;
P/R forced, P@C deployment; source `rows/group_a_table.json` → `paper_rule.per_library`
(`scripts/rq1_group_a_table.py`, which re-derives each integer from `raw/group_a/` and aborts
on disagreement). Group B: truth = manual labels (strict rule: STUB rows are not pairs);
source `rows/group_b_full.json` → `paper_rule.per_library` (`scripts/rq1_group_b_score.py`).
Tool symbols as in the paper: ∘ c2rust, △ Laertes, ◇ CROWN, • C2SaferRust, ★ PtrTrans,
× SACTOR.

| library | split | **A: tools** | **A: pairs** | **A: matcher P / R** | **A: deploy P@C** | **B: tools** | **B: pairs (renamed)** | **B: name-eq R** | **B: matcher P / R** | **B: deploy P@C** |
|---|---|---|---:|---|---|---|---:|---:|---|---|
| qsort | eval | ∘ △ • | 9 | 1.000 / 1.000 | 1.000@1.000 | ★ × | 6 (2) | 0.667 | 0.667 / 0.667 | 0.500@0.500 |
| urlparser | eval | ∘ △ ◇ • | 84 | 0.941 / 0.941 | 0.963@0.714 | — (never run: UB gate) | N/A | N/A | N/A | N/A |
| quadtree | eval | ∘ ◇ | 48 | 1.000 / 1.000 | 1.000@0.875 | ★ | 17 (0) | 1.000 | 0.882 / 0.882 | 1.000@0.647 |
| genann | eval | ∘ △ ◇ • | 48 | 1.000 / 1.000 | 1.000@0.979 | × | 15 (0) | 1.000 | 1.000 / 1.000 | 1.000@0.867 |
| cJSON | dev | ∘ | 58 | 1.000 / 1.000 | 1.000@0.862 | ★ (our run) | 77 (1) | 0.987 | 0.398 / 0.558 | 0.880@0.299 |
| lil | dev | ∘ △ ◇ • | 563 | 0.961 / 0.961 | 0.998@0.849 | — (★ not retained; × never produced) | N/A | N/A | N/A | N/A |
| lodepng | eval | ∘ ◇ | 470 | 0.992 / 0.992 | 1.000@0.745 | ★ (stub shell, non-building) | 8 (4) | 0.500 | 0.021 / 0.625 | 0.000@0.000 |
| bzip2 | eval | ∘ △ ◇ • | 256 | 0.996 / 0.996 | 1.000@0.922 | ★ (stub shell, non-building) | 9 (2) | 0.778 | 0.130 / 0.778 | 0.833@0.556 |
| tulip | eval | ∘ △ ◇ • | 852 | 0.554 / 0.554 | 0.968@0.151 | — (★ PA-stage failure; × not retained) | N/A | N/A | N/A | N/A |
| optipng | eval | ∘ △ • | 1653 | 0.941 / 0.943 | 0.999@0.742 | — (★ PA-stage failure; × parse-fail) | N/A | N/A | N/A | N/A |
| **Overall, all 10 libs (A)** | | 31 outputs | 4041 | **0.938 / 0.939** | **0.993@0.784** | | | | | |
| **Overall, 6/10 libs with B data** | | | | | | 7 outputs | 132 (9) | 0.822 | **0.516 / 0.752** | **0.702@0.478** |
| **Overall, eval-only (A: 8 libs / B: 5 libs)** | | 26 outputs | 3420 | 0.928 / 0.928 | 0.991@0.766 | 6 outputs | 55 (8) | 0.789 | 0.540 / 0.790 | 0.667@0.514 |

Group A values under the paper rule differ slightly from the pooled values in v4 where a
library has several tools (e.g. urlparser 0.941 vs pooled 0.940, deploy 0.963 vs 0.967);
both are in `group_a_table.json` (`paper_rule` vs `per_library`). The eight group-A rows
already in the paper were reproduced exactly by pooling; tulip and optipng are the two new
rows (**measured**, 2026-09-01).

**Group B status: preliminary.** All 7 artifacts are labeled and scored, but every label set
carries `reviewed_by_user: false`; `rq1_group_b_score.py` warns until that flips. The
denominator of the group-B macro-average is **the 6 of 10 corpus libraries that have at
least one function-renaming output** (5 of the 8 evaluation libraries); urlparser, lil,
tulip and optipng have none, for the stage-specific reasons in the B-tools column (never
"accuracy = 0", never "tool broken"). Read the bzip2 / lodepng rows with §Group B fact 1:
each measures matching on a handful of real pairs inside a crate that is otherwise
signature-only stubs.

**Group A aggregates (measured; `rows/group_a_table.json` → `aggregates`):**

| set | artifacts | libraries | pairs | micro P / R | macro P / R | micro coverage | micro abstention | deploy P (micro) |
|---|---:|---:|---:|---|---|---:|---:|---:|
| **evaluation** | **26** | **8** | **3420** | **0.857 / 0.858** | 0.914 / 0.914 | 0.614 | 0.387 | 0.997 |
| development | 5 | 2 | 621 | 0.965 / 0.965 | 0.969 / 0.969 | 0.850 | 0.150 | 0.998 |
| all | 31 | 10 | 4041 | 0.874 / 0.875 | 0.923 / 0.923 | 0.650 | 0.350 | 0.997 |

*Explanation, not headline (rule 3):* the evaluation micro numbers are dominated by tulip
(852 of 3420 pairs at 0.554). The same 22 evaluation artifacts without tulip pool to
P/R = 0.958 / 0.959, coverage 0.768, deploy precision 0.998 — the macro line (0.914),
which weights each artifact equally, already shows the same thing without dropping anything.

### Group A — per artifact (blind sanity check) — **measured**

Source: `rows/group_a_full.json` (runner `scripts/rq1_name_preserving_full.py`, **frozen
2026-09-01**, see *Provenance*). Precision/recall forced; abstention = ambiguous / pairs and
coverage = accepted-on-truth / pairs are deployment-mode; deploy P = correct / accepted.
`fp` = the row's fingerprint id (matcher + analyzer + C-source + header + artifact hashes).

| artifact | split | pairs | matcher P / R | abstention | coverage | deploy P | fp |
|---|---|---:|---|---:|---:|---:|---|
| genann × c2rust | eval | 12 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `6a4dd7613b52e287` |
| genann × Laertes | eval | 12 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `7f5326450afb81ac` |
| genann × C2SaferRust | eval | 12 | 1.000 / 1.000 | 0.083 | 0.917 | 1.000 | `acba9a93217ed939` |
| genann × CROWN | eval | 12 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `efb8656b711886d4` |
| qsort × c2rust | eval | 3 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `fe8b858a63929a84` |
| qsort × Laertes | eval | 3 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `0a146e499deeadc8` |
| qsort × C2SaferRust | eval | 3 | 1.000 / 1.000 | 0.000 | 1.000 | 1.000 | `26df8607c0fdcb42` |
| urlparser × c2rust | eval | 21 | 0.952 / 0.952 | 0.143 | 0.857 | 1.000 | `e8f40e8da01e28a1` |
| urlparser × Laertes | eval | 21 | 0.905 / 0.905 | 0.476 | 0.524 | 0.909 | `db0adc901bbf40b9` |
| urlparser × C2SaferRust | eval | 21 | 0.952 / 0.952 | 0.381 | 0.619 | 1.000 | `c43f7fa7795190a8` |
| urlparser × CROWN | eval | 21 | 0.952 / 0.952 | 0.143 | 0.857 | 0.944 | `090aad942c6696b3` |
| lodepng × c2rust | eval | 235 | 0.983 / 0.983 | 0.243 | 0.757 | 1.000 | `416fea472316e3ae` |
| lodepng × CROWN | eval | 235 | 1.000 / 1.000 | 0.268 | 0.732 | 1.000 | `988d280609c6cbad` |
| quadtree × c2rust | eval | 24 | 1.000 / 1.000 | 0.125 | 0.875 | 1.000 | `56153ed4ee89786a` |
| quadtree × CROWN | eval | 24 | 1.000 / 1.000 | 0.125 | 0.875 | 1.000 | `03ebfbb37dd97914` |
| bzip2 × c2rust | eval | 64 | 1.000 / 1.000 | 0.047 | 0.953 | 1.000 | `475bc0f387db3007` |
| bzip2 × Laertes | eval | 64 | 1.000 / 1.000 | 0.109 | 0.891 | 1.000 | `4e92d29d375020b4` |
| bzip2 × C2SaferRust | eval | 64 | 0.984 / 0.984 | 0.109 | 0.891 | 1.000 | `ec65d0c3c212393e` |
| bzip2 × CROWN | eval | 64 | 1.000 / 1.000 | 0.047 | 0.953 | 1.000 | `cdc5e019bfd05b61` |
| tulip × c2rust | eval | 213 | 0.554 / 0.554 | 0.831 | 0.169 | 0.972 | `e094a281a59a5dd1` |
| tulip × Laertes | eval | 213 | 0.554 / 0.554 | 0.892 | 0.108 | 0.957 | `6b7950fac78cfdc9` |
| tulip × C2SaferRust | eval | 213 | 0.554 / 0.554 | 0.845 | 0.155 | 0.970 | `fc61286571d102b1` |
| tulip × CROWN | eval | 213 | 0.554 / 0.554 | 0.826 | 0.174 | 0.973 | `5419907fa7c65901` |
| optipng × c2rust | eval | 551 | 0.953 / 0.955 | 0.238 | 0.764 | 1.000 | `b72af1c964896818` |
| optipng × Laertes | eval | 551 | 0.937 / 0.938 | 0.283 | 0.719 | 0.998 | `3dbe61d2e5d55fee` |
| optipng × C2SaferRust | eval | 551 | 0.933 / 0.935 | 0.258 | 0.744 | 1.000 | `7bcdaf9ff38314cc` |
| lil × c2rust | dev | 145 | 0.972 / 0.972 | 0.124 | 0.876 | 1.000 | `1e027159a1a1825b` |
| lil × Laertes | dev | 145 | 0.945 / 0.945 | 0.159 | 0.841 | 1.000 | `eefe01f90a5afdc0` |
| lil × C2SaferRust | dev | 145 | 0.959 / 0.959 | 0.172 | 0.828 | 0.992 | `cd2c931e44fcc6ab` |
| lil × CROWN | dev | 128 | 0.969 / 0.969 | 0.148 | 0.852 | 1.000 | `13ecbe5e96b94240` |
| cJSON × c2rust | dev | 58 | 1.000 / 1.000 | 0.138 | 0.862 | 1.000 | `435b90c2bbdd24aa` |

Missing name-preserving cells, with reasons (rule 8 applies to group A too): quadtree, lodepng,
cJSON × Laertes/C2SaferRust — those libraries are not in `laertes_benchmarks`, so no artifact
was ever produced; cJSON × CROWN and optipng × CROWN — *never produced* (CROWN crashed in
`rewrite` resp. `analyse`, `translation_matrix.md` notes 21 and the cJSON matrix); qsort ×
CROWN — *produced but not retained* (we fed c2rust's qsort through CROWN ourselves, matrix
note 20, scratch `crown_qsort_ws/` not archived). That is 31 of a possible 40 cells.

Reading notes:

- **Precision equals recall in group A wherever C functions = pairs.** Forced mode assigns
  every C function, so correct/matched = correct/pairs. optipng is the one library where they
  differ (552 C functions, 551 pairs): `png_rtran_ok` is a `static` in `pngrtran.c` whose
  every caller is compiled out under OptiPNG's `pnglibconf.h.optipng`, and none of the three
  Rust artifacts defines it; forced mode still assigns it, costing ≤ 1/552 of precision.
- **Deployment precision ≥ 0.99 in 26/31 rows; the five below it are urlparser × Laertes
  (0.909), urlparser × CROWN (0.944) and the four tulip rows (0.957–0.973).** The price is
  coverage: 5–48 % on the first eight libraries, ~25 % on optipng, **83–89 % on tulip.**
- **tulip is a homogeneous-cluster collapse, and the matcher's abstention is what makes it
  survivable.** The 213 C functions (104 `ti_*_start` stubs + 104 indicators + 5 helpers) have
  only **103 distinct static fingerprints** (signature/io/ops/consts/metrics identical); 120
  functions sit in a non-singleton group (`raw/group_a/tulip__c2rust/c_analyzer.json`). All
  four translators produce the same forced score, 0.554, because the 95 forced errors are all
  intra-cluster swaps — 54 among the `_start` stubs, 41 among unary element-wise indicators
  (`ti_abs ↔ ti_round`, `ti_acos ↔ ti_torad`). Topology cannot separate them either: every
  indicator has the same one-caller/no-callee shape. Deployment mode abstains on those
  clusters and keeps precision at 0.96–0.97 on the 11–17 % it accepts. This is the
  name-preserving-corpus preview of a limitation that group B must probe with the tool map
  and labels; it is reported as a result, not excluded (rule 3).
- **optipng required a configuration-faithful C reference.** The first pass used libpng's
  generic prebuilt `pnglibconf.h` and produced 838 C functions against 554 Rust functions —
  287 phantom C functions (`png_colorspace_*`, gamma, cHRM/sRGB) that OptiPNG's own build
  (`pnglibconf.h.optipng`, 78 `PNG_*` defines, `src/optipng/build/gcc.mk`) never compiles.
  With the correct header, 552 C vs 554 Rust. The C-header hash is now part of the row
  fingerprint so the cache cannot silently mix configurations
  (`tools/frameworks/optipng-0.7.7/PROVENANCE.txt`).
- Rows differ from the July cells (`cells/name_preserving_v1.json`) in 8 of the 24 places
  both cover; see *Reconciliation*. The July cells are superseded.

### Group B — renaming translators (primary evidence) — **labeled + scored, preliminary (labels unreviewed)**

Two shipped systems rename: **PtrTrans** (FSE'26) and **SACTOR**. `group_b_availability.md`
records, for each of the ten libraries and both tools, whether analyzable Rust exists.
Seven artifacts do; every other cell is N/A with its sub-class recorded (rule 8). Scaffolds:
`annotation/<tool>_<lib>/` (`scripts/rq1_group_b_scaffold.py`, raw in `raw/group_b/`);
labels: `annotation/<case>/labels.json` applied by `scripts/rq1_group_b_label.py`; scores:
`scripts/rq1_group_b_score.py` → `rows/group_b_full.json`.

**Truth vocabulary:** Rust fn | `NONE` | `SPLIT:a;b` | `MERGED:x` | `STUB:x` | `AMBIGUOUS`.
Scorable pair = single-fn truth. `STUB:x` (translator emitted only a signature placeholder
`x`) is **not** a pair under the strict rule; a forced/deployment match proposed on a STUB,
NONE or AMBIGUOUS row counts against precision exactly like group A's unscorable C functions.
The *lenient* line counts `STUB:x` as truth `x`, for comparison only.

**Per artifact (strict; `rows/group_b_full.json` → `rows`):**

| artifact | split | builds | C fns | truth kinds | pairs (renamed) | name-eq R | matcher P / R | deploy P@C | abst | renamed pairs: name-eq R / matcher R / deploy C | tool-map claim P (on pairs) |
|---|---|---|---:|---|---:|---:|---|---|---:|---|---|
| qsort × PtrTrans | eval | yes | 3 | 3 fn | 3 (1) | 0.667 | 1.000 / 1.000 | 1.000@0.667 | 0.333 | 1: 0.000 / 1.000 / 1.000 | 1.000 (3/3) |
| qsort × SACTOR | eval | yes | 3 | 3 fn | 3 (1) | 0.667 | 0.333 / 0.333 | **0.000@0.333** | 0.667 | 1: 0.000 / 0.000 / 0.000 | 1.000 (3/3) |
| quadtree × PtrTrans | eval | yes | 24 | 17 fn, 7 NONE | 17 (0) | 1.000 | 0.882 / 0.882 | 1.000@0.647 | 0.353 | 0 | 1.000 (16/16) |
| genann × SACTOR | eval | yes | 15 | 15 fn | 15 (0) | 1.000 | 1.000 / 1.000 | 1.000@0.867 | 0.133 | 0 | — (no map) |
| cJSON × PtrTrans | **dev** | yes | 113 | 77 fn, 31 STUB, 5 NONE | 77 (1) | 0.987 | 0.398 / 0.558 | 0.880@0.299 | 0.675 | 1: 0.000 / 1.000 / 0.000 | — (no map) |
| ↳ lenient (STUB:x = x) | | | | | 108 (2) | 0.981 | 0.500 / 0.500 | 0.960@0.231 | 0.722 | 2: 0.000 / 0.500 / 0.000 | |
| bzip2 × PtrTrans | eval | **no** | 64 | 9 fn, 52 STUB, 3 NONE | 9 (2) | 0.778 | 0.130 / 0.778 | 0.833@0.556 | 0.444 | 2: 0.000 / 1.000 / 0.500 | 1.000 (9/61) |
| ↳ lenient | | | | | 61 (19) | 0.689 | 0.296 / 0.262 | 0.833@0.098 | 0.738 | 19: 0.000 / 0.316 / 0.053 | |
| lodepng × PtrTrans | eval | **no** | 235 | 8 fn, 214 STUB, 6 NONE, 6 AMBIG, 1 SPLIT | 8 (4) | 0.500 | 0.021 / 0.625 | 0.000@0.000 | 1.000 | 4: 0.000 / 0.750 / 0.000 | 0.143 (7/226) |
| ↳ lenient | | | | | 222 (31) | 0.860 | 0.285 / 0.302 | 1.000@0.005 | 0.995 | 31: 0.000 / 0.258 / 0.000 | |

Reading the columns: forced P is low on bzip2 / lodepng / cJSON because forced mode must
answer for every C function and 52 / 214 / 31 of them have no translation — those answers
are counted as wrong (rule: unscorable rows stay in the precision denominator). Forced R and
deployment P are the matching-quality signals on the real pairs. lodepng deployment accepted
exactly one pair in the whole crate, and that one was a STUB row (strict: 0/1; lenient: 1/1).

**Per library and overall (paper rule; `rows/group_b_full.json` → `paper_rule`):**

| library | split | tools | pairs (renamed) | name-eq R | matcher P / R | deploy P@C |
|---|---|---|---:|---:|---|---|
| qsort | eval | ★ × | 6 (2) | 0.667 | 0.667 / 0.667 | 0.500@0.500 |
| quadtree | eval | ★ | 17 (0) | 1.000 | 0.882 / 0.882 | 1.000@0.647 |
| genann | eval | × | 15 (0) | 1.000 | 1.000 / 1.000 | 1.000@0.867 |
| cJSON | dev | ★ | 77 (1) | 0.987 | 0.398 / 0.558 | 0.880@0.299 |
| lodepng | eval | ★ | 8 (4) | 0.500 | 0.021 / 0.625 | 0.000@0.000 |
| bzip2 | eval | ★ | 9 (2) | 0.778 | 0.130 / 0.778 | 0.833@0.556 |
| **Overall (6 of 10 libraries with a renaming output)** | | 7 outputs | 132 (9) | 0.822 | 0.516 / 0.752 | 0.702@0.478 |
| **Overall, eval only (5 of 8 evaluation libraries)** | | 6 outputs | 55 (8) | 0.789 | 0.540 / 0.790 | 0.667@0.514 |

**Real renamed pairs are scarce: 9 of 132** — qsort `quickSort→quick_sort` (PtrTrans and
SACTOR), cJSON `cJSON_strdup→cjson_strdup`, bzip2 `fallbackSimpleSort→fallback_simple_sort`
and `mainGtU→main_gt_u`, lodepng `lodepng_read32bitInt→lodepng_get32bit_int`,
`lodepng_set32bitInt→lodepng_set32bit_int`, `lodepng_filesize→translated_function`,
`setBitOfReversedStream→set_bit_of_reversed_stream`. Name equality gets 0/9 by construction;
forced matching gets 7/9 (misses: SACTOR `quickSort` → answered `partition`; lodepng
`filesize→translated_function` → answered `lodepng_chunk_generate_crc`); deployment accepts
2/9 (`main_gt_u`, PtrTrans `quick_sort`), abstains on the other 7. That is the
name-independence evidence in this table — small, and it must be reported as 9 pairs, not
as a percentage.

| artifact | split | builds | C fns | Rust fns | tool map | candidate renames | no same-name Rust fn | elided by tool | matcher abstains (deploy) | rows to decide | fp |
|---|---|---|---:|---:|---|---:|---:|---:|---:|---:|---|
| qsort × PtrTrans | eval | yes | 3 | 3 | shipped, 3 claims | 1 | 1 | 0 | 1 / 3 | 1 | `1bf04d8c6acdeb40` |
| qsort × SACTOR | eval | yes | 3 | 3 | `function_name_map.json` | 1 | 1 | 0 | 2 / 3 | 2 | `fda2c8e04711c9d9` |
| quadtree × PtrTrans | eval | yes | 24 | 17 | shipped, 16 claims | 0 | 7 | 7 | 6 / 24 | 3 | `eaa3207cdb42cbe2` |
| genann × SACTOR | eval | yes (assembled) | 15 | 15 | none | 0 | 0 | 0 | 2 / 15 | 0 conflicts | `d5122fb19359e89e` |
| cJSON × PtrTrans | **dev** | yes | 113 | 108 | none retained | 7 | 7 | — | 83 / 113 | 48 | `bbd05c6dbe84e6a7` |
| bzip2 × PtrTrans | eval | **no** | 64 | 61 | shipped, 61 claims | 19 | 22 | 3 | 48 / 64 | 35 | `d93d43403b9a797a` |
| lodepng × PtrTrans | eval | **no** | 235 | 252 | shipped, 226 claims (2 undefined) | 126 | 36 | 3 | 234 / 235 | 205 | `dfbdc9c3007a6e5f` |

"candidate renames" = tool claim ≠ C name (or, without a map, no same-name Rust function);
an upper bound on the realized rename count, to be settled by labels. "rows to decide" =
`CONFLICT` rows (tool map, name equality and matcher disagree) plus, without a map,
matcher-only rows.

Labeling settled the candidate counts: realized renames are PtrTrans qsort 1/3, quadtree
0/17, bzip2 2/9, lodepng 4/8, cJSON 1/77; SACTOR qsort 1/3, genann 0/15. The remaining
"candidates" were stubs (bzip2 17, lodepng ≈120) or map corruption, not renamed translations.

**Facts the labels establish (preliminary until user-reviewed):**

1. **The shipped PtrTrans bzip2 and lodepng crates are stub shells; a third of cJSON is.**
   bzip2 52/64 C functions, lodepng 214/235, cJSON 31/113 have only a signature-only
   placeholder (empty body / `0` / `None` / `unimplemented!()`, doc comment citing the C
   signature). bzip2's git history is one "Initial translation" commit per function with the
   bodies never filled; lodepng additionally has dozens of duplicate same-name definitions
   (the reason it cannot compile) and fragments under generic names (`translated_segment`,
   `unnamed_loop_function`). This is a translator-output property. The v4 explanation "the
   matcher is uninformative under degraded static analysis" was only half right: the matcher
   abstains because there is almost nothing to match — on the real pairs it still gets forced
   R 0.778 (bzip2) and 0.625 (lodepng).
2. **SACTOR's nested-helper reshaping produces a deployment false accept — confirmed.**
   Labels: `swap→swap`, `partition→partition`, `quickSort→quick_sort` (all three via nested
   inner fns). Forced answers `partition→quick_sort`, `quickSort→partition`; **deployment
   accepts `partition→quick_sort` (conf 0.047 > eps)** and abstains on the other two. That is
   the one deployment false accept in the whole group-B set (sactor_qsort deploy P = 0/1),
   the failure mode rule 1 exists for.
3. **Forced ≠ deployment on the smallest cell, as predicted.** qsort × PtrTrans: forced 3/3,
   deployment 2/3 (abstains on `partition`).
4. **PtrTrans's realized renaming is artifact-dependent and small.** quadtree renames nothing
   and elides the seven `*_free`/`*_reset` functions (labeled NONE — the crate has no
   `impl Drop`, they are simply gone); bzip2 and lodepng renames are camelCase→snake_case
   plus one fragment under a generic name (`lodepng_filesize→translated_function`).
5. **On building PtrTrans artifacts matcher and map agree**: quadtree 15/17 forced correct;
   the two errors are the swapped one-line predicates `quadtree_node_isempty` ↔
   `quadtree_node_ispointer` (deployment abstains on both). Map claim precision on labeled
   pairs: PtrTrans qsort 3/3, quadtree 16/16, bzip2 9/9, **lodepng 1/7**; SACTOR qsort 3/3.
6. **Where the crate is a stub shell, forced mode is the wrong configuration to quote.** It
   must answer for 52 / 214 / 31 STUB rows and is scored wrong on each; deployment declines
   almost all of them (accepts 1/52 bzip2, 1/214 lodepng, 5/31 cJSON stub rows — the rest are
   abstentions or no candidate). The strict per-artifact line is the primary one; the lenient
   line shows what "matching stub to stub" would look like.

**N/A cells and why** (full table in `group_b_availability.md`): *never produced* —
PtrTrans urlparser (not run: C reference fails the UB gate), tulip (PA pre-pass crash),
optipng (PA > 2 h); SACTOR urlparser (same gate), cJSON & lil (circular-dependency refusal
before any LLM call), bzip2 & optipng (C-parser failure). *Produced but not retained* —
PtrTrans genann & lil (July scratch runs cleaned); SACTOR quadtree (15 leaf fns), lodepng
(~50 fns), tulip (104 indicators, link-fail) — these five cells are analyzable in principle
and are N/A only because of our archiving; filling them is one paid LLM run each.

### Reconciliation with the July cells (`cells/name_preserving_v1.json`)

The July run was recall-only and used the matcher as of 2026-07-07; the 2026-09-01 rows use
the matcher at commit `509751d` (signal-C 0.20, input-scalar, strings, hir-id identity,
partial matching; commits `932907c`, `509751d` post-date the July cells). 16/24 rows are
identical. The 8 that differ:

| row | July R | now R | Δ fns | note |
|---|---:|---:|---:|---|
| quadtree × CROWN | 0.625 | 1.000 | +9 | July note blamed `--force-box` io-shape drift; current matcher recovers all 24 |
| qsort × Laertes | 0.667 | 1.000 | +1 | 3-function artifact; one function = 0.33 |
| lil × CROWN | 0.922 | 0.969 | +6 | |
| lodepng × CROWN | 0.974 | 1.000 | +6 | |
| urlparser × CROWN | 1.000 | 0.952 | −1 | July scored 20 pairs (excluded `main`); now 21 |
| lodepng × c2rust | 0.991 | 0.983 | −2 | |
| lil × Laertes | 0.959 | 0.945 | −2 | |
| lil × C2SaferRust | 0.986 | 0.959 | −4 | |

Net: 5 rows up, 3 down, by 1–9 functions. **Corpus-hygiene rules now encoded in the runner:**
(i) the Rust side excludes driver/example/test modules exactly as the C side does (with them
included genann reads 0.58 instead of 1.00); (ii) the C reference must be the version *and
build configuration* the artifact was translated from — `laertes_benchmarks` and
`crown/c-code` ship different `genann.c` (12 vs 15 fns) and `lil.c` (145 vs 128); tulip's
Rust artifacts are 0.8.4 not 0.9.2; optipng's are 0.7.7 not 0.7.6, with OptiPNG's own
`pnglibconf`.

### Provenance (runner frozen 2026-09-01)

- Runner `scripts/rq1_name_preserving_full.py`; pooling `scripts/rq1_group_a_table.py`;
  topology `scripts/rq1_topology_resolution.py`; group-B scaffold
  `scripts/rq1_group_b_scaffold.py`. Tool versions recorded in every row/sheet: matcher
  `509751d` (sha `fe49e5bb…`), analyzer src `509751d` / bin `b2adb5ec…`, `c_analyzer.py`
  `b771a6b0…`, repo HEAD `b647a44`, `stu_selector_dirty: false`.
- Row cache is keyed by fingerprint (matcher/analyzer hashes, C sources + headers, Rust
  artifact tree, `eps`, `topo`, exclusions); a changed input recomputes the row.
- Duplicate Rust leaf names are recorded as `ambiguous_truth` (group A: 0 in all 31 rows)
  or `DUPLICATE` (group B: 0 in all 7), never silently resolved.
- No further logic changes to the runner; a change requires a new version block here and a
  full `--force` re-run.

## Table 2 — matcher ablation — **development data only**

| method | recall | precision | coverage | abstentions |
|---|---:|---:|---:|---:|
| signature only | 0.652 | 0.644 | 1.00 | 0 |
| + metrics / operators | 0.820 | 0.811 | 1.00 | 0 |
| + call-graph topology | 0.876 | 0.867 | 1.00 | 0 |
| + abstention (deployment) | 0.708 | 0.969 | 0.73 | 25 |

Measured on the raw-LLM micro corpus (10 programs we translated ourselves), **entirely
development data** under the frozen split. Component study, not held-out evidence.
Replicating the ladder on the labeled group-B artifacts is open work. The `lil`
homogeneous-cluster column (0.359 → 0.742 → 0.984) shows topology cracking a cluster of
near-identical functions — also development data, and tulip (above) is the held-out case
where topology cannot, because the cluster is topologically uniform too.

## Supporting results (not primary evidence)

**S1 — Identifier scrambling: the matcher does not read names.** Renaming every C function to
`c_####` and every Rust function to `r_####` in the analyzer output, preserving topology,
leaves the matched set identical (90 pairs, delta 0) while name-equality recall drops
0.124 → 0.000. *Measured on the micro corpus (dev).* No stored run exists for the group-A
artifacts, although `matcher_master_table.md` asserts one in prose — close the gap or narrow
the claim.

**S2 — raw-LLM translation: behaviour under aggressive restructuring.** Our own gpt-5.1
translations of 7 real libraries, hand-labeled. A **controlled stress test**, not evidence
about shipped translators:

| library | split | true pairs | matcher R | name-eq R | realized rename |
|---|---|---:|---:|---:|---|
| qsort | eval | 3 | 0.667 | 0.000 | full |
| genann | eval | 8 | 1.000 | 0.000 | full |
| urlparser | eval | 17 | 0.882 | 0.000 | full |
| quadtree | eval | 17 | 0.706 | 0.000 | full |
| bzip2 | eval | 41 | 0.634 | 0.146 | heavy |
| cJSON | dev | 40 | 0.550 | 0.150 | partial (6 parse-core names kept) |
| **lil** | dev | 111 | 0.550 | 0.550 | **none — the model ignored the instruction** |
| **overall (7 libs)** | | **237** | **0.616** | **0.308** | |
| evaluation only (5 libs) | | 86 | 0.744 | 0.070 | |

**S3 — qsort downstream effect.** Name equality recovers 2/3 correspondences and 0/1 defective
public contract boundaries, but still 1/1 unique defects, because the name-preserved internal
`partition` exposes the same faulty index rewrite (30,480 / 50,000 records divergent). **A lost
contract boundary, not a lost defect.**

## Independent finding — translator-supplied maps are not trustworthy inputs

PtrTrans's shipped lodepng map disagreed with the Rust artifact on 143/255 entries in the July
mechanical audit. The scaffold explains most of that (63 record-shifted claims, 21
placeholders/libc); the bzip2 map has no shift. **Against manual truth (preliminary,
`tool_map_audit` in `rows/group_b_full.json`, restricted to rows with a labeled pair):**
PtrTrans qsort 3/3, quadtree 16/16, bzip2 9/9, **lodepng 1/7** (the record-shift); SACTOR
qsort 3/3. On STUB rows the PtrTrans map names the stub (bzip2: all 52), i.e. it is a map of
what the translator *emitted*, not of what it *translated* — a map cannot tell the two apart,
a body-reading matcher can. The four-way sub-classification (record-shifted / ambiguous /
missing-abstained) of lodepng's 226 claims is still to do. Two systems ship maps; one is
sound, one is not — that establishes the class and justifies an independent matcher, not a
general claim.

---

## Development / evaluation split — **frozen** (`SPLIT.md`)

Development: cJSON, lil, `benchmark/pairs`. Evaluation: qsort, genann, urlparser, quadtree,
lodepng, bzip2, tulip, optipng. Nothing moves after results are seen.

## Open work, in dependency order

1. **User review of the group-B labels** (`annotation/<case>/labels.json`, 457 rows; every
   row has an evidence note). Highest-value rows to check first: the 9 real renamed pairs,
   the 3 sactor_qsort rows, lodepng's 8 real pairs + 6 AMBIGUOUS + 1 SPLIT, and a sample of
   STUB rows in bzip2/lodepng/cJSON. Flip `reviewed_by_user` per case; the scorer's warning
   disappears and the numbers stop being *preliminary*.
2. **Missing library–tool combinations** (user's step 2; all paid LLM runs, need a decision):
   urlparser × PtrTrans and × SACTOR were never run because of the C-side UB gate, which is
   irrelevant to static matching — ~15 functions, the cheapest way to add a 7th library;
   PtrTrans genann / lil and SACTOR quadtree / lodepng / tulip re-runs (produced-but-not-
   retained); tulip / optipng × PtrTrans only via the `LLM_only` baseline config (Trans_PA
   fails at the PA stage), which must be labeled as a baseline configuration, not PtrTrans.
   SACTOR cJSON / lil / bzip2 / optipng and PtrTrans Trans_PA tulip / optipng stay N/A
   (never produced, stage recorded).
3. **Four-way re-audit of the lodepng PtrTrans map** against the labels (record-shifted /
   ambiguous / missing-abstained), 226 claims.
4. **Close the S1 gap**: run the identifier-scramble check on the 26 evaluation group-A
   artifacts (the runner has the artifact list).
5. Optional: whether rust-analyzer can resolve calls in non-building crates (and the
   `r#type` inventory gap — `lodepng_chunk_init` is defined but not inventoried).
