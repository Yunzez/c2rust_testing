# signal-C — type-tag / enum-variant discriminator (2026-07-07)

## What it is
A 4th name-independent node signal for the matcher, added to break the **flat-leaf-constructor**
failure mode surfaced by cJSON raw-LLM (matcher permuted the 12 `cJSON_Create*` constructors: identical
io-shape `()->Value`/`(scalar)->Value`, tiny bodies, **call nothing** => zero topology to grip).

Each function gets a `consts` set = the TYPE-TAG / enum-variant tokens it constructs, normalized to the
last `_`/`::` segment lowercased so it is **cross-language comparable**:
- C: `item->type = cJSON_Number` (a `#define`, recovered from the pre-expansion **token stream** since the
  AST only sees the expanded integer) -> `number`; enum-constant DECL_REFs handled too.
- Rust: `JsonValue::Number` variant path + bool literals -> `number` / `true` / `false`.

So C `cJSON_Number` and Rust `JsonValue::Number` both collapse to `number` and jaccard-match.

## Where it lives (3 files)
- `tools/stu_selector/c_analyzer.py` — `consts_of()` (token-stream macro + enum-const extraction), emits `consts`.
- `tools/stu_selector/analyzer/src/consts.rs` — `consts_of()` (variant paths + bool literals), wired into
  `FnRec` in `lib.rs`. Analyzer rebuilt (`cargo build --release`).
- `tools/stu_selector/matcher.py` — `const_sim()` (gated jaccard) + `apply_consts()` blended into the
  **post-propagation** similarity at weight `_CONST_W=0.20`, so the discriminator votes at full assignment
  weight (in `node_sim` it was diluted to 1-alpha and drowned by the uniform ~1.0 topology of leaf pairs).
  `--no-consts` CLI flag + `USE_CONSTS` global = ablation control. **Weight is 0.20, NOT 0.35**: at 0.35
  cJSON also reached 0.55 but **lil REGRESSED 0.984→0.969** (signal-C overrode lil's confident
  interpreter-variant matches). 0.20 is the lowest weight giving cJSON full gain with lil intact.

## Regression firewall (the user's hard constraint: don't drop other libraries)
- **Gated**: `const_sim` returns None unless BOTH sides expose tags => tag-less logic functions keep their
  EXACT pre-signal-C score. A library of plain functions cannot regress.
- **Verified** by `harness.py` over **57** reproducible cells (56 name-preserving benchmark/pairs incl.
  lil + cjson), signals OFF (`SIGNALS_OFF=1`) vs ON, both `PYTHONHASHSEED=0` deterministic,
  `final_off.json` vs `final_on.json` via `compare.py`:
  ```
  unchanged=56   improved=1 (cjson_rawllm 0.375->0.55)   REGRESSED=0   lil 0.9844->0.9844
  ```
- **The harness nearly had a hole — and closing it caught a real regression.** lil (128-fn, the biggest
  name-preserving cell) was being **silently dropped**: its crate keeps code in `src/lil.rs`, not
  `src/lib.rs`, so the wrap-into-crate heuristic grabbed only a 1-fn stub → empty truth → skipped. Fixed
  the harness (detect a real crate by `Cargo.toml`). With lil back in the set, W=0.35 signal-C **regressed
  lil 0.984→0.969** — it overrode lil's confident interpreter-variant matches. Dropping to **W=0.20** keeps
  cJSON's full 0.55 and lil (all 56) intact. Lesson: a silently-incomplete guard is worse than none;
  always assert the cell COUNT, not just "no regressions in what ran."
- **Weight sweep** (lil must stay 0.984 / cjson want high): W=0.20→(0.984/0.55), 0.25→(0.984/0.55),
  0.35→(**0.969**/0.55). 0.20-0.25 is the safe band; 0.35 breaks lil. Shipped 0.20 (max margin).

