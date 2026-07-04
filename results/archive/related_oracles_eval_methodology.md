# Closest prior work — Fluorine & RustAssure: their evaluation, and how ours differs (2026-06-28)

These two are our **nearest competitors** (differential testing of C→Rust). Studying *how they evaluate*
shapes our own eval. (Spelling: the paper is **Fluorine**, not FLOURINE.)

## Fluorine — arXiv:2405.11514 (preprint, no confirmed venue)

- **Artifact:** NO GitHub repo — a CloudFront tarball only → license/versions unverifiable (a
  reproducibility weakness, but NOT a documented version-coupling wall). LLM-based (GPT-4o/Claude/Gemini/
  Mixtral via Bedrock, temp 0.2). Fuzzer = Bolero + libFuzzer. Baseline = c2rust.
- **Oracle:** cross-language differential **fuzzing**. Rust harness generates input states, maps to C/Go
  via **JSON ser/de over FFI**, compares output states. Decision rule: *"equivalent if 5 min of fuzzing
  returns no counterexample."* Avg line coverage 97%.
- **Pairing:** NAME-PRESERVING **by force** — prompt instructs "maintain same function/parameter names";
  JSON oracle requires matching **field names**. No rename/restructure handling — it *prevents* renaming.
- **UB:** NONE. No UBSan/ASan/Csmith. Floating point not addressed. Nondeterminism → temp 0.2 + 3 reruns.
- **Corpus:** 408 per-function benchmarks from 7 OSS (2 C: libopenaptx, opl; 5 Go), 13–597 LoC, 1–25 fns.
- **Headline:** success (compiles + passes 5-min fuzz): GPT-4o 47.3% … Mixtral 19.5%. Dominant failure =
  **serialization failures 52.6%** (oracle limitation, not translation bug — they account for this).
- **Threats:** fuzzer incompleteness (equivalence is heuristic); serialization "failures" may be correct.

## RustAssure — arXiv:2510.07604, **ASE 2025** (peer-reviewed; directly critiques Fluorine)

- **Artifact:** github.com/davsec-lab/rustassure (8★, **NO license**, README on branch `configurations`).
  **HARD version wall (CROWN-like):** Clang/LLVM **14.0.0**, Rust **1.64.0** (coupled to LLVM14 backend),
  KLEE v3.1, SVF submodule, custom LLVM pass — all built from source, no lockfile. LLM: GPT-4o etc.
- **Oracle:** differential **symbolic** testing. C & Rust → LLVM IR → KLEE symbolic exec → return-var
  symbolic exprs in KQuery → graph → **graph-edit-distance (S³ score)**; **S³=0 ⇒ equivalent**. A
  normalization stage strips Rust-specific IR artifacts.
- **Pairing:** by function NAME (Symbolizer instruments same-named fns); manual review of buggy cases. No
  rename handling.
- **UB:** NO filter — but **explicitly names it**: *"C and Rust functions will not be symbolically
  equivalent if the C code has memory corruption bugs absent in Rust … can result in false positives."*
  FP/nondeterminism not addressed. Known FP source: KLEE array-modeling.
- **Corpus:** 5 real C codebases, 176 fns (libcsv, urlparser, optipng, libbmp, u8c). + Fluorine's
  libopenaptx for head-to-head.
- **Headline:** compile GPT-4o 89.8%; equivalent (S³=0) ~72% (abstract says 69.9% — unreconciled); bugs
  12 complex + 13 simple; **precision 85.7%/88.2%, recall 100% (on only 2/5 codebases)**; 5–15h/codebase,
  180-min/fn timeout, pointer-nesting cap 10.
- **Baselines:** vs CROWN (compile 97.3% vs 89.8%, but 271 vs 35 raw-ptr decls). vs Fluorine on
  libopenaptx: **"of 31 compilable Fluorine fns, only 10 pass Fluorine's fuzzing (fuzzer crashes), while
  RustAssure establishes equivalence for 25/31."**
- **Threats:** memory-corruption FPs (unsolved), KLEE array FPs, scalability (timeout/nesting cap),
  precision measured on 2/5 only.

## How this shapes OUR evaluation

**Our 3 differentiators each plug a NAMED, unsolved gap in the closest prior work:**
1. **Name-independent matcher** — both are name-bound; Fluorine *forces* name+field-name preservation
   (admits oracle breaks otherwise), RustAssure pairs by name. We remove the precondition both depend on;
   demonstrate on renamed/restructured translations where both silently fail to pair.
2. **UB-free bug counting** — NEITHER uses UBSan/ASan/Csmith. RustAssure's own threats section *asks for*
   exactly this. Cite that sentence; our UB gate is the fix.
3. **Frontier selector** — answers both opposite failure modes: Fluorine's fuzzer brittleness (10/31
   survive) and RustAssure's 5–15h / ≤~600 LoC scalability ceiling. Select risky boundaries → efficiency
   neither has.

**Adopt (RustAssure's rigor + Fluorine's legibility):**
- Report **precision AND recall vs a manually-verified ground-truth subset** (RustAssure's best practice)
  — but cover MORE of the corpus than their 2/5 to beat that threat.
- Use Fluorine's **compile-then-equivalence success metric + failure-mode breakdown table**.
- Build a **Table-VIII-style oracle-comparison** row: ours (selective fuzzing + UB gate + name-free
  matcher) vs differential fuzzing (Fluorine) vs differential symbolic (RustAssure).

**Corpus strategy:** NO shared standard benchmark in this niche; each rolls its own. De-facto shared
artifact = Fluorine's **libopenaptx** (RustAssure reused it) → do a **direct 3-way head-to-head on
libopenaptx**. Plus adopt **CRUST-bench** (100 real repos) for larger scope neither competitor uses.

**Reproducibility = free secondary contribution:** Fluorine (no repo) and RustAssure (8★, no license,
LLVM14↔Rust1.64 hard pin + KLEE/SVF-from-source) are both fragile/version-coupled — a modern, lock-filed,
less-pinned artifact is a credible bonus.
