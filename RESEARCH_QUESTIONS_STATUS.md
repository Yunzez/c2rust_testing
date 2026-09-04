# Research Questions, Experimental Plan, and Current Status

**Internal discussion draft — 2026-09-03**

这份文档用于 research meeting，记录当前四个 research questions、各自的实验单位、实验方法、已有结果和仍需决定的问题。这里以 `results/` 中的最新证据为准；论文中仍有部分旧数字，不能直接把两者混用。

## Current status at a glance

| RQ | Question | Current status | Main remaining work |
|---|---|---|---|
| RQ1 | Function-matching accuracy | **基本完成** | 独立复核人工标注；统一论文措辞 |
| RQ2 | End-to-end defect detection | **21 confirmed + 4 candidates；尚未最终冻结** | 完成新候选的确认，随后冻结 defect set |
| RQ3 | Defect taxonomy | **7 个暂定 mechanism families** | 等 RQ2 冻结后更新 taxonomy 和计数 |
| RQ4 | Coverage beyond shipped tests | **bzip2 正在进行；只有 c2rust coverage 已完整汇总** | 完成 bzip2 其余可运行工具，再决定是否扩到其余库 |
| Component analysis | Why individual validation components matter | **已有 observation pilot；其余证据较窄** | 明确其为解释性实验，不与四个主 RQ 的分母混合 |

## Study scope and counting rules

- Subjects: **10 C libraries × 6 C-to-Rust translation systems**. Matrix cells may be complete, partial, non-building, invalid-reference, or unavailable; unavailable cells are reported as `N/A`, never as zero.
- The matcher is an **enabling component**, not the main novelty. Our claim is that it is sufficiently accurate for constructing differential-validation boundaries in this corpus, not that it is a general state-of-the-art code matcher.
- The main scientific contribution is the differential-validation workflow: constructing comparable C/Rust executions, checking whether C is a valid reference, observing relevant program state, and confirming translation defects.
- A **defect** is one confirmed root cause. Thousands of inputs exposing the same faulty rewrite still count as one defect.
- A **candidate** has an observed symptom but has not completed the confirmation pipeline. Candidates are never included in the confirmed-defect total.
- Coverage is measured over the **translated Rust artifact**. C executes as the reference and UB-checking side, but C coverage is not included in the RQ4 numerator or denominator.
- Record counts, function pairs, defects, and coverage regions are different units and must not be pooled into one rate.

---

## RQ1 — Matching Accuracy

### Research question

> **How accurately does our matcher identify corresponding functions between the original C program and its Rust translation?**

中文解释：当 C 和 Rust 的函数名、签名或程序结构发生变化时，我们的 matcher 能否找到足够准确的对应函数，使后续 differential validation 可以建立正确的比较边界？

### Experimental unit

- Primary unit: **library**. Each library receives equal weight in the reported macro average.
- Supporting evidence unit: a manually or mechanically established **C–Rust function correspondence**.
- Pair counts show evidence volume, but a large library does not receive more weight merely because it has more functions.

### Experimental method

1. For name-preserving translations, hide identifiers from the matcher and use equal names only as ground truth.
2. For PtrTrans and SACTOR outputs that may rename functions, establish ground truth from the C source, Rust output, tool map, and translation provenance.
3. Run the matcher in forced mode so that abstention is not counted as an incorrect match. Report precision and recall separately.
4. Compare against name equality, especially on the naturally renamed correspondences.
5. Trace at least one match into the downstream validator so the experiment distinguishes a missed **contract boundary** from a missed **unique defect**.

Partial outputs are excluded from the primary aggregate and reported separately. Static matching does not require a building crate, but the artifact must contain a genuine translated implementation rather than only a stub.

### Current results

- **40 non-partial outputs**, covering all 10 libraries.
- **4,202 ground-truth correspondences**:
  - 4,041 name-hidden correspondences from name-preserving outputs;
  - 161 manually established correspondences from outputs that may rename functions.
- Library-macro average: **precision 0.829, recall 0.874**.
- The corpus contains only **9 naturally renamed, implemented pairs**. Name equality recovers **0/9**; our matcher recovers **7/9**.
- In PtrTrans qsort, name equality misses the renamed public boundary `quickSort -> quick_sort`. It still finds the same underlying defect through the name-preserved `partition` function. Therefore, this is evidence of a **lost validation boundary**, not a lost unique defect.

### Interpretation

The current evidence supports a limited claim: the matcher is adequate to support our validator on the studied corpus and handles most naturally occurring renames. It does **not** support a broad claim that we solve general cross-language matching. The natural-rename denominator is small because five of the six studied translation systems normally preserve function names.

### Status and concerns

