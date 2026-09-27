# Graph-identity regression audit

Retrospective diagnostic results, separate from frozen `results/rq1_matching`.
See [implementation and interpretation](../../docs/matcher_graph_identity_2026-09-17.md).

`summary.json` contains all 40 per-artifact comparisons and both aggregate bounds.
`provenance.json` records binaries, matcher, source and interpretation.
`evidence.tar.gz` contains old/new extracted features, pair-level comparisons,
the corrected analyzer source/tests, and the re-extraction/recheck script.
The two unadjudicated same-name predictions must not be treated as confirmed
correct matches or confirmed false positives.
