# PC Profile — Joshua Sunshine (FSE 2027 Research Papers PC)

Lens: Rust, C-to-Rust translation, memory safety, unsafe-code analysis.
Prepared: 2026-09-14. All web evidence fetched on that date.

---

## (a) Who / affiliation / which PC list

**Joshua Sunshine**, Associate Professor, Software and Societal Systems Department (S3D),
Carnegie Mellon University; courtesy appointment in the Human-Computer Interaction Institute.
Director of the REUSE undergraduate research program. PhD CMU 2013 (usability of API protocols).
Homepage: <https://www.cs.cmu.edu/~jssunshi/>

**PC list used: FSE 2027 Research Papers — the real, currently published list.**
It is public at
<https://conf.researchr.org/committee/fse-2027/fse-2027-papers-program-committee>
(note: the URL given in the task, `.../fse-2027-research-papers`, returns 404; the working slug is
`fse-2027-papers-program-committee`). Chairs are Michael Pradel (CISPA) and Baishakhi Ray
(Columbia); there are 20 area chairs and ~500 PC members. Sunshine is listed as a regular
**Program Committee member** ("Joshua Sunshine — Carnegie Mellon University — United States"),
not a chair or area chair. No fallback to FSE 2026 was needed.

**Why him for this lens.** He is the senior author on the CMU line of work that is the closest
existing neighbour to UB-gated differential testing of translated C: MiriLLI (running Rust and
LLVM bitcode in one interpreter to find Rust-specific UB *across* the C boundary), the
mixed-methods study of what unsafe-Rust developers actually need from tooling, and the ongoing
BorrowSanitizer project (an LLVM sanitizer for Tree Borrows explicitly designed to be *fast enough
to fuzz with*). He is not a C-to-Rust *transpiler* author — his angle is the oracle and the
adjudication of unsafety, which is exactly where our work lives. He was also not picked for fame:
his h-index and visibility are moderate compared to several chairs on the same list, and all of his
relevant papers are openly downloadable, which is why I could read them in full.

---

## (b) The three papers

