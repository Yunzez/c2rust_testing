# Orphan cleanup after `lil_laertes` batch 01

- Observed: 2026-09-12 18:06 UTC, after the batch's 3,600-second campaign
  and 240-second supervisor grace period.
- Boundary: `lil_parse`.
- Cause: `campaign_guided()` killed the still-running `-fork=1` supervisor at
  its documented deadline, but the supervisor's `sh` child and libFuzzer
  worker remained alive after being reparented to PID 1.
- Evidence state: batch replay, 23 C exports, aggregate `lil_laertes/FINAL`,
  and the independent 51-export identity audit had completed successfully
  before cleanup.  The lingering worker therefore did not contribute to the
  archived corpus or coverage result.
- Action: after verifying their full command lines, the worker PID 3341715
  received `SIGTERM`; its waiting shell PID 3341714 then exited.  No signal
  was sent to the manifest controller, the next cell driver, or any result
  process.
- Resource invariant: the next lodepng batch was still building and had zero
  active fuzz supervisors.  Cleanup occurred before that batch began fuzzing,
  so the global limit of 28 concurrently fuzzed boundaries was preserved.
- Follow-up: the running driver remains frozen.  A read-only orphan detector
  watches later batch transitions; process-tree cleanup should be fixed and
  regression-tested only after the manifest run completes.
