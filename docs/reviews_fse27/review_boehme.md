# FSE 2027 Research Papers — Review

**Paper:** *When Is a Difference a Defect? Differential Validation of C-to-Rust Translations*
**Reviewer persona:** Marcel Böhme (FSE 2027 Research Papers PC), per
`docs/reviews_fse27/pc_boehme.md`
**Scope note:** Content only. Citation style, LaTeX, grammar and sentence-level wording are
out of scope by the authors' request and are not commented on anywhere below.

---

## 1. Paper summary

The paper argues that a differential oracle for C-to-Rust translation is not given merely
because both programs are executable, and decomposes the oracle into four obligations:
recovering which C and Rust functions correspond, realizing one logical input across
representations that the translator may have reshaped, comparing the observable state that
actually carries the functional contract, and attributing a disagreement before counting it as
a defect. It implements these as a pipeline: a name-independent structural matcher
(IsoRank-style propagation plus maximum-weight assignment, with a two-sided abstention margin
ε), a deterministic, LLM-free *HarnessPlan* derived from C definitions that either constructs
a paired harness or records an explicit construction failure, Rust-only coverage-guided
discovery followed by paired replay over a fixed comparison ladder, and a side-isolated
confirmation funnel (C-only ASan+UBSan, combined replay, Rust-only replay with and without
sanitizers) followed by clustering and manual source-level triage. The evaluation spans six
translation systems × ten C applications and reports: matcher precision 0.938 forced /
0.993 at the ε=0.01 operating point (RQ1); 36 confirmed defects in 15 artifacts from the five
restructuring systems and none in the ten mechanical c2rust controls, of which 21 change
values or state without crashing (RQ2); a seven-family root-cause taxonomy (RQ3); campaign
reach across 37 artifacts with per-cell attribution of what limits it (RQ4); and a funnel
showing released FLOURINE/RustAssure/VERT artifacts applied to the frozen 36-defect corpus
(RQ5), plus a five-row component analysis.

---

## 2. Overall decision

**Decision: Weak Reject.**
**Reviewer expertise: Expert.** **Confidence: High.**

I want to be clear that this is not a rejection of the idea or of the engineering. The
framing is right, the confirmation pipeline is the part of this problem most papers get
wrong and this one gets right, and I verified that the funnel numbers reproduce from the
submitted artifact data. What stops me from voting to accept is that several of the paper's
headline claims are stated at a strength that the reported measurements do not reach: the
tool has **no recall measurement of any kind**, every campaign is **n = 1** while the text
nonetheless makes comparative statements about coverage, the **36 defects and their seven
families rest on single-rater judgments by the authors of the tool that found them, with no
second coder and no agreement statistic**, and two of the five component-analysis rows are
**n = 1 anecdotes** presented in a table that the summary paragraph generalizes from. Each of
these is fixable with experiments the authors are clearly capable of running on data they
appear to already hold.

---

## 3. Strengths

**S1. The confirmation funnel is real, and it reproduces (§RQ2, Fig. 2, Table 3).**
I checked the funnel against the archived summaries the figure header names. Summing
`results/rq3_coverage/*/*/confirm_sample/summary.json` over the non-c2rust cells gives
exactly 176,507 candidates and 14,470 adjudicated; `not_reproducible` = 955 accounts exactly
for the 14,470 → 13,515 step; the reference-associated verdicts account exactly for
13,515 → 3,365; and `confirmed_termination` (1,190) + `confirmed_divergence` (741) = 1,931.
Numbers in a paper that reconcile to the archived data at this granularity are rarer than
they should be, and I am giving the authors credit for it.

**S2. Attribution is put *before* deduplication, and the ordering is argued (§Related Work 2.3,
§4.4).** "The first triage question is therefore *which explanation owns the disagreement*;
deduplication comes afterward." This is the correct order for a cross-language differential
setting and it is not the order most crash-triage-derived pipelines use. The side-isolated
ladder — C-only ASan+UBSan to qualify the reference, Rust-only with *and* without sanitizers
to separate a program panic from an instrumentation artifact — is a genuinely careful design,
and Table 3 states per-stage exactly which side carries which check.

