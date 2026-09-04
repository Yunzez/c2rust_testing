# RQ1b / Mutation-Recall — evaluation plan (draft for review 2026-07-02)

The recall half of the comparator claim (RQ2 = precision / no false positives; this = sensitivity / we
still catch real bugs). Mirrors `results/archive/rq2_eval_plan.md`. Based on Codex's design + refinements below.

## RQ1b (tightened)
> Can the comparator **detect injected UB-free semantic bugs** while preserving the precision gains from
> RQ2/RQ5? I.e. when a real (UB-free) translation bug exists, does the differential oracle still find it —
> and does the UB gate / frontier NOT kill that recall?

Forms a closed loop with RQ2: RQ2 shows the gate/frontier suppress UB false positives; RQ1b shows they do
so **without** hiding real UB-free bugs. Keep it a SEPARATE experiment, not an RQ1 anecdote.

## What Codex got right (adopt)
- **Separate RQ, closed-loop framing** with RQ2. ✓
- **Claim = frontier/UB-gate PRESERVE recall, NOT improve it.** Do NOT promise frontier recall > all.
  Wording: "frontier detects X/Y valid injected bugs while avoiding the UB false positives of RQ2/RQ5." ✓
- **Only UB-free semantic mutations. NO pointer/deref/memory mutations** — those introduce UB and would be
  (correctly) gated, muddying the recall metric. Keep the injected bugs UB-free arithmetic/logic. ✓ (key)
- **Every DETECTED bug needs UB-free artifact evidence** (replay + C sanitizer confirms the triggering
  input is UB-free) — reuse the RQ2 replay/evidence machinery. Invalid/UB-only mutations excluded from the
  recall denominator. ✓
- **Classify NOT-detected cases** (not-reached-by-frontier / equivalent-no-observable-diff / fuzzer-timeout
  / invalid-UB-only) — else missed cases are unexplained. ✓
- **4 operator classes** (operator replacement / off-by-one / constant perturbation / guard weakening),
  bounded. Static metadata per injection. ✓
- **Hand-written fixtures OK if auto-patch is fiddly** — results first, fancy mutator later. ✓
- **RQ4 after** (higher engineering risk: competitor envs, benchmark alignment, result reproduction). ✓

## My refinements (the 3-4 that matter)
1. **Base = faithful c2rust (name-preserving) translations, NOT the LLM ones.** c2rust is known-correct, so
   an injected divergence is PURELY the mutation (clean recall measurement). It also DECOUPLES RQ1b from the
   matcher: RQ1b tests the differential ORACLE's recall (matcher = RQ3). Pair by name; no matcher noise.
   (Optionally a small end-to-end slice on renamed+matched pairs to show the whole pipeline, but the clean
   recall number comes from the c2rust base.)
2. **Nail the DENOMINATOR — the equivalent-mutant problem.** recall = detected / **valid-non-equivalent**
   mutations. A mutation is a valid recall target only if SOME UB-free input makes C and mutated-Rust
   differ. Establish this INDEPENDENTLY of our own fuzzer (else circular): a long-budget reference oracle
   (or exhaustive enumeration for small scalar domains) decides `valid` vs `equivalent`. Report equivalents
   separately; they are NOT missed bugs. This is the one methodological point that can sink the recall %.
3. **Reuse `eval_rq2_ubgate.py` wholesale.** "DETECTED_UB_FREE" == the RQ2 classifier's `UB_FREE_DIVERGENCE`
   class (fuzz the mutated pair, replay each artifact, keep only UB-free-input divergences). The mutation
   runner = RQ2 pipeline with (a) a mutated Rust translation and (b) success = a UB_FREE_DIVERGENCE appears.
   Same replay+evidence, same hard-trap/memory exclusion.
4. **Injection mechanics:** hand-pick the site + a scripted textual patch (semi-auto) for M1; decide on a
   real AST mutator only if scaling past ~30 needs it. Each patch must still COMPILE (a non-compiling mutant
   = BUILD_FAIL, excluded, not a miss).

