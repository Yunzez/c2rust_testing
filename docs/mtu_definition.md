# MTU Definition and Checkpoint Strategy

## What is an MTU?
An **MTU (Minimum Testing Unit)** is the smallest *semantically meaningful* unit we can test across C and Rust that:
- Has a clear input domain.
- Produces observable outputs or state changes.
- Can be executed deterministically.

In practice, an MTU is usually **one function**, but it can be a small, well‑bounded call chain if the logic is split across helpers by the transpiler.

### MTU Contract
Each MTU should specify:
- **Inputs**: types and constraints (e.g., ASCII strings, bounded lengths).
- **Outputs**: return value + selected output fields / buffers.
- **Side effects**: memory regions or global state that must be compared.
- **Oracle rule**: C is the reference **only for defined inputs**; C crashes are classified as invalid/UB.

## Why We Don’t Use Lower‑Level IR (e.g., LLVM) for MTU Definition
LLVM IR is excellent for **coverage** and **low‑level equivalence**, but it is **not ideal for MTU definition**. Our goal is **semantic alignment**, not instruction‑level matching.

### Key Reasons to Stay at the C Source Level
- **Semantic meaning**: C source preserves intent (parsing complete, output produced), while IR does not.
- **Cross‑tool robustness**: different transpilers and compiler versions produce different IR shapes, even when semantics match.
- **Checkpoint stability**: semantic checkpoints can be consistently placed at source‑level boundaries (entry/exit, API calls), but are unstable in IR.
- **Human interpretability**: source‑level MTUs and checkpoints are explainable in a paper; IR checkpoints are not.

### What We Still Use IR For
LLVM IR instrumentation remains useful as a **secondary signal**:
- Edge coverage for fuzzing.
- Localization after a divergence is detected.

But **MTU definition and semantic checkpoints stay in the source**.

## Checkpoint‑Based Alignment
When C and Rust have different control flow, we compare **semantic milestones**, not instructions.

### Minimal Checkpoints
- **Entry**: immediately after function start.
- **Exit**: just before return.

### Extended Checkpoints (optional)
Add checkpoints after:
- Parsing or validation.
- Allocation / initialization.
- Major API calls.
- Output materialization.

### What to Compare at Checkpoints
Use small, deterministic state:
- Return value.
- Selected output fields (string hashes, buffer hashes).
- Error category (OK / NULL / error).

## Practical Guideline
Start with **entry/exit checkpoints** for each MTU. Add internal checkpoints only when:
- Divergence is detected, and
- You need localization or finer attribution.