**S3. The paper repeatedly refuses to over-read its own evidence.** "Sanitizer-clean replay
means that no configured check fired; it is not proof that the C execution is free of all
undefined behavior" (§4.4). "These reductions are candidate accounting, not an estimated
false-positive rate, because confirmation caps each boundary's sample at 200" (§RQ2).
"This use is an operational negative control, not an assumption that C2Rust is universally
semantics preserving" (§4.3). "The ten-application corpus was frozen after exploratory work,
not sampled prospectively. Earlier notes explicitly allowed replacement subjects" (§7.2).
That last disclosure in particular is more honest than most authors manage, and it is the
right instinct.

**S4. The reach analysis attributes ceilings instead of reporting a bare percentage (§RQ4).**
Declaring a cell *translation-limited* only when "the C operation completes on the canonical
input, the sanitizer-free Rust execution fails or rejects the operation earlier, and source
inspection identifies the translation-induced cause" is a real criterion with an executable
test behind it. The cJSON × PtrTrans diagnosis — `cJSON_New_Item` is an untranslated stub
returning `None` on all 18 direct-constructor witnesses, so "adding an `Option<&mut T>`
adapter or running longer could not unlock the 64 object-taking boundaries" — is the model of
how to explain a low number instead of hiding it.

**S5. Measurement-instrument validity is checked in two places.** The coverage denominator is
taken from the artifact's own instrumented archive objects rather than a linked binary, and
"for 11 artifacts initially measured through a linked test or denominator binary, rebuilding
the universe from the archive objects reproduced the same in-scope functions and region
counts" (§RQ4). Separately, the Tulip seed study carries a length-matched random control and
a 600-second unseeded control that "reproduces the archived one-hour baselines within one
region". Both are unprompted checks that the instrument measures what it claims. Good.

**S6. The boundary accounting is internally consistent.** I summed the M/P/B columns of
Table 6 by hand: 4,222 matched, 1,854 planned, 1,662 built, matching the §RQ4 summary
sentence exactly. Similarly the 36 defects decompose correctly across Table 4 (20/7/4/4/1 by
tool, 15 crash / 21 semantic) and against `defect_manifest.json`.

**S7. RQ5's funnel separates unsupported input from semantic miss.** Reporting VERT as 36
unsupported with an explicit statement that this "is an applicability boundary for our
fixed-artifact question, not a semantic miss or evidence that VERT fails within its intended
generation workflow" is exactly right, and the refusal to count tool-side failures as misses
is the correct accounting.

---

## 4. Weaknesses, ranked by how much they drive my decision

### W1. There is no recall measurement for the paper's main contribution. (§RQ2, §RQ5, §7.2)
The paper measures, for the *baselines*, detection conditional on completed analysis
(FLOURINE 14/16, RustAssure 5/11). It measures nothing comparable for itself. §7.2 states
the limitation — "it does not estimate population-level recall or precision" — and §RQ5
states "this RQ measures applicability and positive-case sensitivity on these known defects,
not general recall". Disclosure is not a substitute for the measurement here, because the
paper's central quantitative claim (36 defects; 21 invisible to crash fuzzing) is a *count
with no denominator*. I cannot tell whether the validator finds most translation defects in
these artifacts or a small and idiosyncratic fraction, and the asymmetry — baselines are
evaluated on the authors' positives, the authors' tool is evaluated on nothing external — is
the single structural flaw a reviewer in my area will not let pass.

*Why it matters to me specifically:* my objection to ML4VD benchmarking was not that the
scores were wrong but that a top score was achievable without the capability being claimed.
The dual problem here is that a defect count is achievable without knowing what fraction of
the space it covers, and a reader will read "36 defects across five systems" as a capability
statement.

*What would resolve it:* a fault-injection / mutation study. Inject a stratified set of
translation-like faults (one per family in Table 5) into artifacts the pipeline already
builds, with an independent validity oracle, and report recall with its denominator and the
class of misses. Alternatively, run the validator on a corpus of defects discovered by
FLOURINE or RustAssure and report the reverse detection rate. Either makes Table 7 a
two-sided comparison instead of a one-sided one.

