# C2SaferRust differential harnesses (value-oriented bug hunt)

See results/rq1_c2saferrust_round.md for the round findings.

## tulipindicators/ (generic table-dispatch, base-c2rust vs C2SaferRust-WIP)
Method: compare C2SaferRust `_WIP` output against the SAME-SOURCE faithful c2rust `base` (both in
laertes_benchmarks/). Base = faithful (= C behavior); divergence localizes to C2SaferRust's rewrite.
- base_oracle_main.rs : oracle bin over the base (faithful) crate, built with -Zsanitizer=address
  (memory-UB gate). Reads stdin, dispatches via ti_indicators[] fn-pointer table, serializes outputs.
- wip_fuzz_target.rs  : cargo-fuzz target over the C2SaferRust `_WIP` crate; native call + subprocess
  oracle + determinism gate. Rust runs debug-assertions so overflow/OOB panic surfaces.
- ti_oracle.c         : (initial C oracle — DEPRECATED: upstream C was a DIFFERENT source version,
  produced a false `dx` divergence. Kept as the cautionary example; use base_oracle instead.)
Result: 17533 execs, 0 divergences (tulipindicators was NOT rewritten by C2SaferRust -> null target).
Depends on the C2SaferRust crates under tools/frameworks/c2saferrust/laertes_benchmarks (gitignored).
