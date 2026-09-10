# RQ4 runbook — how to run a library's cells without repeating our mistakes

*Operational lessons from bzip2, genann and cJSON (2026-09-04/05). Every item is something that
actually went wrong once. The protocol is `results/rq3_coverage/PROTOCOL.md`; the pipeline is
`scripts/rq4/`; this page is the part the protocol does not say.*

## Before the first cell

- [ ] **Pair per tool, C source = what the translator consumed.** Record the sha in the cell. CROWN is a
      Rust→Rust lifter applied to the c2rust translation: its C is c2rust's C, not the copy under
      `crown/c-code/` (genann-1.0.0 vs 2015 cost a killed cell). The planner now refuses a target the Rust
      translation does not define; a mismatched pair shows up as "not present in the Rust translation".
- [ ] **Pairs live in `benchmark/pairs/rq4/<lib>_<tool>/`** (`source/`, `translated/` with `.linemap.json`
      + `.defs.json`, `build/compile_commands.json` with absolute paths, optional `drivers/*.c`). Flatten
      with `scripts/flatten_translation.py --auto`; a single-file crate gets an identity linemap (see how the
      cJSON / SACTOR pairs were built). CROWN wraps modules in `pub mod src {}` — flatten keeps it.
- [ ] **Tests side first, recomputed from preserved outputs, never from memory.** `tests_side.sh` (bzip2),
      `tests_side_genann.sh`, or `denominator.sh` when the suite is unavailable/fails. Keep `run/` outputs
      so the pass count can be recomputed (Laertes bzip2 was "3/6" in a note and 0/6 in fact).
- [ ] **Shims**: macOS-transpiled crates need `benchmark/pairs/rq4/darwin_shims.c` (`__assert_rtn`,
      `__maskrune`, `__stderrp`…); pass `--shim`. A translation written against `libc::` gets the dep added
      automatically.
- [ ] **Plan the whole library once** (`harness_plan.py --pair … --all --json`) and read the failure
      reasons before spending an hour. Reshaped signatures fail honestly now (arity, `&str` returns,
      two produced objects); if something is "planned" that cannot possibly build, the planner is lying
      and that is the bug to fix first.
- [ ] **Files, not bytes.** The scratchpad has a file-count quota (~300k) that `df -h` and `df -i` do not
      show; `EDQUOT` kills the Bash tool itself. `find $S -type f | wc -l` before a run; archive and
      delete finished cells; never leave four cargo target dirs building at once.

## Running

