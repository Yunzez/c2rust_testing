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

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `get_part` | 3 / 3 | ub_associated 3 | 1 |
| `strrwd` | 3 / 3 | ub_associated 3 | 1 |
| `url_data_inspect` | 5 / 5 | not_reproducible 5 | 1 |
| `url_free` | 3 / 3 | not_reproducible 3 | 1 |
| `url_get_auth` | 2 / 2 | not_reproducible 2 | 1 |
| `url_get_hash` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_host` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_hostname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_path` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_pathname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_port` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_protocol` | 2 / 2 | not_reproducible 2 | 1 |
| `url_get_query` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_search` | 3 / 3 | ub_associated 3 | 1 |
| `url_inspect` | 3 / 3 | ub_associated_termination 3 | 1 |
| `url_parse` | 1 / 1 | not_reproducible 1 | 1 |

Total: not_reproducible 13, ub_associated 30, ub_associated_termination 3

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** (1) The cell's confirmation was NOT run by its post: a regex edit of the chain
script had appended `--dest` to the `confirm_cell.py` line, which made the step exit silently. The
candidate and divergence inputs were archived intact; the harness binaries were rebuilt from the
archived plans (`rebuild_bins.py`, deterministic, no fuzzing) and the 200-per-boundary sample was
adjudicated from those inputs on 2026-09-09 (§6 above is that recovery). (2) §4–§6 of this file were
regenerated from the archive on 2026-09-09; §1–§3a are the original post's.

**What the cell says.** 21 boundaries matched and planned, 20 built (`main` is excluded by the
pair). Only 9 harnesses exported coverage: the other 11 are the boundaries that reach `get_part`,
whose `malloc(1)` + `sscanf` (url.h:208, E1 #27) overflows on the C side for every well-formed URL,
so C provides no reference and libFuzzer restarts on every input. That is the reason for 0.318 of
functions and 0.092 of regions, and it is a property of the library, pre-registered in
`preflight_accept.txt`. Every sampled candidate adjudicated `ub_associated` (30), `not_reproducible`
(13) or `ub_associated_termination` (3): **nothing confirmed** — the negative control holds. The
shipped test program passes on this translation (21/22 functions); the paper does not compare the
two sides (RQ4 reframing), the numbers stay here as the acceptance record.

**Not established.** Coverage of the `get_part` family under a valid C reference (would need a
patched C, which is out of scope: we do not patch the reference).
