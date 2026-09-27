# Related work — differential testing / fuzzing of C→Rust translation (landscape, 2026-07)

Survey of everyone doing correctness testing of C→Rust translation, to position our niche
(**UB-gated + name-independent-matcher + differential fuzzing** for structure-non-preserving LLM
translators). **Bottom line: the three-way intersection is unoccupied.** Each ingredient has prior art;
two pairwise combos exist; no one does all three for translators that rename/reshape.

## Master table

| Work | Venue / Year | Link | Diff C-vs-Rust? | Oracle mechanism | UB gate? | Fn matching | Method | Bugs found |
|---|---|---|---|---|---|---|---|---|
| **FLOURINE** (Eniser et al.) | arXiv 2405.11514, 2024 (AWS/Bristol) | arxiv.org/abs/2405.11514 | Yes | whole-program-state I/O equiv; serialize Rust state→JSON, replay into C, compare | No (avoids via benchmark choice) | **by name** via hand-authored JSON data-exchange spec | coverage-guided fuzzing (libFuzzer/Bolero) | non-equiv counterexamples, 5 LLMs |
| **RustAssure** (Bai, Palit, UC Davis) | **ASE 2025**; arXiv 2510.07604 | arxiv.org/abs/2510.07604 | Yes | KLEE symbolic exec of C+Rust LLVM IR; compare symbolic returns by graph edit-distance ("S³") | **No** — names UB/mem-corruption divergence as an unsolved limitation | by name + repr(C,packed)/Option layout | **symbolic execution** | **12 complex + 13 simple semantic bugs, 4 LLMs** |
| **ACToR** (Li et al.) | arXiv 2510.03879, 2025 | arxiv.org/abs/2510.03879 | Yes | GAN-style agent builds/refines a differential fuzzer over C & Rust binaries | Not addressed | binary/fn level (unspecified) | **differential fuzzing** (agent-driven) | correctness-improvement loop |
| **VERT** (Yang, Takashima, Paulsen, Dodds, Kroening) | arXiv 2404.18852, 2024 | arxiv.org/abs/2404.18852 | oracle is a **Rust** oracle (src→Wasm→rWasm), not C | Bolero PBT → Kani bounded MC → proof | No (Wasm determinism) | **mutation-guided I/O id** (not by name) | PBT + bounded model checking | pass-rate |
| **Syzygy** (Shetty et al., **UC Berkeley**) | LLM4Code/ICSE 2025; arXiv 2412.14234 | arxiv.org/abs/2412.14234 | Yes (dynamic) | SpecMiner runs C for I/O; LLM adapter + EqTester check Rust matches | Not detailed | dep-order + ArgTranslator adapter | dynamic concrete replay | Zopfli (98 fns) |
| **SACTOR** (Zhou et al.) | arXiv 2503.12511, 2025 | arxiv.org/abs/2503.12511 | Yes | FFI: recompile C linking Rust .so; E2E "soft equivalence" | UB = failure class R5, not mitigated | libclang dep-graph, topo order | E2E FFI diff testing | success rates |
| **RustMap** (Cai et al.) | ICECCS 2025; arXiv 2503.17741 | arxiv.org/abs/2503.17741 | Yes | instrument both C & Rust, compare runtime states; binary-search fault loc | Not explicit | scaffold file/fn mapping | dynamic diff testing | bzip2 unsafe 3424→122 |
| **TOUCHSTONE** (Xia, Hua, Peng, USTC) | ~2023 empirical | csslab-ustc.github.io | Yes | Csmith/YARPGen random C → whole-prog checksum C-vs-Rust | **Yes** (Csmith/YARPGen = UB-free C) | whole-program (targets rule-based **c2rust**) | random differential testing | c2rust transpiler bugs |
| **c2rust built-in** | immunant/c2rust | github.com/immunant/c2rust | light | runtime cross-check: instrument fn entry/exit, hash C vs Rust | none (preserves C incl. UB) | by name/ABI (structure-preserving) | regression cross-check | regression harness |
| **MatchFixAgent** (Ibrahimzada et al.) | arXiv 2509.16187 (ICML 2026?) | arxiv.org/abs/2509.16187 | No (LLM-judge) | LLM judges equiv over 6 semantic properties; repo-level | No | **language-agnostic / name-independent (LLM pairing)** — closest to a matcher | LLM judge | repair-oriented |
| **TeTRIS** (Arafat, Nagy, Utah) | ACSAC 2025 | IEEE Xplore 11391838 | general transpiler diff-fuzzing | fuzz w/ validity enforcement | No | N/A | fuzzing | 12 new bugs / 7 transpilers |
| **Rustlantis** (Jung et al.) | OOPSLA 2024 | research.ralfj.de | Rust-internal (not cross-lang) | random diff across backends/opt-levels; **Miri = UB oracle** | **Yes** (Miri) | N/A | random diff testing | 22 rustc bugs |
| **PtrTrans** (Yuan et al., Fudan) | **FSE 2026**; arXiv 2510.10956 | arxiv.org/abs/2510.10956 | **No — a translator** | Pointer-KG guided LLM translation; correctness = manual unit tests | No | N/A | translation | **a TARGET we can test** (repo: FudanSELab/PtrTrans-C2Rust) |

