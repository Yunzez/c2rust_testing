# lil whole-program coverage probe (E3 gate test, 2026-07-07)

**Question this answers** (the E2↔E3 decoupling claim): per-function match recall on the
raw-LLM lil cell is 0.55 — does that cap what differential testing can *cover*? **No.**
Coverage is driven by the eval **entry point**, not by per-function pairing: the dispatch
loop reaches every handler regardless of whether the matcher paired it.

## Result (quick run: ~500 grammar scripts + 6 targeted seeds, ~15 min)

| metric | value |
|---|---|
| script-reachable functions executed | **122/122 = 100%** |
| `fnc_*` command handlers hit | **55/55 = 100%** |
| host-API-only functions (unreachable from scripts *by construction*) | 6 (`lil_callback`, `lil_set_data`, `lil_get_data`, `lil_arg`, `lil_error`, `lil_append_string` — their only mention in lil.c is their own definition; they are embedder API, covered by harness code not by inputs) |
| line coverage (lil.c, 2055 lines) | 62.6% |

**Paper sentence**: exact per-function matching on the LLM-restructured lil is 0.55, yet
whole-program differential fuzzing through `lil_parse` covers **100% of script-reachable
functions (all 55 handlers)** — matching quality and differential coverage are decoupled;
mismatched twins (fnc_inc/fnc_dec) are precisely the pairs a behavioral oracle separates
on the first diverging input.

## Setup
- `driver.c` — host: read script file → `lil_new` → `lil_parse(code, len, 1)` → free.
- `gcc --coverage -O0 driver.c lil.c -lm` (gcov; .gcda accumulates across runs).
- `gen_scripts.py` — grammar-based random scripts over the full stdcmd vocabulary
  (55 commands + vars + `$subst` + `[brackets]` + quotes + deliberate parse-garbage).
- `seeds/` — 6 targeted scripts for the tail: bounded `while`/`for` (NB **correct lil
  condition syntax `{$x > 0}`**, not `{expr $x > 0}` — the stray `expr` word makes the
  condition permanently truthy → infinite loop), `return`, `slice`, `enveval`, list ops.
- Census: `gcov -f lilcov-lil` → per-function "Lines executed" (function counted as
  covered if >0%).

## Gotchas (cost us an hour)
1. **Timeout kills erase coverage**: a script killed by `timeout` (SIGKILL) never flushes
   its .gcda — so hanging scripts contribute ZERO coverage, and the constructs they
   contain look uncovered. Fix: targeted bounded seeds, short timeouts.
2. **lil `while`/`for` condition syntax**: `{expr $x > 0}` loops forever (the word `expr`
   corrupts the expression evaluation into truthy); correct is `{$x > 0}`. Most of the
   random-batch hangs were this, in our own grammar.
3. Raw-LLM lil rust.rs does NOT compile (11 borrow errors incl. a genuine
   `Rc::make_mut(&mut f.clone())` lost-mutation logic bug) — recorded as compile-fail in
   rawllm_v1.json; this coverage probe therefore uses the C side + entry-point reachability,
   which is what the coverage claim is about. The differential oracle itself was exercised
   on lil in E1 (name-preserving translations).

## Rust-crate side (the side the user's argument is actually about) — 2026-07-07

**The claim to test**: matching is 0.55, but if entry-point fuzzing covers the whole
*translated crate*, low matching doesn't cap differential testing. Measured on the raw-LLM
lil translation (patched to compile: 11 mechanical borrow fixes in `rust_cov/`, policy =
**preserve the LLM's semantics exactly**, incl. its lost-mutation writes; the E2 matching
cell continues to use the unpatched original).

Same corpus (500 grammar scripts + 6 seeds) through `LilInterpreter::eval_string`,
`-C instrument-coverage` + llvm-cov:

| metric | value |
|---|---|
| crate functions covered | **112/117 = 95.7%** |
| `fnc_*` handlers covered | 54/55 |
| host-API-only (embedder calls them; definition-only in crate) | 3 (`set_callbacks`, `take_error`, `LilList::into_vec`) |
| dead code (LLM wrote it, never wired it) | 1 (`pop_env` — env restored via `self.env = saved_env` instead) |
| blocked by a translation DEFECT | 1 (`fnc_return`, see below) |
| ⇒ coverage of functions actually wired to the entry | **112/112 = 100%** |

### Bonus finding: a silent semantic divergence, minutes into the probe
```
func g {a b} {expr $a + $b}
print [g 3 4]        →  C: 7      raw-LLM Rust: (empty, no error, exit 0)
```
**User-defined functions are inert in the translation.** Root cause: `fnc_func` registers
the function body via `Rc::make_mut(&mut func.clone())` — mutating a temporary clone that
is immediately discarded. **This is exactly the defect the borrow checker flagged as
E0716** (2 of the 11 compile errors sit in `fnc_func`; same pattern in `register_function`
and `fnc_rename`). Under the semantics the LLM actually wrote, `func`/`rename` silently do
nothing: a **no-crash, wrong-answer divergence** — the class invisible to fuzz-Rust-alone,
caught immediately by C-vs-Rust differential probing. (It also blocks `fnc_return` from
ever executing, which is how the coverage census surfaced it.)

Narrative for the paper: *the borrow checker was pointing at a genuine logic bug; a
translation that "fixes" the error the way the LLM wrote it ships an interpreter whose
user-function feature is dead.* Compile errors in LLM translations are not just build
friction — some are the compiler catching real semantic defects.
