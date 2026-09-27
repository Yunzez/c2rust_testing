# C-to-Rust Validation Uses Differential Fuzzing, but Not as the Only Equivalence Method

## Bottom line: yes, but use the term narrowly

Yes. Researchers and practitioners do use differential fuzzing for C-to-Rust testing, meaning one generated input is run against both the original implementation and the Rust translation or rewrite, and the harness reports a divergence when outputs, crashes, panics, sanitizer failures, or observable state changes differ. A fuzzing-community definition matches this operational view: run two implementations of the same specification on the same input and crash when outputs disagree [1].

The strongest direct academic example is Fluorine: it uses a cross-language differential fuzzer to obtain evidence of I/O equivalence between a source program and a Rust translation [2]. The strongest practitioner example in the available evidence is Runtime Verification’s `kernel-c-to-rust-spike`, which rewrites a Linux-kernel UVC parser in safe Rust and differentially fuzzes the C and Rust implementations using a LibAFL harness [3].

The qualification matters: much of the C-to-Rust validation literature checks equivalence without doing differential fuzzing. C2Rust used trace cross-checking, RustAssure uses differential symbolic testing, VERT uses property-based testing and bounded model checking, and several systems rely on unit tests, end-to-end tests, or project test suites. The actionable takeaway is: differential fuzzing is a practical validation component when the C/Rust boundary is executable and deterministic, but it should be combined with other checks when state mapping, undefined behavior, pointer values, or layout differences make a concrete oracle fragile.

| Work/tool/project | C-to-Rust context | Testing/validation method | Is it differential fuzzing? | Key limitation |
|---|---|---|---|---|
| Fluorine | LLM translation from C/Go to Rust | Cross-language Rust fuzzing harness; Bolero/libFuzzer; JSON state mapping; I/O equivalence [2] | Yes | Five minutes with no counterexample is evidence, not proof |
| SANER 2026 C-to-Rust system | LLM generate-and-check translator | Differential fuzzing; re-prompts LLM when checks fail [4] | Yes | Reuses a time-limited dynamic oracle; still depends on input reachability |
| NDSS 2025 user study | Human C-to-Rust translation study | AFL++ fuzz-testing to find behavioral differences [5] | Yes | Evaluation setting, not a reusable transpiler validation framework |
| Runtime Verification `kernel-c-to-rust-spike` | Linux-kernel UVC parser rewritten in safe Rust | LibAFL in-process differential harness [3] | Yes | Single parser case study; results are harness- and boundary-specific |
| C2Rust | C-to-Rust transpilation and refactoring | Execution-trace cross-checking on test inputs [6] | Related, not fuzzing by itself | Trace comparison can use fuzz-generated inputs, but fuzzing is not the core method |
| RustAssure | LLM-transpiled C functions to Rust | Differential symbolic testing, not concrete fuzzing [7] | No | Symbolic equivalence has its own false-positive cases when Rust intentionally removes C memory corruption |
| VERT | Verified Rust transpilation using a Wasm oracle | Property-based testing with Bolero and model checking with Kani [8] | No | Oracle is not the original C binary; bounded model checking is coverage-bounded |
| Syzygy | Generated Rust code checked against original C | Observational equivalence on test inputs [9] | Not in the strict sense | Depends on the representativeness of test inputs |
| C2RustTV / RustMap / CROWN / C2SaferRust | Translation or refactoring validation | Test execution, conformance tests, or source-project tests [10][11][12][13] | No, unless paired with an explicit fuzzing driver | Test suites under-sample parser and state-space edge cases |

## What C-to-Rust testing is actually trying to validate

- **Compilation validity**: The first gate is whether generated Rust builds under the intended toolchain. This catches syntax, type, module, FFI, and crate-structure failures, but it does not show that the Rust code preserves C behavior. LLM-based systems often use compilation errors as feedback because a compiler error is cheap to localize and can be fed back into the translator.

- **Behavioral equivalence**: The central question is whether Rust produces the same externally visible result as C for the same input. In a parser this may mean the same parsed fields or the same error code; in a library it may mean matching return values and side effects; in a command-line program it may mean matching stdout, stderr, exit code, and output files. Differential fuzzing is attractive here because it does not require a full formal specification; the old implementation acts as the reference oracle.

