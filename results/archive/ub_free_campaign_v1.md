# Frontier UB-free campaign v1 — the loop-closer experiment

**Question:** fuzz the boundaries the selector calls **SAFE** (frontier v2 members), with the
**in-loop UB-free gate ON**. By elimination, a divergence at a SAFE + UB-free boundary cannot
be blamed on UB (the gate excludes it) nor on a bad boundary (the selector vouched SAFE) — so
it is a **real problem**: a genuine c2rust translation bug, or a selector *false-safe*.
**Zero is also a result** = empirical precision of the SAFE claim (complements G2 recall).

`C_UB_CONFIRMED` is **excluded** (unsafe-origin), never counted as a fidelity bug — the main
line measures *translation fidelity*, not security (boundary-unsafety is a separate side
contribution). See `docs/stu_selection.md`, memory framing-and-related-work.

## Result (DUR=30s/boundary, in-loop UB-free gate ON)

| program | frontier_member (SAFE boundary) | result |
|---|---|---|
| g3_case_a | scale_pct | NO_DIVERGENCE |
| g3_g2_bug | **safe_ratio** | **HARNESS_DIVERGENCE** ← real UB-free bug surfaced |
| g3_three_level | safe_ratio | NO_DIVERGENCE |
| div_mod | div_mod_safe | NO_DIVERGENCE |
| hex_encode | hex_encode | NO_DIVERGENCE |
| rle_codec | rle_encode | NO_DIVERGENCE |
| safe_stats | count_above | NO_DIVERGENCE |
| safe_stats | xorshift_fold | NO_DIVERGENCE |

**Summary: 7 SAFE boundaries clean, 1 real divergence — exactly the one with an injected bug.**

## What it shows

1. **Precision of the SAFE claim (the headline):** every faithful SAFE boundary ran clean
   (0 false divergence) under the gate. The selector's "this boundary is safe to differentially
   test" holds empirically — no UB noise, no bad-boundary artifacts.
2. **A real bug at a SAFE boundary IS caught:** `g3_g2_bug:safe_ratio` carries the injected
   mistranslation (Rust `scale` = `x*10` vs C `x*100`). It clamps `pct∈[0,100]` then calls
   scale, so the divergence is on **UB-free** input (no overflow). The gate keeps it →
   `HARNESS_DIVERGENCE`, evidence *"outputs differ; no C UB and no crash"*. Conservatively
   labeled (the auto-classifier never self-declares TRANSLATION_BUG — human-confirmed only),
   but it is a genuine UB-free value divergence = the loop-closer firing. With G2 (recall) this
   gives **recall + precision**: catches the real bug, doesn't cry wolf on the clean ones.
3. **The two novelties compose (div_mod):** the frontier picked `div_mod_safe` (guarded) and it
   ran clean — while the RISKY sibling `mod_signed_i32` (unguarded `a%b`) was classified
   `C_UB_CONFIRMED` (divide-by-zero) when forced in the support matrix. The frontier *statically
   avoids* the hard-trap-UB boundary and selects the comparable one; the gate then keeps the
   comparison honest. Hard-trap UB is "not selected", not "let through".

## Scope / honesty
- **Faithful c2rust → expect ~zero real bugs** (it is byte-faithful). 7/7 clean is the
  *validation* result, not a failure to find bugs. Real fidelity-bug *hunting* is the
  matcher-enabled LLM-translation track (translations there actually err).
- **Subset, not exhaustive:** 8 of the 78 SAFE frontier boundaries (41 programs). Corpus is
  shallow (only ~7/48 programs have a non-trivial frontier choice), so scale is limited until
  deeper real libraries / the LLM track.
- **Hard-trap UB (div-by-zero SIGFPE, memory SIGSEGV)** is handled by frontier-avoidance +
  post-hoc classifier (`C_UB_CONFIRMED`), not the in-loop gate (which covers recoverable UB:
  overflow/shift/bounds). Not building the longjmp gate now — it would duplicate the frontier's
  static avoidance and pull toward building a sanitizer runtime. Revisit only if a non-trivial
  function has deep UB-free behavior gated behind an easy hard-trap UB (none in this corpus).

## Reproduce
```
DUR=30 python3 scripts/frontier_campaign.py [prog ...]   # SAFE frontier boundaries, --ub-free
UB_FREE=1 DUR=30 python3 scripts/run_g1_matrix.py [prog ...]   # per-program ENTRY, --ub-free
```
