# Research index — settled survey results (don't re-research these)

> This file preserves literature findings, not the current contribution or RQ
> framing. Statements below about “our differentiators,” STU/frontier selection,
> or old RQ numbers are historical. Use `results/EVALUATION_PLAN.md` for the
> current evaluation.

One-stop index of the web-research / survey work done for this project. Each item is a SETTLED
conclusion with a pointer to the full write-up. Before launching a new research agent, check here.

---

## 1. SOTA static/hybrid C→Rust tools — `results/sota_static_tools_survey.md` (2026-06-28)
**Settled:** NO pure-static CROWN successor exists; the c2rust→Laertes('21)→CROWN('23) static line
STALLED in 2023; 2024-26 went LLM/hybrid. So CROWN/Laertes remain the only general static baselines.
- Narrow static lifts (KAIST, likely version-pinned like CROWN): **Forcrat** (ASE'25, I/O), **GenC2Rust**
  (ICSE'25, void*→generics), **Concrat** (ICSE'23, locks).
- **Scylla** (OOPSLA'26): genuinely static full C→safe-Rust but "VERY EXPERIMENTAL", needs hand-regularized
  C, won't batch.
- Full-translation hybrid/LLM: **RustMap** (DISMISSED — LLM step = human copy-paste, no e2e), EvoC2Rust.
- Prior-art differential ORACLES (= our category): **Fluorine**, **RustAssure** (see #2); also RustAssure
  cites them. CRUST-bench has NO native C↔Rust diff oracle (building it = our work). "Crusty" = doesn't exist.

## 2. Closest competitors Fluorine & RustAssure (their eval) — `results/related_oracles_eval_methodology.md` (2026-06-28)
**Settled:** both are END-TO-END (gen+compare) and only test their OWN LLM-generated, NAME-PRESERVING
translations (Fluorine prompt forces name preservation; RustAssure pairs by name). Both IGNORE UB.
- **Fluorine** (arXiv:2405.11514, preprint, NO repo): diff fuzzing (JSON-over-FFI), "5min no counterexample
  = equivalent", 97% cov. Corpus 408 fns/7 OSS. GPT-4o 47.3% success; 52.6% failures = serialization.
- **RustAssure** (arXiv:2510.07604, **ASE'25**): diff symbolic (KLEE + graph-edit-dist, S³=0). Corpus 5
  codebases/176 fns. compile 89.8%, equiv ~72%, 12+13 bugs, precision 85.7/88.2% recall 100% (on 2/5).
  HARD version pins LLVM14/Rust1.64. github davsec-lab/rustassure.
- **Our 3 differentiators each fill a NAMED gap**: name-independent matcher (both name-bound), UB-free
  counting (neither does UB), frontier selection (Fluorine fuzzer 10/31 survive; RustAssure 5-15h/≤600LoC).
- **Adopt**: precision/recall vs manual ground truth (RustAssure rigor, cover MORE than 2/5); Fluorine's
  compile→equiv + failure-mode table; Table-VIII-style oracle-comparison row.
- **Shared benchmark = libopenaptx** (both used it) → 3-way head-to-head possible (we have it cloned).

## 3. RQ3 human-port candidates — `results/rq3_human_port_candidates.md` (2026-06-30)
**Settled:** small C libs with INDEPENDENT idiomatic Rust REIMPLEMENTATIONS, for the RQ3 human-port row
(hand-label C↔Rust map). KEY TENSION: literal pairs easiest to label but weakest matcher test.
- **Top picks**: tinyexpr→tinyexpr-rs (~29 fns; ALSO a raw-LLM seed → controlled LLM-vs-human comparison);
  heatshrink→embedded-heatshrink (non-crypto stream); QOI→qoi-rust (HARD: free-fns→struct methods).
  Validate workflow on tiny xoshiro/PCG first.
- **On-topic bonus**: lodepng→lodepng-rust = Citrus-converted-THEN-humanized (transpile+cleanup workflow).
- **AVOID**: FFI bindings (sds/stb_image-servo/jsmn-rs/Monocypher/cityhash-sys), total rewrites
  (serde_json/csv-core/url, no lineage), literal-not-idiomatic (stb_image_rust/tweetnacl = ceiling controls).

---

## Adjacent settled investigations (not web-research, but don't redo)
- **CROWN version wall** — `results/crown_crustbench_investigation.md`: CROWN rewriter version-coupled to
  exact ~2022 c2rust; even old c2rust 0.18 caps at 12/87 clean on CRUST-bench. Don't chase exact commit.
- **Corpus inventory** — `results/corpus_inventory_v1.md`: ~19 aligned C+c2rust+CROWN programs.
- **Head-to-head corpus** — `results/headtohead_corpus.md`: RustAssure repo bundles libopenaptx dual
  translations + 16 ground-truth bugs.
- **LibAFL infra** — real `libfuzzer-sys` not `libafl_libfuzzer` (latter ignores -max_total_time, hangs).
