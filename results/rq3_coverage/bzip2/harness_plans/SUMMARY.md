# bzip2 — generated Harness Plans

**Current: 19 of 64 boundaries plan.** Design: [`docs/harness_oracle_plan.md`](../../../../docs/harness_oracle_plan.md)
(the oracle redesign) over the InputPlan half of [`docs/harness_plan_architecture.md`](../../../../docs/harness_plan_architecture.md).
Generator: `tools/stu_selector/harness_plan.py`. Regenerate with the command in `generate.log`;
the output is byte-identical across runs.

Planned: `BZ2_bzBuffToBuffCompress`, `BZ2_bzBuffToBuffDecompress`, `BZ2_bz__AssertH__fail`, `BZ2_bzlibVersion`, `BZ2_hbAssignCodes`, `BZ2_hbCreateDecodeTables`, `BZ2_hbMakeCodeLengths`, `BZ2_indexIntoF`, `bz_config_ok`, `default_bzalloc`, `default_bzfree`, `fallbackQSort3`, `fallbackSimpleSort`, `fallbackSort`, `mainGtU`, `mainQSort3`, `mainSimpleSort`, `mainSort`, `mmed3`.

**Every remaining failure is an INPUT-construction failure.** There are no output-side rejections
left: a pointer return degrades to the ladder's nullness rung instead of failing.

