# ARCHIVED — FSE plan (superseded)

> **SUPERSEDED 2026-09-02 by `results/EVALUATION_PLAN.md`.** This document
> records the former three-axis evaluation and must not be used to recover the
> current RQ structure. Its experimental history remains useful.

Former canonical plan. Supersedes `results/archive/FSE_PLAN_2026-08-09.md` (kept for the framing argument in its §1)
and `PROJECT_RESET_2026-07-03.md`.

**Revision history**
- **2026-08-09 (v1)** — framing decision: this is a *fuzzing* paper, static-prediction thesis withdrawn.
  Oracle ablation named as the core experiment. Certificate normalisation named as the blocker.
- **2026-08-25 (v2)** — two rounds of external review merged. Added the alignment ablation axis, the
  real-library UB gap, and four **binding constraints** (§2, extended to six in v3) that correct over-claims made while
  planning. Nothing below has been run yet.
- **2026-08-25 (v3)** — review round 3: alignment pilot moved off lodepng (does not build) to
  qsort×PtrTrans; observation axis recast as four channels O-R/O-P/O-S/O-F with a two-driver qsort
  pilot; ATTR witnesses added (lil fn 11 / fn 15); axes renamed OBS/ATTR/ALIGN; a proposed
  "level-2-only" witness was found false and dropped.
- **2026-08-25 (v4)** — post-pilot corrections (commit 747f5f0, `results/ablations/{observation,attribution}/*/RESULT.md`): §4 items 2/3/4
  marked PILOT DONE; lil order dependence (fn 15) **retracted** and removed as a C-unstable witness;
  "reference version provenance" added as an attribution requirement; ALIGN reframed as three separate
  metrics (new C7); two pipeline fixes recorded.

---

## 1. Frozen thesis and contribution list

**Do not restate the contribution in any other words than these until the paper is drafted.**

> The reliability of C→Rust differential validation turns on three oracle-design dimensions that prior
> work leaves implicit: **what state is observed**, **how source-C undefined behaviour is attributed**,
> and **which pair of functions is compared**. We ablate each dimension on a fixed corpus drawn from six
> real translators and ten real libraries, and quantify what each design decision costs in missed bugs
> and false alarms.

**Naming note (binding).** Paper axes are **OBS** (observation), **ATTR** (UB attribution), **ALIGN**
(alignment); the intro/motivation challenges are **CH-O / CH-U / CH-M** with the same long forms. The
following identifiers are *reserved* for repo use and must not appear in the paper: **E1/E2/E3** are the
repo evidence tables (E1 = `rq1` bug corpus, E2 = `rq2` matcher, E3 = `rq3` execution depth); **A1/A2**
are `results/ablations/attribution/ubgate_v1.md`'s 48-boundary sampling frame; **C1–C4** (and C5/C6/C7 below) are the §2 constraints
of this plan. None of those appear in the paper.

Contributions, in the order they should appear:

1. **An oracle-design ablation** over a fixed corpus of 20 confirmed defects and 16 certificates from
   six shipped translators — the first measurement of what each oracle dimension actually buys.
2. **Evidence that alignment is itself an error source.** A published FSE'26 C↔Rust map is 56% wrong
   (143/255); we measure the *downstream* consequence of trusting it (see constraint C4).
3. **A corpus of real, silent semantic divergences** in shipped translations — C terminates normally,
   Rust terminates normally, the observable state differs. Six recurring mechanisms behind 20 instances.
4. **A UB-attribution pipeline** that prevents source-C UB from being reported as translation defects,
   measured on real libraries as well as micro-benchmarks (see constraint C1).

**What we no longer claim:** "first differential fuzzing of C→Rust" (FLOURINE, RustAssure, ACToR,
TOUCHSTONE occupy that). "Translation failure is statically predictable" (withdrawn 2026-08-09; the
predicate was fitted to the bugs it then predicted). "The matcher is what makes the differential
possible" (retracted; 5 of 6 tools preserve names, and the matcher found 0 of the 20 bugs). The matcher
technique is standard BCSD — claim the *application and measurement*, never the technique.

---

## 2. Binding constraints

Six statements that came out of review and must not be re-broken. Each corrects something that was
asserted too strongly while planning.

### C1 — urlparser proves the *pipeline*, not the *in-loop gate*

