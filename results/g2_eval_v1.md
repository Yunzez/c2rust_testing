# G2 — real-bug recall per strategy (Layer 3)

Injected mistranslation: Rust `scale` does `x*10` vs C `x*100` (differs on legal pct in [1,100]; reachable via the middle `safe_ratio`). For each strategy, do its selected boundaries surface the bug? `detected` = a divergence at >=1 selected boundary.

| strategy | #harness | covered | **bug detected (recall)** | boundaries |
|---|--:|--:|:--:|---|
| root | 1 | 3 | **YES** | `report`✓ |
| all-constructible | 3 | 3 | **YES** | `scale`✓, `safe_ratio`✓, `report`✓ |
| leaf-only | 1 | 1 | **YES** | `scale`✓ |
| frontier v1 | 0 | 0 | **NO** | — |
| frontier v2 | 1 | 2 | **YES** | `safe_ratio`✓ |

## Reading

- **frontier v2 keeps recall AND precision**: it detects the bug at `safe_ratio` (the middle boundary, legal inputs) — and on the clean Case D it had 0 false divergences. Cutting false positives did NOT cost a real bug.
- **frontier v1 (sink) MISSES the bug**: it collapses to no boundary, so recall = 0. Over-conservatism is not free — the guarded-rise (v2) is what preserves recall.
- leaf / all / root detect the bug too, but only alongside false divergences (Case D); the frontier detects it cleanly.
