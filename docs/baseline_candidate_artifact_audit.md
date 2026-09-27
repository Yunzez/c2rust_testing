# Audit of adjacent validation and translator-testing artifacts

Audit date: 2026-09-14.

This note records the systems considered when selecting released-artifact
baselines for the paper. It distinguishes artifact availability from artifact
applicability. A public repository is not automatically a comparable baseline:
our numerical comparison requires a documented interface that accepts an
independently produced, frozen C/Rust pair and supplies the analysis or search
procedure being evaluated. Supplying our own generated harnesses, seeds, or
comparators to another framework would reuse the central component under test.

Status statements below mean only what was verifiable from the cited paper,
project page, and repository on the audit date. “Not found” is not a claim that
private or subsequently released code does not exist.

## 1. Validators and comparison frameworks for existing programs

| System | Public artifact | What the artifact accepts | Disposition for the 36-defect comparison |
|---|---|---|---|
| FLOURINE | Yes; [official released archive](https://d34gtk3knhjgeg.cloudfront.net/artifact.tar.gz) used in our experiment | C/Rust source pair in its supported representation; supplies differential generation and checking | Evaluated without modifying its analysis, generation, comparison, or search logic |
| RustAssure | Yes; [official repository](https://github.com/davsec-lab/rustassure) used in our experiment | C/Rust function pair; supplies differential symbolic testing | Evaluated without modifying its compiler or symbolic pipeline |
| VERT | Yes; [Zenodo artifact](https://doi.org/10.5281/zenodo.10927704) used in our experiment | Deposited C benchmarks; its documented build regenerates the Rust candidate | Attempted and reported as unsupported for all 36 frozen external candidates, not counted as semantic misses |
| C2Rust cross-checking | Yes: [repository](https://github.com/immunant/c2rust) and [tutorial](https://github.com/immunant/c2rust/blob/master/docs/cross-check-tutorial.md) | Instrumented C and Rust builds plus user-provided executions and cross-check configuration | Discussed, not scored: it does not synthesize the library executions needed by our corpus; feeding it our harnesses would import our construction method |
| cozy | Yes: [repository](https://github.com/draperlaboratory/cozy), PyPI package `cozy-re`, and [C-to-Rust study](https://arxiv.org/abs/2605.12731) | Compiled binaries plus a user-written Python symbolic-execution harness and ABI model | Discussed, not scored: the evaluator must provide the symbolic input/entry model that is central to our comparison |
| DIFFER | Yes: [repository](https://github.com/trailofbits/differ) and [project description](https://blog.trailofbits.com/2024/01/31/introducing-differ-a-new-tool-for-testing-and-validating-transformed-programs/) | Original and transformed programs plus user seeds, mutation templates, and output comparators | Discussed, not scored: it is a general execution framework, not an independent source-pair harness or boundary-construction baseline |
| CINDER | Thesis available from [Vanderbilt](https://ir.vanderbilt.edu/items/e24cdd40-6e82-4048-a426-5b97a8c6edbf); named code repository unavailable | The thesis describes contract-aware differential tests and SMT checks | Not run: the thesis names `github.com/Charbuscus11/Cinder`, which returned HTTP 404 on the audit date |

## 2. Translation workflows that contain validation

These systems validate candidates inside their own generation or repair loop.
Their artifacts are relevant related work, but their documented workflows do
not expose an independent “check this frozen external Rust translation” input.

| System | Public artifact | Validation scope | Disposition |
|---|---|---|---|
| Syzygy | Yes: [project](https://syzygy-project.github.io/), [Zopfli artifact](https://github.com/syzygy-project/Syzygy_Zopfli), and [current repository](https://github.com/adwait/meng-25-crust) | Dynamic specifications and translated tests guide and check candidates produced within Syzygy | Related work; not an external-pair baseline |
| SafeTrans | Yes: [repository](https://github.com/FarrukhCyber/SafeTrans) | Unit-test feedback drives translation and repair | Related work; not an external-pair baseline |
| DTV | Yes: [repository](https://github.com/qsdrqs/dtv-translation) | Compile and program-level differential feedback are integrated into LLM decoding | Related work/audit only; no documented frozen-external-candidate entry point |
| IDEAS | Yes: [repository](https://github.com/IntelLabs/IDEAS) | Translation workspace and experimental equivalence-test generation | Related work/audit only; expects an IDEAS/TRACTOR workspace rather than an arbitrary pair |
| LAC2R | No identifiable public artifact found | Compilation and test feedback inside MCTS-guided multi-trajectory translation | Related work only; the [paper](https://arxiv.org/abs/2505.15858) links no code, and repository search found no identifiable official artifact |

The six translation systems evaluated as subjects in our paper are not repeated
as validator baselines: their own compilation, test, or FFI feedback is part of
the translation process whose frozen outputs we independently validate.

## 3. Translator testing and benchmark infrastructure

These tools ask whether a translator handles newly generated source programs;
they do not validate a pre-existing real-world translated artifact.

| System | Public artifact | Why it is complementary rather than a baseline |
|---|---|---|
| TOUCHSTONE | Yes: [Zenodo artifact](https://doi.org/10.5281/zenodo.7871547) | Uses Csmith/YARPGen programs to measure C2Rust's functional correctness and other translator properties |
| PROGnosticator | Yes: [repository](https://github.com/FuturesLab/PROGnosticator) | Generates construct-oriented source programs to stress source-to-source translators |
| TeTRIS | Yes: [repository](https://github.com/FuturesLab/TeTRIS) | Fuzzes transpilers by generating and mutating valid source programs across languages |
| Csmith / YARPGen | Yes | Defined-program generators used to test compilers/transpilers, not frozen artifact validators |
| CRUST-Bench | Yes: [repository](https://github.com/anirudhkhatry/CRUST-bench) | Benchmark and test corpus for translation systems, not a standalone relational validator |

## 4. Selection language used in the paper

The defensible selection rule is:

> The numerical released-artifact comparison includes systems whose documented
> interface can accept an independently produced C/Rust pair and supplies its
> own analysis or search. Frameworks requiring user-authored executions,
> symbolic harnesses, seeds, or comparators are discussed but not assigned
> detection counts, because using our generated harnesses would reuse the
> component being evaluated. Integrated translation-and-validation workflows
> without an external-candidate interface are recorded as inapplicable, and
> unavailable artifacts are identified explicitly.

VERT remains in the experimental funnel because we attempted its official
artifact and report the exact gate at which the fixed-pair input is unsupported.
This is evidence about interface applicability, not a claim that VERT fails in
its intended translation workflow.

## 5. Search record

The audit inspected the papers/project pages and exact repository URLs above,
the bibliography and related-work citations of the C-to-Rust validation papers,
and GitHub repository searches for the exact names `LAC2R`, `CINDER`, and
`TOUCHSTONE` together with author/title terms. The TOUCHSTONE paper itself led
to its Zenodo artifact, correcting an initially incomplete repository-only
search. The CINDER repository URL was taken from the thesis rather than inferred
from a name search.