- **Memory safety and undefined behavior**: A Rust rewrite may intentionally avoid C memory bugs, so “same behavior” cannot always mean byte-for-byte replication. If the C side reads out of bounds, overflows signed arithmetic, or depends on uninitialized memory, the validator must decide whether the Rust side should match the C result, panic safely, return an error, or reject the input. This is why C-side sanitizers often belong in the oracle: they distinguish meaningful semantic disagreement from C undefined behavior being exposed.

- **API and state equivalence**: C APIs frequently expose raw pointers, structs with padding, global state, allocator-sensitive addresses, and mutation through aliases. Rust APIs may expose owned values, references, enums, `Result`, or safe wrappers. A useful validator therefore needs a mapping from C state to Rust state; comparing raw memory is often wrong because layout and padding can differ even when the logical state is equivalent.

- **Regression testing during migration**: In staged rewrites, the operational goal is not abstract equivalence over all possible executions; it is whether the Rust implementation can replace the C component at a chosen boundary. That boundary should be narrow enough to drive with a test or fuzz harness and stable enough that failures can be triaged as translation bugs, intentional safety changes, or harness mistakes.

C-to-Rust validation is hard because the languages make different commitments about ownership, aliasing, panics, crashes, undefined behavior, data layout, and error handling. The consequence is visible across the literature: systems combine compiler checks, project test suites, fuzzing, symbolic execution, property-based testing, model checking, trace comparison, and manual review instead of relying on a single equivalence test.

## Direct examples where C and Rust are fuzzed against each other

### Fluorine: cross-language differential fuzzing as the validation core

Fluorine is the clearest academic example. It automatically generates a Rust fuzzing harness, uses Bolero/libFuzzer, maps Rust program states to source-language states through JSON serialization/deserialization, and checks I/O equivalence between the source program and the Rust translation [2]. In the C-to-Rust case, the old C implementation supplies the behavioral reference; the fuzz harness searches for inputs where the Rust candidate disagrees.

The state-mapping detail is the important engineering contribution. Fluorine does not simply compile both sides to a common low-level representation and compare raw values; it explicitly rejects an LLVM-IR-only comparison because compilers for different languages can discard type and layout information needed for cross-language equivalence [2]. That design choice matches a common migration problem: if C uses a pointer-rich struct and Rust uses a safer representation, the oracle has to compare logical fields rather than incidental layout.

Fluorine’s stopping rule is also worth reading conservatively. The paper treats a translation as equivalent if five minutes of fuzzing finds no counterexample [2]. That is a practical acceptance rule for an automated translation pipeline, not a semantic proof. For production use, the same design should be treated as a counterexample generator plus confidence signal; higher-risk code should add longer campaigns, seed corpora, sanitizers, symbolic checks, or manual specifications for edge behavior.

### SANER 2026: differential fuzzing inside an LLM feedback loop

The SANER 2026 C-to-Rust translation system uses differential fuzzing as a check in a generate-and-repair loop: when checks return a negative result, the LLM is re-prompted [4]. The validation mechanism tests input/output equivalence between C and Rust over random inputs within a time limit and uses JSON serialization to map values. This shows a common pattern in LLM migration systems: fuzzing is not just a final audit, it can be a feedback source that produces concrete failing inputs for the next translation attempt.

The limitation is that feedback quality depends on the harness’s ability to reach meaningful paths. If generated inputs only exercise shallow parsing or trivial return paths, the LLM may converge on code that satisfies the harness while preserving deeper semantic bugs. The engineering implication is to seed the fuzzer with real corpus inputs and add structure-aware generators when the API expects formats, protocol messages, or stateful sequences.

### NDSS 2025 user study: fuzzing exposes human translation gaps

The NDSS 2025 user study used AFL++ automated fuzz-testing to check behavioral differences between Rust translations and corresponding C sources [5]. It found that none of the 31 Rust translations was fully equivalent to the original C code, and reported discrepancy rates across fuzz tests ranging from 37% to 100%, with a 68% average. This is direct evidence that differential fuzzing is useful beyond LLM-generated code: even human C-to-Rust rewrites can diverge under generated inputs.

This result should not be generalized as “all human translations fail.” It is a study setting with specific tasks and harnesses. Its decision-relevant point is narrower and stronger: if the acceptance criterion is behavioral compatibility with an existing C implementation, compilation plus review is not enough; automated input generation can reveal mismatches that translators miss.