**Preflight is mandatory (user decision 2026-09-06, after lil_parse crashed for an hour unseen).**
`cell.py` now runs, after the builds and before the campaign: (1) every harness once on the
EMPTY input in c-only and in rust-only mode, and (2) a 60 s fork-mode run with the campaign's own
parameters. A boundary whose C side crashes on the empty input, or whose short run shows
`crashes ~ jobs` with a corpus that never grows, is **flagged**; unless it is listed in
`<pair>/preflight_accept.txt` (reviewed crash-alls: lil's parser internals) the cell stops with
exit 3 *before* spending its hour, the serial script prints `PREFLIGHT_REVIEW <tool>` and moves
to the next cell, and the person decides: generator bug (fix, re-run) or unconstructible
precondition (add to the accept file with the reason, re-run). The preflight corpus is deleted;
the campaign is the campaign. Results are in `preflight/preflight.json`, in each funnel row, and
in RUN.md.

- [ ] **One cell at a time.** `run_<lib>_serial.sh` pattern: free-space precondition, reclaim `target/`
      after each cell. Confirmation never runs beside a campaign (CPU contention breaks comparability).
- [ ] **Launch detached**: `nohup setsid bash chain.sh > out 2>&1 < /dev/null & disown`. Tool-managed
      background tasks were stopped twice within a minute; detached chains survived hours.
- [ ] **Kill by PID or with the `[c]` trick** (`pgrep -f 'rq4/[c]ell.py'`); a plain `pkill -f` matches
      the shell that runs it and exits 144 with the rest of the command skipped. A pattern that also
      matches a Monitor's `tail -F … <log>` kills the monitor.
- [ ] **Budget is 3 600 s and pre-registered**; snapshots at 60/300/600/1 800 s are hard-linked corpora,
      so the 300 s number is recomputable (`recollect.py --snapshot 300`) — no second run, ever.
- [ ] **Seeds are part of the measurement**: shipped sample inputs encoded into the harness format
      (bzip2 438 → 39 corpus inputs without them). genann has none; cJSON's producer is `cJSON_Parse`.
- [ ] Run-to-run variance is real (≈5 % on one bzip2 boundary between two runs of the same
      configuration). Report single-run numbers as single-run numbers.

## What the noise looks like (adjudicated, never promoted)

- `genann_init` as a target: full-range scalars → overflow panics = `ub_associated` (one-sided guard).
- A target that returns a fresh object (`genann_copy`, `cJSON_Duplicate`): nothing frees it → fork
  children hit the rss limit → `oom-*` artifacts that replay normally = `not_reproducible`.
- A reference with its own bug: old cJSON's `parse_string` `\u` overflow, genann's `assert(!isnan)`.
  The producer inherits it; ≈ 2 500 crash artifacts per boundary per hour, all `ub_associated`. The
  generator must not hide it. `cell.py` prunes candidates to 500 per channel after the campaign
  (gz manifest keeps every sha256); `finish_cell.py` finishes a cell that died after its campaign.
- Internal sort routines with out-of-contract input models: 10⁴–10⁵ artifacts per tool, identical
  shape on every tool, sampled at 200 per channel (PROTOCOL §4).
- Wild-address ASan reports are `instrument_only` / `out_of_contract_access` by the layout rule; the
  claimable evidence is a value divergence with the C side in contract (CROWN `bhtab`).

## After the campaign

- [ ] `replay_cell.py` (step 6, the value channel) → `recollect.py` for anything `failed rc=1`
      (read `harnesses/<b>/coverage_cmd.log` first) → `confirm_cell.py --sample 200` → full only on the
      public boundaries / any boundary with a `confirmed_*` in its sample (PROTOCOL §4).
- [ ] `c2r_coverage.py --tests` only when the suite passes completely; otherwise `--denominator`.
      Cross-tool comparison is by fraction and candidate count, never raw region counts.
- [ ] `run_md.py` (numbers; prose after `<!-- prose -->` survives regeneration) → `cell_table.py` →
      `archive_cell.py` (≈ 10–50 MB per cell: verdicts gzipped, stderr only on confirmed rows,
      confirmed inputs kept whole) → `SUMMARY.md` → README / INDEX → the user commits.
- [ ] **After any manifest change**: `scripts/rq4/summary_all.py` has a hand-kept `NEW_IN_RQ4` set (the
      ids first found by an RQ4 cell); an id missing from it is printed as "catalogued earlier; re-found"
      (this misreported C11/S15/C12–C14/S16–S17 on 2026-09-09). Update the set, then rerun
      `summary_all.py` and `paper_tables.py`. Also `cell_table.py` per changed library (cells.json).
- [ ] A defect is promoted only on `confirmed_*`; one root cause on three boundaries is one defect
      (Laertes `incs`); a value divergence at an internal boundary can be the precise localisation of a
      catalogued crash (CROWN `SET_BH`). Re-read `results/rq4_effectiveness/gen_defect_manifest.py`'s
      entry format before adding one, then `--build`.

- [ ] **Recovering a cell whose scratch directory is gone** (lodepng × c2rust, optipng × c2rust,
      2026-09-09): `harness_exports.tar.gz` unpacks as `ours/<boundary>.json` — extract into the
      cell dir, not into an `ours/` dir (that gives `ours/ours/` and `c2r_coverage.py` sees nothing,
      reports 0 and passes its own sanity checks). Each export records the ABSOLUTE path of its
      harness's `src/lib.rs` for the `--expose-entry` alignment; regenerate the harnesses with
      `rebuild_bins.py` (deterministic, no fuzzing) and pass
      `c2r_coverage.py --path-map <old cell dir>=<regenerated dir>` — recorded in `result.json`
      as `path_map`, never a symlink at the old path — and write `analysis/recovery.json`.
      Accept only with 0 functions outside the universe.

- **A `combined` panic is NOT a confirmed termination until the no-sanitizer replay traps too**
  (`classify`, second gap, found 2026-09-09 during the optipng triage, FIXED). The panic branch had
  returned `confirmed_termination` without consulting channel D. 234 archived rows panicked only in
  the ASan build and returned normally without it — optipng × Laertes `uncompress` 200 (an index
  read from uninitialised heap: ASan fills it with 0xBE, so 0xBEBE = 48830 goes out of bounds; a zeroed
  heap gives a silent wrong result), optipng × C2SaferRust `uncompress` 31 (an overlapping
  `copy_from_slice` memcpy the interceptor rejects), tulip × C2SaferRust `ti_adx_start` 3. All
  re-classified offline to `instrument_only`; C14 became S18 (semantic) because its termination
  evidence was the instrument's. The rule as fixed: channel D **normal** → `instrument_only`;
  **panic** → `confirmed_termination`; **signal** → `confirmed_termination` only on the zero page,
  else `out_of_contract_access`; **timeout** or **no replay** → `inconclusive` (never swallowed as
  instrument-only). lil × CROWN's 2 `lil_parse` rows with a no-sanitizer timeout went to
  `inconclusive` the same day. **Check on every cell: a `confirmed_termination` row has
  `rust_no_sanitizer.outcome` = panic, or signal on the zero page.**
- **An overflow check on a wrap C defines is still a defect when the C is in contract**
  (optipng × C2SaferRust `optimize_cmf`, C16, 2026-09-09): `--z_cinfo` on unsigned 0 wraps in C,
  c2rust emits `wrapping_sub`, C2SaferRust emits checked `-=`: the translation panics under the
  crate's debug profile and matches C only in release. Decide with a deterministic probe on a
  VALID input (header 08 1d, FCHECK ok), not by "release happens to agree".
- **A harness input-model gap that looks like a confirmed termination** (optipng × C2SaferRust
  triage, `results/rq3_coverage/optipng/c2saferrust/TRIAGE.md`): a *harness
  input-model gap* — `bmp_memset_bytes(ptr, offset, ch, len)` passes `ptr + offset` to `memset`, the
  planner does not model libc sinks, so `offset` is an unbounded scalar and both sides compute an
  out-of-bounds pointer (C silently, Rust's `ptr::offset` debug precondition traps; 117 rows,
  out-of-contract input, not promoted). Planner rule for the next version: memset/memcpy/strcpy
  arguments derive an extent `offset + len <= extent(ptr)`.

## Generator/planner traps met so far (all fixed; here so a regression is recognised)

coverage replay without `-timeout=25` hangs on a looping input and its `TimeoutExpired` killed the
cell · LSan on the coverage replay failed every boundary that returns malloc'd memory · one-sided
rejection guards had no lowering · `let ref mut fresh` compound assignment is where CROWN's rewrite
broke (`|=` → `=`) · C `static` entries: single-TU pairs keep the static in the oracle TU (fixups
looked for siblings only), forward-declared statics need every `static` stripped, flat crates need no
module re-export · two rest-taking variable-length inputs starve the second · the target that is the
destructor must not be canonicalised afterwards · function-pointer aliases (`cJSON_free = free`)
must feed the call-graph fixpoint · the return contract must read the Rust return type, not the C
sentinel · a `None` template must fail construction, not fall through to an address comparison.
- **Denominator collapsed to 2 functions (cJSON × PtrTrans).** The link-dead-code bin's reference
  call was `cJSON_Version()`, small enough for rustc's automatic cross-crate inlining, so the bin
  referenced no symbol of the rlib, the archive member was never pulled, and `-C link-dead-code`
  had nothing to keep. `#[no_mangle]` translations never showed it (exported symbols are not
  inlined away), which is why every earlier universe was right (verified identical, both routes,
  on all of them). Fixed at the root: `scripts/rq4/rlib_universe.py` exports the universe from the
  rlib's own instrumented objects; `denominator.sh` and the tests-side scripts call it. A universe
  smaller than the translation's `pub fn` count is a bug, not a result.
- **Same shape, one spelling (lil).** c2rust keeps `pub type lil_t = *mut _lil_t;`; the producer
  check saw "returns lil_t" and refused, so the bridge lit on Laertes and CROWN but not on the
  faithful control. `rust_type_aliases` now resolves bare pointer aliases. Check the negative
  control's planned count against the others' before running a library.
- **tulip's `pub mod indicators { pub mod abs; ... }` root block** was carried verbatim by the
  flatten as a "support module" and broke the build (`file not found for module abs`); directory
  groups are now emitted inline and skipped in the carry-over.
- **The same pointer typedef, a second time, in the generator (lil × c2rust, first cell attempt).**
  The planner resolved `lil_t` (above) and planned 51 boundaries, but the generator read the
  translation's parameter/return types raw: a produced `lil: lil_t` was passed `as *const _`
  (E0308, `ateol`) and a `lil_func_t` return was "neither a raw pointer, a reference, nor an
  Option" (`add_func`, 4 of the first 5 builds). The alias map is now applied at the three
  consumption points (`_ptr_alias`); plans.json keeps the translation's own spelling. The cell
  was killed in its build phase and restarted; no data from it exists. Lesson: after a planner
  fix, generate and BUILD one harness from the negative-control pair before launching a chain.
- **`size_t` parameters (lil, second attempt): E0308 `expected u64, found usize`.** The generator's
  C-ABI mapping says `size_t` = `usize`; c2rust spells it `c_ulong` = `u64`. Same width, different
  name, and the plan's `scalar_cast` bridge was recorded but never materialised for scalar /
  length / capacity roles. `_call_and_decl` now casts to the translation's resolved primitive when
  it differs (`codelen as u64`). Six lil boundaries, `lil_parse` among them.
- **A C `static` chosen as producer (lil `real_trim`): E0425 `cannot find function`.** The planner
  found the function in the translation but it is not `pub`, and only the *entry* gets
  `--expose-entry`. Producers must be public in the translation; `_plan_producer` now excludes a
  private one with the reason, and the ranking moves to the next viable candidate
  (`lil_alloc_double`, the public API). Nine lil boundaries.
- Both were found by watching the first 14 builds of the restarted cell: an OK/FAIL tally after the
  first dozen builds is cheap and catches a family of failures before the hour of fuzzing.
- **Comparator plugin vs a reshaped translation (cJSON × PtrTrans): E0425 `cJSON_Delete`, E0609
  `type_0`.** The plugin was matched by C type name and linked blind; its Rust half did not
  compile against PtrTrans's struct. Now `[plugin.requires]` in the manifest is checked against
  the translation (`plugin_compat`) and an incompatible plugin is dropped for that translation
  with the reason printed and recorded in the verdict (`plugin_degraded`); the return contract
  falls to pointer nullness. Never fix this by editing the plugin for one translator.
- **Array-typed parameters (qsort, 2026-09-07): `int arr[]` was "unsupported param type".** A parameter
  declared with array type IS a pointer (C11 6.7.6.3p7); `describe_param_type` now decays it, only
  for parameters (fields and pointees keep their array shape). All six qsort pairs went 1/3 → 3/3.
- **Wrappers that never dereference (qsort `quickSort`, 2026-09-07): zero-filled buffer, unbounded
  indices.** The boundary passes `arr, low, high` to `partition`, which does the indexing; the
  analyzer saw "pointer is never dereferenced" and planned a zero array with plain scalars, so the
  campaign would have been all out-of-bounds. `BodyAnalyzer` now carries a callee's facts about its
  own parameters back onto the arguments the boundary passes (depth ≤ 2, no recursion, memoised):
  derefs, subscripts with the bound rewritten to the boundary's parameters, loop-trip controls,
  advanced pointers, escapes and rejection guards. Rule 7 is untouched (nothing looks at callers of
  the boundary). The 29 golden plans are byte-identical; the generator hash changed, so cells run
  from here carry a different hash than the first nineteen (recorded per funnel row).
- **Renamed entries (qsort × SACTOR / PtrTrans): `<pair>/translated/renames.json`** is the RQ1 map;
  `harness_plan.py`, `c2r_funnel.py` and `cell.py` pass it as `--rust-entry`. Absent = identity.
- **Single-file crate roots as modules (`make_pair.py`):** crate-level `#![feature]` /
  `#![register_tool]` lines are dropped when the file becomes a module (the flatten's header carries
  them); a Laertes root's `mod laertes_rt {}` / `mod __laertes_array {}` blocks are moved verbatim to
  a synthesized lib.rs (`--split-root-mods`) so `crate::laertes_rt::*` still resolves.
- **quadtree / urlparser, 2026-09-07 (all fixed the same day, generator hash changed again):**
  a two-line `static void\nname(` definition was not stripped (`strip_static_c` now spans lines);
  a static defined in a sibling **header** (url.h's `strff`) was never stripped (siblings now include
  `*.h`); the harness compiled `c/test.c` without `-I c` so `#include <url.h>` failed (build.rs has
  `.include("c")`); a POD struct behind a pointer (`quadtree_point_t*`) was planned as `struct_value`
  with no lowering (planner lied) -- now `input_struct` / `inout_struct`, refused when the boundary
  frees it; the type name the target uses (`translated::quadtree_point_t`) is re-exported from the
  ENTRY's module (c2rust re-declares types per file, CROWN aliases per module); a produced object is
  passed `as *mut _` because the producer's module spells the struct differently from the target's;
  a directory module (`src/quadtree`) needed its leaf name for the static re-export; `exclude.txt`
  drops a driver's `main` when the TU is the test program.
- **Shipped suites that fail under the translation (recorded, denominator only):** quadtree × CROWN
  (`NonNull::new_unchecked requires non-null`, debug-assertions), urlparser × Laertes (SIGSEGV),
  urlparser × C2SaferRust (double free). A failing suite is never 0 %, and it is E1 evidence, not RQ4.
- **lodepng / optipng, 2026-09-07:** an `enum` parameter or field was "unsupported" (now its
  underlying integer; c2rust spells it `pub type E = c_uint`, resolved through the alias map --
  47 lodepng boundaries); `size_t*` vs c2rust's `*mut u64` was "different element type" (same
  width, accepted, like the earlier scalar `size_t` fix); optipng's zlib headers have no include
  guards, so the pair is **multi-TU** (`make_pair.py --tus`: one compile command per unit, every
  `source/` directory an include dir, fixups emit every unit into build.rs with the shared rename
  defines) -- the planner parses each unit ONCE (`parsed_tus` cache; before it, every lookup
  re-parsed all 52); directory modules are emitted contiguously in the flatten (an unscored
  `libpng/pngtest` after `zlib/*` opened `pub mod libpng` twice); driver detection uses the leaf
  name of a directory module.
