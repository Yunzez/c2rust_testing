# tulipindicators × SACTOR (gpt-5.1): ✗(verify) — translates, but per-function verification can't link

**Verdict: `✗(verify)`** — a **process failure** at SACTOR's verification-link stage (the tool
delivers no *testable* Rust artifact), same class as the parse-fails but reached later. Settled by a
full paid run, 2026-07-10. **We did not patch SACTOR** — this is a genuine limitation of the
tool as shipped.

## What happened

tulip's C source isn't in the repo; we pulled it from upstream (TulipCharts/tulipindicators v0.9.2,
104 indicator TUs + `indicators.c` dispatch table + `utils/buffer.c`) and ran the whole pipeline
with a real test oracle (12 indicator outputs captured from the compiled C `sample` on the built-in
IBM data).

1. **Parse: PASS.** SACTOR's resolver ingests all 108 TUs, `unresolved=0`. tulip has no
   member-function-pointer *allocator*, so it clears the wall that parse-fails bzip2/optipng.
2. **Translate: the LLM did its job.** All 104 indicators were translated (unidiomatic phase).
3. **Verify: FAIL, 108/108 identically.** SACTOR verifies each function by embedding the translated
   Rust back into the C program and linking a per-function test harness. **Every link failed** with
   `Error: Failed to link project-level harness for function <name>`.

## Root cause (traced to the linker)

SACTOR's `build_link_closure` (`sactor/c_parser/project_index.py`) computes which C TUs to link for a
function's harness by walking **direct call edges only** (`function_dependencies`). tulip's
`ti_indicators[]` is a **function-pointer dispatch table**: it references all 104 indicators by
**address in a `.data` static initializer**, not by calling them. Those address-taken references are
invisible to the closure, so the harness links only `sample.o + indicators.o` and every indicator
symbol is undefined.

Reproduced by hand (`clang sample.o indicators.o -lm`):
```
/usr/bin/ld: indicators.o:(.data+0x10): undefined reference to `ti_abs_start'
/usr/bin/ld: indicators.o:(.data+0x18): undefined reference to `ti_abs'
...  (217 distinct ti_* symbols, all from the .data table)
```
`-lm` is already in SACTOR's link command, so this is **not** a missing-libm issue — it is the
function-pointer table.

## The function-pointer nemesis, three stages

Same construct that recurs across the SACTOR column, failing at a different stage each time:
- **member-fn-ptr allocator** `(*(s->fp))(...)` → **parse-fail** (bzip2 `BZALLOC`, optipng `ZALLOC`)
- **fn-ptr typedef callback** in a struct → **scaffold-break** (genann, lodepng)
- **fn-ptr dispatch table** `ti_indicators[]` → **verify-link-fail** (tulip) ← this cell

It is the sharpest evidence for the paper's thesis that **per-function verification ≢ whole-program
correctness**: SACTOR cannot even *assemble* a per-function test for a program wired through a
function-pointer table.

## Rigor note — a harness bug we caught first (do not repeat)

A first probe copied all 104 indicator `.c` files **flat** into one directory, breaking tulip's
`#include "../indicators.h"` relative includes → a *spurious* `USR=None` resolver error that would
have been mis-recorded as `✗(parse)`. Caught by checking every TU compiles under the exact
`compile_commands.json` before trusting the failure. Lesson: a broken include masquerades as a tool
limitation — verify the C builds first.

## Files
- `compile_commands.json` — correct layout (indicators/ subdir preserved)
- `probe_ingests.log` — the earlier $0 probe confirming clean ingestion
- (full run logs + `batch_summary.json` were in the scratchpad; the link command and 217 undefined
  symbols are reproduced above)

## Cost / decision
One full paid translation pass (104 indicators). Not re-run: the user's call was to report SACTOR's
limitation as-shipped rather than patch `build_link_closure` to make it link.
