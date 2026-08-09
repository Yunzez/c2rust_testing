# E2 Master Table — matcher accuracy (name-independent C↔Rust alignment)

**The paper's Table 2.** E1 shows the tools produce bugs; E2 measures whether the C↔Rust function
correspondence can be recovered **without function names**, on the *same* libraries and the *same* tool
artifacts.

> ### ⚠️ STATUS AND CLAIM SCOPE (corrected 2026-08-09) — read before quoting this table
>
> **Fill state: 32 of 48 attemptable cells.** (70 grid cells − 22 with no artifact = 48 attemptable;
> 16 still open.) The **tulipindicators and optipng rows are almost entirely unfilled**, and the
> **SACTOR column has one open cell in every row it could occupy** — including `genann×SACTOR`, which
> supplies an E1 headline bug. This table is **not complete**; do not present it as such.
>
> **The "enabler" claim is retracted as previously worded.** The header used to say the matcher "is what
> makes the E1 differential possible." That is not supported by our own evidence:
> - **5 of the 6 shipped tools preserve C function names** (only PtrTrans renames, and only a subset) —
>   so name-equality suffices for most of the grid.
> - **The matcher found none of the 20 E1 bugs.** 16 of 20 are in name-preserving artifacts; the qsort
>   ★ cell is the one place a real tool's rename (`quickSort→quick_sort`) made the matcher necessary, and
>   it buys exactly one function.
> - The maximal-rename regime (raw-LLM, where name-eq recall collapses to 0.0) produced **zero E1 bugs**
>   and has no column in Table 1 at all.
>
> **What E2 can honestly claim, in priority order:**
> 1. **Alignment is a measurable error source, and we are the only ones who measured it.** The PtrTrans
>    audit — **143/255 (56%)** of a *shipped, FSE'26-published* C↔Rust map is wrong, 102 of them airtight
>    scrambles including `lodepng_save_file → "load_file"` (mapped to its semantic opposite) — is the
>    strongest and most defensible result here. It answers "why not just use the tool's own map?" with a
>    number. See `PROJECT_RESET_2026-07-03.md:78-85`. **This belongs in the paper as a result, not a
>    footnote.**
> 2. **Precision under abstention, not recall, is the deployment metric.** For a differential oracle a
>    confidently *wrong* pairing manufactures false bugs, while an abstention merely leaves a function
>    untested. The number to lead with is **precision .969 at coverage .73** (25 ambiguous pairs isolated
>    rather than guessed, `rq3_matcher_v1.md:52`) — *not* forced recall .876, and certainly not the
>    per-cell recalls of 0.55 (cJSON), 0.55 (lil), 0.63 (bzip2) under maximal rename.
> 3. **The technique itself is not novel** — signature + call-graph topology + operator histograms is the
>    standard binary-code-similarity feature set, and MatchFixAgent already does name-independent
>    cross-language pairing. Claim the *application and the measurement*, never the technique.
>
> **The one experiment that would restore an enabler claim:** hold the oracle fixed and vary only the
> alignment source ∈ {name-equality, tool-shipped map, matcher, hand-truth}, then report bugs-found and
> false-divergences for each. That converts a recall table into "alignment error costs N real bugs and
> buys M false alarms." Until that runs, E2 is a component measurement, not an enabler.

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
| superscript `ᵏ` | **LLM kept names** (disobeyed the rename order) — so name-eq here is the *fair leaf-name* baseline (strip `Type::`), and this is the one regime where a name-matcher competes; the matcher's edge is the renamed regime, not this one. Honest boundary case. |
| superscript `ᶜ` | **signal-C** (type-tag/enum-variant discriminator + input-element-type) is load-bearing here; the cell shows the with-signal-C number (cJSON deterministic: 0.375 baseline →0.475 tag →0.55 +input-type). Both gated + regression-verified across 56 name-preserving libs incl lil (results/rq2_cells/regression/SIGNAL_C.md) |
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
| **cJSON** | JSON parser | 58 | high | 1.00/1.0 | — | — | — | — | ∅ᴴ(partial) | **0.55/0.15**ᴴ˙ᶜ |
| **lil** | interpreter | 145 | **high ↑topo** | 0.97/1.0 | 0.96/1.0 | 0.99/1.0 | 0.92/1.0 | — | — | **0.55/0.55**ᴴ˙ᵏ |
| **lodepng** | PNG codec | 235 | high | 0.99/1.0 | — | — | 0.97/1.0 | — | — | ∅ |
| **bzip2** | compressor | 64 | high | 1.00/1.0 | 1.00/1.0 | 0.98/1.0 | 1.00/1.0 | — | — | **0.63/0.15**ᴴ |
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

**24 cells** *in this batch* (name-preserving tools only — the table's 32 filled cells = these 24 + 7
raw-LLM + 1 PtrTrans; see the status box at the top for the full fill state)
filled with real matcher runs on the E1 artifacts (`results/rq2_cells/name_preserving_v1.json`),
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

