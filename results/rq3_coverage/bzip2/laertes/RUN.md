# bzip2 × laertes — RQ4 cell (plan pipeline)

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
| `BZ2_bzBuffToBuffCompress` | no | 296 | 221 | divergence 273, normal 11, panic 12 | per-input (284/296 completed) |
| `BZ2_bzBuffToBuffDecompress` | no | 417 | 0 | divergence 26, normal 391 | batch |
| `BZ2_bz__AssertH__fail` | no | 1 | 2 | signal 1 | failed rc=1 |
| `BZ2_bzlibVersion` | no | 1 | 0 | normal 1 | batch |
| `BZ2_hbAssignCodes` | no | 44 | 0 | normal 30, ub-gated 14 | batch |
| `BZ2_hbCreateDecodeTables` | no | 52 | 26834 | normal 51, ub-gated 1 | batch |
| `BZ2_hbMakeCodeLengths` | no | 7 | 18 | normal 6, timeout 1 | per-input (6/7 completed) |
| `BZ2_indexIntoF` | no | 21 | 0 | normal 21 | batch |
| `bz_config_ok` | yes | 1 | 0 | normal 1 | batch |
| `default_bzalloc` | yes | 6 | 14 | normal 5, ub-gated 1 | failed rc=1 |
| `default_bzfree` | yes | 1 | 0 | normal 1 | batch |
| `fallbackQSort3` | yes | 19 | 4252 | normal 19 | batch |
| `fallbackSimpleSort` | yes | 21 | 3895 | normal 21 | batch |
| `fallbackSort` | yes | 56 | 0 | normal 56 | batch |
| `mainGtU` | yes | 5 | 10 | normal 4, signal 1 | per-input (4/5 completed) |
| `mainQSort3` | yes | 10 | 122 | normal 9, signal 1 | per-input (9/10 completed) |
| `mainSimpleSort` | yes | 11 | 135 | normal 10, signal 1 | per-input (10/11 completed) |
| `mainSort` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `mmed3` | yes | 4 | 0 | normal 4 | batch |

## 3. Tests side

Status **TEST-FAILS**, 0/6 passed. compress: empty output. decompress: truncated at a multiple of 5000 bytes (the CLI's write block); the final partial block is never emitted. CLI-level observation only — not a defect until confirmed at a boundary.

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 82 | 0 | 45 | 0 | 0 | 45 | 37 | 0.000 | 0.549 |
| regions | 10065 | 0 | 6206 | 0 | 0 | 6206 | 3859 | 0.000 | 0.617 |

Sanity checks: function pass, region pass. Harnesses unioned: 16. Identities outside the universe (excluded, never added): 0 fn / 1 reg.

### Budget cross-check from the same campaign (hard-linked snapshots)

| budget | fn ours | reg ours | reg only-ours |
|---:|---:|---:|---:|
| 300 s | 45 (0.549) | 6144 (0.610) | 6144 |
| 1800 s | 45 (0.549) | 6174 (0.613) | 6174 |
| 3600 s | 45 (0.549) | 6206 (0.617) | 6206 |

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `BZ2_bzBuffToBuffCompress` | 239 | 291 | 292 | 295 | 296 |
| `BZ2_bzBuffToBuffDecompress` | 204 | 326 | 356 | 392 | 417 |
| `BZ2_hbAssignCodes` | 44 | 44 | 44 | 44 | 44 |
| `BZ2_hbCreateDecodeTables` | 52 | 52 | 52 | 52 | 52 |
| `BZ2_hbMakeCodeLengths` | 4 | 7 | 7 | 7 | 7 |
| `BZ2_indexIntoF` | 21 | 21 | 21 | 21 | 21 |
| `default_bzalloc` | 6 | 6 | 6 | 6 | 6 |
| `fallbackQSort3` | 19 | 19 | 19 | 19 | 19 |
| `fallbackSimpleSort` | 21 | 21 | 21 | 21 | 21 |
| `fallbackSort` | 42 | 44 | 44 | 44 | 56 |
| `mainGtU` | 5 | 5 | 5 | 5 | 5 |
| `mainQSort3` | 10 | 10 | 10 | 10 | 10 |
| `mainSimpleSort` | 11 | 11 | 11 | 11 | 11 |
| `mmed3` | 4 | 4 | 4 | 4 | 4 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 641 |
| divergence | 299 |
| ub-gated | 16 |
| panic | 12 |
| signal | 5 |
| timeout | 1 |

## 6. Confirmation (confirm)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `BZ2_bzBuffToBuffCompress` | 506 / 506 | confirmed_divergence 273, confirmed_termination 233 | 2 |
| `BZ2_bzBuffToBuffDecompress` | 26 / 26 | confirmed_divergence 26 | 1 |

Total: confirmed_divergence 299, confirmed_termination 233

<!-- prose -->


## 7. Procedure, deviations, and what is not established

Same procedure as `../c2rust/RUN.md` §7. The tests side is **not a baseline**: the shipped suite passes
0 of 6 through this translation (`../tests_side_results.json` — compress outputs empty, decompress
outputs truncated at 5 000-byte block boundaries), so the universe comes from the suite build's
link-dead-code export and the partition is Ours / Neither.

**What is established.** 45 of 82 functions (0.549) and 6 206 of 10 065 regions (0.617), 16 of 19
harnesses exported. Of the 37 functions nothing reached, 4 are `laertes_init_*` initialisers and ~28 are
constructors (`new` / `Default`) the translator synthesised and nothing calls. Compress covers 3 294
regions of this artifact where c2rust's compress covers 4 511 of its own — the compressor dies early.
The combined replay of the 974-input corpus yields **299 divergences**: Compress 273/296 on the output
stream, Decompress 26/417 on the return value (valid streams, the shipped samples among them, are
rejected); both are the zeroed `BZ2_crc32Table` (S3, `laertes_init_BZ2_crc32Table` is defined and never
called), on two boundaries of one defect. Full adjudication of both public boundaries: **532 / 532
confirmed, 0 `ub_associated`** — 299 `confirmed_divergence` and 233 `confirmed_termination`. The 233 are
one source site, `mainSimpleSort` `while incs[hp] < bigN { hp += 1 }` on an all-zero `incs` table
(`laertes_init_incs` defined once, never called): C-only under ASan + full UBSan returns normally, the
translation panics, and it panics with no sanitizer and the marker present. The same site accounts for
53 / 122 `mainQSort3` and 18 / 135 `mainSimpleSort` sampled artifacts whose inputs keep C in contract.
This is defect **C8** in the manifest; the severed-init scanner had flagged the static
(`severed_init_scan.json`: incs, poisoned, 3 consumer references) before any input triggered it, and the
suite's three empty compress outputs are the same failure seen from the CLI.

**What is not established.** Whether Laertes consumed a byte-identical copy of the bzip2 1.0.8 the
harness compiles (CROWN's copy of the same benchmark) is not recorded — the mechanism is read from the
Laertes crate itself, the reference C is the shared benchmark copy. `BZ2_rNums`'s initialiser is also
never called; no input in this campaign reaches the randomised-block decode path, so nothing is claimed
about it. Coverage grew +62 regions from 300 s to 3 600 s (+1.0 %), 0 functions.
