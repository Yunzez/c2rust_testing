# tulipindicators × SACTOR (gpt-5.1): INGESTS — not a parse-fail; verdict is a paid run

**Status: `∅` (runnable, cost-gated) — NOT `—`, NOT `✗(parse)`.** Established 2026-07-10.

## Why this cell changed

The E1 table had `—` for tulip × SACTOR because **tulip's C source was not in the repo**. We pulled
it from upstream (TulipCharts/tulipindicators v0.9.2, 104 indicator TUs + `indicators.c` dispatch
table + `utils/buffer.c`) and ran the SACTOR resolver.

**SACTOR ingests it cleanly** — it parses all 108 TUs, completes project-index / dependency
analysis (only *warnings* about ambiguous struct owners for `ti_indicator_info` / `ti_buffer`, not
errors), and reaches the LLM translation stage (`Translating (unidiomatic) indicators.c`). So tulip
is **structurally different from bzip2/optipng**: those parse-fail on member-function-pointer
allocators; tulip has no such construct and gets past the wall that stops them.

Therefore the honest label is **`∅` (runnable, outcome unknown)**, not `—` and not `✗(parse)`.
Producing the actual translate/verify verdict costs real LLM budget (104 indicator functions + the
104-entry `ti_indicators[]` function-pointer dispatch table, which is the genann/bzip2 scaffold-risk
construct — so a translation-stage failure is *plausible*, but it must be measured, not assumed).

## Rigor note — a harness bug we caught first (do not repeat)

The FIRST probe attempt copied all 104 indicator `.c` files **flat** into one directory. tulip's
indicators live in an `indicators/` subdirectory and `#include "../indicators.h"`, so the flat copy
**broke those relative includes** — `gcc -c indicators/adxr.c` failed, and SACTOR's resolver then
reported a spurious `USR=None at adxr.c:74`. That would have been mis-recorded as `✗(parse)`. Fixed
by preserving the upstream directory layout (`indicators/*.c` with `-I.. -Iutils`); every TU then
compiles clean (`gcc` exit 0) and SACTOR ingests. **Lesson: verify the C compiles under the exact
`compile_commands.json` before trusting a resolver-stage failure — a broken include masquerades as a
tool limitation.**

## Files
- `compile_commands.json` — correct layout (indicators/ subdir preserved)
- `probe_ingests.log` — resolver completes, reaches `Translating (unidiomatic)` (killed immediately
  after to keep cost ≈ $0; one translation call had just started)

## Decision needed
Run the full tulip × SACTOR translation for a real verdict (est. paid; 104 fns) — or leave `∅`.
