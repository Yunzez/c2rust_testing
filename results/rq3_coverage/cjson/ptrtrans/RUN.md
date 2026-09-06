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

## 7. What this cell is, and is not

**Construction unsupported, by decision.** PtrTrans reshapes cJSON's API: its producers return
`Option<&mut cJSON>` (`cJSON_Parse(Option<&[u8]>) -> Option<&mut cJSON>`), `cJSON_New_Item` is a
`None` stub, and `cJSON_Delete` is not defined in the crate at all (E1: 24 of 118 functions are
stubs). The producer bridge requires a raw-pointer producer and a reachable destructor
(`docs/producer_bridge_pilot.md` §2, §6a), so the 64 `cJSON*`-taking boundaries (30 `item`,
23 `object`, 9 `array`, 2 `child`) are **construction unsupported** here, not "unsupported schema"
and not a defect. The user's rule for extending the bridge is recorded in the pilot doc: a shape is
implemented only when it recurs across more than one translator with a usable producer; this
one appears in PtrTrans alone. The implementation was frozen before this cell ran.

**Of the 15 direct boundaries, 2 build.** The other 13 fail for reasons that are the translation's,
recorded verbatim in `funnel.json`:

| boundaries | build error | what it means |
|---|---|---|
| 10 × `cJSON_Create{Array,Bool,DoubleArray,False,FloatArray,IntArray,Null,Number,Object,True}` | `E0425: cannot find function cJSON_Delete`; `E0609: no field type_0 on cJSON` | a fresh object is returned, the harness frees it through the library's destructor and compares it through the comparator plugin; PtrTrans has neither the destructor nor the C field layout (`type_0` is renamed) |
| `compare_double`, `parse_hex4` | `E0308: mismatched types` | reshaped signatures (slices and references in place of pointers) with no positional bridge |
| `get_decimal_point` | `linking with cc failed` | the C side's `localeconv` path; the translation carries no equivalent symbol |

The two that build, `cJSON_Version` and `cJSON_GetErrorPtr`, take no input: `cJSON_Version` returns
a version string and `cJSON_GetErrorPtr` reads the parser's global error pointer as it stands in a
fresh process (no parse has run in the harness, so it observes the initial state — a zero-argument
function, not a constant one). The corpus is one input each, the campaign is one execution each,
the combined replay is 2 `normal`, there are no candidates and nothing to confirm. The **negative control is trivially satisfied** and says
nothing about PtrTrans's correctness — the catalogued PtrTrans cJSON defects (S7–S9) came from the
earlier hand-written `cJSON_Parse` campaign (`../campaign_cJSON_Parse/`), which this pipeline
cannot regenerate.

**Deviation: the universe was recomputed once.** The first coverage pass reported fn 1/1, region
3/3: the link-dead-code denominator bin referenced only `cJSON_Version()`, which rustc inlines
across crates, so the linker never pulled the rlib member and `-C link-dead-code` had nothing to
keep. `#[no_mangle]` translations never show this (exported symbols are not inlined away), which is
why every earlier universe was correct — verified identical, bin route vs rlib route, on all of
them (bzip2/genann/cJSON×c2rust/lil). The universe is now exported from the rlib's own
instrumented objects (`scripts/rq4/rlib_universe.py`, 121 functions / 2 125 regions) and the
numbers in §4 are the recomputed ones: **fn 2 / 121 (0.017), regions 9 / 2 125 (0.004)**. That is
the honest figure for what the frozen pipeline reaches on this translation: almost nothing.

**Not established:** any coverage or correctness statement about PtrTrans's cJSON beyond the two
zero-argument functions observed in a fresh process. The cell exists so that the matrix says *why* it is empty.
