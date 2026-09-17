# FSE 2027 — Review of "When Is a Difference a Defect? Differential Validation of C-to-Rust Translations"

Reviewer persona: Joshua Sunshine (CMU), profiled in `pc_sunshine.md`. Rubric = that profile.
Scope: content only. Citation/LaTeX/grammar/style issues ignored per author instruction.

---

## 1. Paper summary

The paper argues that differential testing is not a free oracle for C-to-Rust translation, because a
usable relational oracle presupposes four decisions that are normally left implicit: which C and Rust
functions correspond, how one logical input is realized across two different representations, which
observable state carries the functional contract, and whether an observed disagreement is
attributable to the translation rather than to an undefined C reference, an out-of-contract input, or
the instrumentation. It builds a validator that makes each decision explicit and inspectable: a
name-independent structural matcher with a two-sided abstention margin (§4.2); a deterministic,
definition-derived **HarnessPlan** that records the input relation, the representation adaptations,
and the comparison ladder, and that reports a *construction failure* rather than guessing (§4.3);
Rust-only coverage-guided discovery followed by paired replay through a fixed comparison ladder
(§4.4); and a side-isolated confirmation pipeline — C-only ASan+UBSan, combined replay, Rust-only
replay with and without sanitizers — that must clear a candidate before source-level triage promotes
it to a defect (§4.5). Evaluated over a 10-application × 6-system matrix, it confirms 36 distinct
defects in 15 artifacts from the five restructuring systems, 21 of which terminate normally and
change values or state; no confirmed defect appears in the ten mechanical c2rust controls. The paper
additionally classifies the 36 into seven mechanism families, reports campaign reach with an explicit
matched/planned/built boundary funnel, and applies the released FLOURINE, RustAssure, and VERT
artifacts to the frozen defect corpus, separating unsupported inputs and tool failures from completed
semantic misses.

---

## 2. Overall decision

**Weak Accept.**
**Reviewer expertise: Knowledgeable.** (I work on unsafe-Rust semantics, Rust-specific undefined
behavior, and dynamic analysis across the C/Rust boundary, and on test-oracle design. I do not work
on C-to-Rust translation itself, and I have not used FLOURINE, RustAssure, or VERT.)
**Confidence: medium-high** on the oracle, UB-gating, confirmation, and evaluation-accounting parts;
**medium** on the matcher internals and on whether the RQ5 baseline adapters are fair.

I want this paper in the program. The evidence discipline is better than almost anything I see in
this area: the confirmation funnel reconciles exactly against the archived per-cell summaries
(I checked: 14,470 sampled = 13,515 retained + 955 not-reproducible; 13,515 = 3,365 clean-C +
7,318 + 2,831 + 1 UB-associated; 3,365 = 741 + 1,190 confirmed + 751 instrument-only + 434
out-of-contract + 248 inconclusive + 1 no-verdict), the boundary funnel sums exactly
(4,222 matched / 1,854 planned / 1,662 built are the exact column sums of Table 7), and the defect
counts in Tables 5 and 6 match the manifest per system, per family, and per application. That is the
behaviour I expect and rarely get.

What keeps it at Weak rather than Accept is that the paper's *central* claim — that making the four
obligations explicit is what makes the oracle trustworthy — is supported by its weakest experiment,
that there is no measurement of the validator's own miss rate, and that the Rust side of a
C-vs-Rust differential is never qualified for undefined behavior even though the C side is qualified
twice over. Each of these is fixable within a revision cycle.

---

## 3. Strengths

**S1. The confirmation pipeline is the right shape, and it is reported as a funnel rather than as a
number (§4.5, Table 4, Fig. 3, §5.2).** The paper treats a divergence as a *candidate* and spends
side-isolated ASan/UBSan/sanitizer-free replay only on saved candidates, in the detect-then-confirm
pattern. Critically, it reports the attrition — 176,507 saved → 14,470 sampled → 13,515 combined →
3,365 clean-C → 1,931 confirmed → 36 defects — with every intermediate category named
(reference-associated, instrumentation/out-of-contract, not reproducible, inconclusive, plus one
archived record with no verdict). I verified the last four stages against
`results/rq3_coverage/*/*/confirm_sample/summary.json`; they reconcile exactly. Publishing the
unflattering middle of a pipeline is the single most important thing an empirical paper can do, and
this paper does it.

