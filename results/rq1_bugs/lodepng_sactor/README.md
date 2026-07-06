# lodepng × SACTOR (gpt-5.1): translate-fail — repair loop cannot converge on a struct-heavy API

**Verdict: `✗(translate-fail)`** — confirmed TWICE, with the second run isolating the true cause.

## Run 2 (post scaffold-patch): the definitive evidence
After our typedef-closure patch fixed SACTOR's scaffold (validated on genann: 0/15 → 15/15), we reran
lodepng on the healthy pipeline. Result: **16 of the first 66 functions (24%) exhausted the 5-attempt
repair budget** before our cost circuit-breaker stopped the run (~$10 spent, projecting ~50 failures
across all 200 fns → no assemblable whole either way).

**Failure signature** (`postpatch_error_signature.txt`): E0599 no-method ×1039, E0117 orphan-impl
×492, format-nanny rejections ×82. All 16 exhausted functions
(`postpatch_exhausted_16fns.txt`) are struct-manipulating helpers (`lodepng_color_mode_init`,
`LodePNGText_init`, `readChunk_bKGD/pHYs/tIME/tRNS`, pixel converters): **gpt-5.1 persistently emits
idiomatic method/impl-block Rust for lodepng's 35+-struct API, which SACTOR's free-function scaffold
contract rejects; the repair loop feeds errors back and the model regenerates the same shape.** Not
a pipeline bug — a method-capacity limit at struct-heavy-API scale. Clean contrast: the SAME patched
pipeline took 15-fn genann to 15/15 (and exposed headline #6 there).

## Run 1 (pre-patch): the scaffold wall (historical)

## Probe evidence (`probe_attempt_pattern.txt`)
- 7 leaf functions with **no struct dependencies** (lodepng_malloc/free/memcpy/memset/addofl/mulofl/
  realloc) translated first-attempt — proving env, LLM, and harness all work.
- The **first struct-touching function** (`uivector_init`) immediately entered the retry loop with the
  identical input-independent error genann died of:
  `error[E0425]: cannot find type _IO_FILE in this scope` (×3 before termination) — SACTOR's scaffold
  emits `pub type FILE = _IO_FILE;` without defining `_IO_FILE` (lodepng.h's
  `lodepng_load_file/save_file(FILE*)` pull FILE into the typedef closure).
- The error is in the generated scaffold, not the LLM output → every one of the ~180 struct-dependent
  functions (of ~200 total) inherits it. genann — where the identical bug WAS run to exhaustion —
  ended 0/15 `MAX_ATTEMPTS_EXCEEDED`. Running lodepng to the same end would be ~1200 doomed LLM calls
  (~$40) to re-confirm a failure already sealed at function 8.

## Cross-tool contrast
lodepng is the row where c2rust AND CROWN both hold 3036-image C-backed certificates, and where
PtrTrans compile-fails (#28). SACTOR joins on the fail side — 2/4 faithful, 2/4 (the FSE'26-era LLM
pipelines) unable to produce an artifact.

## Files
- `probe_attempt_pattern.txt` — 7× attempt-0 passes, then uivector_init retry loop
- `systematic_IO_FILE_error.txt` — the repeated scaffold error
- `driver.c` (deterministic encode32→decode32 roundtrip, size+checksum), `test_samples.json`,
  `test_task.json`, `compile_commands.json` — harness (durable copy)