- `pkill -f harness_plan.py` kills the shell that runs it (exit 144): use `pgrep -f "harness_[p]lan"`.
- **zlib's `adler32(adler, buf, len)` (optipng): `buf` planned as a NUL-terminated string, `len` a free
  scalar → heap overflow on both sides (316/9 646 crashes in the smoke).** The body advances `buf`
  (`*buf++`, unknown required extent) and the adjacency/name heuristic skipped every pointer that had
  ANY required-extent entry, unknown included. It now applies when the extent is unknown; adler32
  went to 1 M execs / 0 crashes. Smoke a few boundaries of a new library before its chain: 8 s of
  fuzzing on one harness caught this; a preflight would have flagged it only per cell.
- **The archive went to the scratchpad for nine cells (2026-09-08), and said nothing.**
  `archive_cell.py --dest` defaulted to the RELATIVE `results/rq3_coverage`, and the new chain
  scripts run from the scratchpad (the older tulip chain happened to `cd` into the repo first), so
  qsort x6 and quadtree x3 archived into `<scratchpad>/results/` while their RUN.md, cells.json and
  tables went to the repo through absolute paths. The success line printed as usual. Recovered in
  full (corpus, plans, analysis, divergence inputs) because the scratchpad had not been pruned yet.
  Fixed at the root: the default is now absolute, derived from the script's own location, and the
  post scripts pass `--dest $R/results/rq3_coverage` as well. **Check `ls results/rq3_coverage/<lib>/<tool>`
  against a finished library before deleting a cell from the scratchpad** -- an archive is 15-16
  entries, not two.
