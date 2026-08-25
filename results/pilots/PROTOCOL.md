# Pilot protocol (2026-08-25) — single seed, fix pipeline, THEN scale

Rule of this phase: one fixed seed, 10–15 min per pilot, no multi-seed yet, no paper edits.
A pilot answers only: does the configuration build, does the instrumentation separate the intended
configurations, does the recorded outcome answer the study question. It is NOT a variance claim.

Every pilot writes to its own directory under results/pilots/<name>/ and MUST record, in a
RESULT.md + a machine-readable result.json:
- buildable / harnessable (per configuration)
- reached functions (if instrumented; else "n/a")
- valid differential records (records where the C reference was admissible)
- divergence count per configuration
- time-to-first-divergence per configuration (wall-clock seconds; "none" if 0)
- classification counts: C-UB / C-unstable / Rust-failure / semantic-difference / abstention
- seed, git commit (`git rev-parse HEAD`), exact commands, raw log paths
- what did NOT work and what was changed to make it work (pipeline fixes are the point of a pilot)

Standing rules: real coverage-guided fuzzing when fuzzing is the mechanism (libFuzzer/cargo-fuzz);
replaying a saved corpus through several oracle PROJECTIONS is fine and is the intended design for
OBS. UB gate = C side under ASan+UBSan (subprocess or whole-batch), never skip it. NEVER print
OPENAI_API_KEY. Do not edit c2rust_paper/ or results/*.md outside your pilot directory.
Naming: OBS/ATTR/ALIGN axes; O-R/O-P/O-S/O-F channels; ATTR configs = none / in-loop UBSan gate /
isolated ASan+UBSan oracle (+ repeated C replay for C-unstable).
