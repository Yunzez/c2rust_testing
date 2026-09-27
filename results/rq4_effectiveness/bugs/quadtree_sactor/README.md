# quadtree × SACTOR (gpt-5.1): circular-deps refusal on the core TU

**Verdict: `✗(circular deps)`** — same failure class as SACTOR × cJSON. The tool refuses the core
`quadtree.c` at dependency-analysis time (pre-LLM for that TU):
`ValueError: Circular dependencies for functions is not supported yet`.

## The cycle
Textbook mutual recursion in the insert path — the heart of the data structure:
```c
split_node_()  →  insert_()     (quadtree.c:76)
insert_()      →  split_node_() (quadtree.c:109)   // and self-recurses at 110, 115
```
The 2026-07-02 round's patch unblocked SELF-recursion (a function listing itself as a dependency);
mutual recursion (A↔B) is a different, explicitly-unsupported path in SACTOR's topological translation
order — it cannot order A before B when each needs the other.

## What did translate
3/5 TUs — the leaf files with no cycles — completed unidiomatic translation cleanly:
`point.c`, `bounds.c`, `node.c` (15 functions LLM-translated and verified). But with `quadtree.c`
refused, the driver TU fails to link (`Failed to link project-level harness`) → **no runnable artifact,
no differential possible**. Faithful-or-fail → fail.

## Cross-tool contrast on this row
quadtree is PtrTrans's OWN shipped benchmark (✓F certificate #3) and CROWN lifts it faithfully (✓F*
#18). SACTOR — whose method requires a topological function order — is defeated by the same mutual
recursion both other tools handled. Combined with cJSON (recursive-descent parser, same refusal):
**SACTOR's translation-order prerequisite structurally excludes recursive cores**, independent of LLM
quality.

## Method note
Driver: deterministic LCG insert/search sequence over a fixed-bounds tree (12 cases, C-reference
outputs embedded in `test_samples.json`). Three parser-level local-copy adaptations (output-verified
identical, md5 on 3 seeds): `(*fp)(x)` → `fp(x)` indirect-call spelling (×3 sites — SACTOR's resolver
dies with `USR=None` on C's explicit-deref call syntax), `INFINITY` → `1e308` (SACTOR chokes on
`__builtin_inff`). These unblock SACTOR's *parser*; the circular-deps refusal afterwards is the tool's
own documented limitation.

## Files
- `batch_summary.json` — per-TU verdicts (3 success / quadtree.c circular / driver link-fail)
- `circular_error_trace.txt` — the refusal traceback
- `driver.c`, `test_samples.json`, `test_task.json`, `compile_commands.json` — the harness (durable copy)
