# E1 Master Table — library × tool differential results

**The paper's table 1.** 10 libraries, scale-laddered (30 → ~30k LOC), domain-diverse. Every cell is
one of: bug counts, a certificate, a tool-failure, or a TODO. **0-diff is a result (certificate), not
a blank; tool-can't-run is a finding, not a blank.**

## Cell legend

| mark | meaning |
|---|---|
| `c:x s:y h:z` | confirmed bugs: **c**rash / **s**emantic-diff / **h**ang (the 3 classes; all C-backed + UB-gated) |
| `✓E` | equivalence certificate, **exhaustive** (whole input domain) |
| `✓F(N)` | equivalence certificate, fuzzed, N samples |
| `✓p` | partial certificate (subset of functions; noted) |
| `▲x/y` | tool produced **partial translation** (x of y units; rest = visible stubs — counted separately from silent bugs) |
| `✗(reason)` | tool fails on this library (crash / unsupported) — a finding |
| `⊘(reason)` | tool blocked by environment, or **C-side UB** makes the reference untestable (UB gate excludes) |
| `▽(reason)` | tool's rewrite surface on this library is minimal (nothing semantically interesting to test) |
| `∅` | artifact exists or pipeline ready — **not yet tested (TODO)** |
| `—` | not applicable (no artifact, tool can't target this) |

Method for every filled cell: concrete differential execution vs original C (exhaustive when the
domain is finite, fuzzed otherwise — same method, different coverage), ASan/UBSan gate on the C side.

## The table

| library | domain | LOC | ~#fn | c2rust (mech.) | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans |
|---|---|---:|---:|---|---|---|---|---|---|
| **qsort** | sorting | 30 | 3 | ✓ base | **✓E/F(50k)** ¹⁷ | **c:1** ¹ | — | ✓F | ∅ᵖ |
| **urlparser** | URL parsing | ~1.3k | ~15 | ✓ base | ∅ | **c:1** ² | ⊘(C-side UB)¹² | ∅ | ∅ᵖ |
| **quadtree** | spatial tree | 439 | ~25 | ✓ base | — | — | ∅ | ∅ | **✓F** ³ |
| **genann** | neural net (f64) | 895 | ~20 | ✓F(300k) | **✓F(200k)** ¹⁶ | ✓F(50M) ⁴ | **✓F(300k)** | ∅ | ∅ᵖ |
| **cJSON** | JSON parser (recursive) | 3206 | 118 | ✓F(100k) | — | ⊘(nightly slicer) | ✗(rewrite crash) | ✗(circular deps) | **▲94/118, s:3** ★⁵ |
| **lil** | script interpreter | 3723 | ~128 | ✓ base | **✓F(111k)** ¹⁵ | **c:1** ⁶ | **✓F(111k)** ¹¹ | ∅ | ∅ᵖ |
| **lodepng** | PNG codec | 6658 | ~200 | ∅ | — | — | ∅ | ∅ | ∅ᵖ |
| **bzip2** | compressor | 7344 | ~110 | ✓ base | **s:1** ★¹⁴ | **c:1 s:1** ⁷ | **c:1 s:2** ★¹⁰ | ∅ | ∅ᵖ |
| **tulipindicators** | financial indicators | ~5k | ~100 | ✓ base | ∅ | ∅ (7 utf8 sites untriaged) | ▽(minimal surface)¹³ | — | — |
| **optipng** (incl. zlib) | PNG optimizer | ~30k | ~400 | ✓ base | ✓p ⁸ | **c:1 s:2** ★⁹ | — | — | — |

ᵃ CROWN artifact **already shipped** in laertes_benchmarks (`*_crown/`) — zero-cost to test.
ᵖ C source in PtrTrans crown_dataset — our reproduced pipeline can translate (LLM cost per lib).

## Evidence footnotes

1. qsort `int→usize` sentinel break → infinite recursion + OOB (`rq1_bugs/qsort_c2saferrust/`)
2. `url_is_ssh` `to_str().unwrap()` panics on non-UTF-8 (UTF-8-panic class, `rq1_bugs/utf8_panic_c2saferrust/`)
3. PtrTrans's own shipped quadtree translation, differential clean (scratch `ptr_quad/`)
4. genann: all 4 lifters clean — C ≡ c2rust ≡ CROWN bit-exact 300k (`rq1_genann_matrix.md`); C2SaferRust rewrote only sigmoid (50M/0), kept `genann_run` textually = base; Laertes ditto
5. ★ **headline #2**: parse_string cluster — `\u` escapes dead (call-site empty slice), valuestring=None, non-UTF-8 reject; 40,133/120,050 UB-free divergences; 24/118 units = visible stubs (`rq1_bugs/cjson_ptrtrans/`)
6. `do_system` argv `to_str().unwrap()` panic (UTF-8-panic class)
7. `endsInBz2` panic (c) + `BZ2_bzBuffToBuffCompress` empty-buffer reject (s, NULL/empty conflation)
8. Laertes crc32_z 3-way tested faithful (crc32 only — partial)
9. ★ **headline #1**: crc32_z + adler32_z empty-chunk reset (`is_null`→`is_empty`, NULL/empty conflation class) + `-dir` UTF-8 panic (`rq1_bugs/crc32_c2saferrust/`)
10. ★ **headline #3**: CROWN (a *safety* lifter) breaks bzip2 — compress 29% correct / 46% silently-corrupt output (bunzip2-rejected, BZ_OK returned) / 25% heap-corruption crash; decompress default path (small=0) BZ_DATA_ERROR on valid data. base c2rust byte-exact faithful → CROWN's ownership-lift introduced it. ASan/UBSan-gated (`rq1_bugs/bzip2_crown/`)
11. CROWN lil: **certificate** — `expr` evaluator (CROWN rewrote all `ee_*`) + variable/list/string scripts, **111,043 records, 0 diffs**, UB-gated (12 inputs trigger C-side UB in lil's own expr — shift-out-of-range / INT_MIN-negate / signed-overflow — excluded; CROWN matches C even on those). Same tool as #10: CROWN faithful on lil, broken on bzip2 — the divergence is per-library, sharpening that lifter correctness is not uniform (scratch `lil_crown_diff/`)
12. urlparser CROWN: **UB-gate exclusion**. Original C `url_parse` (url.h header-lib) overflows a fixed buffer on *every* valid URL (`_FORTIFY_SOURCE`/ASan abort) — the reference is UB, so no Rust divergence can be attributed and there is nothing UB-free to certify. A clean illustration of the gate preventing false positives (matches the C2SaferRust note: `url_get_*` malloc(1)+sscanf is pre-existing C UB). CROWN's rewrite surface here is also minimal (only `url_free`). The C2SaferRust bug #2 lives in `url_is_ssh`, a *different*, UB-free function.
13. tulipindicators CROWN: **minimal rewrite surface** — CROWN rewrote only `ti_buffer_new/free` (memory mgmt) + a test-file helper; the ~100 indicator value functions were left mechanical (= the c2rust column). No value-semantics surface to differentially test; deferred as low-information.
14. ★ **headline #4**: Laertes bzip2 — **uncalled static-table init** zeroes `BZ2_crc32Table` (and 37 other globals) → compress returns BZ_OK but writes wrong CRC → 91% of inputs produce integrity-invalid streams (canonical `bunzip2 -t` rejects). **Second independent instance of the checksum-corruption class** (after C2SaferRust crc32), cleaner mechanism: const-initializer → runtime init fn with 0 call sites. base c2rust byte-exact faithful → Laertes-introduced (`rq1_bugs/bzip2_laertes/`)
15. Laertes lil: **certificate** — same 111,043-record corpus, **111,042 identical**. The 1 "diff" (`expr ((1+2)*(3+4))`) is **excluded as oracle-nondeterminism, NOT a Laertes bug**: the original C `expr` is *order-dependent* (returns `[25]` when the record runs first, `[]` when any shorter record precedes it — a global-state/buffer leak in lil's own `expr`, ASan/UBSan-clean). When the C reference is self-inconsistent on an input, no divergence can be attributed. A new exclusion category alongside UB: **stateful-nondeterminism of the reference**. (The harness incidentally surfaced a latent lil-C bug — candidate for a separate finding.) Note Laertes's lil `laertes_init_*` targets are only `running`/`exit_code` scalars (harmless, already 0).
16. Laertes genann: **certificate** — 200k records, 0 diffs, **default cached-sigmoid path** (exercises the `lookup` table). genann's `laertes_init_lookup` is *also* uncalled (same pattern as the bzip2 CRC bug!), but here it is **harmless** because `genann_act_sigmoid_cached` retains its runtime lazy-rebuild (`if !initialized { build }`) — the table repopulates on first use. The precise distinction from #14: the uncalled-init bug is fatal only when the zeroed static is the *sole* source of the value (bzip2 CRC: no rebuild) and harmless when a runtime rebuild exists (genann lookup). Demonstrates the finding is mechanism-specific, not a blanket "Laertes zeroes tables" claim.
17. Laertes qsort: **certificate** — exhaustive small arrays (len 0–3 over [-4,4]) + the `[5,0]` C2SaferRust-bug trigger + 50k fuzz = 50,827 records, 0 diffs. **Laertes did NOT reproduce the C2SaferRust qsort bug**: it kept `low/high/i` as signed `i32` (`i = low - 1`), where C2SaferRust's `int→usize` rewrite broke the sentinel. Same function, opposite outcomes across tools — the cross-tool contrast qsort was chosen for.

## Current totals (filled cells only)

- **Bugs: 7 crash + 9 semantic-diff + 0 hang**, across **4 published tools** (C2SaferRust, PtrTrans,
  CROWN, Laertes), classes: **checksum-corruption (now 2 independent instances: C2SaferRust crc32 +
  Laertes bzip2)**; NULL/empty conflation; UTF-8-panic; call-site contract loss; CROWN
  ownership-lift breaking a codec (corrupt output + memory-unsafety)
- **Certificates: 12 cells** (genann now a full 4-lifter clean row; Laertes column: qsort/lil/genann all
  faithful — rule-based lifter is conservative & mostly correct, *except* the systematic bzip2 CRC bug)
- **Tool failures: 3** (CROWN/SACTOR on cJSON; C2SaferRust env-blocked); **1 UB-gate exclusion**
  (urlparser); **1 oracle-nondeterminism exclusion** (Laertes lil nested-paren — surfaced a latent lil-C bug)
- **Cross-tool contrasts locked**: qsort (C2SaferRust crashes / Laertes faithful — int→usize vs kept-i32);
  bzip2 (all 3 non-mechanical lifters broken, 3 different mechanisms); genann (all 4 faithful)
- **Two headlines from bzip2 alone**: all THREE lifters that touched it are broken (Laertes CRC-zeroing,
  C2SaferRust NULL/empty, CROWN corrupt+unsafe) — only mechanical c2rust is faithful. A checksum/codec is
  a lifter minefield.
- **Irony headline**: CROWN, a *safety* lifter, is the one that introduced memory corruption (bzip2) —
  a mechanical c2rust baseline was safe; the "safety" rewrite was not

## TODO queue (the ∅ cells, by cost-effectiveness)

| priority | cells | cost | expected yield |
|---|---|---|---|
| **P0** | CROWN column: bzip2ᵃ / lilᵃ / urlparserᵃ / tulipᵃ (+ lodepng, quadtree, buffer via CROWN runs) | **zero LLM $** — artifacts exist; build + harness only | CROWN is a *safety lifter* rewriting guards — NULL/empty-class prime suspect |
| **P1** | Laertes column: qsort / urlparser / lil / bzip2 / tulip ∅ | zero $ — artifacts exist | rule-based lifter; likely certificates (strengthens the contrast) |
| **P2** | PtrTrans column ᵖ: genann / lil / urlparser / bzip2 / lodepng / qsort | ~1 gpt-5.1 run per lib (cJSON cost ~70 min) | headline #3 candidates — parse_string pattern suggests more call-site bugs |
| **P3** | SACTOR column: urlparser / quadtree / genann / lil / lodepng | LLM $ + known fragility | likely ✗ or ✓ (faithful-or-fail) — fills the spectrum |
| P4 | c2rust lodepng base + tulip triage (7 utf8 sites) | low | mechanical baseline + cheap crash confirmations |

Alternates if a row underperforms: buffer (605, CROWN bench + crown_dataset), rgba (tiny), snudown
(markdown; C2SaferRust uses safe strings — likely all-certificate row), grabc.
