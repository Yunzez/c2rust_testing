# G3 pilot design — the false-divergence oracle (Layer 2, Step 4)

> Purpose: turn the frontier's *definitional* "0 risk-exposure" into an *empirical* result. G3 perturbs
> / constructs code that is **known semantics-preserving** (so any C-vs-Rust divergence is FALSE by
> construction), then shows that fuzzing at the WRONG boundary manufactures false divergences while the
> STU frontier (the right boundary) stays clean. Pilot first: nail the oracle, implement **Case A only**,
> run the 4 strategies, and STOP if Case A does not reproduce the expected pattern. Created 2026-06-25.

## 1. The false-divergence oracle (operational, fixed before any run)

Ground truth is established by CONSTRUCTION: in G3 the C and its Rust are a faithful c2rust translation
(or a semantics-preserving perturbation of one). There is **no real translation bug**. Therefore:

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