- **A `(begin, end)` RANGE PAIR is planned as two independent buffers (lodepng, 2026-09-08).**
  `lodepng_chunk_find(unsigned char* chunk, unsigned char* end, ..)` guards with
  `if (chunk >= end || end - chunk < 12) return 0;`: the two pointers denote ONE buffer. The plan
  allocated two 4096-byte buffers, so the guard compares unrelated allocations and whether the loop
  walks off the end is memory-layout luck -- c-only ran 110 559 executions clean with a growing
  corpus while rust-only heap-overflowed at once, **on the faithful c2rust control**. Read as a
  defect this would have been a false positive on the negative control; it is an input-model gap.
  The signature is already in the plan: the end pointer's own evidence says *"pointer is never
  dereferenced"* while the other is *"pointer advanced in the body"*, and the body compares them.
  **To fix when the generator unfreezes:** when two pointer parameters of the same element type are
  compared to each other (or subtracted) in the body, allocate ONE buffer and set the second to
  `base + n`. Affects `lodepng_chunk_find` and `lodepng_chunk_next`; the latter was NOT flagged by
  the preflight, so the same lottery can hide the shape entirely. Until then the boundaries run and
  are recorded as a construction limit in `<pair>/preflight_accept.txt`.
  **Second instance, another library and another translator (optipng x Laertes, same day):**
  `png_format_number(start, end, ..)` (libpng/pngerror.c:133) writes BACKWARDS from `end` -- its
  first statement is `*--end = 0;` -- and the plan gave `end` an independent `output_array` whose
  extent it PROVED to be exactly 1, so the first write is already out of bounds. The rule must
  therefore fire on a pointer that is DECREMENTED as well as on one that is advanced, whenever it is
  compared with another pointer parameter of the same element type. The tell in the plan is one
  pointer with a tiny proven extent (or "never dereferenced") beside another of the same element
  type, with the body comparing the two.
