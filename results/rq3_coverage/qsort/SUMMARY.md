# RQ4 — reach of the generated campaign: qsort

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md)
(§2 amendment 2026-09-07); plan: [`../../../docs/rq4_plan_ten_libraries.md`](../../../docs/rq4_plan_ten_libraries.md).
Status 2026-09-07: **six cells complete** — the whole tool row, the only library in the study every
translator produced a buildable artifact for. 3 600 s each, one campaign and one corpus per cell,
`-max_len 4096`, seed 42. C source: `laertes_benchmarks/qsort/qsort.c`, 30 lines, three functions
(`swap`, `partition`, `quickSort`), the file every translation in the study came from.

**Tests side**: qsort ships no test or acceptance target (a CMake example; the drivers in the tree are
ours, not the library's), so every cell is TEST-UNAVAILABLE and the universe is the link-dead-code
denominator (`tests_side_results.json`). Two translators rename the public entry
(`quickSort` → `quick_sort`, SACTOR and PtrTrans); the RQ1 ground-truth map travels with the pair as
`translated/renames.json` and is the first use of the matcher's output inside RQ4.

## Cell table

| tool | planned / built of 3 | corpus | fn ours | reg ours | term. cands | div. cands | confirmed (sample) |
|---|---:|---:|---|---|---:|---:|---|
| **c2rust** | 3 / 3 | 121 | 3 / 3 (**1.000**) | 55 / 55 (**1.000**) | 0 | 0 | **0** (negative control) |
| **Laertes** | 3 / 3 | 118 | 3 / 3 (**1.000**) | 62 / 62 (**1.000**) | 0 | 0 | **0** |
| **C2SaferRust** | 3 / 3 | 118 | 3 / 3 (**1.000**) | 57 / 57 (**1.000**) | 5 006 | 0 | **0 promoted** — 200 sampled, all `out_of_contract_access`, one cluster (§2) |
| **CROWN** | 3 / 3 | 120 | 3 / 3 (**1.000**) | 61 / 61 (**1.000**) | 0 | 0 | **0** |
| **SACTOR** | 3 / 3 | 132 | 6 / 8 (0.750) | 99 / 196 (0.505) | 0 | 0 | **0** |
| **PtrTrans** | 3 / 3 | 131 | 3 / 3 (**1.000**) | 105 / 120 (0.875) | 0 | 57 | **57 `confirmed_divergence`** on `partition` (31) and `quickSort` (26) → **S6 re-found** |

## What this library says

1. **The reach ceiling, and it is real.** Four of six translations are covered **completely** — every
   function and every region of the translation, from three generated harnesses and a corpus of
   ~120 inputs. Nothing about the method caps at tulip's 34 %: that number is tulip's option domain,
   not the campaign's limit. The two cells below 100 % are below it for stated structural reasons,
   not for want of exploration: SACTOR's universe contains two nested helper functions and
   `prog_main`, which no boundary calls (6 of 8 functions, 99 of 196 regions), and PtrTrans reaches 105 of 120
   regions. *Read from the translation, not verified against the export:* its extra regions are the
   arms of the defensive `swap(Option, Option)` that no-op on `None`, which the C function has no
   counterpart for and which no input can reach through a non-null producer. Confirming that from
   the archived llvm-cov exports needs each harness's own `src/lib.rs` for the line alignment, and
   the archive keeps the fuzz target and `build.rs` but not that file, so the check requires
   re-generating the harnesses (deterministic; the generator hash is in the funnel).
2. **S6 re-found, as a value divergence, by the generic pipeline.** PtrTrans's `partition` computes
   the second swap index as `right.get_mut(j - i)` — element `2j − i`, not `j` — and its
   `swap(Option, Option)` silently no-ops when an index is out of range, so nothing ever panics.
   The campaign produced 57 divergence candidates, all 57 adjudicated `confirmed_divergence` with the
   C side clean under ASan and full UBSan. E1 found this with a hand-written batch driver; here it
   falls out of `harness_plan.py` with no boundary-specific input.
3. **C1 is NOT re-promoted, and the reason is a pre-registered rule.** C2SaferRust's `quickSort`
   recurses with `usize` indices, so `i - 1` at `i == 0` wraps and `partition` then reads wildly;
   5 006 of the campaign's inputs trigger it and all 200 sampled land in ONE cluster
   (heap-buffer-overflow, frame `partition`). It is nonetheless recorded as
   `out_of_contract_access`, never promoted: the no-sanitizer replay dies by SIGSEGV **without a
   panic marker**, and the protocol says a wild read that faults is as layout-dependent as the ASan
   report it replaces. The catalogued defect C1 stands on its own E1 evidence; this cell corroborates
   the site and the input class, and is reported as corroboration, not as a second finding.
4. **Every c2rust cell is clean** — the negative control holds for a sixth library.

## Gaps, deviations and limits

- `renames.json` (C name → Rust name) is read by the planner, the funnel and `cell.py`; absent = the
  identity map, so the nineteen cells that ran before it are unaffected. Its content here is the RQ1
  label file, not a guess.
- Two generator gaps were fixed before these cells ran, both C-side facts the planner had been
  reading off the Rust side: a parameter declared with array type (`int arr[]`) is a pointer
  (C11 6.7.6.3p7) and was rejected as an unsupported type; and a boundary that hands its parameters
  to another function of the same unit (`quickSort` → `partition`) had no derived extent at all, so
  its buffer was allocated but never filled and its indices were unbounded. `BodyAnalyzer` now
  carries a callee's facts about its own parameters back onto the arguments the boundary passes
  (depth ≤ 2, no recursion, bounds rewritten to the boundary's parameters). Rule 7 is untouched:
  nothing reads a CALL SITE of the boundary. Golden plans were unchanged by both fixes.
- The generator version for these six cells is **0.7** (`generator_hash` per funnel row). The
  `nullable owned object` bridge family was frozen as 0.8 AFTER this library finished, so no cell
  here mixes versions.
- Single campaign per cell; the confirmation is the 200-per-channel sample, and PtrTrans's 57
  candidates were adjudicated in full (31 + 26 ≤ 200 each).

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md, funnel.json, plans.json, analysis/,
divergences/, confirm_sample/, candidates_sample/, corpus.tar.gz, harness_exports.tar.gz,
artifact_hashes.json, raw/denominator.json). Pairs: `benchmark/pairs/rq4/qsort_{c2rust,laertes,
c2saferrust,crown,sactor,ptrtrans}/` (+ `translated/renames.json` for SACTOR and PtrTrans, and
`PROVENANCE.json` in each). Manifest: **S6 re-found**; C1 corroborated, not re-promoted.
