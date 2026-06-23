# Boundary census v1 — Stage A (static constructibility)

Unit = one matched C↔Rust function (a candidate STU boundary). Source: 18 programs in benchmark/pairs. **No build/fuzz** — pure libclang signature analysis + features.py. Raw: `dataset/boundaries_static.jsonl`.

## Totals

- **85 candidate boundaries** across 18 programs (vs 18 program-level entries before).
- **59 constructible** (69%) — a harness signature is supported.
  - 15 already `pub` in the Rust (callable as-is); 44 are `static` internals that would need exposing (`#[no_mangle] pub`).
- **26 not yet constructible** — the generator's hard-gate set.

## Constructibility breakdown

| outcome | n |
|---|---|
| SUPPORTED | 59 |
| UNSUPPORTED_PARAM | 16 |
| NO_C_DEFINITION | 10 |

## Hard-gate reasons (non-constructible)

| gate | n |
|---|---|
| struct_ptr_field | 11 |
| callback | 3 |
| array_value | 1 |
| struct_value | 1 |

> Stage B will build + fuzz the constructible subset (exposing `static` internals where needed) to attach validity labels: NO_DIVERGENCE_OBSERVED vs FALSE_DIVERGENCE (C-UB) vs … The C-UB boundaries are the first natural NEGATIVES for the validity model.

