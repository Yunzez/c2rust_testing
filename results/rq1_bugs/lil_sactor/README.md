# lil × SACTOR (gpt-5.1): circular-deps refusal — zero LLM calls

**Verdict: `✗(circular deps)`** — third instance of SACTOR's structural refusal (after cJSON,
quadtree). The interpreter TU `lil.c` is refused at dependency-analysis time:
`Circular dependencies for functions is not supported yet` — **0 LLM function calls, $0 verdict**
(the probe log has zero "Translating function" lines). The driver TU then blocks on the untranslated
`lil_new`.

lil's core is mutually recursive by design (the `fnc_*` builtin family ↔ `lil_parse` eval dispatch) —
the same 128-function tangle that stub-killed PtrTrans (footnote 25). SACTOR's topological
function-order prerequisite cannot admit it at all.

## Pattern across the SACTOR column
cJSON (recursive-descent parser) / quadtree (recursive spatial tree) / lil (recursive interpreter) —
all refused by the same check. **Any library whose core is a recursive algorithm is structurally
outside SACTOR's method**, independent of LLM quality or budget.

## Files
- `batch_summary.json`, `circular_error.txt` — the refusal record
- `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json` — harness (durable copy)
