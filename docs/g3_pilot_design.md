# G3 pilot design — the false-divergence oracle (Layer 2, Step 4)

> Purpose: turn the frontier's *definitional* "0 risk-exposure" into an *empirical* result. G3 perturbs
> / constructs code that is **known semantics-preserving** (so any C-vs-Rust divergence is FALSE by
> construction), then shows that fuzzing at the WRONG boundary manufactures false divergences while the
> STU frontier (the right boundary) stays clean. Pilot first: nail the oracle, implement **Case A only**,
> run the 4 strategies, and STOP if Case A does not reproduce the expected pattern. Created 2026-06-25.

## 1. The false-divergence oracle (operational, fixed before any run)

Ground truth must **not** rest on "trust the translator" (we are trying to *find* mistranslations, and
LLM transpilers are not faithful — c2rust itself can mistranslate, design §7). G3 instead establishes
ground truth in two **translator-independent** ways:

- **Path 1 — constructed equivalence.** Take one program and apply a change we *know* preserves semantics
  (e.g. extract a chunk into a helper — pure refactoring). The two versions are equivalent because *we*
  made the change, not because we trust a translator → any divergence between them is FALSE by construction.
- **Path 2 — reachability.** A divergence on an input the boundary's *real callers never produce* is not
  program-relevant: it is dead behavior. `scale` overflows only for large `x`, but `scale` is only ever
  called via `scale_pct` (which clamps to [0,100]), so large `x` is unreachable in the real program.
  Whether or not `scale`'s translation is faithful on large `x` is irrelevant — the program never goes there.

Neither path requires trusting the C↔Rust translation. Given that, a reported mismatch is a
**FALSE divergence** iff ALL hold:

> A reported C-vs-Rust mismatch is a **FALSE divergence** iff ALL hold:
> 1. the harness reports a behavioral mismatch (different return / different crash / one-sided panic), AND
> 2. the program/perturbation is semantics-preserving (by construction — we author it, no bug injected), AND
> 3. the mismatch is triggered by an input the boundary's *real callers never produce* (a precondition
>    violation) OR by structural misalignment of the chosen boundary — NOT by a faithful behavioral
>    difference on a legal input, AND
> 4. the mismatch **disappears when the same logic is tested at the boundary that establishes the
>    precondition** (the higher/API boundary).

Condition 4 is the crux and the measurable signal: *same code, same fuzzer, different boundary → divergence
vanishes.* If a divergence does NOT vanish at the higher boundary, it is NOT a false divergence (it would
indicate a real difference, which G3 is constructed not to contain — so that case means our setup is wrong).

**Metric per strategy:** false-divergence count = # selected boundaries that produce a divergence under
fuzzing (all are false by construction). Lower is better; the API/STU boundary should reach 0.

## 2. Three micro cases (Case A implemented first)

Each is a tiny authored C program (authoring is legitimate here — it is a *controlled* semantics-preserving
construction, not corpus evidence) transpiled with c2rust so the translation is faithful by construction.

### Case A — caller-established precondition (input range) [IMPLEMENT FIRST]
- `helper`: `int scale(int x) { return x * 100; }` — signed overflow for large `x` (precondition: callers
  pass a small, in-range `x`).
- `api`: `int scale_pct(int pct){ if(pct<0)pct=0; if(pct>100)pct=100; return scale(pct); }` — clamps the
  input so `scale` never overflows.
- Mechanism: intrinsic-UB *reachable only under a precondition the caller establishes*.
- Why it diverges at helper level: our fuzz build uses Rust `-Cdebug-assertions`, so `x*100` **panics** on
  overflow in Rust while C wraps / UBSan-flags → one-sided panic = divergence. Legal inputs (|x|≤~21M) agree.

### Case B — structural misalignment (helper extraction)
- A function whose body c2rust (or a perturbation) splits into an extracted helper that has no standalone
  faithful C counterpart at that boundary (or a shifted ABI). Fuzzing the extracted helper alone is an
  invalid harness; the API boundary is clean. (Tests the pure *structure* axis, no UB.)

### Case C — isolation invariant (struct cursor)
- `helper`: reads `s->buf[s->head]` assuming `head` in-bounds; `api`: initializes the struct so `head` is
  valid. Standalone struct construction violates the invariant. (Tests the isolation mechanism directly.)

## 3. Baseline failure mode (what we expect to SEE)

| strategy | boundary it fuzzes (Case A) | expected |
|---|---|---|
| leaf-only / all-constructible | `scale` (the helper) | **FALSE divergence** (overflow panic vs wrap) — invalid harness |
| root / public | `scale_pct` (the api) | **clean** (precondition clamps input) — but coarse in deep programs |
| **STU frontier** | *should be* `scale_pct` | **clean + covers `scale`'s logic via the api** |

## 4. STU expected behavior — and the v1 directionality test

The frontier should **rise to `scale_pct`**: the boundary that *establishes the precondition* under which
`scale` is safe. Testing `scale` standalone (sinking) violates the precondition and manufactures the false
divergence.

