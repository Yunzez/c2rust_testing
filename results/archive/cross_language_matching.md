# Name-independent C↔Rust function matching — results

Core novelty of the project: recover the correspondence between functions of an original C
program and its translation **without using names**. Today's translators (c2rust, CROWN)
preserve names, but an LLM transpiler renames — so a structural matcher is required. This
note is the consolidated evidence (the "cross-language mapping" section).

## Setup

- **Both sides analyzed by a real compiler frontend** (symmetric): C via **libclang**
  (`tools/stu_selector/c_analyzer.py`), Rust via **rust-analyzer**
  (`tools/stu_selector/analyzer/`). Each emits the same per-function record:
  `{name, line, signature, io-shape, ops, metrics}` + `raw_edges`/`indirect_calls`.
- **Matching uses names for NOTHING.** Names are revealed only afterwards to score the
  prediction. Ground truth = name equality, valid because the translations here are
  faithful, name-preserving c2rust output.
- **Corpus**: 4 small hand-built programs + **lil** — a 3167-LOC scripting-language
  interpreter we translated ourselves with c2rust 0.22.1 (**128 functions**), the real
  stress test.

## Signals (all rename-invariant)

| layer | signal | captures |
|---|---|---|
| type | **io-shape** | structural fingerprint of input/output types (ptr `*`, ref `&`, array `[T;N]`, struct `{fields}`), shared canonical leaf vocab (usize≡u64, char→i8, …) |
| size | **metrics** | cyclomatic, stmts, nodes, loops, max_loop_depth, derefs, allocs |
| operation | **opcode histogram** | per-function operator counts (`/` vs `%`, `+` vs `*`, …) — separates structurally identical, different-op twins |
| topology | **call graph** | who each function calls / is called by |

Per-function similarity `N(c,r)` = weighted blend of io-shape (Jaccard + exact), metrics,
opcode cosine, arity.

## Topology: graph matching, not local similarity

Per-function signals **saturate at scale** (see lil below): many functions share an
identical signature. The discriminator is *what each function calls*. We propagate
similarity over the two call graphs (IsoRank-style):

`S(c,r) = (1-α)·N(c,r) + α·topo(c,r)`, iterated to convergence (α=0.7).

Crucially `topo` is a **neighbor-set best-match**, NOT a cartesian average over all
neighbor pairs (which would dilute the one correct neighbor with every wrong pair inside a
homogeneous cluster):

```
dir(A,B)   = mean over a∈A of ( max over b∈B of S(a,b) )      # each callee finds its best correspondent
setsim(A,B)= ½·dir(A,B) + ½·dir(B,A)                          # symmetrized
topo(c,r)  = ½·setsim(callees) + ½·setsim(callers)
```
(1.0 if both neighbor sets empty; 0.0 if exactly one empty = connectivity mismatch.)
Final assignment is **optimal 1-1 (Hungarian)**.

## Ablation (accuracy = correctly recovered pairs)

| program (n fns) | per-fn, greedy | per-fn, Hungarian | +topology, greedy | **+topology, Hungarian** |
|---|---|---|---|---|
| safe_stats (4) | 100% | 100% | 100% | **100%** |
| div_mod (5) | 100% | 100% | 100% | **100%** |
| tinyexpr (29) | 100% | 100% | 100% | **100%** |
| bignum (27) | 92% | 100% | 92% | **100%** |
| **lil (128)** | **61%** | **80%** | **96%** | **98% (126/128)** |

Topology and optimal assignment are **complementary**: on lil each adds a large,
independent gain (61→96 from topology; 61→80 from assignment; together 98%).

## lil breakdown — why this is the headline

- **55 of 128 functions share ONE identical io-shape**: the `fnc_*` command handlers, all
  `(lil_t*, size_t, **value) -> *value`. Their metrics and opcodes are near-identical too.
- Per-function matching therefore resolves only to the **function-family** level: 61%, with
  79% of the misses staying *within* the homogeneous handler cluster (it permutes inside it).
- **Call-graph topology breaks the cluster**: each handler differs in which `lil_*`
  primitives it calls (`fnc_concat`→append/to-string, `fnc_while`→eval-in-a-loop). The
  primitives have distinct shapes (confident `N`), and propagation pins each handler by
  *what it calls* → 96–98%.

## Residual

After the full method, the **only** lil miss is one swapped pair: **`fnc_streq` ↔
`fnc_strcmp`**. They have identical signature, identical metrics/opcodes, AND identical
callee/caller sets — a **true semantic symmetry** the graph cannot split. Distinguishing
them needs a literal/constant-level signal (e.g. the different return constants), a planned
"signal C" — explicitly *not* the current bottleneck. (Greedy left 3 such swaps; Hungarian
showed 2 were assignment artifacts, leaving this single genuine symmetry.)

## Reproduce

```
# wrap a c2rust crate (or use the lil crate), then:
analyzer <rust_crate_dir> --enable-metrics            > rust.json   # rust-analyzer side
c_analyzer.py --compile-commands <cc_dir> --enable-metrics > c.json  # libclang side
matcher.py --c c.json --rust rust.json                # default: topology + Hungarian
matcher.py --c c.json --rust rust.json --no-topo --greedy   # per-function baseline (ablation)
```
