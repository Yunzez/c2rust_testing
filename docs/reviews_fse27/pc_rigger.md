# PC Profile: Manuel Rigger (FSE 2027 Research Papers PC)

Reviewer-in-training lens: **fuzzing and dynamic testing** (coverage-guided fuzzing, differential
testing, test oracles, sanitizers).
Compiled 2026-09-14. All web sources fetched live; see §(g) for the evidence/inference split.

---

## (a) Who / affiliation / which PC list

- **Name:** Manuel Rigger
- **Affiliation:** Assistant Professor, School of Computing, **National University of Singapore**;
  leads the **TEST Lab** (Trustworthy Engineering of Software Technologies) in the PL/SE group.
- **Background:** PhD at JKU Linz under Hanspeter Mössenböck (Sulong / GraalVM); postdoc at ETH
  Zurich with Zhendong Su. Author of the **SQLancer** line of work (PQS, NoREC, TLP) and the
  associated tool; claims 1,000+ previously unknown bugs found across systems.
- **PC list used:** **FSE 2027 Research Papers PC — the real, published list.** It *is* public.
  - Page: https://conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee
    (note: the URL given in the task, `.../fse-2027-research-papers`, 404s; the live slug is
    `fse-2027-papers-program-committee`).
  - Chairs: Michael Pradel (CISPA), Baishakhi Ray (Columbia); 19 area chairs; **461 name entries
    total** in the committee listing. Manuel Rigger appears as an ordinary **Program Committee
    member**, listed with affiliation "National University of Singapore".
  - **No fallback to FSE 2026 was needed.**
