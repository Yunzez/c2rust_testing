# RQ1 — OOP differential harness: coverage & soundness on the c2rust baseline (2026-07-02)

Out-of-process (OOP) differential testing: each side is a normal program, so ANY translation /
signature works. The C oracle is BOTH the UB gate and the Rust-vs-C attribution. This note reports
(a) how much of a real corpus the value-oriented harness generator covers, and (b) the soundness
keystone — the false-positive rate on FAITHFUL c2rust translations, where the correct answer is
"no bug".

Corpus: CRUST-bench CBench, transpiled with Linux-native c2rust 0.22.1 (47 single-TU pairs, **654
extern-C boundaries**; `scripts/transpile_crustbench.py`). Faithful c2rust is the TN control: any
reported divergence on it is a false positive (or, in principle, a real c2rust bug — none here).

## Keystone result

> Across **126** value-oriented boundaries that fuzzed in **~25** faithful-c2rust programs,
> **0 false divergences** — after the soundness census surfaced and we fixed **4 distinct
> false-positive classes** in the harness generator. (Confirmed on the clean re-run: 126 TN,
> 0 DIV; the earlier run's single divergence, `NandC::print_add_bit`, was FP class #4.)

The "0 false positives on faithful translations" property is what licenses treating a divergence on
a NON-faithful translation (SACTOR/CROWN/C2SaferRust, Step 3) as a real-bug candidate.

## The soundness census is a validation loop, not a formality

Running the generated harness against faithful c2rust and demanding TN caught four real
false-positive classes — each a way the two sides could disagree WITHOUT a translation bug. All
fixed and re-verified:

| # | class | trigger | fix |
|---|-------|---------|-----|
| 1 | buffer alloc asymmetry | read past `len`: C `[cap]` stack array reads valid garbage, Rust tight `Vec` wild-reads | oracle `malloc`s EXACT len + poisons zero-len allocs → C traps too, gated before Rust runs |
| 2 | ASan report stall | any UB input | `symbolize=0` fast-abort env + oracle subprocess timeout |
| 3 | side effect / nondeterminism | `fs_c::fs_mkdir` (mkdir syscall): C creates dir, Rust sees EEXIST | **determinism gate** — re-run oracle; if it disagrees with itself, boundary is not a pure function → skip |
| 4 | stdout pollution | `NandC::print_add_bit` (pure printf): oracle stdout = printf + serialization | isolate callee stdout to /dev/null; serialize only the computed value |

The determinism gate (3) is the conceptual core: it makes "value-oriented" a RUNTIME-CHECKABLE
property (pure function of input), not a signature guess. Combined with the UB gate and stdout
isolation, only a self-consistent, side-effect-free C oracle can ever convict Rust.

## Artifact A — coverage decomposition (why each boundary is in/out of scope)

| category | boundaries | harnessable? | note |
|---|---:|---|---|
| **value-oriented, fuzzed** | 126 | yes | scalar / buffer / NUL-string / POD-struct; 0 false divergences |
| value-oriented, translation won't build | 24 | yes (blocked) | c2rust long-double→`f128` crate (16, precision-incompatible anyway), `static inline` body missing (3), etc. — transpile-completeness, not harness |
| non-POD / recursive / opaque struct | 361 | no | pointer-graph internals; out of scope for value differential |
| pointer / handle return | 44 | no | comparing addresses is meaningless |
| unsupported pointer target | 37 | no | `void*` / opaque pointee |
| `T**` / pointer-to-pointer | 21 | no | pointer graph |
| other param type | 10 | no | — |
| callback (fn pointer) | 6-8 | no | higher-order (±2 libclang parse variance) |
| struct-by-value return | 5 | no | (future: could serialize) |
| linker / target-feature (popcnt) | 9 | no | build env; pointer-graph libs anyway |
| float buffer | 2 | no | (future: per-elem bit handling) |

**Value-oriented support ≈ 150/654 = 23%** (scalar/buffer/NUL-string/POD-struct; ±2 boundaries
run-to-run from libclang parse variance). Adding
struct-array + string-table roles would recover only 7 more boundaries → 23% is the value-oriented
CEILING on this (pointer-graph-heavy) corpus. This is CORPUS FIT, not a fixable tool deficiency:
~65% of boundaries are pointer-graph / opaque / higher-order, out of scope for value differential
by construction (and NOT a false-positive source — they are gated out, never fuzzed).

## Artifact B — coverage is bimodal by program family

| program family | example programs | coverage |
|---|---|---|
| value-oriented numeric / codec / hash / string / date | approxidate 25/28, skp 16/69*, libwecan 12/12, gorilla-encode 9/22, morton, murmurhash, leftpad | high on the value boundaries |
| pointer-graph data structures | cJSON, roaring-bitmap, btree-map, bostree, mvptree, hamta | ~0 (out of scope) |

(*skp/gorilla totals include many pointer-graph boundaries excluded; the value subset is fully
covered.)

## Honest scope statement (for the paper)

On constructible, value-oriented boundaries the OOP harness finds real semantic divergences with
**zero false positives on faithful translations** (126 boundaries, 0 FP); pointer-graph / opaque /
higher-order boundaries are systematically identified and excluded (a coverage limitation, not a
false-positive source); a further 24 value-oriented boundaries are blocked by c2rust
transpile-completeness (long-double/f128, static-inline). RQ1 does NOT claim arbitrary C→Rust
verification, nor complex heap-object equivalence.

## Provenance

- `results/rq1_crustbench/oop_soundness_census.json` — per-boundary status + fuzz verdict.
- Generator: `tools/stu_selector/gen_oop_harness.py`; census: `scripts/oop_coverage_census.py --fuzz`.
- Fixes committed: 1b232c6, b3d87ac, ee9a752, 1715a60, + stdout-isolation (this batch).
- Confirmed on the clean re-run: **126 TN, 0 divergences** (the earlier run's single divergence,
  print_add_bit, was FP class #4, now fixed).
