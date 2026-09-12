# Resource-realization plugin — lodepng defect pilot (2026-09-11 night, Opus 5 agent from a remote-control session)

Purpose: run the boundaries the reference plugin (`plugins/lodepng-harness-plan/plugin.toml`) unlocks through the
FULL RQ4 pipeline (plan → build → 3600 s campaign → replay → four-channel confirmation, sample 200), on the CROWN
translation and on the c2rust control. This is the "plugin-assisted" row of the extensibility experiment; it is
NOT part of the RQ4 main table and produces NO defect. Commit 915622f3 passes `--realization-plugins` through
`cell.py` / `confirm_cell.py`; the cell outputs (corpora, harness sources, candidates) stayed in the worktree's
`_pilot/` and are not archived — only `funnel.json`, `campaign_params.json`, `snapshots.json`, the confirmation
verdict counts and the post-campaign logs are kept here.

| cell | planned (auto) | planned (plugin) | built | executed | campaign | confirmation (sample 200) |
|---|---|---|---|---|---|---|
| lodepng × CROWN | 57 | +2 (`lodepng_inspect`, `lodepng_inspect_chunk`) | 2/2 | 2/2 | inspect: corpus 42, 0 candidates; inspect_chunk: corpus 24, 123 716 crashes | inspect_chunk: 192 `confirmed_termination` + 8 `ub_associated` |
| lodepng × c2rust (control) | 64 | +4 (`inspect`, `inspect_chunk`, `state_init`, `state_cleanup`) | 3/4 (`state_init` failed to build, E0428) | 3/3 | inspect 27 candidates, inspect_chunk 501, state_cleanup 22 | inspect 27 `not_reproducible`; inspect_chunk 190 `confirmed_termination` + 10 `ub_associated`; state_cleanup 22 `not_reproducible` |

**Reading.** The faithful c2rust control terminates exactly like CROWN on `lodepng_inspect_chunk`: every sampled
input has `pos ≥ 2^63`; C computes `in + pos` (out-of-object pointer arithmetic, undefined, reported by UBSan
`pointer-overflow` only when the address wraps — the 8/10 `ub_associated`), Rust panics at
`in_0.offset(pos as isize)` (its debug-assertion check on the same undefined operation). Per the project's
binding rule (Rust panic on top of dirty C = UB-associated termination, never a translation defect) these 190/192
are NOT defects and nothing enters the manifest; the classifier's `confirmed_termination` label here is the
documented blind spot of the definedness channel (non-wrapping out-of-bounds pointer arithmetic is invisible to
ASan/UBSan without a dereference), and the c2rust control is what catches it.

**Generic gaps exposed (future work, not fixed here):** (a) input model — `pos` is an offset into `in`, planned
as a free `usize`; an offset-into-buffer role (the sibling of `length_param`) would make the input C-defined by
construction; (b) the definedness channel cannot see non-wrapping out-of-bounds pointer arithmetic; (c) the
realized state's post-state is not compared (only the return value, `w`, `h`).
**Fixed on the branch:** a target that is the realization's own lifecycle function (`lodepng_state_init`) is now
refused at check 1 instead of failing the build with a duplicated extern (E0428).

Decision (user, 2026-09-12): stop here; no further plugin campaigns; the paper is not changed for this.