**7 cells** on the E1 libraries, hand-labeled truth (`results/rq2_cells/rawllm/`). gpt-5.1, the disclosed
prompt (commands renaming, leaks no correspondence). This is the column where **name-equality collapses**
— usually to 0.0, so it is where the matcher's name-independence is actually *tested* rather than validated.
**Two honest exceptions where the LLM disobeyed and kept names**: cJSON (0.15, kept 6 `parse_*`) and **lil
(0.55, kept the `fnc_*` handler names)** — in those the LLM's disobedience makes name-matching a fair
competitor, which is exactly why the matcher's advantage must be read off the *renamed* cells, not these.

| library | matcher | name-eq | what happened |
|---|--:|--:|---|
| **genann** | **1.00** (8/8) | 0.0 | renamed all + dissolved 4 (read/write=FILE-IO, free=Drop, copy=derive Clone) + free-fn→method; matcher still recovered `genann_init→new`, `genann_randomize→randomize_weights`, etc. |
| **urlparser** | **0.88** (15/17) | 0.0 | misses `url_get_query` (get_query/search homogeneous family) + `strff→skip_forward` (tiny helper) |
| **qsort** | 0.67 (2/3) | 0.0 | LLM added idiomatic wrapper `quick_sort`; matcher picked it over the true recursive core `quick_sort_range`. Tiny programs are adversarial (no structure to grip). |
| **quadtree** | 0.64 (9/14) | 0.0 | `is_empty↔is_pointer` swap (near-identical `&self→bool` predicates), `walk` (fn-ptr→closure reshape) missed. 3 `*_new` constructors EXCLUDED — the analyzer collapses 4 `new` methods to 1 ([[matcher-hir-id-todo]] name-collision). |
| **cJSON** | **0.55** (22/40)ᶜ | 0.15 | The **big-but-flat counterexample**, and the case that motivated **signal-C**. Matcher recovers the topology-rich recursive core (parse_*/print_* mutual recursion) but originally **permuted the 12 `cJSON_Create*` leaf constructors** — all `()→Value`/`(scalar)→Value`, tiny bodies, *call nothing* → zero topology, homogeneous io-shape (**0.40 baseline**). signal-C (type-tag/enum-variant jaccard) recovered **7/12 constructors** → **0.50**; the input-element-type signal then split the int/float/double-array trio signal-C had grouped under one `array` tag → **0.55**. Residual (not pursued): True/Bool (bool-literal asymmetry) + ~8 accessor family (shallow shape + LLM helper attractors). name-eq **≠ 0**: LLM kept 6 `parse_*` verbatim. 18/58 dissolved. |
| **lil** | **0.55** (61/111)ᵏ | 0.55 | The **names-kept boundary case — and the round-2 signal story.** The LLM **disobeyed the rename order and KEPT names** (`fnc_append` verbatim, methodized), so a naive leaf-name matcher scores 0.55 → *not the matcher's regime* (its edge is the renamed regime where name-eq→0). At 0.495 the matcher trailed that baseline; **two literature-standard signals closed the gap to a tie (0.55) without reading names**: re-including **unary negation** in the op histogram (previously a documented "too noisy" skip) resolved the `fnc_inc↔fnc_dec` twin — the discriminator is one `-` present on BOTH sides; and **signal-S string-literal refs** (BinDiff lineage; 19/55 handlers carry a distinctive string like `"global"`, preserved verbatim by behavioral equivalence) added 3 more (W=0.10 mid-band, NOT the 0.08 peak — anti-test-tuning). Family-level **92%** (all 55 `fnc_*` pinned to the handler family, zero leak); `fnc_upeval↔fnc_downeval` remains swapped (C `==`/`!` vs Rust nothing — no honest signal). Graceful degradation: c2rust exact-graph 0.97 → LLM approx-graph 0.55. gross 0.48 (61/128), 17/128 dissolved. Zero regression: 57-cell harness, 3-way comparison. |
| **bzip2** | **0.63** (26/41) | 0.15 | **The clean real-rename cell + a per-file-feeding finding.** Unlike lil/cJSON, the LLM heavily renamed (`BZ2_bsInitWrite`→`init_bit_write`, `sendMTFValues`→`send_mtf_values`) → name-eq 0.15, matcher 0.63 (**4.3× gap**), the raw-LLM column doing its job on a structured compressor. Blind-check passed. **Fed PER-FILE** (5 core `.c` translated as separate units with `bzlib_private.h` as read-only context — clean, no truncation, and file boundaries scope the hand-labeling). 23/64 dissolved = the stdio `FILE` API (`BZ2_bzWrite/Read/open/close`, buffer-only Rust) + allocator + Drop-ends. **Per-file tradeoff finding**: feeding by translation-unit is tractable but *duplicates* cross-file-called functions (3 huffman fns re-translated in 2–3 modules) and *stubs* others (`compress_block_stub`) — ~5 of 15 misses are the matcher picking a VALID non-canonical copy/stub under strict 1:1 truth; duplicate-aware recall ≈ 0.75. gross 0.41 (26/64). |

