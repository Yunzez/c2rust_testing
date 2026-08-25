# FSE plan — consolidated

Canonical plan. Supersedes `FSE_PLAN_2026-08-09.md` (which is kept for the framing argument in its §1)
and `PROJECT_RESET_2026-07-03.md`.

**Revision history**
- **2026-08-09 (v1)** — framing decision: this is a *fuzzing* paper, static-prediction thesis withdrawn.
  Oracle ablation named as the core experiment. Certificate normalisation named as the blocker.
- **2026-08-25 (v2)** — two rounds of external review merged. Added the alignment ablation axis, the
  real-library UB gap, and four **binding constraints** (§2) that correct over-claims made while
  planning. Nothing below has been run yet.

---

## 1. Frozen thesis and contribution list

**Do not restate the contribution in any other words than these until the paper is drafted.**

> The reliability of C→Rust differential validation turns on three oracle-design dimensions that prior
> work leaves implicit: **what state is observed**, **how source-C undefined behaviour is attributed**,
> and **which pair of functions is compared**. We ablate each dimension on a fixed corpus drawn from six
> real translators and ten real libraries, and quantify what each design decision costs in missed bugs
> and false alarms.

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

Four statements that came out of review and must not be re-broken. Each corrects something that was
asserted too strongly while planning.

### C1 — urlparser proves the *pipeline*, not the *in-loop gate*

The in-loop `--ub-free` gate instruments the C oracle with
`-fsanitize=signed-integer-overflow,shift,integer-divide-by-zero,bounds,null,unreachable`
(`tools/stu_selector/gen_diff_harness.py:816`, minimal runtime, flag-based). **ASan is not in it.**
`results/rq2_ubgate_v1.md:41` already records this honestly: `MEMORY_UB (tier-3) | 2 | out of in-loop
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

---

## 3. The experiment — three ablation axes on one fixed corpus

**Corpus held fixed:** the 20 confirmed defects + 16 certificates from E1, plus urlparser as the
real-library UB row (currently a library-level `⊘` exclusion, so it is new harness work).

| axis | levels | what it measures |
|---|---|---|
| **A — observation** | return value only · whole-program stdout+exit · full observable state | most of our defects are **state mutation, not return value**: qsort sorts in place (void return), `BZ2_crc32Table` / zlib `crc_table` / genann `lookup` are globals, cJSON `valuestring` is a struct field. A return-value oracle is structurally blind to them |
| **B — UB attribution** | none · in-loop UBSan gate · **UB-aware pipeline** (ASan+UBSan subprocess, per C1) | false translation-bug reports caused by source-C UB. Real-library row = urlparser; the 13 micro-programs become *UB taxonomy coverage*, not the headline |
| **C — alignment** | name equality · tool-provided map · our matcher · manual ground truth | downstream cost of a wrong correspondence, per C4 |

Full observable state = return value + pointer-reachable out-params + globals written + stdout.

Axis B's level names matter: the middle level is what we actually shipped, the top level is what the
urlparser evidence supports. Do not collapse them.

---

## 4. Order of work

| # | item | blocks | cost |
|---|---|---|---|
| 1 | **Freeze thesis; rewrite README / INDEX**, purge MTU / frontier / retired-RQ narrative (`README.md:3,13,22` still describe the pre-reset project) | nothing; do it first because it is currently misleading every reader | hours |
| 2 | **urlparser real-library UB ablation** — build the ASan+UBSan subprocess oracle, run axis B at all three levels | the strongest design claim rests on this | days |
| 3 | **Observation-ablation feasibility** — axis A level 1 against `qsort × C2SaferRust` (void return) and `crc32_z × C2SaferRust` (mutates running state). Both must be missed | commits us to the full axis-A sweep | 1–2 days |
| 4 | **PtrTrans alignment pilot** — axis C, downstream outcomes per C4 | makes E2 load-bearing | days |
| 5 | **Uniform one-hour re-run + multi-seed**, six-quantity reporting per C3; lodepng ×2 mandatory | the false-alarm column | machine time |
| 6 | **Shipped-test baseline pilot** — cJSON + bzip2 × {c2rust, CROWN}, protocol per C2; then scope the rest | E3's third column | unknown until pilot |
| 7 | **Taxonomy · threats to validity · selection protocol · RustAssure qualitative table · upstream disclosure** | disclosure is latency-bound — **start it in parallel, not at the end** | ~1 week total |

### Notes on 7

- **Taxonomy:** `rq1_bugs_detailed.md:17` is a class index, not a taxonomy — it mixes cause and symptom
  on one axis (S1 tagged both `NULL/empty conflation` = cause and `zeroed-table` = symptom), and the
  "zeroed-table ×4 / 3 tools" group spans **three distinct mechanisms**. Rebuild as rows = mechanism,
  columns = detectability; symptom becomes a cross-cutting annotation. Pure re-coding, no runs.
- **Selection protocol:** `rq1_master_table.md:169` reads *"Alternates if a row underperforms: …"*.
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
  never a predictor. (`results/severed_init_law.md` needs retitling: it is a census, not a law.)
- **Running a competitor head-to-head.** High cost, high risk, no public artifact.
- **More E3 cells.** E3's coverage is complete; its gap is the baseline (C2), not the matrix.
- **Claiming we out-test prior work.** Per C3, the comparison is against translator acceptance only.

---

## 6. Evidence for the fuzzing framing — keep in the paper

`genann × Laertes` (footnote 16, `rq1_master_table.md`): `laertes_init_lookup` is uncalled and
**textually identical** to the fatal bzip2 CRC pattern, yet harmless, because
`genann_act_sigmoid_cached` retains a runtime lazy rebuild. A static predicate false-positives here;
only execution separates the two. Footnote 12 (urlparser × CROWN) is the mirror case: attribution
preventing a false positive.
