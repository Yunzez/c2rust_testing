# Weaker-model (gpt-4o-mini) semantic diffs — RQ1 tier-2 (scale evidence)

gpt-4o-mini asked to translate the same value C functions to idiomatic Rust (no repair). Differential
vs the original C (exhaustive). See results/rq4_effectiveness/semantic_diffs.md #4-#5.

- `fixmul.txt`  → fixmul CLEAN (2^32 exhaustive); fixscale **BUG** (i16>>shift masks shift>=16; C
  promotes to 32-bit int → 917,506 diffs over shift 0..31 x all i16).
- `leb128.txt`  → **BUG**: decoded value passed BY VALUE to helper (lost → always encodes 0) + encode
  uses `while value!=0` instead of do-while (drops the 0 case). Thousands of exhaustive diffs.
- `percent.txt` → CLEAN (98k inputs, 0 diffs — faithful).

Contrast: frontier gpt-5.1 translated fixmul/fixscale/leb128 faithfully (fixscale clamped, leb128
exhaustively clean). The weaker model did not → translator-quality spectrum.