### Runtime Verification `kernel-c-to-rust-spike`: practitioner-grade in-process harnessing

Runtime Verification’s `kernel-c-to-rust-spike` is a practitioner-style case study, not primarily a conventional literature paper. The project extracts a Linux-kernel UVC video descriptor parser from C, rewrites it in safe Rust, and differentially fuzzes the two versions with a LibAFL harness [3]. That makes it a useful worked example for migration teams because the repository exposes the harness, build script, and recorded results.

The harness drives both implementations from one input in one process: it calls the C implementation through the shared C ABI and calls the Rust crate’s native `parse()` inside `catch_unwind` [14]. This design keeps input generation, execution, and comparison in one fuzz target, which reduces drift between test paths. It also forces the harness to define what to do with Rust panics: a panic can be treated as a safe failure mode, an expected rejection, or a divergence depending on the migration policy.

The build script compiles the C code with `clang`, enables SanitizerCoverage, and renames `uvc_parse` to `uvc_parse_c` to avoid symbol collisions [15]. Those details are not incidental. SanitizerCoverage improves fuzz feedback on the C side, and symbol renaming makes it possible to link both old and new implementations into one process without the linker binding calls to the wrong function.

The recorded result is bounded but concrete: after fixing one signed-overflow divergence, the harness recorded 54,019 executions at 93% edge coverage with zero divergences [16]. The same results file includes a negative control: reintroducing CVE-2024-53104 causes the C version to trigger a heap-buffer-overflow under ASan while the Rust version panics safely. That negative control is valuable because it shows the harness can detect a real safety-relevant mismatch rather than merely reporting “no differences” on easy inputs.

### Other practitioner evidence: ports and rewrites use the same pattern

Outside strict “translation tool” papers, several Rust reimplementations and ports use differential fuzzing against existing implementations. The `oferchen/rsync` project is a pure-Rust rsync reimplementation that performs wire-level differential fuzzing against upstream C rsync across versions and protocols [17]. Its fuzz targets include comparisons of filter decisions and local-copy outcomes against upstream rsync [18].

`bitcoinfuzz` is a modular differential fuzzing harness across Bitcoin implementations, including Bitcoin Core in C++, Rust Bitcoin libraries, and Core Lightning in C [19]. As of the cited project update, it reports more than 35 bugs found, including CVE-2024-44073 in `rust-miniscript` [20]. This is not a C-to-Rust transpiler evaluation, but it is strong evidence that Rust implementations are being differentially fuzzed against mature C/C++ reference implementations in security-sensitive ecosystems.

`ethp2p-rs` is a clean-room Rust port of a Go ethp2p stack validated against the Go reference through differential fuzzing using a C-ABI shim [21]. The language pair is Go/Rust rather than C/Rust, but the mechanism transfers directly: put the non-Rust implementation behind a C-compatible boundary, run one generated input through both sides, and compare the observable result.

## Related validation methods that should not be collapsed into differential fuzzing

C2Rust is the canonical C-to-Rust transpilation tool, but the cited validation mechanism is cross-checking: it compares execution traces of two programs on a test input rather than operating as a fuzzing framework [6]. Cross-checking can be paired with fuzz-generated inputs, and that combination would be close to differential fuzzing in practice. But the method itself is trace comparison; calling C2Rust’s cross-checking “differential fuzzing” without an explicit fuzzing driver would overstate the evidence.

C2Rust cross-checks were later dropped in June 2022 because they relied on rustc internals and produced non-reproducible values; a specific release-version attribution is not reliable from the available retrievable source. The operational lesson is that trace-based equivalence can fail for tooling reasons even before it fails for semantic reasons: compiler internals, address instability, and instrumentation nondeterminism can make traces difficult to compare across languages.

C2SaferRust combines C2Rust with LLM refactoring and validates through end-to-end tests [13]. It should be classified as test-based validation, not as straightforward differential fuzzing, unless a source explicitly shows fuzz-generated inputs driving both C and Rust implementations for divergence detection. The important distinction is whether fuzzing is part of the oracle loop or merely absent from the validation path.

RustAssure is explicitly adjacent but different: it performs differential symbolic testing rather than differential fuzzing to establish semantic similarity between original C and LLM-transpiled Rust code [7]. The paper’s critique is concrete: exact return values can differ because of runtime differences such as memory layout and heap addresses, so a concrete C/Rust mismatch may be a false positive. The paper body reports that 89.8% of all C functions generated compilable Rust, of which 72% produced equivalent symbolic return values; this is symbolic-equivalence validation, not dynamic fuzzing.