## Unit
`(program, boundary, mutation operator)`. Mutate the Rust translation body; C original stays the oracle.
Count only UB-free divergences.

## Operators (4 classes, UB-free only)
1. operator replacement: `+↔-`, `*↔+`, `/↔%`, `<↔<=`, `==↔!=`
2. off-by-one / boundary: `i<n → i<=n`, `i+1 → i`, cap `< → <=`
3. constant perturbation: `0→1`, `1→0`, `7→8`, `0xff→0x7f`
4. guard weakening / deletion: drop or invert `if cap<required {return 0}` / `if b==0 {...}`
NO pointer/deref/memory mutations (would be UB, not a UB-free semantic bug).

## Data (cover bug types, not huge N)
From boundaries that already auto-bridge + fuzz cleanly (the RQ2 TN set is ideal — known clean, so a post-
injection divergence is the mutation): hex_encode, leb128 (encode/decode/roundtrip), rle_codec,
intmath/bitutils (the non-UB ones: gcd_u64, popcount32, reverse32, isqrt), byte_classify, reduce_overflow_safe,
negate_abs_safe, div_mod_safe, safe_stats (guarded), g3 cases. First cut: **~10 programs × 3 mutations = 30
injected bugs**, ≥4 operator classes represented.

## Runner — `scripts/eval_mutation_recall.py`
Per (program, boundary, operator): copy pair to temp → apply Rust-side patch → confirm it compiles →
run the RQ2 pipeline (`--ub-free`, with strategy ∈ {all, frontier}) → classify:
- `DETECTED_UB_FREE` — a UB_FREE_DIVERGENCE artifact (evidence: replay shows triggering input UB-free)
- `NOT_DETECTED` — no divergence within budget (then sub-classify vs the independent oracle: equivalent /
  not-reached-by-frontier / timeout)
- `BUILD_FAIL` — mutant didn't compile (excluded)
- `UB_ONLY` / `INVALID_MUTATION` — the only divergence is on a UB input, or the operator produced UB
  (excluded from recall denominator)
Static metadata per mutation: `{program, function, operator, patch, expected}`.

## Tables
**Main — recall by operator:**
| operator | injected | valid (non-equiv, UB-free) | detected | recall | median time-to-detect |
**Strategy ablation (recall preservation, NOT superiority):**
| strategy | detected/valid | UB false positives | note |
| all | … | (from RQ2) | baseline recall |
| frontier | … | fewer | frontier preserves recall while cutting FPs |
| frontier + UB-gate | … | ~0 | closed loop with RQ2 |
**Missed-case taxonomy:** not-reached-by-frontier / equivalent / fuzzer-timeout / invalid-UB-only (counts).

## DoD
1. ≥25 valid injected UB-free semantic bugs; ≥4 operator classes.
2. every DETECTED bug carries a UB-free artifact (triggering input sanitizer-confirmed UB-free).
3. time-to-detect reported (fixed seed/budget).
4. every NOT-detected mutation classified (no unexplained misses).
5. denominator = valid-non-equivalent mutations, equivalence decided by an INDEPENDENT oracle.
6. `results/ablations/attribution/mutation_recall_v1.md`.

## Phases
- **M1**: mutation schema + metadata format + 5 smoke mutations end-to-end (prove inject→compile→fuzz→
  DETECTED_UB_FREE with evidence, on e.g. leb128/hex_encode). Reuse eval_rq2_ubgate.
- **M2**: expand to ~30 valid injected bugs across 4 operators / ~10 programs; independent equivalence check.
- **M3**: `results/ablations/attribution/mutation_recall_v1.md` (tables + missed-case taxonomy) + paper table for RQ1b.

## The one risk to watch
Equivalent mutants inflating "NOT_DETECTED". Mitigate with the independent oracle (#2). If a mutation shows
no UB-free divergence under a long independent budget, it is EQUIVALENT (not a miss) — report it as such.
