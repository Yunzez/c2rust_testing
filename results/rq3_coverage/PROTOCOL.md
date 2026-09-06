# RQ4 protocol — pre-registered, 2026-09-04

> **RQ4.** *How much code does our differential validator exercise beyond the test suites shipped
> with existing translators and translated libraries?* (`results/EVALUATION_PLAN.md`)

This file is frozen **before** any cell in the current round runs. `README.md` in this directory
requires exactly that: *"'Shipped tests' is undefined for tools that ship no suite. The definition
must be written down per system before any cell runs, not chosen per cell afterwards"*, and the
same for the budget, seeds and stopping rule. Operational lessons that are not protocol — quotas,
serial cells, detached launches, the noise classes, generator traps — live in
[`docs/rq4_runbook.md`](../../docs/rq4_runbook.md); read it before running another library.

It supersedes `bzip2/PROTOCOL.md`, which was written for one library.

The directory keeps its historical `rq3_coverage` name so existing evidence links do not break.

---

## 1. Unit and the reported partition

The unit is one **library × translator artifact**. Within one artifact, over one region universe,
the code is partitioned into four **mutually exclusive** sets:

```
Tests only  |  Both  |  Ours only  |  Neither
```

reported for **functions** and for **regions** separately, with `Tests = Both + Tests-only`,
`Ours = Both + Ours-only`, and `Union = Both + Tests-only + Ours-only`.

Coverage measures **exploration, not correctness**. Nothing in this partition is a defect claim.

## 2. The baseline: what evidence exists that a translation is correct

**Measured fact, first, because it decides how the baseline must be named.** Across all 55
translated crates in the tree (`laertes_benchmarks/*`, `crown/results/*`), covering all six
translators:

| | count |
|---|---:|
| crates containing `#[test]` or `#[cfg(test)]` | **0** |
| `Cargo.toml` files declaring a `[[test]]` target | **0** |

**No translator ships any test with its translation.** What these tools use as their correctness
check — in their papers and in practice — is the **library's own pre-existing suite**, re-run
through the translation. They reuse it; they do not author it.

So the RQ4 baseline is named for what it is:

> **The acceptance evidence that exists for this translation** = the library's own shipped test
> suite, run through the translated artifact.

Two things follow, and both must appear wherever the number does:

* **The suite is the library authors' work, not the translator's.** It is comprehensive by
  construction, so a high tests-side number is expected and is *not* a claim about the translator.
  We are not claiming to beat a tool's tests; we are asking what code the existing acceptance
  evidence does not reach.
* **Comparing against nothing would be a strawman.** Reporting "the translator ships 0 tests" as
  the baseline would make the comparison trivial and a reviewer would immediately name the library
  suite. The `0 of 55` fact is reported separately as **motivation** — the field ships translations
  with no executable correctness evidence of their own — never as the baseline.

### When the suite counts as a baseline

> **The tests side is a baseline only if the suite passes completely.** A suite that runs but does
> not pass is recorded with its pass rate and used as a **denominator only**: the universe stays
> valid (that build carries `-C link-dead-code`), the Tests column is empty, and the partition
> collapses to Ours / Neither.

This is not caution, it is bias control. A broken translation fails more of the suite, which lowers
the baseline, which *inflates* our only-ours number. Without this rule the reported "coverage beyond
the tests" would correlate with how bad the translation is — the opposite of what RQ4 asks. A
failing suite is an **E1 defect finding**, not an RQ4 baseline.

| library | the suite | driver present in the translation | status |
|---|---|---|---|
| **bzip2** | `Makefile` `test:` — compress `sample{1,2,3}.ref` at `-1/-2/-3`, decompress `sample{1,2,3}.bz2`, six `cmp`s | `bzip2.rs` in every crate (`autobins = false`) | c2rust **6/6 → baseline**; Laertes 3/6, C2SaferRust 0/6, CROWN adapter fails → **denominator only** |
| **genann** | `test.c` (minctest) plus `example1..4.c` | `test.rs`, `example1..4.rs` in every crate | to measure |
| **tulip** | `smoke.c` driven by `tests/{atoz,extra,untest}.txt` | `smoke.rs` in all four crates | to measure |
| **cJSON** | `tests/main.c` + five JSON fixtures | **no** — only `src/cJSON.c` was transpiled | denominator only |
| **lil** | none — ~25 `.lil` demo scripts are not a test target | `main.rs` is transpiled | denominator only |