- **Two chains can fire on the same DONE marker.** The urlparser Laertes re-run and the lodepng chain
  both waited on `URLPARSER_CHAIN_DONE` and started two cells a minute apart (2026-09-08 03:46).
  Re-runs now wait for the LAST library's DONE (`rerun_queue.sh`), never a mid-chain one.
- **A too-broad `sed` put `--dest` on the CONFIRM line and the confirmation silently did not run
  (2026-09-08).** Fixing the archive destination, the substitution matched `--pair $PAIRS/<lib>_$T`
  in BOTH the `archive_cell.py` and the `confirm_cell.py` invocations; `confirm_cell.py` rejects the
  argument, exits 2, and the post script carries on. Four cells (urlparser × c2rust / C2SaferRust /
  CROWN, lodepng × CROWN) reached `RUN.md + archive` with no adjudication at all, and the only
  visible symptom was a confirm step that took **zero seconds** and an archive with no
  `confirm_sample/`. Nothing was lost -- every candidate and divergence INPUT is archived -- but the
  verdicts had to be recomputed. **Check after every post: each finished cell has `confirm_sample/`
  with a non-empty `total`, and the step took more than a second.** Editing a chain script with a
  regex is how this happened; edit the one line, not a pattern.
- **`confirmed_termination` was emitted with its own definition INVERTED when C alone times out
  (`scripts/c2r_campaign.py` `classify`, found 2026-09-08, FIXED 2026-09-09 → `inconclusive`).** It
  had produced 12 false rows, all `Adam7_interlace`/`Adam7_deinterlace`, lodepng × c2rust (6) and
  lodepng × CROWN (6); both cells were re-classified OFFLINE from the archived
  `confirm_sample/*_verdicts/verdicts.json.gz` (rows, clusters.json, summary.json, RUN.md §6; the
  summary carries a `reclassified_2026_09_09` note). No re-run. The original text:
  ```python
  if a_c["outcome"] == "timeout":
      return ("inconclusive" if b_rust["outcome"] == "timeout" else "confirmed_termination",
              "C alone times out")
  ```
  `confirmed_termination` means *"C returned normally and the translation did not"*. Here C did NOT
  return and the translation DID: the four channels read c_only=timeout, rust_only=normal,
  combined=timeout, rust_no_sanitizer=normal. It should be `inconclusive` (or a named
  c-alone-timeout bucket): the C replay carries ASan **and full UBSan** and is an order of magnitude
  slower, so a timeout there is as likely to be instrumentation cost as non-termination, and the
  budget is a replay budget, not a proof that C loops forever.
  **Blast radius, measured over every archived cell: 6 rows, all in lodepng x c2rust**
  (`Adam7_interlace` 5, `Adam7_deinterlace` 1) -- i.e. six false defects on the NEGATIVE CONTROL.
  The other 641 `confirmed_termination` rows come from the two correct branches ("C returned
  normally, the translation panicked", 578; "no UB check fired on C alone, Rust alone fails", 63).
  No manifest entry rests on the six. Fix the branch, and RE-CLASSIFY the archived rows offline --
  `verdicts.json.gz` stores all four channels per row, so no replay has to be repeated.
- **Universe rlib selection.** `rlib_universe.py` takes cargo's `--message-format=json` log and
  picks the exact lib artifact of that build; the newest-by-mtime rlib is only a fallback and is
  labelled as such in `denominator.json._source.selected_by`.
- **`size_t` RETURN (lil `lil_list_size`): E0308 `expected usize, found u64` at `c_ret != r_ret`.**
  The i128 widening for different-but-compatible integer returns keyed on the raw Rust spelling;
  it now resolves the typedef first. The lil × c2rust cell had already started (50/51 built), so
  that boundary is recorded as unbuilt there with this reason; later cells build it.
- **Buffer tables (tulip) are a planner capability now** — see
  `docs/harness_plan_architecture.md`, addendum 2026-09-05: `T**` indexed only by constants →
  rows as pseudo pointer parameters; pointers the body advances (`*out++`) → unknown extent
  (this was an under-allocation for plain parameters too). Tulip cells run with
  `cell.py --max-len 65536` (recorded in `campaign_params.json`); the default stays 4 096.
- **File-count quota, third time (lil × Laertes, build phase).** The scratchpad held 221 k files:
  ten ARCHIVED cell directories (bzip2 ×4, genann ×5, cJSON) still sat there with their corpora,
  candidates and target trees, and one more cell's build pushed it over. Archiving is not
  cleaning: after `archive_cell.py` confirms the archive (RUN.md + corpus.tar.gz in results/),
  delete the cell directory and every `target` tree, and check `find $S | wc -l` before a chain
  (< 100 k is comfortable). `touch` is the quota probe; `df` cannot see it.
- **`UnicodeDecodeError` in the coverage phase (lil × c2rust).** `cargo fuzz coverage` echoes
  corpus file names; libFuzzer names are hex, but a seed copied from a crash artifact was not.
  Every `subprocess.run(..., text=True)` in the RQ4 scripts now decodes with `errors="replace"`.
  The cell's campaign was complete; `finish_cell.py` (now resumable: exported boundaries are
  kept, not recollected) finished it.
