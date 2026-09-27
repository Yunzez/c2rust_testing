# Survey: SOTA static/hybrid C→Rust tools (2026-06-28, agent-researched)

## Headline finding

**There is no pure-static CROWN successor.** The static lineage (c2rust → Laertes OOPSLA'21 → CROWN
'23) **stalled in 2023**; 2024–2026 work went neuro-symbolic/LLM. So we cannot simply swap CROWN for a
newer, more-robust static lifter — CROWN/Laertes remain the only general-purpose static baselines of
record. This justifies keeping our 12 (neutral) + 19 (established) CROWN corpus and not over-investing.

**Also (study-design relevant):** CRUST-bench ships hand-written safe-Rust *interfaces* + Rust-side test
harnesses, and its authors **deliberately excluded c2rust and static transpilers** as incompatible. So
CRUST-bench has **no native C↔Rust differential oracle** — building that oracle is exactly our work.

## Ranked shortlist

| # | Tool | Venue/Year | Link | Does | Static? | Input | Repo / runnable | Batch w/o tuning | Name-preserving |
|--:|------|-----------|------|------|---------|-------|-----------------|------------------|-----------------|
| 1 | **RustMap** | arXiv 2025 | 2503.17741 | full project C→Rust | **hybrid** (analysis+GPT-4o) | raw C | github.com/Cxm211/RustMap | yes (claimed) | **yes** |
| 2 | **Forcrat** | ASE 2025 | 2506.01427 | lift: libc FILE* → std::io | **yes** | c2rust output | KAIST; repo URL unconfirmed | yes-ish (62 progs) | yes |
| 3 | **GenC2Rust** | ICSE 2025 | DOI .../00127, Zenodo 15009030 | lift: void* → generics | **yes** | c2rust Rust | Zenodo; GitHub unconfirmed | promising (42 progs) | yes |
| 4 | **Concrat** | ICSE 2023 | 2301.10943 | lift: pthread locks → Mutex | **yes** | C + c2rust | github.com/kaist-plrg/concrat | partial (locks only) | yes |
| 5 | **EvoC2Rust** | arXiv 2025 | 2508.04295 | full project, ~98% safe | **hybrid** | raw C | github.com/bbzswcf/EvoC2rust | project-level | restructures |
| 6 | **Scylla** | OOPSLA 2026 | 2412.15042 | full static C→safe Rust | **yes** | raw C | github.com/AeneasVerif/scylla | **no** (needs regularized C) | restructures |
| — | Citrus | community | gitlab citrus-rs | syntactic only | yes | raw C | legacy 2023; output often won't compile | — | unreliable |
| — | "Crusty" | — | — | **does not exist** (name conflation) | — | — | — | — |

VERT (2404.18852) / Syzygy (2412.14234) = LLM-hybrid full translation, not static.

## Best next candidate (if we want a richer translation lane)

**RustMap** — only option that is (a) public+runnable, (b) batch whole-repo w/o per-program tuning,
(c) name-preserving (needed for C↔Rust pairing), (d) consumes **raw C** → sidesteps the CROWN/Concrat
old-c2rust version-coupling wall entirely. Caveat: **hybrid (GPT-4o)** → not strictly deterministic, and
costs tokens. If we need a strictly-static point, fall back to stacking narrow static lifts (Forcrat +
GenC2Rust + Concrat) over a pinned c2rust — but each covers only a slice (I/O, void*, locks).

## Prior-art differential oracles (related work + reusable infra)

- **FLOURINE** (arXiv:2405.11514) — differential **fuzzing** with JSON-over-FFI state mapping; notably
  it **forces name-preserving output** to pair C↔Rust. Directly adjacent to our line; our matcher's value
  is precisely handling the **renamed** case FLOURINE sidesteps. (Already a bib key in the paper.)
- **RustAssure** (arXiv:2510.07604) — differential **symbolic** testing (KLEE + graph-edit-distance);
  repo github.com/davsec-lab/rustassure; pins LLVM 14 / KLEE / SVF / Rust 1.64.

## Implications for our study

1. Keep CROWN/Laertes as the static baselines (no successor to swap in). Our CROWN corpus stands.
2. A richer translation lane = LLM/hybrid (RustMap best; raw-C input avoids version walls) — costs tokens,
   not deterministic. This is the same "spend tokens on a neutral dataset" decision as C2SaferRust/SACTOR.
3. FLOURINE / RustAssure are the closest prior art — cite them; our differentiators are (a) the
   name-independent matcher for renamed/idiomatic translations, (b) UB-correct divergence counting +
   frontier selection.
4. 2026 arXiv cluster (ENCRUST, C2RustXW, etc.) = bleeding-edge, unverified — ignore for now.
