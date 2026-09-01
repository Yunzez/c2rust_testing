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
| utf8_crust | **idiomatic FAIL** (`MAX_ATTEMPTS_EXCEEDED`, ~$1.53 in retries) | — | — | **build-fail** (SACTOR couldn't produce idiomatic UTF-8 code) |
| ~~bitset_crust~~ | — | — | — | dropped (pointer-graph `bitset_t*` handle, out of value scope) |

**Gate:** 2/3 end-to-end CLEAN, 1 idiomatic-translation failure. Cross-tool sanity ACHIEVED (below the
≥3 threshold for auto-triggering phase 2; and 0 SACTOR bugs found, so no attribution work needed).

## Key cross-tool findings

1. **SACTOR does NOT reproduce bug #1 (qsort `int→usize`).** SACTOR translated the same `quickSort`
   correctly: the recursion (`quick_sort_rec`) keeps `i32` (so the negative-sentinel `i-1 = -1` works),
   and while `do_partition` uses `usize` + `wrapping_sub(1)`, every index is guarded (`split_at_mut` +
   `if i<j` + `unreachable!()`), correctly mimicking C's `-1` via double-wrapping. 10,770 execs, 0
   divergence. **⇒ the qsort bug is C2SaferRust-specific carelessness, NOT an inherent LLM-transpiler
   pattern.** A second LLM transpiler got the same function right.

2. **SACTOR's idiomatic reshape is clean and semantics-preserving** (hamming): `(ptr,len)` → `&[u8]`,
   `(bool ret, *outsize)` → `Option<usize>`; round-trips correctly vs C.

3. **Three tools, three UTF-8 behaviors** — a clean cross-tool comparison on the SAME byte-string issue:
   - **c2rust** (faithful): keeps `*const c_char`, byte-accepting, correct.
   - **C2SaferRust**: keeps `char*`, inserts `to_str().unwrap()` → **runtime PANIC** on non-UTF-8 (bugs #2–#5).
   - **SACTOR**: changes `char*` → **`&str`** (type-level domain narrowing, e.g. `atoi(s: &str)`) on simple
     cases; on UTF-8-heavy code (utf8_crust) it **fails to translate idiomatically at all**
     (`MAX_ATTEMPTS_EXCEEDED` after ~$1.53 of retries). So SACTOR's UTF-8 failure mode is *domain
     narrowing or a translation failure*, NOT a runtime panic. Different tools, different failure modes.

## Cost
Cumulative ~**$1.75** (gpt-5.1): hamming ~$0.16, qsort ~$0.06, utf8 ~$1.53 (idiomatic retries), atoi
smoke ~$0.05. Cap $10 phase-1 / $30 overall — far under.

## Reading
The pipeline generalizes to a 2nd tool. The two C2SaferRust bug classes do NOT blindly reproduce in
SACTOR: qsort is correct (mechanism was tool-specific), and the UTF-8 issue manifests as a different
(type-level) divergence rather than a panic. This is exactly the cross-tool sanity the round was for:
our findings are not artifacts of one lifter, and different tools fail in different ways.
