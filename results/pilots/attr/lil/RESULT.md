# ATTR pilot — Part 2: lil (CROWN + Laertes), seed 42, commit dda70a4d

> **Scope note (2026-08-25).** This is a **reconstructed 313-record pilot** (12 reconstructed recoverable-UB
> inputs + the fn-15 record + 300 seeded scripts), **not** the archived 111,043-record campaign behind
> `rq1_master_table.md` fns 11/15; its 37 exclusions do not replace the archived "12 of 111,043". The
> **retraction of fn 15** (order dependence → lil.c version mismatch, stable `[21]` under all orderings) is
> recorded in place in `results/rq1_master_table.md` footnote 15; the new exclusion category is
> reference-version provenance, not reference nondeterminism.

Study question: are the three UB-attribution mechanisms — (b) in-loop UBSan gate, (c) isolated
ASan+UBSan oracle, (d) repeated C replay — a simple inclusion chain, or do they separate?

## Setup (what actually ran)
- Corpus: the fn-11/fn-15 111,043-record corpus (scratch `lil_crown_diff/`) is **not archived**.
  Reconstructed with `random.seed(42)`: 12 hand-written recoverable-UB `expr` records (shift
  out-of-range / INT_MIN negate / signed overflow), the fn-15 record `expr ((1+2)*(3+4))` (index 12),
  and 300 generated expr/var/list/string/func scripts → **313 records** (`raw/lil_corpus.{bin,tsv}`).
  Record format = the Laertes-cert `rundiff.rs` format (u16-LE length prefix, fresh `lil_new()` per record).
- C oracle `raw/rundiff_lil.c` built 3 ways (`none` / `gate` = UBSan minimal-runtime recover + flag shim,
  exactly the `--ub-free` flag set / `asan` = `-fsanitize=address,undefined`), and **twice**: from the
  Laertes benchmark `lil.c` (3518 lines) and from CROWN's `crown/c-code/lil/lil.c` (2962 lines) — see fix 3.
- Rust: `lil_crown` and `lil_laertes` rundiff bins (nightly-2023-01-26), from
  `tools/frameworks/c2saferrust/laertes_benchmarks/{lil_crown,lil_laertes}` copies.

## Table (313 records)
| config | valid records | divergences (CROWN vs C) | divergences (Laertes vs C) | exclusions | class | TTFD |
|---|---|---|---|---|---|---|
| (a) none | 313 | 0 (version-matched oracle) | 0 | 0 | — | none |
| (b) in-loop UBSan gate | 276 | 0 | 0 | 37 | C-UB 37 | none |
| (c) isolated ASan+UBSan | 276 | 0 | 0 | 37 (identical set to (b)) | C-UB 37 | none |
| (d) repeated C replay (alone / fwd / rev) | 313 | — | — | 0 | C-unstable 0 | none |

Classification totals: C-UB 37 · C-unstable 0 · Rust-failure 0 · semantic-difference 0 · abstention 0.
Reached functions: n/a (not instrumented). Isolated oracle wall time: 1.6 s for 313 fresh processes.

## (i) the recoverable-UB inputs
11 of the 12 reconstructed records trip UBSan (`raw/c_new_gate.out` shows `[UB-EXCLUDED]`; the ASan+UBSan
process prints `shift exponent 64 is too large` / `negation of -9223372036854775808` / `signed integer
overflow`). Record 7 (`expr 0 - -9223372036854775807 - 1 - 1`) was a wrong reconstruction — it does not
overflow — so it is counted as valid, not as a gate miss. 26 further generated records also trip shift UB.
(b) and (c) exclude the **same 37-record set** (`same_set_as_gate: true`); both CROWN and Laertes still
match C byte-for-byte on those 37 (as fn 11 said: "CROWN matches C even on those").

## (ii) the order-dependence record — expectation DID NOT HOLD
`expr ((1+2)*(3+4))` under the Laertes-version C: `[21]` run alone, `[21]` in forward batch, `[21]` in
reversed batch (313/313 records stable across the three orderings); sanitizer-clean under (b) and (c).
So (d) flags **nothing** here — the fn-15 "[25] first / [] after a shorter record" behaviour did not
reproduce with a NUL-terminated record buffer (also tried a non-NUL-terminated buffer: still `[21]`, and
ASan then reports a harness-level heap over-read in `ateol`, which is the likely mechanism of the old
observation: the earlier C driver read past its record buffer). What DID split on this exact record is
**lil.c version**: CROWN's older `lil.c` (ispunct-based `ee_muldiv`) returns `[]`, and CROWN faithfully
returns `[]`; the Laertes `lil.c` (`ee_invalidpunct`) returns `[21]`, and Laertes returns `[21]`.
Against the wrong-version oracle CROWN shows 1 "divergence" (record 12) that is not a bug.
Net: (b) ⊆ (c) held on this corpus, but (d) is orthogonal to both by construction — it just had nothing
to flag once the harness was fixed. The three mechanisms are separate instruments; the "C-unstable"
branch remains unexercised and needs a genuinely stateful reference to be demonstrated.

## What did not work → what was changed
1. clang 21's `libclang_rt.ubsan_minimal` already defines `__ubsan_handle_load_invalid_value_minimal` →
   duplicate-symbol link error with the `--ub-free` shim; fixed with `-fno-sanitize-link-runtime`
   (`gen_diff_harness.py` will hit the same when its shim is linked by `cc` on clang ≥21).
2. Laertes' `lil.rs` (c2rust run on macOS) implements `isspace/ispunct/isdigit` via
   `_DefaultRuneLocale.__runetype`; the E3-style zero-byte shim makes every char non-space → every record
   returned `[]` = **313 false divergences**. Fixed with `raw/rune_fill.rs` (populates the macOS ctype
   flags at start of `main`). The E3 crates `fuzz/lil_{laertes,c2rust,wip}_e3` carry the zero shim — fine
   for depth, wrong for any differential use.
3. Oracle version mismatch (above): the oracle must be the C the tool actually translated.

## Commands / raw
See `result.json` (`commands`), logs in `raw/` (`c_new_*.out`, `c_old_*.out`, `crown.out`, `laertes.out`,
`lil_isolated.json`, `lil_old_isolated.json`). Build tree: scratchpad `attr/`.
