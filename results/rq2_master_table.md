# E2 Master Table — matcher accuracy (name-independent C↔Rust alignment)

**The paper's Table 2** — the enabler-novelty. E1 shows the tools produce bugs; E2 shows we can
**recover the C↔Rust function correspondence WITHOUT function names**, on the *same* libraries and the
*same* tool artifacts — which is what makes the E1 differential possible on structure-non-preserving
translators (the thing FLOURINE / RustAssure's name-pairing cannot do).

**Rows = the E1 libraries** (locked to E1 on purpose: one corpus, two tables; future-proof if a
reviewer wants more libraries — the format doesn't change, only rows are added).
**Columns = the 6 translators + raw-LLM** (ordered left→right by rename-aggressiveness = matcher
difficulty). Every cell is the same contrast: **what the matcher recovers vs what name-equality
recovers.**

## Cell legend

| mark | meaning |
|---|---|
| `R.xx / N.yy` | **matcher recall** (correct ÷ scorable, name-independent) **/ name-equality recall** (the baseline competitors use). The gap is the result. |
| superscript `ᴺ` | truth is **name-derived** — this tool preserves C names, so name-equality *is* ground truth (free to build); the matcher's job here is to **recover it blind** (validation it isn't cheating on names) |
| superscript `ᴴ` | truth is **hand-labeled** — this tool renames/restructures, so the correspondence was labeled by hand (expensive) |
| superscript `ᴵ` | truth is **independent/mechanical** (e.g. a name-scramble applied to a known-correct pairing) |
| `↑topo` | topology propagation is **load-bearing** here (homogeneous io-shape cluster; per-function signals alone fail) |
| `—` | **no analyzable artifact** — the tool produced no parseable Rust for this library (inherits the E1 tool-failure ✗/×); nothing to match |
| `∅ᴺ / ∅ᴴ` | **not yet run (TODO)**, superscript flags the truth-source cost (ᴺ ≈ free, ᴴ = needs labeling) |
| `≡ / ≠` | **column naming behavior**: `≡` = tool keeps C names (name-equality works) · `≠` = tool **renames** (name-equality breaks → matcher needed). PtrTrans renames a subset (camelCase→snake_case, e.g. `quickSort→quick_sort`); raw-LLM renames everything. c2rust/Laertes/C2SaferRust/CROWN/SACTOR keep names. |
| `★` (PtrTrans qsort) | real-tool rename proof: PtrTrans renamed `quickSort→quick_sort`, so **name-eq = 0.67 (fails on it), matcher = 1.00 (recovers by structure)** — the matcher is needed on a SHIPPED tool, not only the synthetic raw-LLM set. |
| `⊘` | excluded. NOTE: E1's UB-gate does **not** apply to E2 — matching is static, it never executes the C — so E1's `⊘(C-side UB)` cells (urlparser) ARE matchable here. `⊘` in E2 means a *matching-specific* block only. |

Method for every filled cell: run both analyzers (C via `c_analyzer.py`, Rust via `analyzer/`), feed
`matcher.match()` (io-shape + metrics + operator histogram + call-graph topology; NO names), score
`precision / recall / coverage` micro+macro against the truth file. Runner: `scripts/eval_rq2_matcher.py`
(currently `eval_rq3_matcher.py` — legacy name). Matcher: `tools/stu_selector/matcher.py`.

## The table

| library | domain | ~#fn | homog.¹ | c2rust ≡ | Laertes ≡ | C2SaferRust ≡ | CROWN ≡ | SACTOR ≡ | PtrTrans ≠ | **raw-LLM** ≠² |
|---|---|---:|---:|---|---|---|---|---|---|---|
| **qsort** | sorting | 3 | low | 1.00/1.0 | 0.67/1.0 | 1.00/1.0 | ∅ᴺ | ∅ᴴ | **1.00/0.67** ★ | **0.67/0.0**ᴴ |
| **urlparser** | URL parsing | 21 | low | 0.95/1.0 | 0.91/1.0 | 0.95/1.0 | 1.00/1.0 | — | — | **0.88/0.0**ᴴ |
| **quadtree** | spatial tree | 24 | med | 1.00/1.0 | — | — | 0.63/1.0 | — | ∅ᴴ | **0.71/0.0**ᴴ |
| **genann** | neural net | ~20 | med | 1.00/1.0 | 1.00/1.0 | 1.00/1.0 | 1.00/1.0 | ∅ᴴ | ∅ᴴ(decl) | **1.00/0.0**ᴴ |
| **cJSON** | JSON parser | 58 | high | 1.00/1.0 | — | — | — | — | ∅ᴴ(partial) | ∅ |
| **lil** | interpreter | 145 | **high ↑topo** | 0.97/1.0 | 0.96/1.0 | 0.99/1.0 | 0.92/1.0 | — | — | ∅ |
| **lodepng** | PNG codec | 235 | high | 0.99/1.0 | — | — | 0.97/1.0 | — | — | ∅ |
| **bzip2** | compressor | 64 | high | 1.00/1.0 | 1.00/1.0 | 0.98/1.0 | 1.00/1.0 | — | — | ∅ |
| **tulipindicators** | indicators | ~100 | **very high** | ∅ᴺ | ∅ᴺ | ∅ᴺ | ▽ᴺ | — | — | ∅ |
| **optipng** (incl. zlib) | PNG optimizer | ~400 | high | ∅ᴺ | ∅ᴺ | ∅ᴺ | — | — | — | ∅ |

¹ **homog.** = homogeneity of the io-shape distribution — how many functions share the dominant
signature. HIGH homogeneity is what defeats per-function matching and forces call-graph topology
(lil: 55/128 functions share one shape → the `↑topo` stress case). This is E2's analogue of E1's
"scale" column: the axis that actually makes the problem hard.
² **raw-LLM** = our own gpt-5.1 translation (prompt below) — the *maximal-rename* reference column.
Not one of the 6 shipped tools; included because it is the hardest case for name-equality and the
clearest demonstration of the matcher's value. All ∅ pending generation on the E1 libraries.

## Filled so far — name-preserving batch v1 (2026-07-07)

**24 cells** filled with real matcher runs on the E1 artifacts (`results/rq2_cells/name_preserving_v1.json`),
across **8 libraries** (qsort / urlparser / quadtree / genann / cJSON / lil / lodepng / bzip2). Cell =
**matcher-recall / name-eq-recall**; name-eq = 1.0 for these tools (names kept), so the cell is a
**validation** (the matcher recovers the correspondence BLIND, not a "beat the baseline").

**Naming-behavior finding (2026-07-07) — not all shipped tools keep names.** c2rust / Laertes /
C2SaferRust / CROWN / SACTOR keep C names (name-equality = free truth; SACTOR's shipped
`function_name_map.json` is identity in our examples). **PtrTrans RENAMES a subset** (camelCase→snake_case
and stubs) — verified on qsort (`quickSort→quick_sort`) and lodepng. So there IS a real, shipped tool
where name-equality fails, and the matcher is needed on it — not only on the synthetic raw-LLM set.
**Real-tool proof (qsort × PtrTrans, cell ★): name-eq = 0.67 (fails on the renamed `quickSort`),
matcher = 1.00 (recovers it by structure).** raw-LLM (renames everything) remains the only column with
no free mapping → hand-labeled. Column naming behavior is marked in the header: `≡` keeps names,
`≠` renames.

- **Scale holds up**: 235-fn `lodepng` (0.99 c2rust / 0.97 CROWN) and 145-fn `lil` (0.97/0.96/0.99/0.92)
  — the matcher recovers ~95%+ BLIND on the two largest, highest-homogeneity libraries, across every
  name-preserving tool. genann perfect 12/12 ×4.
- **The honest low points, both informative**:
  - `qsort` × Laertes **0.67 (2/3)** — 3 near-identical int-pointer functions; nothing for structure to
    grip. Small = hard.
  - `quadtree` × CROWN **0.63 (15/24)** while c2rust = 1.0 on the same setup — CROWN's `--force-box`
    rewrite reshapes the pointer-heavy node signatures (nw/ne/sw/se) enough to break structural
    matching. A clean "aggressive reshaping degrades matchability" data point — and a motivation for
    signal-C (constants/literals) beyond shape+topology.
- **UB-gate cell recovered**: `urlparser` — E1 had to exclude all its cells (C-side UB); E2 matches it
  fine (0.95/0.91/0.95/1.0), because matching never executes the C. A dead E1 row becomes 4 live E2 cells.

**Blind-check passed on every filled cell**: scrambling the Rust function names to opaque `r_####` IDs
leaves the matcher recall **identical** while name-equality drops to 0.0 — proof the matcher never uses
names (the `score()`/`node_sim()` functions read io-shape/metrics/operators/topology only). This is the
anti-cheating guarantee for the whole ᴺ column: the numbers are the same whether names are there or not.

Corpus-hygiene rule (learned building v1): match the library-core `.rs` only — exclude example/test/
driver files, or extra Rust functions become false attractors (genann×c2rust read 0.58 with drivers
included, 1.00 once isolated). C-side compile_commands must be rewritten with local paths (the shipped
laertes_benchmarks ones point at `/Users/emre/…`).

## raw-LLM column v1 (2026-07-07) — the beat-the-baseline result

**4 cells** on the E1 libraries, hand-labeled truth (`results/rq2_cells/rawllm/`). gpt-5.1, the disclosed
prompt (commands renaming, leaks no correspondence). This is the ONLY column where **name-equality = 0.0**
(the LLM renamed everything) — so it is the only place the matcher's name-independence is actually
*tested* rather than validated.

| library | matcher | name-eq | what happened |
|---|--:|--:|---|
| **genann** | **1.00** (8/8) | 0.0 | renamed all + dissolved 4 (read/write=FILE-IO, free=Drop, copy=derive Clone) + free-fn→method; matcher still recovered `genann_init→new`, `genann_randomize→randomize_weights`, etc. |
| **urlparser** | **0.88** (15/17) | 0.0 | misses `url_get_query` (get_query/search homogeneous family) + `strff→skip_forward` (tiny helper) |
| **qsort** | 0.67 (2/3) | 0.0 | LLM added idiomatic wrapper `quick_sort`; matcher picked it over the true recursive core `quick_sort_range`. Tiny programs are adversarial (no structure to grip). |
| **quadtree** | 0.64 (9/14) | 0.0 | `is_empty↔is_pointer` swap (near-identical `&self→bool` predicates), `walk` (fn-ptr→closure reshape) missed. 3 `*_new` constructors EXCLUDED — the analyzer collapses 4 `new` methods to 1 ([[matcher-hir-id-todo]] name-collision). |

**Read**: matcher recovers **.64–1.00** under genuine LLM renaming where name-equality gets **0.0** across
the board — this is the "matcher enables differential testing on renaming translators" claim, on the E1
libraries. The two low cells (qsort, quadtree) are the adversarial small/pointer-heavy cases with
homogeneous predicate clusters; the medium ones (genann, urlparser) are strong. Matches the separate
llm-transpiler corpus (.88 micro over 10 programs). **Hand-labeling honesty**: `scorable` = C functions
with a genuine Rust counterpart; LLM-dissolved functions (FILE-IO, free/copy → Drop/Clone) are excluded
(can't match what wasn't translated), and dissolution counts are reported per cell, not hidden in recall.

**hir-id fix applied (2026-07-07)**: the analyzer now keys functions by `Type::method` (impl methods)
/ bare name (free fns) instead of bare-name-with-silent-dedup — see [[matcher-hir-id-todo]], DONE. This
recovered quadtree's 3 dropped constructors: **quadtree raw-LLM 0.64 (9/14, 3 excluded) → 0.71 (12/17,
no exclusions)**, all three `*_new` now distinguished by topology. Re-validated: name-preserving cells
stable (±1 fn from edge re-keying), qsort/genann/urlparser raw-LLM unchanged. The fix caps nothing and
only makes multi-constructor libraries scorable — accuracy preserved.