| n | why harness construction failed |
|---:|---|
| 17 | struct-invariant param s: EState has pointer field 'strm' (needs invariant r |
| 11 | unsupported pointer target for b: unsupported BZFILE |
| 7 | struct-invariant param strm: bz_stream has pointer field 'next_in' (needs in |
| 4 | struct-invariant param s: DState has pointer field 'strm' (needs invariant r |
| 3 | struct-invariant param f: FILE has pointer field '_IO_read_ptr' (needs invar |
| 3 | environment input — needs an environment adapter |

Two capabilities would close all of it: constructing (or sequencing) complex objects —
`EState`/`DState`/`bz_stream`/`FILE`/`BZFILE` — and an environment adapter for boundaries whose
real input is external state rather than a value.

Recent changes, all derived rather than declared:

* a **pointer return** no longer fails construction — it degrades to `pointer_nullness`
  (`oracle_strength: partial(nullness)`);
* a **no-argument** entry plans with zero inputs: there is no input to construct, so both sides are
  called once and compared;
* a **`void*`** parameter is passed as NULL — the one universally valid pointer value — and the plan
  records `input_strength`: `complete` when the entry never uses it (`default_bzalloc`'s `opaque`),
  `partial(null-only)` when it does (`default_bzfree`'s `addr`, where `free(addr)` stays
  unreachable and reaching it needs an object from an earlier call);
* a **`const char*` with no length** is a NUL-terminated string, *unless* it flows — transitively,
  through the call graph — into a call whose effect the harness cannot undo. `BZ2_bzopen`'s `path`
  reaches `fopen` through `bzopen_or_bzdopen`, so it is an **environment input**, not a string:
  what the boundary consumes is a file with certain contents, and fuzzing its *name* only ever
  explores `fopen` failing. This rule is about effects, not about `fopen`, and it costs a library
  like cJSON nothing — there a `const char*` **is** the value under test.

---

*The sections below were written for the earlier ObservationPlan design. The derivation evidence
and the hand-schema comparison remain valid; the `c_execution` column, the safety verdicts and the
"14 candidates" counts do not.*

## Planned boundaries

| boundary | C execution | inputs | observations |
|---|---|---|---|
| `BZ2_bzBuffToBuffCompress` | isolated_asan_ubsan | `dest`:output_buffer, `destLen`:capacity_ptr, `source`:input_buffer, `sourceLen`:length, `blockSize100k`:bounded_scalar, `verbosity`:bounded_scalar, `workFactor`:bounded_scalar | `<return>`:scalar_return, `dest`:buffer_prefix, `destLen`:scalar_output, `source`:buffer_contents |
| `BZ2_bzBuffToBuffDecompress` | isolated_asan_ubsan | `dest`:output_buffer, `destLen`:capacity_ptr, `source`:input_buffer, `sourceLen`:length, `small`:bounded_scalar, `verbosity`:bounded_scalar | `<return>`:scalar_return, `dest`:buffer_prefix, `destLen`:scalar_output, `source`:buffer_contents |
| `BZ2_hbAssignCodes` | in_process_ub_gate | `code`:output_buffer, `length`:input_buffer, `minLen`:bounded_scalar, `maxLen`:bounded_scalar, `alphaSize`:length | `code`:buffer_contents, `length`:buffer_contents |
| `BZ2_hbCreateDecodeTables` | isolated_asan_ubsan | `limit`:inout_array, `base`:inout_array, `perm`:output_array, `length`:input_buffer, `minLen`:bounded_scalar, `maxLen`:bounded_scalar, `alphaSize`:length | `limit`:buffer_contents, `base`:buffer_contents, `perm`:buffer_contents, `length`:buffer_contents |
| `BZ2_hbMakeCodeLengths` | in_process_ub_gate | `len`:output_buffer, `freq`:input_buffer, `alphaSize`:length, `maxLen`:scalar | `len`:buffer_contents, `freq`:buffer_contents |
| `BZ2_indexIntoF` | isolated_asan_ubsan | `indx`:scalar, `cftab`:input_array | `<return>`:scalar_return, `cftab`:buffer_contents |
| `fallbackQSort3` | isolated_asan_ubsan | `fmap`:inout_array, `eclass`:input_array, `loSt`:bounded_scalar, `hiSt`:bounded_scalar | `fmap`:buffer_contents, `eclass`:buffer_contents |
| `fallbackSimpleSort` | isolated_asan_ubsan | `fmap`:inout_array, `eclass`:input_array, `lo`:bounded_scalar, `hi`:bounded_scalar | `fmap`:buffer_contents, `eclass`:buffer_contents |
| `fallbackSort` | isolated_asan_ubsan | `fmap`:inout_array, `eclass`:inout_array, `bhtab`:inout_array, `nblock`:bounded_scalar, `verb`:scalar | `fmap`:buffer_contents, `eclass`:buffer_contents, `bhtab`:buffer_contents |
| `mainGtU` | in_process_ub_gate | `i1`:bounded_scalar, `i2`:bounded_scalar, `block`:input_array, `quadrant`:input_array, `nblock`:scalar, `budget`:output_array | `<return>`:scalar_return, `block`:buffer_contents, `quadrant`:buffer_contents, `budget`:buffer_contents |
| `mainQSort3` | isolated_asan_ubsan | `ptr`:inout_array, `block`:input_array, `quadrant`:input_array, `nblock`:scalar, `loSt`:bounded_scalar, `hiSt`:bounded_scalar, `dSt`:bounded_scalar, `budget`:input_array | `ptr`:buffer_contents, `block`:buffer_contents, `quadrant`:buffer_contents, `budget`:buffer_contents |
| `mainSimpleSort` | isolated_asan_ubsan | `ptr`:inout_array, `block`:input_array, `quadrant`:input_array, `nblock`:scalar, `lo`:bounded_scalar, `hi`:scalar, `d`:scalar, `budget`:input_array | `ptr`:buffer_contents, `block`:buffer_contents, `quadrant`:buffer_contents, `budget`:buffer_contents |
| `mainSort` | isolated_asan_ubsan | `ptr`:inout_array, `block`:inout_array, `quadrant`:output_array, `ftab`:inout_array, `nblock`:bounded_scalar, `verb`:scalar, `budget`:input_array | `ptr`:buffer_contents, `block`:buffer_contents, `quadrant`:buffer_contents, `ftab`:buffer_contents, `budget`:buffer_contents |
| `mmed3` | in_process_ub_gate | `a`:scalar, `b`:scalar, `c`:scalar | `<return>`:scalar_return |

`in_process_ub_gate` means every memory-safety obligation was discharged, so the C side may
run in-process behind the UBSan-minimal gate. `isolated_asan_ubsan` means at least one
obligation is open, so rule 6 requires the C side to run under isolated ASan+UBSan.

## Derived facts that used to be hand-written constants

| boundary | parameter | derived | how | evidence |
|---|---|---|---|---|
| `BZ2_bzBuffToBuffCompress` | `blockSize100k` | `[1, 9]` | rejection_guard | `bzlib.c:1259` |
| `BZ2_bzBuffToBuffCompress` | `verbosity` | `[0, 4]` | rejection_guard | `bzlib.c:1259` |
| `BZ2_bzBuffToBuffCompress` | `workFactor` | `[0, 250]` | rejection_guard | `bzlib.c:1259` |
| `BZ2_bzBuffToBuffDecompress` | `small` | `[0, 1]` | rejection_guard | `bzlib.c:1310` |
| `BZ2_bzBuffToBuffDecompress` | `verbosity` | `[0, 4]` | rejection_guard | `bzlib.c:1310` |
| `BZ2_hbAssignCodes` | `code` | extent `alphaSize` | proven_index_bound | `huffman.c:163` |
| `BZ2_hbAssignCodes` | `length` | extent `alphaSize` | proven_index_bound | `huffman.c:163` |
| `BZ2_hbAssignCodes` | `minLen` | `[0, 1024]` | policy_trip_clamp | `huffman.c:161` |
| `BZ2_hbAssignCodes` | `maxLen` | `[0, 1024]` | policy_trip_clamp | `huffman.c:161` |
| `BZ2_hbCreateDecodeTables` | `limit` | extent `max(maxLen+1, 23)` | proven_extent_in_boundary | `huffman.c:190` |
| `BZ2_hbCreateDecodeTables` | `base` | extent `max(maxLen+2, 257)` | proven_extent_in_boundary | `huffman.c:185` |
| `BZ2_hbCreateDecodeTables` | `length` | extent `alphaSize` | proven_index_bound | `huffman.c:183` |
| `BZ2_hbCreateDecodeTables` | `minLen` | `[0, 1024]` | policy_trip_clamp | `huffman.c:181` |
| `BZ2_hbCreateDecodeTables` | `maxLen` | `[0, 1024]` | policy_trip_clamp, extent_fits_allocation | `huffman.c:181` |
| `BZ2_hbMakeCodeLengths` | `len` | extent `alphaSize` | proven_index_bound | `huffman.c:119` |
| `BZ2_hbMakeCodeLengths` | `freq` | extent `alphaSize` | proven_index_bound | `huffman.c:80` |
| `fallbackQSort3` | `loSt` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `fallbackQSort3` | `hiSt` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `fallbackSimpleSort` | `lo` | `[0, 1024]` | policy_trip_clamp | `blocksort.c:43` |
| `fallbackSimpleSort` | `hi` | `[0, 1024]` | policy_trip_clamp, index_clamped_to_allocation | `blocksort.c:43` |
| `fallbackSort` | `nblock` | `[0, 1024]` | policy_trip_clamp, index_clamped_to_allocation | `blocksort.c:232` |
| `mainGtU` | `i1` | `[0, 4095]` | extent_fits_allocation | `policy` |
| `mainGtU` | `i2` | `[0, 4095]` | extent_fits_allocation | `policy` |
| `mainGtU` | `block` | extent `max(i1+1, i2+1)` | proven_extent_in_boundary | `blocksort.c:360` |
| `mainGtU` | `quadrant` | extent `max(i1+1, i2+1)` | proven_extent_in_boundary | `blocksort.c:414` |
| `mainGtU` | `budget` | extent `1` | proven_extent_in_boundary | `blocksort.c:464` |
| `mainQSort3` | `loSt` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `mainQSort3` | `hiSt` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `mainQSort3` | `dSt` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `mainQSort3` | `budget` | extent `1` | proven_extent_in_boundary | `blocksort.c:652` |
| `mainSimpleSort` | `lo` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `mainSimpleSort` | `budget` | extent `1` | proven_extent_in_boundary | `blocksort.c:552` |
| `mainSort` | `nblock` | `[0, 4095]` | index_clamped_to_allocation | `policy` |
| `mainSort` | `budget` | extent `1` | proven_extent_in_boundary | `blocksort.c:902` |

## Why harness construction failed elsewhere

| n | reason |
|---:|---|
| 17 | signature — struct-invariant param s: EState has pointer field 'strm' (needs invariant reconstruction) |
| 11 | signature — unsupported pointer target for b: unsupported BZFILE |
| 7 | signature — struct-invariant param strm: bz_stream has pointer field 'next_in' (needs invariant reconstruction) |
| 4 | signature — struct-invariant param s: DState has pointer field 'strm' (needs invariant reconstruction) |
| 3 | signature — struct-invariant param f: FILE has pointer field '_IO_read_ptr' (needs invariant reconstruction) |
| 3 | return — pointer return (to unsupported) has no declared contract template. A raw pointer cannot be compared across two allocators; declare return.kind = interior_pointer | structured_object | opaque_ |
| 2 | signature — the entry takes no parameters, so no logical input can be constructed |
| 2 | signature — unsupported pointer target for opaque: unsupported void |
| 1 | no observable output — the boundary has nothing to compare |

Each of these names a **generator capability that does not exist yet** (structured-input
reconstruction for `EState`/`DState`/`bz_stream`, an opaque-handle contract for `BZFILE*`,
a `void*` adapter, a return contract for `char*`). None of them is a reason to hand-write a
plan.

## Open memory-safety obligations

| boundary | parameter | obligation | why |
|---|---|---|---|
| `BZ2_bzBuffToBuffCompress` | `dest` | length parameter really is this buffer's extent | paired by adjacency and name, not proven from the body |
| `BZ2_bzBuffToBuffCompress` | `source` | length parameter really is this buffer's extent | paired by adjacency and name, not proven from the body |
| `BZ2_bzBuffToBuffDecompress` | `dest` | length parameter really is this buffer's extent | paired by adjacency and name, not proven from the body |
| `BZ2_bzBuffToBuffDecompress` | `source` | length parameter really is this buffer's extent | paired by adjacency and name, not proven from the body |
| `BZ2_hbCreateDecodeTables` | `perm` | in-bounds access | required extent is unknown(pp is not a loop induction variable in scope) |
| `BZ2_hbCreateDecodeTables` | `perm` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `BZ2_indexIntoF` | `cftab` | in-bounds access | required extent is unknown(mid is not a loop induction variable in scope) |
| `BZ2_indexIntoF` | `cftab` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackQSort3` | `fmap` | in-bounds access | required extent is unknown(lo is not a loop induction variable in scope) |
| `fallbackQSort3` | `fmap` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackQSort3` | `fmap` | in-bounds access | the pointer escapes into another function |
| `fallbackQSort3` | `eclass` | in-bounds access | required extent is 4294967296 |
| `fallbackQSort3` | `eclass` | in-bounds access | the pointer escapes into another function |
| `fallbackSimpleSort` | `fmap` | in-bounds access | required extent is unknown(i is not a loop induction variable in scope) |
| `fallbackSimpleSort` | `fmap` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackSimpleSort` | `eclass` | in-bounds access | required extent is unknown(tmp is not a loop induction variable in scope) |
| `fallbackSimpleSort` | `eclass` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackSort` | `fmap` | in-bounds access | required extent is unknown(k is not a loop induction variable in scope) |
| `fallbackSort` | `fmap` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackSort` | `fmap` | in-bounds access | the pointer escapes into another function |
| `fallbackSort` | `eclass` | in-bounds access | required extent is unknown(k is not a loop induction variable in scope) |
| `fallbackSort` | `eclass` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `fallbackSort` | `eclass` | in-bounds access | the pointer escapes into another function |
| `fallbackSort` | `bhtab` | in-bounds access | required extent is unknown(nBhtab is not a loop induction variable in scope) |
| `fallbackSort` | `bhtab` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainQSort3` | `ptr` | in-bounds access | required extent is unknown(lo is not a loop induction variable in scope) |
| `mainQSort3` | `ptr` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainQSort3` | `ptr` | in-bounds access | the pointer escapes into another function |
| `mainQSort3` | `block` | in-bounds access | required extent is unknown(sum of two non-constants) |
| `mainQSort3` | `block` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainQSort3` | `block` | in-bounds access | the pointer escapes into another function |
| `mainQSort3` | `quadrant` | in-bounds access | required extent is unknown(pointer is never dereferenced) |
| `mainQSort3` | `quadrant` | in-bounds access | the pointer escapes into another function |
| `mainQSort3` | `budget` | in-bounds access | the pointer escapes into another function |
| `mainSimpleSort` | `ptr` | in-bounds access | required extent is unknown(i is not a loop induction variable in scope) |
| `mainSimpleSort` | `ptr` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainSimpleSort` | `block` | in-bounds access | required extent is unknown(pointer is never dereferenced) |
| `mainSimpleSort` | `block` | in-bounds access | the pointer escapes into another function |
| `mainSimpleSort` | `quadrant` | in-bounds access | required extent is unknown(pointer is never dereferenced) |
| `mainSimpleSort` | `quadrant` | in-bounds access | the pointer escapes into another function |
| `mainSimpleSort` | `budget` | in-bounds access | the pointer escapes into another function |
| `mainSort` | `ptr` | in-bounds access | required extent is unknown(j is not a loop induction variable in scope) |
| `mainSort` | `ptr` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainSort` | `ptr` | in-bounds access | the pointer escapes into another function |
| `mainSort` | `block` | in-bounds access | required extent is unknown(i is not a loop induction variable in scope) |
| `mainSort` | `block` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainSort` | `block` | in-bounds access | the pointer escapes into another function |
| `mainSort` | `quadrant` | in-bounds access | required extent is unknown(i is not a loop induction variable in scope) |
| `mainSort` | `quadrant` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainSort` | `quadrant` | in-bounds access | the pointer escapes into another function |
| `mainSort` | `ftab` | in-bounds access | required extent is unknown(i is not a loop induction variable in scope) |
| `mainSort` | `ftab` | non-negative index | the index lower bound could not be shown to be >= 0 |
| `mainSort` | `budget` | in-bounds access | the pointer escapes into another function |

## Known unsoundness (found by review, 2026-09-04 — not yet fixed)

| # | boundary | what is wrong | root cause |
|---|---|---|---|
| 1 | `mainSort` | the plan allocates 4096 elements for `ftab`, but the body executes `ftab[65536]` | a **proven** required extent that exceeds what the policy will allocate is downgraded to `policy_allocation_capped` plus an "unproven" obligation. That is wrong: a proven requirement the harness cannot satisfy is a **plan failure**. Isolated ASan does not fix the input model — it just makes the boundary fail on essentially every input (guaranteed rejection, not a filter) |
| 2 | `mainGtU` | labelled `in_process_ub_gate`, but the extent only covers the initial `block[i1]` / `block[i2]`; the body then walks the indices forward and wraps them with an unconstrained `nblock`, so a generated `i1 = 4095` goes out of bounds — and the in-process UBSan-minimal gate does not see general heap overflow | the index-bound analysis treats a subscript's bound as the whole requirement; indices advanced inside a loop, and modular wrap-around, are not modelled |
| 3 | `BZ2_hbMakeCodeLengths` | labelled `in_process_ub_gate`, but `alphaSize` grows with the input-buffer length and is never constrained by the function's **internal** fixed-size arrays (`heap[BZ_MAX_ALPHA_SIZE+2]`, `weight[…*2]`, `parent[…]`) | the analyser proves safety only for **parameter** pointers. Local `CONSTANTARRAY` declarations have a statically known extent and are the *easiest* constraint to derive — omitting them leaves the parameter effectively unbounded |

**All three were confirmed empirically** by building and running every candidate
([`LOWERING.md`](LOWERING.md)): `mainSort` dies on run 1, `mainGtU` on run 19 with an ASan
heap-buffer-overflow, and `BZ2_hbMakeCodeLengths` hangs at 3 executions per second. Four more
candidates (`fallbackQSort3`, `fallbackSimpleSort`, `mainQSort3`, `mainSimpleSort`) fail the same
way, so the count is **7 of 14 rejected by their own harness** — 6 on memory safety, 1 on
termination — and **7 of 14 sound** (27.1 M clean executions, zero divergences).

Consequence: the `c_execution` field is unusable — two of its four `in_process_ub_gate` labels are
among the failures. Fixing 3 is the highest-value fix, because a local array's extent is provable
from its declaration.

## Migration check against the retired hand-written schemas

**This is migration debugging, not acceptance.** A hand-written schema can itself be wrong,
so it is never ground truth. Acceptance is the seven-item list in
`docs/harness_plan_architecture.md`. The comparison is recorded only because it is how the
derivations were debugged, and because the differences are informative.

| boundary | parameter | hand-written | derived | verdict |
|---|---|---|---|---|
| `BZ2_hbCreateDecodeTables` | `base` | `cap: 257` | extent `max(maxLen+2, 257)` → 1026 elems | **hand constant was an under-allocation** whenever `maxLen > 255`; the derived extent covers it |
| `BZ2_hbCreateDecodeTables` | `limit` | `cap: 257` | extent `max(maxLen+1, 23)` → 1025 elems | derived; 257 was a guess copied from `base` |
| `BZ2_hbAssignCodes` | `code` | `cap: 258` | capacity = `alphaSize` | derived from `code[i], i < alphaSize`; no constant needed |
| `BZ2_hbAssignCodes` | `length` | `max_len: 258` | length = `alphaSize`, ≤ policy | derived; 258 was `BZ_MAX_ALPHA_SIZE`, a fact about bzip2's *callers* |
| `BZ2_indexIntoF` | `cftab` | `output_array, cap 257` (zero-filled) | `input_array`, 4096 elems, fuzz-filled | **coverage gain**: the hand model zeroed the table, making the binary search deterministic |
| `fallbackQSort3` | `eclass` | `output_array, cap 1024` (zero-filled) | `input_array`, fuzz-filled | same coverage gain |
| `fallbackQSort3` | `loSt/hiSt` | `bounded (0,1023)` | `bounded (0,4095)` from the allocation | derived through the local stack array by taint; the bound is the harness's own allocation |
| `fallbackSort` | `verb` | `bounded (0,4)` | free scalar | **hand fact was borrowed from another function's guard**; `fallbackSort` has no such guard |
| `BZ2_bzBuffToBuffCompress` | `blockSize100k` | `bounded (1,9)` | `bounded (1,9)` | identical, now with the guard cited as evidence |
| `BZ2_bzBuffToBuffCompress` | `dest` | `cap: 1179648` | capacity fuzz-decoded ≤ policy 1 MiB | per-boundary constant replaced by the global policy |
| `BZ2_hbAssignCodes` | `maxLen` | `bounded (0,23)`, `min_var: minLen` | `bounded (0,1024)` (policy trip clamp) | 23 and the `maxLen >= minLen` relation were assumptions; neither is provable here |
| `BZ2_bz__AssertH__fail` | `—` | a harness was generated (it `exit(3)`s on every input) | harness construction FAILS: no observable output | the planner rejects it for the right reason |

Four boundaries the hand path could not reach at all — `mainSort`, `mainQSort3`,
`mainSimpleSort`, `mainGtU`, the core of bzip2's block sorter — now plan. They were blocked
by a missing `unsigned short` entry in the generator's scalar map, a gap the schema-writing
workflow had hidden behind "unsupported".

## Regeneration

```
python3 tools/stu_selector/harness_plan.py \
    --pair <pair with build/compile_commands.json + source/*.c> \
    --all --out-dir results/rq3_coverage/bzip2/harness_plans/plans \
    --json results/rq3_coverage/bzip2/harness_plans/all_plans.json
```

The plan is derived from the **C** side only (AST + function body), so it is a property of
the library, not of the translator; the Rust signature enters later, at harness construction.
The pair used here is the bzip2 × c2rust pair built for the RQ4 coverage cell
(`results/rq3_coverage/bzip2/c2rust/RUN.md`).

## Provenance — the pair is reconstructible from this repository

| half | repo path | md5 |
|---|---|---|
| C source | `tools/frameworks/crown/c-code/bzip2/*.c` (e.g. `blocksort.c`) | `0415cdf33485969950474a4b8bbd54d5` |
| c2rust translation | `fuzz/bzip2_c2rust_e3/src/` (nine modules) | flattens to `88541158051b1c3d8dba8ac3bead6210` |

The generator takes a single `.rs`, so the nine translated modules are flattened first. The
flattening is deterministic and was re-run to confirm the byte-identity above:

```
python3 scripts/flatten_translation.py fuzz/bzip2_c2rust_e3/src <pair>/translated/bzip2_c2rust.rs
```

`scripts/flatten_translation.py` also writes the line map that undoes the flattening, so coverage
measured on the flattened crate is comparable identity-for-identity with the ordinary multi-file
crate.