- **A length-0 input buffer was a dangling pointer (lil `lil_parse`): every execution SEGV'd
  in `strlen`, on BOTH sides, and the campaign's corpus stayed at 1 for 3 600 s** while the
  fork-mode log counted 70 k "crashes". `Vec::new().as_ptr()` is 0x1, and lil treats
  `codelen == 0` as "use strlen(code)". Length-carrying buffers (`in_buf`, `io_buf`, both
  copies) now reserve one element past `len` and write a sentinel 0 there — `len`, slices and
  clones are unchanged; only the allocation is never dangling and always NUL-terminated. Golden
  re-frozen (14 entries changed, all by this one line). Detection rule for next time: after a
  campaign, list boundaries whose fork log ends with `crashes ≈ jobs` and `corp ≤ 1`; a boundary
  that crashes on the EMPTY input is a harness bug until proven otherwise (`ateol`,
  `get_dollarpart`, `next_word` also crash-all, but legitimately: parser internals whose
  precondition is an active parse the harness cannot construct). The c2rust and Laertes cells
  keep their campaigns; `lil_parse` alone is re-fuzzed and merged back
  (`scripts/rq4/merge_boundary.py`, deviation recorded in `deviations.json`).
- **C `static` entries under a namespaced translation (lil × CROWN): E0432 `unresolved import
  crate::lil`.** CROWN wraps modules in `pub mod src { .. }`; the funnel's `--expose-entry`
  re-export used `crate::<module>::<fn>`. It now copies the prefix from the flatten's own
  `pub use crate::<prefix><module>::` lines. 15 of 42 CROWN builds; the cell was restarted
  (its campaign had run 20 min).