The in-loop `--ub-free` gate instruments the C oracle with
`-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable`
(`tools/stu_selector/gen_diff_harness.py:816`, minimal runtime, flag-based). **ASan is not in it.**
`results/ablations/attribution/ubgate_v1.md:41` already records this honestly: `MEMORY_UB (tier-3) | 2 | out of in-loop
UBSan scope; post-hoc`.

urlparser's defect is `url_parse → get_part`: `sscanf` at `url.h:208` writes 10 bytes into a 1-byte
`malloc` — a **heap** overflow. `-fsanitize=bounds` covers compile-time-known array bounds, not this.
So the honest claim is:

> the full UB-aware classification pipeline (in-loop UBSan + post-hoc ASan replay + attribution)
> prevents a real library's C-side memory UB from being charged to the translator

and **not** "the gate rejects the input before Rust runs."

**Design consequence:** the ablation's UB-aware configuration runs the C side as an **ASan+UBSan
subprocess oracle** — non-zero exit discards the input. That is the semantically clean version and is
what the ablation should use. The alternative (keep the in-loop gate, name ASan replay as an explicit
second stage, and call the configuration *UB-aware pipeline* rather than *gate*) is acceptable but
weaker. **Chosen: subprocess oracle.**

### C2 — the shipped-test baseline is a porting project, not a day

E3's `theirs = 0` is definitional (`cargo check` executes nothing) and reads as a strawman on its own.
The fix is a third column — *translator acceptance evidence* / *upstream shipped tests* / *differential
fuzzing* — not demoting E3.

Assets exist (bzip2 self-tests, `lodepng_unittest.cpp`, cJSON cargo-test targets, lil script entry
points) but the suites target the original C ABI, a CLI, or a C++ wrapper, while the artifacts vary:
ABI reshape, missing executable, compile-only, crash-all, split or untranslated APIs.

**Porting protocol (binding):**
1. Attempt the original suite against every runnable translation.
2. Record three outcomes separately: *runs as-is* / *needs adapter* / *cannot run* — with the reason.
3. An adapter may perform **representation conversion only**. It may not patch translation logic.
4. Report reach fraction and per-function counts alongside any execution-evidence comparison.
5. **Never record "could not port" as "executed zero times."**

**Scope gate:** pilot on cJSON and bzip2 × {c2rust, CROWN} first. Only after that pilot do we estimate,
or commit to, the full matrix.

### C3 — one hour normalises the budget, not the evidence

Fixing a one-hour campaign per cell is Klees-compliant and is the right *compute* budget. It does **not**
make a 3,036-record certificate equal to a 50M-record one.

Every campaign row reports all six: **wall-clock budget · valid differential records · seeds · reached
functions/branches · divergences · time-to-first-divergence.**

Permitted: *"All campaigns received an equal one-hour budget."*
Forbidden: *"All certificates provide equivalent confidence."*

Klees compliance is also not just the hour — it is independent repetition and a reported distribution.
**Core ablation cells: 10 seeds. If cost forces it: 5 seeds, declared exploratory, with the
exploratory/confirmatory split stated explicitly in the text.**

Comparison wording is bounded too: we may say we execute far more than **the translators' own acceptance
criteria**. We may not say "more than prior work" — RustAssure is symbolic and has no comparable
execution count.

### C4 — 143/255 is a static error rate, not a downstream one

The PtrTrans map audit (143/255 wrong, 102 airtight scrambles, `lodepng_save_file → "load_file"`) is a
metadata defect rate. A wrong pair can go five different ways downstream: ABI-incompatible so the harness
fails to build, rejected by the adapter, compared anyway and producing a false divergence, causing a real
bug to be missed, or coincidentally agreeing and raising nothing.

The figure must therefore report **downstream consequences**: harnessable pairs · buildable pairs · true
bugs recovered · false divergences · missed bugs · abstentions. `lodepng_save_file → load_file` is the
motivating example, not the measurement.

**Corollary:** the audited map's own library is *not* measurable downstream, because lodepng × PtrTrans
does not build (241/255 units `Compile_Failed`, 363 module-assembly errors; `results/rq4_effectiveness/translation_matrix.md:46`,
fn 28). The 143/255 audit therefore stays static, in motivation only; the downstream table is computed
on compiling cells only (qsort × PtrTrans, cJSON × PtrTrans).

### C5 — shipped-map wording

