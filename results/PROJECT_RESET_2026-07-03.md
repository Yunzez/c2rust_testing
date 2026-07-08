# PROJECT RESET — north star after the 2026-07-03 advisor sync

**Why this doc exists.** The advisor's read: we moved too fast and drifted — we hunted *crashes*
(findable by fuzzing Rust alone) and built *frontier-selection theory*, and neither is the thing that
justifies this project. This doc re-anchors the project and **replaces** the earlier divergent plans
(`sactor_min_plan.md`, `rq1_coverage_scope_plan.md` frontier framing). Read this first after compact.

---

## 1. The one idea that justifies the project

> **A semantic difference — C outputs A, Rust outputs B, both terminate normally — is the ONLY bug
> class that fuzzing the Rust alone cannot find.** You need the C as an oracle to know the Rust is
> wrong when it doesn't crash. Crashes and panics? Fuzzing Rust finds those with no C at all.

Without semantic-difference bugs, this project reduces to "fuzz the Rust," which is a solved problem.
**Finding semantic differences is the raison d'être. Everything else is in service of it.**

### Honest audit of what we found so far
All 5 confirmed C2SaferRust bugs are **crashes** (class #3), every one findable by fuzzing Rust alone:

| bug | class | needs the C oracle? |
|---|---|---|
| qsort (hang / OOB) | crash | no — Rust hangs/OOBs by itself |
| url_is_ssh, bzip2 endsInBz2, optipng -dir, lil do_system | crash (UTF-8 panic) | no — Rust panics by itself |
| **— (semantic difference) —** | **#1** | **yes — and we have ZERO so far** |

So the drift is real. The 5 bugs are a fine *warm-up / robustness* result, but the project's core
claim is unproven until we find semantic differences. **This is now Priority 0.**

---

## 2. Bug taxonomy (the classifier, not a filter)

The UB analysis is no longer a filter that discards inputs — it is a **classifier** that sorts every
divergence into one of three buckets:

1. **Semantic difference (transpiler bug #1)** — C is UB-free, both sides terminate, `output(C) ≠
   output(Rust)`. **The crux. Top priority.** Only findable differentially.
2. **Inherent UB (not the transpiler's fault, but still reported)** — the input triggers UB in C
   (UBSan/ASan fires); Rust may crash/differ. We *report and label* these (a real divergence) but
   attribute them to C's UB, not the translation.
3. **Crash (transpiler bug #2)** — C is UB-free and returns normally; Rust panics/crashes **or hangs**
   (Rust non-termination on a C-terminating input goes here too, detected by timeout — that's the qsort
   bug). (Our UTF-8 class lives here.) A real transpiler bug, but *also* findable by fuzzing Rust alone.

Report all three; the value story rests on #1.

**Nuance — don't over-demote the crashes:** fuzzing Rust alone finds the *panic*, but it cannot
*confirm a transpiler bug* — it doesn't know whether the input is C-UB-free and whether C returns
normally on it (the panic could be a precondition violation the C caller never makes). The C oracle +
UB gate is what turned our 5 panics into **confirmed, attributed** transpiler bugs. So: **discovery**
of class #3 doesn't need us; **attribution** does. The gap is that class #1 needs us for both.

## 2b. A SECOND axis of defect — mapping bugs (the tool's own bookkeeping)

Classes #1–#3 above are properties of the **translated program's runtime behavior**. There is a
distinct, orthogonal axis: defects in the **tool's own output metadata** — specifically the C↔Rust
**function-correspondence map** that a tool ships. This is NOT a runtime bug (the translated code can
be perfectly correct while the map lies), so it does not belong in the semantic/crash taxonomy — it is
a different kind of finding on a different axis. Frame as two layers, NOT a flat three-class list:
**(layer 1) program-behavior bugs found by differential testing = semantic + crash/hang; (layer 2)
tool-metadata bugs found by our independent name-independent matcher = mapping.**

**Why only we can find it:** every other verifier (symbolic, formal, name-based differential) *trusts*
the tool's claimed correspondence to set up its check. You cannot audit a tool's mapping using the
tool's own mapping — that is circular. An **independent structural matcher** (name-blind) produces an
alignment that does not come from the tool, so a disagreement between the tool's claimed map and the
structural map is itself the finding. This is the sharpest answer to "why build a matcher when tools
ship maps."

**Audit result (2026-07-08) — we audited BOTH tools that ship an explicit map:**
- **SACTOR — CLEAN.** `function_name_map.json` on every example SACTOR actually produced
  (atoi/fft/hamming/course_manage/cmake_multi): no dangling targets, complete (extra Rust decomposition
  helpers correctly excluded), renames correct (`printUsage→print_usage`). Reason it's trustworthy: the
  map is a **deterministic record of SACTOR's own renaming**, not an inferred alignment. (SACTOR
  produced no map on the real E1 libs — it failed to translate them.)
- **PtrTrans — BROKEN.** `lodepng_Trans_PA_trans_metadata.jsonl`, 255 function-level records:
  **143/255 (56%) have `rust_definition_name` ≠ the C name; only 101 (40%) self-consistent; 102/255
  (40%) are AIRTIGHT scrambles** — the claimed Rust target name is itself a *distinct C function*.
  Smoking guns: `lodepng_save_file → "load_file"` (save mapped to its semantic opposite),
  `set32bitInt → "alloc_string"`, `read32bitInt → "memcpy"`, `update_adler32 → "deflate"`. These can't
  be renames (the name already belongs to a different function). Reason it's broken: the map is an
  **inferred KG alignment**, a separate fallible step. **Correction to earlier notes:** the old
  "143/244 off-by-one" phrasing is imprecise — it's 143/255 mismatches and it is NOT off-by-one (only
  18 are strict neighbor shifts); it is broad scrambling (102 airtight). Reproduce:
  `scratchpad/sactor_audit.py` logic + the metadata jsonl; source at
  `tools/frameworks/ptrtrans_rebuild/.../PA_trans_projects/lodepng_Trans_PA_trans_metadata.jsonl`.

**Impact + the claim we make:** a differential test built on PtrTrans's shipped map would compare ~40%
*wrong function pairs* → spurious divergences (false bugs) and missed real bugs, invisibly. Our claim:
*mapping correctness is tool-specific and not guaranteed a priori (deterministic-record maps are sound,
inferred maps can be badly wrong); an independent name-independent matcher is the only way to tell them
apart, and it caught a 40%-scrambled map that every downstream consumer would have trusted.* One
confirmed instance (PtrTrans) is enough to establish the class and to justify the matcher's audit role.
Do NOT overclaim "mapping bugs are pervasive" — we audited 2 map-emitting tools, found 1 sound + 1
broken.

---

## 3. Novelty — RE-ANCHORED (frontier selection is dead)

Frontier selection is retired as a contribution: **now that the UB gate is just a classifier, we fuzz
every boundary the matcher can align** — there is nothing to "select." (It survives, at most, as a
minor byproduct / coverage optimization.)

**The real novelty:** classical differential testing assumes you *know the two sides' correspondence*
(same source, same names — compilers, EMI). **LLM C→Rust translators rename, restructure, and
idiomatize, so you do NOT know which Rust function is the C `quickSort`, nor its signature.** Without a
name-independent **matcher** + **automatic harness generation across arbitrary/idiomatic signatures**,
differential testing simply cannot be applied to LLM translations at all.

> **DUET's contribution = making differential testing possible for translators that don't preserve
> structure, and using it to find semantic-difference bugs that single-program fuzzing cannot.**

The matcher is not merely an eval axis; it is the *enabler*. This is more honest and stronger than
frontier selection ever was.

**Anticipated attack + answer:** *"the translator itself knows the C↔Rust mapping — why a matcher?"*
Three answers: (a) **published artifacts ship code, not mappings** — C2SaferRust's translated repos
give you final Rust, no per-function correspondence log; (b) **idiomatic tools merge/split/inline
functions**, so even a tool-emitted 1-1 log wouldn't survive its own restructuring; (c) the real use
case is **third-party auditing** — validating a translation you didn't produce (a team that translated
with ChatGPT last year has no logs at all). Differential testing as an *independent check* cannot
depend on the translator's cooperation.

---

## 4. The UB-attribution decision — KILL caller-climbing

We considered handling "the input causes UB in C" by *testing one function up the call graph* (where a
caller might sanitize the input). **Drop this — it is the part a reviewer breaks** (the sanitization
may be several frames up; one level is arbitrary).

**Correct stance instead:**
- **Soundness of attribution** = **UBSan + ASan gating at the function level**: only report divergences
  on inputs where **C is UBSan/ASan-clean**. Semantics-preservation is defined over *all* well-defined
  (UB-free) inputs, not "the inputs a caller happens to pass" — so input *realism* is a red herring.
- **Honest residual limitation** = UBSan/ASan are *incomplete* (don't catch all UB). This is the
  **same** limitation as every dynamic-UB differential-testing line (Csmith, EMI) — citable, bounded,
  not fatal. Our **value-oriented scope** (scalars / buffers / NUL-strings / POD) is precisely where
  ASan+UBSan cover the preconditions best (OOB, overflow, shift), so residual risk is minimized.
- **Bonus alignment (evidence):** SACTOR *itself* filters its generated test inputs through **valgrind**
  (UB-free) — i.e., it defines its own correctness domain as UB-free inputs. We test the *same* domain
  it claims correctness over, just densely. That kills the "unfair/unrealistic input" objection head-on.

---

## 5. Evaluation — THREE directions (replacing the 5 divergent RQs)

| # | Direction | Question | What we have | What's missing |
|---|---|---|---|---|
| **E1** | **Bug finding** (all 3 classes; **semantic diff = the headline**) | Do we find real bugs — especially the semantic differences only differential testing can find? | 5 crash bugs; soundness census (0 FP / 126); **mutation recall 27/27 = we CAN catch injected semantic diffs** | **real semantic-diff bugs (currently 0)** |
| **E2** | **Matcher** | Fair name-independent matching across tools that don't preserve names | matcher (bignum 92%, stable under rename); topology fix designed | run on the **union of CRUST-bench programs that ALL name-changing tools can translate**; **c2rust = baseline** (it keeps names) |
| **E3** | **Coverage** | Do we bring *higher* coverage than what ships? | OOP harness generator; coverage census tool | **per-program: how many functions can we fuzz? coverage after 24h fuzzing? vs the program's shipped unit tests** (if none, count as 0). *If we're below their own tests, we're a joke.* |

Dropped: RQ4 head-to-head (FLOURINE/RustAssure), RQ5 frontier selection (retired).

---

## 6. Where semantic-diff bugs live — the hunt plan

**Hypothesis:** semantic diffs come from translators that *re-implement logic* (LLM), on inputs their
verification never saw. Confirmed leverage: SACTOR verifies against **5–12 LLM-picked, valgrind-filtered
samples** (hamming: 6 samples that vary only a *size scalar* over fixed data) — the entire input space
beyond that is unverified.

**Priority order:**
1. **Raw-LLM translations first (fast, high hit-rate, self-generated).** Prompt frontier models
   (gpt-5.1 / Claude / others) to translate value C functions to Rust with **no test-guided repair** →
   dense bugs, even on plausible inputs. Goal: **land 1–2 real semantic diffs to prove the raison
   d'être**, then move on. Framing caveat: raw-LLM output is *not a published tool* → use as scale-up,
   not the headline.
2. **SACTOR value functions** (published; bugs sparse, in the untested edge space → needs deep/24h
   fuzzing on adversarial inputs).
3. **C2SaferRust arithmetic/index rewrites** (published; tends toward crashes, but int→usize etc. can
   also yield silent wrong output).

**Method for all:** value-comparing harness (not just crash), UBSan/ASan gate on C, output-diff on
UB-free inputs, long campaigns.

---

## 7. Tool scarcity

We have only SACTOR + C2SaferRust (LLM/lifter) + c2rust (mechanical baseline). Not enough for a
credible tool axis. Two-pronged fix:
- **Headline:** find 1–2 more *published* tools — retry RustMap (memory: "unrunnable"), Laertes, or a
  recent LLM transpiler (Flourine / Vert / Syzygy-class). Reviewers want "published."
- **Scale:** self-generate translations with frontier LLMs (each model = a translator). Realistic
  practice (people translate with ChatGPT) and the richest semantic-diff mine — but labeled as
  supplementary, not the published-tool eval.

---

## 8. Asset map — keep vs drift

**Keep (still core):**
- OOP differential harness generator + the 4 soundness fixes + determinism gate → the engine.
- Soundness census (0 FP / 126) → E1 precision foundation; makes reported diffs credible.
- Mutation recall (27/27) → **now doubly important: proof the engine detects injected semantic diffs.**
- Matcher (bignum 92%, rename-stable) → E2, the *enabler-novelty*.
- Coverage census tool → E3.
- The 5 crash bugs + SACTOR cross-tool round → robustness / cross-tool evidence (supporting, not
  headline).

**Drift (retire or demote):**
- Frontier / STU selection as a contribution → retired (byproduct at most).
- Caller-climbing for UB attribution → killed (replaced by UBSan/ASan function-level gate).
- RQ4 head-to-head → out of scope for now.
- Chasing more crash bugs → stop; crashes don't need us.

---

## 9. First moves after compact (in order)

1. **Semantic-diff hunt, raw-LLM:** pick ~5 value C functions with non-trivial logic (codec / numeric /
   bit-op / string-scan / bounds), get raw-LLM Rust translations (no repair), value-diff fuzz vs C
   (UBSan-gated). **Target: 1–2 confirmed semantic diffs → project raison d'être nailed.**
2. Turn the UB gate into the 3-class **classifier** in the harness output.
3. E3 coverage: run the census + a 24h campaign on ≥1 program; get the "vs shipped unit tests" number.
4. E2 matcher: assemble the multi-tool CRUST-bench union; c2rust baseline.

## 10. Still-open questions (decide as we go)
- **E2 ground truth:** matcher accuracy on real LLM outputs needs a labeled C↔Rust correspondence —
  who labels it? (Manual on the CRUST-bench union? Semi-auto via our own harness agreement?) Rename
  experiments have free ground truth; real SACTOR/C2SaferRust outputs don't. Decide the protocol.
- Name: **DUET** (proposed, not locked) vs PARITY / TANDEM.
- Exact published-tool set for E2/E3 (which 1–2 to add).
- How to present raw-LLM findings honestly alongside published-tool findings.
- Venue (FSE) + deadline → back-plan the semantic-diff + coverage experiments from it.
