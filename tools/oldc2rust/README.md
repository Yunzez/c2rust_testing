# oldc2rust — reproducible old c2rust (for CROWN compatibility)

Tracked copies of the reproducibility scripts. The runnable copies + docker images + outputs live under
`tools/frameworks/crown/oldc2rust/` (gitignored: 1.6GB external checkout). See the full write-up in
`results/crown_crustbench_investigation.md`.

Why: CROWN (arXiv:2303.10515) is a rustc-driver safety-lifter tightly coupled to ~2022-era c2rust output
(`libc::`-form, nightly-2022-08-08). Our corpus c2rust is 0.22.1 (`::core::ffi::`) which CROWN rejects.
Host is Ubuntu 26.04 → no apt llvm-15, so old c2rust is built in docker.

- `Dockerfile` / `Dockerfile017` — build c2rust v0.18.0 / v0.17.0 in ubuntu:22.04 (llvm-15,
  nightly-2022-08-08, from git tag). Images `oldc2rust:0.18` / `oldc2rust:0.17`.
- `sweep_crustbench.sh` — project-mode transpile all CRUST-bench repos with old c2rust → CROWN-input form.
- `pilot.sh <N>` — end-to-end: old-c2rust transpile (CROWN's exact flags incl. `--reduce-type-annotations`)
  → fixup (edition 2018 + feature gates + extern crate core/libc) → CROWN preprocess/explicit-addr/rewrite
  → `cargo +nightly-2023-01-26 check`.

Result: CROWN runs on CRUST-bench but compiles cleanly on only 12/87 (version-coupling: 0.17/0.18 over-lift
one pointer vs CROWN's exact unshipped c2rust commit; 30 more land at 1-5 residual errors). Verdict in the
investigation doc: capped at 12, CROWN is a ~0-bug control anyway, don't chase the exact commit.
