# RQ1 coverage / scope plan — claim boundaries, not coverage-chasing (2026-07-02)

Converged plan (Codex framework + Claude's 4 reinforcements). The point is NOT to maximize
harness coverage; it is to **write the RQ1 claim at the right boundary** and back it with
measured evidence. Value-based differential testing has a natural domain (value-oriented
boundaries); pointer-graph data structures are out of scope and are *systematically identified
and excluded*, so they are a coverage limitation, not a false-positive source.

## The core reframe (why this is not a fatal weakness)

Complex-type coverage ≠ false-positive risk. The "no false positive" property only concerns the
boundaries we actually fuzz (the harnessable set). Un-harnessable complex boundaries affect
coverage / recall upper bound / external validity — NOT harness soundness, UB-gate precision, or
"is a reported bug real". So the honest RQ1 claim is:

> On constructible, value-oriented boundaries we find real semantic divergences with low false
> positives; complex pointer-graph boundaries are out of current scope and are systematically
> gated out (hence a coverage limitation, not a false-positive source). Coverage is quantified
> and reported per program family.

RQ1 does NOT claim "we can verify arbitrary C→Rust programs" and does NOT try to prove complex
heap-object equivalence (that would drag in pointer-graph canonicalization, ownership, allocator
identity, aliasing, recursive-structure normalization — a separate research program).

## Measured facts (established today)

- CRUST-bench c2rust baseline: 41/100 programs transpile single-TU. **654 real translated
  function boundaries** (`extern "C" fn NAME(...)` definitions) — this is the correct denominator.
  NOTE: `c2rust_baseline_report.json`'s `extern_c_fns` field says 795, but that is an OVER-COUNT:
  it used `text.count('extern "C" fn')`, which also matches ~141 function-POINTER TYPE annotations
  (e.g. `type f = unsafe extern "C" fn(f32)->f32;` and callback fields) that are NOT callable
  functions. 795 = 654 real fns + 141 fn-ptr types. All coverage numbers below use 654.
  Recipe: `scripts/transpile_crustbench.py`.
- **In-process** (mature emitter) constructibility on the 654 boundaries: **211/654 = 32%**.
- **OOP v1** (scalar/buffer/out_scalar only): **123/654 = 18%** (`results/rq1_crustbench/oop_coverage.json`).
- Unsupported (SHARED root limits, both harnesses): nonPOD_struct 194 + recursive struct ~162 +
  void* 37 + callback 6 + ptr_ptr 6 + T** 11 ≈ **~65% pointer-graph / opaque / higher-order**.
- **~32% is the ceiling** for value-differential on this corpus, and it reflects CORPUS FIT
  (CRUST-bench is pointer-graph-heavy), not a fixable tool deficiency.
- Coverage is **bimodal by program family**: value-oriented → ~100% (Math-Library 16/16,
  NandC 12/12, approxidate 27/28, libwecan 12/12, morton 3/3, murmurhash 1/1); pointer-graph
  data structures (cJSON, roaring, btree, tries) → ~0.

## "value-oriented" — outcome-independent definition (avoid circularity)

Define by a PROPERTY, then show coverage is high on it — do NOT define it as "what we can test":

> A boundary is **value-oriented** iff every parameter and the return are value-representable:
> scalars, contiguous buffers (`T*`+len), NUL-terminated strings, POD structs/arrays — with no
> pointer-graph internals, aliasing, opaque (`void*`) state, or callbacks.

## The 4 reinforcements (turn the argument into evidence)

1. **fuzz-soundness census = the empirical keystone (MUST run).** "Gated ⇒ not a FP source" is an
   assertion until we show the SUPPORTED set has 0 false divergences on real faithful c2rust.
   Deliverable number: **"0 false divergences over N harnessable c2rust boundaries."** (Today's
   debugging found real FP risks — float bit-compare, buffer-len, NaN — so this must be measured,
   not assumed.) Run via `oop_coverage_census.py --fuzz` on the SUPPORTED set.
2. **OOP role-port must target IDIOMATIC shapes, not raw-ptr.** 32% was measured on c2rust
   (all C-ABI). The RQ1 bug hunt targets SACTOR's idiomatic output (`&mut [T]`, `&str`, `Option`,
   methods). Port the call side to idiomatic forms (in-process decode logic = reference, but the
   native Rust call must use slices/refs, not `*mut T`). Otherwise parity-on-c2rust ≠ coverage-on-SACTOR.