**Open**: raw-LLM on the larger libs (cJSON/lil/lodepng/bzip2 — more structure, expect higher recall) +
resolve the analyzer name-collision (hir-id) which currently caps quadtree-like multi-constructor cases.

## Column ordering rationale (rename axis)

Left→right the columns climb a difficulty gradient — the further right, the more the tool destroys
names, the more name-equality collapses, and the more the matcher has to earn its keep:

- **c2rust** — names + io-shapes preserved. Trivial control; matcher should ≈ name-equality ≈ 1.0.
- **Laertes / C2SaferRust / CROWN** — **names preserved, io-shapes transformed** (ptr→slice,
  ownership lifts). Name-equality still ~1.0, so these VALIDATE the matcher recovers the known pairing
  blind despite reshaping. (Same regime we already proved on SACTOR-idiomatic: recall 1.000.)
- **SACTOR (idiomatic) / PtrTrans (reshape)** — partial renaming + heavy reshaping. Name-equality
  starts to break; the matcher's margin opens.
- **raw-LLM** — full aggressive renaming (by explicit prompt instruction). Name-equality floor;
  matcher's win is maximal.

## Anti-cheating: the exact raw-LLM prompt

The raw-LLM translations were generated with the prompt below (`experiments/llm_transpiler/prompts/translate.md`,
verbatim). Two properties matter for E2's integrity:

