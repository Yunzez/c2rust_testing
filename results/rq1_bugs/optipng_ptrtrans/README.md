# optipng × PtrTrans (Trans_PA): ✗(PA) — struct analysis exceeds a 2-hour budget (pa_func passes)

**Verdict: `✗(PA, >2h)`** — process failure, measured at **$0** (2026-07-10), with the budget stated
honestly: PtrTrans's required PA pre-pass does not complete on optipng within a 2-hour cap.

## Setup (all $0)

- Entry: `crown_dataset/optipng/` = flat multi-file entry (bzip2-entry precedent): 53 TUs — optipng
  proper + bundled zlib + libpng + pngxtern + gifread + pnmio + minitiff (demo TUs with their own
  `main()` excluded). Every TU compiles clean (`gcc` exit 0).
- IR: `clang-14 -S -emit-llvm` per TU — **53/53 succeed** — then `llvm-link` → 8.6 MB
  `linked_program.ll` (clean after dropping cexcept's two `example*.c` demos, which multiply-define
  `demo_throw`).

## Result

1. **`pa_func` COMPLETES** ("Analysis completed.", `func_analysis_report.json` produced) — SVF's
   function-level analysis handles the ~95 kLOC bundle.
2. **`pa_struct` does not terminate in 2 h** (exit 124), stuck in
   `Getting the usage scenario of the structure...` — the same struct-usage analysis that
   *crashes* on tulip's fn-ptr table, here grinding through libpng/zlib's large function-pointer-
   bearing structs (`png_struct`'s callback family, `z_stream`'s `zalloc/zfree`).
   No `struct_analysis_report.json` → Trans_PA cannot run.

## Honesty note on the timeout

Unlike tulip (hard crash — decisive), this is a **budget judgment**: 2 h on one core for a stage
that takes seconds on qsort. We label it `✗(PA, >2h)` with the budget explicit; a longer run could
in principle upgrade it. Context that supports the verdict: PtrTrans compile-fails at *assembly* on
lil (3.7k) / bzip2 (7.3k) / lodepng (6.6k) after full paid runs — optipng is 4–15× larger than all
three, so even if PA eventually finished, the downstream scale cliff stands between it and a
runnable artifact.

## The PA-stage pattern (both new cells)

Both PtrTrans PA failures center on **struct-usage analysis of function-pointer-bearing structs**:
tulip = crash on `ti_indicator_info` (the dispatch table), optipng = non-termination on the
libpng/zlib struct family. Combined with SACTOR's three fn-ptr stages, the construct is a
cross-tool nemesis, not a single-tool quirk.

## Files
- `pa_func_completed_tail.log` — successful completion
- `func_analysis_report.json` — the produced function-level report
- `pa_struct_timeout_tail.log` — where it was stuck at the 2 h kill
