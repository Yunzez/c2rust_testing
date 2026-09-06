# genann × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after `<!-- prose -->` is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 12 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 10 |
| built | 10 |
| executed (corpus > 0) | 10 |
| coverage exported | 10 |

Plan failures, by the generator's own reason:

- **1** × signature: struct-invariant param in_: FILE has pointer field '_IO_read_ptr' (needs invari
- **1** × signature: struct-invariant param out: FILE has pointer field '_IO_read_ptr' (needs invari

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `genann_act_linear` | no | 8 | 0 | normal 8 | batch |
| `genann_act_sigmoid` | no | 10 | 0 | normal 10 | batch |
| `genann_act_sigmoid_cached` | no | 10 | 0 | normal 8, ub-gated 2 | batch |
| `genann_act_threshold` | no | 8 | 0 | normal 8 | batch |
| `genann_copy` | no | 15 | 469 | normal 15 | batch |
| `genann_free` | no | 15 | 0 | normal 15 | batch |
| `genann_init` | no | 27 | 774 | normal 26, signal 1 | per-input (26/27 completed) |
| `genann_randomize` | no | 14 | 0 | normal 14 | batch |
| `genann_run` | no | 38 | 0 | normal 38 | batch |
| `genann_train` | no | 45 | 0 | normal 45 | batch |

## 3. Tests side

Status **PASS**. 

Mode used for the partition: **measured**.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 17 | 13 | 12 | 11 | 2 | 1 | 3 | 0.765 | 0.706 |
| regions | 587 | 518 | 467 | 448 | 70 | 19 | 50 | 0.882 | 0.796 |

Sanity checks: function pass, region pass. Harnesses unioned: 10. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `genann_act_linear` | 8 | 8 | 8 | 8 | 8 |
| `genann_act_sigmoid` | 10 | 10 | 10 | 10 | 10 |
| `genann_act_sigmoid_cached` | 10 | 10 | 10 | 10 | 10 |
| `genann_act_threshold` | 8 | 8 | 8 | 8 | 8 |
| `genann_copy` | 15 | 15 | 15 | 15 | 15 |
| `genann_free` | 15 | 15 | 15 | 15 | 15 |
| `genann_init` | 18 | 21 | 21 | 27 | 27 |
| `genann_randomize` | 14 | 14 | 14 | 14 | 14 |
| `genann_run` | 38 | 38 | 38 | 38 | 38 |
| `genann_train` | 45 | 45 | 45 | 45 | 45 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 187 |
| ub-gated | 2 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `genann_copy` | 200 / 469 | not_reproducible 200 | 1 |
| `genann_init` | 201 / 775 | ub_associated 21, ub_associated_termination 180 | 6 |

Total: not_reproducible 200, ub_associated 21, ub_associated_termination 180

<!-- prose -->

## 7. Procedure, deviations, and what is not established

As `../c2rust/RUN.md` §7 (same planner, same sequence harness, same noise classes on `genann_init` and
`genann_copy`, adjudicated and not promoted). The suite passes through this translation, so the partition is
paired. **Negative control holds**: 0 divergences on the corpus replay, 0 confirmed artifacts in the sample —
consistent with the E1 certificate for this translation. The producer-bridge ablation from this campaign is in
`../ablation_producer_bridge.json`. Not established: `run` / `train` output values (pilot oracle limit); single
campaign; tiny corpora.
