# cjson × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 58 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 39 |
| built | 39 |
| executed (corpus > 0) | 39 |
| coverage exported | 39 |

Plan failures, by the generator's own reason:

- **9** × signature: 2 produced objects in one call; ownership transfer between them cannot be ruled
- **8** × signature: struct-invariant param p: printbuffer has pointer field 'buffer' (needs invaria
- **1** × signature: struct-invariant param hooks: cJSON_Hooks has pointer field 'malloc_fn' (needs 
- **1** × it is dereferenced and written rather than indexed, so it is an OUT pointer whose value is

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `cJSON_CreateArray` | no | 1 | 0 | normal 1 | batch |
| `cJSON_CreateBool` | no | 5 | 1 | normal 5 | batch |
| `cJSON_CreateDoubleArray` | no | 19 | 3 | normal 9, ub-gated 10 | batch |
| `cJSON_CreateFalse` | no | 1 | 0 | normal 1 | batch |
| `cJSON_CreateFloatArray` | no | 20 | 1 | normal 12, ub-gated 8 | batch |
| `cJSON_CreateIntArray` | no | 20 | 3 | normal 20 | batch |
| `cJSON_CreateNull` | no | 1 | 0 | normal 1 | batch |
| `cJSON_CreateNumber` | no | 8 | 0 | normal 8 | batch |
| `cJSON_CreateObject` | no | 1 | 0 | normal 1 | batch |
| `cJSON_CreateString` | no | 16 | 0 | normal 16 | batch |
| `cJSON_CreateStringArray` | no | 41 | 0 | normal 41 | batch |
| `cJSON_CreateTrue` | no | 1 | 0 | normal 1 | batch |
| `cJSON_Delete` | no | 426 | 5652 | normal 400, ub-gated 26 | batch |
| `cJSON_DeleteItemFromArray` | no | 465 | 5539 | normal 440, ub-gated 25 | batch |
| `cJSON_DeleteItemFromObject` | no | 470 | 5235 | normal 462, ub-gated 8 | batch |
| `cJSON_DetachItemFromArray` | no | 443 | 5123 | normal 433, ub-gated 10 | batch |
| `cJSON_DetachItemFromObject` | no | 426 | 5203 | normal 419, ub-gated 7 | batch |
| `cJSON_Duplicate` | no | 440 | 5611 | normal 431, ub-gated 9 | batch |
| `cJSON_GetArrayItem` | no | 433 | 5270 | normal 410, ub-gated 23 | batch |
| `cJSON_GetArraySize` | no | 433 | 5072 | normal 409, ub-gated 24 | batch |
| `cJSON_GetErrorPtr` | no | 1 | 0 | normal 1 | batch |
| `cJSON_GetObjectItem` | no | 447 | 5426 | normal 440, ub-gated 7 | batch |
| `cJSON_Minify` | no | 1 | 1 | normal 1 | batch |
| `cJSON_New_Item` | yes | 1 | 1 | normal 1 | batch |
| `cJSON_Parse` | no | 410 | 5492 | normal 390, ub-gated 20 | batch |
| `cJSON_Print` | no | 600 | 4698 | normal 552, ub-gated 48 | batch |
| `cJSON_PrintBuffered` | no | 476 | 5037 | normal 443, ub-gated 33 | batch |
| `cJSON_PrintUnformatted` | no | 547 | 4811 | normal 513, ub-gated 34 | batch |
| `cJSON_strcasecmp` | yes | 1 | 1 | normal 1 | batch |
| `cJSON_strdup` | yes | 16 | 0 | normal 16 | batch |
| `create_reference` | yes | 424 | 5508 | normal 408, ub-gated 16 | batch |
| `parse_array` | yes | 445 | 5181 | normal 431, ub-gated 14 | batch |
| `parse_hex4` | yes | 5 | 11 | normal 5 | batch |
| `parse_number` | yes | 423 | 5323 | normal 406, ub-gated 17 | batch |
| `parse_object` | yes | 438 | 5313 | normal 431, ub-gated 7 | batch |
| `parse_string` | yes | 421 | 5184 | normal 406, ub-gated 15 | batch |
| `parse_value` | yes | 441 | 5277 | normal 430, ub-gated 11 | batch |
| `pow2gt` | yes | 6 | 22 | normal 6 | batch |
| `skip` | yes | 23 | 0 | normal 23 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. driver not transpiled; denominator: 60 functions

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `unrecorded`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 59 | 0 | 49 | 0 | 0 | 49 | 10 | 0.000 | 0.831 |
| regions | 2237 | 0 | 1816 | 0 | 0 | 1816 | 421 | 0.000 | 0.812 |

Sanity checks: function pass, region pass. Harnesses unioned: 39. Identities outside the universe (excluded, never added): 0 fn / 6 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `cJSON_CreateBool` | 5 | 5 | 5 | 5 | 5 |
| `cJSON_CreateDoubleArray` | 19 | 19 | 19 | 19 | 19 |
| `cJSON_CreateFloatArray` | 20 | 20 | 20 | 20 | 20 |
| `cJSON_CreateIntArray` | 20 | 20 | 20 | 20 | 20 |
| `cJSON_CreateNumber` | 8 | 8 | 8 | 8 | 8 |
| `cJSON_CreateString` | 16 | 16 | 16 | 16 | 16 |
| `cJSON_CreateStringArray` | 41 | 41 | 41 | 41 | 41 |
| `cJSON_Delete` | 282 | 419 | 426 | 426 | 426 |
| `cJSON_DeleteItemFromArray` | 380 | 458 | 465 | 465 | 465 |
| `cJSON_DeleteItemFromObject` | 339 | 460 | 470 | 470 | 470 |
| `cJSON_DetachItemFromArray` | 299 | 438 | 443 | 443 | 443 |
| `cJSON_DetachItemFromObject` | 283 | 412 | 426 | 426 | 426 |
| `cJSON_Duplicate` | 277 | 399 | 440 | 440 | 440 |
| `cJSON_GetArrayItem` | 331 | 424 | 433 | 433 | 433 |
| `cJSON_GetArraySize` | 339 | 428 | 433 | 433 | 433 |
| `cJSON_GetObjectItem` | 332 | 439 | 447 | 447 | 447 |
| `cJSON_Parse` | 289 | 363 | 410 | 410 | 410 |
| `cJSON_Print` | 418 | 559 | 600 | 600 | 600 |
| `cJSON_PrintBuffered` | 308 | 457 | 476 | 476 | 476 |
| `cJSON_PrintUnformatted` | 372 | 537 | 547 | 547 | 547 |
| `cJSON_strdup` | 16 | 16 | 16 | 16 | 16 |
| `create_reference` | 324 | 410 | 424 | 424 | 424 |
| `parse_array` | 341 | 441 | 445 | 445 | 445 |
| `parse_hex4` | 5 | 5 | 5 | 5 | 5 |
| `parse_number` | 336 | 409 | 423 | 423 | 423 |
| `parse_object` | 339 | 420 | 438 | 438 | 438 |
| `parse_string` | 315 | 412 | 421 | 421 | 421 |
| `parse_value` | 301 | 425 | 441 | 441 | 441 |
| `pow2gt` | 6 | 6 | 6 | 6 | 6 |
| `skip` | 23 | 23 | 23 | 23 | 23 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 8424 |
| ub-gated | 372 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `cJSON_CreateBool` | 1 / 1 | not_reproducible 1 | 1 |
| `cJSON_CreateDoubleArray` | 3 / 3 | not_reproducible 2, ub_associated 1 | 2 |
| `cJSON_CreateFloatArray` | 1 / 1 | not_reproducible 1 | 1 |
| `cJSON_CreateIntArray` | 3 / 3 | not_reproducible 3 | 1 |
| `cJSON_Delete` | 200 / 500 | not_reproducible 6, ub_associated 190, ub_associated_termination 4 | 9 |
| `cJSON_DeleteItemFromArray` | 200 / 500 | not_reproducible 2, ub_associated 192, ub_associated_termination 6 | 9 |
| `cJSON_DeleteItemFromObject` | 200 / 500 | not_reproducible 8, ub_associated 178, ub_associated_termination 14 | 12 |
| `cJSON_DetachItemFromArray` | 200 / 500 | not_reproducible 6, ub_associated 193, ub_associated_termination 1 | 9 |
| `cJSON_DetachItemFromObject` | 200 / 500 | not_reproducible 1, ub_associated 196, ub_associated_termination 3 | 10 |
| `cJSON_Duplicate` | 200 / 500 | not_reproducible 4, ub_associated 196 | 7 |
| `cJSON_GetArrayItem` | 200 / 500 | not_reproducible 2, ub_associated 194, ub_associated_termination 4 | 10 |
| `cJSON_GetArraySize` | 200 / 500 | not_reproducible 5, ub_associated 186, ub_associated_termination 9 | 9 |
| `cJSON_GetObjectItem` | 200 / 500 | not_reproducible 4, ub_associated 191, ub_associated_termination 5 | 10 |
| `cJSON_Minify` | 1 / 1 | not_reproducible 1 | 1 |
| `cJSON_New_Item` | 1 / 1 | not_reproducible 1 | 1 |
| `cJSON_Parse` | 200 / 500 | not_reproducible 3, ub_associated 196, ub_associated_termination 1 | 10 |
| `cJSON_Print` | 200 / 500 | not_reproducible 6, ub_associated 188, ub_associated_termination 6 | 9 |
| `cJSON_PrintBuffered` | 200 / 500 | not_reproducible 5, ub_associated 193, ub_associated_termination 2 | 11 |
| `cJSON_PrintUnformatted` | 200 / 500 | not_reproducible 4, ub_associated 194, ub_associated_termination 2 | 11 |
| `cJSON_strcasecmp` | 1 / 1 | ub_associated 1 | 1 |
| `create_reference` | 200 / 500 | not_reproducible 5, ub_associated 195 | 9 |
| `parse_array` | 200 / 500 | not_reproducible 6, ub_associated 187, ub_associated_termination 7 | 9 |
| `parse_hex4` | 11 / 11 | not_reproducible 7, ub_associated 4 | 2 |
| `parse_number` | 200 / 500 | not_reproducible 3, ub_associated 194, ub_associated_termination 3 | 9 |
| `parse_object` | 200 / 500 | not_reproducible 4, ub_associated 195, ub_associated_termination 1 | 10 |
| `parse_string` | 200 / 500 | not_reproducible 8, ub_associated 190, ub_associated_termination 2 | 9 |
| `parse_value` | 200 / 500 | not_reproducible 5, ub_associated 195 | 7 |
| `pow2gt` | 22 / 22 | not_reproducible 12, ub_associated_termination 10 | 2 |

Total: not_reproducible 115, ub_associated 3649, ub_associated_termination 80

<!-- prose -->


## 7. Procedure, deviations, and what is not established

**Procedure.** As `../../bzip2/c2rust/RUN.md` §7 under the producer-bridge pilot's cJSON generalisation
(`docs/producer_bridge_pilot.md` §6a): 18 boundaries take a `cJSON*` built on each side by
`cJSON_Parse` from a fuzz-controlled JSON text (ranked first of 14 candidate producers by reachability —
a deterministic heuristic) and released by `cJSON_Delete`; the cJSON comparator plugin canonicalises the
produced object after the producer (a difference there is the producer's) and after the target; 9
boundaries with two produced objects are refused (ownership). Tests side: TEST-UNAVAILABLE (only cJSON.c
was translated); universe from a link-dead-code denominator, partition Ours / Neither. The shipped
`tests/main.c` is used as producer-selection evidence only.

**Deviations recorded, in order.** (1) The cell died after its 3 600 s campaign, on the scratchpad's
file-count quota: 39 boundaries had written 99 999 crash artifacts. `finish_cell.py` reconstructed
`funnel.json` and collected coverage from the intact corpus, snapshots and binaries; candidates were
pruned to 500 per channel per boundary with a gzipped sha256 manifest of every artifact — nothing was
re-fuzzed. (2) The first replay reported 149 divergences on this faithful translation, all at the
producer phase: the in-loop UB gate wrapped only the target's C call, and `cJSON_Parse`'s own UB
(`(int)double` on out-of-range numbers) reached the object-state comparison; the confirmation sample
adjudicated them `ub_associated_value` (C-only under full UBSan fires). The generator now gates the
producer's C call; the 39 discovery binaries were rebuilt with it (campaign kept) and replay,
confirmation and this archive redone. (3) Two single-TU packaging bugs were fixed before the campaign
(static stripping looked for sibling files; forward-declared statics kept a static definition); the
first confirmation pass lacked `c_static` in the reconstructed funnel and is superseded by the second.

**What is established.** 39 of 58 boundaries plan and build (12 of them C `static`); 49 of 59 functions
(0.831) and 1 816 of 2 237 regions (0.812) reached. **Producer-bridge ablation from the same campaign**:
the 21 harnesses without a produced object reach 29 functions (0.492) and 791 regions (0.354).
Corpora on produced boundaries are 420–600 inputs each. **Negative control**: the gated replay of the
8 796-input corpus gives 0 divergences (372 `ub-gated`), and 3 844 sampled artifacts adjudicate to
0 confirmed (3 649 `ub_associated`, 80 `ub_associated_termination`, 115 `not_reproducible`).

**What is not established, and the noise.** Almost every artifact is the reference's own bug — old
cJSON's `parse_string` writes one byte past its allocation on `\u` escapes (reproduced C-only under
ASan) — inherited by the producer; the sampled 200 per boundary are all `ub_associated`, the totals
(≈ 2 500 per boundary per hour) are in `candidates_manifest.json.gz`. `not_reproducible` are the
returned-object leaks (`cJSON_Duplicate`, `cJSON_Create*`) hitting the rss limit in fork mode. The 9
refused boundaries (`cJSON_AddItemTo*`, `ReplaceItem*`, `InsertItemInArray`) and the 8 `printbuffer*`
internals are not measured. No second translation of cJSON is bridged yet: PtrTrans's
`cJSON_Parse(Option<&[u8]>) -> Option<&mut cJSON>` returns an `Option<&mut T>`, outside the pilot's
raw-pointer rule. Single campaign.
