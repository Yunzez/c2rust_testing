# FSE 2027 Review — "When Is a Difference a Defect? Differential Validation of C-to-Rust Translations"

Reviewed in the persona and standards of **Manuel Rigger** (NUS; FSE 2027 Research Papers PC),
using `docs/reviews_fse27/pc_rigger.md` as the rubric.
Scope per the authors: content only. Citation style, LaTeX, grammar and wording are ignored.

---

## 1. Paper summary

The paper argues that differential testing of C-to-Rust translations is not a free oracle: before a
C/Rust behavioral disagreement can be called a translation defect, a validator must decide which
functions correspond, how one logical input is realized across two possibly reshaped interfaces,
which observable state carries the functional contract, and whether the disagreement is attributable
to the translation rather than to undefined behavior in the C reference, an out-of-contract input,
or the instrumentation. The authors build a validator around these four obligations: a
name-independent structural matcher with an abstention margin, a deterministic definition-derived
`HarnessPlan` that either materializes one logical input on both sides or records an explicit
construction failure, coverage-guided Rust-only discovery followed by paired replay through a fixed
comparison ladder, and a side-isolated confirmation stage (C-only ASan+UBSan, combined replay,
Rust-only with and without sanitizers) that precedes clustering and manual source-level triage.
They evaluate on ten C applications × six translation systems, reporting 0.938 matcher precision
with names hidden, 36 confirmed defects in 15 artifacts across the five restructuring systems (15
crash/panic, 21 silent value/state), zero confirmed defects in the ten mechanical C2Rust controls,
seven root-cause mechanism families, campaign reach for 37 artifacts with per-cell attribution of
low reach, and a released-artifact study in which FLOURINE detects 14 of its 16 completed analyses
and RustAssure 5 of 11, while VERT's released workflow accepts no external candidate.

---

## 2. Overall decision

**Weak Reject.**

**Reviewer expertise:** Expert (differential testing, test-oracle design, coverage-guided fuzzing,
sanitizer-based definedness checking; I have built and evaluated oracle-plus-confirmation pipelines
of this shape).
**Confidence:** High on evaluation design, oracle qualification, defect accounting and baseline
methodology. Medium on C-to-Rust translator internals and on the specific semantics of the five
restructuring systems.

I want to be explicit about why this is Weak Reject and not Reject: the framing is the right one,
the abstention and attribution discipline is better than almost anything I have seen in this
subarea, and most of what I am asking for is recoverable from the authors' own archived data rather
than requiring new campaigns. What blocks acceptance is that the paper's central number — 36
confirmed defects — is produced, adjudicated, counted and validated entirely inside the authors'
own pipeline, and that the pipeline's own accounting figure does not in fact account for all 36.

---

## 3. Strengths

**S1. The problem is the right problem, and it is stated as an oracle-qualification problem rather
than a tooling problem.** §1 and §4.5: "A replay difference or Rust termination is a candidate, not
yet a translation defect." §8.1 states plainly that "sanitizer silence does not establish defined
behavior," and §3.3 cites UBFuzz for sanitizer false negatives. Table 4's caption says "'Clean C'
means that no configured check fired, not that defined behavior has been proved." This is exactly
the epistemic care a differential paper needs and usually lacks; the paper never claims to prove
equivalence and never lets a clean sanitizer run masquerade as a definedness proof.

**S2. Abstention and construction failure are first-class outputs, not silently dropped.** §4.3
reports a pair as non-harnessable with a specific reason (the `BZ2_bzReadOpen` `FILE *` example is
well chosen), and §8.1 states that such boundaries "remain explicit construction failures rather
than disappearing from the denominator." Table 6 reports matched/planned/built (4,222/1,854/1,662),
so the reader can see that 56% of matched boundaries never receive a plan. Most papers in this space
would report only the boundaries that worked.

**S3. The C2Rust negative control is a genuine, pre-registered control with a stated role.** §4.3
frames it operationally ("a divergence in a C2Rust control is treated as a harness-qualification
failure that must be resolved before downstream differences are interpreted"), explicitly refuses
the stronger claim ("not an assumption that C2Rust is universally semantics preserving"), and the
argument that three of the five restructuring systems consume C2Rust output is the right reason for
choosing this particular control.

**S4. RQ4 explains its numbers instead of reporting them.** The construction-/translation-/
reference-/domain-limited decomposition is the best section in the paper. The cJSON × PtrTrans
diagnosis is exemplary: rather than reporting 3.2% region reach as a tool weakness, the authors show
`cJSON_New_Item` is an untranslated stub returning `None` on all 18 direct-constructor witnesses and
conclude that "adding an `Option<&mut T>` adapter or running longer could not unlock the 64
object-taking boundaries." Same for lil × C2SaferRust (57 termination witnesses localized to one
site, 18 dependent boundaries blocked). This is the kind of causal accounting I want to see under
every low number in a coverage table.