**Representation-only adapter.** Where the driver is transpiled but not declared runnable, the
adapter is a copy of the crate with a `Cargo.toml` declaring the existing transpiled driver as a
`[[bin]]`. The suite is unchanged. An adapter that will not build makes the cell
`TEST-ADAPTER-FAILS` — denominator only, never 0 %.

We do **not** transpile a test driver ourselves and we do **not** substitute a proxy suite: either
would be our construction rather than evidence that already existed.

## 3. Budget, seeds, stopping rule

| | |
|---|---|
| campaign | **Rust-only** (`C2R_MODE=rust-only`), libFuzzer fork mode, all harnesses of a cell concurrent |
| budget | **3 600 s wall per cell** (amended 2026-09-04, before any cell under this protocol produced a number; see below) |
| flags | `-seed=42 -timeout=25 -rss_limit_mb=8192 -fork=1 -ignore_crashes=1 -ignore_timeouts=1 -ignore_ooms=1` |
| stopping rule | fixed wall clock, no early stop, no extension for a cell that looks interesting |
| seeding | the library's own **shipped sample inputs**, encoded into the harness input format, plus one fixed 64-byte seed per corpus |
| saturation check | corpus snapshots at 1 / 5 / 10 / 30 / 60 s recorded in `result.json`; **not** plotted |

**Seeding is part of the measurement, not a convenience, and it has two separate jobs.**

*The fixed 64-byte seed* keeps a corpus non-empty. Without it a boundary whose inputs are all
equivalent (a zero-argument deterministic entry) or all fatal ends the campaign with an **empty
corpus** and its coverage is lost entirely — five of nineteen bzip2 boundaries on the first run
under this protocol.

*The shipped sample inputs* are what let a format-consuming boundary get past its header. Measured
on bzip2 × c2rust, same generator, 300 s, seeds the only difference:

| harness | with shipped seeds | fixed seed only |
|---|---|---|
| `BZ2_bzBuffToBuffDecompress` | 438 inputs / 2 528 regions | **39 inputs / 653 regions** |
| `BZ2_bzBuffToBuffCompress` | 647 inputs / 4 510 regions | **349 inputs / 1 961 regions** |

Random bytes never form a valid bzip2 stream, so the decompressor bails at the header and the
campaign explores nothing. These are the **library's own shipped samples**
(`sample{1,2,3}.{ref,bz2}`), re-encoded into the byte-cursor format the harness reads — not test
data we invented.

**The saturation evidence is conditional on seeding.** "0 new functions and 48 of 8 789 regions
between minute 1 and minute 60" was measured *with* seeds. It licenses a short budget for a seeded
campaign; it says nothing about an unseeded one, and reading it as if it did is what produced the
first run under this protocol.

**Amendment, 2026-09-04 — 300 s → 3 600 s.** The 300 s figure was licensed by a saturation
measurement taken on **one cell of one tool** (bzip2 × c2rust). Generalising it to every cell of
every translator is exactly the assumption this protocol says not to make: a reshaped translation
may saturate later, and a cell that is still growing at its last checkpoint cannot be told apart
from one that finished. The budget is therefore raised to a full hour, which costs machine time
and nothing else.

The 300 s number is **not** discarded. Snapshots are hard-linked corpus copies, so the 60 / 300 /
600 / 1 800 s corpora survive the run and coverage is recomputable from any of them. Every cell
consequently reports the hour, and can report the five minutes from the same campaign, with no
second run and no choice made after seeing the outcome.

This is an amendment made **before any cell under this protocol produced a number** — the four
bzip2 cells' first attempt died in the build phase (`EDQUOT`, four concurrent cargo target dirs)
and produced no data. It is recorded rather than silently applied, and it moves the budget only
upward: no cell is ever extended because it looked interesting, which §3's stopping rule still
forbids.

The budget was originally short **because saturation was measured, not assumed**: on bzip2 × c2rust the
campaign gained **0 functions and 48 of 8 789 regions between minute 1 and minute 60** while the
corpus doubled. The binding constraint is boundary coverage and seed quality, not campaign length.
The snapshots exist so that claim is re-checked in every cell rather than inherited.