**Read**: matcher recovers **.55–1.00** under genuine LLM renaming where name-equality gets **0.0–0.15** —
this is the "matcher enables differential testing on renaming translators" claim, on the E1 libraries.
The medium cells (genann 1.00, urlparser 0.88) are strong; the low cells expose **when structure runs
out**: qsort/quadtree are the adversarial small/pointer-heavy cases with homogeneous predicate clusters,
and **cJSON (0.40 → 0.55 with signal-C + input-type) is the big-but-flat counterexample** — it refutes the naive "more
functions ⇒ easier" hypothesis. Recall tracks **topological richness, not function count**: cJSON's recursive parse/print core
matches cleanly, but its wide flat `Create*` constructor API (leaf nodes, identical shapes, no call
topology) permutes. Matches the separate llm-transpiler corpus (.88 micro over 10 programs) on the
structured cases. **Hand-labeling honesty**: `scorable` = C functions with a genuine Rust counterpart;
LLM-dissolved functions (FILE-IO, free/copy → Drop/Clone, C-alloc → enum) are excluded (can't match what
wasn't translated), and dissolution counts are reported per cell, not hidden in recall.

**The cJSON finding sharpens E2's thesis** (worth a sentence in the paper): a structural matcher's ceiling
is set by *how much call-graph topology the program exposes*, not its size. This both motivates the
topology signal (it is what carries the wins) AND scopes its limit (flat leaf constructors need a
non-structural discriminator). An honest low cell is a stronger paper artifact than a suspiciously
uniform column.

**signal-C closes part of that gap (implemented 2026-07-07).** The flat-leaf limit motivated a 4th node
signal: the **type-tag / enum-variant** set each function constructs (C `#define`/enum tag recovered from
the token stream, Rust variant path + bool literals), normalized cross-language (`cJSON_Number` ≡
`JsonValue::Number` → `number`) and blended post-propagation at gated weight 0.35. It lifts cJSON
**0.40→0.50** (7/12 constructors recovered) with a validated **regression firewall**: gated on
both-sides-have-tags so tag-less functions are untouched, verified **zero regression across 55
name-preserving libraries** + clean ablation (`results/rq2_cells/regression/`, `SIGNAL_C.md`). The
residual (True/Bool bool-literal asymmetry; identical-body numeric-array trio) is a *different* lever
(input-element-type weight), honestly out of signal-C's reach. This is the ablation story E2 wants:
shape → +node → +topology → +signal-C, each earning its place, each with a characterized limit.

**hir-id fix applied (2026-07-07)**: the analyzer now keys functions by `Type::method` (impl methods)
/ bare name (free fns) instead of bare-name-with-silent-dedup — see [[matcher-hir-id-todo]], DONE. This
recovered quadtree's 3 dropped constructors: **quadtree raw-LLM 0.64 (9/14, 3 excluded) → 0.71 (12/17,
no exclusions)**, all three `*_new` now distinguished by topology. Re-validated: name-preserving cells
stable (±1 fn from edge re-keying), qsort/genann/urlparser raw-LLM unchanged. The fix caps nothing and
only makes multi-constructor libraries scorable — accuracy preserved.

**Open**: raw-LLM on lil/lodepng/bzip2 (cJSON DONE — 0.50 w/ signal-C, the flat-API counterexample; lil
expected strong given its `↑topo` recursive command-dispatch, lodepng/bzip2 TBD). hir-id name-collision
RESOLVED. **signal-C follow-up**: re-score the other 4 raw-LLM cells (genann/urlparser/qsort/quadtree) with
signal-C once their pre-hir-id truth files are refreshed to `Type::method` names — signal-C is gated (never
regressed any of 55 libs) and ~inert on low-variant libs, but quadtree may move.

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

Ablation ladder — two complementary stress cases, each isolating one signal:
- **lil** (128-fn homogeneous command-dispatch): shape-only **.359** → node-only **.742** →
  **+call-graph topology .984** (+24pp from **topology**). Topology cracks the homogeneous `fnc_*` cluster.
- **cJSON** (58-fn flat constructor API, deterministic numbers): **+topology alone leaves .375** (leaf
  constructors call nothing → no topology to grip) → **+signal-C .475** (**type-tag** groups by JSON kind)
  → **+input-element-type .55** (splits the int/float/double-array trio *within* the tag group; note it does
  **nothing alone** — .375 — it only refines what signal-C already grouped). signal-C cracks what topology
  cannot; input-type refines within it. Regression-verified: zero drop across 55 name-preserving libs
  (`SIGNAL_C.md`). *(Validating signal-C also surfaced + fixed a pre-existing non-determinism: topology's
  `_dir` summed over hash-seed-ordered sets, flipping near-ties in cJSON's homogeneous cluster; `_dir` now
  sorts → cJSON stable 0.55 on every seed.)*

Together they show the structural axes are non-redundant and compose: topology carries connected clusters,
signal-C groups flat leaves by type-tag, input-type splits within a tag group. shape → +node → +topology →
+signal-C → +input-type, each earning its rung on a different case, each with a characterized limit.

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
