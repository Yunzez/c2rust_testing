# PC Profile: Marcel Böhme (FSE 2027 Research Papers PC)

Reviewer lens for this profile: **empirical SE methodology and testing-research rigor** —
experimental design, statistics, threats to validity, benchmark construction, replication,
and evaluation of testing tools / test oracles.

Compiled: 2026-09-14. Read-only research; no fuzzing or builds were run.

---

## (a) Who / affiliation / which PC list

- **Name:** Marcel Böhme
- **Affiliation (per PC listing):** CISPA Helmholtz Center for Information Security, Germany.
  His own CV and paper mastheads (2025–2026) say **MPI-SP (Max Planck Institute for Security
  and Privacy), Bochum, Germany**; the FSE 2027 listing records CISPA. Treat the affiliation
  string as in flux; the person is the same.
- **PC list used:** **FSE 2027 Research Papers Program Committee** — the list *is* public.
  Source: <https://conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee>
  (note: the URL given in the task, `.../fse-2027-research-papers`, 404s; the live path is
  `fse-2027-papers-program-committee`). FSE 2027 is in Shenzhen, China, 12–16 July 2027;
  PC chairs are Michael Pradel (CISPA) and Baishakhi Ray (Columbia). Böhme appears as a
  **regular PC member** (not an area chair) on that list.
- **Why this person for this lens:** he is the single closest match on the FSE 2027 PC to
  "rigor of testing-tool evaluation." His recent first- and last-author output is literally
  *meta-research about how testing tools are benchmarked*: how noisy a fuzzer ranking is,
  whether a benchmark measures the construct it claims to measure, and what a proxy metric
  (coverage) is actually good for.
- **Relevant service (observed, from his CV):** ISSTA'26 PC Co-Chair; ASE'25 PC Co-Chair
  (325 PC members, 20 area chairs, 1.2k submissions); ICSE'24 Area Chair (Dependability and
  Security); ASE'24 Area Chair (Security); ACM TOSEM Editorial Board member **and Guest
  Editor-in-Chief for Registered Papers since 2022**; ISSTA/ASE/FUZZING steering committees;
  FSE 2027 New Faculty Mentoring Symposium chair. **Distinguished Reviewer Awards at ICSE'25,
  CCS'23, ICSE'23, ICST'22.** CV: <https://mpi-softsec.github.io/CV.Marcel.Boehme.pdf>
  (retrieved 2026-09-14, "last updated January 2026").

---

## (b) The three papers

Selection rule: the three most recent papers I could fetch **in full text** that sit squarely in
this lens (benchmark construction / measurement validity / statistical evaluation of testing
tools). All three were read as full PDFs (abstract, intro, design, results, threats,
conclusion). Homepage evidence of his 2025–2026 output: <https://mboehme.github.io/>.

### 1. In Bugs We Trust? On Measuring the Randomness of a Fuzzer Benchmarking Outcome
Ardi Madadi, Seongmin Lee, Cornelius Aschermann, Marcel Böhme. **Proc. ACM Softw. Eng. 3, FSE,
Article FSE084 (FSE 2026)**, 22 pages. DOI <https://doi.org/10.1145/3797112>;
PDF read: <https://nimgnoeseel.github.io/assets/pdf/concordance-paper.pdf>.
Artifact: <https://github.com/ardier/in_bugs_we_trust/>.

The paper imports **mean split-half reliability** from psychometrics and renames it
*concordance*: the expected agreement, between two random disjoint equal-sized halves of a
benchmark suite, on the *ranking* a benchmarking procedure produces. Using ~100,000 CPU hours
(FuzzBench, 9 fuzzers × 24 C benchmarks × 20 × 23h; plus their own Magma run, 8 fuzzers ×
18 benchmarks × 20 × 23h), they find coverage-based ranking is strongly concordant
(γ̂ = 0.843 FuzzBench, 0.782 Magma) while bug-based ranking is weak (0.373) on FuzzBench and
moderate (0.603) on Magma. The headline is deliberately uncomfortable and stated as the first
line of the abstract: **on FuzzBench a coverage-based evaluation predicts the outcome of a
bug-based evaluation better than an independent bug-based evaluation does** — i.e. the
"gold-standard" metric is noisier than its own proxy. They then reuse concordance as a
*benchmarking-efficiency* measure (green fuzzing: how far you can cut suite size or campaign
length before the ranking degrades) and report the CO₂ savings, with an appendix modelling
energy consumption.

