# Plan-guided seeding — plan (recorded 2026-09-10, before implementation)

**Status:** planned, not started. Runs after the tulip four-cell seeding runs, their combined replay and
confirmation are archived. No LLM; no change to the harness decoder, the comparison oracle or the confirmation
pipeline; one global deterministic policy, never tuned per application or on results; the 37 archived cells
stay the automatic baseline.

**Revision note.** The first version of this plan (2026-09-10, morning) specified a complete canonical Decode IR
covering all 16 decoder roles, with the harness emitter and the seed encoder both derived from it. Review the
same day (user + GPT): that reimplements infrastructure that exists elsewhere — bytes→typed values
(LLVM FuzzedDataProvider, Rust `arbitrary`), typed values→valid API state (Hopper, GraphFuzz), schema-based
structured input (libprotobuf-mutator) — and the only genuinely new need is "requested semantic value → the
existing harness's byte layout". So the scope is now a **partial, best-effort Seed IR**: lower what can be
lowered without guessing, fall back deterministically otherwise, and report how many boundaries got which.

## Terminology

- **plan-guided seed** — a seed whose targeted fields were placed at offsets computed from the InputPlan
  without guessing; bytes after the first opaque node are deterministic pseudorandom filler, so the seed is
  guided, not fully structured.
- **fallback seed** — the automatic baseline's own 64-byte default seed (`cell.py`: bytes 0x00..0x3F);
  a fallback boundary is therefore identical to the baseline.

## Why

tulip × c2rust: the automatic campaign reaches 3 197 / 9 298 regions (0.344; 3 180 of them from the default seed
alone, 17 from 600 s of fuzzing). Six grid seeds per boundary reach 8 409 alone and 8 595 after 600 s (0.924;
smoke suite 0.927); length-matched random seeds reach 3 602 (0.387), so the gain is legal control values.
Laertes reproduces it (3 169 → 8 563), C2SaferRust 3 167 → 8 393. The frozen tulip rule is
`results/rq4_llm_refinement/tulip/SEED_RULE.md`. A static census of the 37 archived plans finds boundaries with
a bounded scalar, an unbounded scalar or a small numeric array in every library (bzip2 16/19, lodepng 42/64,
optipng 70/128, cJSON 16/39, lil 13/51, genann 6/10, quadtree 7/17, urlparser 1/21, qsort 2/3).

## Design

### 1. Partial Seed IR

Six node kinds, lowered from a boundary's plan inputs in the harness's actual decode order:

```
Sequence(nodes)
Scalar(rust_type, width, little-endian, mapping?)   # bounded_scalar: value = min + raw.rem_euclid(max-min+1)
Repeat(node, fixed_count)                           # input_array with constant extent; buffer_table rows (fixed / policy extent)
FillBytes(length, deterministic_random)             # filler after the last targeted field
ZeroAllocate(shape)                                 # allocations that consume no fuzz bytes (zero-filled rows, output arrays)
Opaque(reason)                                      # everything else: strings, length-prefixed buffers, dynamic extents,
                                                    # structs, nullable / produced objects, pointer tables, capacity cells
```

**Order and the prefix rule.** The plan lowering sets `decode_scalars_first` (harness_plan.py, unconditionally), so
the generated harness decodes every fixed-width scalar before any buffer, then buffers, then plan arrays and
buffer tables last (`items_from_schema`). A targeted field is placed exactly iff every node before it is
`Scalar` / `Repeat` / `ZeroAllocate` — i.e. its offset precedes the first `Opaque`. Consequently every bounded and
unbounded scalar of every plan-lowered boundary is placeable; a small numeric array (rank 2) is placeable only
when no opaque buffer precedes it. Nothing after the first `Opaque` is ever targeted, and no offset is guessed.

A boundary gets plan-guided seeds iff at least one policy field is placeable; otherwise it gets the fallback
seed. `seeds/manifest.json` records per boundary: `plan-guided` (fields placed, IR, offsets, seed hashes) or
`fallback: <first opaque node and reason>`.

### 2. The harness emitter is not refactored

`_decode_and_post` keeps its emission path; generator 0.9's harnesses must be byte-identical to 0.8's
(`scripts/gen_harness_regression.py`, 33 golden entries, is the gate). The Seed IR is lowered from the plan by
its own module (`tools/stu_selector/seed_ir.py`), so it is a second description of the supported roles' byte
semantics — guarded, not shared:

**Round-trip gate.** A test-only generator flag `--decode-dump` emits a variant harness that decodes the input,
prints every decoded value as a SEMANTIC value (scalars, decoded lengths and counts, floats as bits, array and
row contents; never an address) and returns without calling either side.
`scripts/rq4/seed_experiment/test_seed_ir_roundtrip.py`: for boundaries covering every supported node shape
(bounded and unbounded scalars of each width, fixed arrays, single- and multi-row buffer tables with mixed
extents, zero-filled rows, a scalar prefix followed by an opaque string / producer / struct),
`encode(assignments) → bytes → real harness decoder → dump == requested values`, including each bounded
scalar's min and max, zero and negative values, and arrays of every extent. Runs in CI beside the golden test.

**Versioning.** Adding the flag changes the generator source hash even though emitted harnesses do not change:
`GEN_VERSION` 0.8 → 0.9 with that note; the 37 archived cells keep the hashes recorded in their funnel rows;
every seeding run records the 0.9 hash. Old hashes are never rewritten.