RustAssure’s critique does not make fuzzing obsolete. It identifies cases where a concrete oracle is too naive: pointers, allocation-dependent values, layout-sensitive structs, and C memory bugs that safe Rust intentionally removes. In those cases, the validator should either canonicalize outputs before comparison, compare logical state rather than raw addresses, or switch to symbolic/specification-based checks for the affected function boundary.

VERT verifies an LLM-generated Rust candidate against a WebAssembly-compiled oracle Rust program using property-based testing with Bolero and bounded model checking with Kani [8]. The arXiv version reports that combining Claude-2 with VERT increased Rust transpilations passing property-based testing from 31% to 54% and bounded model checking from 1% to 42% [22]. This is not C-vs-Rust differential fuzzing in the strict sense; the reference is a lifted oracle and the validation is PBT plus model checking.

LAC2R/VFT-style “Virtual Fuzzing-based equivalence Test” should be read as fuzzing-inspired validation rather than actual fuzzing unless executable inputs are generated and run through both implementations. A compile-free LLM prediction of divergence-causing inputs may help prioritize review, but it does not provide the same evidence as a coverage-guided harness that executes C and Rust and records concrete counterexamples.

| Method | What it compares | Needs executable C/Rust? | Uses generated inputs? | Why it is or is not differential fuzzing |
|---|---|---|---|---|
| Differential fuzzing | C and Rust implementations at the same API boundary | Yes | Yes | Same generated input drives both sides; divergence is the failure signal [1] |
| C2Rust cross-checking | Execution traces between original C and translated Rust | Yes | Not inherently | Differential comparison, but fuzzing is not the core mechanism [6] |
| RustAssure | Symbolic return values for C and Rust via symbolic execution | Executable source/toolchain needed, but not concrete fuzz execution | Symbolic paths, not fuzz inputs | Avoids concrete runtime-value false positives; not dynamic fuzzing [7] |
| VERT | LLM Rust candidate against Wasm-lifted oracle | Needs candidate and oracle | Property-based inputs plus bounded model checking | PBT/model checking against an oracle, not C-vs-Rust fuzzing [8] |
| Syzygy EqTester | Generated Rust outputs versus original C on test inputs | Yes | Test inputs | Observational equivalence testing; fuzzing is not the defining mechanism [9] |
| CROWN | Translated Rust against available project test suites | Yes | Existing tests | Regression/conformance validation, not fuzzing [12] |
| C2RustTV | Production and test code translation plus conformance execution | Yes | Test cases | Test execution pipeline, not fuzz-guided differential comparison [10] |
| RustMap | Input/output equivalence via test cases plus iterative repair | Yes | Test cases | Uses tests and compiler/test feedback, not differential fuzzing [11] |

## Technical patterns, benefits, and failure modes of differential fuzzing in migration

- **Single-driver harness**: The robust pattern is one fuzz target that constructs or receives one input and invokes both implementations. This prevents the C and Rust runs from seeing different preprocessing, environment state, or corpus mutations. Runtime Verification’s harness follows this pattern by driving C through the shared ABI and Rust through a native parser call [14].

- **C behind FFI or shared ABI**: Rust fuzz harnesses commonly call the C implementation through an FFI boundary. This lets teams use Rust fuzzing infrastructure while retaining the old C as an oracle. The trade-off is that panics, unwinding, allocator behavior, and sanitizer aborts must be handled deliberately; otherwise the harness can confuse an interop artifact with a semantic mismatch.

- **Logical-output comparison**: Comparing raw bytes is often wrong for structs because padding, pointer values, and allocation order differ. Fluorine addresses this with JSON state mapping [2], while the kernel parser case compares parsed results field-by-field [16]. The engineering rule is to compare logical state at the migration boundary, not implementation layout.

- **Sanitizer-assisted C oracle**: C undefined behavior complicates equivalence because an unsafe C result may not be behavior worth preserving. Compiling C with sanitizer and coverage instrumentation, as in the kernel parser build script, helps separate “Rust disagrees with valid C behavior” from “C executed invalid memory or arithmetic behavior” [15].