**Rule-based lifters that only re-run existing tests (weak equiv, no C-vs-Rust diff, no UB gate):**
Laertes (OOPSLA'21), CROWN (CAV'23), C2SaferRust (arXiv 2501.14257), Concrat (ICSE'23), EvoC2Rust
(arXiv 2508.04295 — compares vs a *Rust* reference not C).
**Compiler-testing lineage (informs UB handling):** Csmith, YARPGen (OOPSLA'20), EMI, Alive2, Miri.

## Niche assessment — the 3-way intersection is open

- **Differential fuzzing** exists → FLOURINE, ACToR, VERT — but matching is manual JSON (FLOURINE),
  unspecified binary (ACToR), or mutation-guided I/O (VERT); **none gate UB**.
- **UB-gating** exists → TOUCHSTONE (Csmith UB-free), Rustlantis (Miri) — but both target
  **structure-preserving** systems (rule-based c2rust / rustc), so the matcher problem never arises.
- **Name-independent matching** exists → only MatchFixAgent — but LLM-judge, not a fuzzing oracle, no UB.

**Unoccupied slot = a name-independent matcher used as the alignment layer of a UB-gated differential
fuzzing oracle, for LLM translators that rename and reshape.** Sharpest framing:
*"TOUCHSTONE/Rustlantis-style UB-gated differential testing, but for structure-non-preserving LLM
translators where function alignment can no longer be assumed."* Novelty = the matcher-enabled
combination, not any single ingredient.

## The competitors to differentiate against (in order)

1. **RustAssure (ASE 2025)** — PRIMARY head-to-head. Same domain (validate LLM-transpiled C→Rust),
   differential, finds real semantic bugs (**25**). But **symbolic (KLEE), name-based, and explicitly
   does NOT gate UB** (names UB/mem-corruption as an unsolved false-positive source). Our wedge = both
   the UB gate AND the matcher. Its multi-byte-UTF-8 blind spot (our u8strlen/u8next_ finds) is concrete
   ammo that concrete fuzzing beats symbolic here.
2. **ACToR (arXiv 2510.03879)** — closest on METHOD (builds a differential fuzzer over C & Rust in an
   agent loop). No UB gate, no principled matcher (binary-level). Distinguish on UB-gate + explicit matcher.
3. **FLOURINE (arXiv 2405.11514)** — the original C→Rust differential fuzzer. Coverage-guided
   whole-program-state, but matches **by name via a hand-authored JSON signature spec** and no UB
   handling. Our matcher automates exactly the alignment it does by hand; our UB gate is what it lacks.

## Strategic implication for our bug count

**RustAssure found 25 semantic bugs in LLM-transpiled C→Rust.** We found few so far because we tested
C2SaferRust (a lifter, mostly faithful except crc32) and SACTOR (frontier, faithful). **The bugs live in
raw / weaker LLM translation output** — exactly what RustAssure tested and what PtrTrans
(FudanSELab/PtrTrans-C2Rust) releases (C source + idiomatic Rust + C↔Rust mapping + degraded ablation
variants). Next target: C-backed differential of PtrTrans's output → the right place to grow to ~10 bugs.

---

## Excluded from citation — C2VR (decision 2026-07-07)

**Paper:** Xia, Ou, Su, Guo, Y. Li, L. Chen (Nanjing University), *"From C to Verifiable Rust: Towards
Practical Migration of Code and Specifications,"* Science of Computer Programming, accepted 2026-06-29,
DOI 10.1016/j.scico.2026.103535.

**What it is:** C2VR migrates ACSL-annotated C to Verus-verifiable Rust — an LLM translates the code and a
rule-based engine migrates ACSL contracts into Verus specs; Verus then deductively (SMT) proves the Rust.
175 annotated programs → 92 auto-verified / 139 with manual fixes. Contribution taxonomy = 3 categories /
15 symptoms / 10 root causes / 9 repair strategies of **migration failure**.

**Why we do NOT cite it:**
1. **Venue tier.** Science of Computer Programming is a CCF-C / low-tier journal (legitimate Elsevier, but
   not a bar-setting venue for a top-SE submission). SE colleagues consulted; consensus = not required RW.
2. **Different verification paradigm.** Their "verification" is deductive formal proof (Verus/SMT) of
   *pre-annotated* programs; ours is UB-gated differential *testing* of *arbitrary, unannotated* translator
   output. Non-overlapping.
3. **Orthogonal taxonomy.** Their 10 root causes categorize why *their pipeline fails to verify*
   (compile / verify / spec-migration failures) — not runtime behavioral bugs. Our 3 classes
   (crash / semantic-diff / hang) are observable-outcome classes tied to a differential oracle. Different axis.
4. **Input-regime mismatch.** C2VR requires ACSL contracts on the C; our niche is exactly the arbitrary
   translator output where no specification exists. So C2VR is **not added as a translator column in E1/E2.**

**Optional (not pursued):** differentially test C2VR's Verus-verified Rust to demonstrate "formally
verified ≠ behaviorally equivalent" when the migrated spec is incomplete — a strong but ambitious point
needing their artifact.