**S5. The baseline funnel separates tool failure from semantic miss.** Table 7 splits S/A/C/R/D with
separate Unsup., A-fail and C-fail columns, and the caption states "A defect is a miss only when the
released artifact completed its analysis without exposing it." Reporting FLOURINE as 14/16-completed
rather than 14/36 is the honest presentation, and the VERT row is accompanied by an explicit
statement that this is "an applicability boundary for our fixed-artifact question, not a semantic
miss."

**S6. The two RQ3 case studies are concrete, mechanistic and checkable.** The zlib case
(`state->offset > copy` becoming `state as usize > copy`, comparing a heap address with a byte count,
169 confirmed output-length divergences plus four null-window dereferences) is a genuinely good bug,
and the observation that the translation "even annotates the generated line as a pointer comparison"
makes the root cause unambiguous. The bzip2 `incs` case is well paired with it because the same
family produces a crash rather than silent corruption.

**S7. The threats section names real threats rather than boilerplate.** §8.2 discloses that the
corpus "was frozen after exploratory work, not sampled prospectively" and that "earlier notes
explicitly allowed replacement subjects"; that "the same researchers discovered and categorized the
defects"; that "the 36 instances are not 36 independent mechanisms"; and that the PtrTrans lodepng
map's effect "remains unmeasured" because the artifact does not build. I credit this. It is also why
I believe the remaining gaps are fixable rather than concealed.

---

## 4. Weaknesses, ranked by decision weight

### W1. Fourteen of the 36 headline defects do not flow through the confirmation pipeline the paper presents as producing them. (Figure 2 / §6.2 / `figure/confirmation_funnel.py`) — decisive

§6.2 presents one continuous accounting: "the archived campaigns save 176,507 candidate inputs.
Confirmation uses a deterministic per-boundary sample of 14,470: combined replay retains 13,515
candidates, of which 3,365 have clean C-only replay... Side-isolated confirmation retains 1,931
confirmed value/state or termination divergence records... Source-level triage merges repeated
inputs, boundaries, and symptoms caused by the same faulty rewrite into **the 36 defects reported
here**." Figure 2 draws that chain and prints a defect count at its right-hand end.

The figure's own generator contradicts this. `figure/confirmation_funnel.py` sets `TRACED = False`
so the last column prints "the manifest totals 20/4/7/1/4 = 36", and its header comment states:

> `TRACED = True` prints, per tool, the manifest defects that these sampled records manifest
> (**22 of 36**). The other 14 manifest entries rest on evidence outside this sample: C7, S10, S3 in
> the full (unsampled) confirmation of the bzip2 CROWN / Laertes cells; C1-C6, S12, S13
> (C2SaferRust) and S7-S9 (PtrTrans) in **the earlier driver-level campaigns**.

So the figure as published attaches a 36 to a funnel that accounts for 22, and eleven of the
remaining defects rest on an "earlier driver-level campaign" that the paper never describes. §4 does
not mention driver-level campaigns at all; the only trace of them in the paper is the "Driver
externalization / Silent stdout/exit" row of Table 8 and the phrase "process output under a silent
driver" in §6.6. The defect manifest confirms the shape of this: 27 of the 36 defects have no
observation cell and record every observation channel as "not run"; C2, C7 and S12 have
`obs_cell: None`.

Why this matters to me specifically: the paper's title and thesis are about *when a difference is a
defect*, and the confirmation funnel is the paper's entire quantitative answer. If a third of the
defects were established by a different, undescribed procedure, then the paper has not shown that
its described procedure produced its headline result. This is not a presentation quibble — it is the
load-bearing claim.

*What would resolve it:* either (a) re-run the described pipeline so that all 36 defects are
witnessed by it and redraw the funnel with a traced defect count, or (b) state explicitly in §6.2
and the caption how many defects each evidence channel established, describe the driver-level
campaign in §4 as part of the method, and report the funnel's defect column as 22 with the other 14
accounted for separately. Option (b) is cheap and I would accept it.

### W2. Not one defect was reported to the translators' authors, and no duplicate check against known limitations is described. (§6.2, §8.2) — decisive

Searching the paper for any reporting activity returns nothing: no issue-tracker link, no
"confirmed by the developers", no "fixed", no statement of a reporting policy, no check that a
defect was not already known to the translator's own authors (several of these systems ship papers
that document test failures and unsupported constructs). "Confirmed" in this paper means confirmed
by the authors' pipeline plus the authors' own source review, and §8.2 concedes that "the same
researchers discovered and categorized the defects."

