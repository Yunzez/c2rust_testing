# cjson × ptrtrans — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 113 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 15 |
| built | 9 |
| executed (corpus > 0) | 9 |
| coverage exported | 9 |

Plan failures, by the generator's own reason:

- **30** × signature: struct-invariant param item: cJSON has pointer field 'next' (needs invariant re
- **23** × signature: struct-invariant param object: cJSON has pointer field 'next' (needs invariant 
- **9** × signature: struct-invariant param array: cJSON has pointer field 'next' (needs invariant r
- **6** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is
- **2** × signature: struct-invariant param buffer: parse_buffer has pointer field 'content' (needs 
- **2** × signature: struct-invariant param child: cJSON has pointer field 'next' (needs invariant r
- **2** × Rust signature has 4 parameters, C has 1: reshaped API, no positional bridge
- **2** × Rust signature has 4 parameters, C has 2: reshaped API, no positional bridge
- **2** × signature: struct-invariant param parent: cJSON has pointer field 'next' (needs invariant 
- **1** × signature: struct-invariant param a: cJSON has pointer field 'next' (needs invariant recon
- **1** × input buffer of i8 has Rust type Option<&'a str>, which is not a raw pointer, a slice, a V
- **1** × signature: cJSON_Delete is not present in the Rust translation (no boundary)
- **1** × signature: cJSON_DeleteItemFromArray is not present in the Rust translation (no boundary)
- **1** × signature: cJSON_DeleteItemFromObject is not present in the Rust translation (no boundary)
- **1** × signature: cJSON_DeleteItemFromObjectCaseSensitive is not present in the Rust translation 
- **1** × signature: struct-invariant param hooks: cJSON_Hooks has pointer field 'malloc_fn' (needs 
- **1** × input buffer of i8 has Rust type Option<&mut [u8]>, which is not a raw pointer, a slice, a
- **1** × signature: struct-invariant param hooks: internal_hooks has pointer field 'allocate' (need
- **1** × input buffer of i8 has Rust type Option<&'a [u8]>, which is not a raw pointer, a slice, a 
- **1** × signature: cJSON_SetValuestring is not present in the Rust translation (no boundary)
- **1** × signature: cJSON_free is not present in the Rust translation (no boundary)
- **1** × Rust signature has 2 parameters, C has 1: reshaped API, no positional bridge
- **1** × signature: cJSON_strdup is not present in the Rust translation (no boundary)
- **1** × input buffer of u8 has Rust type Option<&'a str>, which is not a raw pointer, a slice, a V
- **1** × void* parameter has Rust type Option<&'a T>; only a raw pointer is lossless
- **1** × signature: struct-invariant param p: printbuffer has pointer field 'buffer' (needs invaria
- **1** × signature: struct-invariant param output_buffer: printbuffer has pointer field 'buffer' (n
- **1** × signature: struct-invariant param prev: cJSON has pointer field 'next' (needs invariant re
- **1** × signature: struct-invariant param buffer: printbuffer has pointer field 'buffer' (needs in

Planned but not built:

- `cJSON_CreateDoubleArray`: error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments 
- `cJSON_CreateFloatArray`: error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments 
- `cJSON_CreateIntArray`: error[E0061]: this function takes 2 arguments but 1 argument was supplied
error[E0061]: this function takes 2 arguments 
- `compare_double`: error[E0308]: mismatched types
error: could not compile `cjson_ptrtrans-fuzz` (bin "cjson_ptrtrans_ft") due to 1 previou
- `get_decimal_point`: error: linking with `cc` failed: exit status: 1
error: could not compile `cjson_ptrtrans-fuzz` (bin "cjson_ptrtrans_ft")
- `parse_hex4`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `cjson_ptrtrans-fuzz` (bin "cjson

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `cJSON_CreateArray` | no | 1 | 0 | divergence 1 | batch |
| `cJSON_CreateBool` | no | 5 | 0 | divergence 5 | batch |
| `cJSON_CreateFalse` | no | 1 | 0 | divergence 1 | batch |
| `cJSON_CreateNull` | no | 1 | 0 | divergence 1 | batch |
| `cJSON_CreateNumber` | no | 8 | 0 | divergence 8 | batch |
| `cJSON_CreateObject` | no | 1 | 0 | divergence 1 | batch |
| `cJSON_CreateTrue` | no | 1 | 0 | divergence 1 | batch |
| `cJSON_GetErrorPtr` | no | 1 | 0 | normal 1 | batch |
| `cJSON_Version` | no | 1 | 0 | normal 1 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. driver not transpiled; denominator: 121 functions / 2 125 regions (from the rlib's instrumented objects -- the bin route collapsed to 2 functions, see ptrtrans/RUN.md section 7)

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 9 harnesses, 0 crash-all (none).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `a8925ba6292dd9ac`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 121 | 0 | 10 | 0 | 0 | 10 | 111 | 0.000 | 0.083 |
| regions | 2125 | 0 | 68 | 0 | 0 | 68 | 2057 | 0.000 | 0.032 |

Sanity checks: function pass, region pass. Harnesses unioned: 9. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `cJSON_CreateBool` | 5 | 5 | 5 | 5 | 5 |
| `cJSON_CreateNumber` | 8 | 8 | 8 | 8 | 8 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| divergence | 18 |
| normal | 2 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `cJSON_CreateArray` | 1 / 1 | confirmed_divergence 1 | 1 |
| `cJSON_CreateBool` | 5 / 5 | confirmed_divergence 5 | 1 |
| `cJSON_CreateFalse` | 1 / 1 | confirmed_divergence 1 | 1 |
| `cJSON_CreateNull` | 1 / 1 | confirmed_divergence 1 | 1 |
| `cJSON_CreateNumber` | 8 / 8 | confirmed_divergence 8 | 1 |
| `cJSON_CreateObject` | 1 / 1 | confirmed_divergence 1 | 1 |
| `cJSON_CreateTrue` | 1 / 1 | confirmed_divergence 1 | 1 |

Total: confirmed_divergence 18

<!-- prose -->

## 7. What this cell is, and is not (re-run of 2026-09-06)

**Second run, under the plugin-compatibility degradation.** The first run of this cell (2026-09-05)
built 2 of the 15 direct boundaries: the 10 `cJSON_Create*` boundaries failed to *build* because the
cJSON comparator plugin was linked blind and its Rust half named a field (`type_0`) and a destructor
(`cJSON_Delete`) that PtrTrans's crate does not have. That was a generator defect, not a property of
the translation: by the comparison ladder an incompatible plugin must **degrade** the return contract to
pointer nullness, never fail the build. The generator now checks the plugin's declared requirements
(`[plugin.requires]`: struct, fields, destructor) against the translation and drops it with the reason
when they are not met; this cell was re-run in full after that fix (denominator, campaign, replay,
confirmation), and its numbers replace the first run's.

**Funnel.** 113 matched, 15 planned (the 64 `cJSON*`-taking boundaries are *construction unsupported*
under the frozen bridge: PtrTrans's producers return `Option<&mut cJSON>`, `cJSON_New_Item` is a `None`
stub, `cJSON_Delete` does not exist — see the pilot doc), **9 built**: `cJSON_Create{Array,Bool,False,
Null,Number,Object,True}`, `cJSON_GetErrorPtr`, `cJSON_Version`. The 6 that still do not build are the
translation's reshaping, recorded verbatim in `funnel.json`: `cJSON_Create{Double,Float,Int}Array` take a
slice in Rust where C takes `(ptr, count)` (E0061, arity), `compare_double` and `parse_hex4` take reshaped
types (E0308), `get_decimal_point` fails to link (`localeconv` path). The oracle for every built boundary
is `partial(nullness)`.

**Campaign and replay.** 3 600 s, seed 42, all nine concurrent; corpora of 1–8 inputs (the inputs are a
few scalars, so the fuzzer saturates in seconds: `cJSON_CreateNumber` 8, `cJSON_CreateBool` 5, the rest 1).
Combined replay of the 20 corpus inputs: **18 `divergence`**, 2 `normal` (`cJSON_Version` and
`cJSON_GetErrorPtr` in its fresh-process state). Every divergence is rung 3: **the C side returns a
non-NULL object, the translation returns `None`**.

**Confirmation.** 18 of 18 sampled → `confirmed_divergence` (`c_only` normal under ASan+UBSan,
`rust_only` normal, combined replay reports the nullness difference at phase 4). One site, and it is
visible in the source: every `cJSON_Create*` calls `cJSON_New_Item(&global_hooks)`, and PtrTrans's
`cJSON_New_Item` (`cjson_ptrtrans.rs:1122`) is an unimplemented stub returning `None`, so no object can
ever be created. This is the same root as the `Option<&mut T>` producer problem that makes the other 64
boundaries unconstructible.

**How it is recorded.** As a **candidate (CAND-5)**, not a promoted defect: the mechanism is an
*untranslated* function (E1 already counts PtrTrans's cJSON as 24 of 118 stubs, a process failure), not a
mistranslated one; the manifest's defect rows are mistranslations with a source-level mechanism. The
confirmed divergences are the first instrumented, replayable evidence of what the stubs do to a caller,
which is why they are kept as evidence rather than dropped. The author decides whether that distinction
holds.

**Coverage.** 10 of 121 functions (0.083), 68 of 2 125 regions (0.032) — the nine constructors' own
bodies up to the stub, `cJSON_Version`, `cJSON_GetErrorPtr`. Universe from the rlib's own instrumented
objects (`rlib_universe.py`; the first run's link-dead-code denominator had collapsed to two functions
because `cJSON_Version()` is cross-crate inlined — verified identical to the bin route on every other
cell).

**Not established:** anything about PtrTrans's cJSON beyond object construction; the 64 unsupported
boundaries carry no number. The catalogued PtrTrans cJSON defects (S7–S9) come from the earlier
hand-written `cJSON_Parse` campaign (`../campaign_cJSON_Parse/`), which this pipeline cannot regenerate.
