# C-only manifest finalization (2026-09-13)

The formal campaigns completed at 2026-09-13 10:33:38 UTC.  All twelve
`FINAL` directories were present and the independent checker reconstructed
the frozen manifest as 12/12 units and 543/543 archived-built boundaries.

The recovery controller was intentionally invoked with a six-unit `--units`
subset.  `run_c_only_manifest.py` only writes a full-manifest certificate when
it is invoked without that option, so it exited successfully after updating
`c_only_manifest_queue.json` but did not write the three aggregate files.  No
campaign or coverage measurement was rerun.  At 2026-09-13 10:36 UTC, after
the independent checker had verified all twelve `FINAL` records, the same
frozen controller's built-in aggregator was invoked as follows:

```python
from pathlib import Path
import scripts.rq4.run_c_only_manifest as m
m.write_manifest_summary(
    Path('/home/yunzez/c2rust_archive/cg_c_only_v3'), m.UNITS)
```

It atomically produced `SUMMARY.json`, `SUMMARY.md`, and finally
`MANIFEST_DONE.json`.  The independent completion command then passed:

```text
$ python3 scripts/rq4/audit_c_only_manifest.py --require-complete
audited=12/12 boundaries=543/543 live_supervisors=0
```

`SUMMARY.json` SHA-256:

```text
5a7b1ae1a8712f947d9154e82df8d99e8bcb2a32fee22336e2e9dceb6195cd8e
```

Additional completion checks found 543 C coverage exports, no Rust coverage
artifacts, a 3,600-second campaign budget in every unit, no live boundary
fuzzer or orphan, and a recorded maximum of 28 concurrent boundary fuzzers.

One earlier batch exposed a teardown bug in the driver: killing only a timed
out libFuzzer supervisor can leave a fork worker orphaned.  That concrete
orphan was removed and documented in `ORPHAN_CLEANUP_2026-09-12.md` before the
next campaign.  After all formal runs completed, the driver was changed to
use a private process group per supervisor and a regression test was added.
This post-run code change does not alter any archived harness, corpus, replay,
coverage identity, or result.
