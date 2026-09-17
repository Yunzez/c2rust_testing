# FSE 2027 — Review

**Paper:** *When Is a Difference a Defect? Differential Validation of C-to-Rust Translations*
**Reviewer persona:** Shaohua Li (CUHK) — profile in `docs/reviews_fse27/pc_li.md`, used as the rubric.
**Track:** Research Papers.

---

## 1. Paper summary

The paper argues that differential testing of an already-produced C-to-Rust translation is not
an oracle you get for free once both sides are executable: it silently assumes four things —
that you know which C and Rust functions correspond, that one logical input can be realized at
both (possibly reshaped) interfaces, that you compare the state that actually carries the
functional contract, and that an observed disagreement is attributable to the translation
rather than to undefined behavior in the C reference, an out-of-contract input, the harness, or
instrumentation. The authors build a validator that makes each of these explicit: a
name-independent structural matcher with a two-sided abstention margin (§4.2), a deterministic,
LLM-free *HarnessPlan* that derives a canonical C input and a checked adaptation to the Rust
signature, recording construction failures instead of guessing (§4.3), Rust-only coverage-guided
discovery followed by paired replay under a fixed comparison ladder (§4.4), and a side-isolated
confirmation pipeline — C-only ASan+UBSan, combined replay, Rust-only with and without
sanitizers — before clustering and human source-level triage (§4.5). Evaluated over ten C
applications × six translation systems, the study reports 36 defects in 15 artifacts from the
five restructuring systems (21 of them normally terminating value/state errors), zero in the ten
mechanical c2rust controls, a seven-family root-cause taxonomy, campaign reach over 37 artifacts,
and a frozen-corpus applicability study of three released validators (FLOURINE 14/16 completed,
RustAssure 5/11, VERT 0/36 unsupported).

## 2. Overall decision

**Decision: Weak Reject.**
**Reviewer expertise: Expert** (compiler testing, differential testing, UB-aware oracles,
sanitizer false negatives, Rust toolchain reliability).
**Confidence: High** on the oracle/evaluation-design critiques; Medium on the C-to-Rust
translator-specific claims, where I am reading the artifacts through the paper's descriptions.

I want to be explicit that this is a *close* Weak Reject, not a dismissal. The problem framing is
right, the writing is unusually honest about what it did not establish, and I independently
reproduced the confirmation-funnel arithmetic from the authors' own archive (see §5, RQ2), which
is more than most submissions survive. What blocks acceptance for me is that the paper's central
promise — deciding *when a difference is a defect* — is delivered by an adjudicator whose own
error rate is never measured, and the two results that carry the paper (36 defects; 0 in the
control) are both counts produced by that unmeasured adjudicator plus unblinded human judgement,
with no detection-power check, no validator false-positive rate, no upstream confirmation, and no
negative control for the baseline arm. Every one of these is measurable from data the authors
already appear to hold. That is why I am at Weak Reject rather than Reject.

## 3. Strengths

**S1. The framing is a genuine mechanism claim, not a re-targeting claim.** §1 and §3 explicitly
refuse the "first to apply differential testing to C-to-Rust" move ("our claim is not that such
validation is new. We study four assumptions that its oracle otherwise couples together",
related_work.tex). The four obligations are stated as separable and each is given machinery. This
is the correct shape for a contribution.

**S2. The motivation is concrete, real, and executed.** §2's three cases (C2SaferRust ti_adx_start
casting the options pointer; Laertes's severed bzip2 CRC initializer; PtrTrans's qsort subslice
index) are traced to a source-level rewrite, and Table 1 reports the signal each validation
regime does or does not produce *on those exact executions*. The qsort case is threaded from the
intro through §2 and RQ1. This is the running-example discipline I look for.

**S3. Abstention and construction failure are first-class outputs.** §4.2's two-sided margin
$m(c,r)\ge\epsilon$ and §4.3's "the plan records the construction failure rather than inserting a
guessed constant" mean the pipeline can say *I don't know* instead of manufacturing evidence. The
bzip2 `BZ2_bzReadOpen` vs `BZ2_bzBuffToBuffDecompress` contrast (§4.3) is a good worked example of
the boundary between the two.

**S4. Attribution is ordered correctly, and the ordering is argued.** §3.3 states the point I care
most about: "the first triage question is *which explanation owns the disagreement*; deduplication
comes afterward." §4.5 and Table 3 then implement exactly that, with a Rust replay *without*
sanitizers to separate a program panic from an instrumentation-only termination. The "recoverable
minimal-UBSan check … serves only as an inexpensive noise filter; the isolated runs perform
attribution" sentence is the right separation of a cheap filter from an adjudicator.

