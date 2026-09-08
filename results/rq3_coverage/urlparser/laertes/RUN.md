# urlparser × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 21 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 21 |
| built | 20 |
| executed (corpus > 0) | 0 |
| coverage exported | 0 |

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `get_part` | yes | 0 | 0 | — | None |
| `strdup` | no | 0 | 0 | — | None |
| `strff` | yes | 0 | 0 | — | None |
| `strrwd` | yes | 0 | 0 | — | None |
| `url_data_inspect` | no | 0 | 0 | — | None |
| `url_free` | no | 0 | 0 | — | None |
| `url_get_auth` | no | 0 | 0 | — | None |
| `url_get_hash` | no | 0 | 0 | — | None |
| `url_get_host` | no | 0 | 0 | — | None |
| `url_get_hostname` | no | 0 | 0 | — | None |
| `url_get_path` | no | 0 | 0 | — | None |
| `url_get_pathname` | no | 0 | 0 | — | None |
| `url_get_port` | no | 0 | 0 | — | None |
| `url_get_protocol` | no | 0 | 0 | — | None |
| `url_get_query` | no | 0 | 0 | — | None |
| `url_get_search` | no | 0 | 0 | — | None |
| `url_inspect` | no | 0 | 0 | — | None |
| `url_is_protocol` | no | 0 | 0 | — | None |
| `url_is_ssh` | no | 0 | 0 | — | None |
| `url_parse` | no | 0 | 0 | — | None |

## 3. Tests side

Status **TEST-FAILS**. exit 139 (SIGSEGV) with no output: the Laertes translation crashes inside the suite's first url_parse; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 20 harnesses, 15 crash-all (`url_data_inspect` accepted, `url_free` accepted, `url_get_auth` accepted, `url_get_hash` accepted, `url_get_host` accepted, `url_get_hostname` accepted, `url_get_path` accepted, `url_get_pathname` accepted, `url_get_port` accepted, `url_get_protocol` accepted, `url_get_query` accepted, `url_get_search` accepted, `url_inspect` accepted, `url_is_protocol` FLAGGED, `url_parse` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 25 | 0 | 0 | 0 | 0 | 0 | 25 | 0.000 | 0.000 |
| regions | 1477 | 0 | 0 | 0 | 0 | 0 | 1477 | 0.000 | 0.000 |

Sanity checks: function pass, region pass. Harnesses unioned: 0. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

<!-- prose -->