- **Counterexample feedback**: Fuzzing produces concrete failing inputs. In LLM translation systems, that input can be fed back into the prompt or repair loop; in manual migrations, it becomes a regression test. The benefit is operational: the validator does not just say “not equivalent,” it gives a reproducible input that narrows the failing semantic path.

- **Reachability limits**: A fuzz campaign that never reaches deep parser states, rare error paths, or multi-call state transitions provides weak evidence for those behaviors. Structure-aware generators, seed corpora, dictionaries, and stateful fuzzing are the usual mitigations. Without them, “no divergence” mostly means “no divergence on the paths the harness reached.”

- **Intentional safety divergence**: A Rust rewrite may panic or return an error where C would corrupt memory. Runtime Verification’s negative control illustrates this: reintroducing CVE-2024-53104 produces a C heap-buffer-overflow under ASan while Rust panics safely [16]. The test policy must decide whether such a case is success, divergence, or a separate safety finding.

- **Harness correctness risk**: Serialization, FFI wrappers, symbol renaming, field canonicalization, and panic handling become part of the trusted test code. A bug in any of them can hide a real mismatch or report a false one. For high-risk migrations, harnesses should include negative controls, known-equivalent smoke tests, and minimized reproducer checks.

In sum, differential fuzzing is strongest when the compared boundary is narrow, deterministic, and representable as a stable logical state. It is weaker for APIs whose semantics depend on pointer identity, allocator layout, global mutable state, concurrency, or undefined C behavior; those cases need symbolic reasoning, manual specifications, or trace-level instrumentation in addition to fuzzing.

## Overall assessment and gaps

Differential fuzzing is a real method in C-to-Rust validation, not just a proposed idea. Fluorine is the clearest academic example, and Runtime Verification’s kernel parser spike is the clearest practitioner case with public harness details and recorded results. The SANER 2026 system and NDSS 2025 user study add evidence that differential fuzzing is being used both in automated LLM translation loops and in evaluation of human translations.

It is not the dominant or universal method. The broader literature uses a mix of trace comparison, symbolic execution, property-based testing, bounded model checking, unit tests, project test suites, and end-to-end conformance tests. The CACM survey classifies Fluorine as the fuzzer-based equivalence approach while distinguishing other approaches that use end-to-end tests, LLM-generated tests, or PBT/model checking [23]. The PLDI 2025 paper likewise evaluates type compatibility and I/O equivalence using source-project test suites while listing differential fuzzing as a prior validation strategy [24].

The main open gap is comparative evaluation. Current papers and projects show that individual validation strategies work on particular benchmarks or harnesses, but there is not yet a broad standardized benchmark that measures fuzzing versus symbolic methods versus trace checking across the same C-to-Rust tasks. Without that, the right validation stack should be chosen by boundary type: fuzzing for executable deterministic APIs, symbolic methods for path-sensitive functions and pointer/layout false positives, sanitizers for C-side invalid behavior, and test suites for integration compatibility.

The safest wording for results is therefore bounded: “found no divergence under this harness,” “provides evidence of I/O equivalence,” or “validated on this benchmark.” Avoid claiming full semantic equivalence unless the method actually proves the relevant property over the full input and state space.

## Explore further

- **Annotated bibliography**: Build a tool-by-tool map of C2Rust, C2SaferRust, Fluorine, RustAssure, VERT, Syzygy, RustMap, CROWN, and C2RustTV, with each entry classified by oracle type, input source, and equivalence claim.

- **Practical harness guide**: For a C library and Rust rewrite, specify the FFI boundary, input generator, sanitizer configuration, panic policy, logical-state serializer, divergence minimizer, and negative controls.

- **Validation strategy comparison**: Compare unit tests, project test suites, cross-checking, differential fuzzing, symbolic execution, PBT, bounded model checking, and formal verification by cost, false-positive risk, bug classes found, and suitability for incremental migration.

## References