3. **value-oriented defined outcome-independently** (above) — put the property in the paper, then
   show the coverage, so it can't be read as circular.
4. **Recursion depth-guard** in the shared `describe_type`: recursive structs currently CRASH
   (`RecursionError`); a crash is not a clean exclusion. Make it return a clean UNSUPPORTED so
   "systematically identified and excluded" is literally true.

## Execution sequence

1. **Port OOP emitter roles** to ~value-boundary parity (~28–32%), targeting idiomatic shapes:
   (a) NUL-terminated string in/out (most common; currently mis-classified as out_scalar — e.g.
   utf8/leftpad), (b) POD struct by value / by pointer / struct return, (c) simple fixed
   array / table if the in-process reference ports cheaply. **Add the recursion depth-guard.**
   STOP at ~parity — do NOT chase pointer graphs.
   Explicitly NOT porting: recursive struct, non-POD struct w/ pointer fields, void*, callback,
   general T**, arbitrary pointer graph.
2. **Run the fuzz-soundness census** on the SUPPORTED set → produces BOTH (a) the validated
   coverage-decomposition table and (b) the "0 false divergences / N boundaries" number.
3. **RQ1 SACTOR bug hunt on the value-oriented high-coverage subset ONLY** (Math-Library, NandC,
   approxidate, libwecan, morton, murmurhash, codec/string/hash/bit-op SACTOR outputs, plus our
   existing value-oriented benchmark). NOT on cJSON/roaring/btree. State this as experiment-design
   ↔ tool-scope alignment, not avoidance. (LLM $ — needs user auth.)
4. **Do not pursue pointer-graph coverage.** Write it as explicit limitation / future work.

## Two paper artifacts to produce

**A. Coverage decomposition (3-layer):**
| category | harnessable? | reason |
|---|---|---|
| scalar / buffer / string / POD struct / arrays | yes | value-oriented boundary |
| struct return / NUL string | yes after OOP role port | emitter gap |
| non-POD struct / recursive pointer graph / void* / callback | no | out of scope |

**B. Coverage by program family:**
| program family | coverage |
|---|---|
| value-oriented numeric / codec / hash / string-scanning | high (~100%) |
| pointer-graph data structures | low (~0) |
| overall CRUST-bench | ~32% ceiling |

## RQ scope (locked)

- RQ1: on value-oriented harnessable boundaries, find real bugs (precision + recall)?
- RQ2: on faithful translations, does the UB-aware oracle cut false positives?
- RQ3: matcher under rename/restructure?
- RQ5: frontier picks the right layer?
- Pointer graphs = threats/limitations, NOT an RQ1 obligation.

## Current state / where things are

- Bugs in published translations: bug #1 (qsort/C2SaferRust, fuzzer-found, `results/rq4_effectiveness/bugs/`).
- OOP harness generator: `tools/stu_selector/gen_oop_harness.py` (scalar/buffer/out_scalar/float/
  bool/static-expose; commits 1adde22, 35085b7, + bug fixes). OOP skeleton `results/rq1_oop_skeleton/`.
- In-process generator: `tools/stu_selector/gen_diff_harness.py` (mature; has the roles to port from).
- Coverage census tool: `scripts/oop_coverage_census.py` (--fuzz for soundness run).
- Architecture (locked): OOP = general default; in-process = C-ABI speed specialization. extern-C
  is an in-process-FFI requirement, not a cargo-fuzz one. See memory rq1-realbug-hunt-state.
