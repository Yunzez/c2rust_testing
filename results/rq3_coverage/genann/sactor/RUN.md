# genann × sactor — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after `<!-- prose -->` is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 15 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 13 |
| built | 13 |
| executed (corpus > 0) | 13 |
| coverage exported | 13 |

Plan failures, by the generator's own reason:

- **1** × signature: struct-invariant param in_: FILE has pointer field '_IO_read_ptr' (needs invari
- **1** × signature: struct-invariant param out: FILE has pointer field '_IO_read_ptr' (needs invari

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `genann_act_hidden_indirect` | no | 21 | 3979 | divergence 17, normal 4 | batch |
| `genann_act_linear` | no | 20 | 0 | normal 20 | batch |
| `genann_act_output_indirect` | no | 21 | 3979 | divergence 17, normal 4 | batch |
| `genann_act_sigmoid` | no | 22 | 0 | normal 22 | batch |
| `genann_act_sigmoid_cached` | no | 21 | 3988 | divergence 17, normal 4 | batch |
| `genann_act_threshold` | no | 19 | 0 | normal 19 | batch |
| `genann_copy` | no | 16 | 630 | normal 16 | batch |
| `genann_free` | no | 16 | 0 | normal 16 | batch |
| `genann_init` | no | 25 | 803 | normal 24, signal 1 | per-input (24/25 completed) |
| `genann_init_sigmoid_lookup` | no | 16 | 0 | normal 16 | batch |
| `genann_randomize` | no | 15 | 0 | normal 15 | batch |
| `genann_run` | no | 46 | 0 | normal 46 | batch |
| `genann_train` | no | 53 | 0 | normal 53 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. single lib.rs, no transpiled driver; universe from the denom bin

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 21 | 0 | 15 | 0 | 0 | 15 | 6 | 0.000 | 0.714 |
| regions | 716 | 0 | 506 | 0 | 0 | 506 | 210 | 0.000 | 0.707 |

Sanity checks: function pass, region pass. Harnesses unioned: 13. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `genann_act_hidden_indirect` | 21 | 21 | 21 | 21 | 21 |
| `genann_act_linear` | 20 | 20 | 20 | 20 | 20 |
| `genann_act_output_indirect` | 21 | 21 | 21 | 21 | 21 |
| `genann_act_sigmoid` | 22 | 22 | 22 | 22 | 22 |
| `genann_act_sigmoid_cached` | 21 | 21 | 21 | 21 | 21 |
| `genann_act_threshold` | 19 | 19 | 19 | 19 | 19 |
| `genann_copy` | 16 | 16 | 16 | 16 | 16 |
| `genann_free` | 16 | 16 | 16 | 16 | 16 |
| `genann_init` | 20 | 25 | 25 | 25 | 25 |
| `genann_init_sigmoid_lookup` | 16 | 16 | 16 | 16 | 16 |
| `genann_randomize` | 15 | 15 | 15 | 15 | 15 |
| `genann_run` | 46 | 46 | 46 | 46 | 46 |
| `genann_train` | 53 | 53 | 53 | 53 | 53 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 259 |
| divergence | 51 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `genann_act_hidden_indirect` | 217 / 3996 | confirmed_divergence 17, ub_associated 200 | 2 |
| `genann_act_output_indirect` | 217 / 3996 | confirmed_divergence 17, ub_associated 200 | 2 |
| `genann_act_sigmoid_cached` | 217 / 4005 | confirmed_divergence 17, ub_associated 200 | 2 |
| `genann_copy` | 200 / 630 | not_reproducible 200 | 1 |
| `genann_init` | 201 / 804 | ub_associated_termination 201 | 4 |

Total: confirmed_divergence 51, not_reproducible 200, ub_associated 600, ub_associated_termination 201

<!-- prose -->

## 7. Procedure, deviations, and what is not established

As `../c2rust/RUN.md` §7, on the genann-1.0.0 source (15 functions; SACTOR's `lib.rs` carries
`genann_init_sigmoid_lookup` and the `*_indirect` activations, which the 2015 source lacks). No transpiled
driver exists, so the tests side is TEST-UNAVAILABLE and the universe comes from the denominator build:
15 / 21 functions, 506 / 716 regions (0.707) — the universe includes SACTOR's own helper shims.

**What is established.** In 1.0.0 every activation takes `const genann*`, so **none of the three cached-sigmoid
boundaries is reachable without the producer bridge** (planner 1 / 15 before, 13 / 15 with). On the corpus
replay, `genann_act_sigmoid_cached`, `genann_act_hidden_indirect` and `genann_act_output_indirect` diverge on
17 of 21 inputs each — the 4 that agree are the `a < −15` / `a > 15` clamp paths that never read the lookup
table — and all **51 adjudicate to `confirmed_divergence`** (C-only under ASan + full UBSan normal, ladder rung 2
return value). This is E1's headline #32 (the immutable lookup table whose initialisation is written away),
re-found through the generated sequence with no hand-written harness.

**What is not established, and the noise.** `genann_run` / `genann_train` show 0 divergences because their
output values are not compared (pilot oracle limit): the all-zero network outputs #32 produces are invisible
there. The ~4 000 `crash-*` artifacts on each cached-sigmoid boundary are NaN inputs that trip C's own
`assert(!isnan(a))` (genann.c:87), which SACTOR transpiled as well — both sides abort, `ub_associated`, and the
fuzzer's crash gradient chases them (corpus 21). `genann_init` overflow panics and `genann_copy` OOMs as on
every tool. Single campaign.
