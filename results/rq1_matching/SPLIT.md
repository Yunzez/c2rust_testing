# RQ1 development / evaluation split — FROZEN 2026-09-01

**Binding.** No matcher weight, threshold, signal, or regression gate may be tuned against any
artifact listed under *evaluation*. Any number measured on a development artifact is a fitted
number and must be labelled as such wherever it is reported.

The split is **library-disjoint**: a library is assigned as a whole, so every artifact derived
from it — across all translators, plus our own raw-LLM control — falls on the same side. This
prevents a weight tuned on one translation of a library from being validated on a sibling
translation of the same source.

## Development libraries

| library | why it is development |
|---|---|
| **cJSON** | `signal-C` weight (0.20) and the input-element-scalar weight (0.12) were tuned to raise raw-LLM cJSON from 0.375 → 0.475 → 0.550. The weights *are* fitted to this library. |
| **lil** | The sole fixture of the matcher regression gate (`scripts/matcher_regression.sh`, `FIXTURES=("lil source/lil.c translated 126 128")`), and the constraint that held signal-C at 0.20 instead of 0.35 (at 0.35, lil regressed 0.984 → 0.969). Weight selection was decided *by* lil. |
| **`benchmark/pairs` microbenchmarks** (56 programs) | Fixtures of `results/rq1_matching/cells/regression/harness.py`, which re-scores them on every matcher change. Includes all 10 programs of the raw-LLM micro corpus: base64, bignum, hash_table, hex_encode, leb128, linked_list, opcode_dispatch, rle_codec, rpn_eval, tinyexpr. |

## Evaluation libraries (held out)

qsort · genann · urlparser · quadtree · lodepng · bzip2 · tulipindicators · optipng

## Resulting artifact assignment

| group | development | evaluation |
|---|---:|---:|
| A. name-preserving artifacts (24 total, 8 libraries) | **5** — lil × {c2rust, Laertes, C2SaferRust, CROWN}; cJSON × c2rust | **19** — genann ×4, qsort ×3, urlparser ×4, lodepng ×2, quadtree ×2, bzip2 ×4 |
| B. real PtrTrans artifacts (5 total) | **1** — cJSON × PtrTrans | **4** — qsort, quadtree, bzip2, lodepng |
| raw-LLM control (7 libraries) | **2** — lil, cJSON | **5** — qsort, genann, urlparser, quadtree, bzip2 |
| micro-corpus ablation (10 programs) | **10 — all of it** | 0 |

## Consequences that must be carried into the write-up

1. **The matcher ablation ladder (signature-only → +metrics/operators → +topology →
   +abstention) is entirely a development-set result.** Its corpus is the 10 micro programs,
   all of which are regression fixtures. It may be reported as a controlled component study;
   it may not be reported as held-out evidence.
2. **The `lil` topology stress numbers (0.359 → 0.742 → 0.984) are development numbers.** They
   remain the clearest demonstration that call-graph topology cracks a homogeneous cluster,
   but the artifact that demonstrates it is the one that selected the weights.
3. **cJSON's 0.550 raw-LLM recall is a fitted number**, being the target the signal weights
   were raised against. It must never be quoted as an independent result.
4. **Group B's evaluation set is 4 artifacts: qsort, quadtree, bzip2, lodepng × PtrTrans.**
   These are the only held-out real-translator renaming evidence that exists. qsort is already
   labelled (3 correspondences); the other three are not labelled at all.
5. Held-out group A is **19 artifacts across 6 libraries**, not 24 across 8.

## Amendment rule

This file changes only by an explicit, logged decision recorded here with its date and reason.
Moving a library from evaluation to development after seeing its result is prohibited; moving
one from development to evaluation requires showing that no gate, weight, or threshold was
ever selected using it.
