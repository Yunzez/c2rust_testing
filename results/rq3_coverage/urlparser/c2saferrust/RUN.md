# urlparser × c2saferrust — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 21 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 19 |
| built | 18 |
| executed (corpus > 0) | 18 |
| coverage exported | 9 |

Plan failures, by the generator's own reason:

- **1** × Rust signature has 1 parameters, C has 2: reshaped API, no positional bridge
- **1** × input buffer of i8 has Rust type &str, which is not a raw pointer, a slice, a Vec or a Box

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `get_part` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `strdup` | no | 14 | 0 | normal 14 | batch |
| `strff` | yes | 15 | 0 | normal 15 | batch |
| `url_data_inspect` | no | 2 | 3 | normal 2 | batch |
| `url_free` | no | 2 | 5 | normal 2 | batch |
| `url_get_auth` | no | 1 | 2 | normal 1 | batch |
| `url_get_hash` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_host` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_hostname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_path` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_pathname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_port` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_protocol` | no | 1 | 1 | normal 1 | batch |
| `url_get_query` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_search` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_is_protocol` | no | 1 | 0 | normal 1 | batch |
| `url_is_ssh` | no | 1 | 0 | normal 1 | batch |
| `url_parse` | no | 1 | 1 | normal 1 | batch |

## 3. Tests side

Status **TEST-FAILS**. exit 134: `free(): double free detected in tcache 2` -- the C2SaferRust translation double-frees inside the suite; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 18 harnesses, 9 crash-all (`get_part` accepted, `url_get_hash` accepted, `url_get_host` accepted, `url_get_hostname` accepted, `url_get_path` accepted, `url_get_pathname` accepted, `url_get_port` accepted, `url_get_query` accepted, `url_get_search` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 24 | 0 | 7 | 0 | 0 | 7 | 17 | 0.000 | 0.292 |
| regions | 1183 | 0 | 107 | 0 | 0 | 107 | 1076 | 0.000 | 0.090 |

Sanity checks: function pass, region pass. Harnesses unioned: 9. Identities outside the universe (excluded, never added): 0 fn / 1 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `strdup` | 14 | 14 | 14 | 14 | 14 |
| `strff` | 15 | 15 | 15 | 15 | 15 |
| `url_data_inspect` | 2 | 2 | 2 | 2 | 2 |
| `url_free` | 2 | 2 | 2 | 2 | 2 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 38 |
| signal | 9 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `get_part` | 3 / 3 | out_of_contract_access 1, ub_associated 2 | 2 |
| `url_data_inspect` | 3 / 3 | not_reproducible 3 | 1 |
| `url_free` | 5 / 5 | not_reproducible 5 | 1 |
| `url_get_auth` | 2 / 2 | not_reproducible 2 | 1 |
| `url_get_hash` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_host` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_hostname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_path` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_pathname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_port` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_protocol` | 1 / 1 | not_reproducible 1 | 1 |
| `url_get_query` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_search` | 3 / 3 | ub_associated 3 | 1 |
| `url_parse` | 1 / 1 | not_reproducible 1 | 1 |

Total: not_reproducible 12, out_of_contract_access 1, ub_associated 26

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** Confirmation recovered on 2026-09-09 from the archived inputs with rebuilt binaries
(the original post skipped it: `--dest` appended to the confirm line by a regex edit). §4–§6
regenerated from the archive the same day.

**What the cell says.** 19 of 21 boundaries planned: two have no lossless bridge in this
translation (one parameter fewer than C on one boundary; `url` lifted to `&str`, which cannot carry
an arbitrary C byte string, on the other). 18 built, 9 exported — the same `get_part` crash-all
family as every urlparser cell (C heap overflow on the reference side, preflight-accepted). Sample:
26 `ub_associated`, 12 `not_reproducible`, 1 `out_of_contract_access` (a wild read without a panic
marker; never promoted). **Nothing confirmed.** The transpiled test program fails on this
translation (double free), so the cell is denominator-only.

**Not established.** Whether the two unbridged boundaries (`&str` lift) narrow the input domain —
a candidate for the byte-string family, but no harness reached them, so it is not a finding.
