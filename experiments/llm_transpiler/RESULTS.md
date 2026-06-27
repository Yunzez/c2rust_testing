# LLM-transpiler rename results (matcher under genuine renaming)

The matcher's headline numbers (98% on lil) were on **faithful c2rust**, where names are
preserved (`#[no_mangle]`) so name-equality IS the ground truth. This track is the first
test where the translator **renames / restructures**: a real `gpt-5-mini` C→Rust
translation, names hidden, scored against a **hand-labeled** correspondence
(`truth/<pair>.json`), since name-equality no longer holds.

## Baseline (gpt-5-mini, 2026-06-27; matcher = test+trait-boilerplate exclusion, partial matching, df-cap 0.5)

10 real LLM C→Rust translations, names hidden, scored against hand-labeled `truth/<p>.json`.
**This table is the frozen baseline — diff future matcher/analyzer changes against it.**

| pair | C fns | accuracy (labeled) | what the LLM did |
|------|------:|--------------------|------------------|
| hex_encode | 2 | **2/2 = 100%** | rename + **folded** `(src,len,dst,cap)→size_t` to `(&[u8],&mut[u8])→usize` |
| rle_codec | 2 | **2/2 = 100%** | rename |
| base64 | 2 | **2/2 = 100%** | **decomposed** 2 C fns into 5 Rust fns (picked the 2 real ones) |
| leb128 | 3 | **3/3 = 100%** | rename (`leb128_*`→`*_uleb128`) |
| rpn_eval | 4 | **4/4 = 100%** | rename (1 Rust-only helper left unmatched) |
| linked_list | 5 | **5/5 = 100%** | rename (`ll_*`→idiomatic) |
| hash_table | 8 | **8/8 = 100%** | rename (`ht_*`→idiomatic; `ht_init`→`new`) |
| opcode_dispatch | 8 | **8/8 = 100%** | rename of a homogeneous `op_*` handler cluster → `handle_*` (+ dispatch table) |
| bignum | 27 | **25/27 = 92%** | `bignum_*`→`bn_*` + semantic renames; residual = `require` (fixed by df-cap) + `to_int`/`to_string` signal-C swap |
| tinyexpr | 28 | **20/28 = 71%** | exploded the math builtin table into ~40 `builtin_*` one-liners (see below) |

**Aggregate: 79/89 = 88.8%; 8 of 10 programs at 100%.** The matcher survives **rename**,
**signature folding**, **decomposition**, and **homogeneous-handler renaming** — none of
which a name-preserving tool (c2rust/CROWN) ever produces.

## Abstention (the STU-aligned goal: don't guess — isolate)

The matcher's value for the STU frontier is **precision, not coverage**: a high-confidence-
but-wrong alignment makes that spot's differential test meaningless. So the matcher emits a
per-pair **two-sided confidence** = min(C-side margin, R-side margin) — how clearly r wins
for c AND c wins for r. `--abstain-eps E` moves pairs below E from `matched` to `ambiguous`
(isolated, flagged for human/dynamic confirmation), rather than guessing. Output categories:
**matched / ambiguous / c_only / rust_only**. Default off (accept all) so baselines/gate are
unchanged.

| seed | forced | accepted-precision @ eps=0.01 | coverage | ambiguous (wrong isolated) |
|------|-------:|------------------------------:|---------:|----------------------------|
| 6 perfect seeds | 100% | 100% | 100% | 0 |
| linked_list | 100% | 100% | 80% | 1 (0) |
| hash_table | 100% | 100% | 87% | 1 (0) |
| bignum | 92% | **100%** | 77% | 6 (2 — incl. the to_int/to_string swap) |
| tinyexpr | 71% | **85%** | 50% | 15 (7 builtin-cluster errors) |
| base64 | 100% | 0% ⚠️ | 0% | 2 (0 — over-abstained, see note) |
| **AGGREGATE** | **79/89 = 88%** | **63/65 = 96%** | **73%** | **25 (9 genuinely-wrong isolated)** |

**Two-sided confidence catches AMBIGUITY (many near-equal candidates: tinyexpr builtins) AND
the non-mutual swaps (bignum to_int/to_string is not each other's clear best → isolated).**
It is a tunable precision/coverage frontier, not a fixed constant — the consumer picks E for
its precision need. Wart: `base64`'s correct matches have very low *absolute* score (0.10,
from decomposition), so a global E over-abstains them — confidence may need per-program
normalization (future). Forced accuracy stays the comparable baseline; abstention is the lens
that makes the matcher honest about what it cannot resolve.

