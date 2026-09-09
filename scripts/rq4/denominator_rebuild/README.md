# Denominator step 2 (2026-09-09) — scripts as run

- `build_all.sh` — rebuilds the rlib universe for the 11 cells that had a bin-route or no independent
  denominator (reference call per cell inside); each build < 5 s. Writes `$RQ4_WORK/denom_<lib>_<tool>/`.
- `compare.py` — compares each rebuilt universe with the archived one (bin-route file: basename identity +
  every region; tests-build universe: (module file, line) identities through the pair's linemap + region
  count). `compare.json` is the result: all SAME, the 8 bin-route cells up to `denom::main`.
- `verify_link_dead_code_and_empty_profile*.sh` — the qsort × Laertes control: rlib with and without
  `-C link-dead-code` identical (83 fn / 463 reg); `llvm-cov export --empty-profile` fails on the rlib
  archive but works on the unpacked `.rcgu.o` and equals the current zeroing route.
Outcome recorded in `docs/rq4_denominator_decision_2026-09-08.md` ("Step 2 executed").
