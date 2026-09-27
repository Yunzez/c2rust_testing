# C-guided companion run — superseded attempt-5 workflow

**Status: ABORTED.** This workflow compared an archived Rust-guided corpus grown
under an older harness with a C-guided corpus grown under the current harness,
and ran several cells concurrently on the same CPU set. It is retained only as
an audit record. The binding protocol is now
`docs/c_guided_controlled_plan.md`; no file below `cg_run/attempt-*` is an
experimental result.

Binding for this run. Nothing here is edited while a lane is executing it. Attempt 4 was stopped after 20
minutes on a review that found five defects (intermediate products only in /tmp; the "resume" deleted the
checkpoint; `result.json` was not a safe completion marker; CR and CC were measured on different harness
builds; tulip's `max_len` and bzip2's shipped-sample seeds were not reproduced). All five are fixed below.

Fixed locations (all on /home, reboot-safe):

| what | where |
|---|---|
| lane driver / stripper / pids | `~/c2rust_archive/cg_lane4.sh`, `cg_stripper.sh`, `cg_lanes4.pid`, `cg_stripper.pid` |
| lane logs | `~/c2rust_archive/cg_logs/lane4{T,A,B}.log` (one `C_GUIDED_DONE` + `FINAL`/`NOT FINAL` line per cell, `CG_LANE_DONE` at the end) |
| per-cell OUTPUT | `~/c2rust_archive/cg_run/<lib>_<tool>/attempt-N/` while running → renamed `FINAL/` once valid; failed attempts are never overwritten |
| per-cell log | `<attempt or FINAL>/cell.log` |
| universe lib.rs stand-ins (the archived universe exports name a wiped /tmp crate) | `~/c2rust_archive/universe_libs/<tests|denom>_<pair>/src/lib.rs` = the pair's flattened translation |
| work dir (deleted when the cell ends) | `/tmp/claude-1000/.../scratchpad/cg_<lib>_<tool>/` |

CPU: the three lanes were launched with `taskset -c 0-27 nice -n 10`; every child inherits it. 28 cores maximum.

## A. What happens automatically when a cell runs (`cg_lane4.sh` → `scripts/rq4/c_guided_cell.py`)

1. Skip if `FINAL/DONE.json` exists. If the latest `attempt-N` has `CAMPAIGN_DONE` but no `DONE.json`,
   resume it (the campaign is restored from its `cc_corpus.tar.gz`, never refuzzed); otherwise open
   `attempt-(N+1)`. Wait while `/tmp/claude-1000` > 13 GB.
2. Read the ARCHIVED campaign parameters (`campaign_params.json`, else RUN.md's `libFuzzer parameters:` line,
   else cell.py defaults; source recorded): budget, `max_len` (tulip 65 536), timeout, rss limit, PRNG seed.
   Initial corpus = the archived corpus's non-fuzz files per boundary (`seed_fixed`, bzip2's `words_*`, …).
3. Rebuild every archived-built boundary ONCE (generator 0.9.2, `--c-coverage`); strip the binary after copying
   the C objects; keep the tree. Record per boundary whether the regenerated fuzz target equals the archived
   one modulo the coverage/NaN-oracle lines (`build.json: harness_drift`). → `BUILD_DONE`.
4. `C2R_MODE=c-only` campaign with those parameters; every snapshot (60/300/600/1 800 s) is also tarred to the
   attempt dir. At the end `cc_corpus.tar.gz` + `campaign.json` are written atomically → `CAMPAIGN_DONE`.
5. C side on BOTH corpora with these binaries: CR (extracted from the archived `corpus.tar.gz`) and CC →
   `c_exports_{cr,cc}/`, `per_input_{cr,cc}.json`, `c_rows.json` → `C_MEASURE_DONE`; bins/objs deleted.
6. Rust side on BOTH corpora with the SAME rebuilt harnesses: one `cargo fuzz coverage` build per harness, CR
   replayed by its batch, CC replayed on the same instrumented binary (per-input fallback either way);
   exports → `rust_exports_{cr,cc}/`; `c2r_coverage.py` per arm against the archived universe →
   `rust_{cr,cc}_analysis/` → `RUST_MEASURE_DONE`. The archived Rust measurement of CR (old harness build) is
   kept in `result.json` as `CR_rust_archived`, reference only.