### 2. On Interaction Effects in Greybox Fuzzing
Konstantinos Kitsios, Marcel Böhme, Alberto Bacchelli. **ICSE 2026**, 12 pages.
arXiv:2510.19984 (22 Oct 2025): <https://arxiv.org/abs/2510.19984>.
Artifact: <https://doi.org/10.5281/zenodo.17391100>.

Hypothesis-first tool paper: the *order* in which AFL++'s 32 mutators are applied matters, so
the standard "sample each mutator independently from a learned weight" design is leaving value
on the table. Contribution 1 is empirical evidence — a linear model over all mutator pairs whose
**interaction term explains a statistically significant share of variance** (with per-program
adjusted R², and with three programs *excluded from the ANOVA because they violated the
independence assumption*). Contribution 2 is MuoFuzz, which samples the next mutator from a
distribution conditioned on the previous one (explicitly framed as a bigram language model).
Contribution 3 is the evaluation: FuzzBench (13 programs, 24h, **30 trials**) and Magma (all 8
programs, 24h, 20 trials), Mann-Whitney U with p-values and **Vargha-Delaney Â₁₂ effect sizes**
in the main table, plus a time-to-reach-baseline-coverage table, plus a four-variant ablation.

### 3. Top Score on the Wrong Exam: On Benchmarking in Machine Learning for Vulnerability Detection
Niklas Risse, Jing Liu, Marcel Böhme. **Proc. ACM Softw. Eng. 2, ISSTA, Article ISSTA018 (ISSTA
2025)**, 23 pages. **ACM SIGSOFT Distinguished Paper.** DOI <https://doi.org/10.1145/3728887>;
PDF read: <https://mboehme.github.io/paper/ISSTA25-topscore.pdf>; arXiv:2408.12986.
Artifact: <https://github.com/niklasrisse/TopScoreWrongExam>.

A construct-validity demolition. A literature survey of 81 papers establishes that 9 in 10 ML4VD
papers define the task as function-level binary classification. Two security researchers then
**manually label 300 randomly sampled "vulnerable" functions from BigVul, Devign and DiverseVul
over 150+ hours**, double-blind-ish (independent, then disagreements resolved by discussion),
reporting Cohen's κ = 0.64 for vulnerable/secure and κ = 0.96 for context-dependence. Findings:
**all 151 actually-vulnerable functions required context beyond the function**, and 82 of 90
"secure" functions would be vulnerable under some plausible calling context — so the label is
not a function of the input the model is given. RQ.2 lands the knife: a gradient-boosting
classifier on **word counts alone** reaches 62.2% F1, i.e. the benchmark can be topped without
any vulnerability-detection capability. They conclude the prevailing problem statement is
ill-defined and spend a long constructive Discussion on alternatives (abstention, other base
units, repository-level context, generating a proof-of-vulnerability test case).

---

## (c) Writing taste

**Framing of contributions.** He frames contributions as *measured claims about a measurement*,
not as feature lists. Two of the three papers are meta-research: the object of study is the
community's own evaluation apparatus. Even the tool paper (MuoFuzz) is structured
hypothesis → empirical evidence for the hypothesis → tool that exploits it → evidence the tool
works → ablation that the gain comes from the hypothesized mechanism. He does not let a tool
paper be *only* a tool paper.

**Bold thesis, hedged details.** The rhetorical signature is a single provocative sentence up
front, immediately followed by carefully scoped numbers. "In Bugs We Trust?" opens its abstract
with the counterintuitive finding rather than with background. "Top Score on the Wrong Exam" is
a title that accuses a whole subfield. But inside, every claim is bounded: "weak on FuzzBench
and moderate on Magma", "the target programs we use are open-source C libraries, hence we do not
make claims beyond that." Bold headline, hedged scope — never the reverse.

**What counts as evidence.** (i) Numbers with a stated interpretation scale — he adopts
Schober et al.'s agreement bands and Landis–Koch's κ bands rather than inventing adjectives;
(ii) effect sizes alongside p-values (Â₁₂) — never a bare "significant"; (iii) manual, expensive
ground truth with inter-rater agreement when automation would be unsound (150 hours of labeling);
(iv) negative/inconvenient results reported at equal weight — three programs dropped from the
ANOVA for violating independence, six programs where MuoFuzz shows no significant gain, each
one *explained* rather than buried. Anecdote is never evidence; a running example is used to
*motivate* (the TFLite `ResizeOutputTensors` function opening "Top Score"), never to substantiate.

