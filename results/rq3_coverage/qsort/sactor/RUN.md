# qsort × sactor — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 3 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 3 |
| built | 3 |
| executed (corpus > 0) | 3 |
| coverage exported | 3 |

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `partition` | no | 41 | 0 | normal 41 | batch |
| `quickSort` | no | 83 | 0 | normal 83 | batch |
| `swap` | no | 8 | 0 | normal 8 | batch |

## 3. Tests side

Status **TEST-UNAVAILABLE**. no shipped suite; denominator: 8 functions (nested helper fns and prog_main are separate identities)

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 3 harnesses, 0 crash-all (none).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `7ad221ba41608c0e`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 8 | 0 | 6 | 0 | 0 | 6 | 2 | 0.000 | 0.750 |
| regions | 196 | 0 | 99 | 0 | 0 | 99 | 97 | 0.000 | 0.505 |

Sanity checks: function pass, region pass. Harnesses unioned: 3. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `partition` | 32 | 41 | 41 | 41 | 41 |
| `quickSort` | 74 | 76 | 78 | 81 | 83 |
| `swap` | 8 | 8 | 8 | 8 | 8 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 132 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|

Total: 

<!-- prose -->
