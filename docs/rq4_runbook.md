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
- [ ] A defect is promoted only on `confirmed_*`; one root cause on three boundaries is one defect
      (Laertes `incs`); a value divergence at an internal boundary can be the precise localisation of a
      catalogued crash (CROWN `SET_BH`). Re-read `results/rq4_effectiveness/gen_defect_manifest.py`'s
      entry format before adding one, then `--build`.

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