### W2. Every campaign is n = 1, yet the paper makes comparative statements from single runs. (§RQ4, Table 6)
§7.2 says "Each reported campaign is one fixed-budget observation of one frozen artifact, not
an estimate of expected fuzzer performance." That defence is adequate for RQ2, where
"defect promotion relies on deterministic, repeated replay rather than the frequency with
which fuzzing rediscovers an input" — I accept that, and it is well argued. It is *not*
adequate for RQ4, where the paper nonetheless writes "These seeds raise c2rust, C2SaferRust,
and CROWN to 90.2–93.2% region coverage and all functions; a length-matched random control on
c2rust reaches only 38.7%", and "the subsequent 600-second fuzzing adds 179–190 regions", and
compares Laertes' 64.9% against a decomposed 92.0%. Those are comparisons between stochastic
processes reported from one observation each, with no variance, no repetitions, and no test.
A single 1-hour libFuzzer run has run-to-run spread that can easily exceed the differences
the text interprets.

*What would resolve it:* repeat every coverage cell at least 10 times (20 preferred), report
median and a dispersion measure in Table 6, and attach a Mann-Whitney U with a
Vargha-Delaney Â₁₂ to each of the seed-vs-random and seeded-vs-unseeded comparisons in §RQ4.
If the compute budget does not allow 37 artifacts × 20 trials, repeat the cells the text
actually draws comparisons from and say so.

### W3. The 36 defects, the seven families, and the cluster→defect merge are single-rater judgments by the tool's authors, with no agreement statistic. (§RQ2, §RQ3, §7.2)
§7.2 concedes "The same researchers discovered and categorized the defects" and that "The 36
instances are not 36 independent mechanisms." But the mitigation offered is only "We separate
mechanism from symptom, preserve per-instance evidence, and count a defect by its source-level
faulty rewrite". There is no second independent coder, no κ, and no operational rule that
distinguishes one faulty rewrite from two — which matters most exactly where the counts are
largest (10 of the 36 defects are in a single cell, optipng × C2SaferRust). The same applies
to the promotion decision itself: "a correspondence that does not survive source-level review"
blocks promotion, but who reviewed, against what criterion, and with what agreement?

*Why it matters to me specifically:* when my group had to decide whether 300 functions were
vulnerable, two researchers labeled independently, we reported Cohen's κ for both labeling
steps against the Landis–Koch bands, we published every label with its natural-language
justification, and we stated the labelers' expertise. That is the standard I apply to my own
group and I will apply it here. A 36-item count is small enough that a full double-coding is
cheap.