I used his three most recent publications **within this lens**. Publication years from his own
publications page. (He has a fourth recent paper, "Visual Consistency in Formal Modeling: An
Empirical Evaluation", TOSEM 2026, with Eunsuk Kang — off-lens, abstract only; see §(g).)

### P1 — A Mixed Methods Study on the Implications of Unsafe Rust for Interoperation, Encapsulation, and Tooling
Ian McCormack, Tomás Dougan, Sam Estep, Hanan Hibshi, Jonathan Aldrich, **Joshua Sunshine**.
**TOSEM 2026.** DOI <https://doi.org/10.1145/3798277> · arXiv <https://arxiv.org/abs/2404.02230> (v4, 18 Feb 2026).
*Read: full text (arXiv HTML v4) — abstract, intro, background, methodology, ethics + threats, discussion, conclusion; results skimmed.*

Semi-structured interviews with 19 developers who regularly write unsafe Rust, followed by a
community survey with 160 valid responses, asking four questions: how developers reason about
memory safety across FFI boundaries, what tools they use, why they reach for `unsafe`, and how they
reason about encapsulating it. The headline findings are that most `unsafe` use is for foreign
function calls; that Miri is simultaneously the most-used and most-complained-about tool (too slow,
no FFI support); that developers use `unsafe` mostly from perceived necessity but also for
ergonomics and performance; and that almost nobody is *certain* their safe encapsulation is sound
(only 23% were always certain). The paper converts these into concrete tooling asks — a fast
Rust-aware sanitizer that crosses the FFI boundary, static checks on bindings, and better
documentation of contested semantics.

### P2 — A Study of Undefined Behavior Across Foreign Function Boundaries in Rust Libraries
Ian McCormack, **Joshua Sunshine**, Jonathan Aldrich.
**ICSE 2025.** DOI <https://doi.org/10.1109/ICSE55347.2025.00167> · arXiv <https://arxiv.org/abs/2404.11671> (v5).
Artifact: <https://github.com/icmccorm/mirilli>
*Read: full text (arXiv HTML v5) — abstract, intro, background, methodology, evaluation protocol, discussion, related work, threats, conclusion.*

They build **MiriLLI**, which welds Miri (Rust MIR interpreter, enforces Stacked/Tree Borrows) to
LLI (an LLVM bitcode interpreter) so that a Rust test and the C/C++ it calls execute inside one
checked semantics. They start from a 125,804-crate crates.io snapshot, funnel down to 9,130 test
cases from 957 crates that actually call a foreign function they can execute, and find **47
instances of undefined or undesired behavior in 37 libraries**, three of them in crates with >10k
daily downloads and one in a crate maintained by the Rust Project. RQ2 compares the two aliasing
models as *specifications*: of 90 tests that violated Stacked Borrows, 66% (59) did not violate Tree
Borrows. The conclusion is deliberately anti-triumphalist — "Our goal was not to create a
production-ready tool, but to understand the types of errors that can occur" — and the call to
action is that the Rust Project must fund real tooling here.

### P3 — TerzoN: Human-in-the-Loop Software Testing with a Composite Oracle
Matthew C. Davis, Amy Wei, Brad A. Myers, **Joshua Sunshine**.
**FSE 2025** (Proc. ACM Softw. Eng. 2, FSE, Article FSE089). DOI <https://doi.org/10.1145/3729359> ·
PDF <https://www.cs.cmu.edu/~jssunshi/assets/pdf/terzon.pdf>
*Read: full text (PDF) — abstract, intro, scope/limitations, evaluation design, results, discussion, threats, conclusion, data availability.*

Off the Rust lens but squarely on the *oracle* question, which is why it matters here. The paper
defines a **Composite Oracle**: a hierarchy that combines an implicit oracle (crashes, NaN),
example-based assertions, and property-based assertions into one verdict per execution, and that
surfaces *disagreements between oracle types* as a first-class signal to the user. TerzoN
instantiates it for TypeScript in VS Code. Evaluation is a randomized controlled trial with 14
professional engineers, control = fast-check: 72% more bugs elicited (p<0.01), more than twice as
many bugs accurately *described* (p<0.01), 16% faster (p<0.05); the confidence measure improved 39%
but **did not reach significance**, and they say so. The intro itself concedes the study cannot
apportion the effect between the oracle design and the UI.

### Context: ongoing work
**BorrowSanitizer** (McCormack, Braunsdorf, Kinder, Aldrich, **Sunshine**) — an LLVM-based Tree
Borrows sanitizer, deliberately designed for "compatibility with fuzzing tools" because Miri is up
to 1000x slower than native. Short paper/talk: <https://borrowsanitizer.com/pdfs/rw2025.pdf>;
project site <https://borrowsanitizer.com/>; it is a 2026 Rust Project Goal. *Read: full short
paper (~1.8k words).* This tells you what he thinks the field needs next, and it means he will read
a differential-fuzzing-with-a-UB-gate paper as a direct neighbour of his own agenda.

---

## (c) Writing taste

**Contribution framing.** Questions, not conquest. Every paper states 2–4 explicitly labelled,
*one-sentence* RQs ("RQ1: What types of errors occur in Rust libraries that call foreign
functions?"; "RQ2: Which of Rust's aliasing models permits more real-world programs...?"). In P1 the
RQs are even given names — RQ1 (Interoperation), RQ2 (Tooling), RQ3 (Motivations), RQ4
(Encapsulation). P3 numbers its contributions C1/C2/C3 and points each at a section. Tools are
framed as *instruments for a study*, not as the result: MiriLLI exists to answer RQ1/RQ2, and he
says outright that building a production tool was not the goal.

**What counts as evidence.** Counts with denominators, essentially without exception. Almost every
percentage in P2 carries its raw number: "67% (84,106)", "39% (9,130)", "66% (59)", "2% (156) of our
test cases from 15% (140) of crates". He reports the full **attrition funnel** — 125,804 crates →
121,015 valid → 84,106 compiled → 3,785 with both tests and bitcode → 9,130 runnable FFI tests → 394
deduplicated errors → 47 bugs — including the unflattering middle: 61% of test executions died on an
unsupported operation and 10% timed out, broken down by cause. A bug is only a bug after manual
investigation *and* a disclosure attempt; the paper describes an ethical disclosure protocol (private
email first, public issue after one month or if not exploitable, PR when the fix is trivial). Impact
is evidenced by download counts and by who maintains the crate, not by adjectives.

**Baselines and controls.** Real and current, and sometimes used as *negative controls*: in P2 an
*unmodified* Miri is run first specifically to identify the tests that fail only because Miri lacks
FFI support — that is a control, not a baseline. In P3 the control is fast-check, a tool people
actually use, not a strawman. He is also willing to make the *specification* the object of
comparison (Stacked Borrows vs Tree Borrows) rather than assuming one oracle is ground truth, and he
states plainly that "neither of the two models is canonical."

**Negative results and self-incrimination.** This is his most distinctive habit. P3 puts the
un-apportionable confound in the **introduction**. P3 reports that the 15-minute task cap bit the
control (11 tasks) more than the treatment (6 tasks) and that this could have inflated the effect;
it reports the four participants who *preferred the control tool*; it reports the non-significant
confidence result at full volume. P2's construct-validity paragraph concedes the gap that matters
most to his own claim: "we did not evaluate whether optimization would have had any effect on our
results, and we did not determine whether any of the bugs that we found had an effect on native
execution" — i.e. he volunteers that "violates the model" is not yet "misbehaves". Deduplication is
labelled "conservative" and he lists the reasons the 47 is an undercount.

**Threats to validity.** Always present, always split into Construct / Internal / External, always
specific rather than boilerplate, and typically ending with a scoped defence rather than a shrug
("these limitations did not prevent us from answering our research questions"). P1 adds a separate
**Ethics** subsection (IRB, consent script, redaction of direct and indirect identifiers) and
discusses bot/fraud screening thresholds (reCaptcha <0.5, RelevantID >0.3) and the geographic bias
introduced by North-America-redeemable gift cards.

**Artifacts.** Every paper has one and says where: GitHub for MiriLLI (plus an Appendix with
per-library metadata and links to every upstream contribution), Zenodo for P1 (codebook, all coding
decisions, full survey with branching logic), ACM supplementary material for P3 — and P3 states
*what is withheld and why* (audio/video, IRB).

**Structure and tone.** Short intro: gap in 2–4 paragraphs, then the RQs, then a "Contribution" and
an "Overview" paragraph that maps every section. A real **Background** section that teaches the
semantics (ownership, retags, provenance, Tree Borrows trees) before any method. Discussion split by
*audience* — "For Rust Developers" / "For The Rust Project", or §7.1 Dynamic Tooling / §7.2 Static
Tooling / §7.3 Unsafe Code Guidelines. Related Work organized by RQ or by category, and used to say
what prior work *excluded* ("most considered foreign function calls to be out of scope"). Tone is
hedged in the claims ("may", "we expect", "our results indicate") and bold only in the call to
action ("The Rust community is in dire need of...", "must invest"). He does not use the word "novel"
as a load-bearing claim; when he claims a "first" he scopes it narrowly ("the first human-subject
study of visual consistency in formal modeling").

**Figures and tables.** No single running example — instead **one minimal reproducer per defect
class**, as a short code listing with the UB-triggering line highlighted, plus a small state table
showing the borrow tree before/after. An architecture data-flow figure for the tool. For qualitative
work, codebook tables of Code / Definition / Participant / Verbatim quote, paired with the exact
survey question derived from those codes. For the RCT, a per-measure results table with means and
p-values, and a numbered UI screenshot. Percentages in prose are followed by counts; distributions
get means and standard deviations.

---

## (d) Likely reviewing standards — checklist for a paper in this lens

Items 1–11 are what I would expect him to demand of a C-to-Rust differential-testing paper. These
are **inferred from his own papers' standards**, not from any published review (see §(g)).

1. **Show the funnel, not just the numerator.** Universe of candidate functions/libraries →
   compilable → harnessable → actually executed → divergences → adjudicated defects, with the
   attrition cause at every stage. He reports his own 61%-unsupported and 10%-timeout rates; he will
   notice their absence in yours.
2. **Every percentage carries its denominator and raw count**, in prose and in tables.
3. **Name the oracle, and name its specification and version.** Which sanitizer, which UBSan checks,
   Stacked or Tree Borrows, which toolchain/nightly, which optimization level — and *why* those were
   pinned. He pins `nightly-2023-09-25` and explains that mixing Miri versions has no defined
   behaviour; he flags that Tree Borrows was still evolving during the study.
4. **Separate "difference" from "defect" with a stated adjudication procedure.** He distinguishes
   "undefined *or undesired*" behaviour in his own abstract and counts memory leaks separately from
   UB. A Rust panic sitting on top of C that was already out of bounds is not a translation defect,
   and he will expect the paper to say how it ruled that out — per input, not per cluster.
5. **Deduplicate explicitly, and say which direction the error goes.** State the dedup key (exit
   code / stack trace / message, with addresses stripped), call it conservative or not, and say
   whether the headline count is an over- or under-count and why.
6. **Externally validate at least some findings.** Report upstream: bug reports filed, the disclosure
   protocol, maintainer responses, fixes merged, and the deployment weight of the affected code
   (downloads, dependents). "We found N divergences" with no one outside the author list agreeing
   will read as unvalidated.
7. **Quantify false positives and describe the mitigation.** He names his own (alignment,
   uninitialized-byte reads from LLVM's bit-twiddling) and adds *two execution modes* so true
   positives can be separated from artefacts. Expect him to ask what your harness's false-positive
   sources are and how you measured them, not whether you believe there are none.
8. **Close, or explicitly concede, the gap between the checker's verdict and observable
   misbehaviour.** He volunteers that he did not check whether optimization or native execution
   actually manifests the UB he found. A paper that silently equates "our oracle flagged it" with
   "the translation is wrong in practice" will be pushed on this; a paper that concedes it in the
   threats section, as he does, will be forgiven.
9. **Use a real baseline, plus a negative control.** Prior tools at their current versions, run by
   you, with their failures reported honestly; *and* a control condition that establishes the
   instrument is not manufacturing the signal (his unmodified-Miri control is exactly this).
10. **Report performance honestly and say whether the method can actually be used.** BorrowSanitizer
    exists because Miri's 1000x overhead makes it useless for fuzzing. He will ask about throughput,
    executions, wall-clock budget, and whether budget was equal across arms.
11. **Artifact, with the pipeline and not only the headline.** Replication package, per-subject
    metadata, links to upstream contributions, and an explicit statement of anything withheld and
    why.
12. **Threats to validity split Construct / Internal / External, specific and non-generic**, plus an
    ethics/IRB statement for anything involving humans, and disclosure ethics for anything involving
    other people's software.

---

## (e) Red flags that would make this reviewer reject

- **Percentages with no denominators**, or a headline bug count with no funnel behind it.
- **An unexamined oracle.** No statement of which UB checkers ran, which aliasing model, which
  toolchain, which optimization level — or treating one sanitizer's silence as proof of definedness.
  He explicitly writes that neither aliasing model is canonical; asserting a single ground truth
  without argument is the fastest way to lose him.
- **Conflating crashes/panics with translation defects.** Counting a Rust panic on top of already-UB
  C as a bug found. Equally: counting N divergence clusters as N distinct defects with no root-cause
  analysis.
- **Hidden failure modes.** No timeouts, no unsupported constructs, no excluded subjects, no
  harness-construction failures reported. His own papers publish theirs; a suspiciously clean
  pipeline reads as under-reported, not as strong engineering.
- **Unapportioned confounds sold as a causal claim.** If the technique is a bundle (matcher +
  harness generator + UB gate), claiming the whole effect for one component without an ablation, or
  without conceding the confound the way he does in P3's introduction.
- **Overclaiming.** "First", "novel", "fully automatic", "complete" without a scope clause; a tool
  paper whose abstract promises production readiness that the evaluation does not support.
- **Bug counts with no external validation** — nothing reported upstream, no maintainer response, no
  indication anyone but the authors thinks these are bugs.
- **A method that cannot run at realistic speed** on realistic subjects, presented as if it could.
- **Human data without ethics.** No IRB, no consent, unstated compensation, un-redacted identifiers
  in the artifact.
- **Boilerplate threats to validity** ("we used only open-source projects, results may not
  generalize") with none of the study-specific admissions that his own papers volunteer.
- **Cherry-picked reporting:** non-significant results omitted, arms given unequal budgets,
  participants/subjects who favoured the baseline quietly dropped.

---

## (f) What would delight him

- **A funnel table** from universe → analyzed → divergences → confirmed defects, with the cause of
  every drop, published in the artifact as well as the paper.
- **A negative control that comes back clean**, and is reported as such. This is his own instinct
  (unmodified Miri as control) and it is rare enough in this area that he would notice it.
- **A defect taxonomy where every class has a minimal reproducer** — the C snippet, the Rust snippet,
  the one line that diverges — in the style of his Figure 1. He reasons in minimal examples.
- **Treating the oracle as an object of study rather than a given**: comparing two definedness
  criteria (e.g. ASan-only vs ASan+full UBSan) and showing quantitatively that one misadjudicates a
  named class of inputs. His Stacked-vs-Tree-Borrows RQ is exactly this move, and a paper that shows
  "sanitizer A alone would have called this clean, and been wrong" speaks his language directly.
- **An explicit, defensible non-defect category.** A rule like "C-alone-under-ASan+UBSan decides"
  applied consistently, with the rejected candidates counted and characterized, not hidden.
- **Honest blindness reporting**: which boundaries the method could not harness at all, and why —
  framed as a limitation of the analysis rather than as an unsupported schema.
- **Upstream fixes.** Bug reports filed with a disclosure protocol, maintainer acknowledgements,
  merged PRs, and the download/dependent weight of affected code.
- **Throughput that makes the technique usable** — a UB gate cheap enough to keep in the fuzzing
  loop, with measured overhead. This is literally the premise of BorrowSanitizer.
- **A statement of what the tool is for.** He respects "our goal was not a production tool, but to
  understand X" and he respects "this is production-ready, here is the overhead" — what he does not
  respect is a paper that is vague about which one it is.
- **A named audience for the recommendations** — a "For translator authors" / "For the Rust Project"
  discussion section rather than generic future work.

---

## (g) Evidence vs inference disclosure

**Observed (I read these directly, in full or near-full):**
- The FSE 2027 Research Papers PC page, fetched and parsed in full; Sunshine's listing confirmed
  verbatim. Chairs, area chairs, and roster all read from that page.
- P2 (ICSE 2025, arXiv 2404.11671v5): complete text.
- P1 (TOSEM 2026, arXiv 2404.02230v4): complete text of abstract, intro, background, methodology,
  ethics + threats, discussion, conclusion; the Results section (§5.1–5.5) was skimmed rather than
  read line by line, so specific per-question percentages beyond those quoted here are not verified.
- P3 (FSE 2025, author PDF): complete text.
- BorrowSanitizer RW2025 short paper: complete text (~1.8k words).
- His CMU homepage and publications page, including the 2026 listings.

**Read only as abstract:** "Visual Consistency in Formal Modeling: An Empirical Evaluation" (Liang,
Palliyil, Kang, Sunshine; TOSEM 2026). It is one of his two most recent papers by date but is
off-lens, which is why P3 (FSE 2025) was used as the third paper instead. Its abstract is consistent
with the taste described above (controlled human-subject experiment, "first ... study" scoped
narrowly, partial-consistency result reported as a practical alternative).

**Inference, not observation — treat §(d), §(e), §(f) as predictions:**
- **I found no public reviews, review guidelines, "how I review" posts, PC-chair notes, or talks on
  reviewing by Sunshine.** An OpenReview profile exists but I did not retrieve any public reviews
  from it, and SE venues do not publish reviews. Nothing in this profile is an observed review.
- The entire reviewing-standards checklist is reverse-engineered from the standards his own papers
  hold themselves to — specifically from what he volunteers against his own interest (the
  optimization/native-execution gap in P2; the oracle-vs-UI confound and the asymmetric task cap in
  P3; the snowball-sampling and self-report biases in P1). The premise is that a reviewer asks of
  others what he visibly asks of himself; that premise is reasonable but unverified.
- His **service record** (which PCs, how many reviews, any chairing) was not established beyond the
  FSE 2027 PC listing itself and a PLATEAU committee mention in search results; I did not confirm the
  latter on a primary source.
- Some items in §(d)/§(f) are extrapolations *into* our lens from adjacent work — e.g. the "separate
  difference from defect" and "ASan-alone misadjudicates" items are my mapping of his
  Stacked-vs-Tree-Borrows and UB-vs-undesired-behavior distinctions onto C-to-Rust differential
  testing. He has not written about C-to-Rust translation validation directly; P1's introduction
  mentions automated translation tools only in passing, citing Larsen, and immediately says
  interoperation is "the most practical method" today. **So: he is an expert on the oracle side and
  on unsafe Rust, but a C-to-Rust-translation paper would be adjacent to, not inside, his own
  publication record.** Expect him to be authoritative about UB adjudication and unconvinced by
  transpiler-specific folklore he has not seen evidenced.
- The HCI-flavoured predictions ("would a developer act on this report?") follow from his joint
  PL/HCI appointment and from P3/NaNofuzz being usability RCTs. Reasonable, but he has never applied
  that lens to a translation-validation paper in print.