- **Relevant service (from his homepage, https://www.manuelrigger.at/):** PC 2027 for OSDI, ICSE,
  FSE, VLDB; **FSE 2027 Poster Co-Chair** (so he holds a second FSE 2027 role beyond the PC);
  **PLDI 2024 Artifact Evaluation Co-Chair**; ISSTA 2024 Workshops Co-Chair; co-organizer of the
  **Fuzzing Summer Schools (2025–2026)** and the **FUZZING'27 workshop**.
  **Multiple distinguished reviewer awards: VLDB, ICSE, ASPLOS, ESEC/FSE.**
- **Why he fits this lens:** his entire output is test-oracle design, differential testing, and
  fuzz/generation-based bug finding on production systems; his current grant is literally a
  Singapore NRF/CSA "Fuzz Testing" programme award (acknowledged in two of the three papers below).

---

## (b) The three papers

I selected his three most recent papers (2026 venues) that are available in full as arXiv HTML and
that sit in this lens. I read each **end to end** (abstract, intro, approach, full evaluation,
discussion/threats, related work, conclusion, data-availability), not just abstracts.

### P1. *Metamorphic Coverage* — ISSTA 2026 / PACMSE (DOI 10.1145/3832104)
Jinsheng Ba, Yuancheng Jiang, **Manuel Rigger**. arXiv:2508.16307**v2** (16 Jul 2026, i.e. post-
acceptance) — https://arxiv.org/abs/2508.16307 · artifact https://github.com/nus-test/metamorphic_coverage

Proposes **Metamorphic Coverage (MC)**: instead of the union of code covered by a metamorphic
input pair, take the **symmetric difference** — the code executed by one input but not the other —
on the grounds that a bug is observable exactly when the faulty code is executed asymmetrically
across the pair. The paper first runs a small study of how ten well-known metamorphic testing
methods (EMI, HirGen, NoREC, TLP, YinYang, SAE, MetaOD, RTI, EDEFuzz, MROP) were evaluated, finding
that *bug count* is the only widely used metric and it is inherently *a posteriori*. MC is then
evaluated on five methods against SQLite, DuckDB, TVM, Z3 and CVC4: MC overlaps the bug-fix
locations of **50 of 64** real bugs, has PCC > 0.9 with bug count (vs. line coverage), is **4×** more
sensitive (coefficient of variation) than line coverage, **6×** smaller in absolute value, costs
**359×** less than mutation testing, and — used as fuzzing feedback on top of SQLancer's loop with
AFL++-style in-memory instrumentation — finds **41%** more bugs than branch-coverage guidance.
The implementation is ~100 lines of Python over `gcov`/`gcovr`.

### P2. *ACME / SQLxDiff: Enhanced Differential Testing in Emerging Database Systems* — FSE 2026
Yuancheng Jiang, Jianing Wang, Chuqi Zhang, Roland Yap, Zhenkai Liang, **Manuel Rigger**.
arXiv:2501.01236v1 (2 Jan 2025) — https://arxiv.org/abs/2501.01236
*(Caveat: the only arXiv version is v1, formatted for ISSTA 2025 under the title "Enhanced
Differential Testing in Emerging Database Systems". The TEST Lab site lists this work as **ACME**,
FSE 2026. I read v1 in full; the FSE camera-ready may differ — flagged again in §(g).)*

Key insight: emerging DBMSs (time-series, streaming: QuestDB, TDEngine, RisingWave, CrateDB) are
conceptually *extensions* of relational ones, so a mature relational DBMS (PostgreSQL) can serve as
the differential reference. The obstacle is that only a small set of **shared clauses** is directly
comparable; dialect gaps either cause false alarms or force the tool to avoid entire features
(e.g. QuestDB treats `null` as a value, not SQL `NULL`). The approach classifies clauses into
Shared / Failed / **Mappable**, then hand-writes **clause mappings** that rewrite target-specific
features into equivalent reference SQL (`sample by` → `group by` + subquery; `latest on` → join +
window function; `in` → `case…when` to restore NULL semantics; type aliases), and generates
semantically equivalent but syntactically different query pairs. Result: **57 previously unknown
bugs (17 logic, 40 internal errors), 50 fixed and 5 confirmed**; +20% unique query plans over
unmapped differential testing; a ported TLP oracle finds **0 of the 17** logic bugs, with an
explanation of why (the bugs do not live in predicates).

### P3. *Scaling Automated Database System Testing (SQLancer++)* — ASPLOS 2026
Suyang Zhong, **Manuel Rigger**. arXiv:2503.21424**v2** (26 Jan 2026), DOI 10.1145/3779212.3790215 —
https://arxiv.org/abs/2503.21424 · artifact https://doi.org/10.5281/zenodo.18289297

Framed openly as "**both a vision and a platform**". The problem: SQLancer's per-DBMS generators
average 3,729 LOC each, so most DBMS teams will never adopt oracle-based logic-bug testing (with a
verbatim Vitess blog quote to prove the pain is real). SQLancer++ replaces the hand-written
generator with an **adaptive statement generator** that emits statements using features that may or
may not exist in the target, observes errors, and statistically learns the dialect — plus an
internal schema model and a **bug prioritizer** that suppresses bug-inducing cases whose feature set
matches an already-reported bug. Across **18 DBMSs**: **196 unique previously unknown bugs, 180
fixed**, 140 of them logic bugs (vs. 77 and 51 in the original TLP and NoREC papers). Validity rate
on SQLite rises 292.5% with feedback. Prioritization collapses >60K bug-inducing cases per hour on
CrateDB to 35.8 reports, of which **11.4 are unique**, verified by bisecting to distinct fix commits.
The paper reports that **SQLancer beats SQLancer++ on line coverage by 59% (SQLite) and 21%
(PostgreSQL)**, and that plain fuzzers beat both — and argues from this that coverage does not track
logic-bug finding.

*(Supporting, read as abstract only: "On the Mistaken Assumption of Interchangeable Deep
Reinforcement Learning Implementations", ICSE 2025, arXiv:2503.22575 — a differential-testing study
that shows five PPO implementations are not interchangeable and **replicates a prior study to show
the assumption flips its conclusions**. Used below only as evidence of his appetite for
assumption-breaking negative results.)*

---

## (c) Writing taste

**How he frames contributions.** Layered and explicitly labelled. SQLancer++ closes its intro with
"At a conceptual level… At a technical level… At an empirical level…". MC separates "we propose a
metric" from "we evaluate it on five production methods". The framing word that recurs is
**simple**: "we believe that these techniques are both simple and practical"; "a simple and
practical metric"; "this simple approach". He sells *cheapness and generality*, not sophistication —
and he quantifies the cheapness (MC = ~100 LOC of Python; SQLancer++ = 16 LOC per new DBMS, 4 LOC
for most; SQLxDiff = 10–20 clause mappings, minutes-to-hours each, a few days per system).

**What counts as a claim vs. evidence.** For him the gold standard of evidence is **a real bug in a
real system that a real developer fixed**, with the issue-tracker URL in a footnote. Every bug table
carries a **status taxonomy** (unknown / confirmed / fixed) and fix counts are reported as the
*measure of importance*: "180 bugs have been fixed as a direct response to our bug reports, which
demonstrates that the DBMS developers considered most of the bugs important." He goes further and
treats **developer reception as data**, quoting CrateDB and Dolt developers verbatim with links.
Anything that is not a confirmed bug is explicitly demoted: coverage is called "some indication",
and in SQLxDiff he cites Inozemtseva & Holmes *against his own favourable coverage numbers*
("Higher coverage does not necessarily detect more bugs without a proper test oracle").

**Quantification.** Heavy, but always as a small number of memorable ratios: 50 of 64, 4×, 6×, 359×,
41%, PCC > 0.9, 292.5%, 196/180, 60K → 35.8 → 11.4, +20% query plans, 59% coverage deficit. Every
RQ in MC ends with a **one-sentence boxed takeaway** repeating its numbers. Statistical machinery is
light but present and appropriate: 5–10 repeated runs, 24-hour budgets, **coefficient of variation**
for sensitivity and **Pearson correlation** for the metric-vs-bugs claim. Notably, in what I read he
does **not** use Mann-Whitney/Â₁₂ significance testing — repetition counts plus CV are his answer to
randomness.

**Baselines.** Scrupulous, and this is his most distinctive habit. (i) He compares against the
*strongest* thing available, including the expensive one — MC is benchmarked against **mutation
score** (Mull, ~19K mutants on SQLite), not just line coverage. (ii) He **justifies every exclusion
by capability, not convenience**: a footnote explains why TLP and not NoREC/QPG/PQS/DQE; AFL-based
fuzzers are excluded because they cannot find logic bugs and need C/C++ instrumentation; EMI is
excluded because its source is unavailable and its bug reports are pre-minimized. (iii) He
**repairs the baseline in the baseline's favour** — he found and fixed an open SQLancer bug in its
QuestDB support "aiming for a fairer comparison". (iv) When an artifact is unavailable he says he
asked and got no reply (Unicorn). (v) He **deliberately avoids easy targets** (Timescale excluded
because it mirrors PostgreSQL, making differential testing trivial).

**Negative results.** He reports them prominently rather than burying them. SQLancer++ states
outright that the baseline wins on coverage by 59%/21% and that plain fuzzers beat both — and turns
it into the argument. MC reports that MC-guided fuzzing beats the domain-specific QPG baseline on
only **5 of 11** targets and explains why that is *expected*; it names the one target where plain
coverage guidance wins and diagnoses it (the bugs there are single-input internal errors). It also
reports **measurement failures**: mutation score was uninformative (~100%) for two configurations
and ~0.01% for two others, with the mechanism explained; HirGen has a false-alarm rate that the
authors confirmed when told; the SAE authors never answered their GitHub issues. Data losses are
quantified rather than glossed: only 63% (27/43) and 69% (20/29) of historical bug reports could be
reconstructed into metamorphic pairs, and TVM/CVC4 were dropped from the correlation analysis for
too few bugs.

**Threats to validity.** Variable in *form*, consistent in *substance*. MC has a conventional named
"Threats to Validity" paragraph (internal: randomness → 10 repetitions; external: MR choice,
language, coverage criterion) plus a separate **"Limitations"** subsection placed inside the
*approach* section, before any results, giving two concrete counterexamples where the metric fails
in each direction (MC = 0 yet the bug is findable; MC = 100% yet the oracle is weak). It also
confesses a scale bottleneck (gcov emits ~1 GB per input pair on TVM) and argues why the Q4
in-memory pipeline covers the larger-scale case. The conclusion says plainly: "**MC is no
panacea**". SQLxDiff (v1) has **no** named threats section — limitations live in "Discussion of
clause mapping" and "Discussion — false alarms" instead. So: he clearly cares that limitations are
*stated and concrete*, and is not dogmatic about the section heading.

**Artifacts.** MC: GitHub link under a "Data Availability" heading. SQLancer++: **Zenodo DOI plus a
full ASPLOS Artifact Appendix** (check-list, hardware/software dependencies, datasets, installation,
workflow, expected results, methodology). SQLxDiff v1 (preprint stage) says only "will be available
upon paper acceptance" — so he tolerates that at preprint but ships a real, citable artifact at
camera-ready. He is a former **PLDI Artifact Evaluation co-chair**.

**Intro/motivation structure.** Consistently: (1) the domain matters, with a market or deployment
number (DBMS market $162.25B, 15.2% CAGR; MySQL 5.5M LOC; 35.1% of 450 new DBMSs use SQL);
(2) a concrete pain point, ideally quoted from a practitioner (the Vitess blog; the NoREC authors'
own line that "code coverage is not particularly useful for fuzzing DBMS"); (3) a **minimal running
example** — a 6-line C absolute-difference function with two metamorphic relations for MC; a single
QuestDB `in`-with-`null` query for SQLxDiff — carried through background and approach;
(4) numbered contribution bullets. Motivation is empirical, never rhetorical: SQLxDiff opens with
its *own three-RQ study* of the emerging-DBMS landscape before proposing anything, and MC opens with
a ten-method survey of how the field actually evaluates itself.

**Tone and length.** Measured, hedged on generality ("we believe", "might", "has the potential",
"promising"), bold only where a counted number backs it. He under-claims scope by design:
SQLancer++ calls itself "the **first major step**… various follow-up challenges remain" and lists
them in the conclusion. RQs are short, named, and few — 3 (SQLxDiff) to 5 (MC), each one sentence,
phrased as a capability question ("Q.1 Effectiveness: Can MC evaluate the bug-finding capability of
metamorphic testing methods?").

**Tables/figures he favours.** (1) A per-system **bug table** with columns for bug type and status;
(2) **short reduced listings** of the actual bug-inducing input next to the wrong output and the
root cause/fix, one per interesting bug; (3) **time-series line charts over 24 h** of coverage or
unique query plans, averaged over runs; (4) compact numeric comparison tables (CV, validity rate,
time cost); (5) a single workflow/architecture figure with numbered steps ①–④ referenced from prose;
(6) a Venn-style figure for shared vs. mapped clauses. He also likes a **related-work comparison
table** (MC's Table 9) that positions prior metrics on explicit axes.

**Related work.** Not a list. MC devotes a full paragraph to "the closest prior work" (Cao et al.'s
BCMD) and states the delta at an almost surgical level: BCMD is the *cardinality* of the differential
set, MC retains *the set itself*, which makes it **linkable to code** — so it can be checked against
bug-fix locations and used to find untested regions, which a scalar cannot. He then characterises
*all* prior work on one axis (manually crafted relations, small programs, seeded mutations) versus
his own (production methods, industrial systems, 64 real bugs).

---

## (d) Likely reviewing standards — a concrete checklist

What I would expect him to demand of a paper in the fuzzing / differential-testing / oracle lens.
Items 1–9 are strongly grounded in his own papers' practices; 10–12 are more inferential.

1. **Bugs must be real, reported, and tracked.** Expect a table of bugs with an explicit
   reported/confirmed/fixed taxonomy, issue-tracker links, and a fix rate. "We found N bugs" with no
   upstream reports, or with everything sitting at "unknown", will not count as effectiveness
   evidence. *(Observed in all three papers.)*
2. **Deduplication with a defensible ground truth.** He will ask how many *inputs* became how many
   *reports* became how many *unique root causes*, and how uniqueness was established. His own
   answer is bisecting to distinct fix commits (60K → 35.8 → 11.4). Crash-hash or stack-hash
   clustering presented as a bug count is exactly what he built a whole component to avoid.
3. **Oracle precision must be measured, not asserted.** Expect a false-alarm discussion that
   includes the ones you hit *during development*, plus a statement of how a divergence is
   adjudicated into a defect and who adjudicated it (you, the developers, a sanitizer).
4. **Coverage is a proxy and must be labelled as one.** He will accept coverage as "some
   indication" of breadth, never as the effectiveness claim; and he expects the paper to say so —
   he cites Inozemtseva & Holmes against his own favourable numbers, and publishes a 59% coverage
   loss to his baseline. Conversely, a coverage-only evaluation with no oracle story is fatal.
5. **Baseline choice justified by capability.** For every omitted competitor, state whether it was
   omitted because it cannot express the property (e.g. a crash oracle cannot find silent
   miscomputation), because the artifact is unavailable (say that you asked), or because the
   comparison is apples-to-oranges — and put it in a footnote if need be.
6. **Baselines run in good faith.** Fix known bugs in the baseline, use its intended configuration,
   do not pick trivially favourable targets, and state the criteria by which targets were chosen
   (he uses GitHub stars / DB-Engines rank / prior-work coverage).
7. **Repetitions, budget, and hardware disclosed.** 5–10 runs minimum, a stated wall-clock budget
   (24 h is his default, citing Klees et al.), the exact machine, and per-target breakdowns rather
   than one aggregate mean. Variability reported (he uses CV); a single-seed single-run result will
   be challenged.
8. **The technique's failure modes stated up front.** He puts a "Limitations" block *before* the
   evaluation with a concrete counterexample in each direction. Expect him to ask: when does your
   oracle miss a bug, and when does it fire on a non-bug?
9. **Adoption cost quantified.** LOC, hours, or per-target effort for applying the technique to a
   new system — and honesty that manual effort exists, benchmarked against what competitors require.
10. **The delta against the single closest prior work, stated precisely.** Not "unlike prior work,
    we…" but a sentence a skeptic could falsify (set vs. cardinality; production systems vs. seeded
    mutants on ≤800-LOC programs). *(Inferred from how he writes his own related work.)*
11. **An artifact with a stable identifier and a usable appendix.** Zenodo/DOI, dependencies,
    workflow, expected results. As a former PLDI AE co-chair he will notice "available upon
    request". *(Observed artifact practice; the reviewing demand is inferred.)*
12. **Scope claimed no wider than measured.** He hedges generality in his own abstracts and calls
    his platform "a first step". Expect him to red-pen "general", "fully automatic", "complete", or
    "first" that the evaluation does not reach. *(Inferred.)*

---

## (e) Red flags that would make him reject

- **A headline count of divergences, crashes, or clusters presented as a bug count**, with no
  deduplication method and no root-cause or fix-commit grounding.
- **No upstream engagement**: bugs never reported, or reported and unconfirmed, with the paper still
  claiming practical impact. (He even declines to report bugs to projects that don't fix them —
  MySQL — and says so.)
- **Coverage as the effectiveness claim.** "We achieve X% higher coverage, therefore we are better"
  is a thesis he has publicly falsified with his own data.
- **A new oracle with no false-positive analysis**, or false alarms dismissed as "easily filtered".
- **Missing or hobbled baseline**: skipping the obvious competitor, running it out of its intended
  scope, not fixing its known defects, or silently benchmarking on targets chosen because the
  baseline is weak there.
- **Single run, no budget, no hardware, no variance.** Also: aggregate-only numbers that hide which
  targets carried the result.
- **Unstated manual effort.** Claiming "automatic" while the approach needs hand-written mappings,
  schemas, or harnesses that the paper does not cost out.
- **Overclaimed generality or a "first" that the evaluation doesn't support**; likewise a technique
  evaluated only on toy programs or seeded mutants and claimed for production systems.
- **Related work that lists neighbours without stating the delta against the nearest one** — his
  own related-work sections show he considers this a substantive, not cosmetic, obligation.
- **No artifact, or a non-reproducible one**, for a tools paper.
- **Threats to validity that are boilerplate** ("we mitigated by using well-known subjects") rather
  than a named failure mode with a worked counterexample.

---

## (f) What would delight him

- **A simple idea with an outsized, measured payoff** — ideally one whose implementation cost is
  small enough to quote (his own: ~100 LOC of Python; 16 LOC per new DBMS). Cheap + general is the
  aesthetic he explicitly argues for.
- **Bugs that developers cared enough to fix**, especially long-latent ones in heavily-tested
  software, with the fix commit analysed. (SQLancer++'s SQLite `REPLACE` bug, bisectable to a 2014
  commit and missed by a decade of tools, is exactly the anecdote he chose to lead with.)
- **Measured linkage between a proxy metric and real defects** — MC's headline is not "our metric is
  higher" but "the code our metric measures overlaps the fix locations of 50 of 64 real bugs".
  Showing that your signal actually touches the faulty line will land with him.
- **Reporting a loss to the baseline and reasoning from it.** He does this twice and builds an
  argument out of it; a paper that does the same reads as trustworthy to him.
- **Adjudicating "N findings" down to "M root causes" rigorously**, with the reduction ratio
  reported as a contribution rather than hidden.
- **Testing systems nobody has tested before**, or reaching boundaries/dialects/features that prior
  tools structurally could not reach — with evidence that the prior tool genuinely cannot (he ports
  TLP and shows it finds 0 of 17).
- **Clean, honest ablations**: with/without the mechanism, per target, repeated, including the
  targets where the ablation says the mechanism doesn't help.
- **A study that measures a community assumption and finds it false**, ideally by re-running a prior
  paper and flipping its conclusion (his ICSE'25 DRL paper does precisely this).
- **Developer or practitioner voice as evidence** — quoted issue-tracker replies, adoption requests,
  a blog post from the vendor.
- **A minimal, memorable running example** introduced in the intro and reused in the approach.

---

## (g) Evidence vs. inference disclosure

**Observed (fetched and read this session):**
- The FSE 2027 Research Papers PC page (live HTML, 461 committee name entries), confirming Rigger as
  a PC member with NUS affiliation, and confirming chairs/area chairs. The FSE 2027 PC **is public**;
  no FSE 2026 fallback was used. The task's URL 404s; the working slug is
  `/committee/fse-2027/fse-2027-papers-program-committee`.
- His homepage https://www.manuelrigger.at/ (via WebFetch summarisation), for bio, PC/chair service,
  the Fuzzing Summer Schools / FUZZING'27 organisation, and the statement of **distinguished
  reviewer awards at VLDB, ICSE, ASPLOS and ESEC/FSE**. I did not independently verify each award
  against the awarding venues' own pages.
- The TEST Lab publication listing (https://nus-test.github.io/publication/), for the 2025–2026
  publication set and venue assignments.
- **P1 (Metamorphic Coverage) and P3 (SQLancer++): read in full** from arXiv HTML (v2 of each,
  both post-acceptance, both carrying their ACM DOIs). Every section including evaluation, threats,
  discussion, related work, conclusion and data availability.
- **P2 (ACME / SQLxDiff): read in full**, but from **arXiv v1 only** — a January 2025 version
  formatted for ISSTA 2025 under a different title. The TEST Lab site lists the work as ACME at
  FSE 2026; no newer arXiv version exists. **The FSE camera-ready may add a threats-to-validity
  section, an artifact link, and different numbers; my observations about P2's missing threats
  section and "available upon acceptance" artifact statement apply to v1 and may not hold for the
  published paper.**
- The ICSE 2025 DRL paper: **abstract only** (arXiv:2503.22575). Used only for the narrow claim that
  he publishes assumption-breaking differential-testing studies.
- I read arXiv **HTML renderings**, not the publisher PDFs. Mathematical notation and figure/table
  contents are therefore partially lost; all numbers I quote come from prose or from prose that
  narrates a table, never from a table I could not see. Figure-only claims are not quoted.

**Inferred (labelled as such in-line):**
- **Everything in §(d)–§(f) is inference.** I found **no** public "how I review" statement by
  Rigger: no blog, no OpenReview record, no PC-chair notes, no talk on reviewing that I could
  locate. The checklist is reverse-engineered from the standards his own papers hold themselves to,
  on the assumption — common but not guaranteed — that a reviewer asks of others roughly what he
  practises. The distinguished-reviewer awards and the PLDI AE co-chair role support the general
  inference that he reviews thoroughly and weighs artifacts heavily; they say nothing specific about
  *which* criteria he applies.
- The statistical-methods observation ("no Mann-Whitney/Â₁₂ in what I read") is an observation about
  three papers, not a claim that he rejects significance testing; a reviewer may well demand of
  others a rigour his own venue norms did not require.
- Items 10–12 of the checklist and all of §(f) beyond the directly-mirrored practices are the most
  speculative parts of this profile.

**Not done:** no attempt to contact him; no non-public sources; nothing read from behind a paywall;
no PDFs downloaded from ACM DL.