*What would resolve it:* have a second rater — ideally one not involved in building the
pipeline — independently (a) confirm/reject each of the 36 promotions from the archived
evidence and (b) assign a primary family, then report Cohen's κ for both, resolve
disagreements by discussion, and publish the labels with justifications. Also state the
merge rule operationally ("two symptoms are one defect iff they trace to the same edited
source construct in the translated artifact") and report how many clusters merged into each
of the 36.

### W4. The headline "21 of 36 are invisible to crash fuzzing" is a classification, not a measurement, on 27 of the 36. (§Abstract, §RQ2, §Component Analysis, Table 8)
The abstract says 21 defects "silently change values or state and therefore provide no
failure signal to Rust-only crash fuzzing"; §RQ2 repeats "Rust-only crash fuzzing has no
failure signal for their normally terminating executions". The only place this is *measured*
is the component analysis, and there it rests on "the fixed nine-defect replay set" where
"process output under a silent driver recovers three" of nine. The archived manifest confirms
the asymmetry: 9 defects have measured observation channels and 27 are recorded as "not run".
The paper never tells the reader that 27 of 36 were not measured under the reduced-observation
configurations, nor how the 9 were chosen — and they are visibly a convenience set
(qsort, crc32, bzip2, cJSON, tulip) rather than a sample of the corpus.

Worse, the cheapest version of this experiment is already inside the pipeline and is not
reported: discovery *is* Rust-only fuzzing, and confirmation distinguishes
`confirmed_termination` from `confirmed_divergence`. The paper could state, for each of the
36, whether the Rust-only discovery phase alone produced a failure artifact at the same
budget. That is the true ablation of the paired oracle over the whole corpus, and it is
absent.

*What would resolve it:* a per-defect column in Table 4 or Table 5 recording whether the
Rust-only discovery phase alone terminated on that defect at the recorded budget, and a
restatement of the 58% claim scoped to whatever that measurement shows. If the 9-defect set
must remain, characterize how those 9 were selected and say explicitly that the other 27 were
not run.

### W5. The negative control is reported as a bare zero, with no denominator — even though the denominator exists in the artifact. (§RQ2, §4.3)
"No confirmed defect appears in the ten mechanical c2rust controls" is the load-bearing
evidence that the harness machinery does not manufacture differences, and it is stated
without any measure of exposure. A zero with no denominator has no power: it is consistent
with a clean control *and* with an under-tested one. The archived data contains the much
stronger statement the paper does not make — the c2rust controls produce 68,981 candidates
and 8,962 adjudicated records, of which **zero** are `confirmed_divergence` or
`confirmed_termination`, against 1,931 of 14,470 in the treatment arm. That is the result;
put it in the paper.

The exposure argument is also available and unmade: in most applications the control is
tested *harder* than the treatment (cJSON 81.2% vs 3.2% region, lil 87.2% vs 6.3%), which
makes the contrast conservative. The paper should say so rather than leaving a reviewer to
work it out from Table 6.

*What would resolve it:* add the control arm to Figure 2 as a second funnel, or at minimum
report (candidates, adjudicated, clean-C, confirmed) for the ten controls next to the same
four numbers for the treatment arm, plus one sentence on relative exposure.

### W6. Table 4 invites a tool-quality ranking that the evaluation cannot support, and the paper does not warn against it. (§RQ2, Table 4)
"Of the 36, 20 occur in C2SaferRust, 7 in Laertes, four each in PtrTrans and CROWN, and 1 in
SACTOR." Every reader will take that as a ranking of translator reliability. It is not one.
C2SaferRust contributes seven deeply-reached artifacts (Table 6: lil 47 built harnesses,
tulip 212, optipng 96) while SACTOR contributes two, one of which is a three-function qsort,
after "7 compilation failures" removed the rest. The defect counts are therefore confounded
with artifact availability and campaign reach by close to an order of magnitude, and §7.2's
promise to "avoid prevalence claims" is not carried into the text that presents these counts.

*What would resolve it:* normalize, or explicitly forbid the reading. Report defects per
*compared artifact* and per *built harness*, or state in the RQ2 text that the per-tool
counts are not comparable across tools because exposure differs by an order of magnitude and
name the confound. A one-sentence caveat in the Table 4 caption is the minimum.

### W7. The UB gate's false-negative rate is never examined, although it discards 75% of replayed candidates. (§4.4, §7.2, Fig. 2)
"A C sanitizer report prevents promotion to a translation defect." In the funnel,
13,515 → 3,365 discards 10,150 records as reference-associated. The threats section discusses
only the *opposite* error — "a silent sanitizer miss remains a possible attribution error",
i.e. admitting a non-defect. The direction that concerns me is not addressed at all: a C
execution can contain incidental UB that is unrelated to the operation producing the observed
difference, and this policy would discard a genuine translation defect in that case. Nothing
in the paper establishes that the sanitizer report and the diverging observation are on the
same operation, and no sample of the 10,150 exclusions was audited.

*What would resolve it:* draw a random sample (100 is plenty) of the reference-associated
exclusions, manually adjudicate whether the reported UB plausibly explains the specific
observed difference, and report the proportion that do not, with a confidence interval.
That converts "we gate on sanitizer reports" into "our gate has a measured false-negative
rate of x%".

### W8. The candidate sample is a deterministic per-boundary cap, not a sample that supports any inference. (§RQ2, Fig. 2 caption)
"Confirmation uses a deterministic per-boundary sample of 14,470" capped "at 200" per
boundary, and the caption correctly says "the sample does not estimate a corpus-wide
false-positive rate". I appreciate the honesty, but a stratified *random* sample with the same
budget would have cost nothing extra and would have licensed exactly the inference the paper
currently has to disclaim. The cap also biases the funnel proportions toward boundaries with
few candidates: one bzip2 boundary alone contributes 26,814 candidates of which 200 are
adjudicated.

*What would resolve it:* keep the cap for throughput, but additionally draw a small
proportional random sample across the 176,507 and report the funnel proportions with binomial
confidence intervals. Then the funnel is an estimate, not just an accounting.

### W9. Two of the five component-analysis rows are n = 1. (§Component Analysis, Table 8)
"Memory-UB attribution — urlparser, 1 witness" and "Source provenance — lil, 1 mismatch".
A row with one observation is an anecdote; five such rows in a table headed "Effect of
reducing or removing individual validation components" read as a systematic ablation. The
summary then generalizes: "These checks support the confirmation policy". The abstract goes
further — "Controlled component checks further show where weaker observation, C-reference
failures, ambiguous alignment, and instrumentation effects change the resulting evidence."
"Show" is too strong for n = 1.

*What would resolve it:* enlarge the evidence sets — for the UB-attribution row, the 10,150
reference-associated records give a natural population to sample from (see W7); for
provenance, run the reduced configuration across all cells and count admitted candidates.
If an enlargement is impossible, mark the row's n in the table and weaken the abstract to
"illustrate" rather than "show".

### W10. §RQ2 claims complete matrix coverage that Table 4 contradicts. (§RQ2, Table 4)
"We attempt the complete validation workflow in every cell of the ten-application by
six-system matrix." Table 4 contains six cells marked "--", defined in the caption only as
"unavailable", with no reason given anywhere in the text — quadtree, cJSON and lodepng ×
{Laertes, C2SaferRust}. Fourteen "×" cells *are* explained (the SACTOR analysis in §RQ2 is
good and I want more of exactly that), and the UB cells are explained. The six "--" cells are
not. Either the workflow was not attempted there, in which case the sentence is wrong, or it
was, in which case the outcome should be recorded like every other cell.

*What would resolve it:* give each "--" cell a reason code in the caption or text, in the
same style as the SACTOR failure analysis.

### W11. RQ1's ground truth in Panel (a) is name equality, which is the very signal the matcher claims independence from. (§RQ1, Table 2)
"C2Rust, Laertes, CROWN, and C2SaferRust preserve C function names. Their names reveal which
functions originated from one another, but we hide the names from the matcher before
evaluation." Hiding names from the matcher is right. But the *truth* is then defined by the
translator's own naming, which assumes these four systems never merge, split, or repurpose a
function while keeping a name. C2SaferRust performs slice-level rewriting; that assumption is
not obviously safe, and it is not validated. Panel (b) is manually labeled, which is the
correct treatment — Panel (a), which carries the headline 0.938 over 4,041 pairs, is not.
Relatedly, ε = 0.01 is described as "the preselected abstention threshold" with no statement
of how it was selected or on what data; if it was tuned on this corpus, the 0.993 operating
point is optimistic.

*What would resolve it:* manually audit a random sample of the name-derived ground truth
(say 100 pairs across the four systems) and report the agreement between name-equality truth
and manual truth; and state where ε came from — a held-out set, a pilot artifact, or an a
priori choice.

### W12. It is unclear whether the 36 defects were actually discovered through the frozen automatic matcher. (§RQ2, §Component Analysis, §7.2)
§RQ2 asserts the protocol: "We freeze the automatic matcher configuration before execution and
pass its accepted pairs directly to harness planning; abstained pairs remain untested, and we
do not manually repair its proposals before running a campaign." But other passages describe
runs that do not fit this: the component analysis row says "Name equality misses PtrTrans's
renamed qsort API boundary", and §7.2 says "the qsort pilot tests name equality rather than the
tool map". If some cells in Table 4 were produced under name equality or an earlier matcher
configuration, then the RQ1 operating point (which abstains on 21.6% of pairs, mostly in
Tulip) does not describe the configuration that produced the RQ2 results, and the two RQs are
not composable.

*What would resolve it:* state per cell, or at least per system, which matcher configuration
produced the campaign, and confirm that every one of the 36 defects is reachable from a
boundary the frozen ε = 0.01 matcher accepts. If any defect required a boundary the frozen
matcher abstains on, say so and count it.

### W13. RQ4 states no claim that could have come out the other way. (§RQ4)
"Importantly, campaign reach is a property of the interaction between a generated campaign and
a particular translated artifact; it is not, by itself, a score for the harness generator."
I agree with the caution, but after it, RQ4 has no hypothesis: it reports a range
(8.3–100% functions, 2.8–100% regions) and then explains each low cell individually. Those
explanations are the best part of the section and they are n = 1 narratives. As written, RQ4
cannot fail. If the intended claim is "low reach is usually attributable to construction,
reference, or translation limits rather than to shallow exploration", then state it and report
the proportion of low-reach cells for which side-isolated replay established each cause —
that is a claim with a denominator and a possible negative answer.

### W14. The ε = 0.01 operating point loses boundaries, and the cost is stated but not carried into RQ2. (§RQ1)
"This operating point accepts 78.4% of pairs with 0.993 macro-average precision... Most
abstentions occur in Tulip's structurally indistinguishable indicator families, showing the
boundary coverage traded for the higher accepted-pair precision." Yet Table 6 reports Tulip
at 213/213/212 matched/planned/built for three of four artifacts — i.e. essentially every
boundary was planned. Those two statements are hard to reconcile, and the reader cannot tell
which configuration Table 6 reflects. (This is the same concern as W12 from a different
direction.)

---

## 5. Detailed comments per section

**§1 Introduction.** The four-obligation framing is the right abstraction and is stated
crisply. The closing disclaimer that was cut from the submitted version ("We do not claim to
be the first C-to-Rust differential validator...") is, in my view, worth restoring in some
form: the paper is stronger when the novelty claim is narrow and explicit, and §2/§3 already
behave that way. One framing issue: "A wrong decision causes systematic false negatives or
false positives rather than merely reducing fuzzing coverage" (abstract) is the paper's
strongest motivating claim and it is never measured — see W1 and W7. Either measure one of
the two directions or soften the sentence.

**§2 Motivating Study.** Three well-chosen cases, and Table 1 is a good device: it maps each
acceptance signal to each case and the "No value" cell for qsort makes the point better than
prose would. The section does what a running example should — motivate, not substantiate —
which is the right use. My only content concern is that Table 1 is a claim about what these
*signals* can see, and two of the three rows ("Rust-only crash or panic", "C–Rust return and
termination") are asserted for the exact executions rather than shown; since the executions
exist, the cells could each be backed by a recorded run.

**§3 Related Work.** This is a strong section, and unusually honest about what is and is not
new: "These systems establish that differential or relational validation is valuable; our
claim is not that such validation is new", and the repeated framing of the planner as "a
deliberately bounded, definition-driven specialization" of FuzzGen/UTOPIA/GraphFuzz/Hopper.
The argument for why C2Rust cross-checker, cozy and DIFFER are not usable as baselines —
"using our generated harnesses would import the component being evaluated" — is correct and
well stated. §3.3's ordering argument (attribution before deduplication) is the paper's best
conceptual contribution and deserves more prominence than a related-work subsection.
Note that the referenced Table `tab:validator-scope` does not appear in the built document, so
the claims in the paragraph beginning "Tulip exposes an incorrect return value..." currently
have no table behind them.

**§4 Design and Implementation.** §4.2's abstention margin is well defined and the two-sided
formulation is the right one. §4.3 is the most convincing part of the method: the bzip2
`BZ2_bzReadOpen` vs `BZ2_bzBuffToBuffDecompress` contrast explains the eligibility criterion
better than a definition could, and "the plan records the construction failure rather than
inserting a guessed constant" is the correct discipline. The producer-bridge description is
clear about its bound ("at most a producer–target–destructor sequence"). §4.4 is careful.
Two gaps: (i) the comparison ladder "does not directly match arbitrary global variables"
(§7.2) — given that initialization loss is the largest defect family (9 of 36), the paper
should say what fraction of that family was observable only because it happened to propagate
to a harness-owned buffer, and what a globals-aware ladder would add; (ii) the optional
comparator plugin is described as "untrusted, translation-layout-dependent code" that "extends
only output observation" — how many of the 36 defects required a plugin? If any did, the
corresponding oracle strength should be attached to those defects in Table 4 or 5.

**§5 Study Subjects.** The "Stochastic translation outputs" paragraph is exactly right and I
want to commend it specifically: freezing the source hash, refusing best-of-n selection, and
stating that "our results therefore characterize the studied artifacts produced by each
system rather than run-to-run model variance" is the correct experimental unit for
LLM-in-the-loop systems, and most papers in this space get it wrong. It also means Table 4's
per-tool counts are single-draw observations of a stochastic generator, which reinforces W6.

**§6 Evaluation.** RQ1: see W11, W14. The Tulip failure analysis ("Its 213 functions collapse
to only 103 static fingerprints, with 120 functions in a non-singleton group... leaving no
name-independent structural signal that distinguishes one member of a group from another") is
an excellent negative result, reported at full strength with its mechanism — this is the
habit I look for. RQ2: see W3–W6, W10, W12. The SACTOR compilation-failure analysis is the
right level of diagnosis and should be the template for the "--" cells. RQ3: see W3; the
taxonomy is plausible and the two case studies are well chosen, particularly the observation
that the same family produces a crash in bzip2 and silent corruption in the CRC table —
"The shared family therefore records the lost initialization, not a particular symptom" is
the correct counting discipline. RQ4: see W2, W13. RQ5: the funnel design is right; see the
questions below about the arithmetic. Component analysis: see W4, W9.

**§7 Discussion.** The threats section is better than most: it is organized, each threat names
a mitigation, and several of the mitigations were actually performed (retaining failed and
UB-excluded cells; counting a defect by its source-level rewrite; archiving adapters). Two
gaps relative to what I expect: there is no **conclusion-validity** paragraph at all — the
paper's conclusions rest on single-run measurements and on counts whose unit is defined by the
authors, and both belong here — and the **sanitizer-completeness** paragraph covers only one
of the two error directions (W7). The "Alignment truth" paragraph is candid about the
unmeasured lodepng map ("whether those entries manufacture false divergences or hide defects
downstream remains unmeasured"), which I appreciate.

---

## 6. Questions for the authors

1. **Recall.** Can you provide any denominator for the 36? Concretely: if you inject one
   synthetic fault per Table 5 family into artifacts your pipeline already builds, using an
   independent validity oracle, what recall do you measure, and which families do you miss?
2. **Rust-only baseline over the whole corpus.** Your discovery phase is Rust-only fuzzing and
   your confirmation distinguishes `confirmed_termination` from `confirmed_divergence`. For
   each of the 36 defects, did the Rust-only discovery phase alone produce a failure artifact
   at the recorded budget? Please give that as a 36-row answer; it is the direct measurement
   of the abstract's "no failure signal to Rust-only crash fuzzing" claim.
3. **Control denominator.** How many candidates and adjudicated records did the ten c2rust
   controls produce, and how many reached the clean-C and confirmed-divergence stages? Why is
   the control arm absent from Figure 2?
4. **Second rater.** Were the 36 promotions and the seven family assignments coded by more
   than one person? If so, what is the agreement (κ) and how were disagreements resolved? If
   not, are you able to add an independent coder for the camera-ready?
5. **Defect counting rule.** State the operational rule for "one faulty rewrite". For
   optipng × C2SaferRust (10 defects) and bzip2 (8 across three systems), how many confirmed
   clusters merged into each reported defect, and what would the count be under a stricter
   rule (one defect per edited source construct) and a looser one (one defect per boundary)?
6. **UB-gate false negatives.** Of the 10,150 reference-associated exclusions, what fraction
   would survive a manual check that the reported C sanitizer error actually explains the
   specific observed difference? A 100-record sample with a confidence interval would settle
   this.
7. **Matcher configuration in RQ2.** Was every one of the 36 defects discovered from a
   boundary accepted by the frozen ε = 0.01 matcher, with no name-equality fallback and no
   manual repair? If not, which cells used a different configuration, and how many defects
   would be lost under the frozen configuration alone?
8. **ε provenance.** How was ε = 0.01 chosen, and on what data? Was any part of the corpus in
   Table 2 used to select it?
9. **Name-equality ground truth.** Have you audited any sample of Panel (a)'s name-derived
   ground truth against manual inspection? What is the disagreement rate, particularly for
   C2SaferRust, which rewrites at slice level?
10. **RQ4 repetitions.** How many trials underlie each cell of Table 6, and what is the
    run-to-run spread? Specifically for the Tulip seed comparison (90.2–93.2% seeded vs 38.7%
    random control), how many repetitions, and what is the effect size?
11. **Table 7 arithmetic.** FLOURINE shows S = 33, A = 20, A-fail = 14. 33 − 14 = 19, not 20.
    Please reconcile the funnel columns, or define precisely which stage each column counts.
12. **The six "--" cells.** Why were quadtree, cJSON and lodepng × {Laertes, C2SaferRust}
    unavailable, and how does that square with "We attempt the complete validation workflow in
    every cell"?
13. **Globals and plugins.** Of the 9 initialization-loss defects, how many were observable
    only because the lost state propagated into a harness-owned buffer, and how many of the 36
    required the optional comparator plugin?
14. **Budget parity with the controls.** For each application, were the c2rust control and the
    restructuring artifacts given the same wall-clock budget, the same number of built
    harnesses, and the same corpus-replay sample cap? Table 6 suggests the control is often
    reached more deeply, which is conservative — if so, say it explicitly.

---

## 7. What would move this up one grade

A **Weak Accept** needs three of the following, and I would say W1, W3 and W5 are the ones
that matter most. First, any recall denominator for the validator itself — a fault-injection
study with a stated denominator and a characterized miss class would do it, and it does not
need to be large to change the paper's standing. Second, double-coding of the 36 promotions
and seven family assignments with a reported κ and published per-defect justifications;
with 36 items this is days of work, not months, and it converts the corpus from an
authors' count into a reviewable dataset. Third, the control arm added to Figure 2 with its
candidate and adjudication denominators, plus one sentence establishing that the control was
exercised at least as hard as the treatment. A **Weak Accept** also requires that the 58%
crash-blind claim be scoped to what was measured, and that Table 4's per-tool counts carry an
explicit non-comparability caveat.

For **Accept**, add repeated campaigns with dispersion and effect sizes for every comparison
RQ4 draws (W2), and an audited sample of the reference-associated exclusions establishing the
UB gate's false-negative rate (W7). With those, the paper would be making claims exactly as
strong as its measurements, which — given how good the confirmation design and the funnel
accounting already are — is well within reach.

---

## 8. Self-disclosure

Weaknesses W1–W5, W7–W9 and W11, and the demands for κ, effect sizes, repeated trials,
control denominators, ablation evidence sets and construct-validity of the ground truth follow
directly from the profiled reviewer's published standards and documented review criteria
(double-coded labeling with κ and published justifications in ISSTA'25; 30 trials with
Mann-Whitney and Vargha-Delaney in ICSE'26; split-half reliability and "does the metric
measure the construct" in FSE'26; his stated registered-report criterion of judging
methodology independently of results); W6, W10, W12–W14, the RQ-by-RQ comments and the
grade thresholds are my own inference about how he would apply those standards to this
specific paper, and the decision itself is my inference, not his.
