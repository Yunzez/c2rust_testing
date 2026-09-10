# The deterministic grid seed rule — frozen 2026-09-09

**Rule (frozen before the Laertes / C2SaferRust / CROWN runs; never adjusted on results):** for every planned boundary whose
InputPlan contains a fuzz-filled array parameter named `options`, emit six initial seeds; seed *k* is a deterministic
pseudorandom byte string of the harness's full decoded length (PRNG key `7:<boundary>:<k>`) in which **every element of
`options` is overwritten by the k-th value of {1, 2, 3, 5, 10, 20}** (IEEE double, little-endian, at the offset the plan
resolves). Nothing else is set: `size` and the input rows keep their pseudorandom bytes, so a seed's `size` is whatever
the plan's bounded decoder makes of four random bytes (uniform 0..1024).

**What the InputPlan supplies and what it does not.** The plan supplies the byte layout (scalars first, then
`buffer_table` / `input_array` items in declaration order), each field's C type and width, the extent of `options`
(1, 2 or 3 elements, `proven_extent_in_boundary`) and therefore the exact offsets — that is what makes the seed decode
to the intended values without touching the harness. The plan does **not** supply the value set: {1,2,3,5,10,20} is a
fixed small-positive-integer grid chosen once, on the observation that tulip's option guards are `period < 1` /
`period < 2`, non-decreasing period orderings (equal values pass) and `alpha ∈ [0,1]` (1 passes). It is not derived
from per-boundary guard analysis; a rule that varies values by option index (which would satisfy `ti_psar`'s
`accel_max > accel_step`) was deliberately NOT added after seeing the c2rust result.

**Identity across translations.** The materialiser was run on each cell's own archived `plans.json`; all four produce the
same 121 seeded boundaries (92 skipped: no fuzz-filled `options`) and byte-identical seeds — 121 × 6 grid seeds, lengths
8–98316 bytes, 16 boundaries longer than `-max_len 65536`; joint sha256 of the
per-boundary seed hashes `9b83644ae75d2713` (identical for c2rust, Laertes, C2SaferRust, CROWN; each cell's `seeds/manifest.json`).
Generation: `python3 scripts/rq4/materialize_seeds.py --plans results/rq3_coverage/tulip/<tool>/plans.json --out DIR`.

**Status in the study.** Seed refinement is an ablation on top of the automatic baseline (34.4 % on c2rust), not part of
the method's headline numbers; the shipped smoke suite is a reach reference only. No LLM is involved anywhere in it.