- [ ] Independently review the manual labels, especially the 9 renamed pairs.
- [ ] Decide whether non-building but genuine implementations remain in the main RQ1 table; keep the rule consistent.
- [ ] Keep partial outputs and stubs outside the primary aggregate.
- [ ] Do not advertise `7/9` as a population-level accuracy estimate.
- [ ] Verify that every number in the paper table is regenerated from the archived per-output files.
- **My concern:**

---

## RQ2 — End-to-End Effectiveness

### Research question

> **How effectively does our validator detect real defects in C-to-Rust translations across libraries and translation strategies?**

中文解释：把完整 workflow 用在整个 translation matrix 上时，它究竟能发现多少真实的翻译错误？这些错误是否跨越不同库和翻译策略，而不仅仅是某一个工具或某一种 crash？

### Experimental unit

The primary unit is a **confirmed root cause**. Repeated divergences, fuzzing crashes, or multiple triggering inputs do not increase the defect count unless root-cause analysis shows independent faulty transformations.

### Experimental method

For every available library–translator artifact:

1. Match candidate C/Rust function boundaries.
2. Generate and run differential harnesses where the input contract can be represented.
3. Execute C first and compare with Rust only when the C reference passes the applicable UB check.
4. Observe return values, designated output memory, relevant global state, process output, and failures as appropriate for the boundary.
5. Minimize and deterministically replay each candidate divergence.
6. Apply isolated ASan+UBSan checking and source-version/provenance checking.
7. Inspect the C, base translation, and rewritten Rust to identify the root cause.
8. Classify the outcome as a confirmed defect, candidate, bounded no-difference result, invalid reference, partial output, or process failure.

### Current results

The current canonical manifest contains:

- **21 confirmed defects**: 7 crash/panic defects and 14 semantic defects;
- defects in **8 of 10 libraries** and **5 rewriting systems**;
- **4 unresolved candidates**.

The latest confirmed defect is bzip2 × C2SaferRust `mmed3`: the rewrite changes a median-of-three computation into a minimum computation. Exhaustive enumeration of all 16,777,216 `u8` triples finds 16,679,040 differing results (**99.41%**). Its reference is the faithful base-c2rust input to C2SaferRust, which pins the error to the rewriting stage. The demonstrated consequence is an incorrect pivot choice and potential performance degradation; we do not currently claim incorrect compressed output from this defect.

Current candidates:

| Candidate | Observed symptom | What is still missing |
|---|---|---|
| cJSON × PtrTrans | Failure-path offset-only divergence | Establish whether this is a distinct translation defect |
| tulip × C2SaferRust | stdout-only display differences | Root-cause and semantic-significance analysis |
| bzip2 × C2SaferRust `sendMTFValues` | Deterministic Rust out-of-bounds failure in shipped test | Complete provenance and differential confirmation |
| bzip2 × CROWN `fallbackSort` | `bhtab` output divergence | Exclude the possibility that the hand-written input schema caused it |

The corrected automatic bzip2 harnesses also rediscovered two previously known defects: Laertes's zeroed checksum table and CROWN's corrupted written length. These strengthen the evidence for automation but do not add new defect rows.

### Interpretation

The current result supports the paper's core effectiveness claim: accepted C-to-Rust translations can contain silent semantic defects, and differential validation finds defects across several translation strategies. However, **21 is not yet a frozen paper number** because the current coverage campaigns are still producing candidates and confirmation work remains.

### Status and concerns

- [ ] Triage the two new bzip2 candidates before freezing the corpus.
- [ ] Decide whether Laertes's newly observed zero-initialized `incs` table is a new root cause or another witness of the existing initialization-loss root cause.
- [ ] Complete the same confirmation checklist for every confirmed row.
- [ ] Freeze the final defect set before updating all paper counts.
- [ ] Preserve failures and partial outputs in the matrix rather than removing difficult cells.
- **My concern:**

---

## RQ3 — Defect Taxonomy

### Research question

> **What recurring mechanisms cause defects in C-to-Rust translations, and how do they manifest across translation strategies?**

中文解释：RQ2 说明“发现了哪些 defect”，RQ3 则问这些 defect 为什么发生。它寻找跨工具重复出现的 root-cause mechanisms，而不是简单按 crash、panic 或 wrong output 分类。

### Experimental unit

The unit is the same confirmed defect used in RQ2. Each defect receives exactly one primary mechanism family for counting, even if it has several symptoms.

### Experimental method

1. Inspect each confirmed faulty transformation and its faithful reference.
2. Assign one primary family based on root cause rather than surface symptom.
3. Allow secondary descriptive tags, but do not count one defect in multiple primary families.
4. Compare how the same mechanism appears in different translation strategies.
5. Treat the family counts as a description of this corpus, not an estimate of real-world prevalence.

### Current provisional taxonomy

