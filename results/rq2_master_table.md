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
| `⊘` | excluded. NOTE: E1's UB-gate does **not** apply to E2 — matching is static, it never executes the C — so E1's `⊘(C-side UB)` cells (urlparser) ARE matchable here. `⊘` in E2 means a *matching-specific* block only. |

Method for every filled cell: run both analyzers (C via `c_analyzer.py`, Rust via `analyzer/`), feed
`matcher.match()` (io-shape + metrics + operator histogram + call-graph topology; NO names), score
`precision / recall / coverage` micro+macro against the truth file. Runner: `scripts/eval_rq2_matcher.py`
(currently `eval_rq3_matcher.py` — legacy name). Matcher: `tools/stu_selector/matcher.py`.

## The table

| library | domain | ~#fn | homog.¹ | c2rust (mech.) | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans | **raw-LLM**² |
|---|---|---:|---:|---|---|---|---|---|---|---|
| **qsort** | sorting | 3 | low | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴴ | ∅ᴴ | ∅ |
| **urlparser** | URL parsing | ~15 | low | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴺ | — | — | ∅ |
| **quadtree** | spatial tree | ~25 | med | ∅ᴺ | — | — | ∅ᴺ | — | ∅ᴴ | ∅ |
| **genann** | neural net | ~20 | med | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴴ | ∅ᴴ(decl) | ∅ |
| **cJSON** | JSON parser | 118 | high | ∅ᴺ | — | — | — | — | ∅ᴴ(partial) | ∅ |
| **lil** | interpreter | ~128 | **high ↑topo** | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴺ | — | — | ∅ |
| **lodepng** | PNG codec | ~200 | high | ∅ᴺ | — | — | ∅ᴺ | — | — | ∅ |
| **bzip2** | compressor | ~110 | high | ∅ᴺ | ∅ᴺ | ∅ᴺ | ∅ᴺ | — | — | ∅ |
| **tulipindicators** | indicators | ~100 | **very high** | ∅ᴺ | ∅ᴺ | ∅ᴺ | ▽ᴺ | — | — | ∅ |
| **optipng** (incl. zlib) | PNG optimizer | ~400 | high | ∅ᴺ | ∅ᴺ | ∅ᴺ | — | — | — | ∅ |

¹ **homog.** = homogeneity of the io-shape distribution — how many functions share the dominant
signature. HIGH homogeneity is what defeats per-function matching and forces call-graph topology
(lil: 55/128 functions share one shape → the `↑topo` stress case). This is E2's analogue of E1's
"scale" column: the axis that actually makes the problem hard.
² **raw-LLM** = our own gpt-5.1 translation (prompt below) — the *maximal-rename* reference column.
Not one of the 6 shipped tools; included because it is the hardest case for name-equality and the
clearest demonstration of the matcher's value. All ∅ pending generation on the E1 libraries.

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
