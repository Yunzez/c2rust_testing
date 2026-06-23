# Independent label audit — Stage B dataset — 2026-06-23

To remove single-perspective bias, four **independent adversarial auditor agents** (which did not
build the pipeline) re-examined every `valid`/`invalid` label from the C source + generated harness
+ evidence JSON, with a mandate to *refute*. Partitions: 2 over the 13 `invalid`, 2 over the 33
`valid`. Verdicts are stored per-row in `dataset/boundaries.jsonl` (`audit` + `validity_v2`).

## Headline: 8 of 46 labels (17%) were unreliable — caught before training

| validity_v2 | n | meaning |
|---|---|---|
| **valid** | 28 | reliable positive (real inputs, function exercised, no divergence is meaningful) |
| weak_exclude | 5 | uninformative positive — drop from training |
| **invalid_intrinsic_ub** | 3 | solid negative — genuine C-UB at the boundary |
| invalid_isolation_invariant | 7 | **contested** — see below |
| excluded_generator | 3 | our generator mis-modeled the input — exclude + fix |
| excluded (build/gen fail) | 13 | return-type gaps (enum/pointer returns) |

## What the audit overturned

**3 "invalid" → `excluded_generator` (generator mis-modeling, both auditors' logic agrees).**
`hash_table::ht_insert_into`, `mergesort_search::merge_runs`, `mergesort_search::msort_range` take a
`T* + size` **array**, but the generator built a *single* element (a lone `Slot` / lone `i32`) with
`size` decoded independently as a garbage value → guaranteed OOB. The crash is **our bug, not a
boundary property**; labeling these "invalid" would have taught the model that array-taking
functions are invalid. → Fix: support `T* + len` arrays of scalars/structs (a real generator gap).

**5 "valid" → `weak_exclude` (uninformative positives).**
- `glob_match::match_at` / `match_class`: a **cursor/index** param (`p`,`t`) is decoded as a full
  random `usize` while the buffer it indexes is length 0–63, so the index is almost always
  out of range → the function early-returns and the real matcher is never exercised.
- `rpn_eval::read_le32`, `tiny_vm::read_i32`, `read_u16`: trivial fixed-size byte decoders; both
  sides read the identical buffer and do the same arithmetic, so agreement is guaranteed by
  construction — "valid" is true but carries no signal. (Auditor also *refuted* my prior worry that
  `const T*` is modeled as a NUL-scan string: it is a length-prefixed byte vector, correctly shaped.)

**28 "valid" confirmed real** (scalars, length-prefixed buffers, NUL strings, and the nested-pointer
cases — `matrix_*` int**, `word_tokens` char** — all built correctly with substantial work exercised).

## The contested class (your call): `invalid_isolation_invariant` (7)

`opcode_dispatch::{op_add,op_dup,op_mul,op_push,vm_pop,vm_push}`, `tiny_vm::vm_step` — internal
helpers taking a `VM`/`vm_state` struct built field-by-field from random bytes. The struct's `sp`
field is decoded unconstrained, but the function trusts `0 ≤ sp ≤ STACK_MAX` (it bounds-checks only
one side) → OOB. **The two auditors split:**
- **Auditor A (artifact):** the garbage `sp` is a state *unreachable in real execution* (the VM loop
  starts `sp=0` and only pushes 0–255); also `op_mul`'s ~1e9 overflow operands can never sit on a
  legitimately-built stack. → exclude.
- **Auditor B (real):** the function has a genuine *precondition* (`sp ∈ [0,64]`) it never checks; a
  boundary whose input invariant the type doesn't express is genuinely risky to differential-test. →
  a real negative the model should learn.

Both are right about different things. The reconciling reading: these are **internal helpers
testable only *through their caller*** (the caller establishes the VM invariant) — so they are
invalid for *isolated* differential testing, which is *exactly the STU boundary-selection signal*:
prefer the enclosing boundary (`run_program`, valid) over the internal `vm_push` (invalid in
isolation).

**Decision (project lead):** keep them as a **distinct negative subclass**
`invalid_isolation_invariant`. They count as negatives for the binary `P(valid|x)` label, but are
tagged so the "needs-caller-context" phenomenon stays separable from intrinsic-UB negatives.

## Binary label for the model

- **positive (valid):** `validity_v2 == "valid"` → 28
- **negative (invalid):** `invalid_intrinsic_ub` (3) + `invalid_isolation_invariant` (7) → 10
- **dropped (not training data):** `weak_exclude` (5) + `excluded_generator` (3) + `excluded` (13) → 21

→ **38 labeled boundaries, 28:10.** Solid positives; negative class thin and must grow.

## Implication for the dataset

Clean now: **28 valid : 3 solid negatives** (+7 contested). The audit confirms the *positive* class
is solid but the *negative* class is thin and was partly artifactual — so a solid model needs (a) the
7 contested kept as a distinct negative subclass (if accepted), AND (b) more data: scale the
harvester to external programs and/or add the generator fixes (T*+len arrays) that recover real
boundaries. The pipeline now exists to do both.