| Mechanism family | Defects | Meaning |
|---|---:|---|
| Control-flow preservation failure | 3 | A rewrite changes guards, recursion, or execution order |
| Byte-string domain narrowing | 5 | Rust accepts a narrower byte/string domain than C |
| Ownership-state corruption | 3 | Ownership or slice rewriting corrupts state or memory behavior |
| Null/empty conflation | 3 | A null pointer and a valid empty object are treated as equivalent |
| Initialization loss or corruption | 3 | Required tables or global state are absent or initialized incorrectly |
| Interface-contract loss | 3 | Reshaping an interface loses offsets, lengths, or caller obligations |
| Semantic computation substitution | 1 | A computation is replaced by a different computation, as in median-to-minimum |
| **Total** | **21** | One primary family per confirmed defect |

### Interpretation

The taxonomy currently shows that defects are not explained by a single unsafe-code issue or by one translation strategy. Several mechanisms recur across systems, while individual systems exhibit multiple mechanisms. The seventh family is new and currently has one member, so it should be presented as an observed mechanism rather than a recurring trend.

### Status and concerns

- [ ] Re-run the taxonomy table after the RQ2 defect set is frozen.
- [ ] Confirm that the seven families are mutually exclusive under a written coding rule.
- [ ] Avoid prevalence language such as “the most common C-to-Rust defect” unless the selection procedure supports it.
- [ ] Decide whether one-member families belong in the main table or should be described as an additional mechanism.
- **My concern:**

---

## RQ4 — Coverage Beyond Shipped Tests

### Research question

> **How much translated Rust code does our differential validator exercise compared with the shipped acceptance tests?**

This wording may be more accurate than assuming in advance that our coverage is always higher. The important measurements are total coverage and the complementary regions reached only by each workload.

中文解释：在同一个 translated Rust artifact 上，原有测试和我们的 differential fuzzing 分别走到哪些代码？我们的目标不是保证每个库的总覆盖率都更高，而是测量 differential validation 是否执行了原测试没有执行的 translated behavior。

### Experimental unit

- Primary unit: one **library–translator artifact** for which both workloads can be run.
- Library-level result: equal-weight mean over its paired, runnable artifacts.
- Coverage is unioned across all successful function harnesses for an artifact; it is never summed across harness binaries.

### Experimental method

For each runnable translated artifact:

1. Instrument the translated Rust artifact once with a common coverage mechanism.
2. Run the shipped acceptance tests against that translated artifact.
3. From the frozen RQ1 matches, determine which function boundaries are eligible for automatic differential harnessing.
4. Build and run every eligible harness under a declared campaign budget.
5. Union function and region identities across the artifact's harnesses.
6. Report the complete pipeline: `matched -> eligible -> built -> executed`.
7. Report test coverage, validator coverage, intersection, test-only coverage, and validator-only coverage.
8. Mark partial/non-building artifacts as `N/A`, not 0%.

The current experiment uses schema-based automatic function harnesses. Stateful interfaces requiring `FILE*`, opaque handles, callbacks, or process-owned state are presently unsupported. Reusing test setup to create environment-aware fuzz drivers is a possible future extension, but it is **not part of the current experiment**.

### Completed pilot: bzip2 × c2rust

| Metric | Shipped tests | Validator | Both | Test only | Validator only |
|---|---:|---:|---:|---:|---:|
| Functions (denominator 66) | 51 (0.773) | 45 (0.682) | 43 | 8 | 2 |
| Regions (denominator 8,789) | 7,007 (0.797) | 7,018 (0.798) | 6,609 | 398 | 409 |

Pipeline: **64 matched -> 14 eligible -> 11 built -> 10 executed successfully**.

Important observations:

- Function coverage is lower than the shipped tests, but region coverage is approximately equal.
- The two workloads are complementary: 409 regions are validator-only and 398 are test-only.
- Validator function coverage saturates within the first minute: it reaches 45 functions and 6,970 regions at one minute; the next 59 minutes add no functions and only 48 regions.
- The primary limitation is boundary eligibility, not campaign duration. Of the 21 uncovered functions, 19 are `FILE*`/`BZFILE*` APIs or functions reachable only through them; the remaining two are degenerate boundaries.
- On a fixed-corpus replay, gated, no-gate, and Rust-only modes all cover exactly 45 functions and 7,018 regions. Thus the online UB gate causes no coverage loss on this corpus. This does not measure the runtime cost of executing C during campaign generation.
- The corrected input schemas substantially changed the pilot result. The initial automatic schemas were wrong for most boundaries, so schema creation and generator fixes must be treated as part of the experimental method rather than hidden setup work.

### Work in progress on bzip2

