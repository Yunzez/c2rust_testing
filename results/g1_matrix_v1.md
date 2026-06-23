# G1 Support Matrix v1 (2026-06-23)

Completes the near-term milestone: *given any fuzz artifact, reproducibly replay and emit an
evidence-backed conservative classification; then produce a structured G1 table over all currently
generatable ENTRYs.* Driver: `scripts/run_g1_matrix.py` (DUR=30 s/program, shared `CARGO_TARGET_DIR`
caches LibAFL); each artifact auto-classified by `classify_artifact.py`. Raw table: `g1_matrix.md`.

## Outcome (18 ENTRYs) — refreshed after the char** string-pointer table

| label | n | programs |
|---|---|---|
| NO_DIVERGENCE_OBSERVED (30 s) | 15 | bitutils, dynamic_array, glob_match, **graph_dfs**, hash_table, intmath, kv_config, leb128, linked_list, **matrix_reduce**, mergesort_search, rle_codec, state_machine, tiny_vm, **word_tokens** |
| C_UB_CONFIRMED | 2 | rpn_eval, opcode_dispatch |
| UNSUPPORTED_SIGNATURE | 1 | array_map_reduce (callback / fn-ptr) |

**17 generatable ENTRYs: 15 NO_DIVERGENCE_OBSERVED under 30-second fuzzing + 2 real divergences.
0 *observed* harness false positives.** All three nested-pointer shapes are now supported:
`graph_dfs` (ptr-to-array, bounded `n`), `matrix_reduce` (`int**` rectangular table), and
`word_tokens` (`char**` string-pointer table over independent NUL-terminated backings). The single
remaining unsupported ENTRY is the callback / fn-ptr program — the next P3 step.

> NOTE: `NO_DIVERGENCE_OBSERVED` is exactly that — 30 s of fuzzing found nothing; it is **not**
> evidence of semantic equivalence. The matrix also records per-run telemetry (elapsed, exit code,
> terminated_by_timeout, executions); a fuzzer that dies early with no artifact is
> `FUZZER_EXITED_EARLY`, never counted as no-divergence.

## What the two divergences are

Both `rpn_eval` and `opcode_dispatch` auto-classified **C_UB_CONFIRMED**: the C entry, run alone
under UBSan, reports `signed integer overflow`; the Rust translation panics on the same input
(overflow checks). Same root cause as the first finding — c2rust copies `a * b` / `a + b` verbatim,
so C's silent signed-overflow UB becomes a Rust overflow panic. These are genuine behavioral
divergences in the **C-UB class**, not harness artifacts and not (yet) confirmed translation bugs.

## Why this matrix shape matters (vs CLEAN/DIVERGENCE)

- **UNSUPPORTED_SIGNATURE is tracked separately** — entries out of the generator's current coverage,
  NOT "clean" and NOT selector failures. All three nested-pointer shapes are now supported
  (ptr-to-array=graph_dfs, `int**`=matrix_reduce, `char**`=word_tokens); the 1 remaining is the
  callback program. This keeps `generator_supported` distinct from `runnable_now` and from
  `theoretically_valid` (the selector treats nested pointers as a soft cost; the generator currently
  rejects only the callback/fn-ptr boundary — the gap is explicit in the data).
- Every divergence carries an evidence-backed label (`results/classified/<prog>.json`): toolchain,
  flags, artifact sha256, and the independent C-UBSan / Rust-only / diff-replay outcomes.

## Bugs this surfaced and fixed (P2 doing its job)

- `gen_diff_harness.py`: C param names that are Rust keywords (`leb128`'s `in`) produced invalid
  Rust → now sanitized (`in` → `in_`).
- `classify_artifact.py`: the Rust-only driver inherited the shared `CARGO_TARGET_DIR` and built
  its binary elsewhere → now isolates its own target dir; replay also searches the shared dir.

## Next (per project-lead review)

- **P3 generator correctness:** explicit param **schema/annotation** instead of adjacency
  inference; persist a harness schema per benchmark; separate input-buf / output-buf / capacity /
  logical-output-length; generator unit tests + replay fixtures. (Adjacency pairing happens to be
  correct on this corpus but is not robust.)
- Extend generator coverage: ptr-to-array (graph_dfs), `int**` rectangular table (matrix_reduce),
  and `char**` string-pointer table (word_tokens) all done; only callback binding remains, to close
  the last UNSUPPORTED_SIGNATURE.
- **P4:** G3 semantics-preserving refactors (reliable "structure-changed, semantics-preserved"
  labels) → then G2 injected bugs → only then a learned `P(valid|x_f)` with per-program grouped CV.