## Companion signal: input-element-type (added same day)
A 5th term, `input_sim` = jaccard of the **input** scalar/element types only (return type excluded, since
the C `cJSON*` return expands to a huge shared struct that drowns the element type). `&[i32]`->{i32},
`const double*`->{f64}. Weight 0.12, gated on both-sides-have-scalar-inputs, blended in the same
post-propagation pass. Purpose: split the int/float/double/string-array constructor trio that share
everything except element type.

**Composition (not independence)** — the ablation matrix on cJSON (deterministic, shipped weights
_CONST_W=0.20 / _INPUT_W=0.12):
```
baseline (both off)   15/40 = 0.375
+ signal-C only       19/40 = 0.475
+ input-scalar only   15/40 = 0.375   <- ZERO effect alone
+ BOTH (shipped)      22/40 = 0.55
```
input-scalar does nothing by itself; it only refines WITHIN the type-tag groups signal-C forms (tag groups
all four arrays under `array`, then element-type splits int vs double). A clean "signals compose" story.
Both verified in the final run (`final_off.json` vs `final_on.json`, 57 cells incl lil): unchanged=56,
improved=1 (cjson 0.375->0.55), REGRESSED=0.

## Determinism fix (found while validating signal-C — a pre-existing bug)
The matcher was **non-deterministic**: `matcher._dir` (topology neighbor-set best-match) summed `max`
values over Python **sets**, whose iteration order varies with `PYTHONHASHSEED`; float addition is
non-associative, so last-bit differences flipped **near-ties** in homogeneous clusters. cJSON recall
wandered **0.375–0.55 across runs/seeds**. FIX: `_dir` now iterates neighbor sets **sorted** → fully
deterministic (cJSON = 0.55 on every seed). This was latent before signal-C (cJSON's flat cluster is what
surfaced it); the 53 clean-1.0 libs never had ties so were unaffected. Regression runs pin
`PYTHONHASHSEED=0` for belt-and-suspenders.

## Result
cJSON raw-LLM **0.375 -> 0.55** (deterministic baseline → signal-C 0.475 → +input-type 0.55; the old
"0.40" baseline was a lucky seed). Residual **not pursued** (diminishing
returns, rising overfit risk — user call): CreateTrue/Bool (bool-literal asymmetry) + ~8 accessor family
(Get/Add/Detach: shallow `(container,key)->item` shape + LLM-added helper attractors). 50-55% is the
structural ceiling here; the rest characterizes where structure runs out.

## Why we STOP at ~0.55 (the argument — do not chase higher)
E2's thesis is **not** "the matcher is near-perfect"; it is "**name-independent** matching enables the E1
differential on renaming translators." That claim is carried by the **gap** between matcher recall and
name-equality recall on renamed code — cJSON 0.55 vs 0.15 (**3.7×**), large and robust. Pushing 0.55→0.8
does not strengthen the claim. Concretely:

1. **The residual is characterized, not mysterious.** What's left = CreateTrue/Bool (bool-literal
   cross-language asymmetry) + ~8 accessor family (shallow `(container,key)->item` shape + LLM-added helper
   attractors the C lacked). We know what and why. A **bounded 0.55 is a stronger artifact than an
   uncharacterized 0.8.**
2. **Higher = overfitting, and we have the proof.** W=0.35 squeezed nothing extra from cJSON but immediately
   **regressed lil 0.984→0.969**. That is the overfitting boundary made concrete: marginal gain on the
   target traded against generality on the corpus. Hitting that trade-off IS the stop signal.
3. **cJSON is the adversarial FLOOR, not the typical case** (big + flat + LLM commanded to rename fully).
   Name-preserving cells are 0.92-1.0; other raw-LLM cells 0.64-1.0. Reporting the honest floor + its cause
   beats inflating it — reviewers trust a paper that shows its worst case.
4. **The signals we added earn their place by a clean ablation** (shape→node→topology→signal-C→input-type,
   each cracking a *different* failure mode, each regression-verified). A 6th signal for the accessor family
   would be a special-case hack with no such narrative.
5. **Cost/value.** The ~8 accessors each need bespoke handling (false-attractor rejection, arg-role
   modeling) with rising regression risk and diminishing returns, for a cell whose point (the gap) is
   already made. Not worth it.