For me this is the single strongest evidence standard in bug-finding work, and its complete absence
converts the headline from "36 defects" into "36 author-adjudicated findings". I recognize the
setting differs from testing a maintained DBMS — several subjects are frozen research artifacts
whose authors may not fix anything. That is an acceptable answer, but it has to be *given*: say how
many defects were reported, to whom, what came back, and where you deliberately did not report
because the artifact is unmaintained. Reporting even a handful — the zlib `state as usize > copy`
bug in C2SaferRust, the Laertes severed-initializer family — and getting one acknowledgement would
change the evidentiary character of the whole paper. Absent that, I also cannot rule out that some
of the 36 are already-known limitations of the respective systems.

*What would resolve it:* a paragraph in §6.2 with a reporting/duplicate-check policy, per-system
counts of reported / acknowledged / fixed / not reported (with the reason), and links; plus a check
of each defect against the corresponding system's published limitations.

### W3. The comparison is one-directional, on a corpus the proposed tool defined, and there is no configuration in which the proposed validator can lose. (§6.5, Table 7) — decisive

RQ5 runs FLOURINE, RustAssure and VERT on "the same 36 frozen defective translations". §6.5 states
the limitation honestly ("Because the corpus was discovered by our validator, this RQ measures
applicability and positive-case sensitivity on these known defects, not general recall, precision,
or relative bug-finding ability"). But an honest disclaimer is not a substitute for the missing
experiment. Three consequences:

1. **No reverse direction.** The paper never runs FLOURINE or RustAssure on the 37 artifacts
   end-to-end and asks what *they* find that is not in the 36-defect corpus. Without that, the
   reader cannot distinguish "our validator is stronger" from "our validator defined the target".
2. **The headline conditional rates rest on small self-selected subsets.** Table 7: FLOURINE
   completes 16 of 36 (A-fail 14, C-fail 3, Unsup. 3); RustAssure completes 11 of 36 (C-fail 22).
   "14 of 16" and "5 of 11" are computed on 44% and 31% of the corpus, and the subsets are selected
   by the baselines' own failure modes, which is the worst possible selection mechanism for a
   sensitivity estimate. The abstract leads with these ratios.
3. **No evidence the baselines were run in good faith.** More than half of each baseline's funnel is
   tool failure. §6.5 says "We do not patch baseline analysis, generation, comparison, or search
   logic; adapters only package the existing pair" — which is the right principle — but there is no
   statement that the baselines' authors were contacted about the A-fail/C-fail cases, no account of
   what the failures were, and no sanity check that the adapters are not themselves the cause. In my
   own work I would state that I contacted the authors and what they said, precisely so a reviewer
   can separate "the artifact is brittle" from "the evaluator used it wrong."

I also disagree with excluding DIFFER, cozy and the C2Rust cross-checker on the grounds that "using
our generated harnesses would import the component being evaluated" (§3.1, §6.5). That is exactly
the experiment that would isolate this paper's contribution: hand all three the *same* harness and
show that the confirmation/attribution stage is what changes the verdict, or hand a hand-written
harness to both yours and theirs. The current exclusion protects the contribution from being
isolated rather than isolating it.

*What would resolve it:* (a) the reverse direction on at least the subset of artifacts each baseline
accepts, reporting defects they find that you do not; (b) a statement that the baselines' authors
were contacted about the failures; (c) at least one shared-harness comparison against DIFFER or the
C2Rust cross-checker.

### W4. The paper never reports the precision of the pipeline it is about. (§6.2, Figure 2, §6.6)

§6.2 explicitly declines a rate: "These reductions are candidate accounting, not an estimated
false-positive rate, because confirmation caps each boundary's sample at 200." I accept that a
capped, non-random 14,470-of-176,507 sample cannot estimate a corpus-wide FP rate. But that is not
the number I want. I want: **of the 1,931 records that the automated confirmation stage promoted to
"confirmed divergence", how many survived source-level triage as translation defects and how many
were rejected — and for what reasons?** §6.2 says "A correspondence error is an alignment failure
rather than a translation defect", which implies some were rejected, and the archived manifest
carries five records still marked "candidate — untriaged". That rejection count is a clean,
sample-independent measure of the automated oracle's precision on exactly the evidence it produced,
and it is the number a paper titled "When Is a Difference a Defect?" owes the reader. Right now the
only false-positive evidence in the paper is two n=1 anecdotes in Table 8.

*What would resolve it:* report promoted / rejected / untriaged over the 1,931 confirmed records,
with a breakdown of rejection reasons (alignment, provenance, contract, other).

### W5. The clustering step is described, positioned against prior work, and never evaluated. (§4.5, §5, §3.3, Figure 2)

§4.5: "Candidates are grouped by attributed outcome, sanitizer class, and top failure site." §5:
"we follow FuzzerAid and Igor's principle that many artifacts need not represent many bugs." §3.3
devotes a whole subsection to crash-triage literature. Yet the evaluation contains no measurement of
the clustering at all: no over-/under-merge analysis, no comparison against a stack-hash baseline,
no cluster count in the text. Cluster counts appear only as unlabeled node values in Figure 2's
generator (C2SaferRust 38 clusters → 20 defects; Laertes 16 → 7; PtrTrans 9 → 4; CROWN 2 → 4;
SACTOR 3 → 1), and the CROWN row is the interesting one: two automatic clusters correspond to four
defects, i.e. the automatic grouping under-merges in one direction and over-merges in another, and
the paper never discusses it.

Deduplication with a defensible ground truth is something I consider mandatory in bug-finding work,
and it is cheap here: the authors have per-defect root causes, so they can measure cluster purity
and completeness against their own triage labels.

*What would resolve it:* a small table of clusters → defects per system with purity/completeness
against the manual root-cause labels, and a comparison against grouping by failure site alone.

### W6. Everything is a single 1-hour run with a single seed; the zeros that carry the negative-control claim have no repetition. (§6.4, §8.2, Table 3)

§6.4: "The standard campaign runs for one hour." §8.2 defends this as "one fixed-budget observation
of one frozen artifact, not an estimate of expected fuzzer performance." I understand the argument
for the *positive* findings: a defect confirmed by deterministic replay does not need repetition.
But the argument does not transfer to the *negative* cells, and the paper leans on them heavily: 11
restructuring cells and all 10 control cells are reported as "0 = no confirmed divergence found at
the recorded budget", and the abstract elevates this to "No confirmed defect appears in the ten
mechanical c2rust controls." A zero from one seed and one hour is weak evidence of absence, and the
community standard for this kind of claim (Klees et al., which this line of work routinely cites)
is repeated runs at a longer budget.

*What would resolve it:* ≥5 seeds per cell for the zero cells, or a 24-hour run for the ten controls
and the eleven restructuring zeros, reporting whether any cell changes verdict. At minimum, report
saturation evidence (time-to-last-new-confirmed-divergence per cell).

### W7. The negative control is both under-reported and un-powered. (§4.3, §6.2)

Two opposite problems in the same claim.

*Under-reported:* the paper's control claim is "no confirmed defect", but the archived data support
a much stronger and more informative statement — the ten controls produced 68,981 candidates, of
which 8,962 were adjudicated, 8,527 reproduced under combined replay, 371 had clean C-only replay,
and **zero** reached a confirmed value/state or termination divergence. That "0 of 8,962" is a far
better negative-control result than "no confirmed defect" and the paper leaves it on the table.

*Un-powered:* nothing in the paper establishes that the control *could* have produced a false
positive. The control's power is not quantified, and the comparison is not conditioned on reach —
the paper contrasts "15 of the 25 restructuring artifacts" with "none of the ten C2Rust controls"
even though per-cell reach differs by an order of magnitude (Table 6: urlparser 9.2% region vs.
qsort 100%). The obvious strengthening is a positive control: inject known semantic-changing
rewrites into the C2Rust outputs and measure how many the pipeline flags at the same boundaries,
turning "we observed none" into "we would have detected X of Y". A seeded-defect study is standard
for exactly this purpose and is absent.

*What would resolve it:* report the 0/8,962 figure; add a mutation/injection study over the C2Rust
controls; and present at least one matched-reach control contrast (optipng × C2Rust vs. optipng ×
C2SaferRust are both at 26.3% region coverage, which is the comparison you want to make).

### W8. The component analysis — the only ablation — is n=9 with an unstated selection rule, and three of its five rows are n=1. (§6.6, Table 8)

§6.6 leads with "Observation has the largest measured effect on defect recovery. On the fixed
nine-defect replay set, normalized function state recovers all nine defects, whereas return values
recover six and process output under a silent driver recovers three." This is the measurement that
justifies obligation #3 (observe contract-relevant state), which is arguably the paper's most
generalizable claim — and it rests on 9 of 36 defects. The paper never says why those nine. The
manifest shows the reason: exactly 9 defects have an observation cell with the channels actually
run, drawn from 6 cells (two share `tulip_c2saferrust`, three share `cjson_ptrtrans`) and 3 of the 5
restructuring systems. So the effective independent sample is 6 cells, not 9 defects, and the
selection rule is "wherever we happened to run the channel replay".

The other rows are worse as measurements: "Function alignment: qsort, 3 true pairs", "Memory-UB
attribution: urlparser, 1 witness", "Source provenance: lil, 1 mismatch". These are illustrative
anecdotes, and presenting them in a table with a "Consequence" column ("1 false candidate survives")
gives them the appearance of measurement. The caption's warning that rows "must not be pooled" is
correct but does not fix the underlying n.

The frustrating part is that the full measurement appears to be mechanically available: the inputs
are archived, so all 36 defects could be replayed through the reduced observation ladders.

*What would resolve it:* replay all 36 (or state precisely why a defect cannot be replayed through a
reduced ladder and report the reduced denominator with the rule); demote the n=1 rows to prose
examples or expand them to every witness of that class in the corpus.

### W9. RQ1's headline precision uses the aggregation that flatters it, and recall is never reported. (§6.1, Table 5)

Panel (a)'s 0.938 is an unweighted mean over ten applications; the footnote concedes that the pair
count "is evidence volume, not the denominator of the macro-average." Weighting by the 4,041
ground-truth pair instances the table itself reports gives roughly **0.874**, because the one
catastrophic cell (Tulip, 0.554, "the same 95 assignments are wrong") carries 852 pairs and the
perfect cells carry 9–58. Both numbers are defensible; reporting only the higher one in the abstract
of the section is not. Two further gaps: (i) recall against ground truth is never reported for
panel (a) — "78.4% of pairs accepted with 0.993 macro-average precision" is an acceptance rate at
one operating point, not recall; (ii) ε = 0.01 is asserted as "preselected" with no sensitivity
curve, so the reader cannot see the precision/coverage trade-off the threshold controls, and cannot
tell whether 0.993 is a property of the matcher or of a well-chosen ε.

*What would resolve it:* report macro and micro precision side by side, report recall, and add a
precision-vs-accepted-pairs curve over ε with the chosen operating point marked.

### W10. One application is evaluated under a different, hand-tuned input policy than the other nine. (§6.4, Table 6 caption)

Table 6's caption: "Tulip reports the deterministic seed-refined campaign described in RQ4." §6.4:
"we retain identical pseudorandom bytes for the large input tables and sample every option from the
fixed set {1, 2, 3, 5, 10, 20}", which moves Tulip from 24.0–34.7% to 90.2–93.2% region coverage.
The length-matched random control (38.7%) and the unseeded 600-second control are good and I credit
them. But the main reach table now mixes two protocols, and the paper does not report what the other
nine applications would do under an equivalent per-application seeding effort, nor whether Tulip's
defect count (4 defects) changed under the refined campaign. If per-application seed engineering
lifts coverage by 2.6×, then Table 6's cross-application numbers are not comparable, and RQ2's zeros
for the unseeded applications are correspondingly weaker.

*What would resolve it:* report Tulip under the uniform protocol in Table 6 and the seeded campaign
as a clearly separated ablation, or apply an equivalent seeding policy to all ten applications;
either way, state whether seeding changed any RQ2 verdict.

### W11. Defect individuation is author-defined, undisclosed in its rule, and heavily concentrated. (§6.2, §6.3, Table 4)

"Multiple inputs that exercise the same faulty rewrite count as one defect" (§6.2) is the right
principle, but the unit is "faulty rewrite in one artifact", which is not obviously the unit a reader
assumes. Consequences: 20 of the 36 come from one system (C2SaferRust) and 10 from a single cell
(optipng × C2SaferRust); all seven Laertes defects belong to one mechanism family (§6.3: "All seven
Laertes defects involve initialization") and, on the paper's own description, arise from the same
systematic behavior — moving static initializers into generated helpers that are never called. Under
a per-translator root-cause unit, that is plausibly one translator defect with seven manifestations,
which is exactly the distinction I care about. §8.2 concedes "The 36 instances are not 36 independent
mechanisms", but the abstract, intro, RQ2 and conclusion all lead with 36.

*What would resolve it:* report the count at both granularities — per-artifact faulty rewrites (36)
and per-translator distinct root causes — and state the individuation rule where the number is first
used, not only in threats.

### W12. Small but checkable accounting inconsistency in RQ2's artifact counts. (§6.2, Table 3)

§6.2 states: "this occurs in all ten c2rust controls and **ten** artifacts produced by the
restructuring tools", and separately "**15 of the 25** restructuring artifacts on which a comparison
was performed contain at least one". Counting the `0` entries in the five restructuring columns of
Table 3 gives **eleven**, not ten (qsort: Laertes, CROWN, SACTOR; quadtree: CROWN, PtrTrans; genann:
Laertes, C2SaferRust, CROWN; lil: Laertes; lodepng: CROWN; tulip: CROWN), so 15 defect-bearing + 11
zero cells = **26** comparisons performed, not 25. Either the table or the prose is off by one.
Small, but this is the arithmetic behind the negative-control contrast and a reader will check it.

### W13. "Recurring mechanisms" overstates what the taxonomy shows. (Abstract, §6.3, Table 4)

The abstract says the defects are "grouped into seven recurring mechanisms". Table 4 shows three of
the seven families occur in exactly one system each (control-flow preservation failure: C2SaferRust
only; ownership-state corruption: CROWN only; interface-contract loss: PtrTrans only), and §6.3
itself says "The families also form clear strategy-specific signatures in this corpus" — which is
close to the opposite of recurring. With 36 instances, no prevalence claim, and a corpus frozen after
exploratory work (§8.2), "recurring" is doing more work than the data supports.

### W14. Two claims in the abstract and §6.2 are asserted rather than measured, although the measurement is in hand.

- "21 silently change values or state and therefore provide no failure signal to Rust-only crash
  fuzzing" (abstract; §6.2: "Rust-only crash fuzzing has no failure signal for their normally
  terminating executions"). The discovery stage *is* Rust-only fuzzing (§4.4), so this is directly
  measurable and should be stated as a measurement: the Rust-only discovery stage flagged 0 of the
  21. As written it reads as an inference from the classification.
- The complement is missing and it is the honest part: of the 15 crash/panic defects, how many did
  the Rust-only discovery stage flag on its own, and how many of *its* terminations were later
  attributed to reference UB, out-of-contract inputs or instrumentation? That is the cheapest and
  most informative baseline in the paper — "Rust-only crash fuzzing with our harnesses finds N of 36
  at a false-report rate of M" — it is fully derivable from the archived confirmation data, and its
  absence is conspicuous because it is the one comparison where the paper would partly lose.

---

## 5. Detailed comments per section

**§1 Introduction.** The four obligations are a good organizing device and the qsort example earns
its place. Two content issues. First, the contribution list claims "An empirical characterization of
translation defects across six systems and ten applications", but the corpus is 15 defect-bearing
artifacts out of 26 compared cells, with 14 cells producing no testable artifact at all —
"characterization" implies a prevalence claim the paper elsewhere correctly refuses (§8.2: "avoid
prevalence claims"). Second, the paper positions itself against FLOURINE/VERT/RustAssure by saying
"our claim is not that such validation is new. We study four assumptions that its oracle otherwise
couples together" (§3.1). That is a clear statement of intent, but the delta is never isolated
experimentally: of the four obligations, only observation gets a measurement (n=9, W8), and the
FLOURINE/RustAssure misses in Table 7 are not attributed to any of the four. If the thesis is that
these four assumptions are what matter, then for each completed baseline miss (FLOURINE's C16 and
S17; RustAssure's six) the paper should say *which* obligation explains it.

**§2 Motivating Study.** Table 1's construction is fine and the three cases do separate the signals
cleanly. The "No value" entry for qsort is a nice detail. One caution: all three motivating defects
are also members of the 36, so the motivation and the result are the same evidence; that is normal
but it means Table 1 is an illustration, not a finding, and the text should not let it read as one.

**§3 Related Work.** The best part is §3.3's framing: "The first triage question is therefore *which
explanation owns the disagreement*; deduplication comes afterward." That is precisely right and it is
the paper's real conceptual contribution. §3.1's exclusion argument for DIFFER/cozy/cross-checker is
where I disagree (see W3). Note also that `table/validator_scope.tex` is entirely commented out while
`related_work.tex` still `\input`s it and the surrounding prose reasons about what FLOURINE, VERT and
RustAssure "can express" — so in the built paper that comparative claim about the three baselines'
observation mechanisms has no supporting table. Since RQ5's fairness rests on those mechanisms, the
claim needs to be visible somewhere.

**§4 Design and Implementation.** Technically the strongest section. The two-sided abstention margin
is well motivated ("ε measures ambiguity relative to the strongest alternative on both sides, rather
than imposing an absolute similarity cutoff"). The producer-bridge design and its stated failure
mode ("If no usable producer or lossless owner-to-argument bridge exists, construction fails") are
exactly the right shape. Three content gaps: (i) the comparison ladder "stops at the first
difference" — this makes rung statistics non-independent and means a termination difference masks any
value difference; the consequences for the 15/21 crash/semantic split are not discussed; (ii)
optional comparator plugins are described as "untrusted, translation-layout-dependent code" that can
"add a final comparison rung" — which defects depended on a plugin rung, and would any of the 21
semantic defects disappear without plugins? Not reported; (iii) §4.3's claim that "212 Tulip
harnesses collectively enter all 213 functions" is used to argue transitive reach, but transitive
entry through an unmatched callee means a divergence inside that callee is attributed to the
enclosing boundary — the attribution consequences of transitive reach are not discussed.

**§5 Study Subjects.** The "Stochastic translation outputs" paragraph is the right policy and clearly
stated: one frozen artifact, no best-of-n selection, source hash frozen. Good. What is missing is
the number of artifacts examined and discarded during the exploratory phase that §8.2 discloses;
"frozen after exploratory work" without a count leaves the selection effect unbounded.

**§6.1 RQ1.** See W9. Additionally, Panel (b)'s 0.516 forced precision is presented with an
explanation (stubs and functions with no counterpart) that is convincing, but the panel mixes
artifacts with 3 true pairs (qsort) and 77 (cJSON) under an application-weighted mean, so the
"Overall 0.516" is hard to interpret. The rename result (7/9 vs. tool maps 4/8 vs. name equality 0/9)
is the genuinely interesting number and deserves to be the panel's headline rather than a
forced-precision aggregate.

**§6.2 RQ2.** See W1, W2, W4, W6, W12. One more: the paragraph on SACTOR's seven compilation
failures ("its topological translation order rejects the recursive dependency cycles in cJSON,
quadtree, and lil"; function-pointer issues in the other four) is a genuinely useful finding about a
translator's reach and is buried. It is also the only place in the paper where a non-defect finding
about a translation system is reported, and it should be counted as such.

**§6.3 RQ3.** See W11, W13. The mechanism/symptom decoupling argument (Laertes and SACTOR both
produce all-zero tables for different reasons; byte-string narrowing produces a panic in one system
and a rejection in another) is well made and is the right level of analysis. The claim that families
"form clear strategy-specific signatures" would be more convincing with the per-system × per-family
matrix rather than prose counts.

**§6.4 RQ4.** See W10. The framing sentence — "campaign reach is a property of the interaction
between a generated campaign and a particular translated artifact; it is not, by itself, a score for
the harness generator" — is correct and I appreciate it, but it raises the question of why this is an
RQ at all. There is no comparator: §6.4 states "Project tests remain acceptance evidence for the
translation and do not supply a common coverage baseline." I do not fully accept that. The C-side
project tests run against the C program would give a perfectly usable reference for "how much of this
library do its own tests exercise", which is the comparison a reader wants in order to calibrate
8.3%–100%. Without any reference point, Table 6 is a descriptive appendix promoted to an RQ. The
Laertes decomposition (64.9% artifact-level vs. 92.0% excluding three translator-added functions,
3,887 regions of which are the severed initializer) is excellent and shows the authors know exactly
what the denominator issues are — that rigor should be applied to giving the table a comparator.

**§6.5 RQ5.** See W3. Additionally, listing VERT as a row of zeros in Table 7 is a presentation
choice I would not make even with the caveat: my own practice is to exclude a tool that structurally
cannot accept the input and explain why in prose, because a zero row invites exactly the misreading
the caveat then has to spend three sentences repairing.

**§6.6 Component Analysis.** See W8.

**§7/§8 Discussion, Threats, Conclusion.** §8.1 and §8.2 are strong and I would not want them
shortened. The one substantive gap is that the threats do not mention the two largest ones surfaced
above: that a third of the defects rest on evidence outside the described pipeline (W1), and that no
defect was externally confirmed (W2). The conclusion's "These results show that translation
validation requires both broad execution and a relational oracle capable of observing and attributing
semantic change" is supported in direction but "show" is stronger than a 9-defect observation
ablation and an unmeasured attribution stage support.

---

## 6. Questions for the authors

1. **Provenance of the 36.** How many of the 36 defects were established by the confirmation pipeline
   described in §4 and drawn in Figure 2, and how many by other evidence? The figure's generator
   states that the sampled records manifest 22 of 36 and that the rest rest on "the earlier
   driver-level campaigns" and on unsampled confirmation. Please reconcile this with §6.2's sentence
   that source-level triage merges the funnel's records "into the 36 defects reported here", and
   describe the driver-level campaign in §4 if it is part of the method.
2. **Oracle precision.** Of the 1,931 records promoted to "confirmed divergence", how many survived
   source-level triage and how many were rejected, by rejection reason (alignment, provenance,
   contract, other)? The manifest also carries five records marked "candidate — untriaged": are these
   inside or outside the 1,931, and under what rule?
3. **Reporting policy.** Were any of the 36 defects reported to the authors of C2SaferRust, Laertes,
   CROWN, PtrTrans or SACTOR? If not, why not, and was each defect checked against the corresponding
   system's published limitations and issue tracker to establish that it is previously unknown?
4. **Reverse baseline direction.** On the artifacts FLOURINE and RustAssure accept, do they report
   any translation defect that is *not* in your 36? If you have not run this, what prevents it?
5. **Baseline failure modes.** What are the 14 FLOURINE analysis failures and the 22 RustAssure
   compilation failures caused by, and were the baselines' authors contacted about them? How did you
   verify that your packaging adapters are not the cause?
6. **Rust-only crash fuzzing as a baseline.** Using the archived discovery-stage data: how many of
   the 15 crash/panic defects did the Rust-only discovery stage flag without the paired oracle, how
   many of the 21 semantic defects did it flag (I expect 0, but please report it as measured), and
   what fraction of the Rust-only terminations were subsequently attributed to reference UB,
   out-of-contract inputs or instrumentation?
7. **Negative-control power.** Can you report the control as "0 confirmed divergences of 8,962
   adjudicated records" rather than only "no confirmed defect"? And can you provide any positive
   control — e.g. injecting known semantic-changing rewrites into the C2Rust outputs — showing what
   fraction the pipeline would have flagged at the same boundaries?
8. **Zeros and budget.** For the ten control cells and the restructuring cells reported as `0`, does
   the verdict change under ≥5 seeds or a 24-hour budget? If you have saturation data (time to last
   new confirmed divergence per cell), please report it.
9. **Clustering quality.** What are the cluster counts per system, and what are their purity and
   completeness against your manual root-cause labels? Why do CROWN's two automatic clusters
   correspond to four defects?
10. **RQ1 aggregation and ε.** Please report micro-averaged precision alongside the macro average
    (I compute ≈0.874 from Table 5's own pair counts), report recall, and show precision and
    accepted-pair fraction as a function of ε rather than only the ε = 0.01 operating point.
11. **Tulip protocol.** Why is the seed-refined campaign the one reported in Table 6 while the other
    nine applications use the uniform protocol, and did the seed refinement change Tulip's RQ2
    verdict (4 defects) in either direction?
12. **Defect individuation.** Under a per-translator root-cause unit rather than a per-artifact
    faulty-rewrite unit, how many distinct defects are there? Specifically, are the seven Laertes
    initialization defects seven translator bugs or one systematic behavior observed in seven
    artifacts?
13. **Comparison-ladder and plugin dependence.** Which of the 36 defects were detected only at a
    plugin-provided structured-comparison rung, and how many value differences are masked by the
    ladder stopping at a termination difference?
14. **Table 3 arithmetic.** I count eleven `0` cells in the five restructuring columns, not ten, and
    therefore 26 compared restructuring artifacts rather than 25. Which is correct?
15. **RQ2 accounting provenance.** §6.2 states that the automatic matcher configuration was frozen
    before execution and its accepted pairs passed directly to harness planning. The submitted source
    of `evaluation.tex` carries a commented instruction to "regenerate the RQ2 accounting from the
    frozen automatic matcher and update the retained/lost-defect counts reported below". Is the
    reported RQ2 accounting the output of the frozen automatic matcher, or of an earlier
    configuration? (I raise this because it is in the submitted sources; it is not visible in the
    rendered paper.)

---

## 7. What would move this up one grade

To reach **Weak Accept** I need three things, all of which I believe are recoverable from data the
authors already hold. First, close W1: make the confirmation funnel actually account for the defect
count it prints, or state plainly how many defects each evidence channel established and describe
the driver-level campaign as part of the method — I cannot accept a headline number that the paper's
own accounting figure does not cover. Second, give the pipeline a precision number (W4): promoted vs.
rejected over the 1,931 confirmed records, with rejection reasons; a paper asking "when is a
difference a defect?" must report how often its own answer was wrong. Third, add the one baseline
where the paper can lose (W14/W6): Rust-only crash fuzzing with the same harnesses, reported as "N of
36 found, M false reports", plus the measured — not inferred — statement that the Rust-only stage
flagged 0 of the 21 semantic defects. A **Weak Accept → Accept** move would additionally require
external confirmation of at least a few defects (W2), the reverse baseline direction (W3), repeated
seeds for the zero cells (W6), and the observation ablation extended from 9 to all 36 defects (W8).

---

## 8. Self-disclosure

Judgements that follow directly from the profiled reviewer's published standards: the demand for
upstream reporting with confirmed/fixed status and duplicate checks (W2); refusing coverage as an
effectiveness claim and asking what the reach table is compared against (W7 comment, §6.4); baselines
justified by capability, run in good faith, with authors contacted, and a configuration in which the
proposed tool can lose (W3); deduplication measured against a defensible ground truth rather than
merely described (W5); repetitions, budgets and per-target breakdowns (W6); failure modes and manual
effort stated up front; and insistence that claimed scope not exceed what was measured (W13, W14).
My own inference, extrapolated from that profile rather than observed in his writing: the specific
severity ordering (treating the funnel/defect-provenance mismatch as decisive rather than
presentational), the micro-vs-macro precision recomputation in W9, the demand for a seeded-defect
positive control in W7, and the judgement that Weak Reject rather than Weak Accept is the correct
grade for a paper this honest about its own limits.
