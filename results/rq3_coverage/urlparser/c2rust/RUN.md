# urlparser × c2rust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 21 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 21 |
| built | 20 |
| executed (corpus > 0) | 20 |
| coverage exported | 9 |

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `get_part` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `strdup` | no | 15 | 0 | normal 15 | batch |
| `strff` | yes | 15 | 0 | normal 15 | batch |
| `strrwd` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `url_data_inspect` | no | 2 | 5 | normal 2 | batch |
| `url_free` | no | 2 | 3 | normal 2 | batch |
| `url_get_auth` | no | 1 | 2 | normal 1 | batch |
| `url_get_hash` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_host` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_hostname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_path` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_pathname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_port` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_protocol` | no | 1 | 2 | normal 1 | batch |
| `url_get_query` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_search` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_inspect` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_is_protocol` | no | 1 | 0 | normal 1 | batch |
| `url_is_ssh` | no | 1 | 0 | normal 1 | batch |
| `url_parse` | no | 1 | 1 | normal 1 | batch |

## 3. Tests side

Status **PASS**. test program exits 0; raw/tests_run.log

Mode used for the partition: **measured**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 20 harnesses, 9 crash-all (`url_get_hash` accepted, `url_get_host` accepted, `url_get_hostname` accepted, `url_get_path` accepted, `url_get_pathname` accepted, `url_get_port` accepted, `url_get_query` accepted, `url_get_search` accepted, `url_inspect` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 22 | 21 | 7 | 7 | 14 | 0 | 1 | 0.955 | 0.318 |
| regions | 1202 | 893 | 110 | 105 | 788 | 5 | 304 | 0.743 | 0.092 |

Sanity checks: function pass, region pass. Harnesses unioned: 9. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `strdup` | 15 | 15 | 15 | 15 | 15 |
| `strff` | 15 | 15 | 15 | 15 | 15 |
| `url_data_inspect` | 2 | 2 | 2 | 2 | 2 |
| `url_free` | 2 | 2 | 2 | 2 | 2 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 39 |
| signal | 11 |

<!-- prose -->
