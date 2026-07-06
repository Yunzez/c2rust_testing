# lodepng × SACTOR (gpt-5.1): translate-fail — same FILE-typedef scaffold break as genann (probe-evidenced)

**Verdict: `✗(translate-fail)`** — same systematic scaffold failure as genann (`rq1_bugs/genann_sactor/`),
evidenced by a watchdogged probe and terminated before burning the budget on a doomed run.

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
