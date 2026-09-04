# RQ4 — coverage beyond shipped tests: bzip2

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.*
Protocol: [`PROTOCOL.md`](PROTOCOL.md). Inventory: [`INVENTORY.md`](INVENTORY.md).

Status 2026-09-03: **one cell run** (c2rust), twice — round 1 had a broken input model and its
numbers are superseded (`c2rust/RUN.md` §2 and §8). By instruction, work stops after this corrected
pilot. No library-level mean is computed, and nothing here is ready for the paper table.

## Result table

| Tool | Artifact status | Test status | Matched | Eligible | Built | Executed | Corpus inputs | Fuzz s (alloc/actual) | Test fn cov | Our fn cov | Δ | Test reg cov | Our reg cov | Δ |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| **c2rust** | complete, builds | shipped suite, representation-only adapter, **6/6 pass** | 64 | 14 | 11 | **10** | 1563 | 3600 wall (concurrent) | **0.773** | **0.682** | **−0.091** | **0.797** | **0.798** | **+0.001** |
| Laertes | complete, builds | same | — | — | — | — | — | — | — | — | — | — | — | — |
| C2SaferRust | complete; reshaped API SIGSEGVs on 100 % of E3 inputs | same | — | — | — | — | — | — | — | — | — | — | — | — |
| CROWN | complete; macOS-transpiled, link needs shims | same | — | — | — | — | — | — | — | — | — | — | — | — |
| SACTOR | **NON-BUILDING** — `✗(parse)`, no artifact produced | n/a | — | — | — | — | — | — | — | — | — | — | — | — |
| PtrTrans | **NON-BUILDING** — `✗(compile)`, 73 errors, 55/78 stub-reverts | n/a | — | — | — | — | — | — | — | — | — | — | — | — |

`—` = not attempted (stopped after the corrected c2rust pilot). **No outcome is converted to 0 %.**

Function-level sets for c2rust: total in scope 66, both 43, only-tests 8, only-ours 2, covered by
neither 13. Regions: total 8789, both 6609, only-tests 398, only-ours 409. 7 inputs were excluded
by the C-side UB gate before Rust ran.

### Coverage curve (artifact-level union over all ten harnesses)

| minute | corpus inputs | functions | fn coverage | regions | region coverage |
|---:|---:|---:|---:|---:|---:|
| 1 | 755 | 45 | 0.682 | 6970 | 0.793 |
| 5 | 1263 | 45 | 0.682 | 6982 | 0.794 |
| 10 | 1426 | 45 | 0.682 | 6998 | 0.796 |
| 30 | 1495 | 45 | 0.682 | 7005 | 0.797 |
| 60 | 1563 | 45 | 0.682 | 7018 | 0.798 |

**It saturates inside the first minute.** Fifty-nine further minutes add 0 functions and 48 of 8789
regions while the corpus doubles. The binding constraint is boundary eligibility and seed quality,
not campaign length. This is the number that should decide the budget frozen for the other nine
libraries: a longer campaign buys nothing here.

## bzip2 mean over paired complete outputs

**Not computed.** One artifact has been measured; a mean over a single cell is not a library
result. It will be computed only over cells that reach PAIRED, and the tools excluded from it will
be named with their reason.

## Multi-harness merging: was it valid?

Yes, and it is the reason the c2rust number is trustworthy at all. Coverage is unioned at the level
of function and region **identity** across the 10 harnesses, at each of the five checkpoints —
never summed, never averaged. Because
`llvm-cov` cannot deduplicate across separate binaries, each harness was exported separately and the
union was computed over identities defined as
`(source file, start line)` for functions and `(source file, start line, start col, end line, end col)`
for regions, in the artifact's original per-file coordinates. Three corrections were required before
the union was sound, each of which had silently changed the numbers:

1. symbol names differ between the two sides (different crate-disambiguator hashes) → identity is
   `(file, line)`;
2. matching `lib.rs` by basename also matched libfuzzer-sys's own instrumented `src/lib.rs`;
3. `--expose-entry` inserts a line into a harness's `lib.rs`, shifting all later lines by one.

