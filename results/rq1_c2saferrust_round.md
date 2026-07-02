# RQ1 — C2SaferRust value-oriented bug-hunt round (2026-07-02)

Target: real semantic bugs in C2SaferRust's published safety-lifted output, value-oriented subset.
Method locked from bug #1 (qsort): compare C2SaferRust `_WIP` output against the SAME-SOURCE
faithful c2rust `base` (both shipped in `laertes_benchmarks/`) — the base behaves like the original
C, so a divergence localizes to C2SaferRust's rewrite.

## Methodological lesson (a false divergence caught and killed)

First attempt used the upstream tulipindicators C (`tools/frameworks/tulipindicators`) as the oracle.
It immediately "found" a divergence in `dx`: C=`[nan, finite]`, Rust=`[nan, nan]`. **Bogus** — the
upstream C's `dx` is a 2-input (high/low) version, but C2SaferRust's `dx` is a 3-input (high/low/
**close**, ATR-normalized) version. Different SOURCE versions → apples-to-oranges. Switching the
oracle to the **base c2rust of the same source** (built as an ASan binary), `dx` matches on both
(`nan:nan`). **Attribution REQUIRES a same-source faithful reference; a different-version oracle
manufactures false divergences.** This is the analogue of the "faithful control column" in bug #1.

## Harness built (reusable)

Generic table-dispatch differential harness for tulipindicators: both C2SaferRust `_WIP` and faithful
`base` expose `ti_indicators[]` (fn-pointer + input/option/output counts), so ONE harness covers all
104 indicators. `scratchpad/ti_poc/` (WIP fuzz target) + `scratchpad/ti_base_oracle/` (faithful ASan
oracle). Byte protocol: idx, size, per-input `size` doubles, per-option a small positive; NaN-canonical
bit compare of outputs; C2SaferRust side runs debug-assertions (overflow/OOB panic surfaces).
Both crates build on Linux under nightly-2025-09-01 (strip the macOS `smoke` module + stale feature
gates; stub `__assert_rtn`→abort). Result: **17,533 execs, 0 divergences.**

## The decisive finding: C2SaferRust's rewrite surface is narrow

`ti_indicators` 0 divergences is **trivially expected** — C2SaferRust did NOT rewrite tulipindicators.
Measuring the rewrite delta (WIP minus base, over `as usize|wrapping_|&mut [|get_unchecked|.iter()`):

| program | base | WIP | delta | note |
|---|---:|---:|---:|---|
| optipng | 2792 | 5287 | **+2495** | PNG codec — big rewrite, but pointer-graph-heavy |
| snudown | 542 | 1071 | **+529** | markdown parser — string/pointer-graph |
| urlparser | 9 | 21 | **+12** | small; string→bool/substring (value-ish, harnessable) |
| qsort | 0 | 5 | **+5** | = **bug #1** (confirmed, archived) |
| grabc | 0 | 2 | +2 | X11 tool (needs display) |
| tulipindicators | 158 | 163 | ~0 | NOT rewritten (`&[` noise only) → null target |
| genann/bzip2/lil/xzoom | — | — | **negative** | C2SaferRust lift INCOMPLETE (reverted idioms) |

So the value-oriented C2SaferRust bug surface is essentially: **qsort (found, bug #1)** + urlparser
(small, untested) + two large pointer-graph codecs (optipng/snudown) that need heavy per-function
value-boundary selection. The one clean value program (tulipindicators) was left untouched.

## Candidate table (DoD)

| Program | Boundaries | Result | Evidence |
|---|---|---|---|
| tulipindicators | 104 (all indicators) | CLEAN — but *unrewritten* (uninformative) | 17533 execs, 0 div; base==WIP |
| qsort | quickSort/partition | **BUG_CANDIDATE (confirmed)** | `results/rq1_bugs/qsort_c2saferrust/` (bug #1) |
| optipng | (rewritten, not harnessed) | not run — complex pointer-graph codec | rewrite delta +2495 |
| snudown | (rewritten, not harnessed) | not run — complex string/pointer-graph | rewrite delta +529 |
| urlparser | url_is_*/url_get_* | not run — available (string→bool/substring) | rewrite delta +12 |

## Conclusion & recommendation

Per the round-stop rule: a rigorous round yielded **no NEW value bug beyond qsort**, because
C2SaferRust only meaningfully lifted a few programs and skewed them toward complex pointer-graph
codecs; the clean value program was untouched. C2SaferRust's value-oriented yield ≈ **1 (qsort)**.

Recommendation: do NOT deep-dive optipng/snudown value-boundary selection (high cost, uncertain).
Two options for more value bugs: (a) one cheap free attempt on **urlparser** (rewritten, string-value,
~rgba-shaped harness); (b) authorize **SACTOR** on 5–8 high-coverage CRUST-bench value targets —
SACTOR lifts value code idiomatically and fully (unlike C2SaferRust's partial/skewed lift), so it is
the richer value-bug source. If both still yield only 1–2, write RQ1 honestly as a bug-finding case
study, not a large-scale RQ.