### 3. The sampling policy (frozen before any census run; no per-application values)

| field (from the plan) | values |
|---|---|
| bounded scalar with plan `min`/`max` | {min, min+1, ⌊(min+max)/2⌋, max−1, max}, deduplicated |
| unbounded signed integer | {0, 1, 2, 3, 5, 10, 20, −1} |
| unbounded unsigned integer | {0, 1, 2, 3, 5, 10, 20} |
| unbounded float | {0.0, 0.5, 1.0, 2.0, 5.0, −1.0} |
| numeric array, constant extent ≤ 4, placeable | one seed per grid value with every element = v; for extent ≥ 2 additionally one seed per position with that element = the next grid value and the others = v₀; no Cartesian product |
| everything else | never targeted (filler or fallback) |

Per boundary: one seed per assignment set, at most 16 seeds and 2 MiB in total (excess dropped in canonical
order and recorded); filler from PRNG key `7:<boundary>:<k>`, so the seeds of one application are byte-identical
across its translators (verified by hashing, as for tulip). The tulip grid {1,2,3,5,10,20} stays as already run
and archived; it is not re-derived from this table.

### 4. Census: seeds-only replay on the 33 non-tulip cells

Per cell: regenerate the harnesses (deterministic; generator hash recorded per funnel row), one coverage build
per harness, replay (a) the fallback seed alone and (b) the plan-guided seeds alone → two seeds-only coverage
numbers per cell (`c2r_coverage`, the cell's archived universe), reported beside the archived campaign. No
fuzzing. ≈ 600 boundaries, ≈ 5 h serial; one cell at a time, targets and exports deleted as consumed, per-input
replay fallback when a seed panics (lessons of 2026-09-09).

### 5. Pre-registered rerun rule

An application qualifies iff, on at least one of its cells, **plan-guided seeds-only region coverage exceeds the
archived campaign's final region coverage by ≥ 5 percentage points** of that cell's universe. Every buildable
tool cell of a qualifying application is rerun with the archived budget (3 600 s), flags and libFuzzer seed, a
fresh corpus seeded with the plan-guided seeds; base = the archived campaign (its 600 s reproduction was exact
on tulip × c2rust and × Laertes); t=0 measured for both. Non-qualifying applications are reported as measured,
never rerun. Expected: qsort is at 100 %; urlparser has one eligible parameter; cJSON / lil need syntactically
valid strings; lodepng / optipng need formatted buffers and objects; quadtree needs operation sequences;
bzip2 and genann may gain.

### 6. Replay and confirmation of every new corpus

`replay_cell.py` (combined) and `confirm_cell.py --sample 200` for every rerun corpus, as for the 37 cells; a
confirmed defect goes through the manifest family rule. Coverage is never the only update.

### 7. Reporting

The 37 cells are untouched. One ablation table (application × translator: automatic / seeds-only / seeded
campaign) with the artifact-level denominator and, where a translator-added unreachable function inflates it
(tulip × Laertes: C11's severed initialiser, 3 887 regions), the scoped figure beside it. Per cell the manifest
gives the plan-guided / fallback split; the paper states it in one aggregate sentence. Paper text is the user's.

## Order of work

1. Finish tulip: CROWN grid arm; combined replay + confirmation of the four grid corpora; archive; SUMMARY; commit.
2. `seed_ir.py` (lowering + encode + reference decode) and `scripts/rq4/seed_policy.py`; `materialize_seeds.py`
   kept for the tulip provenance.
3. `--decode-dump` + round-trip regression; GEN_VERSION 0.9.
4. Manifest for all 37 cells (plan-guided / fallback counts) from the archived plans.
5. Seeds-only census on the 33 non-tulip cells; census table.
6. Reruns for qualifying applications; replay + confirmation; archive; ablation table.
7. No new decoder roles; no per-application values.

## Status log

- 2026-09-10 (evening): partial Seed IR implemented on branch `seed-ir` — `tools/stu_selector/seed_ir.py` (Scalar / Repeat / Zero / Opaque; order = C declaration order from the generator's parser, rank-sorted like the harness; typedef aliases resolved from the translation), `scripts/rq4/seed_policy.py` (frozen policy, manifest), generator 0.9.1 `--decode-dump` (test-only), `scripts/rq4/seed_experiment/test_seed_ir_roundtrip.py` **passes 40/40** on seven shapes. Census with the IR (plan-guided / planned): bzip2 16/19, lodepng 41/64, optipng 63–64/121–128, tulip 162/213, qsort 3/3, cJSON 6/39, genann 5/10, lil 5/51, quadtree 3/17, urlparser 1/21. `census_cell.sh` ready; seeds-only census runs after the generator-0.9 re-replay of the tulip corpora.

- 2026-09-10 (later): order revised by review — (1) commit checkpoint d5577c49 done; (2) NaN oracle fixed FIRST: generator 0.9 (`c2r_feq*`, `nan_equivalent` outcome/verdict; golden re-frozen), tulip corpora re-replayed under 0.9 without re-fuzzing; (3) then the partial Seed IR; (4) seeds-only census; (5) component-analysis use unless the census shows broad gains (then uniform reruns of every plan-guided cell, never a subset).

- 2026-09-10 morning: first version (complete Decode IR) recorded, then superseded the same day by this partial
  Seed IR version after review. tulip × c2rust (three arms), × Laertes, × C2SaferRust done; × CROWN running;
  replay + confirmation of the four grid corpora queued.