- Laertes, CROWN, and C2SaferRust campaigns have run, but their final paired coverage rows are not yet frozen here.
- The campaigns rediscovered known defects and produced new candidates. Early divergences and panics can reduce corpus growth, so each coverage row must report whether defect-triggered restarts suppressed exploration.
- PtrTrans and SACTOR do not currently provide runnable bzip2 artifacts for this paired comparison; their result is `N/A`, not zero coverage.

### Status and concerns

- [ ] Finish the complete bzip2 table for every runnable translation artifact.
- [ ] Merge and freeze the generator fixes before starting another library.
- [ ] Decide whether bzip2 provides enough evidence to scale the protocol to all ten libraries.
- [ ] Inventory which shipped test suites actually run on each translated artifact.
- [ ] Report saturation curves and early-divergence effects, not only a one-hour headline.
- [ ] Reconsider the title “Coverage Beyond Shipped Tests” if the final result is primarily complementary coverage rather than uniformly higher coverage.
- [ ] Do not silently add test-derived/environment-aware harnesses midway through the experiment; treat that as a separately declared extension if pursued.
- **My concern:**
我们现在的coverage 太低了，不过没有跑完 如果最后coverage平均下来只有60%这样 会不会不太好

---

## Component Analysis — Why the workflow needs each component

These are supporting experiments, not additional main RQs. Their units and evidence sets differ, so they must not be combined into a single aggregate accuracy number.

### Observation channels

On a fixed replay set containing 9 of the 21 confirmed defects:

| Observation configuration | Defects recovered |
|---|---:|
| Return only | 6/9 |
| Process output, silent driver | 3/9 |
| Process output, printing driver | 9/9 |
| Normalized function state | 9/9 |
| Full observable state | 9/9 |

This shows that the relevant contract may be expressed in an in-place array, output buffer, structure field, global table, or process output rather than in the return value. It is a fixed-input, single-seed component study, not an estimate of fresh-campaign discovery probability.

### C-reference attribution

- The urlparser witness shows that an in-process UBSan screen can miss a C heap overflow that isolated ASan+UBSan detects.
- The lil revision mismatch shows that sanitizer-clean executions can still disagree when the C reference and translation come from different source revisions.
- These experiments justify the confirmation policy. They do not add confirmed translation defects and currently cover no confirmed-defect row as an attribution ablation.

### Alignment source

- The PtrTrans qsort study shows that name equality can omit a renamed public validation boundary.
- In that example, a name-preserved internal boundary still exposes the same unique defect.
- The downstream effect of an incorrect translator-provided map remains unmeasured.

### Component-analysis concerns

- [ ] Keep the denominators explicit and separate.
- [ ] Do not describe attribution pilots as evidence that all 21 defects passed an identical ablation.
- [ ] Clearly distinguish defect recovery, false-candidate suppression, and boundary recovery.
- **My concern:**

---

## Decisions recommended for the next research meeting

1. **Defect freeze:** Should we freeze at 21 now, or wait until the two new bzip2 candidates complete triage?
2. **RQ1 review:** Who will independently review the manual truth labels, especially the nine renamed pairs?
3. **RQ4 stopping rule:** Finish all runnable bzip2 artifacts first; then decide whether the evidence justifies scaling to the remaining nine libraries.
4. **Harness scope:** Keep RQ4 limited to automatic schema-based harnesses, or define environment-aware/test-derived harnesses as a separate extension?
5. **Coverage framing:** Retain “beyond shipped tests,” or frame the result as complementary exploration if that better matches the full data?
6. **Generator freeze:** Which scratchpad fixes must be merged and regression-tested before any additional coverage campaigns?
7. **Taxonomy freeze:** Is `semantic computation substitution` sufficiently distinct to remain a seventh primary family?

## Paper synchronization still pending

The latest internal result is **21 defects in 7 families**, but the paper still contains the older **20 defects in 6 families** result. After the defect set is frozen, synchronize at least:

- `c2rust_paper/main.tex`
- `c2rust_paper/introduction.tex`
- `c2rust_paper/evaluation.tex`
- `c2rust_paper/figures_tables.tex`
- `c2rust_paper/discussion.tex`
- `results/EVALUATION_PLAN.md`
- `results/rq4_effectiveness/translation_matrix.md`

Do not update only the headline total: bzip2 × C2SaferRust now contains an additional semantic defect, and the taxonomy table needs a seventh family.

## Evidence pointers

- RQ1 assembled evidence: `results/rq1_matching/`
- Canonical defect manifest: `results/rq4_effectiveness/defect_manifest.md`
- Detailed defect ledger: `results/rq4_effectiveness/bugs_detailed.md`
- New `mmed3` evidence: `results/rq4_effectiveness/bugs/bzip2_c2saferrust_mmed3/`
- bzip2 coverage protocol and result: `results/rq3_coverage/bzip2/`
- Component studies: `results/ablations/`

