# Research Proposal

Title: Characterizing Semantic Divergence in C to Rust Translation via MTU-Guided Differential Fuzzing

## 1. Research Questions

RQ1: What classes of semantic divergence occur when real-world C code is translated to Rust through automated transpilers and manual rewrites?

RQ2: How do existing transpiler techniques differ in their failure modes, including crashes, incorrect outputs, undefined behavior exposure, or panic outcomes?

RQ3: What is a defensible definition of a Minimum Testing Unit (MTU) for cross-language differential fuzzing that enables scalable comparison between C implementations and Rust translations?

RQ4: In what cases do manual rewrites exhibit different divergence patterns compared to machine-transpiled code, and can these patterns be predicted based on code features?

**Oracle Principle**: Use the C implementation as the reference behavior for non-Undefined Behavior inputs; classify Rust panics/crashes as divergence outcomes relative to C behavior under defined input domains.

**Note**: Oracle definition requires explicit exclusion or classification of undefined behavior, as C may exhibit unspecified outcomes but is treated as the canonical reference for behavior preservation under valid inputs.

## 2. Related Work

### 2.1 C→Rust Transpilation and Human Translation

- *Translating C to Rust: Lessons from a User Study* — Reports that automatic tools struggle compared to humans in translating C to Rust, with users choosing idiomatic Rust patterns unlike automated systems. (NDSS Symposium)
- *An Empirical Study of C to Rust Transpilers* — Surveys transpiler reliability, limitations, quality of generated code, and functional correctness concerns, indicating a gap in empirical understanding. (CSS Lab)

### 2.2 Test Generation and Differential Testing in Translation Context

- *RustAssure: Differential Symbolic Testing for LLM-Transpiled C-to-Rust Code* — Uses symbolic testing to compare semantics, demonstrating feasibility of differential equivalence evaluation. (arXiv)
- *SafeTrans: LLM-assisted Transpilation from C to Rust* — Combines LLM translation with iterative guided repair and asserts correctness via comprehensive tests. (arXiv)

(There are more; these serve as context for semantic divergence and testing approaches but do not define MTUs or systematically fuzz cross-language pairs.)

## 3. Benchmark Availability

Benchmarks relevant to C→Rust translation fall into two broad categories:

### 3.1 Repository-Level Transpilation Benchmark

**CRUST-Bench**

- Comprehensive C-to-safe-Rust benchmark with real repositories and paired Rust interfaces/tests.
- Provides cross-file and realistic translation challenges.
- Includes Rust test suites specifically designed to validate functional equivalence.
- Link: https://github.com/anirudhkhatry/CRUST-bench

This dataset is suitable for macro-level evaluation of complete translation outcomes.

### 3.2 Function-Level Transpilation Benchmark

**C2Rust-Bench**

- Minimized representative dataset of C functions selected for evaluating transpilers.
- Offers ~2,905 functions distilled for coverage and efficiency.
- Focused on common control structures, pointer usage, type casts, and memory operations.
- Link: OpenReview submission (ICLR 2026) under identifier VqI7rDJqiI (OpenReview)

This dataset is suitable for function-level differential fuzzing when mapping MTUs.

### 3.3 Tools and Ecosystem

**C2Rust** (tooling)

- Automatic transpilation of C99 code into Rust.
- Produces near-semantically-equivalent Rust output that can be further tested; does not provide oracle mechanisms itself.
- Link: https://github.com/immunant/c2rust

## 4. Evaluation Plan

### 4.1 MTU Definition and Implementation

- Define MTU as callable, deterministic functions with a bounded input domain and observable outputs.
- Input domains exclude undefined behavior or categorize it separately.
- Outputs normalized across languages (e.g., semantic equivalence classes instead of exact text).
- Include fixtures to capture side effects where relevant.
- Construct harnesses for each MTU in C and its corresponding Rust translation.
- Normalize outputs according to a defined schema (return value, memory state, error category).

### 4.2 Differential Fuzzing Pipeline

- Use existing fuzzers (e.g., AFL, libAFL) or generative test approaches to generate inputs within MTU domains.
- For each input:
  - Run C and Rust versions.
  - Classify outcomes: match, silent divergence, panic/crash, UB classification.
- Aggregate divergence metrics per MTU category (control flow complexity, pointer intensity, etc.).
- Compare across:
  - Machine-transpiled code (e.g., C2Rust outputs)
  - Human-rewritten Rust versions (when available, such as from CRUST-Bench interface layers)

### 4.3 Metrics and Reporting

Measurement dimensions:

- Divergence Rate: percentage of inputs with semantic divergence.
- Divergence Class Distribution: silent differences, crashes, panics, etc.
- Feature Correlations: association between code features and divergence likelihood.
- Transpiler vs Manual Profile Differences: comparative patterns.

Review artifacts:

- Classification schema for divergence.
- MTU definitions per benchmark.
- Detailed empirical tables over benchmark sets.

## 5. Expected Findings

- Identification of predictable divergence patterns for automated transpilers vs manual rewrites.
- Empirically derived taxonomy of failure modes in C→Rust translation under fuzzing.
- Evidence that a structured MTU framework improves cross-language differential testing.