Only 2 of the 6 systems ship an explicit correspondence map. SACTOR's emitted maps audit clean on the
bundled examples **and** on the study's qsort run (archived at
`tools/frameworks/sactor/tests/c_examples/qsort/result/translated_code_idiomatic/specs/function_name_map.json`:
`swap/partition/quickSort→quick_sort/main`; this is the run behind `results/rq4_effectiveness/reach_cells/qsort__sactor.json`). Of
SACTOR's two runnable study translations, qsort retains an archived map; the genann study archive does
not. PtrTrans's inferred map is the only non-trivial shipped map audited, and it is 56% wrong. The class
is established on the one case where it could be tested — it is **not a two-tool rate**.

Two earlier versions of this sentence were wrong and must not return: (a) "SACTOR did not translate
them" / "no SACTOR map exists for any study library" — false, the qsort map is archived; (b) a bare
"identity map" description of SACTOR's output — false, `quickSort→quick_sort` is a rename.

### C6 — observation channels are subsets

The observation configurations are four channels: **O-R** (return value only), **O-P** (process output:
stdout + exit status), **O-S** (boundary state: return + designated output memory + designated globals),
**O-F** (full union, O-S ∪ O-P — the oracle we ship). **O-P ⊆ O-F and O-S ⊆ O-F by definition.** There is
no configuration O-P sees and O-F misses, and no reverse-direction count against O-F may be written
("level 2 is not a subset of level 3", "not nested", "level-2-only witness" are all banned).

Why the earlier "level-2-only" idea was wrong: it proposed a checksum-table witness that only a
process-output oracle would catch. But optipng × Laertes `crc32("a")` returns the wrong value directly
(C `e8b7be43` vs Laertes `ff000000`, fn 8) — **O-R already sees it**; and bzip2 × Laertes returns
`BZ_OK` but writes the zero CRC table into the output buffer (fn 14) — **O-S sees the buffer**. Neither
is an O-P-only case. What the two-driver qsort pilot does show is that O-P's sensitivity depends on
whether the driver externalises internal state: *process-output oracles are driver-dependent*.

### C7 — alignment: three metrics, kept separate (from the ALIGN pilot, P5)

The qsort × PtrTrans pilot (`results/rq1_matching/align_qsort_ptrtrans/RESULT.md`) showed that "recall" alone
misdescribes the alignment cost. Report three separate quantities per alignment source:

- **correspondence recall** — name-eq 2/3, tool map 3/3, matcher 3/3, manual 3/3;
- **defective contract boundaries recovered** — name-eq 0/1 (quickSort↔quick_sort never proposed), all
  others 1/1;
- **unique underlying defects recovered** — name-eq 1/1, all others 1/1 (the name-preserved internal
  `partition` pair exposes the same defect, 30,480/50,000 divergences).

Required wording: *"Name equality misses the translated top-level API boundary at which the sorting
contract is expressed, although a name-preserved internal function still exposes the same underlying
defect."* Forbidden: "name equality misses the pair and therefore the confirmed defect." Zero false
divergences under every source. Matcher abstention (eps = 0.01) isolates `partition` (margin 0.018).
PtrTrans's tool-provided qsort map is correct. No buildable wrong-map witness exists — a witness requires a
PtrTrans re-run that keeps the trans-metadata jsonl (archived only for bzip2 / lodepng / qsort); until then
wrong-map downstream harm remains *potential*. C4's downstream columns are amended accordingly: "true bugs
recovered" splits into the last two metrics above.

---

## 3. The experiment — three ablation axes on one fixed corpus

**Corpus held fixed:** the 20 confirmed defects + 16 certificates from E1, plus urlparser as the
real-library UB row (currently a library-level `⊘` exclusion, so it is new harness work).

