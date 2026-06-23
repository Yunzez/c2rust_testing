# Boundary dataset v2 — audited labels — 2026-06-23

v2 scales the corpus (18 → 40 programs; 40/40 c2rust-clean) and re-harvests with generator `v0.4`.
Labels were assigned in two passes and **`validity_v2` is the authoritative label** (the raw Stage-B
label in `boundaries_v2.jsonl.label` is NOT — it is "no divergence in DUR s", a weak positive).

## Two-pass audit

1. **Semi-automated heuristics** (`scripts/audit_heuristics.py`) auto-classify the confident
   majority and FLAG the uncertain. Calibration vs the v1 4-agent ground truth: the heuristic
   **catches 8/8 labels the v1 independent audit overturned (5/5 weak_exclude, 3/3
   excluded_generator)**; overall flag-vs-auto agreement is **36/46**, and it **intentionally
   over-flags some valid cases** (conservative — it never auto-accepts a label the agents rejected).
2. **Independent spot-check** of the 35 flagged by a 3-agent panel (adversarial, judging from
   source + harness + evidence, blind to the heuristic guess) → final `validity_v2`.

Per row: `validity_v2`, `audit_status` ∈ {auto (72), reviewed (35), n/a (20)}, `review_reason`,
plus the `generator` capability stamp.

## Final distribution (127 boundaries) — v2.1 (telemetry-corrected)

| validity_v2 | n |
|---|---|
| valid | 66 |
| invalid_isolation_invariant | 19 |
| invalid_intrinsic_ub | 15 |
| weak_exclude | 7 |
| excluded_generator | 8 |
| excluded (build/gen return-type gaps) | 12 |

**Binary training set: 66 valid : 34 invalid** (drop 27: 7 weak + 8 generator-artifact + 12 excluded).
vs v1's 28:10.

### v2.1 telemetry correction (the 8 FUZZER_EXITED_EARLY)

The v2 raw harvest mislabeled 8 boundaries `FUZZER_EXITED_EARLY` (→ excluded). All 8 were in fact
real **AddressSanitizer aborts** (heap/stack-overflow, use-after-free): an ASan SIGABRT bypasses
libFuzzer's artifact writer, so the crash exited early with no saved artifact. Fix: `harvest_stage_b`
now reads the sanitizer SUMMARY from the run log and labels such cases `C_CRASH_NO_ARTIFACT`
(→ invalid) instead of dropping them. Re-run + categorized:
- **3 → invalid_isolation_invariant** (real boundary crashes): `unsafe_decode::char_at` (trusts
  `pos<len`), `unsafe_decode::sum_declared` (trusts embedded length `data[0]`),
  `sorted_insert::sorted_insert` (trusts `len<cap`).
- **5 → excluded_generator** (harness mis-models, each a new generator gap): `graph_bfs`/`count_edges`
  (an **n×n matrix** modeled as a flat `len=n` buffer), `prefix_sum_excl` (an **output array** modeled
  as a single out-scalar), `free_matrix` (`int**` mis-read as a string-table + a **destructor** that
  frees harness memory), `copy_span` (a **pointer-pair span** modeled as two independent strings).

So negatives 31 → 34, and the generator backlog gains: n×n-matrix buffers, output-array sizing,
pointer-pair spans, destructor boundaries (alongside the known sliced-buffer gap).

## What the spot-check changed (of 35 reviewed)

- 23 valid → **valid** (confirmed real, exercised)
- 7 valid → **weak_exclude** — uninformative positives: `match_at`/`match_class` (internal cursor
  params fed arbitrary `usize` → trivial early-return), `read_le32`/`read_i32`/`read_u16`/
  `nibble_to_hex` (fixed-size decoders / table lookup → agreement by construction), `kv_config::is_space`.
- 3 invalid → **excluded_generator** — sliced-buffer mis-models (`merge_runs`, `msort_range`,
  `reverse_range`): a `T* a, … size_t lo/mid/hi` array built as a single element with unconstrained
  index params → SEGV a correct harness would not hit. (Same family flagged as a real generator gap;
  see `results/struct_array_v1.md` "Next" — needs an explicit sliced-buffer schema.)
- 1 invalid → **invalid_intrinsic_ub** (`postfix_run`: overflows its own fixed `stack[16]`, UBSan-confirmed)
- 1 invalid → **invalid_isolation_invariant** (`vm_step`: trusts `sp ≤ STACK_MAX`, harness fuzzes it → OOB)

## Negative-class diversity (the v1 risk — resolved)

The 31 negatives are **balanced across both mechanisms** (16 isolation-invariant + 15 intrinsic-UB)
and span **15 distinct programs** (v1: ~3, almost all one VM family):

- **intrinsic-UB** mechanisms: signed sub/negate/abs/shift/div-mod/reduction overflow + a self
  stack-overflow (`div_mod`, `negate_abs`, `shift_ops`, `sub_overflow`, `reduce_overflow`,
  `safe_stats`, `rpn_eval`, `postfix_machine`).
- **isolation-invariant** structures (non-VM): `bounded_queue`, `bounded_stack`, `gap_buffer`,
  `histogram`, `ring_buffer`, plus the VM family (`opcode_dispatch`, `tiny_vm`).

`opcode_dispatch` is the largest single source (7) but ~23% of negatives, not ~70% as in v1.

## Caveats (honest)

- **`weak_exclude` is a judgment call** on near-identical functions: part-1 kept `byte_classify::is_space`
  *valid* while part-2 marked `kv_config::is_space` *weak_exclude*. Kept each panelist's independent
  call rather than overriding; a borderline a future pass may normalize.
- The 8 FUZZER_EXITED_EARLY were diagnosed and corrected in v2.1 (above); the remaining 12 `excluded`
  are BUILD_FAIL return-type gaps (enum/struct/pointer returns), a known generator limitation.
- Negatives still lean on the authored corpus; external c2rust-clean libs (next) add external validity.
