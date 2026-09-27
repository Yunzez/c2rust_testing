# lodepng × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 235 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 64 |
| built | 54 |
| executed (corpus > 0) | 54 |
| coverage exported | 47 |

Plan failures, by the generator's own reason:

- **28** × signature: struct-invariant param info: LodePNGInfo has nested struct field 'color' (needs
- **26** × signature: struct-invariant param out: ucvector has pointer field 'data' (needs invariant 
- **23** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is
- **12** × signature: struct-invariant param info: LodePNGColorMode has pointer field 'palette' (need
- **9** × signature: struct-invariant param reader: LodePNGBitReader has pointer field 'data' (needs
- **8** × signature: struct-invariant param tree: HuffmanTree has pointer field 'codes' (needs invar
- **6** × signature: struct-invariant param writer: LodePNGBitWriter has pointer field 'data' (needs
- **6** × signature: struct-invariant param mode: LodePNGColorMode has pointer field 'palette' (need
- **6** × it flows into fopen(), whose effect the harness cannot undo. What the boundary consumes is
- **5** × signature: struct-invariant param tree: ColorTree has unsupported field 'children' (array)
- **4** × signature: struct-invariant param dest: LodePNGInfo has nested struct field 'color' (needs
- **4** × signature: struct-invariant param color: LodePNGColorMode has pointer field 'palette' (nee
- **4** × signature: struct-invariant param state: LodePNGState has nested struct field 'decoder' (n
- **3** × signature: struct-invariant param mode_out: LodePNGColorMode has pointer field 'palette' (
- **3** × signature: struct-invariant param hash: Hash has pointer field 'head' (needs invariant rec
- **3** × signature: struct-invariant param p: uivector has pointer field 'data' (needs invariant re
- **2** × signature: struct-invariant param lists: BPMLists has pointer field 'memory' (needs invari
- **2** × signature: struct-invariant param tree_ll: HuffmanTree has pointer field 'codes' (needs in
- **2** × it flows into lodepng_decode_file(), whose effect the harness cannot undo. What the bounda
- **2** × signature: struct-invariant param p: ucvector has pointer field 'data' (needs invariant re
- **1** × signature: struct-invariant param values: uivector has pointer field 'data' (needs invaria
- **1** × signature: struct-invariant param leaves: BPMNode has pointer field 'tail' (needs invarian
- **1** × signature: struct-invariant param out: uivector has pointer field 'data' (needs invariant 
- **1** × signature: struct-invariant param dest: LodePNGColorMode has pointer field 'palette' (need
- **1** × signature: struct-invariant param a: LodePNGColorMode has pointer field 'palette' (needs i
- **1** × signature: struct-invariant param settings: LodePNGCompressSettings has pointer field 'cus
- **1** × signature: struct-invariant param mode_in: LodePNGColorMode has pointer field 'palette' (n
- **1** × signature: struct-invariant param settings: LodePNGDecoderSettings has nested struct field
- **1** × signature: struct-invariant param settings: LodePNGDecompressSettings has pointer field 'c
- **1** × signature: struct-invariant param settings: LodePNGEncoderSettings has nested struct field
- **1** × signature: struct-invariant param pngcolor: LodePNGColorMode has pointer field 'palette' (
- **1** × signature: struct-invariant param dest: LodePNGState has nested struct field 'decoder' (ne
- **1** × signature: struct-invariant param info_png: LodePNGInfo has nested struct field 'color' (n

Planned but not built:

- `Adam7_getpassvalues`: error[E0308]: arguments to this function are incorrect
error[E0308]: arguments to this function are incorrect
error: cou
- `lodepng_addofl`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodep
- `lodepng_color_mode_make`:   plan: 2 inputs; bridges colortype=scalar_copy, bitdepth=scalar_copy
harness construction failed: return -- return type
- `lodepng_color_stats_add`: error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodepng_c2rust_ft") due to 1 previou
- `lodepng_color_stats_init`: error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodepng_c2rust_ft") due to 1 previou
- `lodepng_mulofl`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodep
- `readBitFromReversedStream`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodep
- `readBitsFromReversedStream`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodep
- `setBitOfReversedStream`: error[E0308]: mismatched types
error[E0308]: mismatched types
error: could not compile `lodepng_c2rust-fuzz` (bin "lodep
- `ucvector_init`:   plan: 2 inputs; bridges buffer=c_abi, size=scalar_cast
harness construction failed: return -- return type struct is ne

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `Adam7_deinterlace` | yes | 9 | 83 | normal 7, signal 1, timeout 1 | per-input (7/9 completed) |
| `Adam7_interlace` | yes | 14 | 244 | normal 8, signal 1, timeout 5 | per-input (8/14 completed) |
| `addColorBits` | yes | 6 | 18 | normal 5, ub-gated 1 | per-input (5/6 completed) |
| `addPaddingBits` | yes | 22 | 544 | normal 21, signal 1 | per-input (21/22 completed) |
| `adler32` | yes | 12 | 0 | normal 12 | batch |
| `alloc_string` | yes | 19 | 0 | normal 19 | batch |
| `alloc_string_sized` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `checkColorValidity` | yes | 16 | 0 | normal 16 | batch |
| `countZeros` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `filterScanline` | yes | 91 | 8150 | normal 91 | batch |
| `getHash` | yes | 18 | 0 | normal 18 | batch |
| `getNumColorChannels` | yes | 6 | 0 | normal 6 | batch |
| `getValueRequiredBits` | yes | 4 | 0 | normal 4 | batch |
| `ilog2` | yes | 8 | 0 | normal 8 | batch |
| `ilog2i` | yes | 8 | 0 | normal 8 | batch |
| `isGrayICCProfile` | yes | 27 | 0 | normal 27 | batch |
| `isRGBICCProfile` | yes | 26 | 0 | normal 26 | batch |
| `lodepng_chunk_ancillary` | no | 6 | 0 | normal 6 | batch |
| `lodepng_chunk_check_crc` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lodepng_chunk_data` | no | 10 | 0 | normal 10 | batch |
| `lodepng_chunk_data_const` | no | 10 | 0 | normal 10 | batch |
| `lodepng_chunk_find` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lodepng_chunk_find_const` | no | 45 | 21734 | normal 45 | batch |
| `lodepng_chunk_generate_crc` | no | 1 | 2 | signal 1 | failed rc=1 |
| `lodepng_chunk_length` | no | 5 | 0 | normal 5 | batch |
| `lodepng_chunk_next` | no | 15 | 0 | normal 15 | batch |
| `lodepng_chunk_next_const` | no | 14 | 0 | normal 14 | batch |
| `lodepng_chunk_private` | no | 8 | 0 | normal 8 | batch |
| `lodepng_chunk_safetocopy` | no | 8 | 0 | normal 8 | batch |
| `lodepng_chunk_type` | no | 14 | 0 | normal 14 | batch |
| `lodepng_chunk_type_equals` | no | 18 | 0 | normal 18 | batch |
| `lodepng_crc32` | no | 13 | 0 | normal 13 | batch |
| `lodepng_error_text` | no | 5 | 0 | normal 5 | batch |
| `lodepng_free` | yes | 1 | 0 | normal 1 | batch |
| `lodepng_get_bpp_lct` | yes | 9 | 0 | normal 9 | batch |
| `lodepng_get_raw_size_idat` | yes | 13 | 0 | normal 13 | batch |
| `lodepng_get_raw_size_lct` | yes | 17 | 0 | normal 17 | batch |
| `lodepng_gtofl` | yes | 9 | 0 | normal 9 | batch |
| `lodepng_huffman_code_lengths` | no | 23 | 1294 | normal 22, ub-gated 1 | per-input (22/23 completed) |
| `lodepng_malloc` | yes | 4 | 8 | normal 3, signal 1 | per-input (3/4 completed) |
| `lodepng_memcpy` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `lodepng_memset` | yes | 7 | 25 | normal 6, signal 1 | per-input (6/7 completed) |
| `lodepng_read32bitInt` | yes | 5 | 0 | normal 5 | batch |
| `lodepng_realloc` | yes | 4 | 8 | normal 3, signal 1 | per-input (3/4 completed) |
| `lodepng_set32bitInt` | yes | 5 | 0 | normal 5 | batch |
| `lodepng_strlen` | yes | 17 | 0 | normal 17 | batch |
| `paethPredictor` | yes | 7 | 0 | normal 7 | batch |
| `removePaddingBits` | yes | 34 | 718 | normal 33, signal 1 | per-input (33/34 completed) |
| `reverseBits` | yes | 10 | 67 | normal 9, ub-gated 1 | per-input (9/10 completed) |
| `searchCodeIndex` | yes | 1 | 1 | normal 1 | batch |
| `uivector_cleanup` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `unfilter` | yes | 186 | 15262 | normal 186 | batch |
| `unfilterScanline` | yes | 48 | 7626 | normal 48 | batch |
| `update_adler32` | yes | 15 | 0 | normal 15 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no transpiled driver; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 54 harnesses, 4 crash-all (`lodepng_chunk_check_crc` accepted, `lodepng_chunk_find` accepted, `lodepng_chunk_generate_crc` accepted, `uivector_cleanup` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`, `3d43f75aea807da1 (bin reused from the killed run)` — more than one: see deviations.

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 236 | 0 | 54 | 0 | 0 | 54 | 182 | 0.000 | 0.229 |
| regions | 13260 | 0 | 1675 | 0 | 0 | 1675 | 11585 | 0.000 | 0.126 |

Sanity checks: function pass, region pass. Harnesses unioned: 47. Identities outside the universe (excluded, never added): 0 fn / 7 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `Adam7_deinterlace` | 9 | 9 | 9 | 9 | 9 |
| `Adam7_interlace` | 14 | 14 | 14 | 14 | 14 |
| `addColorBits` | 6 | 6 | 6 | 6 | 6 |
| `addPaddingBits` | 13 | 20 | 21 | 21 | 22 |
| `adler32` | 12 | 12 | 12 | 12 | 12 |
| `alloc_string` | 19 | 19 | 19 | 19 | 19 |
| `checkColorValidity` | 16 | 16 | 16 | 16 | 16 |
| `filterScanline` | 69 | 76 | 77 | 89 | 91 |
| `getHash` | 18 | 18 | 18 | 18 | 18 |
| `getNumColorChannels` | 6 | 6 | 6 | 6 | 6 |
| `getValueRequiredBits` | 4 | 4 | 4 | 4 | 4 |
| `ilog2` | 8 | 8 | 8 | 8 | 8 |
| `ilog2i` | 8 | 8 | 8 | 8 | 8 |
| `isGrayICCProfile` | 27 | 27 | 27 | 27 | 27 |
| `isRGBICCProfile` | 25 | 26 | 26 | 26 | 26 |
| `lodepng_chunk_ancillary` | 6 | 6 | 6 | 6 | 6 |
| `lodepng_chunk_data` | 10 | 10 | 10 | 10 | 10 |
| `lodepng_chunk_data_const` | 10 | 10 | 10 | 10 | 10 |
| `lodepng_chunk_find_const` | 41 | 44 | 44 | 44 | 45 |
| `lodepng_chunk_length` | 5 | 5 | 5 | 5 | 5 |
| `lodepng_chunk_next` | 11 | 15 | 15 | 15 | 15 |
| `lodepng_chunk_next_const` | 14 | 14 | 14 | 14 | 14 |
| `lodepng_chunk_private` | 8 | 8 | 8 | 8 | 8 |
| `lodepng_chunk_safetocopy` | 8 | 8 | 8 | 8 | 8 |
| `lodepng_chunk_type` | 14 | 14 | 14 | 14 | 14 |
| `lodepng_chunk_type_equals` | 18 | 18 | 18 | 18 | 18 |
| `lodepng_crc32` | 13 | 13 | 13 | 13 | 13 |
| `lodepng_error_text` | 5 | 5 | 5 | 5 | 5 |
| `lodepng_get_bpp_lct` | 9 | 9 | 9 | 9 | 9 |
| `lodepng_get_raw_size_idat` | 13 | 13 | 13 | 13 | 13 |
| `lodepng_get_raw_size_lct` | 17 | 17 | 17 | 17 | 17 |
| `lodepng_gtofl` | 9 | 9 | 9 | 9 | 9 |
| `lodepng_huffman_code_lengths` | 21 | 22 | 23 | 23 | 23 |
| `lodepng_malloc` | 4 | 4 | 4 | 4 | 4 |
| `lodepng_memset` | 6 | 7 | 7 | 7 | 7 |
| `lodepng_read32bitInt` | 5 | 5 | 5 | 5 | 5 |
| `lodepng_realloc` | 4 | 4 | 4 | 4 | 4 |
| `lodepng_set32bitInt` | 5 | 5 | 5 | 5 | 5 |
| `lodepng_strlen` | 17 | 17 | 17 | 17 | 17 |
| `paethPredictor` | 7 | 7 | 7 | 7 | 7 |
| `removePaddingBits` | 13 | 27 | 32 | 33 | 34 |
| `reverseBits` | 10 | 10 | 10 | 10 | 10 |
| `unfilter` | 40 | 111 | 143 | 174 | 186 |
| `unfilterScanline` | 40 | 48 | 48 | 48 | 48 |
| `update_adler32` | 15 | 15 | 15 | 15 | 15 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 856 |
| signal | 14 |
| timeout | 6 |
| ub-gated | 3 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `Adam7_deinterlace` | 85 / 85 | inconclusive 1, ub_associated 84 | 3 |
| `Adam7_interlace` | 206 / 250 | inconclusive 5, ub_associated 201 | 6 |
| `addColorBits` | 18 / 18 | ub_associated_termination 18 | 2 |
| `addPaddingBits` | 201 / 501 | ub_associated 201 | 2 |
| `alloc_string_sized` | 3 / 3 | ub_associated 3 | 1 |
| `countZeros` | 3 / 3 | ub_associated 3 | 2 |
| `filterScanline` | 200 / 500 | ub_associated 200 | 8 |
| `lodepng_chunk_check_crc` | 3 / 3 | ub_associated 3 | 2 |
| `lodepng_chunk_find` | 3 / 3 | ub_associated 3 | 1 |
| `lodepng_chunk_find_const` | 200 / 500 | ub_associated 200 | 5 |
| `lodepng_chunk_generate_crc` | 3 / 3 | ub_associated 3 | 1 |
| `lodepng_huffman_code_lengths` | 200 / 500 | ub_associated_termination 200 | 1 |
| `lodepng_malloc` | 9 / 9 | ub_associated 9 | 2 |
| `lodepng_memcpy` | 3 / 3 | ub_associated_termination 3 | 1 |
| `lodepng_memset` | 26 / 26 | ub_associated_termination 26 | 1 |
| `lodepng_realloc` | 9 / 9 | ub_associated 9 | 2 |
| `removePaddingBits` | 201 / 501 | ub_associated 201 | 3 |
| `reverseBits` | 67 / 67 | ub_associated_termination 67 | 1 |
| `searchCodeIndex` | 1 / 1 | inconclusive 1 | 1 |
| `uivector_cleanup` | 3 / 3 | ub_associated_termination 3 | 1 |
| `unfilter` | 200 / 500 | ub_associated 200 | 10 |
| `unfilterScanline` | 200 / 500 | ub_associated 200 | 5 |

Total: inconclusive 7, ub_associated 1520, ub_associated_termination 317

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** (1) The cell's scratch directory was deleted before its coverage analysis existed;
the archived llvm-cov exports were intact, but each records the absolute path of its harness's
`src/lib.rs` (needed for the `--expose-entry` line alignment). The harnesses were regenerated from
the archived plans (`rebuild_bins.py`, deterministic, no fuzzing) and `c2r_coverage.py` was run with
an explicit `--path-map` (recorded in `analysis/result.json`; procedure and hashes in
`analysis/recovery.json`). Accepted with 0 functions outside the universe. (2) Six sampled rows on
`Adam7_interlace` (5) and `Adam7_deinterlace` (1) had been labelled `confirmed_termination` by a
classifier inversion — C alone timed out under ASan+UBSan and the translation returned; that is no
reference execution, not a termination defect. They were re-classified offline to `inconclusive`
from the archived verdict rows (summary.json carries the note); the classifier is fixed. (3) §4–§6
regenerated from the archive.

**What the cell says.** 235 boundaries matched, 64 planned: 171 fail at the signature for one
reason class — struct-invariant parameters (`LodePNGInfo` with nested structs, `ucvector` and the
bit reader/writer/HuffmanTree with owning pointer fields, `T**` outputs the callee allocates), which
the frozen plan generator does not construct. 54 built, 47 exported, corpus 879. Reach 54/236
functions (0.229), 1 675/13 260 regions (0.126). Four boundaries are pre-accepted C-side crash-alls
(`lodepng_chunk_check_crc`, `lodepng_chunk_generate_crc`, `uivector_cleanup`, `lodepng_chunk_find`:
a chunk pointer must address a real chunk header; `uivector_cleanup` frees memory the harness owns).
Sample after re-classification: 1 520 `ub_associated`, 317 `ub_associated_termination`,
7 `inconclusive`: **nothing confirmed** — c2rust clean for an eighth library.

**Not established / limits.** `lodepng_chunk_find`/`lodepng_chunk_next` take a `(begin, end)`
range that the planner models as two independent buffers, so the C side reads between unrelated
allocations (layout luck, adjudicated `ub_associated`). A range-pair rule is recorded for the next
generator version and was not applied mid-chain; the affected boundaries contribute noise, not
findings. lodepng ships no test target: TEST-UNAVAILABLE, denominator from the instrumented rlib.