1. **It commands renaming** — *"Rename functions to idiomatic Rust style … do NOT keep the original C
   names … This renaming is intentional and required."* So the name-equality baseline collapse is
   genuine and adversarial, not manufactured by us. We test the matcher on the HARDEST naming case.
2. **It leaks nothing about the correspondence** — the prompt says only "roughly one Rust function per
   C function (preserve the call structure)"; it never tells the model (or the matcher) which Rust
   function maps to which C function. The matcher sees only structure.

> You are an expert C-to-Rust translator. Translate the given C source into a single, self-contained,
> idiomatic, **safe** Rust **library** crate that is semantically equivalent to the C.
> Rules: **Rename functions to idiomatic Rust style (snake_case, descriptive names). This renaming is
> intentional and required** — do NOT keep the original C names, and do NOT use `#[no_mangle]` or
> `extern "C"`. Keep roughly **one Rust function per C function** (preserve the call structure) …
> Use **only the Rust standard library** … The crate must compile as a `lib` crate.

Caveat to state in the paper: "roughly one function per C function" is a simplifying assumption that
makes an approximate 1:1 truth exist. It is NOT perfectly honored in practice — on tinyexpr the model
dissolved 13/28 C functions into inlined one-liners, on bignum it did a to_int↔to_string return-swap —
and those are exactly the residual matcher failures (§ capability), so the assumption is not doing
hidden work.

