# lodepng × PtrTrans (FSE'26, gpt-5.1, Trans_PA): compile-fail

**Verdict: `✗(compile)`** — third PtrTrans compile-fail (after lil, bzip2). The full pointer-analysis
pipeline translated all 271 code units of the 6.6k-LOC PNG codec but the assembled crate **does not
compile**, so there is no runnable artifact to differential-test.

The contrast is the point: lodepng is the library where BOTH mechanical c2rust AND CROWN earned clean
C-backed certificates (✓F over 3,036 codec round-trips). PtrTrans — the most aggressive reshaper —
can't even assemble it.

## Numbers
- **255 code units persisted; 241/255 ended Compile_Failed** (95%):
  Fixed_5_Compile_Failed ×176 (repair budget exhausted), Fixed_1..4_Compile_Failed ×65,
  Compile_Success ×11, Free_Function ×3.
- **363 final compile errors, all module-assembly class** (same signature as bzip2's 73):
  E0252 ×177 (duplicate imports), E0255 ×65 (name defined multiple times), E0428 ×51 (duplicate
  definitions), E0432 ×46 (unresolved imports: `LodePNGCompressSettings` etc. — cross-struct forward
  refs its assembly never places), E0277/E0106/misc ×24.
- Unlike genann (decl-only), the LLM *did* attempt bodies: only 5 `unimplemented!()` + 15 explicit
  stub markers in the final crate. The failure is PtrTrans's dedup/assembly step, not translation
  refusal.

## PtrTrans-internal bugs patched to get this far (tool bugs, not translation content)
1. `obtain_SA_result` "Owning" struct branch crashed with `TypeError: 'NoneType' object is not
   subscriptable` when SVF's `usage_paths` is null (triggered by `uivector`); the adjacent "Borrowed"
   branch already had the None-guard — copied it over (main.py ~L719).
2. Resume trap: `translation_setup()` ends with a cargo-check gate that `exit(1)`s silently; after a
   mid-run crash leaves the crate broken, every resume dies at setup before the translation loop.
   Recovered by clearing the partial crate + metadata and re-running clean (SVF reports reused).
3. Stale `compile_commands.json` `directory` (`/home/yzq/...` from the shipped dataset) fixed to local.

## Method note
Same permanent pipeline as bzip2 (`tools/frameworks/ptrtrans_rebuild/`): SVF reports pre-generated
with our compiled `pa_func`/`pa_struct` on clang-14 IR of the macro-expanded source (drop
`-D_Float128` — the expanded lodepng.c typedefs `__float128` itself). gpt-5.1, ~40 min, 271 units.

## PtrTrans column now complete (7 cells, no ∅ left)
quadtree ✓F / cJSON ▲94/118+s:3 (headline #2) / genann ▲decl-only / lil ✗compile / bzip2 ✗compile /
lodepng ✗compile / qsort (see `qsort_ptrtrans/`) / urlparser ⊘(C-side UB, library-level gate).
Scale cliff: the three biggest tightly-coupled libs (lil 3.7k, bzip2 7.3k, lodepng 6.6k) ALL
compile-fail; the tool's runnable outputs are the small/mid regular libs.

## Files
- `compile_errors.txt` — full cargo output (363 errors)
- `generated_lib.rs` — crate root
- Full generated crate: `tools/frameworks/ptrtrans_rebuild/PtrTrans-C2Rust/Code_Package/dataset/PA_trans_projects/lodepng/`
