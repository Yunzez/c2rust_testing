# RQ4 — the denominator, and what is NOT computed (user decision, 2026-09-08)

*Recorded when RQ4 was reframed away from a tests-vs-ours comparison. This page is the operational
record: what the denominator is, what happens to the three cells that do not yet have an independent
one, and the two things we deliberately do not compute. The paper edits themselves are the user's.*

## The definition

> The denominator is **all LLVM source-based functions and regions in the declared translated-code
> scope, extracted from the instrumented library objects independently of any test, harness, or
> executable driver.**

A function nothing executed still belongs in it. If the denominator followed whatever a harness or a
driver happened to link, coverage would be circular: code the harness never referenced would vanish
from the denominator and the fraction would rise for free. The scope filter stays what it is today —
the scored modules of the pair's linemap, which exclude transpiled drivers (`test`, `example*`,
`smoke`, `main`) — so "declared translated-code scope" means the library modules of that artifact.

## What has to be done, and what does not

1. **The running chain finishes unchanged.** No mid-flight switch. (2026-09-08: qsort ×6, quadtree ×3
   done; urlparser running; lodepng ×2 and optipng ×3 queued. lodepng and optipng are
   TEST-UNAVAILABLE by construction, so nothing in the queue produces tests-side data.)
2. **Afterwards, build an independent rlib denominator for the three cells that lack one**, and
   restate their coverage against it: `bzip2 × c2rust`, `genann × c2rust`, `genann × Laertes`. Every
   other paired cell (genann × CROWN, tulip ×2, quadtree × c2rust, urlparser × c2rust and CROWN)
   already has `raw/denominator.json` archived. Rebuild only; no fuzzing, no new campaign.
3. **Do not estimate a theoretical maximum coverage.** Instrumentation yields the syntactic region
   count, not which regions are reachable under some valid input. The paper states the conflation
   instead:
   > The denominator is the complete instrumented region set in the declared artifact scope.
   > Consequently, uncovered regions may include both behavior not reached by our campaign and code
   > infeasible under any valid input; distinguishing the two generally requires reachability
   > analysis.
4. **Do not bucket uncovered regions.** It would need harness regeneration and line-number recovery,
   it carries manual judgement, it cannot be made uniform across the matrix, and it does not earn its
   space in ten pages. The three named explanations already carry the argument:
   * **tulip** — inputs land outside the valid option domain: broad boundary invocation, shallow
     execution inside them;
   * **cJSON × PtrTrans** — the bridge does not support its reshaped object interfaces;
   * **lil × C2SaferRust** — the producer itself fails, blocking every dependent boundary.
5. **qsort's four 100 % cells are an example, not a corpus-level claim.** They show that for those
   artifacts the instrumented set contains nothing unreachable. The infeasible fraction may differ
   per artifact, and no general statement is made from them.

## One caveat on step 2, found before starting it

Rebuilding the denominator is straightforward; **restating the numerator may not be.** What is
archived per cell is:

| level | archived | recomputable from the archive alone |
|---|---|---|
| function universe | the four identity lists (`covered_by_both`, `only_tests`, `only_ours`, `covered_by_neither`) — their union is the universe | **yes** |
| region universe | the count only (`total_in_scope` = 8 789 / 573 / 587) | count only |
| ours-covered regions | the count only (7 090 / 462 / 467) | **no** |

Recomputing a region-level covered set means re-running `c2r_coverage.py::extract` over the archived
llvm-cov exports, and that needs **each harness's own `src/lib.rs`** for the line alignment:
`--expose-entry` inserts a `#[no_mangle]` line, so a harness's lib.rs is shifted by one against the
canonical flattened file. The archive keeps the fuzz target and `build.rs`, not that file, and the
cells' scratch directories are gone.

So step 2 runs in two stages:

* **(a)** build the rlib denominator for the three cells and compare it with the archived universe —
  the function identity set exactly, the region count numerically;
* **(b)** if they agree, the published numbers stand unchanged and only the provenance sentence moves
  from "the instrumented build that ran the transpiled suite" to "the translation's own instrumented
  objects". If they disagree, the numerator has to be recomputed, which requires regenerating those
  cells' harnesses — deterministic, the generator hash is recorded per funnel row, and still no
  fuzzing. That decision gets raised before it is taken.

Expected outcome is (a) agreeing: both routes compile the same crate with `-C link-dead-code`, and
the two routes were verified identical on every cell where both exist.

## Fix worth making when the generator is next unfrozen

`--expose-entry` should not change the line count of the harness's lib.rs (rewrite in place instead
of inserting a line). That removes the alignment step entirely, and with it a class of failure that
has already caused one bug (every function after the exposure point attributed to the wrong line) and
now blocks post-hoc region analysis. Until then, record the insert position and line count per
harness so alignment needs no file.
