# urlparser × laertes — RQ4 cell (plan pipeline)

Numbers below are assembled by `scripts/rq4/run_md.py` from the cell's own files; the prose after the prose marker (section 7) is written by hand. Protocol: `results/rq3_coverage/PROTOCOL.md`.

## 1. Funnel

| stage | n |
|---|---:|
| matched boundaries (frozen RQ1 output) | 21 |
| planned (complete HarnessPlan, lossless bridge for every parameter) | 21 |
| built | 20 |
| executed (corpus > 0) | 20 |
| coverage exported | 3 |

## 2. Per boundary

| boundary | C static | corpus | term. candidates | div. replay | coverage mode |
|---|---|---:|---:|---|---|
| `get_part` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `strdup` | no | 15 | 0 | normal 15 | batch |
| `strff` | yes | 15 | 0 | normal 15 | batch |
| `strrwd` | yes | 1 | 2 | signal 1 | failed rc=1 |
| `url_data_inspect` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_free` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_auth` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_hash` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_host` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_hostname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_path` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_pathname` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_port` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_protocol` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_query` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_get_search` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_inspect` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_is_protocol` | no | 1 | 2 | signal 1 | failed rc=1 |
| `url_is_ssh` | no | 1 | 0 | normal 1 | batch |
| `url_parse` | no | 1 | 2 | signal 1 | failed rc=1 |

## 3. Tests side

Status **TEST-FAILS**. exit 139 (SIGSEGV) with no output: the Laertes translation crashes inside the suite's first url_parse; denominator only

Mode used for the partition: **TEST-UNAVAILABLE (denominator only)**.

## 3a. Campaign parameters, preflight, generator

Preflight (60 s test run + empty-input probe before the campaign): 20 harnesses, 15 crash-all (`url_data_inspect` accepted, `url_free` accepted, `url_get_auth` accepted, `url_get_hash` accepted, `url_get_host` accepted, `url_get_hostname` accepted, `url_get_path` accepted, `url_get_pathname` accepted, `url_get_port` accepted, `url_get_protocol` accepted, `url_get_query` accepted, `url_get_search` accepted, `url_inspect` accepted, `url_is_protocol` accepted, `url_parse` accepted).

Generator sources (sha256[:16] over gen_diff_harness.py, harness_plan.py, c2r_funnel.py, flatten_translation.py) that built the harnesses: `3d43f75aea807da1 (bin reused from the killed run)`

libFuzzer parameters: `mode=rust-only`, `fork=1`, `max_total_time_s=3600`, `seed=42`, `timeout_s=25`, `rss_limit_mb=2048`, `max_len=4096`, `ignore=['crashes', 'timeouts', 'ooms']`, `snapshots_s=[60, 300, 600, 1800]`

## 4. Coverage (four-set partition, identities = (file, line) / (file, l1,c1,l2,c2))

| | universe | tests | ours | both | only-tests | only-ours | neither | tests cov | ours cov |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| functions | 25 | 0 | 3 | 0 | 0 | 3 | 22 | 0.000 | 0.120 |
| regions | 1477 | 0 | 41 | 0 | 0 | 41 | 1436 | 0.000 | 0.028 |

Sanity checks: function pass, region pass. Harnesses unioned: 3. Identities outside the universe (excluded, never added): 0 fn / 0 reg.

### Corpus growth at the checkpoints (inputs)

| boundary | 60 s | 300 s | 600 s | 1800 s | 3600 s |
|---|---:|---:|---:|---:|---:|
| `strdup` | 15 | 15 | 15 | 15 | 15 |
| `strff` | 15 | 15 | 15 | 15 | 15 |

## 5. Combined replay of the coverage corpus (protocol §4 step 6)

Outcome tally over every saved corpus input, C reference beside the translation, ladder on:

| outcome | inputs |
|---|---:|
| normal | 31 |
| signal | 17 |

## 6. Confirmation (confirm_sample, first 200 artifacts per boundary — a labelled SAMPLE, not the cell's adjudication)

| boundary | adjudicated / total | verdicts | clusters |
|---|---:|---|---:|
| `get_part` | 3 / 3 | ub_associated 3 | 1 |
| `strrwd` | 3 / 3 | ub_associated 3 | 1 |
| `url_data_inspect` | 3 / 3 | confirmed_termination 3 | 1 |
| `url_free` | 3 / 3 | confirmed_termination 3 | 1 |
| `url_get_auth` | 3 / 3 | confirmed_termination 3 | 1 |
| `url_get_hash` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_host` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_hostname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_path` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_pathname` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_port` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_protocol` | 3 / 3 | confirmed_termination 3 | 1 |
| `url_get_query` | 3 / 3 | ub_associated 3 | 1 |
| `url_get_search` | 3 / 3 | ub_associated 3 | 1 |
| `url_inspect` | 3 / 3 | ub_associated 3 | 1 |
| `url_is_protocol` | 3 / 3 | confirmed_termination 3 | 1 |
| `url_parse` | 3 / 3 | confirmed_termination 3 | 1 |

Total: confirmed_termination 18, ub_associated 33

<!-- prose -->
## 7. Prose (2026-09-09)

**Deviations.** This is the rerun of 2026-09-09 (the first run's confirmation had been skipped by
the same chain-script edit as the other three cells; the rerun re-used the built binaries and ran
the campaign, replay and sample again). §4–§6 are from the rerun's archive.

**What the cell says.** 21 planned, 20 built, only 3 exported: besides the `get_part` family, every
boundary that goes through `url_is_protocol` crashes on the **Rust** side with a clean C —
`URL_SCHEMES[177]` is emitted as 177 NULLs and its initializer sits in `laertes_init_URL_SCHEMES()`,
which nothing calls (0 call sites; the severed-init law of this translator). `url_is_protocol`
3/3 `confirmed_termination` (c_only normal, rust_no_sanitizer SIGSEGV on the zero page), and
`url_parse`, `url_get_protocol`, `url_get_auth`, `url_data_inspect`, `url_free` 3/3 each through it —
**one root cause, manifest C12**, family *initialization loss or corruption*. The remaining 33 are
`ub_associated` (the C-side `get_part` overflow). The E1 severed-init scanner had filed url.h as a
fixture (`poisoned_lib: 0`): a false negative for a header-only library, which is why C12 is new.

**Not established.** Region reach (0.028) says nothing about the translation beyond the two
crash-all families; no boundary with a clean reference on both sides reached deeper code.
