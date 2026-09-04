# SACTOR minimal bug-hunt plan (awaiting go-ahead — costs LLM $)

Goal: show the RQ1 findings generalize beyond one lifter — run SACTOR (idiomatic LLM C→Rust
transpiler, gpt-5.1) on a small value-oriented target set, differential-test vs the original C,
and see whether it (a) reproduces the C2SaferRust bug classes, (b) introduces new ones, or
(c) is clean. NOT a full-corpus run.

## Why SACTOR is the right next tool
- Different mechanism from C2SaferRust: SACTOR rewrites **idiomatically and fully** (`&[T]`, `Option`,
  methods) — a richer value-semantics surface than C2SaferRust's partial/skewed lift.
- External validity: 2 tools with overlapping bug classes >> 1 tool. Head-to-head on the SAME C
  (qsort, urlparser) directly tests "is this a C2SaferRust quirk or an LLM-transpiler pattern?"

## Setup status
- DONE: `tools/frameworks/sactor` — `.venv`, `sactor.toml` (model=gpt-5.1, key via `OPENAI_API_KEY`),
  CLI `sactor translate <c> <test_task.json> -r <out> --type lib`.
- TODO (pre-run, no LLM $): (1) verify `rust_ast_parser` builds + one smoke translation on a
  trivial C (e.g. `atoi`) to confirm the pipeline runs end-to-end; (2) validate the OOP harness
  **idiomatic call path** (slice/`Option`/method) on ONE SACTOR output — the raw-ptr path is proven,
  the idiomatic path is coded but only lightly exercised; (3) prepare a minimal `test_task.json` per
  target (SACTOR's verify loop needs tests — small input/output pairs).

## Differential method (per target)
- Oracle = the ORIGINAL C (the exact input to SACTOR), compiled `-fsanitize=undefined,address`
  → UB gate. **No version-mismatch risk** (unlike the C2SaferRust tulip lesson — here we own the C).
- Test = SACTOR's idiomatic Rust output (`--type lib`), called natively via the OOP harness.
- UB-free divergence (value mismatch or Rust panic where C is clean) = bug candidate → triage +
  standalone repro (same protocol as bugs #1–#5).

## SCOPE (tightened, per Codex): cross-tool SANITY, not a new big RQ

The point is to show the pipeline serves more than C2SaferRust — NOT to farm bugs. Two hard-gated phases.

### Phase 1 — 4 cheap targets (hard cap **$10**) — REVISED to use SACTOR's shipped examples

Free-prep discovery: SACTOR ships ready `*_crust` examples WITH `test_task.json` (gate 3 free for these).

| target | why | prep |
|---|---|---|
| **utf8_crust** | HEAD-TO-HEAD on the bug class: does SACTOR also mishandle UTF-8/bytes? (C2SaferRust's `to_str().unwrap()` class). Best cross-tool probe. | shipped ✓ |
| **bitset_crust** | fresh value: bit operations — bit-op, boundary-heavy | shipped ✓ |
| **hamming_crust** | fresh value: hamming codec — bit-op/codec, round-trip | shipped ✓ |
| **qsort** | direct HEAD-TO-HEAD: does SACTOR also break the `int→usize` sort? (C2SaferRust bug #1). C is in hand (`results/rq4_effectiveness/bugs/qsort_c2saferrust/`). | ~15 min prep |

Dropped from phase 1 (need prep, bigger): urlparser, murmurhash, leftpad, approxidate, morton →
phase-2 candidates. The shipped `*_crust` set + qsort covers UTF-8-class probe + bit-op + codec +
sort head-to-head with almost no prep.

**Phase-1 DoD table:**

| Target | SACTOR build | Matcher | Harness | Result |
|---|---|---|---|---|
| utf8_crust | pass/fail | matched? | fuzzed? | clean/bug |
| bitset_crust | pass/fail | matched? | fuzzed? | clean/bug |
| hamming_crust | pass/fail | matched? | fuzzed? | clean/bug |
| qsort | pass/fail | matched? | fuzzed? | clean/bug |

**Gate:** ≥3 of 4 end-to-end → decide on Phase 2. <3 build/bridge failures → STOP, don't burn $,
fix pipeline or write SACTOR as *preliminary*.

### Phase 2 — only IF triggered (overall hard cap **$30**)
- Phase-1 = 0 bug but pipeline stable → run **approxidate** (complex value parser, likely to expose bugs).
- Phase-1 = bug found → do NOT rush approxidate; confirm/attribute/table first.
- Phase-1 = many build/bridge failures → STOP.

## FREE gates (must pass BEFORE the paid targets)
1. **`atoi` smoke** (~$0.1): confirm the whole chain — SACTOR output format, build, matcher, OOP harness.
2. **idiomatic bridge self-test** (free): the OOP harness must handle SACTOR shapes — slice, `Option`,
   method/associated-fn — at least one each. Raw-ptr path is proven; this path is only lightly tested.
3. **minimal `test_task.json` per target** (free): keep SACTOR's verify loop from writing unrelated tests.

If gate 1 or 2 fails → STOP before the 4 targets; fix first.

## Expected outcomes (any is publishable)
- SACTOR reproduces the UTF-8-panic class (e.g. on urlparser) → **class generalizes across tools** (strong).
- SACTOR breaks qsort differently / correctly → tool-comparison data point.
- SACTOR clean on the value set → "idiomatic + verified lift is more robust; C2SaferRust's partial
  lift is the weak point" (also a finding).

## Cost guard (reminder)
`key.env` = bare OpenAI key (gitignored, never commit). All SACTOR runs are billed to it; I will
report spend after each target. **Caps: Phase 1 = $10, overall = $30.** Stop at the cap or on your word.
(Deliberately NOT $40 — a tight cap keeps this a sanity check, not "keep trying".)