- **A CRASH-ALL translation (lil × C2SaferRust: `lil_new()` itself crashes) yields `failed rc=1`
  coverage on every boundary that needs the object** — batch and per-input replays alike, because
  a crashing process writes no profile. That is the result, not a collection failure:
  `annotate_campaign_status.py` (run by post_*.sh) marks those rows `crash_all` with a note, and
  RUN.md must say "no coverage: the translation crashes on every input", never "export failed".
- **BYTE quota, not only file count (tulip × 4, twice: every cell died in its build phase at
  ~10–12 GB of scratchpad).** Three multipliers per harness: (1) every harness crate compiles
  its own copy of the translated library into the SHARED cargo target — 213 rlibs + objects +
  C-oracle build dirs; (2) the kept campaign binary is 18.6 MB of which 16 MB is debug info;
  (3) `cargo fuzz coverage` leaves a ~200 MB instrumented target under each harness. Fixes in
  `cell.py`: `_prune_target` removes the harness's own artifacts from the target after its
  binary is kept; the campaign `.bin` is `strip --strip-debug`ed (fuzzing and replay never
  symbolise; confirmation builds its own `_san`/`_nosan` with debug info and now deletes them
  after each boundary); the coverage phase deletes `harnesses/<b>/{target,fuzz/target,
  fuzz/coverage,percov}` after the export. The serial scripts abort a cell when the scratchpad
  exceeds 6 000 MB or 150 000 files. Measure with `du -sm $S`, not `df`.
- **The fuzzed library wrote 855 files into the repository root (lil).** lil's `store` builtin is
  `fopen(name, "wb")` with the name taken from the script, and the campaign, preflight, coverage
  replays and confirmation all ran with the chain's cwd (the repo). Nothing tracked was touched
  (checked: no deletions, no unexpected modifications, home clean), but it could have been. Every
  harness execution now runs in the cell's own `sandbox/` directory (`cell.py::sandbox_dir`,
  `c2r_campaign.py` confirm). lil has no `system`/`popen`; a library that does would need the
  environment adapter the planner already names for parameters flowing into effectful calls —
  and a harness for it must not run outside a sandbox at all. Before committing, look for
  untracked top-level files with campaign-window mtimes.