7. Four sets on CR, CC, CR ∪ CC; `matched_sets_*.json`, `result.json`, `RUN.md`; every JSON via
   tmp → fsync → rename; **`DONE.json` (with `valid`) is written last** and is the only completion signal.
8. The lane renames a valid attempt to `FINAL/`; otherwise it stays as `attempt-N` (nothing deleted). The work
   dir is deleted.

## B. What I do on every `C_GUIDED_DONE` / `FINAL` / `NOT FINAL` notification

1. Validate the cell, read-only, from `FINAL/result.json`:
   - `matrix.CR.C.functions_total > 0`, `matrix.CC.C.functions_total > 0`, `matrix.CR.Rust` and
     `matrix.CC.Rust` not null; `DONE.json.valid` true;
   - C-side inputs on CC (completed + crash + timeout) equal `campaign.corpus_sizes` summed; on CR equal the
     archived corpus size;
   - `campaign.max_len/seed/timeout_s/rss_limit_mb/max_total_time_s` equal the archived cell's;
   - on a c2rust cell the non-terminated exclusive sets on CR and on CR ∪ CC are empty (control);
   - `harness_drift` recorded (informational: the 2×2 is on one build either way).
2. If it passes: nothing else. No message to the user.
3. `NOT FINAL`: do not delete anything; read `attempt-N/cell.log`; note the cause in memory; the cell is
   rerun **after its lane has finished** by launching `CELLS="<lib>/<tool>" cg_lane4.sh` again with the same
   `taskset`/`nice` (it resumes from `CAMPAIGN_DONE` when the campaign survived). Never edit `cg_lane4.sh` /
   `c_guided_cell.py` while a lane runs.
4. Machine health on each notification: `uptime` load and `du -sh /tmp/claude-1000`; if load > 28 or /tmp
   > 20 GB, tell the user immediately.

## C. End of run (all three logs show `CG_LANE_DONE`, stripper exited)

1. Validate all 37 with the same checks; rerun any failed cell as in B.3; wait for it.
2. Write `scripts/rq4/c_guided_table.py` and generate the study table from `~/c2rust_archive/cg_run`:
   per cell the 2×2 (C / Rust × CR / CC), the four sets on CR, CC, CR ∪ CC with the terminated columns,
   corpus sizes, C-side crash counts; totals; the control check; the cells whose C(CC) ≫ C(CR).
3. Archive compact files into the repo: `results/rq4_c_reach/c_guided/<cell>/` ← `result.json`,
   `matched_sets_*.json`, `per_input_cc.json`, `c_rows_cc.json`, `RUN.md`, plus a compact
   `c_regions_cc.json.gz` (reached functions + covered region identities per boundary). The raw
   `c_exports_cc/` (hundreds of MB) go into `~/c2rust_archive/rq4_c_guided_raw_<date>.tar.gz`, verified by
   listing, and stay out of the repo.
4. Update `results/rq4_c_reach/SUMMARY.md` (a "C-guided companion, 37 cells" section: 2×2 totals, the
   controls, where C guidance changed reach and where it did not) and `docs/c_reach_plan.md` status.
5. Commit on `dataset-v2.1` with the attribution lines; push.
6. Merge `wt-plugin-impl` (only now: generator 0.9.2 → 0.10): replace main's uncommitted
   `docs/construction_recipe_plugin_plan.md` with the branch's copy, `git merge wt-plugin-impl`, run
   `python3 scripts/gen_harness_regression.py` (expect 35 unchanged), commit, push; remove the worktree.
7. Update memory (`rq4-c-reach-diagnostic.md`, `harness-plan-realization-plugin.md`, index).
8. One report to the user: the table, the control result, what is not established, links to the files.

## D. Never

- Never store a result only under /tmp. Never edit a running script. Never `pgrep -f` + `kill` with the
  pattern text present in my own command line. Never exceed 28 cores. Never delete before the archive is
  verified by content.