**S5. The funnel numbers are real.** I recomputed Figure 3's chain from
`results/rq3_coverage/*/*/confirm_sample/summary.json`: 176,507 saved candidates over the five
restructuring systems, 14,470 adjudicated, 13,515 after removing 955 `not_reproducible`, 3,365
after removing 7,318 `ub_associated` + 2,831 `ub_associated_termination` + 1 `ub_associated_value`,
and 1,931 = 741 `confirmed_divergence` + 1,190 `confirmed_termination`, with the residual
1,434 = 751 `instrument_only` + 434 `out_of_contract_access` + 248 `inconclusive` + the one
verdict-less Laertes record the caption already discloses. Every figure in §6.2 reproduces exactly.
Likewise Table 7's M/P/B columns sum to exactly the 4,222 / 1,854 / 1,662 reported in the RQ4
summary, and the defect manifest's family counts match Table 6 term for term. I do not often get
to write this paragraph.

**S6. Denominator provenance was actually checked.** §6.4: coverage denominators come from each
translation's own instrumented archive objects rather than from a linked test binary, and "for 11
artifacts initially measured through a linked test or denominator binary, rebuilding the universe
from the archive objects reproduced the same in-scope functions and region counts." That is a
measured provenance check, not an assertion.

**S7. Honest hedging in the right places.** "Sanitizer-clean replay means that no configured check
fired; it is not proof that the C execution is free of all undefined behavior" (§4.5); "These
reductions are candidate accounting, not an estimated false-positive rate" (§6.2); "this RQ
measures applicability and positive-case sensitivity on these known defects, not general recall"
(§6.5); the selection-bias disclosure in §7.2. The paper does not pretend.

**S8. RQ5 separates applicability from semantic miss.** Table 9's S/A/C/R/D funnel, and the refusal
to count VERT's 36 unsupported as misses, is the right way to report a competitor that does not fit
your setting.

## 4. Weaknesses (ranked by influence on my decision)

### W1 (decisive). The adjudicator that the title is about is itself unmeasured and unspecified.

*Where:* §4.5; Table 3 footnote; §6.2; §7.1 "Sanitizer completeness"; related_work.tex ¶ on oracle
qualification.

The paper's thesis is "when is a difference a defect", and the answer is operationally: *when C-only
ASan + "full UBSan" replay is clean*. Three problems.

(a) **The check set is never enumerated.** §4.5 says "ASan and full UBSan"; Table 3's footnote says
"Clean C uses full UBSan"; §6.2 says "clean C-only replay under the configured checks". Nowhere does
the paper list which checks are enabled. This matters concretely: `-fsanitize=undefined` does *not*
include `unsigned-integer-overflow`, `implicit-integer-conversion`, or `implicit-integer-sign-change`,
and several of the paper's own defect families (byte-string domain narrowing, "which integer
conversions wrap or truncate", §6.3) live exactly in that gap. A paper whose title is a definedness
question must print its definedness configuration.

(b) **The gate does most of the work and its error rate is never estimated.** From the archive:
10,150 of 14,470 adjudicated restructuring candidates (70.1%) are removed as reference-associated,
and for the c2rust controls the figure is 7,036 of 8,962 adjudicated (78.5%). So both the 36 and the
headline zero are, quantitatively, products of this gate. The paper cites UBFuzz~\cite{ubfuzz}
precisely to say that production sanitizers have false negatives — and then relies on production
sanitizers alone, offering only the disclaimer in §7.1. A false negative here promotes a UB-caused
difference to a "defect"; a false positive discards a genuine defect. Neither direction is bounded.

(c) **The cheapest corroborating oracle in this lens is missing.** Csmith/EMI practice, and my own
work, would not accept sanitizer silence as the sole definedness signal: the standard cheap
cross-check is C-side self-consistency — compile the C reference at `-O0/-O1/-O2/-O3`, and with a
second compiler, on the *same* saved input, and confirm the C observations agree. Divergence there is
strong evidence of latent UB that the sanitizers missed; agreement materially strengthens every one
of the 36. The paper never runs this, and it requires no new machinery: the confirmation driver
already replays the C side in isolation.

*What would resolve it:* (i) print the exact sanitizer flag set; (ii) report C-side self-consistency
across at least two optimization levels and two compilers for all 36 confirmed defects, and for a
sample of the excluded `ub_associated` records; (iii) a seeded-UB positive control — inject known UB
of each class into a C reference and report what fraction the configured gate actually catches, which
is exactly the UBfuzz-style experiment and would turn §7.1's disclaimer into a number.

### W2 (decisive). The negative control has no measured detection power, and per-system counts are uncontrolled for exposure.

