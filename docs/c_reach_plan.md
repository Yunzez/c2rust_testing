# Same-corpus C reach — a paired reach diagnostic for the RQ4 cells

Status: PLAN, 2026-09-11. Nothing measured yet. Implementation starts after the seeds-only census
lane finishes (no generator edit while a lane executes it).

## 1. The question, and what this is not

Every RQ4 cell reports how much of the *translated artifact* the generated campaign exercises
(`results/rq3_coverage/<lib>/<tool>/analysis/result.json`, Rust-side function and region reach).
Several cells are low, and today the explanation of *why* is per-case prose (input-domain guards
on tulip, object construction on lodepng/optipng, crash-all on lil x C2SaferRust). This diagnostic
turns that explanation into a measurement:

> For exactly the same inputs, where does the original C go, and where does the translation go?

It is **not** a C coverage baseline. The corpus was produced by Rust-only fuzzing
(`scripts/rq4/cell.py:171`, `C2R_MODE=rust-only`): C never contributed coverage feedback. It is
therefore a *paired reach* number on a Rust-guided corpus. The paper wording (agreed 2026-09-11):

> We additionally replay each final corpus in C-only mode. This is not an independently fuzzed C
> baseline: the corpus remains Rust-guided. Instead, it is a paired reach diagnostic that determines
> whether the same inputs also exercise the reference implementation. We compare reach over matched
> functions, while retaining side-specific region percentages because C and Rust regions are not
> structurally interchangeable.

The main RQ4 table keeps reporting Rust artifact reach. This adds one small table (or figure) of
matched-function Both / C-only / Rust-only / Neither counts per cell.

## 2. What already exists (verified in the tree, 2026-09-11)

* **c-only mode exists in every archived harness shape.** `gen_diff_harness.py:1931-1950` reads
  `C2R_MODE`; the `c-only` branch (`:2044-2054`) decodes the input exactly as the other modes, runs
  the producer bridge (cJSON) and the C entry, frees, and skips the Rust call, the comparison and
  the in-loop UB gate. Plan-lowered (`--plan`) targets are the same `gen_target`. The only harness
  without modes is the E3 pure-Rust target, which RQ4 does not use.
* **c-only has only ever run one input at a time** (`c2r_campaign.py:85`, confirmation channel A)
  or on the empty input (preflight, `cell.py:263`). A corpus-wide c-only replay is new code; the
  per-input template is `cell.py:329-359` (the Rust per-input coverage fallback).
* **The C objects carry no coverage instrumentation.** `build.rs` template `gen_diff_harness.py:2451-2467`:
  clang, `-O1 -g -fsanitize-coverage=inline-8bit-counters,pc-table,trace-cmp` + UB-free flags,
  static lib `c_oracle`. No `-fprofile-instr-generate` / `-fcoverage-mapping` anywhere in
  `scripts/`, `tools/`, `docs/`. But `libclang_rt.profile` is already linked statically
  (`:2464`), so adding the two flags needs no link change.
* **Tool versions are compatible.** System clang 21.1.8 compiles the C; the pinned toolchain's
  `llvm-cov` / `llvm-profdata` are LLVM 21.1.0. Same major, same profile format. The protocol
  rule "tools from the toolchain only" still applies to the export.
* **Corpora are archived per boundary in every one of the 37 cells** (`corpus.tar.gz` ->
  `corpus/<boundary>/*`), and each cell's `harnesses/<b>/{build.rs,Cargo.toml,fuzz_targets}` is
  archived, so the exact C build line is recoverable. Binaries are not archived: every cell must be
  rebuilt (`scripts/rq4/seed_experiment/rebuild_bins.py` does exactly this; the 2026-09-11
  `c2r_funnel.py` re-export fix is required for cJSON x c2rust).
* **The C<->Rust function map is not in the pair.** It is
  `results/rq1_matching/raw/group_a/<lib>__<tool>/matcher_output.json` (name-preserving) and
  `group_b/<tool>_<lib>/matcher_output.json` (idiomatic); entries are `[c, rust, score, confidence]`
  in `forced` (full bijection) and `deployment` / `deployment_ambiguous` (after abstention).
  **qsort x crown has no map** (side-specific numbers only for that cell).
* **C source is shared per library except genann x SACTOR and cJSON x PtrTrans**, which consumed a
  different C file. bzip2, quadtree and tulip compile an *amalgamation* (`bzip2lib.c`,
  `quadtree_all.c`, `tulip.c`); optipng compiles 52 translation units.
* A one-off version of this comparison exists for lil (gcov, July 2026,
  `fuzz/lil_coverage/README.md`) and was never generalised. Its lesson carries over: a killed
  process writes no profile, so hanging or crashing inputs contribute zero unless isolated.

## 3. Design

### 3.1 Per cell

