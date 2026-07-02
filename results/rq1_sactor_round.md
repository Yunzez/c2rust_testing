# RQ1 — SACTOR cross-tool sanity round (2026-07-02)

Goal (per the tightened plan): cross-tool SANITY, not a new big RQ. Does the differential pipeline
serve a SECOND tool (SACTOR, idiomatic LLM C→Rust, gpt-5.1), and do the C2SaferRust bug classes
reproduce / differ / vanish? Method: original C (ASan/UBSan oracle) vs SACTOR idiomatic Rust — we own
the C, so NO version-mismatch risk.

## Pipeline setup (gates)
- SACTOR needs `crown` AND `sactor` on PATH (its verifier spawns `sactor run-tests` as a subprocess) +
  `LD_LIBRARY_PATH` to the nightly-2023-01-26 rustc lib (crown). Once set, translate + verify runs.
- **SACTOR limitation found + patched:** it fails on SELF-RECURSIVE functions (dependency checker
  can't order `f` before `f`). Patched two sites (`translator.py:check_dependencies`,
  `idiomatic_translator.py:1812`) to skip self-references — mirrors SACTOR's own self-skip elsewhere.
  Without this, `quickSort` (and any recursive value function) cannot be translated.
- Idiomatic harness slice-path validated free (hand `sum(&[i32])` vs C, 6720 execs clean).

## Phase-1 DoD

| Target | SACTOR build | Matcher | Harness (execs) | Result |
|---|---|---|---|---|
| hamming_crust | pass | trivial (names kept) | fuzzed (13,245) | **CLEAN** |
| qsort | pass (needed recursion patch) | trivial | fuzzed (10,770) | **CLEAN** |
| utf8_crust | idiomatic retrying | — | — | pending |
| ~~bitset_crust~~ | — | — | — | dropped (pointer-graph `bitset_t*` handle, out of value scope) |

## Key cross-tool findings

1. **SACTOR does NOT reproduce bug #1 (qsort `int→usize`).** SACTOR translated the same `quickSort`
   correctly: the recursion (`quick_sort_rec`) keeps `i32` (so the negative-sentinel `i-1 = -1` works),
   and while `do_partition` uses `usize` + `wrapping_sub(1)`, every index is guarded (`split_at_mut` +
   `if i<j` + `unreachable!()`), correctly mimicking C's `-1` via double-wrapping. 10,770 execs, 0
   divergence. **⇒ the qsort bug is C2SaferRust-specific carelessness, NOT an inherent LLM-transpiler
   pattern.** A second LLM transpiler got the same function right.

2. **SACTOR's idiomatic reshape is clean and semantics-preserving** (hamming): `(ptr,len)` → `&[u8]`,
   `(bool ret, *outsize)` → `Option<usize>`; round-trips correctly vs C.

3. **UTF-8 handling differs by DESIGN** (from atoi + utf8 unidiomatic): SACTOR changes `char*` → `&str`
   at the API (e.g. `atoi(s: &str)`), so non-UTF-8 is excluded at the TYPE level — a domain narrowing,
   not C2SaferRust's runtime `to_str().unwrap()` panic. (utf8_crust idiomatic pending to confirm the
   reshape; unidiomatic keeps `*const c_char`.) Both diverge from C's "accepts arbitrary bytes", but
   differently — a clean tool-comparison point.

## Cost
Cumulative ~**$0.30** (gpt-5.1). Cap $10 phase-1 / $30 overall — far under.

## Reading
The pipeline generalizes to a 2nd tool. The two C2SaferRust bug classes do NOT blindly reproduce in
SACTOR: qsort is correct (mechanism was tool-specific), and the UTF-8 issue manifests as a different
(type-level) divergence rather than a panic. This is exactly the cross-tool sanity the round was for:
our findings are not artifacts of one lifter, and different tools fail in different ways.
