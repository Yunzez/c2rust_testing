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
| `⊘(reason)` | tool blocked by environment (not the tool's fault semantically) |
| `∅` | artifact exists or pipeline ready — **not yet tested (TODO)** |
| `—` | not applicable (no artifact, tool can't target this) |

Method for every filled cell: concrete differential execution vs original C (exhaustive when the
domain is finite, fuzzed otherwise — same method, different coverage), ASan/UBSan gate on the C side.

## The table

| library | domain | LOC | ~#fn | c2rust (mech.) | Laertes | C2SaferRust | CROWN | SACTOR | PtrTrans |
|---|---|---:|---:|---|---|---|---|---|---|
| **qsort** | sorting | 30 | 3 | ✓ base | ∅ | **c:1** ¹ | — | ✓F | ∅ᵖ |
| **urlparser** | URL parsing | ~1.3k | ~15 | ✓ base | ∅ | **c:1** ² | ∅ᵃ | ∅ | ∅ᵖ |
| **quadtree** | spatial tree | 439 | ~25 | ✓ base | — | — | ∅ | ∅ | **✓F** ³ |
| **genann** | neural net (f64) | 895 | ~20 | ✓F(300k) | ✓ ⁴ | ✓F(50M) ⁴ | **✓F(300k)** | ∅ | ∅ᵖ |
| **cJSON** | JSON parser (recursive) | 3206 | 118 | ✓F(100k) | — | ⊘(nightly slicer) | ✗(rewrite crash) | ✗(circular deps) | **▲94/118, s:3** ★⁵ |
| **lil** | script interpreter | 3723 | ~128 | ✓ base | ∅ | **c:1** ⁶ | ∅ᵃ | ∅ | ∅ᵖ |
| **lodepng** | PNG codec | 6658 | ~200 | ∅ | — | — | ∅ | ∅ | ∅ᵖ |
| **bzip2** | compressor | 7344 | ~110 | ✓ base | ∅ | **c:1 s:1** ⁷ | **c:1 s:2** ★¹⁰ | ∅ | ∅ᵖ |
| **tulipindicators** | financial indicators | ~5k | ~100 | ✓ base | ∅ | ∅ (7 utf8 sites untriaged) | ∅ᵃ | — | — |
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

## Current totals (filled cells only)

- **Bugs: 7 crash + 8 semantic-diff + 0 hang**, across **3 published tools** (C2SaferRust, PtrTrans,
  CROWN), classes: NULL/empty conflation; UTF-8-panic; call-site contract loss; **CROWN
  ownership-lift breaking a codec (corrupt output + memory-unsafety)**
- **Certificates: 7 cells** (incl. one full 4-lifter row: genann)
- **Tool failures: 3** (CROWN/SACTOR on cJSON; C2SaferRust env-blocked)
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