**Procedural amendment 2026-09-06 — preflight before every campaign.** After the harnesses are
built and before the campaign starts, each harness is run once on the empty input in c-only and
in rust-only mode, and all harnesses run a 60 s fork-mode test with the campaign's own libFuzzer
parameters. A harness whose C side crashes on the empty input, or whose test run crashes on
~every job with a corpus that never grows, is flagged; unless it is listed in the pair's
`preflight_accept.txt` (a reviewed, reasoned "unconstructible precondition"), the cell stops
before the campaign (exit 3) and is re-run after review. The preflight's corpus is discarded and
the budget, seed and stopping rule above are untouched; this changes what is measured only in
that a harness bug is caught in one minute instead of after an hour (lil × c2rust/Laertes,
`lil_parse`: a dangling length-0 buffer crashed every execution; those two cells re-ran that one
boundary and record the deviation). Results: `preflight/preflight.json` per cell, the funnel row,
RUN.md.

## 4. One campaign, one corpus

The coverage measurement and the divergence search are **not** two separately budgeted experiments:

```
Rust-only campaign (§3)
  └─ save the corpus
       ├─ coverage from that corpus            → the four-set partition
       └─ combined replay of the SAME corpus   → divergence candidates
            └─ confirmation of each candidate  → verdicts
```

Candidates are therefore candidates *on the coverage corpus*, and both numbers come from one
budget.

**Two candidate channels, one confirmation** (recorded 2026-09-05 02:10, before any full
adjudication ran). The Rust-only campaign leaves *termination* artifacts (an input on which the
translation trapped; libFuzzer keeps it out of the corpus). The combined replay of the saved corpus
produces *divergence* artifacts (both sides returned and disagreed on a ladder rung, or one side
trapped where the other did not). Both go through the same confirmation; `ub-gated` replay
outcomes — the in-loop noise filter fired on the C side — are counted and listed but are not
candidates, because confirmation re-checks C-definedness under ASan + full UBSan anyway.

**Adjudication depth.** Every boundary is first adjudicated on a fixed **sample of 200 artifacts
per channel** (terminations and divergences separately, each sorted by artifact name — libFuzzer
names are content hashes, so the draw within a channel is unbiased; sampling the two channels
together would let thousands of `crash-*` names crowd out every `divergence-*`, which is what
happened on CROWN `fallbackSort` before this sentence was added at 03:25) and the sample is
labelled as such wherever it is quoted. A boundary is adjudicated **in full** when (a) it
is a public-API boundary whose input model is sound — the same model produced zero artifacts on the
faithful c2rust translation — or (b) any `confirmed_*` verdict appears in its sample. A boundary
whose sample is entirely `ub_associated*` / `instrument_only` / timeout-`inconclusive` is reported
at the sample together with its total artifact count. Reason, stated before the fact: the internal
sort routines (`mainQSort3`, `hbCreateDecodeTables`, …) yield 10⁴–10⁵ artifacts per tool because
their input model puts them out of contract by construction, identically on all four tools;
re-deriving "out of contract" 10⁵ times at four replays each would cost ~40 machine-hours and
establish nothing the sample does not. Every defect found so far lives on the public-API
boundaries, which have 10²–10³ artifacts and are adjudicated completely.

A differential discovery run — fuzzing guided by the comparison rather than by Rust
coverage — is a **separate experiment** needing its own pre-registered budget, and its corpus is
**never** merged into the coverage corpus.

## 5. Instrumentation

| | |
|---|---|
| toolchain | `nightly-2025-09-01` = rustc 1.91.0-nightly (07d246fc6), **LLVM 21.1.0**, pinned by `rust-toolchain` in both the harness crate and its `fuzz/` crate |
| profdata / cov | `llvm-profdata` and `llvm-cov` from **that toolchain only** (`~/.rustup/toolchains/nightly-2025-09-01-…/lib/rustlib/x86_64-unknown-linux-gnu/bin`). The system LLVM 21.1.8 must never be used. |
| tests side | `-C instrument-coverage -C codegen-units=1 -C link-dead-code -C debug-assertions --cfg fuzzing` |
| ours side | whatever cargo-fuzz forces (sancov, `-Zsanitizer=address`, `-Cdebug-assertions`, `-Ccodegen-units=1`) plus `-C instrument-coverage` |

The last two tests-side flags are deliberate: cargo-fuzz forces them on the ours side and cannot be
told otherwise, so mirroring them is what makes both sides compile the same MIR.

## 6. Identity and universe

* **Function identity** = `(source file, start line)`. **Region identity** =
  `(source file, l1, c1, l2, c2)`. Both in the artifact's original per-file coordinates, remapped
  through the flattening line map.
* **Never the symbol name.** The two sides are separate cargo invocations, so Rust v0 mangling
  embeds a different crate-disambiguator hash and the same function has different symbols.
