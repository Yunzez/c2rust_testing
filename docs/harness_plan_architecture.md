# Harness Plan architecture

**Binding, 2026-09-04.** This supersedes every earlier description in which a "schema" was something
a person wrote. It is recorded because an afternoon was spent hand-authoring schemas, which is the
same thing as hand-authoring harnesses.

## A harness has exactly two parts

Every generator decision belongs to one of them.

### 1. Input processing

Turn fuzz bytes into two equivalent function inputs:

```
fuzz bytes  ->  logical values  ->  C representation + Rust representation
```

It owns: how scalars are decoded; how large buffers are; the relation between a pointer and its
length; output-buffer capacity; how structs are constructed; which parameters share or own memory.

### 2. Output processing

Turn the two post-states into comparable observations:

```
C post-state        Rust post-state
      |                    |
      +--> normalized observation records <--+
                      |
                   equality
```

It owns: the scalar return; an output buffer's valid length; mutated inputs; interior-pointer
offset; structured-object fields; lifecycle / free outcome.

## The pipeline

```
Matched Pair
     |
I/O Adapter Selection          (selects input adapters AND output adapters)
     |
Harness Construction
     |
Input Processing  ->  C/Rust Execution  ->  Observable-State Comparison
```

Internally:

```
C AST + function body + Rust signature
        |
        +-- analyze_inputs()        -> InputPlan
        +-- analyze_observations()  -> ObservationPlan
                     |
                 HarnessPlan        (generator IR, like compiler IR)
                     |
             Harness Construction
```

A Harness Plan names, per parameter and per observation, which adapter applies:

```json
{
  "boundary": "BZ2_bzBuffToBuffCompress",
  "inputs": [
    {"adapter": "buffer",        "param": "source", "length":   "sourceLen"},
    {"adapter": "output_buffer", "param": "dest",   "capacity": "destLen"},
    {"adapter": "scalar",        "param": "blockSize100k"}
  ],
  "observations": [
    {"adapter": "scalar_return"},
    {"adapter": "buffer_prefix", "param": "dest", "length": "destLen"},
    {"adapter": "scalar_output", "param": "destLen"}
  ]
}
```

## Rules

1. **The plan is generated, never written.** It is the generator's intermediate representation. It
   may be persisted for audit, but its only source is one unified analysis. Nobody writes it and
   nobody patches it.
2. **Adapters are fixed generator code** (buffer input adapter, structured-object output adapter,
   interior-pointer output adapter, ...). The plan only says which adapter applies where. Template
   selection is internal; there is no user-facing "template" or "schema" mode.
3. **Equality is one uniform differential property.** It is not configured per boundary.
4. **A boundary that cannot be planned failed harness construction.** That is a fact about the
   generator, reported as such — not an "unsupported schema", and never a reason to hand-write one.
5. **Buffer bounds come from one global generator policy**, not from per-boundary constants.
6. **Memory safety is the harness's job; business legality is the function's.** These are two
   different things and must not be conflated.

   > The harness must satisfy all memory-safety preconditions that it can derive. Rejection guards
   > may narrow the input domain, but the generator must not assume that invalid inputs are safely
   > rejected. If pointer validity, extent, or object invariants cannot be established, the C
   > execution must use isolated ASan+UBSan or harness construction fails.

   A function may reject a *business*-illegal value and the differential still holds, because both
   sides return the same error. It may equally not check at all and simply invoke UB — real C APIs
   routinely do. Observed, three times: cJSON's truncated `\uXXXX` reads out of bounds; a bzip2 array
   parameter with no length can be overrun; and the in-process UBSan-minimal gate does not see
   raw-pointer heap overflow at all. So a range may be narrowed only from a *provable* rejection
   guard in the entry's own body (a parameter compared against a constant, followed by an early
   return with no side effects), with the guard cited in the plan — and narrowing is never what
   establishes memory safety.
7. **A capacity is only as good as its source.** Admissible sources, exhaustively:
   - a length/capacity carried by a parameter;
   - a fixed extent provable *inside* the boundary under test;
   - a type-level constant that the API documents;
   - a size the harness itself allocates and passes in, from the global policy.

   An array extent at a *call site* is **not** admissible. A function may have many callers, each
   allocating a different size; one caller's `Int32 cftab[257]` is that caller's fact, not the
   boundary's contract. A bare pointer with no length may not have its capacity guessed from a
   caller — that is rule 6 territory, and the boundary either gets a policy-allocated buffer or
   fails construction.
8. **Ownership is derived or it is not claimed.** "`cJSON*` is freed by `cJSON_Delete`" may not be
   written down, per library or anywhere else — a per-library ownership declaration is a hand-written
   schema with fewer fields. It must come from the function body / call graph / allocator pairing and
   be verified, or be proposed by an LLM and confirmed by a deterministic verifier. If neither
   succeeds, the harness may not claim to observe lifecycle. Running the execution in a subprocess and
   letting exit reclaim memory is a legitimate way to avoid *leaking*, but it observes nothing about
   the free stage and must not be reported as lifecycle coverage.
9. **LLM assistance is the fallback after planning fails**, never the first move.

## Lowering (implemented 2026-09-04)

`tools/stu_selector/harness_plan.py` produces the plan; `lower_to_schema()` is its **only**
consumer and turns it into the parameter list the existing code emitters already take. The
lowering is not a file format and has no user-facing surface:

```
python3 tools/stu_selector/gen_diff_harness.py --pair <pair> --entry <fn> --plan [--plan-json p.json]
```

`--plan` reads no schema and writes none. A boundary whose plan is incomplete prints
`harness construction failed: <reasons>` and exits 2.

