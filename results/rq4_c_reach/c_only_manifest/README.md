# C-guided C-only application reach (formal manifest)

This directory is the Git-sized projection of the completed formal run defined
by `docs/c_guided_c_only_plan.md`.  It is distinct from the earlier
same-corpus diagnostic in the parent directory: these corpora were grown by
fresh `C2R_MODE=c-only` campaigns, and only C coverage was measured.

The manifest completed 12/12 selected C realizations and 543/543
archived-built boundaries.  Every boundary received the frozen 3,600-second
budget; the recorded global maximum was 28 concurrent boundary fuzzers.  No
Rust campaign or Rust coverage measurement was performed.

## Contents

- `SUMMARY.{json,md}`: aggregate counts and the paper-facing table.
- `MANIFEST_DONE.json`: completion certificate, including the SHA-256 of
  `SUMMARY.json`.
- `units/<unit>/`: final result, completion record, run provenance, and merged
  C coverage identities when produced by the batched controller.  The three
  retained small-unit runs also include their compact build, campaign, and
  replay metadata.
- `FINALIZATION_NOTE_2026-09-13.md`: why the aggregate certificate was emitted
  after the subset recovery controller exited and the independent audit used
  before finalization.
- `ORPHAN_CLEANUP_2026-09-12.md`: the one interrupted-run cleanup event and its
  disposition before subsequent formal campaigns.

The full 158 MiB archive remains at
`/home/yunzez/c2rust_archive/cg_c_only_v3/`.  It additionally contains the
corpora, snapshots, batch records, and all 543 per-boundary LLVM coverage
exports.  These large rebuild/replay artifacts are intentionally excluded from
Git.  The formal `SUMMARY.json` SHA-256 is:

```
5a7b1ae1a8712f947d9154e82df8d99e8bcb2a32fee22336e2e9dceb6195cd8e
```

The authoritative completion check is:

```sh
python3 scripts/rq4/audit_c_only_manifest.py \
  --archive /home/yunzez/c2rust_archive/cg_c_only_v3 \
  --require-complete
```

It reports `12/12` units, `543/543` boundaries, and zero live supervisors.