**In one line:** the thesis is proven by the matcher/name-eq gap, which is large and robust; the residual is
a characterized structural ceiling; the lil regression showed pushing past it trades generality for one
adversarial cell — so we stop at the honest, regression-safe, characterized number.

## Known follow-up (NOT done — column consistency)
The other 4 raw-LLM cells (genann/urlparser/qsort/quadtree) were scored pre-signal-C and their stored
`truth.json` files predate the hir-id fix (bare Rust names, not `Type::method`), so they cannot be
re-scored cleanly without re-labeling. signal-C is gated (never hurt any of the 55 libs) and is ~inert on
low-variant libs (genann/qsort/urlparser); quadtree (has constructors) may move. Re-score with refreshed
truth in a future pass before quoting a signal-C-uniform column mean.

## Reproduce
```
PYTHONHASHSEED=0 SIGNALS_OFF=1 python3 results/rq1_matching/cells/regression/harness.py > /tmp/off.json   # signals off
PYTHONHASHSEED=0                python3 results/rq1_matching/cells/regression/harness.py > /tmp/on.json    # shipped
python3 results/rq1_matching/cells/regression/compare.py /tmp/off.json /tmp/on.json
```

## Round 2 (2026-07-07 evening): signal audit → neg + signal-S, validated the same way

A reviewer-defense audit of the signal space ("which signals exist that you don't use, and why?")
found two gaps with **in-corpus evidence**, both fixed and validated:

1. **Unary negation re-included in the base op histogram.** `ops.rs` had a documented skip
   ("neg — too noisy cross-language") — but probing showed neg is the EXACT discriminator of the
   lil `fnc_inc`/`fnc_dec` twin (`-(...)` in C, `-amount` in Rust, both preserved). Re-included on
   both sides. Effect: resolves the twin; **zero change on all 57 harness cells** (the "noisy"
   worry was empirically unfounded).
2. **signal-S: string-literal references** (BinDiff-lineage; the standard signal we lacked).
   User-facing strings survive translation verbatim (behavioral equivalence forces it): 19/55 lil
   handlers carry a distinctive string (`"global"`, `"clean"`, `"unknown function '%s'"`).
   Extracted both sides (source-text form so `"\n"`≡`"\n"`), gated jaccard, post-propagation
   **W=0.10 — the MID-BAND, deliberately NOT the 0.08 peak** (band 0.05–0.12 all gain; taking the
   peak would be tuning on the test cell).

**Result: lil raw-LLM 0.495 → 0.550 (61/111)** — the name-independent matcher now TIES the
leaf-name baseline (0.55) on the very cell where the LLM kept names, without reading a name.
fnc cluster 23→28/55, family-level 92%. cJSON unchanged (0.55). `fnc_upeval↔fnc_downeval` stays
swapped — C-side discriminator `==` vs `!`, Rust-side nothing: no honest signal exists; documented.

**Overfit guard (3-way, all zero-regression):** v3_off vs v3_on (gated signals), final_off vs
v3_off (neg alone — 57/57 unchanged), final_on vs v3_on (old shipped vs new shipped — 57/57
unchanged). The lil_rawllm cell is now IN the harness (58 cells forward).

**Reviewer-ready signal inventory** (use verbatim if asked "why not signal X?"):
- USED (9): io-shape soft+exact, arity, metrics vector, op histogram (incl. neg), local call-graph
  topology (IsoRank, df-cap), type-tag consts, input-element scalars, string-literal refs.
- EXCLUDED with reasons: external/std-call fingerprints (needs curated cross-language std mapping —
  future work, BinDiff-imports precedent); source/declaration order (positional not semantic, zero
  robustness to reordering, would smuggle the answer into twin cases); CFG-shape hashes (reshaping
  translators destroy CFG isomorphism; metrics vector is the coarse proxy); field-access sets
  (field names rename too; shape covers struct types); identifiers/comments (the thesis).
- TESTED-AND-BOUNDED: every adopted signal carries a weight-band + 57-cell zero-regression run;
  every excluded signal has either a principled reason or an empirical probe.