*Where:* Abstract; §1; §6.2 ¶3–4; Table 5; §4.3 "Harness qualification"; §6.3 ¶"strategy-specific
signatures".

"No confirmed defect appears in the ten mechanical c2rust controls" is used in the abstract, the
intro, RQ2 and the conclusion. I like that the control exists — most papers in this space have none.
But as reported it cannot discriminate between *the controls are clean* and *we could not have found
a defect there anyway*:

- There is **no positive control**. Nothing in the paper establishes that the pipeline, run on a
  c2rust artifact at the same budget, would detect a defect if one were present. A defect-injection
  experiment (mutate the c2rust Rust output with the same seven mechanism families, re-run, report
  recall) is the obvious and cheap answer, and it is standard practice for exactly this claim.
- **No statistical test.** §6.2 says "this markedly lower observed incidence" for 0/10 versus 15/25.
  A one-line Fisher exact test on that contingency table is expected here, not optional.
- **Exposure is not matched.** Table 7 shows the controls and the restructuring artifacts are not
  comparably exercised: cJSON c2rust builds 39 harnesses and reaches 81.2% regions while cJSON
  PtrTrans builds 9 and reaches 3.2%; optipng c2rust builds 54 harnesses while C2SaferRust builds 96.
  Yet §6.2 reports raw per-system totals ("20 occur in C2SaferRust, 7 in Laertes, four each in
  PtrTrans and CROWN, and 1 in SACTOR") and §6.3 escalates to "the families also form clear
  strategy-specific signatures in this corpus". With built-harness counts, reached regions, and
  adjudicated-candidate counts differing by an order of magnitude across those cells, a raw count
  ranking is not a measurement of the translators; it is a measurement of reach. Either normalize
  (defects per built harness, per adjudicated candidate, or per reached region) or drop the
  cross-system ranking.
- The controls' **own funnel is never shown**. From the archive the c2rust controls produced 68,981
  candidates, 8,962 adjudications, and zero confirmed records. That is a *stronger* statement than
  the one in the paper and it belongs in Figure 3 as a second panel — but it also makes point (b) of
  W1 unavoidable, because 78.5% of those adjudications were removed by the UB gate.

### W3 (decisive). The baseline arm has no negatives and no failure diagnosis.

*Where:* §6.5; Table 9.

FLOURINE and RustAssure are run **only on the 36 positives**. There is therefore no precision for
either baseline and no way to read 14/16 or 5/11 as sensitivity: if FLOURINE also reports a
difference on the ten c2rust controls or on the ten restructuring artifacts with zero confirmed
defects, 14/16 means nothing. Running both released artifacts on those defect-free pairs and
reporting their false-positive counts is a small, cheap, and mandatory addition.

Second, the funnel losses are reported as bare counts with no attribution. RustAssure fails to
compile 22 of 34 accepted submissions (65%); FLOURINE has 14 analysis failures of 33 submitted.
§6.5 asserts "We do not patch baseline analysis … adapters only package the existing pair", but
never shows that those 36 failures are the baselines' own limits rather than adapter artifacts. When
I report that a competitor underperforms, I say *why* at a mechanism level (e.g. a directed fuzzer
that cannot start because it requires a CFG shape absent from real patches); the same standard
applies here. A short taxonomy of the 22 C-fails and 14 A-fails, with one representative error per
class, would settle it.

Third, the conditional metric is small-n and the abstract does not say so: 16 and 11 completed
analyses out of a 36-case corpus, compared with no interval. The table is honest; the abstract's
"detect 14 of 16 and 5 of 11" is not enough context.

### W4 (major). The headline count rests on unblinded single-team judgement, with unevaluated clustering and no validator false-positive rate.

*Where:* §4.5 last ¶; §6.2 ¶2; Figure 3 caption; §7.2 "Bug and mechanism coding"; Table 6.

- "Our automated grouping … groups candidates by adjudicated verdict, sanitizer class, and top
  failure site. Human review then verifies provenance and boundary correspondence, minimizes a
  representative input, and traces the translated change to a source-level cause. Only that final
  evidence promotes a cluster to a defect." (§4.6) The 176,507 → 36 reduction is therefore decided
  by a merge policy that is never evaluated. There is no measurement of over-merge (two distinct
  faulty rewrites collapsed into one) or under-merge, even though the paper's related-work section
  positions this against Igor and FuzzerAid, both of which *do* evaluate their grouping.
- **No false-positive rate at the human stage.** How many clusters entered source-level review, and
  how many were rejected there (correspondence failure, provenance mismatch, no identifiable
  rewrite)? That ratio is the validator's precision and it is recoverable from the archive. Figure 3
  stops at 1,931 confirmed records and jumps to 36 defects with no intermediate cluster count. As
  written, the paper reports no precision number of any kind for its own tool.
- **No inter-rater agreement and no per-defect evidence.** §7.2 discloses that "the same researchers
  discovered and categorized the defects" — good — but the mitigation ("we separate mechanism from
  symptom, preserve per-instance evidence") is a procedure, not a measurement. Table 6 assigns each
  of 36 defects to exactly one of seven families with no agreement statistic and no second coder.
  The paper needs either a blind re-coding of a sample by someone outside the discovery loop, or a
  per-defect appendix table (defect id, artifact, boundary, distinguishing C line, distinguishing
  Rust line, observed symptom) so a reader can audit the merge and the family assignment themselves.
- **No upstream confirmation.** 36 defects are claimed in artifacts published by five research
  systems, and not one is reported as confirmed or fixed by those systems' authors. In my own work a
  bug count is only as good as the maintainer's response; here every one of the 36 is self-adjudicated.
  I accept that research prototypes are not GCC, but the paper should say whether any were reported,
  to whom, and what came back.

### W5 (major). The component analysis compares against strawmen, on a quarter of the corpus.

*Where:* §6.6; Table 10; and its direct conflict with related_work.tex.

Table 10's first row reports "Function state: 9/9" versus "Return only: 6/9", and §6.6 concludes
"Observation has the largest measured effect on defect recovery." But related_work.tex states, about
the very tools this paper positions against: "FLOURINE compares serialized output states, and
RustAssure compares symbolic output parameters as well as explicit returns; both therefore go beyond
return-only checking." So the reduced configuration that establishes the paper's largest component
effect is *weaker than the acknowledged state of the art as the paper itself describes it*. The
same applies to the "silent stdout/exit" driver row (3/9) — no prior system in §3 uses a silent
driver as its oracle. Ablating to a variant nobody proposes inflates the apparent contribution. The
correct reduced configurations are: FLOURINE's serialized-output-state comparison, and
RustAssure's returns-plus-output-parameters.

Separately, all five rows use fixed evidence sets of size 9, 9, 3, 1, and 1. The observation row
uses 9 of the 36 defects and the paper never says why the other 27 are excluded (the manifest calls
them `obs_not_run_defects`). §7.2 explains what the component analysis *is not*, but not why the
denominator is 9. If the other 27 cannot be replayed under the reduced configurations, say so and
say why; otherwise run them.

### W6 (major). Protocol is not uniform across the corpus, and single-run budgets carry cross-cell conclusions.

*Where:* §6.4; Table 7 caption; §7.2 "Fuzzing randomness and budgets".

- Table 7's caption states "Tulip reports the deterministic seed-refined campaign described in RQ4."
  One application of ten is reported in the headline reach table under a *different and better*
  protocol than the other nine, and the summary range ("8.3–100% of functions and 2.8–100% of
  regions") and the intro's characterization are computed over that mixture. The seed set
  \{1,2,3,5,10,20\} is chosen from knowledge of the target's period guard — that is target-specific
  oracle tuning. The paper does the right thing in the text (it reports the unseeded 24.0–34.7%, a
  length-matched random control at 38.7%, and a 600s unseeded control on two artifacts), which makes
  it all the more puzzling that the *table* shows the tuned numbers. Report the uniform protocol in
  Table 7 and keep seed refinement as a clearly separated ablation; or apply the same seed-derivation
  procedure to all ten applications and show it is not Tulip-specific.
- Every campaign is a single one-hour run at one seed. There is no repetition, no variance, and no
  defects-versus-time or coverage-versus-time curve, so a reader cannot tell whether 36 is near
  saturation or the visible tip of a much larger set, nor whether the ten zero-cells are zeros or
  under-sampled. §7.2 acknowledges this framing ("one fixed-budget observation") but the paper still
  draws cross-system and cross-artifact conclusions from those single observations (W2). At minimum:
  5 repetitions on a representative subset with per-run defect counts, and one saturation curve.
- The confirmation sample is capped at 200 per boundary. From the archive, 96 of 567 boundaries with
  candidates (17%) were capped, including 40 of 154 c2rust control boundaries; one bzip2 boundary
  had 26,814 candidates and 200 adjudications. The paper mentions the cap only to disclaim a
  false-positive rate (§6.2). It should also state the cap's effect on *recall*: a distinct root
  cause whose only witnesses sit outside a capped boundary's 200 is invisible, in the controls as
  well as the treatment cells. Report the capped-boundary count, whether the deterministic sample is
  stratified by cluster or by file hash, and a spot check on one or two high-volume boundaries with
  the cap raised.

### W7 (moderate). The "no crash signal" claim is definitional, not measured — although the paper has the data to measure it.

*Where:* Abstract; §1; §6.2 ("Rust-only crash fuzzing has no failure signal for their normally
terminating executions"); §8.

This is logically true by construction and therefore carries no information: a defect classified as
*normally terminating* cannot, by definition, be a crash. What a reader wants is the measured split
from the authors' own pipeline, which already runs Rust-only discovery as its first stage: of the 36,
how many were first surfaced by the Rust-only termination stream, and how many *only* by paired
replay? That is the honest quantification of "crash fuzzing would not have found these", and it
distinguishes the contribution of the relational oracle from the contribution of harness generation
and the UB gate. As stated, the claim credits the oracle for a partition it did not measure.

### W8 (moderate). A denominator in RQ2 does not close.

*Where:* §6.2 ¶3; Table 5.

"15 of the 25 restructuring artifacts on which a comparison was performed contain at least one,
accounting for all 36 defects." Counting Table 5: 50 restructuring cells minus 14 $\times$, 2 Partial,
6 "--", and 3 UB-only leaves exactly 25 — which *excludes* cJSON $\times$ PtrTrans, marked
"Partial + 3S". But that cell is one of the 15 artifacts with a defect and contributes 3 of the 36.
So the numerator and the denominator use different inclusion rules, and the sentence as written is
not self-consistent. Fix the accounting rule and state it once (is a Partial artifact on which a
comparison *was* performed inside or outside the denominator?).

### W9 (moderate). Matcher operating point and abstention cost are unquantified.

*Where:* §4.2; §6.1; Algorithm 1.

$\epsilon = 0.01$ is called "preselected" twice but the selection procedure is never given, and there
is no sensitivity curve over $\epsilon$ — a one-parameter sweep that costs nothing and is the natural
figure for this section. More consequentially, §6.1 reports that the operating point "accepts 78.4%
of pairs with 0.993 macro-average precision", and §6.2 states that "abstained pairs remain untested".
So roughly one boundary in five is silently outside the defect search, and the paper never estimates
how many defects that costs. A single run with $\epsilon = 0$ (forced matching) on two or three
applications, reporting additional confirmed defects and additional false candidates, would bound it.
Note also that Panel (a)'s 0.938 is a macro-average over applications while the 4,041 pair instances
are explicitly *not* the weighting — with Tulip at 0.554 the two aggregations differ substantially,
and both should be shown.

### W10 (minor, but it affects reviewability). The artifact is a placeholder.

*Where:* introduction.tex, contribution bullet 4: "We release the validator, evaluation artifacts,
and replayable defect corpus at \url{ANONYMOUS_LINK}".

Under double-anonymous review this should be an anonymized archive that a reviewer can actually open.
As submitted there is nothing to evaluate. I can verify the funnel only because I happened to be able
to reach the authors' result files; a normal reviewer cannot. For a paper whose entire argument is
"the assumptions must be auditable", shipping an unopenable artifact link is a self-inflicted wound.

### W11 (minor). Coverage percentages are aggregated across incomparable denominators.

*Where:* §6.4; Table 7.

The paper correctly warns that a translation can add code to its own measured universe and that raw
region counts should not be compared across translations, and the Laertes Tulip decomposition
(64.9% artifact-level vs 92.0% excluding three translator-added functions, 3,887 regions of which
belong to one severed initializer) shows how large the effect is. Yet the summary still quotes a
single cross-artifact range. A shared-code denominator — regions in functions with a matched
counterpart — reported as a second column would make Table 7 comparable without abandoning the
artifact-level number.

## 5. Detailed comments by section

**§1 Introduction.** The four obligations are the right decomposition and the qsort lead is good. Two
content issues. (i) The claim "No confirmed defect appears in the ten mechanical c2rust controls
under the recorded testing budgets" appears here without the power caveat that §4.3 and §6.2 later
supply; the intro should carry the qualification, not only the later sections. (ii) The
contributions list promises "an empirical characterization of translation defects across six systems
and ten applications", but the c2rust column contributes zero defects and the characterization is of
five systems; the claim is stronger than the content.

**§2 Motivating Study.** Good. Table 1's rows are described as measured on "the exact motivating
executions" — please state explicitly that all four rows were executed for all three cases (in
particular, that the "Rust-only crash or panic" row is an actual run and not an inference from the
defect being non-crashing, which would make it circular with W7).

**§3 Related Work.** The positioning is careful and the "supplying our generated harnesses to these
frameworks would therefore reuse the component being evaluated" argument (about cozy, DIFFER, the
C2Rust cross-checker) is correct and well made. Two things. (i) The paragraph beginning "Tulip
exposes an incorrect return value…" reads as commentary on a comparison table that does not appear
in the rendered paper (`table/validator_scope.tex` is entirely commented out); as a result the
mechanism claims about FLOURINE/VERT/RustAssure are asserted in prose with no accompanying evidence
column. (ii) That same paragraph is the source of the conflict in W5: if FLOURINE and RustAssure
"go beyond return-only checking", then "return only" cannot be the ablation baseline in §6.6.

**§4.2 Function Matching.** The two-sided margin is a clean formulation and excluding identifiers
from the similarity computation is the right call. The Tulip failure analysis in §6.1 — 213
functions collapsing to 103 static fingerprints, 95 identical wrong assignments in all four outputs,
54 `ti_*_start` exchanges and 41 unary-indicator exchanges — is exactly the kind of failure
explanation I want to see, and it is the best-argued negative result in the paper. Please also state
what happens downstream: those 95 mismatches are in a *control-and-treatment* application; do they
manufacture candidate divergences, and were any of them the reason a candidate was rejected at human
review? That connects §6.1 to W4's missing precision number.

**§4.3 Harness Planning.** The producer-bridge design and the explicit refusal to construct a `FILE *`
are well argued. Two gaps. (i) "The current generator handles scalars, nullability, strings,
pointer–length buffers, Rust slices, scalar output parameters, fixed arrays, and plain structures
composed of scalar fields" — this is the represented input domain, and the paper never quantifies the
*domain-narrowing risk* of the adaptations themselves, even though §1 identifies exactly that as a
hazard ("If arbitrary C byte strings are filtered through a Rust string representation … non-UTF-8
inputs may be discarded, hiding exactly the domain narrowing that validation should detect", in the
commented-out text). Since "byte-string domain narrowing" is the second-largest defect family (7 of
36), the paper must show its own bridges do not narrow: e.g. an adapter-level test that every byte
string admitted on the C side is admitted at the Rust boundary. (ii) "Operationally, harness
construction is followed by screening… generated harnesses must compile and survive a short
execution before they are used" — how many harnesses were discarded by screening, and does a
discarded harness ever coincide with a translation-limited cell (i.e. is screening silently removing
boundaries where the translation is the thing that fails)? That is a real confound for RQ4 and it is
not addressed.

**§4.4 Discovery and Replay.** "The discovery loop does not run C; C participates when the saved
corpus is replayed" is an important design fact and it is stated clearly. Consequence worth stating:
the search is guided only by *Rust* coverage, so the corpus is biased toward Rust-side behaviors and
systematically under-explores C behaviors that the translation cannot reach — which is precisely the
class where a translation-limited artifact hides its own defects. The cJSON × PtrTrans analysis in
§6.4 is a nice instance of noticing this; it should be generalized into a stated limitation of the
discovery design, not only a per-cell observation.

**§4.5 Confirmation.** See W1. One additional specific: "Timeouts are measured independently on each
side" and a timeout "must be attributed to Rust alone while C returns normally". What timeout values,
and were they tuned per application? A translation that is merely *slower* (e.g. bounds checks in a
hot loop) is not a defect, and the paper's threshold is the only thing separating the two.

**§5 Study Subjects.** The frozen-artifact / no-resampling policy is exactly right for LLM-based
translators and clearly stated. Please also record which model and which version produced each
LLM-based artifact, and the date — "one recorded translation run" of SACTOR or PtrTrans in 2025
versus 2026 are different experiments, and a reader cannot reproduce the corpus without that.

**§6.1 RQ1.** Precision is reported; recall at the operating point is not (see W9). Panel (b)'s
forced precision of 0.516 is explained honestly ("PtrTrans's bzip2 and lodepng outputs contain many
stubs or C functions with no one-to-one Rust counterpart, for which a forced assignment must be
wrong") — but then the number is not measuring the matcher, it is measuring the artifact, and the
Overall row aggregates it anyway. Report the forced precision restricted to C functions that *have*
a true counterpart, alongside the current value. The rename result (7/9 vs tool maps 4/8 vs name
equality 0/9) is the interesting finding and is under-sold; n = 9 is small and should be labelled
as such.

**§6.2 RQ2.** See W2, W4, W7, W8. One further point: the SACTOR failure analysis ("its topological
translation order rejects the recursive dependency cycles in cJSON, quadtree, and lil… allocator
callbacks stored in structure fields prevent parsing bzip2 and optipng…") is excellent — it is a
mechanism-level diagnosis of why a competitor fails, and it is exactly the standard I asked for in
W3 and did not get for FLOURINE and RustAssure. Apply the same rigor there.

**§6.3 RQ3.** The taxonomy is plausible and the two case studies are the best technical writing in
the paper; the `state->offset > copy` → `state as usize > copy` case, with 169 confirmed
output-length divergences and 4 null-window dereferences from the same rewrite, is precisely the
kind of "one root cause, many artifacts" evidence that justifies the merge policy — in that one
instance. Generalize it: give the per-defect witness counts for all 36 so the merge is auditable
(W4). The "strategy-specific signatures" paragraph overreaches given exposure differences (W2).

**§6.4 RQ4.** The four-way decomposition of low reach (construction-limited / translation-limited /
reference-limited / domain- and state-limited) is genuinely useful and the cJSON × PtrTrans analysis
— `cJSON_New_Item` is an untranslated stub returning `None` on all 18 direct-constructor witnesses,
so "adding an `Option<&mut T>` adapter or running longer could not unlock the 64 object-taking
boundaries" — is a model of attributing a ceiling to the right component with same-input isolated
replay. This is the section that most resembles work I would champion. It is undercut by the
non-uniform Tulip protocol (W6) and the mixed denominators (W11).

**§6.5 RQ5.** See W3. Also: the phrase "their outcomes neither define the corpus nor alter its defect
classification" is a good safeguard and should be accompanied by the converse experiment — were any
of the baselines' *additional* reports (on the same 36 artifacts) inspected, and did any of them
identify a real defect this validator missed? That would be the first honest false-negative signal
for the proposed tool, of which the paper currently has none.

**§6.6 Component Analysis.** See W5.

**§7 Discussion / Threats.** Better than most. The selection-history disclosure ("frozen after
exploratory work, not sampled prospectively… Earlier notes explicitly allowed replacement subjects")
is unusually candid and I credit it. The gap is that several threats are stated and then not
measured when measurement is available: sanitizer completeness (W1), budgets and randomness (W6),
adapter-induced precondition strengthening (§4.3 comment above). A threat that could have been turned
into a number and was not is a weaker threat statement, not a stronger one.

**§8 Conclusion.** "These results show that translation validation requires both broad execution and
a relational oracle capable of observing and attributing semantic change." The paper demonstrates
that a relational oracle *finds* things crash-only testing cannot; "requires" is stronger than what
was measured, since no experiment establishes that the relational oracle is necessary rather than
sufficient.

## 6. Questions for the authors

1. Print the exact sanitizer configuration used for "full UBSan" in C-only confirmation. Are
   `unsigned-integer-overflow`, `implicit-integer-conversion`, and `implicit-integer-sign-change`
   enabled? If not, how do you exclude the possibility that defects in the "byte-string domain
   narrowing" and integer-conversion families are UB-caused rather than translation-caused?
2. For each of the 36 confirmed defects, does the C reference produce identical observations when
   compiled at `-O0`, `-O2` and `-O3`, and under a second compiler (GCC and Clang)? If you have not
   run this, can you run it for the rebuttal? A disagreement on any of the 36 would change that
   defect's classification.
3. Can you report a measured detection power for the c2rust control — e.g. inject one defect from
   each of the seven mechanism families into a c2rust artifact and report how many the pipeline
   recovers at the same one-hour budget? Without it, how should a reader distinguish "the controls
   are clean" from "the controls were not effectively searched"?
4. What is the Fisher exact p-value for 0/10 controls versus 15/25 restructuring artifacts, and does
   the contrast survive normalizing by built harnesses or by adjudicated candidates per cell?
5. How many clusters entered human source-level review, and how many were rejected there and for
   which reason (correspondence failure / provenance mismatch / no identifiable rewrite / other)?
   That ratio is your validator's precision and it is absent from the paper.
6. Of the 36 defects, how many were first surfaced by the Rust-only discovery stream (crash/panic/
   timeout artifacts) versus only by paired replay of the saved corpus? This is the measured form of
   the "no signal to Rust-only crash fuzzing" claim.
7. What are the 22 RustAssure compilation failures and the 14 FLOURINE analysis failures caused by?
   Please give a short taxonomy with one representative error per class, and state what evidence
   excludes the adapter as the cause.
8. What do FLOURINE and RustAssure report on defect-free pairs — the ten c2rust controls and the ten
   restructuring artifacts with zero confirmed defects? Without their false-positive counts, 14/16
   and 5/11 cannot be interpreted.
9. Why is the observation ablation run on 9 of the 36 defects? What prevents the other 27 from being
   replayed under the reduced observation configurations?
10. Please re-run the observation ablation against FLOURINE's serialized-output-state comparison and
    RustAssure's returns-plus-output-parameters rather than "return only" and "silent stdout/exit",
    which your own related-work section says are weaker than the state of the art.
11. In §6.2, is a "Partial" artifact inside or outside the 25-artifact denominator? cJSON × PtrTrans
    appears to be outside the 25 but inside the 15 and contributes 3 of the 36.
12. How was $\epsilon = 0.01$ selected, and what does the precision/acceptance curve look like over
    $\epsilon \in [0, 0.1]$? How many additional confirmed defects appear on two or three
    applications if you run the pipeline at $\epsilon = 0$?
13. 96 of 567 boundaries hit the 200-candidate confirmation cap. Is the deterministic per-boundary
    sample stratified (by cluster, verdict, or file) or is it a hash order? What happens on one or
    two high-volume boundaries if the cap is raised to 2,000?
14. Were any of the 36 defects reported to the authors of Laertes, CROWN, C2SaferRust, SACTOR, or
    PtrTrans, and were any confirmed or fixed?
15. Was any part of the defect identification or family assignment re-coded by someone outside the
    team that built the validator? If not, can you provide a per-defect table (id, artifact,
    boundary, distinguishing C line, distinguishing Rust line, symptom) so the merge and the taxonomy
    can be audited?
16. Why does Table 7 report Tulip under the seed-refined protocol while the other nine applications
    use the uniform one-hour campaign? Can the seed-derivation procedure be applied uniformly, or is
    it Tulip-specific?
17. What timeout values were used, per side and per application, and how do you separate a
    translation that is merely slower from one that hangs?
18. Do your input adapters preserve the full C-representable domain? Specifically, is every byte
    string accepted by a C boundary also constructible at its Rust counterpart, given that
    byte-string domain narrowing is 7 of your 36 defects?
19. Which model and version produced each LLM-based translation artifact, and on what date?
20. Will an anonymized artifact be available? `ANONYMOUS_LINK` currently resolves to nothing.

## 7. What would move this up one grade

To reach **Weak Accept** I would need, at minimum, three things measured rather than disclaimed.
First, the definedness gate must stop being a black box: print the check set, and corroborate it with
a C-side self-consistency cross-check (two optimization levels, two compilers) on all 36 confirmed
defects plus a sample of the excluded `ub_associated` records — if the 36 survive that, the paper's
central claim becomes evidence-backed instead of assumption-backed. Second, the negative control
needs power: a defect-injection positive control on the c2rust artifacts, plus a significance test on
the 0/10 versus 15/25 contrast, plus one exposure-normalized view of the per-system counts so the
"strategy-specific signatures" claim is about translators rather than about reach. Third, both arms
need their missing negatives: a cluster-level rejection count that yields a false-positive rate for
this validator, and a run of FLOURINE and RustAssure on defect-free pairs so their 14/16 and 5/11
have a precision to sit against. Fixing W5 (ablate against the prior tools' actual oracles, not
return-only), W8 (close the 25/15 denominator), and W10 (a real anonymized artifact) is comparatively
cheap and would remove the remaining objections. For a full **Accept** I would additionally want
repeated campaigns with variance, a uniform reporting protocol across all ten applications, an
auditable per-defect evidence table, and some upstream response from at least one translator's
authors.

## 8. Self-disclosure

Weaknesses W1 (unmeasured/unspecified definedness oracle; the C-self-consistency and seeded-UB
asks), W2 (missing positive control and unexposure-normalized counts), W3 (no baseline negatives, no
mechanism-level diagnosis of baseline failures), W4 (no measured false-positive rate, unevaluated
clustering, no upstream confirmation) and W6 (single-run budgets, non-uniform protocol) follow
directly from the profiled reviewer's published standards — his admission-gate-with-measured-FP-rate
design in Archer, his null-baseline and database-cross-substitution ablations and explicit
Confirmed/Fixed/Duplicate/Not-Planned accounting in LegoFuzz, his UBfuzz result that production
sanitizers have false negatives, and his three artifact-evaluation committee terms. W5 (the ablation
is a strawman relative to the paper's own description of prior tools), W7, W8, W9 and W11 are my own
readings of internal inconsistencies in this manuscript, though the underlying standard
(denominators must close; a claim must be measured, not defined into existence) is his. The archive
cross-checks I report in S5 and in W1/W2/W6 are my own computations from
`results/rq3_coverage/*/*/confirm_sample/summary.json` and
`results/rq4_effectiveness/defect_manifest.json`, performed to test whether the paper's numbers are
backed; the profiled reviewer would not have had that access, and a normal FSE reviewer would not
either — which is itself the point of W10.
