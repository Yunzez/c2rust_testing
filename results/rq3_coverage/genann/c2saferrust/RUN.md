# genann × c2saferrust — RQ4 cell (plan pipeline)

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
| `genann_copy` | no | 13 | 134 | normal 13 | batch |
| `genann_free` | no | 16 | 0 | normal 16 | batch |
| `genann_init` | no | 27 | 782 | normal 26, signal 1 | per-input (26/27 completed) |
| `genann_randomize` | no | 13 | 0 | normal 13 | batch |
| `genann_run` | no | 36 | 0 | normal 36 | batch |
| `genann_train` | no | 45 | 0 | normal 45 | batch |

## 3. Tests side

Status **TEST-FAILS**. groups basic..copy pass (7 assertions each), then `persist` aborts: genann_write(ann, &mut std::fs::File) -> writeln!(..).unwrap() on EBADF (Bad file descriptor). C2SaferRust reshaped FILE* into &mut File; the transpiled test's tmpfile() path does not produce a usable File. Driver-level observation; not a fuzz finding.

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 12 | 0 | 10 | 0 | 0 | 10 | 2 | 0.000 | 0.833 |
| regions | 563 | 0 | 459 | 0 | 0 | 459 | 104 | 0.000 | 0.815 |

Sanity checks: function pass, region pass. Harnesses unioned: 10. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `genann_act_linear` | 8 | 8 | 8 | 8 | 8 |
| `genann_act_sigmoid` | 10 | 10 | 10 | 10 | 10 |
| `genann_act_sigmoid_cached` | 10 | 10 | 10 | 10 | 10 |
| `genann_act_threshold` | 8 | 8 | 8 | 8 | 8 |
| `genann_copy` | 13 | 13 | 13 | 13 | 13 |
| `genann_free` | 16 | 16 | 16 | 16 | 16 |
| `genann_init` | 18 | 21 | 21 | 27 | 27 |
| `genann_randomize` | 13 | 13 | 13 | 13 | 13 |
| `genann_run` | 36 | 36 | 36 | 36 | 36 |
| `genann_train` | 45 | 45 | 45 | 45 | 45 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 183 |
| ub-gated | 2 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `genann_copy` | 134 / 134 | not_reproducible 134 | 1 |
| `genann_init` | 201 / 783 | ub_associated 19, ub_associated_termination 182 | 6 |

Total: not_reproducible 134, ub_associated 19, ub_associated_termination 182

<!-- prose -->

## 7. Procedure, deviations, and what is not established

As `../c2rust/RUN.md` §7. The tests side is **not a baseline**: the transpiled suite passes `basic` … `copy`
and aborts in `persist` — `genann_write(ann, &mut std::fs::File)` calls `writeln!(..).unwrap()` on a bad
descriptor (C2SaferRust reshaped `FILE*` into `&mut File`; the transpiled `tmpfile()` path yields no usable
`File`). A driver-level observation, not a fuzz finding. Universe from the denominator build; partition
Ours / Neither: 10 / 12 functions, 459 / 563 regions (0.815). **Negative control holds**: 0 divergences on 186
corpus inputs, 0 confirmed of 335 sampled — consistent with the E1 50M-record certificate. Not established:
`run` / `train` output values; the two `FILE*` boundaries; single campaign.