**S2. The paper repeatedly refuses to overclaim about sanitizer silence (§4.5, §7.1, Table 4
caption, Fig. 3 caption).** "Sanitizer-clean replay means that no configured check fired; it is not
proof that the C execution is free of all undefined behavior"; "``Clean C'' means that no configured
check fired, not that defined behavior has been proved." The paper also explicitly declines to read
the funnel as a false-positive rate ("These reductions are candidate accounting, not an estimated
false-positive rate, because confirmation caps each boundary's sample at 200"). This is the correct
epistemic stance and it is stated in the places a reader would otherwise misread.

**S3. Abstention and non-harnessability are first-class outcomes, not silent losses (§4.2, §4.3,
Table 7).** The matcher abstains at a two-sided margin; the planner records a *construction failure*
with a specific reason ("environment-input construction failure" for `BZ2_bzReadOpen`) rather than
inserting a guessed constant; Table 7 carries matched/planned/built for all 37 artifacts so the
reader can see that 2,368 of 4,222 matched boundaries never got a plan. Most harness-generation
papers report only what worked.

**S4. RQ5 separates applicability from sensitivity (§5.5, Table 8).** Running released FLOURINE,
RustAssure, and VERT on a frozen corpus and reporting an execution funnel (submitted / accepted /
compiled / completed / detected, with unsupported inputs, analysis failures and compile failures kept
separate from completed misses) is exactly right, as is the refusal to count VERT's 36 unsupported
cases as misses: "This is an applicability boundary for our fixed-artifact question, not a semantic
miss." The paper also declines the easy claim: "this RQ measures applicability and positive-case
sensitivity on these known defects, not general recall, precision, or relative bug-finding ability."

**S5. The motivating examples do real work (§3, Fig. 1, Table 1).** Three minimal, distinct
reproducers, each defeating a *different* plausible check, with a signal table naming which check
each one escapes and why ("No value" for qsort because the function returns nothing). The zlib
`state->offset > copy` → `state as usize > copy` case (§5.3, Case 2) is a genuinely excellent
reproducer: one field load omitted, a heap address compared against a byte count, 169 confirmed
output-length divergences, and the generated code even annotates the line as a pointer comparison.

**S6. Reach is reported as a property of the interaction, with the ceiling attributed per cell
(§5.4).** "campaign reach ... is not, by itself, a score for the harness generator," followed by four
named ceiling classes (construction-, translation-, reference-, domain/state-limited) each backed by
a side-isolated witness — e.g. cJSON × PtrTrans's 3.2% is attributed to `cJSON_New_Item` being an
untranslated stub returning `None` on all 18 direct-constructor witnesses, not to shallow fuzzing.
That is the correct way to stop a low number from being read as a failure of the wrong component.

**S7. Honest disclosure of subject-selection history (§7.2, "Selection").** "The ten-application
corpus was frozen after exploratory work, not sampled prospectively. Earlier notes explicitly allowed
replacement subjects. We disclose this history, retain failed and UB-excluded cells, prohibit
replacement during confirmatory experiments, and avoid prevalence claims." Very few authors write
that sentence.

---

## 4. Weaknesses, ranked by how much they drive my decision

### W1. The Rust side of the differential is never qualified for undefined behavior. (§4.5, Table 4; §7.2 "Sanitizer completeness")
Table 4 shows the attribution ladder as C-UBSan, C-ASan, Rust-ASan. The C reference is qualified
twice (ASan **and** full UBSan) and "A C sanitizer report prevents promotion to a translation
defect." The Rust side gets ASan only, and only to "distinguish a program panic from a termination
introduced by instrumentation." There is no Rust-side undefined-behavior model anywhere in the paper:
the words Miri, Stacked Borrows, Tree Borrows, aliasing, and provenance do not appear once, and no
Rust optimization level is stated.

Why this matters here specifically, and not as a generic wish: the subjects are exactly the
population where Rust-side UB is expected. The paper itself says c2rust emits "structurally similar,
predominantly unsafe Rust" (§4 related work) and that Laertes, CROWN, and C2SaferRust take c2rust
output as their starting point (§4.3). A translated function that violates Rust's aliasing rules has
undefined behavior whose *observable value* is a function of the optimization level, which means a
confirmed value divergence in such a function is not a stable observation — it may vanish or change
under a different build, and a clean comparison may be hiding a latent violation. The paper's
threats section covers only C-side sanitizer incompleteness ("ASan and UBSan do not detect all C UB")
and says nothing about the symmetric question on the Rust side. The asymmetry is structural: the
paper's whole thesis is "when is a difference a defect?", and it answers that question rigorously for
one of the two programs.

Resolution: (a) re-run the 36 confirmed inputs — or a stated sample — under Miri with Tree Borrows
(or state why Miri cannot load these crates, which for FFI-linked harnesses is a legitimate answer
and is itself a finding), and report how many confirmed defects sit on Rust executions that also
violate the aliasing model; (b) state the optimization level of both builds and whether any confirmed
value divergence changes across `-O0`/`-O2` on either side; (c) if Rust-side UB is out of scope, say
so explicitly in §7.2 and state the consequence for the stability of the 21 value/state defects.
A one-paragraph scoping statement plus a spot check would satisfy me; a silent omission will not.

### W2. The paper's central methodological claim rests on its weakest table. (§5.6, Table 9; §7.2 "Component checks")
The thesis is that explicitness and attribution are what make the oracle reliable. The only
experiment that tests this is Table 9, and it has five rows over four different evidence sets: three
of the five rows rest on a *single witness* ("urlparser, 1 witness"; "lil, 1 mismatch"; "qsort,
3 true pairs"), and the two observation rows use "9 confirmed defects" — a subset of 36 whose
membership and selection rule are never stated. Why nine? Which nine? If the nine were chosen because
their observation channels were already instrumented, the 9/9 vs 6/9 vs 3/9 result is not
generalizable to the other 27 and the paper cannot say so.

The caption is honest ("Rows use separate fixed evidence sets and must not be pooled"), and §7.2
correctly says it "does not estimate population-level recall or precision." I credit that. But the
honesty does not substitute for the measurement. A single-witness row cannot support a design claim,
and the paper's headline framing — that the four obligations are what separate a trustworthy
validator from a naive one — is a claim about the whole corpus.

Resolution: run the *end-to-end* ablation on the full matrix, not a replay of pre-selected witnesses.
Concretely: (i) the same pipeline with name-equality matching instead of the structural matcher —
how many of the 36 are lost, and how many boundaries disappear? (ii) the same pipeline with a
return-and-termination-only ladder — how many of the 36 survive? (iii) the same pipeline with the
UB gate removed — how many *false* defects would have been reported, i.e. how many of the 7,318 +
2,831 UB-associated records would have been promoted? That third number is available in the archived
data and is the single most persuasive number the paper could report. Also state the selection rule
for the nine-defect replay set.

### W3. The negative control is under-reported in the paper, and it doubles as the debugging set. (§4.3 "Harness qualification"; §5.2; Fig. 3)
Two separate problems with the paper's strongest structural claim.

*(a) The control's disposition is missing.* §5.2 scopes the funnel to the treatment: "Across the five
restructuring systems, the archived campaigns save 176,507 candidate inputs." The paper therefore
never tells the reader whether the ten c2rust controls produced *zero candidates* or produced many
candidates that were all correctly attributed away. Those are epistemically very different results,
and only the second is evidence that the harness machinery does not manufacture differences. The
archived data contains the answer — aggregating the same `confirm_sample/summary.json` files over
the c2rust cells gives roughly 8,960 sampled control candidates resolving to reference-UB,
instrumentation, non-reproduction and inconclusive, with **zero** confirmed divergences and zero
confirmed terminations. That is a genuinely strong control result and it is not in the paper. Report
it, in the funnel, as a control row beside the treatment row.

*(b) The control was also the tuning set.* §4.3 states: "a divergence in a C2Rust control is treated
as a harness-qualification failure that must be resolved before downstream differences are
interpreted." So the machinery was debugged until the control was clean. That is a reasonable
development practice, but it means "No confirmed defect appears in the ten mechanical c2rust
controls" (§5.2, abstract) is partly a consequence of having fixed the harness against that control,
and the paper presents it as an independent result. How many qualification failures occurred, what
was changed in response, and was the final control run performed with a frozen harness after the last
change? Without that, the control cannot bear the weight §5.2 puts on it ("This markedly lower
observed incidence supports the qualified-negative-control role").

*(c) A scope limit the paper does not state.* The paper's own justification for the control is that
"C2Rust retains source structure and C-shaped interfaces." That is precisely why the control does
**not** exercise the components most likely to inject a spurious difference: the representation
bridges (pointer+length → slice, nullable pointer → `Option`, byte buffer → `String`). The control
qualifies the decoder and the ladder; it does not qualify the adapters. §7.2 concedes adapters "may
accidentally strengthen a precondition or hide a difference" and offers only archiving as mitigation.
Since seven of the 36 defects are "byte-string domain narrowing" — exactly the class an
over-narrow adapter could fabricate — this gap sits directly under a headline number. Some
adapter-level control is needed, e.g. round-trip tests showing each adapter's realized C domain
equals the planned one, or a control in which a c2rust artifact is deliberately given a
slice/`Option` bridge.

### W4. No measurement of the validator's own miss rate; the only sensitivity numbers in the paper are for other people's tools. (§5.5, §7.2)
The paper is scrupulous that RQ5 "measures applicability and positive-case sensitivity ... not
general recall," and §7.2 says "the confirmed-defect corpus is not treated as exhaustive." Good — but
that leaves the reader with no basis at all for judging detection power, and two places in the paper
depend on it. First, the twenty "0" cells in Table 5 are read as bounded agreement; their
informativeness depends entirely on an unmeasured miss rate, and some of those cells have 6.3%–12.6%
region reach. Second, the confirmation sample is capped at 200 per boundary, so 162,037 of 176,507
saved candidates were never confirmed; the paper cannot say whether additional *distinct* defects
live there.

Resolution, in increasing order of cost: (i) for two or three boundaries with the most saved
candidates, lift the 200 cap, confirm the full set, and report how many new *distinct* defects appear
beyond the capped sample — this directly bounds the sampling loss; (ii) run a seeded-defect study —
inject a set of realistic translation mutations (an omitted field load, a severed initializer, a
`String`-narrowed buffer, an off-by-one slice bound) into c2rust artifacts and report how many the
pipeline recovers, with the injection set defined independently of the 36; (iii) report, for each of
the seven families, whether the family is detectable in principle by the current ladder.

### W5. Defect counts are confounded with exposure, yet per-system conclusions are drawn from them. (§5.3, Table 6, Table 7)
§5.3 states: "All seven Laertes defects involve initialization; three of CROWN's four defects corrupt
ownership-managed state; and three of PtrTrans's four defects lose an interface contract during
pointer-to-slice reshaping" and concludes "The families also form clear strategy-specific
signatures." But exposure across systems differs by more than an order of magnitude, from Table 7's
own numbers: summing built harnesses gives roughly 403 for C2SaferRust, 370 for Laertes, 372 for
CROWN, but only 23 for PtrTrans and 16 for SACTOR; and the number of artifacts per system ranges
from 2 (SACTOR) to 8 (CROWN). The defect distribution is likewise extremely concentrated — 20 of 36
in one system, and 13 of 36 in one application (optipng, from the manifest), with 10 of 36 in the
single optipng × C2SaferRust cell. "Three of PtrTrans's four defects" is three observations behind 23
built harnesses in three applications; calling that a strategy signature is over-reading.

The same confound undercuts the sentence "The defects range from a 30-line qsort to the bzip2 and
optipng codecs; they are therefore not confined to one program scale or translation strategy" —
true as a coverage statement, misleading as a distributional one.

Resolution: either normalize (defects per built harness, or per confirmed record, or per boundary
reaching the target) and report the denominators next to Table 6's counts, or explicitly retract the
per-system signature claim and present Table 6 as a catalogue of observed mechanisms with no
cross-system comparison. Given §7.2's own rule ("avoid prevalence claims"), the second option is more
consistent with the rest of the paper.

### W6. The matcher operating point used in the campaigns is not reconcilable with the matcher evaluated in RQ1. (§5.1, §5.2, Table 7)
§5.2 says "We freeze the automatic matcher configuration before execution and pass its accepted pairs
directly to harness planning; abstained pairs remain untested." §5.1 says that at the deployed
threshold ε = 0.01 the matcher "accepts 78.4% of pairs" and that "Most abstentions occur in Tulip's
structurally indistinguishable indicator families." Yet Table 7 reports M = 213 matched boundaries
for **all four** Tulip artifacts — i.e. every function — and 213/213/212 planned/built. Either the
M column is not "accepted pairs at the deployed operating point" (in which case the funnel does not
describe the pipeline §5.2 says was run), or abstention was not applied in the reach campaigns.

This is not cosmetic. §5.1 also reports Tulip's forced precision as 0.554 with "the same 95
assignments ... wrong" in each output. If the Tulip campaigns ran on all 213 pairs, then roughly 95
boundaries per Tulip artifact were comparing functions that do not correspond, and Tulip's headline
90.2–93.2% region reach was partly achieved through known-incorrect pairs. The paper's defence — human
review of "every candidate-producing pair" (§5.2) — protects against *false* defects but says nothing
about defects missed because the correct pair was never harnessed.

Resolution: state, per artifact, how many pairs the deployed configuration accepted versus how many
exist; report reach restricted to correct correspondences for at least the Tulip cells; and report
the accepted-pair precision for Panel (b) artifacts (the renaming tools), which is the case the
matcher exists for and is the one operating point the paper never gives. Panel (b)'s forced precision
of 0.021 (lodepng × PtrTrans) and 0.130 (bzip2 × PtrTrans) makes this urgent.

### W7. Budget, environment and configuration are unspecified to a degree that blocks replication and weakens the "0" cells. (§5.4, §7.2)
§5.4 says only "The standard campaign runs for one hour in the Rust-only execution mode." One hour
**per what**? Per artifact, per boundary, or per built harness? For optipng × C2SaferRust that is
either 1 hour or 96 hours, and the meaning of every "0" in Table 5 and of the negative control
depends on which. The paper never says. Likewise, Tulip "reaches stable coverage in 600 seconds" —
per harness or per campaign, across 212 harnesses?

Beyond the budget unit, the paper gives no Rust toolchain version, no clang/LLVM version, no
libFuzzer configuration beyond "fork mode", no list of which UBSan checks constitute "full UBSan", no
sanitizer flags, no optimization levels, and no machine or resource description. Sanitizer coverage
and coverage instrumentation are version-sensitive; "clean C" is defined by a check set the paper
does not enumerate. §7.2's "Results therefore mean 'sanitizer-clean under this configuration'" is
exactly right — but the configuration is never given.

Resolution: one short paragraph in §4.6 with the budget unit, per-campaign wall-clock and exec
counts, toolchain and clang versions, the UBSan check list, optimization levels for both sides, and
the machine/parallelism used.

### W8. No upstream disclosure is reported for 36 defects in five published research artifacts. (whole paper)
The paper reports 36 confirmed defects in artifacts released by five published systems and never says
whether any were reported to their authors, whether any were acknowledged, disputed, or fixed, or
what disclosure policy was followed. The only occurrence of "disclose" in the paper refers to
disclosing the subject-selection history. For work whose entire contribution is "this difference
really is a defect," independent confirmation by the people who wrote the translator is the cheapest
and strongest corroboration available, and its absence leaves the 36 resting solely on the authors'
own source-level triage — the same authors who, per §7.2, also did the family coding. I would expect
a short paragraph on the disclosure protocol and a table column recording each defect's upstream
status, plus a sentence on how the authors handled the fact that several of these are research
artifacts rather than production software.

### W9. Defect coding has no independent check. (§7.2 "Bug and mechanism coding")
"The same researchers discovered and categorized the defects." The mitigations offered — separating
mechanism from symptom, preserving per-instance evidence, counting by faulty rewrite — are about the
*definition* of the unit, not about the reliability of applying it. Seven families assigned to 36
items by the people who found them, with no second coder, no independent re-coding pass, and no
agreement statistic, is the weakest link in RQ3, and RQ3 is a full research question. The paper also
supplies its counting rule ("Multiple inputs that exercise the same faulty rewrite count as one
defect") but never shows it being applied to a hard case — e.g. whether C2SaferRust's six
"semantic computation substitutions" in one codec are six rewrites or one systematic rewriting
behaviour observed six times. Resolution: have someone not involved in discovery independently
re-assign the 36 to the seven families from the evidence records alone, report the disagreements and
how they were resolved, and ship the family definitions used.

### W10. The Tulip seed intervention makes the headline reach table internally inhomogeneous. (§5.4, Table 7 caption)
Table 7 reports 33 artifacts under a one-hour unseeded protocol and four Tulip artifacts under a
different one: 600 seconds with hand-chosen option seeds sampled from "the fixed set {1, 2, 3, 5, 10,
20}", introduced after observing that the unseeded campaigns sat at 24.0–34.7%. The paper discloses
this (caption: "Tulip reports the deterministic seed-refined campaign"), and it reports two real
controls — a length-matched random control reaching 38.7%, and a 600-second unseeded control
reproducing the archived one-hour baselines within one region. I credit both controls; they are
exactly right. But the fixed set is target-specific knowledge of the guard, chosen post hoc, applied
to one of ten applications, and the resulting numbers are placed in the same column as everyone
else's. Either apply an analogous deterministic seed policy uniformly (including a null result where
it does not help, as the paper already predicts for quadtree), or report Tulip's unseeded numbers in
Table 7 and move the seeded result to a clearly separated analysis.

### W11. The RQ5 funnel is not reconstructable from the table as captioned. (Table 8, §5.5)
The caption says "All funnel columns use the full 36-defect denominator" and "A-fail and C-fail
denote analysis and compilation failures." RustAssure decomposes exactly: 2 unsupported + 22 C-fail +
1 A-fail + 11 completed = 36, and 34 accepted − 22 = 12 compiled, 12 − 1 = 11 completed. FLOURINE
does not: 33 submitted, 20 accepted, 17 compiled, 16 completed, with A-fail = 14 and C-fail = 3.
20 + 14 = 34 > 33, and 17 − 16 = 1 ≠ 14. The only reading that works is that FLOURINE's A-fail
silently pools 13 acceptance rejections with 1 post-compile analysis failure, i.e. the column means
something different in each row. Since the headline "14 of 16" is entirely determined by which stage
lost the other 20 cases, the stage decomposition needs to be explicit.

Three related RQ5 gaps: (i) *why* FLOURINE rejected 13 of 33 submissions is never stated, and those
rejections are the difference between "14/16" and "14/36"; (ii) RustAssure's 22 compilation failures
out of 34 accepted (65%) are not attributed — released artifact, our adapter, or the translated code
itself? — yet "5 of 11" depends on it; (iii) no budget is stated for either baseline, though the text
mentions one analysis "exhausts its full symbolic-execution budget."

### W12. The paper's own headline counterfactual is asserted rather than measured. (abstract, §5.2)
The abstract and §5.2 claim the 21 value/state defects "provide no failure signal to Rust-only crash
fuzzing." The paper actually *ran* Rust-only crash fuzzing — the discovery phase — and saved
"translation-side terminations" separately from the coverage corpus (§4.4). So the measurement
exists: for each of the 21, did the Rust-only discovery phase ever produce a termination artifact?
One sentence reporting that would convert the paper's single most quotable claim from an argument
into a result. As written it is inferred from the classification (semantic = normally terminating),
which is close to definitional.

### W13. Seven defects are "byte-string domain narrowing," but in-contract-ness is never established for them. (Table 6, §4.5, §7.1)
§4.5 says out-of-contract inputs "may be retained as robustness evidence while remaining separate
from confirmed translation defects," and the confirmation pipeline has an explicit
out-of-contract category (434 records). Yet the largest-but-one family is defects found by supplying
byte strings that C accepts and Rust rejects or panics on, which raises precisely the question the
category exists to answer: does the *application's* contract for that function admit arbitrary bytes,
or does the harness supply inputs the C API never receives in practice? The paper's general position
— that silently narrowing the represented C domain is itself the defect (§1) — is defensible and I
am inclined to accept it, but it needs to be argued for these seven specifically, with the evidence
used to decide in-contract-ness per defect.

### W14. The 36 defects are not individually presented in the paper. (§5.2, §5.3, §5.5)
The paper reports the 36 only as aggregates (Table 5 per cell, Table 6 per family) and shows five
reproducers total (three in §3, two in §5.3). §5.5 then refers to "the two completed misses are C16
and S17" — identifiers that appear nowhere else in the paper and that a reader cannot resolve. A
compact per-defect table — id, system, application, boundary, family, crash/semantic, observation rung
that exposed it, confirmation verdict, upstream status — would make the corpus auditable in the paper
rather than only in the artifact, and would let a reader check the RQ5 misses against the mechanism
claims.

### W15. Single-run reporting for the one comparative claim in RQ4. (§5.4, §7.2)
§7.2 is candid: "Each reported campaign is one fixed-budget observation of one frozen artifact, not an
estimate of expected fuzzer performance," and defect promotion relies on deterministic replay rather
than rediscovery frequency. That correctly insulates the 36. It does not insulate the seed-refinement
comparison (90.2–93.2% seeded vs 38.7% random control vs 24.0–34.7% unseeded), which is a
between-condition claim reported with n = 1 per arm and no variance. Even three repetitions per arm
with min/median/max would settle it.

---

## 5. Detailed comments per section

**§1 Introduction.** The four obligations are a genuinely useful decomposition and the paper is
disciplined about not claiming differential validation is new ("our claim is not that such validation
is new," §4). The `lodepng_save_file` ↔ `load_file` example is a good, checkable motivation for
name-independence. Two content notes. First, the contribution list promises "an explicit, auditable
method ... Unsupported relations are reported explicitly rather than silently approximated" — the
paper delivers the reporting, but the *auditability* claim is never tested; no one outside the author
team is shown auditing a HarnessPlan, and no measurement shows that a reader can detect a wrong plan
from the recorded evidence. If auditability is a contribution, it needs an audit. Second, the intro
asserts the 21 no-crash-signal figure before the reader has any definition of "defect"; the definition
only arrives in §5.2.

**§3 Motivating Study.** Strong. Table 1's "No value" cell for qsort is the kind of distinction most
papers elide. One gap: the three cases are drawn from three different systems, which is good, but the
paper does not say whether they were selected before or after the main campaign, and they later
reappear as members of the 36. If they were the exploratory cases that motivated the design, say so —
§7.2 already discloses that the corpus was frozen after exploratory work, and the same disclosure
should cover the motivating examples.

**§4 Related Work.** Unusually good at stating what each neighbour *requires* rather than only what
it does — "none independently recovers source–Rust library boundaries and realizes one logical input
across their representations," and the argument for why supplying our harnesses to cozy/DIFFER "would
reuse the component being evaluated" is correct and well put. The §4.3 paragraph on oracle
qualification (Csmith's avoidance vs CsmithEdge's post-hoc definedness check, and UBFuzz on sanitizer
false negatives) shows the authors understand the problem I care most about — which makes the absence
of the symmetric Rust-side treatment (W1) more conspicuous, not less. Also: the paragraph beginning
"Tulip exposes an incorrect return value..." discusses a comparison table that no longer appears in
the paper, so the claim "These mechanisms can express the relevant observations" is currently
unsupported by anything the reader can see.

**§4.2 Function Matching.** The two-sided margin is a clean formulation and crediting IsoRank and
Hungarian assignment while naming the three translation-specific additions is the right level of
claim. But the matcher is evaluated (§5.1) in a forced regime and deployed (§5.2) in an abstaining
regime, and only the forced regime is reported for the renaming tools. See W6.

**§4.3 Harness Planning.** The `BZ2_bzReadOpen` vs `BZ2_bzBuffToBuffDecompress` contrast is the best
single explanation of the eligibility boundary in the paper. The producer-bridge design is well
scoped ("at most a producer–target–destructor sequence"), and the honesty that "Harness eligibility
establishes representability, not that every generated execution satisfies an unstated semantic
precondition" is exactly the right caveat. The harness-qualification paragraph is where W3(b) and
W3(c) live. Also, the sentence "One eligible boundary can nevertheless exercise many functions
transitively" is used to justify reporting whole-artifact coverage, but the transitive reach through
a *mismatched* boundary is coverage without an oracle — the ladder only compares the paired
boundary's observable state, so code entered transitively is executed, not validated. The paper
should distinguish "reached" from "differentially compared."

**§4.4 Discovery and Replay.** Rust-only discovery followed by paired replay is a sensible split, and
"The discovery loop does not run C; C participates when the saved corpus is replayed" is a useful
clarification. Two consequences are unstated: (i) coverage guidance optimizes Rust coverage only, so
inputs that maximize *C*-side behavioural diversity are never sought — a plausible source of missed
value divergences, worth a sentence in §7.2; (ii) the comparison ladder "stops at the first
difference," so a single execution can only ever evidence one rung, and the funnel's confirmed-record
counts are therefore rung-censored.

**§4.5 Confirmation.** The best-designed part of the paper, modulo W1. The distinction between the
in-loop "recoverable minimal-UBSan check ... only as an inexpensive noise filter" and the isolated
runs that "perform attribution" is precisely the right architecture. The claim that clustering is "a
review unit, not a defect count" is consistent with how Table 6 is used.

**§5.1 RQ1.** Panel (a)'s 0.938 is precision only; no recall is reported, which is defensible under
forced matching but should be said. The Tulip analysis (213 functions → 103 static fingerprints, 120
in non-singleton groups, 54 + 41 errors) is a model of how to explain an outlier rather than bury it.
Panel (b)'s claim that "structural matching recovers more correspondences than the maps shipped with
the translations" is supported (7/9 vs 4/8) but is a nine-item result and should be labelled as such
wherever it is repeated.

**§5.2 RQ2.** See W3, W5, W6, W12. The sentence "This markedly lower observed incidence supports the
qualified-negative-control role" carries more than the design can bear while the control's candidate
disposition is unreported. The SACTOR compilation-failure analysis (topological order vs recursive
cycles in cJSON/quadtree/lil; function pointers in the other four, at four different stages) is
excellent and exactly the right way to report a baseline's 7 failures — it is repeatable-limitation
evidence, not a shrug.

**§5.3 RQ3.** See W5 and W9. The Laertes-vs-SACTOR "same symptom, different mechanism" observation
and the byte-string "same mechanism, different symptom" observation are genuinely valuable and are
the strongest argument for the taxonomy being about mechanisms rather than symptoms. Case 2 (zlib) is
the best reproducer in the paper.

**§5.4 RQ4.** The denominator discipline here is the paper's second-best feature and deserves to be
stated more prominently: taking function and region universes "directly from each translation's own
LLVM-instrumented archive objects," and checking that "For 11 artifacts initially measured through a
linked test or denominator binary, rebuilding the universe from the archive objects reproduced the
same in-scope functions and region counts." That is a real provenance check. The decision to retain
translator-added dead code in the denominator and then report the Laertes decomposition (64.9%
artifact-level, 92.0% excluding three translator-added functions, 3,887 of 3,893 added regions in the
severed initializer) is the honest choice and is well executed. See W7 and W10 for the budget and
Tulip issues.

**§5.5 RQ5.** See W11. The artifact-selection audit (why cozy/DIFFER/Syzygy/SafeTrans/DTV/IDEAS are
excluded, LAC2R's missing artifact, CINDER's unreachable repository, with an audit date) is exactly
the level of detail I want and rarely see.

**§5.6 Component Analysis.** See W2.

**§7 Discussion.** §7.1 is correctly scoped. §7.2 is specific, non-boilerplate, and covers selection,
component checks, coder identity, sanitizer completeness, budgets, adapters, and alignment truth —
this is a better threats section than most FSE papers have. What it does not cover: Rust-side UB
(W1), the control's dual role as tuning set (W3b), the adapter blind spot in the control (W3c), and
the sampling cap's effect on missed defects (W4). The "Alignment truth" paragraph's admission that
PtrTrans's lodepng map's effect "remains unmeasured" because the artifact does not build is exactly
the right way to leave a loose end.

---

## 6. Questions for the authors

1. **Rust-side UB.** Did you run any Rust-specific undefined-behavior check (Miri under Stacked or
   Tree Borrows, or any aliasing/provenance checker) on the 36 confirmed executions? If not, can you
   report for a stated sample whether the confirmed value/state divergences are stable across
   optimization levels on the Rust side? What are the optimization levels of the C and Rust builds?
2. **Control disposition.** How many candidate inputs did the ten c2rust controls save and sample,
   and how did they resolve across the same funnel categories (UB-associated, instrumentation-only,
   out-of-contract, not reproducible, inconclusive, confirmed)? Please give the control row of Fig. 3.
3. **Control as tuning set.** How many harness-qualification failures (§4.3) occurred on the c2rust
   controls during development, what changed in response, and was the reported control run executed
   with the harness generator frozen after the final change?
4. **Adapter qualification.** Since the c2rust controls keep C-shaped interfaces, which experiment
   qualifies the representation bridges (pointer+length → slice, nullable → `Option`, bytes →
   `String`)? For the seven byte-string-narrowing defects, what evidence establishes that the adapter
   did not itself narrow the realized C domain?
5. **UB gate value.** Of the 7,318 UB-associated and 2,831 UB-associated-termination records in the
   restructuring campaigns, how many would have been promoted to defects had the C-side ASan+UBSan
   gate been removed? This is the number that most directly supports your central claim.
6. **Matcher operating point.** Table 7 reports M = 213 for all four Tulip artifacts while §5.1 says
   the deployed ε = 0.01 configuration abstains on 21.6% of pairs, mostly in Tulip. Which
   configuration produced Table 7? Per artifact, how many pairs did the deployed configuration accept
   out of how many candidates, and what is the accepted-pair precision for the Panel (b) renaming
   artifacts?
7. **Mismatched boundaries.** For the Tulip cells, how much of the reported 90.2–93.2% region reach
   is attributable to boundaries whose correspondence §5.1 shows to be wrong? What is reach
   restricted to verified-correct pairs?
8. **Budget unit.** Is the one-hour standard campaign per artifact, per boundary, or per built
   harness? Same question for Tulip's 600 seconds. Please also give total executions per campaign.
9. **Sampling cap.** For the two or three boundaries with the most saved candidates, if the 200-cap
   is lifted and the full candidate set confirmed, how many *additional distinct* defects appear?
10. **Nine-defect replay set.** Which nine of the 36 defects form the Table 9 observation evidence
    set, and by what rule were they selected?
11. **Crash-fuzzing counterfactual.** Did the Rust-only discovery phase ever produce a
    termination artifact for any of the 21 value/state defects? A yes/no per defect would convert your
    headline claim into a measurement.
12. **RQ5 funnel.** Please decompose FLOURINE's 33 → 20 → 17 → 16 explicitly: what does A-fail = 14
    count, why were 13 submissions not accepted, what caused RustAssure's 22 compilation failures
    (artifact, adapter, or translated code), and what budget was each baseline given?
13. **Disclosure.** Were any of the 36 defects reported to the authors of the five translation
    systems? What was the outcome? If disclosure was deliberately deferred, what was the reasoning?
14. **Coding reliability.** Was the family assignment for the 36 defects checked by anyone who did
    not participate in discovery? If so, what was the disagreement rate and how was it resolved?
15. **Counting rule, hard case.** Are C2SaferRust's six "semantic computation substitutions" six
    distinct faulty rewrites or repeated instances of one rewriting behaviour? Please show the rule
    applied to that case.

---

## 7. What would move this up one grade

To Accept, I need three things, and none requires new subjects. **First**, close the asymmetry in the
oracle: either qualify the Rust side for Rust-specific undefined behavior on the confirmed inputs, or
state explicitly in §7.2 that Rust-side UB is out of scope and argue what that costs the stability of
the 21 value/state defects (W1). **Second**, move the central claim off Table 9 onto the corpus: run
the end-to-end ablations (name-equality matching; return-and-termination-only ladder; UB gate
removed) across the full matrix and report defects lost and false defects gained — the third of these
is already computable from the archived records and is, on its own, the most persuasive number in the
paper (W2, Q5). **Third**, report the negative control properly: put the c2rust candidate disposition
in the funnel beside the treatment, disclose how many qualification failures shaped the harness and
whether the final control run used a frozen generator, and add some qualification of the
representation bridges, which the c2rust control by construction cannot exercise (W3).

Fixing the matcher/reach inconsistency (W6), stating the budget unit and toolchain configuration
(W7), and adding a per-defect table with upstream-disclosure status (W8, W14) would make me
enthusiastic rather than merely positive. I would not require the seeded-defect recall study (W4-ii)
for acceptance, but I would require the sampling-cap probe (W4-i), because it is cheap and it bounds
a loss the paper currently leaves unbounded.

---

## 8. Self-disclosure

Weaknesses W1 (Rust-side UB / aliasing models / optimization sensitivity), W3 (negative-control
disposition and its dual role as tuning set), W7 (budget unit, toolchain and sanitizer-check
enumeration), W8 (upstream disclosure protocol), W9 (independent re-coding), W11 (funnel must
decompose), and the "report the control row" request follow directly from the profiled reviewer's
published practice — the MiriLLI/ICSE 2025 sampling funnel and disclosure protocol, the unmodified-Miri
control, the TOSEM codebook and two-coder protocol, and his own construct-validity admission that he
"did not determine whether any of the bugs that we found had an effect on native execution"; W2, W4,
W5, W6, W10, W12, W13, W14 and W15 are my own inferences about how that same standard applies to a
C-to-Rust differential-validation paper, a setting on which he has not published and where his
expertise is adjacent rather than direct.
