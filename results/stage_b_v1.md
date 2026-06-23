# Boundary dataset v1 — Stage B (validity labels)

DUR=25s/boundary; shared LibAFL cache; each artifact classified by classify_artifact.py (--ignore-schema, exposed crate). Raw: `dataset/boundaries.jsonl`.

## 59 constructible boundaries labeled

| validity | n |
|---|---|
| valid | 33 |
| invalid | 13 |
| excluded | 13 |

## By boundary scope (public API vs exposed internal)

| scope | valid | invalid | review | excluded |
|---|---|---|---|---|
| public | 13 | 2 | 0 | 0 |
| internal | 20 | 11 | 0 | 13 |

## Summary labels

| label | n |
|---|---|
| NO_DIVERGENCE_OBSERVED | 33 |
| BUILD_FAIL | 11 |
| C_UB_CONFIRMED | 9 |
| C_CRASH | 4 |
| FUZZER_EXITED_EARLY | 2 |

> NO_DIVERGENCE_OBSERVED = no divergence found in DUR s — a **weak positive**, NOT proof of equivalence. Internal (exposed-static) boundaries are valid STU learning targets but are NOT public-API equivalence; public/internal are reported separately above. The label auditor (results/audit_v1.md) further verified each label; validity_v2 is the authoritative label.