1. **Rebuild** the cell's harness binaries with the current generator plus a new flag
   `--c-coverage` that appends `.flag("-fprofile-instr-generate").flag("-fcoverage-mapping")` to
   the C build. Flag off by default -> golden regression (33 entries) unchanged. UB-free flags stay
   as archived; `--c-sanitize` is **not** combined with coverage (`-fno-sanitize-recover=all`
   aborts on the first UB hit and truncates the profile). The Rust side of this build is not
   measured (the archived `harness_exports.tar.gz` is the Rust measurement; no re-fuzzing, no
   Rust re-measurement).
2. **Replay the archived corpus in c-only mode**, per boundary, `C2R_MODE=c-only`,
   `LLVM_PROFILE_FILE=<dir>/%m-%p.profraw`: batch first (the binary with the corpus directory,
   `-runs=0`-style replay, 900 s cap, `-timeout=25`), per-input fallback when the batch dies —
   the same two-step as `cell.py:307-359`. Record per input: `normal` / `crash` / `timeout`, and
   the sanitizer or ubshim signature on crash. With the gate stripped, every input reaches C, so
   the C side may crash where the Rust campaign was never told; that count is itself a result
   (§4, "reference-limited").
3. **Merge and export**: `llvm-profdata merge -sparse`, then `llvm-cov export <bin>
   -instr-profile=...` from the pinned toolchain. Scope = the library's own C files (the
   `build.file(...)` units of `build.rs`), excluding `ubshim.c`, `shims.c`, `c2r_plugin.c`. The
   export lists every function in the mapping, executed or not, so the C universe comes from the
   same export; no separate denominator build. The C identity is `(file, start line)` in the
   compiled file's coordinates (for an amalgamation that is `bzip2lib.c:1234`, not
   `blocksort.c`), unioned across boundaries by identity exactly as `c2r_coverage.py` does.
   **The C universe is built from every instrumented C object file** (the members of
   `libc_oracle.a` / `$OUT_DIR/**/*.o`, unpacked, exported with `-object` per file), never
   inferred from the linked binary: the linker pulls only referenced archive members, so a
   binary-derived denominator collapses exactly as the rlib archive-selection denominator did
   (`docs/rq4_denominator_decision_2026-09-08.md`; `rlib_universe.py --empty-profile` is the
   Rust-side precedent). For an amalgamation the two coincide; for optipng (52 units) they do not.
   The covered set is exported from the same object list with the merged profile.
4. **Join to the Rust side by name, not by coordinates.** Rust reach per function comes from the
   archived `analysis/covered_by_*.txt` (demangled names) of the same cell. C names come from the
   export. The correspondence is the matcher's **`deployment`** list — the pairs accepted under
   the frozen abstention threshold (`abstain_eps` 0.01) — plus `renames.json` where the pair has
   one. `forced` is **not** used: on the renaming translators its precision was ≈0.52, and a wrong
   pair manufactures one `c_only` and one `rust_only` out of nothing.
   **Universe of the four sets = accepted correspondence pairs ∩ C scope ∩ Rust scope.** Over it:
   `both`, `c_only`, `rust_only`, `neither`. Outside it, reported in their own columns and never
   folded into `neither`: `ambiguous` (pairs the matcher abstained on), `c_unmatched` (in-scope C
   functions with no accepted pair), `rust_unmatched` (in-scope Rust functions with no accepted
   pair). Any asymmetric pair (`c_only` or `rust_only`) that a paper sentence rests on is checked
   by hand against the two sources before it is cited.
5. **Producers count as reach, with the phase recorded.** The Rust coverage already includes the
   producer calls (cJSON's `cJSON_Parse` etc. run inside the harness on both sides); dropping them
   on the C side would bias the pair. Per input, the C replay additionally records the phase it
   reached from the existing markers (`C2R_PH_PRODUCER` -> `C2R_PH_C` -> `C2R_PH_C_DONE`):
   *producer reached -> target entered -> target completed*, so producer-only reach is visible
   without being removed from coverage.

### 3.2 Reported numbers

Per cell: C function reach `c_fn / c_total`, C region reach `c_reg / c_total_reg` (side-specific,
never subtracted from the Rust percentage), the four matched-function set sizes, unmatched counts,
and the per-input outcome distribution on the C side (normal / crash / timeout).
Study-level: one table of the four sets for all 37 cells (36 with a map), and one sentence per
pattern in §4. No per-application seed numbers, no "coverage regression" wording.

## 4. Reading the four sets

| pattern | reading | expected where |
|---|---|---|
| both low, few C-only | input model or semantic guard limits both sides (input-domain limitation) | tulip without grid seeds; genann; lodepng/optipng PNG entry points (unplanned = construction unsupported, not counted as C-only) |
| C reached, Rust not (`c_only` large) | translation-induced reach loss | lil x C2SaferRust (`lil_new` construction fails in Rust); cJSON x PtrTrans direct constructors (C continues, Rust stops at the `None` stub) |
| Rust reached, C not (`rust_only`) | either a matcher error or Rust taking a path C cannot on the same bytes | should be near zero on c2rust cells; every case looked at |
| C side crash/timeout on many inputs | *candidate* reference-limited: C leaves early on these inputs; whether they are C-undefined is decided by the ASan+UBSan confirmation, not here | cJSON, lil (`UB-associated`, never a defect) |
| boundary not built | construction unsupported; C-only reach cannot be attributed to it (the 64 unplanned cJSON x PtrTrans boundaries stay unbuilt on both sides) | as in each cell's funnel |

**Negative control.** The c2rust cells (10) are faithful, name-preserving translations: on the same
inputs the two *exclusive* sets `c_only` and `rust_only` should be ~0 (not the two coverages — a
c2rust cell may legitimately have low `both` when the input model limits both sides). `both` should
equal the archived Rust reach on the accepted pairs. A c2rust cell with a non-empty exclusive set
is a bug in the diagnostic (map, join, identity, lost profile) until shown otherwise, and is fixed
before any non-c2rust number is read.

**Reach is not definedness.** A C-side `normal` return on an input shows only that C *got there*.
It does not show the input is C-defined; "reference-limited" is still adjudicated by the
confirmation channel (C alone under ASan + full UBSan, `c2r_campaign.py` phase A), never by this
replay. The per-input `crash` count here is a reach statistic, not a UB verdict.

## 5. Steps and order

0. **Pilot, three cells, one lane** — bzip2 x c2rust (negative control, 19 boundaries, largest
   corpus), lil x C2SaferRust (expected C-only shape), tulip x c2rust (expected both-low shape).
   Gate: control passes §4's negative-control check; every C-side profile accounted for (inputs =
   normal + crash + timeout); C universe function count equals the number of functions in the
   compiled scope files. Stop and re-decide if the control fails.
