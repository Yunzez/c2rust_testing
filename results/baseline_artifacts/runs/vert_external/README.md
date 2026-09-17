# VERT external frozen-pair experiment

All 36 frozen defect pairs were submitted through the same external adapter. 
The adapter packages audited C/Rust payloads but leaves VERT's released 
two-element array model, PARAM1-only reference injection, and return-only 
comparison unchanged.

| Outcome | Count |
|---|---:|
| Detected | 0 |
| Completed miss | 1 |
| Analysis failure | 21 |
| Compile failure | 10 |
| Unsupported input | 4 |

VERT emitted apparent counterexamples for eight pairs, including S14. 
None is a valid detection: the C/rWasm side receives only `PARAM1`, 
whereas the candidate receives all dynamic parameters. The untouched 
correct S14 control fails identically.

C13 is the only valid completed comparison. Its one-scalar wrapper fits 
VERT's native input envelope, but the release-mode comparison completes 
without revealing the frozen `free(NULL)` translation defect.

Raw generated projects and logs are under 
`/home/yunzez/c2rust_baselines/runs/vert/external_pairs/`; each small 
per-defect result is stored in this directory.
