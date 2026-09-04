# OBS detection matrix — 6 defects, fixed corpus each, single seed 42 (2026-08-25/26)

Generality here comes from **defect diversity**, not from repeating qsort seeds. Every cell: one corpus
(libFuzzer ≤5 min, fresh corpus dir, unless noted), C under ASan+UBSan, one execution per input projected
onto O-R / O-P / O-S / O-F, two drivers (silent-consumer / printing). Numbers are divergences out of
valid records. **Single-seed pilots — not rates.** Per-cell RESULT.md carries commands, logs, TTFD.

| cell (defect) | valid | O-R | O-P silent | O-P print | O-S | O-F | class | expected |
|---|---|---|---|---|---|---|---|---|
| qsort × PtrTrans (in-place array, void return) — `../obs_qsort_ptrtrans` | 104 | 0 | 0 | 71 | 71 | 71 | semantic 71 | holds |
| bzip2 × Laertes (BZ_OK, zero CRC table in output stream, fn 14) | 529 | 0 | 0 | 528 | 528 | 528 | semantic 528 | holds |
| crc32_z × C2SaferRust (empty-chunk reset, return-visible, S1) | 65 | 15 | 0 | 15 | 15 | 15 | semantic 15 | holds |
| cJSON × PtrTrans parse_string (fn 5) — whole cell | 139 | 31 | 0 | 111 | 113 | 113 | semantic 113 | see split |
| ↳ class (b) valuestring lost, return equal | 80 | **0** | 0 | 80 | 80 | 80 | | holds |
| ↳ class (a) return flips (non-UTF-8 / `\u`) | 31 | 31 | 0 | 31 | 31 | 31 | | — |
| ↳ class (c) failure-path `offset` only | 2 | 0 | 0 | 0 | 2 | 2 | | O-S-only |
| qsort × C2SaferRust (usize sentinel, stack overflow, C1) | 113 | 51 | 51 | 51 | 51 | 51 | **Rust-failure 51**, semantic 0 | holds |
| tulip × C2SaferRust argc-off-by-one (fn 30), CLI boundary † | 201 | 48 | 48 | 78 | 48 | 78 | C-UB 1 excl · Rust-failure 1 · semantic 77 | partly |

† tulip corpus is a **seeded generator (random.seed 42), not coverage-guided** — libFuzzer is not the
natural mechanism at a CLI boundary. Label it a proxy. At a CLI boundary O-S has no designated output
memory, so O-S = O-R by construction.

## What the matrix shows
1. **Return-only is structurally blind to state-mutation defects**: qsort (0/71), bzip2 (0/528), cJSON
   valuestring (0/80). Where the defect reaches the return value (crc32 15/15, cJSON class (a) 31/31,
   tulip exit code 47/77) O-R sees it — the axis is about *where the contract is expressed*, not about
   "weak vs strong" oracles.
2. **Process-output is driver-dependent in every cell**: O-P-silent is 0 in all five non-crash cells; with
   a printing driver it equals O-S wherever the printed state covers the designated state (qsort, bzip2,
   crc32, cJSON (a)/(b)), and it misses O-S-only state that is not printed (cJSON class (c) offset: 0/2).
   Conversely at the CLI boundary O-P-print sees 30 records O-S cannot (tulip 78 vs 48: usage text and a
   separate untriaged `sample.rs` display divergence), because there the *only* state is the output.
3. **cJSON refinement**: O-R is not blind to the *cell* (31/113) — it reports the UTF-8 return flip and
   would attribute the cell to the wrong root cause while missing the 80-record value loss entirely.
   Root-cause attribution, not just detection count, differs by channel.
4. **Rust-failure is channel-independent**: qsort × C2SaferRust crashes are seen 51/51 by every channel
   in both drivers (a crash is not an observation-boundary question).
5. **UB gate still matters inside OBS**: tulip `hma 1 4` is a real ASan global-buffer-overflow in the
   *original* C `sample.c` (period 1 → negative start index); without the gate it would have been the
   79th "semantic difference".

## Caveats (carry into the paper as-is)
- TTFD from corpus replay is per-input cost, not fuzzing time-to-bug; all first hits are at record 0–6.
- cJSON class (c) (`offset` bookkeeping on the failure path) is new and untriaged; not in the archived
  README; do not count it as a confirmed defect until triaged.
- tulip stdout-only/both-exit-0 (9 records) is a separate untriaged WIP display divergence, not the argc bug.
- The OBS-matrix agent was terminated by a session limit after writing all five cell directories; this
  summary was assembled from the cells' result.json/RESULT.md by the orchestrator.
