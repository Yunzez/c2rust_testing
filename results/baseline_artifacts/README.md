# Released baseline artifacts

This directory records an artifact-level comparison against RustAssure,
FLOURINE, and VERT. The experiment protocol is
`docs/baseline_artifact_evaluation_plan.md`.

Large downloaded artifacts and build trees live outside Git under
`/home/yunzez/c2rust_baselines/`. This directory retains their official URLs,
checksums, input adapters, commands, logs, gate outcomes, and generated paper
tables. No baseline implementation is vendored or patched here.

`adapter_policy.json` is the scored-adapter audit. It distinguishes input-contract
realization and source packaging from interface changes that work around a
released baseline failure. The latter may remain under an adapter's `history/`
directory, but never contributes a detection.

`scoring_decisions/` separates released-artifact signals from scored outcomes.
Inspectors only extract graph distances, counterexamples, crashes, and gate
status. A recorder may mark `detected` only when a committed decision explicitly
links that signal to the catalogued root cause on a contract-valid C execution.
