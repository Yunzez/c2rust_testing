# genann × c2rust — RQ4 cell (plan pipeline)

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
| `genann_copy` | no | 15 | 875 | normal 15 | batch |
| `genann_free` | no | 15 | 0 | normal 15 | batch |
| `genann_init` | no | 27 | 755 | normal 26, signal 1 | per-input (26/27 completed) |
| `genann_randomize` | no | 12 | 0 | normal 12 | batch |
| `genann_run` | no | 34 | 0 | normal 34 | batch |
| `genann_train` | no | 44 | 0 | normal 44 | batch |

## 3. Tests side

Status **PASS**. 

Mode used for the partition: **measured**.

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 12 | 11 | 10 | 9 | 2 | 1 | 0 | 0.917 | 0.833 |
| regions | 573 | 513 | 462 | 443 | 70 | 19 | 41 | 0.895 | 0.806 |

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
| `genann_randomize` | 12 | 12 | 12 | 12 | 12 |
| `genann_run` | 34 | 34 | 34 | 34 | 34 |
| `genann_train` | 44 | 44 | 44 | 44 | 44 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 180 |
| ub-gated | 2 |
| signal | 1 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `genann_copy` | 200 / 875 | not_reproducible 200 | 1 |
| `genann_init` | 201 / 756 | ub_associated 21, ub_associated_termination 180 | 6 |

Total: not_reproducible 200, ub_associated 21, ub_associated_termination 180

<!-- prose -->

## 7. Procedure, deviations, and what is not established

**Procedure.** As `../../bzip2/c2rust/RUN.md` §7 with one change: the planner is the producer-bridge pilot
(`docs/producer_bridge_pilot.md`). Boundaries taking `genann*` are run as the sequence
`srand(42) → genann_init(inputs, hidden_layers, hidden, outputs) → target → genann_free` on each side, the four
producer scalars decoded once and capped at 32 (`policy.producer_scalar_max`). No shipped sample inputs exist
(inputs are doubles), so only the fixed seed applies. Tests side: the transpiled `test.rs` (minctest, 9 groups /
38 assertions) through a two-line adapter, exit 0 = PASS.

**Deviations recorded.** The first cell attempt failed to build `genann_init`: a one-sided rejection guard
(`if (inputs < 1) return 0;`) had no lowering; it now lowers as a full-range scalar and the cell was restarted
(8 minutes lost, no data used from the first attempt). Consequence, kept and reported: `genann_init` as a target
allocates arbitrarily large objects, so its artifacts are overflow panics and OOMs (adjudicated `ub_associated*`).
`genann_copy` returns a fresh object the harness never frees (rung 3 compares nullness), so its fork-mode
children hit the rss limit: `oom-*` artifacts that replay normally (`not_reproducible`).

**What is established.** Suite 513 / 573 regions (0.895), validator 462 (0.806); both 443, only-tests 70,
only-ours 19; functions 11 vs 10 of 12. Only-tests is `genann_read` / `genann_write` (`FILE*`); only-ours is
`genann_act_linear` and branches of `run` / `train`. **Negative control**: 183 corpus inputs replayed with C
beside Rust, 0 divergences; 401 sampled artifacts, 0 confirmed. **Producer-bridge ablation from this campaign**:
the five scalar-only harnesses alone reach 131 / 573 regions (0.229); with the bridge 462 (0.806).

**What is not established.** `genann_run`'s and `genann_train`'s output *values* are not compared (interior
pointer / void: nullness and the input arrays only); a wrong network output would be invisible here, which is
why this cell's value oracle lives in the activation boundaries. Corpora are tiny (8–44 inputs); the hour
buys nothing on this library and is kept for uniformity. Single campaign.
