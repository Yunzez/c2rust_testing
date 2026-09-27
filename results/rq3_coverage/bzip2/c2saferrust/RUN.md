# bzip2 × c2saferrust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 64 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 17 |
| built | 17 |
| executed (corpus > 0) | 17 |
| coverage exported | 15 |

Plan failures, by the generator's own reason:

- **17** × signature: struct-invariant param s: EState has pointer field 'strm' (needs invariant reco
- **11** × signature: unsupported pointer target for b: unsupported BZFILE
- **7** × signature: struct-invariant param strm: bz_stream has pointer field 'next_in' (needs invar
- **4** × signature: struct-invariant param s: DState has pointer field 'strm' (needs invariant reco
- **3** × signature: struct-invariant param f: FILE has pointer field '_IO_read_ptr' (needs invarian
- **2** × it flows into bzopen_or_bzdopen(), whose effect the harness cannot undo. What the boundary
- **1** × Rust signature has 5 parameters, C has 7: reshaped API, no positional bridge
- **1** × Rust signature has 4 parameters, C has 6: reshaped API, no positional bridge
- **1** × it flows into fopen(), whose effect the harness cannot undo. What the boundary consumes is

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `BZ2_bz__AssertH__fail` | no | 1 | 2 | signal 1 | failed rc=1 |
| `BZ2_bzlibVersion` | no | 1 | 0 | — | batch |
| `BZ2_hbAssignCodes` | no | 43 | 0 | normal 27, ub-gated 16 | batch |
| `BZ2_hbCreateDecodeTables` | no | 52 | 27239 | normal 51, ub-gated 1 | batch |
| `BZ2_hbMakeCodeLengths` | no | 7 | 18 | normal 6, timeout 1 | per-input (6/7 completed) |
| `BZ2_indexIntoF` | no | 21 | 0 | normal 21 | batch |
| `bz_config_ok` | yes | 1 | 0 | normal 1 | batch |
| `default_bzalloc` | yes | 8 | 49 | normal 5, signal 2, ub-gated 1 | per-input (6/8 completed) |
| `default_bzfree` | yes | 1 | 0 | normal 1 | batch |
| `fallbackQSort3` | yes | 19 | 4253 | normal 19 | batch |
| `fallbackSimpleSort` | yes | 21 | 3923 | normal 21 | batch |
| `fallbackSort` | yes | 47 | 0 | normal 47 | batch |
| `mainGtU` | yes | 5 | 11 | normal 4, signal 1 | per-input (4/5 completed) |
| `mainQSort3` | yes | 41 | 7499 | normal 40, signal 1 | per-input (40/41 completed) |
| `mainSimpleSort` | yes | 29 | 3729 | normal 28, signal 1 | per-input (28/29 completed) |
| `mainSort` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `mmed3` | yes | 4 | 0 | divergence 3, normal 1 | batch |

## 3. Tests side

Status **TEST-FAILS**, 0/6 passed. every command produced no output; consistent with the reshaped-API SIGSEGV recorded in INVENTORY.md.

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 69 | 0 | 15 | 0 | 0 | 15 | 54 | 0.000 | 0.217 |
| regions | 8227 | 0 | 1158 | 0 | 0 | 1158 | 7069 | 0.000 | 0.141 |

Sanity checks: function pass, region pass. Harnesses unioned: 15. Identities outside the universe (excluded, never added): 0 fn / 4 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `BZ2_hbAssignCodes` | 43 | 43 | 43 | 43 | 43 |
| `BZ2_hbCreateDecodeTables` | 52 | 52 | 52 | 52 | 52 |
| `BZ2_hbMakeCodeLengths` | 4 | 7 | 7 | 7 | 7 |
| `BZ2_indexIntoF` | 21 | 21 | 21 | 21 | 21 |
| `default_bzalloc` | 8 | 8 | 8 | 8 | 8 |
| `fallbackQSort3` | 19 | 19 | 19 | 19 | 19 |
| `fallbackSimpleSort` | 21 | 21 | 21 | 21 | 21 |
| `fallbackSort` | 35 | 37 | 37 | 37 | 47 |
| `mainGtU` | 5 | 5 | 5 | 5 | 5 |
| `mainQSort3` | 37 | 41 | 41 | 41 | 41 |
| `mainSimpleSort` | 28 | 29 | 29 | 29 | 29 |
| `mmed3` | 4 | 4 | 4 | 4 | 4 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 272 |
| ub-gated | 18 |
| signal | 7 |
| divergence | 3 |
| timeout | 1 |

## 6. Confirmation (confirm)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `mmed3` | 3 / 3 | confirmed_divergence 3 | 1 |

Total: confirmed_divergence 3

<!-- prose -->


## 7. Procedure, deviations, and what is not established

Same procedure as `../c2rust/RUN.md` §7, with two planner corrections made after this cell's first
build phase and applied before its numbers were read. (1) The old planner reported 19 planned
boundaries by silently bridging every parameter of a **reshaped** signature as C-ABI; the build then
failed with `E0061` on `BZ2_bzBuffToBuffCompress` (5 Rust parameters for 7 C parameters:
`&mut Vec<u8>` / `&[u8]`) and `BZ2_bzBuffToBuffDecompress`. An arity mismatch is now a plan failure
with its reason, and the honest count is **17 planned** (`plans.json`; the old planner's list is kept as
`plans_oldplanner.json`). (2) `BZ2_bzlibVersion` returns `&str` for `const char*`; the return bridge did
not read the Rust return type and cast a reference to `*const c_void`. It now does (a reference is never
null), and that boundary was built and run **alone at the same 3 600 s budget** afterwards
(`funnel_bzlibVersion_alone.json`), so the funnel reads 17 planned / 17 built / 15 exported.

The tests side is **not a baseline**: 0 of 6 (every command produces no output; the reshaped API
SIGSEGVs), universe from the suite build's export, partition Ours / Neither.

**What is established.** 15 of 69 functions (0.217), 1 158 of 8 227 regions (0.141). The two public
boundaries are **unbridgeable** by this generator — a reshaped `(dest: &mut Vec<u8>, source: &[u8], ..)`
has no positional correspondence to the C signature — so the validator reaches only internal routines;
the number measures the generator's reach on a reshaped API, not the translation. Combined replay of the
301-input corpus: **3 divergences, all `mmed3`** (3 of its 4 inputs, return value), fully adjudicated
**3/3 `confirmed_divergence`** = S14 (the median-of-three helper wrong on most of its domain), re-found
by the pipeline with no hand work.

**What is not established.** S12 (`BZ2_bzBuffToBuffCompress` returning `BZ_PARAM_ERROR` for
`sourceLen == 0`) lives on the unbridged public boundary and cannot be reached by this pipeline; it is
in the manifest from a hand harness and is not re-confirmed here. 46 725 termination artifacts on the
internal sort boundaries are out-of-contract inputs, adjudicated at the sample (856 `ub_associated`,
200 `ub_associated_termination`, 15 `instrument_only`, 17 timeout-inconclusive).
