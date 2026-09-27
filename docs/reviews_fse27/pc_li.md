# PC profile — Shaohua Li (CUHK)

Reviewer-in-training profile, FSE 2027. Lens: program analysis, translation validation,
equivalence checking, undefined behaviour and semantics (compiler testing, Csmith/EMI-style
differential testing, refinement/equivalence, UB-aware reasoning).

Prepared 2026-09-14.

---

## (a) Who / affiliation / which PC list

**Shaohua Li**, Assistant Professor, Department of Computer Science and Engineering,
The Chinese University of Hong Kong (CUHK). PhD from ETH Zurich (AST Lab, advisor Zhendong
Su); BSc/MSc USTC. Group page: https://www.cse.cuhk.edu.hk/people/faculty/shaohuali/ ;
personal page: https://shao-hua-li.github.io/ ; group GitHub org `cuhk-s3`.

**PC list used: FSE 2027 Research Papers — the list IS public.**
Source: https://conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee
(note: the URL given in my instructions, `.../fse-2027-research-papers`, 404s; the live path
is `fse-2027-papers-program-committee`). Chairs: Michael Pradel (CISPA) and Baishakhi Ray
(Columbia); 19 area chairs; ~440 PC members. The roster contains the row
`Shaohua Li ~ The Chinese University of Hong Kong ~ Hong Kong SAR China`.
His conf.researchr profile (https://conf.researchr.org/profile/conf/shaohuali) independently
lists **ESEC/FSE 2027: Program Committee Member (Research Papers) and Proceedings Co-chair**.

Why he fits this lens rather than "he is famous": his entire output is compiler
correctness — UB program generation and sanitizer testing (UBfuzz/UBGen, ASPLOS'24,
Distinguished Artifact), differential testing across compilers to find *unstable code*
(ASPLOS'23 CompDiff), real-world-code-injection compiler fuzzing (Creal, PLDI'24,
Distinguished Artifact), and Alive2-backed translation validation in his 2026 work. He also
co-authored an empirical study of **rustc**-specific bugs (OOPSLA'25), so he is one of the
few PC members who will read a C→Rust translation-validation paper with both compiler-semantics
and Rust-toolchain priors.

Reviewing-role track record (observed, from conf.researchr): PLDI 2025 Review Committee;
OOPSLA/SPLASH 2026 Review Committee; ISSTA 2026 and ASE 2025/2026 PC; CGO 2027 PC;
ECOOP 2023 Extended Review Committee; **PLDI 2023 + 2024 Artifact Evaluation Committee**;
ECOOP 2023 AEC. Three AEC terms is a meaningful signal about how seriously he takes artifacts.

---

## (b) The three papers

I read all three from arXiv HTML in full (abstract → conclusion, including evaluation setup,
ablations, discussion and related work). None of the three was read from abstract alone.

### 1. Archer: Towards Agentic Review for Compiler Optimizations (2026)
Yunbo Ni, Shaohua Li. arXiv:2607.01808 (submitted 2 Jul 2026).
https://arxiv.org/abs/2607.01808 — artifact: https://github.com/cuhk-s3/Archer

Archer is an LLM-agent reviewer for LLVM middle-end optimization pull requests, built on two
constraints rather than on a bigger model: *dynamic obligation construction* (historical
correctness fixes are distilled by a three-stage LLM pipeline into pass-level "obligations",
each retained only if it can be re-materialised as an IR reproducer that **Alive2 confirms**
exposes a semantic mismatch), and a *deterministic validation guard* (no finding may be
reported unless it is reduced to an executable IR pair that (i) is judged by a compiler-aware
oracle — Alive2 `ProofCheck`, falling back to `TestCheck`, a differential execution under the
UB-aware interpreter LLUBI — and (ii) is **patch-triggering**, i.e. the discrepancy appears
under the post-patch compiler but not the pre-patch one). Evaluated on 398 real LLVM PRs
(70 open, 328 closed, Dec 2025–Feb 2026): 51 semantic bugs, 34 of them miscompilations, 33
already fixed and 11 confirmed, 3 "not planned" (a 6 % false-positive rate); plus a curated
47-case bisected regression benchmark for head-to-head comparison against Direct-LLM,
mini-SWE-agent, the directed fuzzer Optimuzz, and four commercial review tools (Codex,
Copilot, CodeRabbit, Greptile), with ablations on obligations, on the guard, and on two
alternative obligation-construction designs (`rag`, `all`). Cost is reported openly
($988.7 total, $2.5 and 877 s per PR), and the paper states its own **recall gap** on the
regression set as the first item of the discussion.

### 2. Understanding Agent-Based Patching of Compiler Missed Optimizations (2026)
Batu Guan, Zirui Wang, Shaohua Li. arXiv:2607.02370 (2 Jul 2026, v2 3 Jul 2026).
https://arxiv.org/abs/2607.02370 — artifact (anonymised ICSE submission):
https://anonymous.4open.science/r/icse-1919-artifact-62FE

An empirical study asking not "can an agent fix the reported case?" but "does the agent's
patch have the *scope* the developer intended?". The paper formalises an optimization's
scope as the set of programs on which the patch applies while preserving semantics, and
classifies an agent patch against the developer "golden" patch into ⊂ / partial-overlap /
⊃ / indistinguishable. Because the scopes cannot be enumerated, they are approximated from
two sides: withheld golden regression tests (does the agent cover intended cases?) and
fuzzing — `alive-mutate` mutation plus a WhiteFox-style LLM generator prompted with *both*
patches — where every candidate is first checked by **Alive2 for refinement validity** and
then costed with llvm-mca (does the agent optimize outside the intended scope?). Benchmark:
43 strictly validated LLVM missed-optimization issues; four models (GPT-5.5, DeepSeek-V4-Pro,
Qwen3.5-Plus, Kimi K2.5) under an open 50-step harness. Findings are mostly negative:
agents patch the initial case ~60–74 % of the time but only about half of those match the
golden scope, and an explicit "please generalise" instruction **does not help and often
hurts** (it converts scope mismatches into outright failures). Historical-knowledge
augmentation (RAG over indexed source→target IR pairs, and pass-level distillation from 869
prior PRs) gives consistent but modest, model-dependent gains, corroborated downstream by
optimization-hit counts on eight real projects (git, linux, ffmpeg, opencv, llama.cpp, z3,
uv, tree-sitter).

### 3. Interleaving Large Language Models for Compiler Testing (LegoFuzz) — OOPSLA 2025
Yunbo Ni, Shaohua Li (corresponding). PACMPL 9(OOPSLA2), doi 10.1145/3763079;
arXiv:2508.18955. https://arxiv.org/abs/2508.18955 — artifact:
https://github.com/cuhk-s3/LegoFuzz

LegoFuzz splits LLM-based compiler fuzzing into an offline phase (an LLM transforms
real-world functions from AnghaBench into 553,246 small, *validated*, UB-free, profiled
building blocks, for $394 of gpt-4o-mini) and an online phase (iterative synthesis that
wires blocks together via type-compatible call insertion and shared globals, with no further
LLM calls, producing programs of thousands to >15,000 lines). The validity discipline is the
whole point: only UB-free functions with UB-free input profiles are admitted, because
"compilers are only designed to correctly compile valid code" and invalid programs can only
find crashes, never miscompilations. Result: 66 GCC/LLVM bugs, 30 of them **miscompilations**
— a class that neither Fuzz4All nor WhiteFox had ever found in a C compiler — with 56 fixed;
7 GCC bugs pre-dating GCC-12 and one traced to a 2006 change. The evaluation includes a
null baseline (the database functions alone find **zero** bugs), database-size ablations
(½, ¼), cross-substitution of databases with the prior tool Creal in both directions,
three different LLMs, an iteration-count sweep that reports a *non-monotone* coverage curve
(coverage plateaus after ~100 iterations and GCC coverage declines), and generation-speed
numbers (193 s vs 12,121 s / 28,284 s / 13,997 s for 10,000 programs).

Supplementary calibration (read as abstracts only, and labelled as such): UBfuzz/UBGen
(ASPLOS'24, arXiv:2401.04538) — differential testing of *sanitizers* using deliberately
UB-bearing programs, with a "crash-site mapping" oracle invented specifically to separate
sanitizer bugs from optimization effects; and "An Empirical Study of Rust-Specific Bugs in
the rustc Compiler" (OOPSLA'25, arXiv:2503.23985) — 301 manually reviewed rustc issues,
concluding that existing rustc testing tools "struggle to detect non-crash errors".

---

## (c) Writing taste

**Contribution framing.** Always a named artifact (Archer, LegoFuzz, UBfuzz, Creal) plus a
mechanism with a name of its own (*dynamic obligation construction*, *deterministic validation
guard*, *real-world code-aligned prompting*, *iterative program synthesis*, *shadow statement
insertion*, *crash-site mapping*). The claim is never "we use an LLM/fuzzer for X"; it is
always "X fails for reason R, and here is the structural constraint that fixes R". Both 2026
papers open by explicitly distinguishing their problem from ordinary compiler testing or
ordinary bug fixing — he is allergic to a contribution that is only a re-targeting.

**Intro structure, near-identical across papers.** Domain stakes (one paragraph) → a
research question posed as a sentence in italics → **two enumerated challenges** with a
`➤ Challenge 1/2:` marker, each backed by a number pulled from the *baseline's own reported
data* (Fuzz4All "generates only 37.26 % valid C programs"; "average length 18 lines";
"neither Fuzz4All nor WhiteFox found any miscompilation bugs") → `➤ Our core idea` →
`➤ Our approach` → headline result → bulleted contributions (4 bullets, invariably) →
artifact URL in the introduction, not only at the end. He quotes practitioners verbatim when
they support the motivation (an LLVM maintainer twice in Archer: "lack of review capacity…"
and "There are few false positives, and the analysis is generally on-point").

**What counts as evidence.** Only things a machine can check. Across all three papers the
accepted oracles are: developer confirmation/fix of a reported bug in the real upstream
tracker; Alive2 refinement proofs; UB-aware interpretation (LLUBI); differential execution;
llvm-mca cost; line coverage. Natural-language reasoning is explicitly *not* evidence — the
entire Archer design exists because "LLMs can produce plausible but unfaithful explanations",
and the guard's contract is that a finding without an executable reproducer cannot be
reported at all. In the patching study the same instinct appears as the refusal to call a
test-passing patch correct.

**Quantification.** Heavy and specific, including numbers that are not flattering: dollar
cost ($988.7; $2.5/PR; $394), tokens (5.05 M/case), wall-clock (877 s/case; 193 s vs 13,997 s),
false-positive count (3/51 = 6 %), duplicates (8/66), database yield (53 % of AnghaBench
retained; 14.2 % discarded for invalid I/O). Exact commit ranges are given for compiler
versions under test (`9366940`→`eb26b66`, `b1560bd`→`029cb8a`) and exact model checkpoints
(`gpt-4o-mini-2024-07-18`, temperature 0.7, max_token 512).

**Baselines.** Plural, of different *kinds*, and never straw. LegoFuzz compares against LLM
fuzzers (Fuzz4All, WhiteFox), a non-LLM real-code injector (Creal), and — crucially — a
**null baseline** (the un-synthesised functions), and then performs a *cross-substitution*:
Creal's database inside LegoFuzz and LegoFuzz's database inside Creal, to separate the
contribution of the data from the contribution of the algorithm. Archer compares against
direct LLM, an open agent framework, a research directed fuzzer, and four commercial
products, and honestly reports why the research baseline underperforms (Optimuzz could not
even start on 23/47 cases because of CFG-construction requirements) rather than just
reporting the score.

**Negative results.** Reported as first-class findings, not buried. The 2026 patching study's
Finding 2 is that their own natural intervention fails ("generic generalization instructions
do not reliably improve alignment and may reduce success"); Archer's Discussion opens with
"Recall gap of Archer" before any future-work framing; LegoFuzz reports that coverage
*declines* past 100 iterations. He also concedes a limitation of a tool he depends on
(Alive2 "incorrectly reports the transformation as correct" on a loop case) instead of hiding
it — and turns that concession into the argument for his own complementary oracle.

**Threats to validity.** *He does not write a section called "Threats to Validity."* In the
two 2026 papers the equivalent material lives in a `Discussion` section as bulleted `➤`
items: pretraining-data leakage, stochasticity of agentic runs, granularity choices,
alternative sources, generalisation beyond C. The patching study additionally puts its main
construct-validity caveat *in the problem definition itself*: "We acknowledge that this
proxy-based assessment cannot guarantee perfect enumeration… our classification is an
empirical characterization of observed equivalence or divergence, **not a formal proof of
intent**." Data-leakage control is handled by construction (historical cases overlapping the
benchmark are excluded before building obligations / the RAG index) and then *stated*.

**Tone.** Bold headline, hedged mechanism. He will write "The results are striking",
"The review results are shocking and concerning", "demonstrates its strong bug-finding
capability" — but every such sentence sits next to a table with a denominator, and the
technical claims are carefully scoped ("suggests", "we approximate", "lower-bound
approximation", "likely ≡", "not a formal proof"). He never claims soundness or completeness
he has not earned.

**RQs.** Always exactly four, one sentence each, with a parenthetical tag, and in a fixed
rhythm: RQ1 = does it work on the real world, RQ2 = how does it compare / how effective on a
controlled benchmark, RQ3 = ablation of the components, RQ4 = case study. Findings in the
empirical paper are boxed as numbered `Finding N` one-liners.

**Running examples.** Yes, and always a *real* one with a URL: Archer opens on LLVM PR
#183329 and closes on the `icmp samesign` poison case; the patching study runs LLVM issue
#158326 (masked equality implies a range check) end-to-end through direct patch → generic
instruction → RAG. The same example is threaded through motivation, design, and case study.

**Tables/figures.** Small, countable, boringly honest: status-of-reported-bugs tables
(Confirmed / Fixed / Duplicate / Not Planned), symptom tables (Crash vs Miscompilation),
affected-component tables, coverage bar charts, "stable versions affected" histograms, tool-call
distributions, and transition (before/after) plots. Code figures are always *reduced* IR or C
with the semantically relevant line annotated. He prefers a table of raw counts to any
composite score, and never reports a single aggregate number where a breakdown exists.

**Artifacts.** Every paper ships one, linked from the intro and from a Data-Availability
Statement; even the anonymised ICSE submission has a working anonymous.4open.science link.
He has served on PLDI/ECOOP artifact-evaluation committees three times.

---

## (d) Likely reviewing standards — checklist for a paper in this lens

Items 1–12 are what I expect him to demand of a translation-validation / differential-testing
/ UB-aware paper. Marked **(O)** where directly observed in his own papers' standards, **(I)**
where inferred.

1. **Define your oracle formally and name what it cannot decide. (O)** He builds papers
   around oracle design (crash-site mapping; ProofCheck+TestCheck; scope classification) and
   always states the incompleteness (Alive2 timeouts, unsupported intrinsics, "not a formal
   proof of intent"). A paper that says "we compare outputs" without saying what
   *comparison completeness* means will be pushed hard.
2. **Input validity must be enforced, not assumed. (O)** LegoFuzz's core argument is that
   UB-bearing inputs can only find crashes, never miscompilations, and each building block is
   validated with randomised inputs and profiled to keep only UB-free ones. For a C→Rust
   paper he will ask: exactly how do you establish the C side is UB-free, with which
   sanitizers at which optimisation level, and what fraction of your reported differences
   survive that filter?
3. **Separate "the tool reported it" from "it is a defect", with a machine-checkable
   adjudicator. (O)** Both 2026 papers spend more design effort on admission control than on
   generation. Expect him to ask for the adjudication procedure, its false-positive rate, and
   at least one worked misadjudication.
4. **Show the difference is attributable to the thing under study. (O)** Archer's
   *patch-triggering* requirement — a discrepancy counts only if it appears post-patch and not
   pre-patch — is exactly the attribution discipline he will look for. In a translation paper:
   prove the divergence is caused by the translation and not by the harness, the input model,
   or a latent bug on both sides.
5. **A null / degenerate baseline. (O)** "Functions alone found no bugs." He will want the
   result of the obvious cheap thing (fuzz the Rust side alone; run the shipped test suite;
   compare with no matcher) and treat its absence as a gap.
6. **Ablate every named component, and ablate the *data* separately from the *algorithm*. (O)**
   The Creal↔LegoFuzz database cross-substitution is a distinctive move; the obligation
   `base`/`rag`/`all` comparison is another. If a paper's gain could come from a corpus rather
   than a technique, he will notice.
7. **Report cost, throughput and budget honestly — and for LLM components, exact model
   checkpoints and prices. (O)** Dollars, tokens, seconds, timeouts, memory caps, commit
   hashes. A "24-hour fuzzing campaign" with no per-configuration budget parity will draw a
   question.
8. **Bug claims must be upstream-grounded. (O)** He reports Confirmed / Fixed / Duplicate /
   Not Planned as separate rows and counts duplicates against himself. Self-assessed
   "bugs we believe are real" will not carry a headline; he will ask how many were reported,
   confirmed, fixed, and how many were duplicates.
9. **Counts must have denominators and a de-duplication story. (I, strongly supported)**
   Every table gives the total; long-latency claims are backed by version-bisection histograms.
   Expect "N clusters ≠ N bugs" scrutiny and a demand for a reduction step (he uses C-Reduce)
   plus root-cause-level grouping.
10. **Negative and non-monotone results must be reported, not smoothed. (O)** He publishes
    "our instruction made it worse" and "coverage declines after 100 iterations". A paper
    where every knob monotonically helps will read as under-reported to him.
11. **An artifact that actually runs, with data for every table. (O)** Three AEC terms,
    two Distinguished Artifact Awards, artifact URLs in the introduction, and a
    Data-Availability Statement promising "all source code and data for reproducing the
    experimental results".
12. **Generality claimed only as far as it was tested. (O/I)** He writes "this paper provides
    the first proof-of-concept implementation of this new paradigm on testing C compilers" and
    flags Rust as *future* work; he says gains are "model-dependent" when they are.
    Over-claimed generalisation from one or two subjects will be cut down.

---

## (e) Red flags that would make him reject

- **Textual / LLM-judged evidence presented as ground truth.** The single most consistent
  value in his 2026 work: "such textual comments are often insufficient… LLMs can produce
  plausible but unfaithful explanations". An LLM-as-judge oracle with no executable
  cross-check is close to an automatic reject in this lens. (O)
- **Test-passing treated as correctness.** The whole 2026 patching study exists to attack
  this equation, and it cites the overfitting-in-APR literature to do so. (O)
- **Differences reported on inputs that may be UB / out of contract.** He will read a
  "semantic divergence" found on a program with undefined behaviour as an artefact, not a
  finding. (O)
- **A single baseline, or a baseline reimplemented by the authors instead of run as released.**
  He runs competitors as shipped and reports their failures mechanically (Optimuzz failing to
  start on 23/47 cases) rather than quietly dropping them. (O/I)
- **Bug counts with no confirmation status, no duplicates row, and no denominator.** (O)
- **Monotone, all-positive ablations with no cost reported.** (I)
- **"Our approach is general" supported by one subject program or one language.** (O)
- **Missing or non-functional artifact, or tables whose data is not in the artifact.** (O)
- **Novelty framed as "first to apply X to Y."** In all three papers the contribution is a
  mechanism that survives ablation, not a new application domain; both 2026 papers open by
  arguing why the *problem* is different, not why the *setting* is new. (I, strongly supported)
- **Hidden-prompt commercial agents used as the experimental platform.** He explicitly
  justifies using "an open and controllable agent harness rather than productized coding
  agents with hidden prompts, tools, policies, or updates". Reproducibility of the pipeline
  is a hard requirement, not a nicety. (O)

## (f) What would delight him

- **An admission gate**: a paper where a candidate finding cannot be reported unless it is
  reduced to a machine-checkable artifact, and where the gate's own false-positive rate is
  measured. This is Archer's thesis, and it is also the shape of a UB-gated differential
  oracle.
- **Two complementary oracles that cover each other's incompleteness**, with a concrete case
  where one fails and the other succeeds — his `samesign` case study, where Alive2 says
  "correct" and the UB-aware interpreter catches branch-on-poison, is precisely this, and he
  presents it as the most valuable result in the paper.
- **A null baseline that finds nothing**, making the contribution's necessity undeniable.
- **A negative finding stated as a Finding**, especially one that kills the authors' own
  first idea.
- **Cross-substitution ablations** that separate corpus from method.
- **Upstream impact**: bugs reported, confirmed and fixed by real maintainers, with issue
  URLs, and a long-latency analysis showing prior techniques missed them for years.
- **Rust-toolchain literacy.** Via the rustc bug study he already believes existing Rust
  testing tools "struggle to detect non-crash errors" — a paper that finds *silent, non-crash*
  semantic differences in Rust code lands directly on a gap he has documented.
- **An honestly stated recall gap** with a plan, rather than a claim of completeness.
- **A working artifact with a reduced, readable reproducer per finding.**

## (g) Evidence vs inference disclosure

**Observed (read directly, in full, this session):**
- FSE 2027 Research Papers PC roster (live page, parsed; 461 entries incl. chairs and area
  chairs) containing his name; his conf.researchr profile listing FSE 2027 PC + Proceedings
  Co-chair and his prior PC/RC/AEC service.
- arXiv:2607.01808 (Archer) — full text, abstract through conclusion and references.
- arXiv:2607.02370 (Agent-Based Patching) — full text, abstract through conclusion.
- arXiv:2508.18955 (LegoFuzz, OOPSLA'25) — full text, abstract through related work and
  Data-Availability Statement.
- His homepage publication list and CUHK faculty page (bio, awards).
- **Abstracts only** (explicitly not full text): UBfuzz (arXiv:2401.04538) and the rustc
  empirical study (arXiv:2503.23985). Used only for lens calibration, and marked as such above.

**Not found / not available:**
- No public "how I review" post, blog, OpenReview record, PC-chair notes, or talk on
  reviewing could be located for him. Section (d) and (e) are therefore reconstructed from the
  standards his own papers hold themselves to and from his artifact-evaluation service — not
  from any statement he has made about reviewing. Every item is tagged (O) or (I) accordingly.
- I did not fetch the published ACM DL versions (paywall); the OOPSLA'25 text read was the
  arXiv v1, which carries the PACMPL DOI 10.1145/3763079 and the camera-ready front matter.
- The "51 bugs" / "66 bugs" figures are as reported by the authors; I did not verify them
  against the LLVM/GCC trackers.

**Principal inferences (flagged as such):**
- That he would apply an Archer-style *patch-triggering* attribution requirement to a
  translation-validation paper is an extrapolation from a compiler-PR setting to a
  translated-program setting. The underlying value (a difference must be attributable to the
  change under study) is observed; the transfer is mine.
- The "novelty as mechanism, not application" red flag is inferred from the consistent
  rhetorical structure of three papers, not from any stated reviewing criterion.
- Claim strength of the checklist items rises with the (O) tags; the (I) items are where a
  second reviewer might reasonably disagree with me.