Adapters currently lowered: `scalar`, `bounded_scalar`, `length`, `capacity_ptr`, `input_buffer`,
`inout_buffer`, `output_buffer` (both the capacity-by-pointer and the capacity-by-scalar flavours),
and one uniform `plan_array` — a harness-owned allocation sized by a plan expression (a constant,
or a `usize` expression over already-decoded parameters), filled from the fuzz input exactly when
the callee reads it. An adapter with no lowering **raises**; it never falls back to a guess.

Two lowering rules that are easy to get wrong and are therefore written down:

* a **non-const** C buffer gets a separate allocation per side even when the C body never writes
  it, because the observation plan compares it and one shared allocation could not tell a write by
  the translation from a write by the original;
* a length parameter shared by several buffers is capped by the **widest** of them, so the global
  byte budget holds for every buffer it sizes.

## Acceptance

**A hand-written schema is not ground truth.** Diffing a generated plan field-by-field against one of
the old hand-authored schemas is useful for migration debugging only; it cannot be the acceptance
criterion, because the hand-written schema may itself be wrong.

Acceptance is:

1. deterministic plan generation;
2. signature / type validation against both sides;
3. the harness builds;
4. sanitizer-clean positive execution;
5. zero divergence on a faithful C/Rust translation;
6. a seeded negative mutation is caught by the comparator;
7. every inference in the plan carries a machine-generated evidence location.

## The mistake this replaces

`results/rq3_coverage/bzip2/c2rust/raw/make_schemas.py` carried **35 hand-written constants** over 10
schemas (`"bounded": (1, 9)`, `"cap": 257`, `"max_len": 258`, ...). Every one had a provenance
comment citing a line of bzip2 source. **Justification in a comment is not derivation**: a
hand-tuned JSON schema is a hand-written harness wearing a different hat, and it makes the
eligibility number a measure of how much was typed rather than of what the tool can do.

cJSON was closer to right — its parameters came from `_infer_abi()` unmodified — but its `return`
block was still authored by hand.

Both are to be replaced by generated plans. Some of the 35 constants are recoverable — guard-derived
ranges from the entry's own parameter validation, and input sizes from the global policy — but not
all of them, and the ones that are not recoverable are failures to report, not blanks to fill.
Several of the `cap` values were read off a caller's local array declaration, which rule 7 forbids;
those boundaries take a policy-allocated buffer or fail construction. Ownership (which function
releases a returned object) is a genuinely non-type fact and falls under rule 8.

## Addendum 2026-09-05 — `T**` buffer tables (tulip), and pointers the body advances

**Shape.** tulip's 104 indicator functions all take
`(int size, TI_REAL const *const *inputs, TI_REAL const *options, TI_REAL *const *outputs)`:
a table of input rows and a table of output rows, each row `size` doubles, the row count a fact of
the indicator (`inputs[0..3]` for `ad`, `outputs[0..2]` for `bbands`). Every translator in the study
keeps that C shape (`*const *const f64`, `*const *mut f64`), so this is a **C-side InputPlan
capability**, not a per-translator bridge: the planner refused all 104 with "T** is dereferenced and
written rather than indexed" because the only `T**` it knew was the `char** + count` string table.

**Rule (derived, never from a caller).** A `T**` parameter of scalar element type that the body
indexes **only by non-negative constants** is a *buffer table* with `max k + 1` rows. Each row is a
pseudo pointer parameter (`inputs__row0`) whose extent is derived exactly like any pointer's — the
body reaches it through the local that names it (`const TI_REAL *input = inputs[0];` makes `input`
an alias for row 0) or through a nested subscript (`outputs[1][i]`). Rows the body never touches get
the policy allocation, zero-filled. Rows never define a length parameter (a shared `size` stays a
bounded scalar, clamped by `extent_fits_allocation` and the loop-trip policy). A `char** + count`
keeps its string-table reading. The row count is capped by `max_table_rows`.

**Pointers the body advances — a soundness fix that this exposed.** `*output++ = v` was recorded as
a dereference, i.e. extent 1, for a pointer that the loop writes `size` times. Any pointer parameter
or row alias that is incremented, compound-assigned or reassigned is now `advanced`: its derefs and
subscripts are relative to a moving base and bound nothing, so the extent is unknown → policy
allocation (4 096 elements), with the loop-trip parameters clamped by `max_trip` (1 024) as the
recorded, unproven obligation. This applies to plain pointer parameters too (it was wrong for them
before; no earlier library hit the pattern).

**Lowering and generation.** `buffer_table` → one schema param carrying `rows: [{elems, fill,
written}]`. The generator allocates each row per side (fuzz-filled when the callee reads it), builds
one `Vec<*const T>` / `Vec<*mut T>` of row pointers per side, passes `.as_ptr()`, and after the call
compares every written row **bit for bit** (`to_bits()` for floats: `!=` would report NaN ≠ NaN as a
divergence both sides produced). The Rust bridge accepts only a pointer to pointers of the same
element type; a reshaped table (`&[&[f64]]`) is refused with the reason.

**Campaign parameter.** A table boundary decodes up to rows × 1 024 doubles; libFuzzer's default
`-max_len=4096` bytes would leave the rows almost entirely zero. `cell.py --max-len` (default 4 096,
tulip cells 65 536) is recorded in each cell's `campaign_params.json` and printed in RUN.md.

**Evidence.** tulip × c2rust planner: 109/213 → **213/213**; row extents: `inputs` 104 proven
(`size`) / 70 policy, `outputs` 31 proven / 83 policy (pointer-advanced). Golden regression
unchanged (26 entries) before the tulip cases were added.
