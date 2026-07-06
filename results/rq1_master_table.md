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
| **qsort** | sorting | 30 | 3 | ✓ base | **✓E/F(50k)** ¹⁷ | **c:1** ¹ | **✓E/F(50k)** ²⁰ | ✓F | **s:1** ★²⁹ |
| **urlparser** | URL parsing | ~1.3k | ~15 | ✓ base | ⊘(C-side UB)²² | **c:1** ² | ⊘(C-side UB)¹² | ∅ | ⊘(C-side UB)²⁷ |
| **quadtree** | spatial tree | 439 | ~25 | ✓ base | — | — | **✓F*** ¹⁸ | ∅ | **✓F** ³ |
| **genann** | neural net (f64) | 895 | ~20 | ✓F(300k) | **✓F(200k)** ¹⁶ | ✓F(50M) ⁴ | **✓F(300k)** | ∅ | **▲decl-only** ²⁴ |
| **cJSON** | JSON parser (recursive) | 3206 | 118 | ✓F(100k) | — | ⊘(nightly slicer) | ✗(rewrite crash) | ✗(circular deps) | **▲94/118, s:3** ★⁵ |
| **lil** | script interpreter | 3723 | ~128 | ✓ base | **✓F(111k)** ¹⁵ | **c:1** ⁶ | **✓F(111k)** ¹¹ | ∅ | **✗(compile)** ²⁵ |
| **lodepng** | PNG codec | 6658 | ~200 | **✓F(3036)** ¹⁹ | — | — | **✓F(3036)** ★¹⁹ | ∅ | **✗(compile)** ²⁸ |
| **bzip2** | compressor | 7344 | ~110 | ✓ base | **s:1** ★¹⁴ | **c:1 s:1** ⁷ | **c:1 s:2** ★¹⁰ | ∅ | **✗(compile)** ²⁶ |
| **tulipindicators** | financial indicators | ~5k | ~100 | ✓ base | **✓F(150k)** ²³ | ∅ (7 utf8 sites untriaged) | ▽(minimal surface)¹³ | — | — |
| **optipng** (incl. zlib) | PNG optimizer | ~30k | ~400 | ✓ base | **s:1** ★⁸ | **c:1 s:2** ★⁹ | **✗(analyse panic)** ²¹ | — | — |

ᵃ CROWN artifact **already shipped** in laertes_benchmarks (`*_crown/`) — zero-cost to test.
ᵖ C source in PtrTrans crown_dataset — our reproduced pipeline can translate (LLM cost per lib).
`✓F*` = certificate on all non-crashing inputs; a shared crash (base c2rust crashes identically) is not tool-attributable.

## Evidence footnotes