### tinyexpr — the signal-C frontier at scale
tinyexpr's 8 misses are almost all one homogeneous cluster: the LLM turned C's function-
pointer builtin table into ~40 `builtin_*` functions, every one a trivial `()→f64` /
`(f64)→f64` / `(f64,f64)→f64` one-liner. `negate`→`builtin_acos`, `pi`→`peek_char`,
`e`→`builtin_abs`, `npr`→`builtin_ncr`: structurally indistinguishable, so structure
saturates (worse than lil's `fnc_*` because the bodies are trivial too). The parsing/eval
infrastructure (`parse_*`, `eval`, `compile`, `interp`, `find_*`, `next_token`) all matched.
This is the **same signal-C (literals/constants) residual** — `pi` returns 3.14159, `e`
returns 2.718, `fac` loops — now demonstrated at scale on real LLM output.

## The bignum story (an adversarial case) — what the diagnostics revealed

bignum is hard because the LLM injected Rust nodes with **no C counterpart** that act as
**topological hubs** and poison similarity propagation. The `--diag` output is the evidence:

```
hubs (in-degree df)  Rust: [(25,'require'), (9,'default'), (4,'bn_assign')]
true-match topo_delta (sim_topo - sim_node), all NEGATIVE:
  bignum_init  -0.178   bignum_to_int -0.317   bignum_assign -0.151
```
`require` (an assert helper, called by 25 fns) and `default` (`impl Default`) attract the
propagation, pushing every true match down in rank. This is **hub poisoning**, and it
settles a design debate empirically: keeping an extracted helper (`require`) in the
topology graph is *harmful* when it is a hub.

### What each lever buys (bignum, ceiling = 92%; last 2 are signal-failure)

| stage | mechanism | catches | bignum |
|-------|-----------|---------|--------|
| baseline (forced Hungarian) | — | — | 23/27 = 85% |
| **v1** | `#[cfg(test)]`/`#[test]` exclusion + partial (dummy) matching | tests | 23/27 = 85% |
| **+ trait-boilerplate exclusion** | `impl Default/Clone/Debug/...` out of candidate+graph | `default` | 24/27 = 88% ✅ done |
| **+ df-cap on topology** | drop >50%-of-program hub "stopword" neighbors | `require` (df 0.89) | **25/27 = 92%** ✅ done |
| + signal C (deferred) | literal/constant features | `to_int`↔`to_string` swap | 27/27 |

**df-cap validated against the lil gate.** The fear was that df-cap would weaken the
homogeneous-cluster topology lil depends on. It does not: degree cleanly separates the
fake hub (`require`, called by 0.89 of bignum) from real shared primitives
(`lil_to_string` 0.33, `lil_free_value` 0.19 — lil's busiest). A cap anywhere in
[0.4, 0.7] gives bignum 25/27 AND keeps lil at 126/128 (98%); 0.3 starts dropping real
functions (`bn_isqrt`). Chosen cap = 0.5 ("called by >half the program ⇒ no signal"),
mid-band. Gate: `bash scripts/matcher_regression.sh`.

Why these mechanisms and not a score threshold: real matches score ~0.10 while the
pollutant `init→default` scored 0.43 — **absolute score cannot separate them**, so τ is a
low outside-option floor (0.05), not a quality gate. And margins are ~0.00 even on correct
matches (homogeneous clusters), so margin can flag *ambiguous* but cannot *reject*.

### Residual signal-failure (the deferred signal-C case)
bignum's last 2 misses are a **swap**: `bignum_to_int`↔`bignum_to_string`. The LLM rewrote
`to_string` from C's 3-param buffer-fill `(bn*,char*,int)→void` into 1-param `(&BigNum)→String`,
collapsing the arity/shape difference. True match sits at **rank 25 even in node_sim** — no
assignment/topology trick recovers it; needs **signal C (literals)**: `to_string` carries the
`"0123456789abcdef"` table + shift-by-4, `to_int` does not. Same class as `fnc_streq`↔`fnc_strcmp`.

## Roadmap
- **done:** matcher v1 (test exclusion, partial/dummy matching, rank+hub diagnostics),
  trait-boilerplate exclusion, df-cap hub-stopword removal, and **abstention** (two-sided
  confidence → matched/ambiguous) — all gated by the lil regression fixture
  (`scripts/matcher_regression.sh`, 126/128). Zero risk to faithful c2rust.
- **the goal is precision + isolation, not 100% accuracy.** The matcher identifies
  high-confidence cross-language alignments (96% accepted-precision) and isolates
  structurally-indistinguishable regions, instead of guessing.
- **optional refinement (NOT mainline, NOT for chasing 100%):** signal C (literals/constants)
  — would *shrink* the ambiguous clusters (tinyexpr builtins, `streq`/`strcmp`, `to_int`/
  `to_string`) by adding the distinguishing constant. Enable only when higher coverage is
  needed; otherwise these stay correctly isolated as ambiguous.
- **possible refinement:** per-program confidence normalization (base64 over-abstention wart).
- **separate latent fix:** hir FunctionId as node identity (name-collision dedup bug).

## How to reproduce
```
OPENAI_API_KEY=$(cat key.env) python3 transpile.py --pair <p> --real
bash run_pipeline.sh <p>            # auto-uses truth/<p>.json if present
matcher.py --c c.json --rust r.json --truth t.json --diag   # rank + hub diagnostics
```
Truth files hand-labeled in `truth/`. `out/` is gitignored (regenerate via the two commands).
