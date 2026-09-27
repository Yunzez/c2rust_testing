# G3 evaluation — false divergences per boundary-selection strategy (Layer 3)

Controlled semantics-preserving cases (any divergence is FALSE by construction). Each strategy's selected boundaries are differentially fuzzed; **false-div** = boundaries that produced a (false) divergence. Lower is better; the STU frontier should be 0 at good coverage. Dynamic is the ORACLE here, not the selector.

## g3_case_a  (2 funcs)

| strategy | #harness | covered | **false-div** | boundaries |
|---|--:|--:|--:|---|
| root | 1 | 2 | **0** | `scale_pct` |
| all-constructible | 2 | 2 | **1** | `scale`⚠, `scale_pct` |
| leaf-only | 1 | 1 | **1** | `scale`⚠ |
| frontier v1 | 0 | 0 | **0** | — |
| frontier v2 | 1 | 2 | **0** | `scale_pct` |

## g3_case_c  (2 funcs)

| strategy | #harness | covered | **false-div** | boundaries |
|---|--:|--:|--:|---|
| root | 1 | 2 | **0** | `ring_get` |
| all-constructible | 2 | 2 | **1** | `ring_at`⚠, `ring_get` |
| leaf-only | 1 | 1 | **1** | `ring_at`⚠ |
| frontier v1 | 0 | 0 | **0** | — |
| frontier v2 | 0 | 0 | **0** | — |

## g3_three_level  (3 funcs)

| strategy | #harness | covered | **false-div** | boundaries |
|---|--:|--:|--:|---|
| root | 1 | 3 | **1** | `report`⚠ |
| all-constructible | 3 | 3 | **2** | `scale`⚠, `safe_ratio`, `report`⚠ |
| leaf-only | 1 | 1 | **1** | `scale`⚠ |
| frontier v1 | 0 | 0 | **0** | — |
| frontier v2 | 1 | 2 | **0** | `safe_ratio` |

## Reading

- **g3_case_a**: leaf false-div 1, all-constructible 1; frontier v1 0 (covers 0), v2 0 (covers 2).
- **g3_case_c**: leaf false-div 1, all-constructible 1; frontier v1 0 (covers 0), v2 0 (covers 0).
- **g3_three_level**: leaf false-div 1, all-constructible 2; frontier v1 0 (covers 0), v2 0 (covers 2).