**Quantification density.** Very high, and always with dispersion: median ± st.dev over 30
trials, distributions over 10,000 sampled benchmark-subset pairs, CPU-hours stated as a budget
("over 100,000 CPU hours ≈ 11 CPU years"), even carbon cost modelled in an appendix. Resource
budget is treated as a first-class experimental parameter, and program selection is justified
*against* the budget ("FuzzBench comes with 28 target programs, which is beyond our
computational budget; for this reason we ... use the same programs as two recent fuzzing
papers" — reuse of prior selections explicitly to avoid cherry-picking).

**Baselines.** Chosen with an argued rationale and defended for fairness: AFL++ because MuoFuzz
is built on it; MOPT because it is the isolated-probability analogue *and* because a prior study
found it is the most common recent baseline. Crucially, they **drop a baseline (SeamFuzz)
rather than run it unfairly** on an obsolete AFL++ v3.15, say so in the text, and note they
contacted the authors. Baseline fairness beats baseline count.

**Threats to validity.** Always a dedicated numbered section, always split into the classic
internal / external / construct / conclusion categories (not every paper uses all four), and —
the distinguishing habit — **each threat is paired with a mitigation that was actually
performed, often with a re-analysis**. Example: the concordance paper worried that programs with
many harnesses dominate, so they *recomputed every analysis at program level* and report that
the interpretation changed for exactly one suite/procedure combination. That is a threats
section doing work, not a ritual.

**Artifact availability.** Non-negotiable and structural. Every one of the three papers has a
named artifact (GitHub or Zenodo DOI), and two have a dedicated numbered "Data Availability and
Reproducibility" section rather than a footnote. His CV states he runs his group under an "Open
Science and Reproducibility Policy" that includes publishing "the scripts to produce tables and
figures" — i.e. figure-level reproducibility, not just code drop.

**Structure.** Intro = problem-in-the-field → the uncomfortable question → summary of findings →
explicit bulleted contribution list (the last bullet is frequently the artifact). RQs are short,
one sentence, often with a parenthetical name (RQ.1 (Concordance), RQ.2 (Concordance as a
Function of Benchmarking Suite Size)), sometimes decomposed into lettered sub-questions
(RQ.1-a/b/c) where each sub-question is a separate measurement. Results sections are headed by
the RQ and closed by a **boxed "Finding N" summary** stating the result in one or two sentences.
Discussion/Future Work is substantive and constructive — after tearing down ML4VD he spends two
pages proposing replacements.

**Tables and figures.** Favoured: dense comparison tables where each row is a program and the
statistics columns (p, Â₁₂) sit beside the medians; small inline summary tables of the headline
statistic with an "Interpretation" column mapping the number to a band; scatter plots of ranking
pairs with frequency encoded by size/colour; a process flowchart for any manual labeling
protocol (Fig. 5 of "Top Score" is the labeling decision tree). Bar charts of *how often the
community uses each dataset* — he likes to quantify the community's own practice before
critiquing it.

**Tone.** Plain, declarative, first-person plural, willing to be blunt about the field
("the prevailing problem statement of ML4VD is ill-defined and call into question the internal
validity of this growing body of work") but never about people, and always followed by
"Constructively, we ...". Self-critical in public: the concordance paper says of his own prior
ICSE'22 work, "what we seemed to miss is that the outcome of a bug-based evaluation itself may
be substantially noisy."

---

## (d) Likely reviewing standards — a concrete checklist

Items marked **[O]** are observed in his own papers or documented service; **[I]** are my
inferences from those observations.

1. **[O] Is the claim separated from the result, and is the methodology sound independent of how
   the numbers came out?** His registered-report initiative evaluates submissions on
   "(i) the significance and novelty of the hypotheses or techniques and (ii) the soundness and
   reproducibility of the methodology specified to validate the claims or hypotheses — but
   explicitly not based on the strength of the (preliminary) results." Expect him to ask, of any
   evaluation: *would this design have been convincing if the result had been null?*
2. **[O] Does the metric measure the construct the paper claims?** This is his central move in
   two of three papers. A paper that reports "we found N bugs" or "X% coverage" must argue that
   the measure tracks the capability being claimed, and must acknowledge where it doesn't.
3. **[O] Multiple randomized trials, with dispersion, effect size, and a named statistical test.**
   20–30 trials per configuration is his own practice; medians ± st.dev., Mann-Whitney U, and
   Vargha-Delaney Â₁₂ appear in the main table, not an appendix. A single run, or means without
   variance, is disqualifying for a stochastic technique.
4. **[O] Is the benchmark/subject selection justified against a stated budget, and free of
   cherry-picking?** He states the budget in CPU-hours, explains why the full suite was not used,
   and reuses other papers' program sets specifically so the choice is not his own. Expect
   "why these subjects and not others?" on every empirical paper.
5. **[O] Are baselines *fair* as well as present?** Version parity, same harness/infrastructure,
   same seeds, and a willingness to *exclude* a baseline (with explanation) rather than run it
   crippled. He also reads which baselines the community currently expects.
6. **[O] Is there an ablation isolating the claimed mechanism?** He cites the fuzzer-evaluation
   literature to justify demanding one, and runs a four-variant ablation himself. "Our tool is
   better" without "and better *because of the thing we said*" is incomplete.
7. **[O] Are the cases where the technique did NOT win reported and explained?** MuoFuzz loses or
   ties on 4–5 of 13 programs and each is diagnosed (weak interaction effect, low adjusted R²,
   coverage saturation in 15 minutes). Silence about non-wins reads as suppression.
8. **[O] Are statistical assumptions checked, and data excluded when violated?** Three programs
   were dropped from the ANOVA for violating independence *and that exclusion is stated in the
   results*. Expect him to check whether a test's assumptions were even mentioned.
9. **[O] Is human labeling done by qualified people, independently, with an agreement statistic
   and a published protocol?** Two labelers, Cohen's κ reported and interpreted against
   Landis–Koch, a decision-tree figure, a 15-minute per-item time box, explicit statement of the
   labelers' expertise, and all labels + free-text justifications published.
10. **[O] Is there an artifact, and does it regenerate the tables and figures?** Not just "code
    available" — his own policy is scripts that produce the tables and figures, and his papers
    carry a numbered Data Availability section.
11. **[O] Does the threats section contain mitigations that were actually executed?** He
    re-runs analyses to answer his own threats. A threats section listing risks with no
    countermeasure, or one that omits construct and conclusion validity, will draw fire.
12. **[I] Is the scope of the claim exactly the scope of the evidence?** Given his habit of
    writing "hence we do not make claims beyond that," expect him to strike over-generalization
    from C/C++ to "software", from one benchmark suite to "in practice", or from a proxy metric
    to the capability it proxies.
13. **[I] Is the paper honest about cost/efficiency?** He models CPU-hours and even carbon.
    Expect questions about how much compute a claimed improvement needs, and whether the
    comparison gave both tools equal resources.

---

## (e) Red flags that would make this reviewer reject

- **A benchmark whose labels/oracle are not shown to be valid.** If the paper's ground truth can
  be satisfied by something other than the capability under study — spurious features, a leaky
  label, an oracle that fires for reasons unrelated to the defect — he has published the
  playbook for demolishing exactly that ("word counts alone get 62.2% F1"). **[O]**
- **Single-run or few-run results for a stochastic tool; means with no variance; "significant"
  with no test, or a test with no effect size.** **[O]**
- **A subject/benchmark set that looks hand-picked**, or an unexplained subset of a standard
  suite, especially if the excluded subjects are plausibly the hard ones. **[O]**
- **Unfair baselines**: outdated versions, different hardware, different time budgets, or
  "we compare against no baseline because none exists" without an argued degenerate baseline.
  **[O]** (He went out of his way *not* to run SeamFuzz unfairly.)
- **A new tool with no ablation**, where the improvement could be attributable to an incidental
  engineering difference rather than the stated idea. **[O]**
- **Missing or unreproducible artifact**, or an artifact that does not regenerate the reported
  numbers. **[O]/[I]**
- **A ritual threats-to-validity section** — a paragraph of generic risks with no mitigation,
  no construct validity, no conclusion validity. **[O]**
- **Claims broader than the evidence**: "we show that fuzzing is X" from 8 C programs; "our
  approach generalizes" with no cross-domain data point. **[I]**
- **Counting outcomes without de-duplication or root-cause analysis** — he is on record (via
  his own and Magma's critique of crash counting) that "N crashes" is not "N bugs" unless
  de-duplication is defined and defended. **[O, via the papers' framing of bug counting]**
- **[I] Novelty claimed by overlooking the prior meta-work.** A paper that reports a coverage
  ranking, or a bug-count ranking, as if its reliability were unquestioned — without engaging
  the fact that these rankings are known to be noisy and to disagree — will read as unaware of
  the state of the art in its own evaluation method.
- **[I] Defensive framing.** Given his willingness to write "what we seemed to miss" about his
  own prior paper, a rebuttal that argues a limitation away rather than measuring it is likely
  to cost more than admitting it.

---

## (f) What would delight them

- **A measurement of the paper's own measurement.** Reporting how stable/reliable your own
  evaluation procedure is — e.g. split-half agreement of your ranking, sensitivity of your
  conclusion to benchmark subset or campaign length — is the thing he invented a metric for.
  Doing it unprompted signals you read the field's methodology literature. **[O]**
- **A named, falsifiable hypothesis stated before the tool**, then evidence for the hypothesis
  that is independent of the tool's success. That is the MuoFuzz structure exactly. **[O]**
- **Expensive, honest ground truth**: manual labeling with κ, published labels *and*
  justifications, plus a validity check on the labels themselves. **[O]**
- **A negative or inconvenient finding reported at full strength**, including "our technique did
  not help here, and here is the mechanism-level reason why." **[O]**
- **A constructive path after a critique.** If you break something, propose the replacement and
  say what evidence would validate it — he spends two pages doing this in "Top Score." **[O]**
- **An unignorable single-sentence headline backed by a scoped number** — he writes that way and
  will recognize it. **[I]**
- **[I] Cheap-but-equivalent evaluation.** Because he cares about benchmarking efficiency and
  carbon, an evaluation that shows "we get the same conclusion with 1/4 of the compute, and here
  is the evidence the conclusion is unchanged" is likely to please rather than worry him.
- **[I] Explicit oracle/definedness discipline.** He funded an entire paper on the fact that a
  "vulnerable" label is meaningless without the calling context; a paper that is precise about
  when a difference counts as a defect versus an artifact of an undefined input would be
  speaking his language.

---

## (g) Evidence-vs-inference disclosure

**Observed (read directly this session):**
- FSE 2027 Research Papers PC listing, retrieved as raw HTML from
  `conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee` on 2026-09-14;
  Böhme listed as a PC member (affiliation string: CISPA). The `.../fse-2027-research-papers`
  URL in the task description returns HTTP 404.
- Full PDF of **"In Bugs We Trust?" (FSE 2026)**, 21 pages — read abstract, intro, formal
  definitions (§2), experimental setup and RQs (§3), results (§4), efficiency study (§5),
  threats (§6), related work (§7), conclusion (§8), data availability (§9), energy appendix.
- Full PDF of **"On Interaction Effects in Greybox Fuzzing" (ICSE 2026, arXiv:2510.19984)**,
  12 pages — read abstract, intro, evaluation design and all four RQs (§6), implications (§7.2),
  threats (§8), conclusion (§9). Skimmed rather than closely read: the MAB/algorithm details
  in §5 and the background in §2.
- Full PDF of **"Top Score on the Wrong Exam" (ISSTA 2025)**, 23 pages — read abstract, intro,
  study design and RQs (§4), the labeling protocol in detail, results summaries (§5), threats
  (§6), discussion (§7), data availability (§8). Skimmed: the literature-survey mechanics (§3)
  and the RQ.2 classifier hyperparameters.
- His **CV** (`mpi-softsec.github.io/CV.Marcel.Boehme.pdf`, self-dated January 2026) for service
  roles, awards, the Open Science policy, and the verbatim description of his preregistration
  initiative's two-stage review criteria.
- His **homepage** (`mboehme.github.io`) publication listing for 2025–2026, used to choose the
  three papers.

**Not observed / could not verify:**
- **No "how I review" post, blog, OpenReview record, or recorded talk on reviewing was found.**
  I searched for one explicitly and came up empty; SE conferences do not publish reviews, so
  there is no public corpus of his actual review texts. Everything in section (d) beyond item 1
  is therefore inferred from the standards his own papers hold themselves to, not from observed
  reviews. Item 1 is the exception: the review criteria quoted there are his own words about a
  review process he designed, taken from his CV.
- DBLP was unreachable from this machine (bot-blocked), so the "three most recent" set was
  assembled from his homepage plus targeted search. His homepage lists further 2026 papers I did
  **not** read (e.g. "Evaluating LLM-Based Regression Test Generation", FSE'26;
  "Dependency-aware Residual Risk Analysis", ICSE'26; "Scaling Security Testing by Addressing
  the Reachability Gap", ICSE'26). My three are the most lens-relevant, not provably the three
  chronologically newest.
- I did **not** read "On the Reliability of Coverage-Based Fuzzer Benchmarking" (ICSE'22) in
  this session; where it is referenced above, the characterization comes from how the FSE 2026
  paper describes and self-critiques it.
- The CISPA-vs-MPI-SP affiliation discrepancy is unresolved; I report both.
- No claim here is based on personal communication, private information, or any source I did
  not retrieve.