1. qsort `int→usize` sentinel break → infinite recursion + OOB (`rq1_bugs/qsort_c2saferrust/`)
2. `url_is_ssh` `to_str().unwrap()` panics on non-UTF-8 (UTF-8-panic class, `rq1_bugs/utf8_panic_c2saferrust/`)
3. PtrTrans's own shipped quadtree translation, differential clean (scratch `ptr_quad/`)
4. genann: all 4 lifters clean — C ≡ c2rust ≡ CROWN bit-exact 300k (`rq1_genann_matrix.md`); C2SaferRust rewrote only sigmoid (50M/0), kept `genann_run` textually = base; Laertes ditto
5. ★ **headline #2**: parse_string cluster — `\u` escapes dead (call-site empty slice), valuestring=None, non-UTF-8 reject; 40,133/120,050 UB-free divergences; 24/118 units = visible stubs (`rq1_bugs/cjson_ptrtrans/`)
6. `do_system` argv `to_str().unwrap()` panic (UTF-8-panic class)
7. `endsInBz2` panic (c) + `BZ2_bzBuffToBuffCompress` empty-buffer reject (s, NULL/empty conflation)
8. ★ **headline (3rd checksum-corruption instance)** — CORRECTS an earlier premature "✓p faithful" note. Laertes optipng: optipng's bundled zlib is compiled **without `DYNAMIC_CRC_TABLE`**, so `crc_table` is a precomputed static read directly by `crc32_z` — **no runtime rebuild**. Laertes lowered it to `laertes_init_crc_table()` with **0 call sites** → all-zero at runtime → degenerate CRC. **C (canonical zlib) ≡ base c2rust: 0 diffs; C vs Laertes: 196,985/200,006 wrong CRC (98.49%)**; `crc32("a")` C=`e8b7be43` vs Laertes=`ff000000`. **adler32: 0 diffs** (arithmetic, no table — cleanly isolates the bug to the table path). ASan/UBSan 0 reports. Same mechanism as bzip2_laertes (#14). **This is the SAME `crc32_z` C2SaferRust broke differently (#9, empty-chunk reset) — one function, two tools, two distinct silent bugs.** Evidence: `rq1_bugs/optipng_laertes/` (commit 2523f36)
9. ★ **headline #1**: crc32_z + adler32_z empty-chunk reset (`is_null`→`is_empty`, NULL/empty conflation class) + `-dir` UTF-8 panic (`rq1_bugs/crc32_c2saferrust/`)
10. ★ **headline #3**: CROWN (a *safety* lifter) breaks bzip2 — compress 29% correct / 46% silently-corrupt output (bunzip2-rejected, BZ_OK returned) / 25% heap-corruption crash; decompress default path (small=0) BZ_DATA_ERROR on valid data. base c2rust byte-exact faithful → CROWN's ownership-lift introduced it. ASan/UBSan-gated (`rq1_bugs/bzip2_crown/`)
11. CROWN lil: **certificate** — `expr` evaluator (CROWN rewrote all `ee_*`) + variable/list/string scripts, **111,043 records, 0 diffs**, UB-gated (12 inputs trigger C-side UB in lil's own expr — shift-out-of-range / INT_MIN-negate / signed-overflow — excluded; CROWN matches C even on those). Same tool as #10: CROWN faithful on lil, broken on bzip2 — the divergence is per-library, sharpening that lifter correctness is not uniform (scratch `lil_crown_diff/`)
12. urlparser CROWN: **UB-gate exclusion**. Original C `url_parse` (url.h header-lib) overflows a fixed buffer on *every* valid URL (`_FORTIFY_SOURCE`/ASan abort) — the reference is UB, so no Rust divergence can be attributed and there is nothing UB-free to certify. A clean illustration of the gate preventing false positives (matches the C2SaferRust note: `url_get_*` malloc(1)+sscanf is pre-existing C UB). CROWN's rewrite surface here is also minimal (only `url_free`). The C2SaferRust bug #2 lives in `url_is_ssh`, a *different*, UB-free function.
13. tulipindicators CROWN: **minimal rewrite surface** — CROWN rewrote only `ti_buffer_new/free` (memory mgmt) + a test-file helper; the ~100 indicator value functions were left mechanical (= the c2rust column). No value-semantics surface to differentially test; deferred as low-information.
14. ★ **headline #4**: Laertes bzip2 — **uncalled static-table init** zeroes `BZ2_crc32Table` (and 37 other globals) → compress returns BZ_OK but writes wrong CRC → 91% of inputs produce integrity-invalid streams (canonical `bunzip2 -t` rejects). **Second independent instance of the checksum-corruption class** (after C2SaferRust crc32), cleaner mechanism: const-initializer → runtime init fn with 0 call sites. base c2rust byte-exact faithful → Laertes-introduced (`rq1_bugs/bzip2_laertes/`)
15. Laertes lil: **certificate** — same 111,043-record corpus, **111,042 identical**. The 1 "diff" (`expr ((1+2)*(3+4))`) is **excluded as oracle-nondeterminism, NOT a Laertes bug**: the original C `expr` is *order-dependent* (returns `[25]` when the record runs first, `[]` when any shorter record precedes it — a global-state/buffer leak in lil's own `expr`, ASan/UBSan-clean). When the C reference is self-inconsistent on an input, no divergence can be attributed. A new exclusion category alongside UB: **stateful-nondeterminism of the reference**. (The harness incidentally surfaced a latent lil-C bug — candidate for a separate finding.) Note Laertes's lil `laertes_init_*` targets are only `running`/`exit_code` scalars (harmless, already 0).
16. Laertes genann: **certificate** — 200k records, 0 diffs, **default cached-sigmoid path** (exercises the `lookup` table). genann's `laertes_init_lookup` is *also* uncalled (same pattern as the bzip2 CRC bug!), but here it is **harmless** because `genann_act_sigmoid_cached` retains its runtime lazy-rebuild (`if !initialized { build }`) — the table repopulates on first use. The precise distinction from #14: the uncalled-init bug is fatal only when the zeroed static is the *sole* source of the value (bzip2 CRC: no rebuild) and harmless when a runtime rebuild exists (genann lookup). Demonstrates the finding is mechanism-specific, not a blanket "Laertes zeroes tables" claim.
17. Laertes qsort: **certificate** — exhaustive small arrays (len 0–3 over [-4,4]) + the `[5,0]` C2SaferRust-bug trigger + 50k fuzz = 50,827 records, 0 diffs. **Laertes did NOT reproduce the C2SaferRust qsort bug**: it kept `low/high/i` as signed `i32` (`i = low - 1`), where C2SaferRust's `int→usize` rewrite broke the sentinel. Same function, opposite outcomes across tools — the cross-tool contrast qsort was chosen for.
18. CROWN quadtree: ran CROWN's own pipeline (preprocess→analyse→rewrite `--force-box`, 444 lines rewritten, builds). base-c2rust-vs-CROWN differential over insert/search op-sequences: **0 diffs on every non-crashing input**. Clustered/deep inputs cause a **shared crash** (base c2rust segfaults identically — a pre-existing c2rust/quadtree deep-recursion artifact, NOT CROWN-introduced). Corroborates the earlier PtrTrans quadtree certificate (#3). Scratch `cq_ws/`, `cq_base/`.
20. CROWN qsort: **certificate** — qsort is NOT in CROWN's benchmark set, so we **fed it to CROWN ourselves**: repackaged the existing base c2rust qsort into CROWN's crate format (lib.rs entry + `src/` + `strict_provenance`/`raw_ref_op` feature flags), ran preprocess→analyse→rewrite (16 lines rewritten: `(*a)` parens + `offset` call rewrites; kept raw-ptr sigs + i32). 50,827 records vs C, 0 diffs. **Confirms the recipe: any C lib can be fed to CROWN via `C → c2rust → CROWN`** (CROWN is a Rust→Rust safety lifter, not a C→Rust translator). Did NOT reproduce the C2SaferRust int→usize bug. Scratch `crown_qsort_ws/`.
22. Laertes urlparser: **UB-gate exclusion** — same as the CROWN column (#12); reconfirmed the `_FORTIFY_SOURCE` abort on the original `url.h url_parse`. Reference is UB → nothing attributable.
23. Laertes tulipindicators: **certificate (base-c2rust-referenced)** — 11 arithmetic indicators (sma/ema/wma/rsi/mom/roc/dema/tema/trima/wilders/zlema), 150,000 random price-series records, 0 diffs vs base c2rust. *Not full C-backed* — tulip's C source isn't in the repo, so base c2rust is the reference (mechanical baseline, validated faithful elsewhere). The 2 grep-flagged "zeroed tables" are **test-file scratch buffers** (`buf:[c_char;1024]`), not const lookup tables → no headline-#4 risk here.
25. PtrTrans lil (gpt-5.1, Trans_PA): **compile-fail** (clean rerun, not the earlier session-limit artifact). **70/121 code units exhausted 5 repair attempts → empty-body stub-reverts** (the `fnc_*` command family — the interpreter's tightly-coupled builtins — mostly unfixable), and the assembled crate **does not compile: 116 errors, syntax-level** (`expected type, found >`, malformed `>>` generics) that PtrTrans's own repair loop could not resolve. `unimplemented!()` = 0 (unlike genann — here the LLM *attempted* the bodies but emitted broken type syntax). Interpreter-scale mutual recursion + heavy pointer reshaping defeats PtrTrans on lil — a "faithful-or-fail → fail" outcome; no runnable artifact to differential-test. Scratch `PA_trans_projects/lil/`.
26. PtrTrans bzip2 (gpt-5.1, Trans_PA): **compile-fail** — same class as lil. Pipeline rebuilt post-reboot (permanent `tools/frameworks/ptrtrans_rebuild/`); ran end-to-end. **55/78 stub-reverts** (71%, worse than lil), and the assembled crate **fails to compile with 73 module-assembly errors** (E0255×30 name-redefined, E0432×19 unresolved-import, E0252×17, E0428×4 — shared c2rust types `Bool`/`UChar`/`Int32`/`BzFile` emitted in multiple modules with no shared `common` mod; PtrTrans's dedup/assembly step failed, repair loop couldn't fix). `BZ2_bzBuffToBuffCompress` WAS translated with the `Option<&mut [u8]>` reshaping (same call-site-risk shape as cJSON's parse_string) but can't run → no differential. **bzip2 now breaks ALL FOUR non-mechanical tools** (Laertes CRC-zero / C2SaferRust NULL-empty / CROWN corrupt+unsafe / PtrTrans compile-fail) — four distinct failure modes; only mechanical c2rust faithful. Two PtrTrans-internal path bugs patched to reach the LLM stage (circular `pub mod lib`, struct-path FileNotFound) — tool bugs, not translation content. Evidence: `rq1_bugs/bzip2_ptrtrans/`.
29. ★ PtrTrans qsort (gpt-5.1, Trans_PA): **the sort that doesn't sort** — headline-class semantic bug. Corpus extension (qsort NOT in PtrTrans's shipped dataset; same our-extension caveat as bzip2). All 3 units translated, crate **compiles cleanly and passes PtrTrans's own cargo-check gate** — but **34,012/50,000 = 68% of UB-free random arrays come back UNSORTED** (whole batch ASan+UBSan-clean on the C side; every diverging output fails the sortedness check; minimal repro `[3,1,2,5,4]` → `2 5 3 1 4`). Mechanism: the ptr→slice reshaping rewrites `swap(&arr[i],&arr[j])` via `split_at_mut(j)` but computes the second index as `right.get_mut(j-i)` — that's element `2j−i`, not `j` (correct is `right[0]`); repeated in the post-loop swap. The defensively-designed `swap(Option,Option)` **no-ops on None**, so out-of-range wrong indices are silently swallowed — no panic ever. Fuzz-Rust-alone finds nothing; one differential run exposes it on 2/3 of inputs. Same reshaping-contract class as cJSON parse_string (#5), distilled to 30 LOC. Evidence: `rq1_bugs/qsort_ptrtrans/`.
28. PtrTrans lodepng (gpt-5.1, Trans_PA): **compile-fail** — third instance (after lil #25, bzip2 #26), and the sharpest contrast in the table: lodepng is the library where BOTH mechanical c2rust AND CROWN hold clean C-backed certificates (✓F 3,036 codec round-trips), yet PtrTrans cannot assemble a compiling crate. All 271 units translated (255 persisted); **241/255 = 95% ended Compile_Failed** (Fixed_5 ×176 = repair budget exhausted); final crate: **363 module-assembly errors** (E0252 ×177 dup-imports, E0255 ×65 name-redefined, E0428 ×51 dup-defs, E0432 ×46 unresolved `LodePNGCompressSettings`-class forward refs). NOT translation refusal — only 5 `unimplemented!()` (vs genann's decl-only core); the LLM wrote bodies, PtrTrans's dedup/assembly step failed, same signature as bzip2. Scale cliff confirmed: the three biggest tightly-coupled libs (lil 3.7k / bzip2 7.3k / lodepng 6.6k) ALL compile-fail. One more PtrTrans-internal bug patched to complete the run (None `usage_paths` crash in `obtain_SA_result` on `uivector`, + a resume trap where the setup-time cargo gate `exit(1)`s forever after a mid-run crash) — tool bugs, not translation content. Evidence: `rq1_bugs/lodepng_ptrtrans/`.
27. PtrTrans urlparser: **UB-gate exclusion** — library-level, same gate as the Laertes (#22) and CROWN (#12) columns. Reconfirmed under ASan on our rebuilt setup: the original `url.h url_parse` heap-buffer-overflows in `get_part` (`sscanf` at url.h:208, WRITE of size 10 into a 1-byte malloc) on the *first, entirely normal* URL (`http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash`). The C reference is UB on every URL through the primary parse path → no Rust divergence is attributable and there is nothing UB-free to certify. Not run through the LLM pipeline: the exclusion is library-level (the reference oracle can't execute), so it holds regardless of translator — no gpt-5.1 cost. (The one UB-free function C2SaferRust flagged, `url_is_ssh` → bug #2, is a *different* function; PtrTrans's slicer targets the pointer-heavy parse core, which is exactly the UB region.)
21. CROWN optipng: fed via the same recipe (base c2rust builds under nightly-2023-01-26 with the crown feature flags; preprocess OK). **CROWN CRASHES in the `analyse` phase** — ownership inference solves 1087 functions then hard-aborts: `assertion failed: fitter.next().is_none()` at `crates/analysis/src/ownership/infer.rs:658` on `png_create_png_struct` (signature solves at precision 1–2, trips the precision-3 pass). `analysis_results/` empty → rewrite can never run; **no `--no-attempt` escape** (panic is in the solver, not per-function rewrite). Parallel to cJSON's rewrite crash but **one phase earlier**. → **the two largest/most realistic libraries both defeat CROWN's pipeline** — cJSON (3.2k recursive parser) in *rewrite*, optipng (~95k codec bundle) in *analysis*; CROWN's certificates are all on smaller, regular crates. Evidence: `rq1_bugs/optipng_crown/` (commit 47882c3).
24. PtrTrans genann (gpt-5.1, Trans_PA): **declaration-only core** — the pipeline completed with 0 stub-reverts and the crate builds, BUT the core functions (`genann_run`, `genann_init`, `genann_train`, `genann_copy`, `genann_read`) are all `unimplemented!()` panic stubs: PtrTrans's slicer fed the LLM only the **header declarations**, never the .c bodies ("No implementation provided in the C snippet"). The forward pass was never translated — a *visible* failure mode distinct from cJSON's exhausted-repair stubs (here the tool never even attempted the bodies). What WAS translated: activation leaves — `genann_act_sigmoid` **faithful (200k/0 bit-exact)**; `genann_act_sigmoid_cached` diverges on 79% of in-domain inputs with **max |diff| = 9.2e-4** (LLM used `interval=30/4095` where C uses `30/4096` table stride + reciprocal-interval indexing) — an approximation-constant drift, below bug threshold but a nice example of silent numeric reshaping. Method note: an initial "100% garbage" reading was OUR harness's fault (byte-order decode + missing `genann_init_sigmoid_lookup` call + corpus realignment) — caught by single-input reproduction before it could be misreported; the discipline cuts both ways. Scratch `ptrtrans_genann/`. (lil resolved separately in #25; **bzip2 PtrTrans still pending** — the interrupted run left a self-referential lib.rs skeleton; rerun in progress.)
19. ★ CROWN lodepng: ran CROWN's pipeline (preprocess→analyse→rewrite `--no-attempt bpmnode_create|uivector_resize`, **5689 lines rewritten**, builds — did NOT crash like cJSON). **C-backed certificate**: original lodepng.c ≡ base c2rust ≡ CROWN, **3036 diverse images (solid/gradient/random/patterns, 1×1–64×64), encode32+decode32 roundtrip, 0 diffs**. C oracle from crown_dataset lodepng.c (commit 997936f). CROWN CAN faithfully lift a 6.6k-LOC PNG codec — sharpens the per-library story: CROWN faithful on lodepng (big codec) yet broken on bzip2 (also a codec). Scratch `cl_ws/`, `cl_base/`, `cl_oracle.c`.

## Current totals (filled cells only)

- **Bugs: 6 crash + 11 semantic-diff + 0 hang**, across **4 published tools** (C2SaferRust, PtrTrans,
  CROWN, Laertes), classes: **checksum-corruption — now 3 independent instances** (C2SaferRust crc32,
  Laertes bzip2, Laertes optipng); NULL/empty conflation; UTF-8-panic; call-site contract loss; CROWN
  ownership-lift breaking a codec (corrupt output + memory-unsafety); **PtrTrans qsort wrong-index
  reshaping — the sort that doesn't sort (68% of inputs unsorted, zero panics)** (#29)
- **Checksum corruption is the dominant class**: 3 instances across 2 tools. Laertes's uncalled-init
  mechanism zeroes any no-rebuild lookup table (bzip2 CRC, optipng zlib CRC); C2SaferRust's slice-lift
  conflates empty/NULL (crc32/adler32). **optipng `crc32_z` is broken by BOTH tools, differently** —
  one function, two independent silent bugs.
- **Certificates: 16 cells** (genann full 4-lifter clean row; Laertes qsort/lil/genann/tulip faithful;
  **CROWN lodepng C-backed cert — faithfully lifts a 6.6k-LOC PNG codec**; CROWN quadtree faithful)
- **CROWN column now complete**: broken on bzip2 (headline #3) + cJSON (rewrite crash); faithful on
  lodepng/genann/lil/quadtree; urlparser UB-excluded; tulip minimal-surface. **Per-library, not per-tool.**
- **Tool failures: 7** (CROWN on cJSON *rewrite* + optipng *analyse*; SACTOR on cJSON; C2SaferRust
  env-blocked; **PtrTrans compile-fail ×3: lil, bzip2, lodepng** — scale cliff: its three biggest
  tightly-coupled targets all fail assembly); **3 UB-gate exclusions** (urlparser × CROWN, × Laertes,
  × PtrTrans — same UB C, library-level gate); **1 oracle-nondeterminism exclusion** (Laertes lil)
- **PtrTrans column now complete (8/8 C-backed cells)**: quadtree ✓F / cJSON ▲+s:3 (headline #2) /
  genann ▲decl-only / qsort **s:1 — the sort that doesn't sort** (#29) / lil ✗ / bzip2 ✗ / lodepng ✗ /
  urlparser ⊘. Its only two runnable nontrivial artifacts (cJSON, qsort) BOTH carry silent semantic
  bugs of the same reshaping-contract class.
- **Scale defeats CROWN's pipeline**: the two largest/most realistic libs both crash it — cJSON (3.2k
  recursive parser, rewrite) + optipng (~95k codec bundle, analysis). Its certificates are all smaller,
  regular crates (lodepng/lil/genann/quadtree/qsort). "Real-world code at scale defeats the safety lifter."
- **Cross-tool contrasts locked**: qsort (C2SaferRust crashes / Laertes+CROWN faithful / **PtrTrans
  doesn't sort** — 3 tools, 3 outcomes on 30 LOC); bzip2 (all 4 non-mechanical tools broken, 4 different
  mechanisms); genann (all 4 faithful); lodepng (c2rust+CROWN certified faithful / PtrTrans can't compile)
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
| ~~P2~~ | ~~PtrTrans column~~ **DONE 2026-07-06** — column complete (8/8 C-backed cells): urlparser ⊘UB, lodepng ✗compile (#28), qsort **s:1 headline** (#29) | spent | prediction confirmed: the reshaping-contract class produced the qsort headline |
| **P3** | SACTOR column: urlparser / quadtree / genann / lil / lodepng | LLM $ + known fragility | likely ✗ or ✓ (faithful-or-fail) — fills the spectrum |
| P4 | c2rust lodepng base + tulip triage (7 utf8 sites) | low | mechanical baseline + cheap crash confirmations |

Alternates if a row underperforms: buffer (605, CROWN bench + crown_dataset), rgba (tiny), snudown
(markdown; C2SaferRust uses safe strings — likely all-certificate row), grabc.