## Matcher capability already established (on the LLM-transpiler corpus)

These numbers are on a SEPARATE 10-program corpus (base64…tinyexpr), not the E1 libraries above — they
prove the matcher works before we port it onto the E1 artifacts. Full data: `results/rq3_matcher_v1.md`,
rows `results/rq3_rows/*.v2.json`.

| regime | progs | matcher recall μ/M | name-eq recall μ/M | gap |
|---|--:|--:|--:|--:|
| raw-LLM gpt-5-mini (renamed, hand truth) | 10 | **.876 / .961** | .124 / .086 | **~7–11×** |
| raw-LLM gpt-5-nano (renamed, hand truth) | 10 | **.867 / .914** | .613 / .502 | matcher stable, name-eq swings |
| SACTOR idiomatic (names kept, shape moved, indep. truth) | 2 | 1.000 / 1.000 | 1.000 / 1.000 | validation |
| SACTOR mechanically-renamed (indep. truth) | 2 | **1.000 / 1.000** | 0.000 / 0.000 | maximal |

Ablation ladder (lil 128-fn homogeneous stress): shape-only **.359** → node-only **.742** →
**full + call-graph topology .984** (+24pp from topology alone). The topology signal is what cracks the
homogeneous `fnc_*` cluster — the plan in `~/.claude/plans/jaunty-noodling-lark.md` is DONE, not pending.

## What this table needs to become "filled" (the work-list)

- **`ᴺ` cells (cheap, ~40 cells)**: name-preserving tool artifacts already exist (E1's c2rust/Laertes/
  C2SaferRust/CROWN columns). Truth = name-equality (auto). Run matcher blind, record recall vs the
  1.0 name-eq baseline. This is a batch job, no labeling, no LLM cost.
- **`ᴴ` cells (expensive, ~6 cells)**: SACTOR/PtrTrans artifacts that exist (qsort, genann, quadtree,
  cJSON) — need hand-labeled truth (the tools rename/reshape). We already have the hand-labeling
  discipline from the LLM-transpiler corpus.
- **raw-LLM column (LLM cost)**: generate gpt-5.1 translations of the 10 E1 libraries with the prompt
  above, hand-label truth, run. The most decisive column — highest priority after the `ᴺ` batch.
- **`—` cells**: nothing to do (no artifact) — they inherit E1's tool-failures and stay `—`.

Totals when filled will read like E1's: per-column mean matcher-recall vs name-eq-recall, plus the
headline "matcher holds ~.9 while name-equality collapses to ~.1 on the renaming tools."