| axis | levels | what it measures |
|---|---|---|
| **OBS — observation** | four channels: **O-R** return value only · **O-P** process output (stdout + exit status) · **O-S** boundary state (return + designated output memory + designated globals) · **O-F** full union O-S ∪ O-P (the shipped oracle). O-P ⊆ O-F and O-S ⊆ O-F by definition (C6) | most of our defects are **state mutation, not return value**: qsort sorts in place (void return), `BZ2_crc32Table` / zlib `crc_table` / genann `lookup` are globals, cJSON `valuestring` is a struct field. O-R is structurally blind to them. Expected pattern: qsort — O-R misses, O-S finds; bzip2 × Laertes (fn 14) — O-R misses, O-P and O-S both find; tulip argc-off-by-one (fn 30) — O-P finds. **Process-output oracles are driver-dependent**: a silent-consumer driver leaves O-P blind to qsort, an array-printing driver lets O-P see it, O-S sees it under both |
| **ATTR — UB attribution** | none · in-loop UBSan gate · **isolated ASan+UBSan oracle** (subprocess, per C1) | false translation-bug reports caused by source-C UB. Real-library row = urlparser; the 13 micro-programs become *UB taxonomy coverage*, not the headline. **Witness set:** (i) urlparser — memory UB (`url.h:208` heap overflow), caught by the isolated oracle only, **no confirmed defect** (fns 12/22/27/31): proves *suppression*, not survival; (ii) lil `expr` × CROWN (fn 11) — 12 inputs trigger recoverable C-side UB (shift-out-of-range / INT_MIN-negate / signed-overflow), the in-loop gate suffices — the opposite direction; (iii) **C-unstable: no corpus witness observed.** The earlier lil `expr` order-dependence witness (fn 15) is **retracted** (`results/ablations/attribution/lil/RESULT.md`: `expr ((1+2)*(3+4))` is stable `[21]` under all orderings; the `[]`/`[21]` split was a lil.c version mismatch between CROWN's 2962-line source and the Laertes 3518-line source). Repeated C replay on the fixed corpus with randomised order is the planned check; if it stays 0, report 0. **Attribution requirement — reference version provenance:** the C oracle must be the exact source the tool translated, otherwise a faithful translation shows a false divergence (lil record 12 under mismatched versions); (iv) 27/27 mutation recall (`mutation_recall_v1.md`) — the retained-difference branch, with an oracle-independent validity denominator |
| **ALIGN — alignment** | name equality · tool-provided map · our matcher · manual ground truth | downstream cost of a wrong correspondence, per C4/C5; measured on compiling PtrTrans cells only |

O-F = return value + pointer-reachable out-params + globals written + stdout + exit status.

ATTR's level names matter: the middle level is what we actually shipped, the top level is what the
urlparser evidence supports. Do not collapse them, and never use "UB gate" as an umbrella term.

---

## 4. Order of work

| # | item | blocks | cost |
|---|---|---|---|
| 1 | **Freeze thesis; rewrite README / INDEX**, purge MTU / frontier / retired-RQ narrative (`README.md:3,13,22` still describe the pre-reset project) | nothing; do it first because it is currently misleading every reader | hours |
| 2 | **urlparser real-library UB ablation** — build the ASan+UBSan subprocess oracle, run ATTR at all three levels. **PILOT DONE (commit 747f5f0, `results/ablations/attribution/urlparser/`)**: (a) none and (b) in-loop gate both yield an unattributed crash candidate on the first ordinary URL (gate UB flag = 0, heap overflow out of scope); only (c) isolated ASan+UBSan excludes it as C-UB; raw candidates 1 / admissible 0 / confirmed translation divergences 0 in every config. lil part (`results/ablations/attribution/lil/`): reconstructed 313-record corpus, (b) and (c) exclude the same 37, fn 15 retracted | the strongest design claim rests on this | days |
| 3 | **Observation feasibility** — `qsort × PtrTrans` (compiles, 68% unsorted, fn 29) under **two drivers** (silent-consumer / array-printing): expect O-R blind under both, O-S sees under both, O-P blind → sees. Deliverable = a **per-channel detection table**, not a staircase. Explicitly: there is no reverse-direction check; O-P ⊆ O-F by definition (C6). **PILOT DONE (commit 747f5f0, `results/ablations/observation/obs_qsort_ptrtrans/`)**: single seed 42, 104 valid records, C ASan+UBSan clean 104/104 — O-R 0/0 · O-P 0 (silent driver) / 71 (printing driver) · O-S 71/71 · O-F 71/71; single-seed pilot, not a general rate | commits us to the full OBS sweep | 1–2 days |
| 4 | **ALIGN pilot** — `qsort × PtrTrans` (name-eq recall 0.67 vs matcher 1.00, `results/rq1_matching/matcher_master_table.md:107-113`; confirmed defect fn 29), scale-up `cJSON × PtrTrans` (fn 5). `lodepng × PtrTrans` **disqualified**: 241/255 `Compile_Failed`, 363 assembly errors (fn 28). The 143/255 audit stays static in motivation (per C4). **PILOT DONE (commit 747f5f0, `results/rq1_matching/align_qsort_ptrtrans/`)**: three pairs × 50,000 records, C gate clean; correspondence recall name-eq 2/3 vs 3/3 for tool map / matcher / manual; defective contract boundary recovered 0/1 (name-eq) vs 1/1; unique underlying defect recovered 1/1 for all sources (partition exposes it); zero false divergences; PtrTrans's qsort map is correct (see C7). **Wrong-map downstream witness requires a PtrTrans re-run**: trans metadata is archived only for bzip2 / lodepng / qsort, none for cJSON | makes E2 load-bearing | days |
| 5 | **Uniform one-hour re-run + multi-seed**, six-quantity reporting per C3; lodepng × {c2rust, CROWN} (the two certificate cells, fn 19) | the false-alarm column | machine time |
| 6 | **Shipped-test baseline pilot** — cJSON + bzip2 × {c2rust, CROWN}, protocol per C2; then scope the rest | E3's third column | unknown until pilot |
| 7 | **Taxonomy · threats to validity · selection protocol · RustAssure qualitative table · upstream disclosure** | disclosure is latency-bound — **start it in parallel, not at the end** | ~1 week total |
| 8 | **Pipeline fix** — `tools/stu_selector/gen_diff_harness.py --ub-free` needs `-fno-sanitize-link-runtime` on clang ≥ 21 (`libclang_rt.ubsan_minimal` already defines `__ubsan_handle_load_invalid_value_minimal`; duplicate-symbol link error against the shim; hit in both ATTR pilots) | any new `--ub-free` build on clang ≥ 21 | hours |
| 9 | **Pipeline fix** — the `fuzz/lil_{laertes,c2rust,wip}_e3` zeroed `_DefaultRuneLocale` rune shim is **wrong for differential use** (every char non-space → every record `[]` → 313/313 false divergences); acceptable for depth-only runs, must be replaced by the `rune_fill.rs` population for any lil differential cell | any lil differential re-run | hours |

### Notes on 7

- **Taxonomy:** `results/rq4_effectiveness/bugs_detailed.md:17` is a class index, not a taxonomy — it mixes cause and symptom
  on one axis (S1 tagged both `NULL/empty conflation` = cause and `zeroed-table` = symptom), and the
  "zeroed-table ×4 / 3 tools" group spans **three distinct mechanisms**. Rebuild as rows = mechanism,
  columns = detectability; symptom becomes a cross-cutting annotation. Pure re-coding, no runs.
- **Selection protocol:** `results/rq4_effectiveness/translation_matrix.md:169` reads *"Alternates if a row underperforms: …"*.
  That sentence will be quoted back at us. Freeze the protocol — tools, projects, function inclusion and
  exclusion rules, how failed artifacts are counted — **before** any new run, and state it was frozen
  post-hoc rather than pretending otherwise.
- **Bug-count deflation:** 20 instances ≈ 6 mechanisms by our own index. Present both numbers first,
  before a reviewer does it for us.
- **Baseline comparison:** RustAssure's source and its bug list are vendored at
  `tools/frameworks/rustassure/` (13 rows, 8 unique function sites; subjects Libcsv / u8c / optipng).
  `libcsv` is also one of the unused CROWN triples, so a shared subject is reachable. **Qualitative
  table only** — no head-to-head reproduction.

---

## 5. Explicitly not doing

- **Static-prediction experiments.** `scan_severed_init.py` stays a **prevalence census** — "the class
  we found dynamically in 3 tools exists at 139 library sites across 10 shipped Laertes translations" —
  never a predictor. (`results/rq4_effectiveness/severed_init_law.md` needs retitling: it is a census, not a law.)
- **Running a competitor head-to-head.** High cost, high risk, no public artifact.
- **More E3 cells.** E3's coverage is complete; its gap is the baseline (C2), not the matrix.
- **Claiming we out-test prior work.** Per C3, the comparison is against translator acceptance only.

---

## 6. Evidence for the fuzzing framing — keep in the paper

`genann × Laertes` (footnote 16, `results/rq4_effectiveness/translation_matrix.md`): `laertes_init_lookup` is uncalled and
**textually identical** to the fatal bzip2 CRC pattern, yet harmless, because
`genann_act_sigmoid_cached` retains a runtime lazy rebuild. A static predicate false-positives here;
only execution separates the two. Footnote 12 (urlparser × CROWN) is the mirror case: attribution
preventing a false positive.