[1] fuzze.rs documents differential fuzzing as running two implementations of the same spec on the same input and crashing when outputs disagree, with a Rust-vs-C example (serde_json vs json-c via C ABI).. https://fuzze.rs/blog/differential-fuzzing
[2] Fluorine uses a cross-language differential fuzzer to obtain evidence of I/O equivalence between the input source program and the Rust translation.. https://arxiv.org/pdf/2405.11514
[3] runtimeverification/kernel-c-to-rust-spike is a Linux-kernel C parser (UVC video descriptor parser) extracted to C, rewritten in safe Rust, and differentially fuzzed with a LibAFL harness.. https://github.com/runtimeverification/kernel-c-to-rust-spike
[4] The SANER 2026 C-to-Rust translation system uses differential fuzzing and re-prompts the LLM when checks return a negative result.. https://arxiv.org/pdf/2512.02567
[5] The NDSS 2025 user study employs automated fuzz-testing (AFL++) to check for behavioral differences between Rust translations and corresponding C source.. https://www.ndss-symposium.org/ndss-paper/translating-c-to-rust-lessons-from-a-user-study/
[6] C2Rust's validation relies on a cross-check tool that compares execution traces of two programs on a test input, not on fuzzing.. https://hardekbc.github.io/files/emre21translating.pdf
[7] RustAssure performs differential symbolic testing rather than differential fuzzing to establish semantic similarity between original C and LLM-transpiled Rust code.. https://arxiv.org/pdf/2510.07604
[8] VERT uses a WebAssembly-compiled oracle Rust program and verifies an LLM-generated candidate against it with property-based testing (bolero) and model-checking (Kani).. https://conf.researchr.org/details/ase-2025/ase-2025-papers/57/VERT-Polyglot-Verified-Equivalent-Rust-Transpilation-with-Large-Language-Models
[9] Syzygy's EqTester checks for equivalence of generated Rust code with the original C code via observational equivalence on a set of test inputs.. https://arxiv.org/pdf/2412.14234
[10] C2RustTV integrates test case generation, automated translation of production and test code, and conformance validation via test execution rather than fuzzing.. https://ieeexplore.ieee.org/document/11126570
[11] RustMap uses test cases to check input/output equivalence and iteratively refines translation using feedback from compilation and test errors.. https://arxiv.org/abs/2503.17741
[12] CROWN (CAV 2023) validates observational equivalence empirically using all available test suites, which continue to pass after translation.. https://arxiv.org/abs/2303.10515
[13] C2SaferRust combines C2Rust with LLM refactoring and validates via end-to-end tests, with fuzzing used only to construct high-coverage system-level test cases for GNU coreutils.. https://ieeexplore.ieee.org/document/11285862/
[14] The kernel-c-to-rust-spike harness drives both the C and Rust implementations from one input in one process, calling C via the shared C ABI and Rust via the crate's native parse() inside catch_unwind.. https://raw.githubusercontent.com/runtimeverification/kernel-c-to-rust-spike/main/fuzz/src/main.rs
[15] The kernel-c-to-rust-spike build script compiles the C code with clang, enables SanitizerCoverage, and renames uvc_parse to uvc_parse_c to avoid symbol clash.. https://raw.githubusercontent.com/runtimeverification/kernel-c-to-rust-spike/main/fuzz/build.rs
[16] The kernel-c-to-rust-spike harness recorded 54,019 executions at 93% edge coverage with zero divergences after fixing one signed-overflow divergence.. https://raw.githubusercontent.com/runtimeverification/kernel-c-to-rust-spike/main/fuzz/RESULTS.md
[17] oferchen/rsync is a pure-Rust rsync reimplementation that performs wire-level differential fuzzing against upstream C rsync across multiple versions and protocols.. https://github.com/oferchen/rsync
[18] https://github.com/oferchen/rsync/tree/master/fuzz
[19] https://github.com/brunoerg/bitcoinfuzz
[20] bitcoinfuzz has discovered and reported over 35 bugs including CVE-2024-44073 in rust-miniscript, and a PR was opened to integrate it into OSS-Fuzz.. https://delvingbitcoin.org/t/the-state-of-bitcoinfuzz/1946
[21] lambdaclass/ethp2p-rs is a clean-room Rust port of a Go ethp2p stack validated against the Go reference via differential fuzzing using a C-ABI shim.. https://github.com/lambdaclass/ethp2p-rs
[22] VERT's arXiv version reports that combining Claude-2 with VERT increases Rust transpilations passing property-based testing from 31% to 54% and bounded model-checking from 1% to 42%.. https://arxiv.org/abs/2404.18852
[23] https://cacm.acm.org/research/automatically-translating-c-to-rust/
[24] The PLDI 2025 paper evaluates type-compatibility and I/O equivalence using the source project's test suite and lists differential fuzzing as a prior validation strategy.. https://mengwangoxf.github.io/Papers/PLDI25.pdf