After these, **0 functions and 0 regions** in the ours union fall outside the tests-side universe.
Sanity checks `both + only_tests = covered_tests`, `both + only_ours = covered_ours`,
`both + only_tests + only_ours = union`, scope membership and `covered ≤ denominator` all pass.

## Methodological blockers

1. **The eligibility rule is wrong in both directions, and it dominates the result.**
   It *under*-accepts by never inspecting the return type — `BZ2_bzopen`/`bzdopen`/`bzopen_or_bzdopen`
   pass the rule, then fail to build because the comparator cannot compare `BZFILE*` values.
   It *over*-accepts pointers with no length parameter: with a correct schema those boundaries are
   memory-safe, but their arrays cannot be filled from the fuzz input at all, so
   `BZ2_indexIntoF`, `fallbackQSort3` and `fallbackSimpleSort` are driven with degenerate data.
   The in-loop UB gate does not compensate: it is UBSan-minimal on the C side and does not check
   raw-pointer heap accesses.
2. **The automatic input model was wrong on 6 of 7 boundaries in round 1** and had to be replaced
   by hand-authored schemas plus five generator fixes (`c2rust/RUN.md` §2–§3). Nine libraries
   remain; each will need the same schema work, and the cost of that must be stated in the paper
   rather than implied to be automatic.
3. **Coverage saturates in under a minute**, so no budget statement of the form "we fuzzed for N
   hours" carries information for this artifact. Report the saturation point instead.
4. **Removing the C side would buy nothing.** Replaying the same corpora with the UB gate off, and
   again with C never called, gives *identical* Rust coverage: 45 functions / 7018 regions in all
   three modes. The gate withholds 0 functions and 0 regions. Both self-checks pass
   (`gated` reproduces the headline; `nogate` and `rust-only` are identical set-for-set).
   `c2rust/RUN.md` §10.
5. **The whole gap is one missing capability.** Of the 21 in-scope functions the validator misses,
   17 are the `FILE*`/`BZFILE*` API, 2 are reachable only through it (`isdigit`, `__isctype`, from
   mode-string parsing), and 2 are degenerate (`BZ2_bzlibVersion` has no parameters,
   `BZ2_bz__AssertH__fail` calls `exit(3)`). The validator covers every one of the 45 functions
   that is not behind a file handle. An environment adapter for process-owned file state — stage
   3's deferred case, and the example the workflow figure already uses — would close essentially
   all of it.
6. **The comparison is between different API layers.** All 8 only-tests functions are the
   `FILE*`/`BZFILE*` stream API, which the validator is structurally unable to drive; reaching them
   needs environment adapters (stage 3's deferred case). At region level the two sides are at
   parity while reaching substantially different code (409 only-ours vs 398 only-tests). Any paper
   sentence using −0.091 must say what the gap is made of.
7. **C-source provenance is still unbound.** Phase 0 records that no bzip2 harness ever bound a C
   hash to the translator's input; only the version string matches. Hashes are archived in
   `c2rust/artifact_hashes.json`, but the binding itself remains unverified.
8. **Generator/validator defects were found and fixed in a scratchpad copy** (repo copies
   untouched, both diffs archived). Two of them — C globals not renamed in the oracle, and a
   `bounded_scalar` that used `%` instead of `rem_euclid` — would silently corrupt any differential
   campaign on any library. Whether to upstream them into `tools/stu_selector/` is a separate
   decision and should be made before the other nine libraries are run.

## Directory

- `c2rust/` — the cell: `RUN.md`, `scope.json`, `artifact_hashes.json`, `result.json` (curve +
  final + sanity), the four identity lists, both sides' coverage exports and profiles, all 10
  campaign corpora, the seed corpora, the 10 hand-authored schemas, the per-pair eligibility
  verdicts, logs, and every script used plus both generator diffs.
- `c2rust_diagnostic_pilot/` — **rejected**, not an RQ4 result; kept for the one diagnostic
  observation it supports. Its banner says why.
- `c2rust/round1_superseded/` — round-1 corpora, exports and logs, kept as provenance for §8 of
  `c2rust/RUN.md`. Its numbers describe a broken input model and must not be cited.
