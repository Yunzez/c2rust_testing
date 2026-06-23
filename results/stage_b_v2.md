# Boundary dataset v2 — Stage B (RAW harvest labels)

DUR=20s/boundary; shared LibAFL cache; each artifact classified by classify_artifact.py (--ignore-schema, exposed crate). Raw dataset: `dataset/boundaries_v2.jsonl` (generator 0.4).

> These are RAW harvest labels. The AUDITED label is `validity_v2` (see `scripts/audit_heuristics.py` + the audit report); do not train on these directly.

## 127 constructible boundaries labeled

| validity | n |
|---|---|
| valid | 73 |
| invalid | 34 |
| excluded | 20 |

## By boundary scope (public API vs exposed internal)

| scope | valid | invalid | review | excluded |
|---|---|---|---|---|
| public | 36 | 23 | 0 | 7 |
| internal | 37 | 11 | 0 | 13 |

## Summary labels

| label | n |
|---|---|
| NO_DIVERGENCE_OBSERVED | 73 |
| C_UB_CONFIRMED | 30 |
| BUILD_FAIL | 12 |
| FUZZER_EXITED_EARLY | 8 |
| C_CRASH | 4 |

> NO_DIVERGENCE_OBSERVED = no divergence in DUR s — a **weak positive**, not proof of equivalence.