> ⚠️ **Selector v1 does the OPPOSITE.** Its rule propagates RISKY upward and *sinks below* it, so on Case A
> it will refuse `scale_pct` (reaches RISKY `scale`) and cannot pick `scale` (itself RISKY) → it collapses
> to **0/0** (tests nothing). Case A is therefore also a test of the selector's *direction*: we expect it to
> reveal that v1's "sink below RISKY" is incomplete, and that the frontier needs a **"rise to the boundary
> that constrains the risky callee's input domain"** rule (selector v2). `root` accidentally gets Case A
> right because the api IS the root in a 2-level program; in a DEEP program the correct boundary is a
> *mid-level* api that neither `root` (too coarse) nor v1-`frontier` (sinks) selects — that is the real
> motivation v2 must earn.

## 5. Pilot success criterion (gate before scaling)

Case A is a success iff, with the faithful c2rust translation:
- fuzzing `scale` (helper) reproduces a **false divergence** (Rust overflow panic vs C wrap), AND
- fuzzing `scale_pct` (api) is **clean** over the same budget.

If both hold, the false-divergence phenomenon is real and boundary-dependent → proceed to wire it into the
strategy comparison and (separately) decide whether selector v2 needs the "rise" rule. **If Case A does not
reproduce, STOP and rethink the oracle/setup — do not implement Cases B/C or scale.**

## 6a. Pilot result — Case A ✅ REPRODUCED (2026-06-25)

`benchmark/pairs/g3_case_a` (authored `scale`/`scale_pct`, transpiled with c2rust). Differential fuzz:

| boundary | outcome | reading |
|---|---|---|
| `scale` (helper) | **C_UB_CONFIRMED → invalid** (overflow hit at exec 1) | FALSE divergence — overflow is on an input unreachable in the real program |
| `scale_pct` (api) | **NO_DIVERGENCE → valid** (1.09M execs clean) | the clamp neutralizes the risk; same code, higher boundary, divergence vanishes |

The gate is met: *same code, different boundary, divergence appears then vanishes* — the false-divergence
phenomenon is real and boundary-dependent, established without trusting the translation (Path 2 reachability).

**Selector finding (the directionality + static-coarseness gaps, now empirical):**
- Selector **v1 collapses to 0/0** on Case A (sinks below RISKY `scale`, cannot pick RISKY `scale`, never
  rises to `scale_pct`). The correct boundary is `scale_pct`; only `root` picks it, and only because
  api==root in a 2-level program. In a deep program the right boundary is a mid-level api that neither
  `root` (too coarse) nor v1-`frontier` (sinks) selects → this is what selector **v2's "rise" rule must earn**.
- The static **`reaches RISKY` exposure metric flags `scale_pct` as exposed (1)** even though it is
  empirically clean — because static reachability cannot see that `scale_pct` *constrains* `scale`'s input
  domain. So v2 needs not just "rise" but "rise to the boundary that **constrains the risky callee's input
  domain**"; detecting that statically (interprocedural range/precondition) is the open research core. G3
  measures it empirically; the selector must learn to predict it.

→ Per the gate, Case A reproduced, so we MAY proceed to Cases B/C + wire into the strategy comparison. Next
decision is the selector v2 "rise + constraint-aware" rule (separate from continuing G3).

## 6. Two assumptions — flagged, not hardcoded

We currently lean on two translator properties that hold for c2rust but break for an LLM transpiler. Keep
them as explicit config, defaulting to the c2rust values, never baked into the algorithm:

- **`name_preserving_mapping`** (default true) — whether `align()` may match C↔Rust functions by name
  (c2rust `#[no_mangle]` makes it free; coverage 1.00 on our corpus). The matching is already isolated to
  `mapping.align()`; consumers use only its OUTPUT (a `matched` set / a `mapped(node)` predicate), never
  re-deriving by name. false → swap `align()`'s internals for a semantic/structural mapping provider
  (design §3 region alignment; N:M possible), same interface. **A boundary is usable only if it is mappable
  on both sides** (condition 1), so when this is false the mappable-boundary set shrinks and becomes the
  bottleneck. Differential testing only needs the ENTRY to correspond (black-box I/O compare), not internal
  alignment — so the task is "find the Rust entry with the same observable contract," not full alignment.
- **`translation_trusted`** (default false — do NOT rely on the shortcut even for c2rust) — whether a
  divergence may be auto-labeled false because "the translation is faithful." We deliberately avoid this
  shortcut; G3 uses Path 1 + Path 2 instead. When trust is absent (the real LLM case), a divergence may be
  real or false and cannot be auto-labeled. The selector then becomes a **triage / prioritizer**, NOT an
  oracle: a divergence at a predicted-VALID boundary is a *candidate real bug* (send to human review); one
  at a predicted-INVALID boundary is *likely false* (suppress/deprioritize). This — telling you *which
  divergences are worth investigating* — is the deployment value proposition when translation is untrusted.