1. **Tooling** (after the census lane is done, in the `seed-ir` worktree or after its merge):
   `gen_diff_harness.py --c-coverage` (build.rs only; golden unchanged),
   `scripts/rq4/c_reach.py <cell> <out>` (rebuild via `rebuild_bins.py --c-coverage`, replay,
   export, join, per-cell `result.json` + `RUN.md`), `scripts/rq4/c_reach_table.py` (study table).
   No edit to any script while a lane executes it.
2. **All 37 cells**, three lanes at most (scratchpad file quota), targets deleted per cell,
   exports gzipped. Rebuild is the cost: ~8 min per cell; replay is seconds to minutes except
   bzip2 x c2rust / x Laertes. Estimate 6–10 machine hours, ~3–4 h wall.
3. **Archive** under `results/rq4_c_reach/<lib>_<tool>/`: `c_export.json.gz`, `per_input.json`,
   `matched_sets.json` (the four sets with names), `RUN.md` (procedure, deviations, what is not
   established), and `results/rq4_c_reach/SUMMARY.md` + `table.md`. Nothing in
   `results/rq3_coverage/` changes.
4. **Text**: the definition paragraph above, the small table, one paragraph per §4 pattern with
   the cells that show it. The user edits the paper; this document and the SUMMARY carry the
   numbers.

## 6. Caveats to write down before the first number

* **The corpus is Rust-guided.** Regions C reaches only through C-specific control flow were never
  targeted. C reach here is a floor on what those inputs exercise in C, not C coverage.
* **The map has errors** (RQ1 macro P/R 0.829/0.874 over the ten libraries). A wrong pair shows up as
  one `c_only` plus one `rust_only`; the table reports `deployment_ambiguous` counts and the
  `deployment`-only variant so the reader can bound the effect.
* **Private C functions** (`c_static`, exposed by `strip_static_c`) are in the C universe as they
  are in the Rust one; the funnel marks which boundaries are private.
* **Amalgamation coordinates**: C identities are in the compiled file; no C linemap exists and none
  is needed, because the join is by name.
* **Inlining**: `-fcoverage-mapping` instruments per source function before inlining, so an inlined
  callee is still counted. `-O1` stays as archived.
* **Strip**: `cell.py:132` strips debug info from campaign binaries; the C-reach build keeps the
  unstripped binary until export (the `__llvm_covmap` sections must survive).
* **optipng** compiles 52 TUs; the scope filter is the `build.file` list, and the C universe is
  correspondingly large. This is the cell where "both low" is expected to dominate.
* **genann x SACTOR and cJSON x PtrTrans** have their own C file; their C universes are per pair,
  not per library, and their C reach is not comparable to the sibling cells' C reach.
* **Timeouts**: the same 25 s per-input cap as the campaign; a timed-out input contributes no
  profile and is counted, never silently dropped.

## 7. Decisions (user, 2026-09-11)

1. Map: the frozen-threshold **`deployment`** pairs; `forced` is not used. Ambiguous and unmatched
   functions are their own columns, never in `neither`. Asymmetric pairs cited in the paper are
   hand-verified.
2. Scope: private (`c_static`) functions are included when in scope and reliably paired — reach
   through an indirect call is real reach.
3. Producers count as reach (they are in the Rust coverage already); the C replay records
   producer reached -> target entered -> target completed per input in addition.
4. Negative control is stated on the exclusive sets, not on the coverages.
5. C-only `normal` is reach, not C-definedness; reference-limited needs the sanitizer confirmation.
6. The C denominator comes from all instrumented C object files, not from the linked binary.