* **Universe** = the tests-side in-scope identity set, because that build carries
  `-C link-dead-code` and is complete. Where the tests side is unavailable, a `denom` binary built
  with `-C link-dead-code` that merely references the library supplies the denominator.
* **Scope** is declared per cell in `scope.json`: the translated library modules, with CLI and
  driver modules excluded by full path.
* Ours-side identities outside the universe are **counted and reported, never added**.

Three corrections are part of the analysis and are not optional: full-path `lib.rs` filtering (so
libfuzzer-sys's own instrumented `src/lib.rs` is not remapped into the library), difflib alignment
for the line shift `--expose-entry` introduces, and `(file, line)` identity in place of symbols.

**Sanity checks, asserted in every `result.json`:** `both + only_tests = covered_tests`;
`both + only_ours = covered_ours`; the four sets sum to the universe; every universe identity is in
a scored module; `covered ≤ denominator`. A cell that fails one is a bug, not a result.

## 7. Cell status vocabulary

`complete` (suite passes, two-sided) · `TESTS-PARTIAL (n/m pass)` · `TESTS-FAIL (0/m pass)` ·
`TEST-UNAVAILABLE (reason)` · `TEST-ADAPTER-FAILS` · `CRASH-ALL` ·
`non-building ✗(stage)` using E1's stage taxonomy (`parse` / `circular` / `scaffold` / `translate` /
`compile` / `verify` / `PA` / `analyse` / `rewrite`) · `no artifact`.

**No outcome is ever converted to 0 %.** The middle four all mean the same thing for the reported
partition — denominator only, Tests column empty — but they are distinguished because *why* the
evidence is missing is itself a result.

## 8. Defect promotion

Coverage and defects are reported separately and are never mixed into one figure. A candidate is
promoted only on the confirmation verdict, and the rule differs by kind:

| verdict | promoted? | condition |
|---|---|---|
| `confirmed_divergence` | **yes** | a reproducible value or state difference with no UB check firing on the C-only replay. **No panic marker is required** — a pure return-value defect is exactly this shape. |
| `confirmed_termination` | **yes** | the translation still traps with **no sanitizer** and with the **panic marker** present. A SIGSEGV alone does not qualify: whether a wild read faults is as layout-dependent as the ASan report it replaces. |
| `ub_associated`, `ub_associated_value`, `ub_associated_termination` | no | a UB check fired on C alone |
| `out_of_contract_access` | no | both sides make the same out-of-contract access; which one dies is heap-layout luck |
| `instrument_only` | no | the failure is the instrument's. This says **this candidate** was an instrumentation artifact; it is *not* evidence that the translation is correct. |
| `ub_gated`, `not_reproducible`, `inconclusive` | no | |

A sanitizer raises **check coverage**; it never establishes that an execution is free of undefined
behaviour. Every `confirmed_*` verdict means *no check fired*, not *C is defined*.

Promoted defects go into `DEFECTS` in `results/rq4_effectiveness/gen_defect_manifest.py` with the
evidence path pointing at the campaign directory.

## 9. Aggregation

* Normalise **inside the artifact** first. Translations differ in region universe by more than they
  differ in coverage; pooling raw counts lets the most verbose translation set the result.
* **Never** pool raw region or function counts across translations.
* Equal weight per library for any overall figure.
* A per-library mean over tools, if printed at all, must name the cells it covers — with N/A cells
  the denominator changes from row to row, which is the trap E3 already fell into.

## 10. Provenance obligations per cell

* `artifact_hashes.json`: sha256 of every C source file and every translated `.rs`, plus the
  toolchain block.
* **C-source version skew must be recorded.** genann has two C copies in the tree (2018, 15
  functions vs 2015, 12 functions) and lil has two (2 962 vs 3 518 lines). Record which one the
  translator actually consumed.
* Where a C hash cannot be bound to the translator's input, `matches_translator_input` is
  `"unknown"` and the cell says so — bzip2 is in this state today.

## 11. Per-cell artifacts

```
results/rq3_coverage/<library>/<tool>/
  RUN.md                    procedure, deviations, corrections, and what is NOT established
  scope.json                the universe definition
  artifact_hashes.json
  result.json               four sets, snapshots, sanity checks
  covered_by_both.txt  only_tests.txt  only_ours.txt  covered_by_neither.txt
  funnel.json               matched -> planned -> built -> executed -> clean/degraded/no-useful
  campaign/                 candidates -> verdicts.json -> clusters.json
  raw/                      corpora and exports as tarballs, logs, linemap
```

Per library, `SUMMARY.md` with the cell table and the gaps stated in the open.