- Branch dataset-v2.1. Commits pushed by user.

## PROGRESS LOG (2026-07-02, execution)

**Step 1 DONE — OOP emitter ported to value-boundary parity (commits 1b232c6, b3d87ac, ee9a752).**
Added roles: NUL-string (`in_str`), POD struct by pointer (`in_struct`/`io_struct`) + recursion
depth-guard + opaque/incomplete-struct → clean non-POD. Coverage: **152/654 = 23%** OOP value
support on the c2rust baseline (was 18% v1 → 19% +strings → 23% +structs). Measured that adding
struct-array + string-table roles would recover only **7 more boundaries**, so 23% is the
value-oriented CEILING on this pointer-graph-heavy corpus → STOPPED porting, per plan.

**Two REAL false-positive bugs in the OOP generator caught + fixed (the census earned its keep):**
(1) buffer alloc asymmetry — C oracle used fixed `[cap]` stack arrays vs Rust's tight `Vec`, so a
read-past-len was silent in C but SEGV'd in Rust → false Rust divergence. Fix: oracle `malloc`s
EXACTLY len (ASan-symmetric) + **poisons zero-len allocs** (ASan does NOT flag `malloc(0)` reads,
verified, but Rust's dangling empty-Vec faults). (2) ASan symbolization stalled every UB input for
seconds → `symbolize=0` fast-abort env + oracle subprocess timeout (drain-on-thread). Validated TN
(no divergence): leftpad 10142, murmurhash 7963, tm_to_time_t 8223, date_time 7676 execs.

**Honest build-fail decomposition (67 fails):** 44 pointer/handle returns, ~7 struct-by-value
returns, ~8 opaque types (EVP_PKEY/BIGNUM), rest target-feature/linker — ALL pointer-graph/opaque,
out of scope; none are recoverable value-boundary emitter bugs.

**Step 2 DONE — fuzz-soundness census (commits 1715a60, f31219f). KEYSTONE: 0 false divergences /
126 value-oriented boundaries fuzzed on faithful c2rust (126 TN, 0 DIV).** Writeup +
artifacts A/B in `results/rq1_oop_soundness.md`; data in `results/rq1_crustbench/oop_soundness_census.json`.
Getting to 0 required fixing 4 FP classes the census surfaced: (1) buffer alloc asymmetry, (2) ASan
symbolization stall, (3) side-effect/nondeterminism (fs_mkdir) → DETERMINISM GATE, (4) stdout
pollution (print_add_bit) → callee-stdout isolation. Coverage ~150/654=23% (value ceiling; ±2 parse
variance); 24 more value boundaries blocked by c2rust transpile-completeness (long-double→f128,
static-inline) — orthogonal to soundness. Decomposition: 361 non-POD/pointer-graph struct + 44
pointer-return + 37 void* + 21 T** + ... all GATED OUT (coverage limit, not FP source).

**Step 3 NEXT — RQ1 bug hunt on NON-faithful translations (needs LLM $ auth).** The 0-FP-on-faithful
result licenses treating a divergence on SACTOR/CROWN/C2SaferRust output as a real-bug candidate.
Target the value-oriented high-coverage subset (approxidate, libwecan, Math-Library[if f64],
morton, murmurhash, gorilla-encode, skp, codec/hash/string). Triage: C-alone ASan+UBSan replay (C
clean + Rust diverges ⇒ candidate bug = qsort protocol). Bug #1 (qsort/C2SaferRust) already banked.

## One-line judgment

The problem is not "the tool is weak" — it's "the claim must not exceed scope." Bring the OOP
emitter to value-boundary parity (idiomatic-shaped), MEASURE soundness (0 FP / N), report coverage
as a layered measured limitation, run RQ1 only on the value-oriented subset, and declare
pointer-graph data structures out of scope. RQ1 proves bug-finding precision/recall on harnessable
value boundaries — not complex heap-object equivalence.
