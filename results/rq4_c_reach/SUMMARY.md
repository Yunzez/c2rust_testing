# Same-corpus C reach — 37 cells, plus a three-cell C-guided companion pilot (2026-09-11)

Plan and definitions: `docs/c_reach_plan.md`. Study table: `table.md` (regenerate with
`scripts/rq4/c_reach_table.py <root> table.md`). Per cell `<lib>_<tool>/`: `result.json` (side-specific reach, per-input
outcomes, matched counts), `matched_sets.json` (the four sets with names, ambiguous / unmatched lists, and for every
exclusive function its provenance: reaching boundaries, the other side's coverage status, `rust_terminated` /
`c_terminated`), `per_input.json` (outcome + harness phase per input), `c_functions.json`, `c_regions.json.gz`
(per boundary: reached functions and covered region identities, aligned to the pair's source), `RUN.md`.
Raw llvm-cov exports and the edited C copies (518 MB) are outside the repo:
`/home/yunzez/c2rust_archive/rq4_c_reach_raw_2026-09-11.tar.gz`. Tooling: `scripts/rq4/c_reach.py`,
`scripts/rq4/c_guided_cell.py`, generator 0.9.2 `--c-coverage`.

## What was measured

Each cell's archived corpus (Rust-guided: `C2R_MODE=rust-only` fork campaign) was replayed in `C2R_MODE=c-only`
through harnesses rebuilt with the C oracle compiled `-fprofile-instr-generate -fcoverage-mapping`
(`cargo fuzz build --sanitizer none`). C universe = every instrumented C object file of each boundary (helper
units excluded), union over boundaries by identity; regions = llvm-cov code regions in files that define
functions; the Rust side is the archived cell, not re-measured. Four sets over the matcher's `deployment` pairs
(frozen `abstain_eps` 0.01) ∩ C scope ∩ Rust scope; ambiguous and unmatched functions in their own columns.
40 350 inputs replayed on the C side (40 021 completed, 315 crashed, 14 timed out).

**This is a paired reach diagnostic on a Rust-guided corpus, not a C coverage baseline.** The corpus grew only
where the Rust side let the fuzzer go, so the C numbers are a floor on what those inputs exercise in C. A C-side
`completed` shows reach, not C-definedness; reference-limited stays a confirmation-channel verdict.

## Results

| | accepted pairs | both | C only | of which Rust terminated | Rust only | of which C terminated | neither | ambiguous |
|---|---|---|---|---|---|---|---|---|
| 36 cells with a map | 2 651 | 1 096 | 82 | 53 | 12 | 9 | 1 461 | 1 510 |
| 10 c2rust cells (control) | | | 4 | 4 | 0 | 0 | | |

*Terminated* = the other side crashed / panicked / timed out on every input that reached the function, so its
reach there is unmeasured, not measured-unreached; whether that termination is a defect or UB-associated is the
cell's confirmation verdict, never this replay's. qsort × CROWN has no map (side-specific numbers only).

**Negative control passes.** On the ten c2rust cells the exclusive sets are empty once terminations are set aside
(4 C-only functions, all on inputs where the Rust replay died: urlparser `get_part`, lodepng ×3).

**Measured asymmetries — the same inputs, one side completes, the other side never enters the function:**

| cell | C only | Rust only | what |
|---|---|---|---|
| optipng × Laertes | 27 | 0 | zlib deflate internals (`deflate`, `fill_window`, `build_tree`, `compress_block`, …) reached through `compress` / `compress2`, whose Rust replays completed. A silent early return on the Rust side. **To hand-verify** (Laertes severed-init family). |
| bzip2 × CROWN | 1 | 0 | `mainSort` via `BZ2_bzBuffToBuffCompress`; the Rust replay of that boundary lost 19 of 187 inputs — the path of the confirmed C7 / S10 (`SET_BH`). |
| bzip2 × Laertes | 1 | 0 | `mainSort`, same boundary, 12 of 296 Rust inputs lost. |
| optipng × C2SaferRust | 0 | 3 | `fixedtables`, `updatewindow`, `inflate_table` via `uncompress` (97 inputs, all completed on C): Rust goes deeper than C on the same bytes. **To hand-verify** against the cell's 1 199 root-caused confirmed rows. |

**Terminated exclusives with a translation root cause:** lil × C2SaferRust — 37 C-only functions
(`lil_new`, `hm_*`, `lil_parse`, …): C constructs the interpreter and parses, the Rust side crashes at
construction (C9, CRASH-ALL). urlparser × Laertes — 5 C-only (`url_parse`, `url_is_protocol`, …), Rust crashes on
the same inputs.

**Both-low cells are input-model / construction limits, not translation.** Same C reach for every tool of a
library on the same corpus: urlparser 9/21 (nine boundaries segfault in C on the default seed), lodepng 56/235
(the PNG entry points were never planned), cJSON × PtrTrans 10/113 (64 boundaries unbuilt), optipng
101–133/552, tulip 212/213 functions but 0.27 of regions (option guards; the seed ablation's finding).

## C-guided companion pilot (`c_guided_pilot/`)

Same harnesses, seed, libFuzzer parameters and 3 600 s budget with `C2R_MODE=c-only` (the C oracle is already
sancov-instrumented, so libFuzzer is guided by C's edges alone), then both sides measured on the C-guided corpus CC.

| cell | corpus | C fn | C reg | Rust fn | Rust reg | both | C only (Rust term.) | Rust only | neither |
|---|---|---|---|---|---|---|---|---|---|
| lil × C2SaferRust | CR (Rust-guided, 340 inputs) | 62/145 | 0.203 | 25/154 | 0.063 | 24 | 37 (37) | 0 | 58 |
| | CC (C-guided, 3 575 inputs) | **138/145** | **0.821** | 25/154 | 0.063 | 24 | **92 (92)** | 0 | 3 |
| lil × c2rust (control) | CR (4 756) | 139/145 | 0.839 | 143/151 | 0.87 | 123 | 0 | 0 | 4 |
| | CC (3 561) | 140/145 | 0.85 | 144/151 | – | 124 | 0 | 0 | 3 |
| lodepng × c2rust | CR (1 000) | 56/235 | 0.12 | 54/236 | 0.13 | 43 | 3 (3) | 0 | 132 |
| | CC (1 498) | 56/235 | 0.12 | 54/236 | – | 43 | 3 (3) | 0 | 132 |

Reading: with C guidance lil × C2SaferRust's C reach rises to the level of the faithful translation's (0.82 vs
0.84) while the Rust side does not move — the cell's 0.06 is entirely translation-induced. The control makes no
false asymmetry under either guidance. lodepng is identical under both guidances: a construction limit, not a
feedback limit. Candidates found by the C-guided campaigns are reach only; none is adjudicated.

## Caveats and deviations

- Rust regions on CC are measured; CR ∪ CC is function level (by name) only.
- `BZ2_hbCreateDecodeTables` (bzip2, all tools) does not rebuild under the current generator (planner drift,
  E0425) and is counted unbuilt, as in the census.
- tulip: the matcher accepts 36 of 213 pairs (177 ambiguous), so its four sets cover 36 pairs; side-specific
  numbers cover everything.
- genann × SACTOR and cJSON × PtrTrans compiled their own C file: their C universes are per pair.
- The C-guided pilot's `c_regions_cc.json.gz` is unaligned for `c_static` boundaries (the edited C copies of
  those work trees were not kept); function-level data is unaffected.
- No paper text was changed. Decision pending (user): whether the C-guided companion becomes a formal 37-cell
  experiment (uniform, never a subset).
