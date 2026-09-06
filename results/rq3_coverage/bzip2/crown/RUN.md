# bzip2 × crown — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 64 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 19 |
| built | 19 |
| executed (corpus > 0) | 19 |
| coverage exported | 16 |

Plan failures, by the generator's own reason:

- **17** × signature: struct-invariant param s: EState has pointer field 'strm' (needs invariant reco
- **11** × signature: unsupported pointer target for b: unsupported BZFILE
- **7** × signature: struct-invariant param strm: bz_stream has pointer field 'next_in' (needs invar
- **4** × signature: struct-invariant param s: DState has pointer field 'strm' (needs invariant reco
- **3** × signature: struct-invariant param f: FILE has pointer field '_IO_read_ptr' (needs invarian
- **2** × it flows into bzopen_or_bzdopen(), whose effect the harness cannot undo. What the boundary
- **1** × it flows into fopen(), whose effect the harness cannot undo. What the boundary consumes is

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `BZ2_bzBuffToBuffCompress` | no | 187 | 3153 | divergence 97, normal 71, signal 19 | per-input (168/187 completed) |
| `BZ2_bzBuffToBuffDecompress` | no | 527 | 0 | divergence 113, normal 414 | batch |
| `BZ2_bz__AssertH__fail` | no | 1 | 2 | signal 1 | failed rc=1 |
| `BZ2_bzlibVersion` | no | 1 | 0 | normal 1 | batch |
| `BZ2_hbAssignCodes` | no | 47 | 0 | normal 30, ub-gated 17 | batch |
| `BZ2_hbCreateDecodeTables` | no | 52 | 26783 | normal 51, ub-gated 1 | batch |
| `BZ2_hbMakeCodeLengths` | no | 7 | 18 | normal 6, timeout 1 | per-input (6/7 completed) |
| `BZ2_indexIntoF` | no | 21 | 0 | normal 21 | batch |
| `bz_config_ok` | yes | 1 | 0 | normal 1 | batch |
| `default_bzalloc` | yes | 6 | 14 | normal 5, ub-gated 1 | failed rc=1 |
| `default_bzfree` | yes | 1 | 0 | normal 1 | batch |
| `fallbackQSort3` | yes | 19 | 4246 | normal 19 | batch |
| `fallbackSimpleSort` | yes | 21 | 3895 | normal 21 | batch |
| `fallbackSort` | yes | 32 | 5109 | divergence 32 | batch |
| `mainGtU` | yes | 5 | 10 | normal 4, signal 1 | per-input (4/5 completed) |
| `mainQSort3` | yes | 39 | 12496 | normal 38, signal 1 | per-input (38/39 completed) |
| `mainSimpleSort` | yes | 31 | 4079 | normal 30, signal 1 | per-input (30/31 completed) |
| `mainSort` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `mmed3` | yes | 4 | 0 | normal 4 | batch |

## 3. Tests side

Status **TEST-ADAPTER-FAILS**. the adapter bin does not compile: `bzip2_crown::bzip2::main` not found (CROWN comments out the CLI main). Universe from denom_crown.

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 74 | 0 | 51 | 0 | 0 | 51 | 23 | 0.000 | 0.689 |
| regions | 9084 | 0 | 5414 | 0 | 0 | 5414 | 3670 | 0.000 | 0.596 |

Sanity checks: function pass, region pass. Harnesses unioned: 16. Identities outside the universe (excluded, never added): 0 fn / 1 reg.

### Budget cross-check from the same campaign (hard-linked snapshots)

| budget | fn ours | reg ours | reg only-ours |
|---:|---:|---:|---:|
| 300 s | 51 (0.689) | 4984 (0.549) | 4984 |
| 1800 s | 51 (0.689) | 5367 (0.591) | 5367 |
| 3600 s | 51 (0.689) | 5414 (0.596) | 5414 |

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `BZ2_bzBuffToBuffCompress` | 168 | 174 | 176 | 182 | 187 |
| `BZ2_bzBuffToBuffDecompress` | 257 | 335 | 353 | 492 | 527 |
| `BZ2_hbAssignCodes` | 47 | 47 | 47 | 47 | 47 |
| `BZ2_hbCreateDecodeTables` | 52 | 52 | 52 | 52 | 52 |
| `BZ2_hbMakeCodeLengths` | 4 | 7 | 7 | 7 | 7 |
| `BZ2_indexIntoF` | 21 | 21 | 21 | 21 | 21 |
| `default_bzalloc` | 6 | 6 | 6 | 6 | 6 |
| `fallbackQSort3` | 19 | 19 | 19 | 19 | 19 |
| `fallbackSimpleSort` | 21 | 21 | 21 | 21 | 21 |
| `fallbackSort` | 31 | 31 | 31 | 31 | 32 |
| `mainGtU` | 5 | 5 | 5 | 5 | 5 |
| `mainQSort3` | 38 | 38 | 38 | 39 | 39 |
| `mainSimpleSort` | 30 | 31 | 31 | 31 | 31 |
| `mmed3` | 4 | 4 | 4 | 4 | 4 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 717 |
| divergence | 242 |
| signal | 24 |
| ub-gated | 19 |
| timeout | 1 |

## 6. Confirmation (confirm)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `BZ2_bzBuffToBuffCompress` | 3269 / 3269 | confirmed_divergence 97, confirmed_termination 4, instrument_only 2645, out_of_contract_access 523 | 8 |
| `BZ2_bzBuffToBuffDecompress` | 113 / 113 | confirmed_divergence 113 | 1 |
| `fallbackSort` | 5141 / 5141 | confirmed_divergence 32, out_of_contract_access 5109 | 2 |

Total: confirmed_divergence 242, confirmed_termination 4, instrument_only 2645, out_of_contract_access 5632

<!-- prose -->


## 7. Procedure, deviations, and what is not established

Same procedure as `../c2rust/RUN.md` §7. The tests side is **TEST-ADAPTER-FAILS**: CROWN comments out
the CLI `main`, the adapter does not compile, and the universe comes from `raw/denom_crown.tar.gz`
(a link-dead-code build that references the library and runs nothing). Partition: Ours / Neither.

**What is established.** 51 of 74 functions (0.689), 5 414 of 9 084 regions (0.596). The compressor is
crash-dominated: Compress's corpus stalled at 168 inputs after the first minute (187 at the hour) while
Decompress grew 257 → 527; exploration on Compress was suppressed by defect-triggered restarts, so its
coverage is reported with that qualifier and never bare. **This is the cell where the hour mattered**:
regions 4 984 → 5 367 → 5 414 (300 s / 1 800 s / 3 600 s), +8.6 % over the five-minute point, all of it
on Decompress. Combined replay of the 1 003-input corpus: **242 divergences** — Compress 97/187
(`written length destLen`, output stream = S10), Decompress 113/527 (return value, output = S11), and
**`fallbackSort` 32/32 on `array bhtab`** with the C side in contract, a boundary on which c2rust and
Laertes replay 57/57 and 56/56 normal. Full adjudication: Decompress 113/113 `confirmed_divergence`;
Compress 97 `confirmed_divergence` + 4 `confirmed_termination` (a bounds-checked index in
`generateMTFValues`, `mtfFreq[j+1]`, with no sanitizer); `fallbackSort` **32/32 `confirmed_divergence`**.
The source diff behind the `bhtab` divergence: c2rust's compound assignment through a reborrow
(`let ref mut fresh0 = *bhtab.offset(..); *fresh0 |= 1 << ..`, the `SET_BH` macro) was rewritten by CROWN
to the plain store `*bhtab.offset(..) = 1 << ..` at all three `SET_BH` sites; setting a bit clears the rest
of the word. That corrupted sort is upstream of S10's invalid stream, C7's heap overrun and the `mtfFreq`
panic — one root cause, recorded on C7 and S10 in the manifest.

**What is not established.** The 2 645 `instrument_only` and 523 `out_of_contract_access` artifacts on
Compress, and the 5 109 on `fallbackSort`, are wild-address ASan reports on the translation with the C
side clean: asymmetric *detection* of a far access is not asymmetric behaviour (the C side may read into
a live allocation silently), so none of them is claimed on its own — they are attached to the
layout-independent `bhtab` evidence as its memory face. Whether C7 and S10 should be one manifest entry
is a taxonomy decision left open; the evidence now supports it.