- **First real PREFLIGHT_REVIEW (tulip × Laertes, `ti_find_indicator`).** Empty input: C ok,
  Rust crash; 60 s run 129/130 jobs crashed, corpus 0. Source: Laertes renders the static
  `ti_indicators[105]` table as all-default (`::new()`), name pointers NULL — the
  initialization-loss family (C8's mechanism). That is a translation-side crash-all, i.e. a
  finding for the campaign and the confirmation to record, so the review outcome is: write the
  reason into `<pair>/preflight_accept.txt` and re-run the cell with `--reuse-bins` (the build
  phase stands). The rule of thumb: **C ok + Rust crash on the empty input = translation,
  accept with reason; C crash on the empty input = harness bug until proven otherwise.** The
  serial script had moved on to the next cell meanwhile, as designed; the re-run is queued after
  the chain (`tulip_chain5.sh`).
- **Target pruning, second pass (tulip × C2SaferRust).** After the first pruning pass the
  target-triple profile was down to 94 MB, but `target/release/build/<crate>-<hash>/` — the HOST
  profile, one compiled build script plus its C-oracle outputs per harness — held 160 × ~6 MB.
  `_prune_target` now prunes both profiles. Measure `du -sm <cell>/target/*` during a build
  before trusting any pruning claim.
- **A grep is a hypothesis, a probe is evidence (tulip × C2SaferRust).** A regex for
  `options.offset(0) as i32` matched 19 `_start` functions and read as a family of pointer-cast
  mistranslations; a deterministic probe (options[0] = 5.0, combined replay on the cell's own
  binaries) showed 18 of them agree with C — the pattern had not excluded the leading `*`. One
  site (`ti_adx_start`) is real (S15). Never widen a defect from one confirmed site to a family
  without replaying each site.
- **Zero-page faults are deterministic (tulip × Laertes `ti_find_indicator`).** The layout-luck
  rule (`out_of_contract_access` for a raw signal without a panic) does not apply to a fault on
  the zero page: that page is never mapped, so a NULL dereference faults on every layout, on
  both sides. `classify` now records `null_page` from the ASan report ("address points to the
  zero page" / address < 0x1000) and returns `confirmed_termination` for it; the three
  ti_find_indicator candidates were re-adjudicated under the new rule.
- **tulip's option domain is the coverage gap.** Options are doubles the indicator casts to
  int; random doubles overflow the cast (C-side UB, 243 `ub_associated_termination`, one per
  boundary) or land outside `1..size`, so every corpus saturated within a minute (1528 → 1640
  inputs, 60 → 1800 s) and the validator reached 0.34 of the regions the smoke suite reaches
  (functions: 212/213 both). A rejection guard on a local derived from an array element
  (`period = (int)options[0]; if (period < 1) return`) is not a parameter guard the planner can
  see; that is the concrete input-model limit this library exposes, recorded, not patched.

## Seed refinement (2026-09-10)

Global deterministic seeding policy, shared codec with the generator, census and rerun rule: `docs/seeding_policy_plan.md`. tulip results: `results/rq4_llm_refinement/tulip/`.

**Oracle NaN blind spots → generator 0.9 (2026-09-10).** Found by the tulip grid-corpus replay: the f64 row compare (`to_bits`) reported NaNs with different payloads as a divergence (C keeps an input NaN's payload, Rust emits the canonical NaN), and the read-only `plan_arr` compare used `Vec<f64> !=`, so identical NaNs compared unequal — 86 false `confirmed_divergence` across the four seeded corpora, verified per input with `scripts/rq4/seed_experiment/nan_probe.py`. Generator **0.9** implements the rule: numeric float outputs (rows, arrays, out params, struct fields, return values) are equal iff bits are equal or both are NaN (`c2r_feq64/32`, `c2r_fslice64/32` in the prelude); NaN against a number is a divergence; raw byte buffers and plugin canonical forms stay byte-for-byte. A run whose only differences were NaN payloads reports **`nan_equivalent`** (a distinct outcome in `c2r_campaign.OUTCOMES`, its own verdict in `classify`, never a candidate in `replay_cell.py`). 0.9 changes every emitted harness (golden re-frozen, 33 entries); the 37 archived cells stay bound to the 0.8 hashes in their funnel rows and are not re-run — their divergences were all UB-gated or adjudicated on integer/byte outputs. The tulip seeded corpora are re-replayed and re-adjudicated under 0.9 (`scripts/rq4/seed_experiment/rebuild_bins.py` + `replay_cell.py` + `confirm_cell.py`, results beside the 0.8 ones).
