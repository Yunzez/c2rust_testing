# cJSON × PtrTrans (FSE'26, gpt-5.1): silent semantic-difference cluster in `parse_string`

**Second confirmed semantic-difference finding** (after C2SaferRust crc32). Target: cJSON translated by
**PtrTrans** (FSE'26, pointer-analysis-guided LLM translation, backend **gpt-5.1**, `Trans_PA` mode =
their full method). The translation **compiles** (cargo check green) and the pipeline committed
`parse_string`, `utf16_literal_to_utf8`, `parse_hex4`, `parse_number` as **successfully translated**
(they were NOT among the 24 stub-reverted groups). Differential vs the original C found
**40,133 divergences / 120,050 UB-free records — all in `parse_string`**.

## The three root causes (all in tool-declared-SUCCESS code)

### 1. `\u` escapes always rejected — pointer-distance lost in ptr→slice lift (26,657 diffs' main driver)
C passes `input_end` (a bound pointer) to `utf16_literal_to_utf8`; the check is
`(input_end - first_sequence) < 6`. The Rust lift models both as slices — but the **call site** in
`parse_string` passes an **empty slice** as `input_end`:
```rust
let end_slice = &content[input_end_index..input_end_index]; // empty slice, used just to match signature
sequence_length = utf16_literal_to_utf8(Some(input_slice), Some(end_slice), Some(&mut output));
```
and the callee gates on `if input_end.len() < 6 { return 0; }` → **every `\uXXXX` escape fails**.
`"A"`: C → `"A"`; Rust → parse error. `"𝄞"` (surrogate pair 𝄞): C → correct UTF-8
`f09d849e`; Rust → parse error.
**Key insight: `utf16_literal_to_utf8` alone is FAITHFUL** (our op-3 standalone differential: 0 diffs
when given consistent slices). The bug is the **call-site contract** — unit-testing the function in
isolation cannot find it; only a differential through the caller can. Same family as crc32's
`is_null→is_empty`: a pointer-idiom → slice-idiom rewrite that silently changes boundary semantics.

### 2. Parsed string silently discarded (9,802 both-succeed diffs)
`parse_string`'s success path ends with:
```rust
// We cannot safely create &mut str tied to item from a local Vec without unsafe ...
item.valuestring = None; // ← the parsed string is thrown away
```
Return code says success (`1`), `type` is set to string — but the **value is gone**. `"plain"`:
C → `valuestring="plain"`; Rust → `valuestring=None`. Downstream (getters, printing, compare) silently
sees NULL. The comment shows the model *knew* and punted; the pipeline's compile-only verification
accepted it.

### 3. Non-UTF-8 bytes rejected (within class 1's count)
C cJSON stores raw string bytes without UTF-8 validation; the Rust lift runs `core::str::from_utf8`
and fails the whole parse on non-UTF-8 content. `"\xff\xfe raw"`: C → success (raw bytes); Rust →
parse error. Semantic narrowing introduced by the `&str` safety idiom (third distinct UTF-8 behavior
across tools — cf. SACTOR utf8 idiomatic-fail).

## Method (C-backed, UB-gated)
- Oracle: original cJSON.c (`#include "cJSON.c"` to reach statics). Driver: `diffdrv.rs` as a bin in
  the generated crate (raw-pointer laundering to escape the `&'a mut cJSON<'a>` borrow lock).
- Per-function ops: 0=`parse_hex4`, 1=`parse_number`, 2=`parse_string`, 3=`utf16_literal_to_utf8`;
  shared binary corpus, canonical one-line outputs, byte-compare.
- Corpus: 120,050 records (targeted `\u`/surrogate/escape/high-byte/malformed + random fuzz).
- **UB gate: ASan+UBSan rebuild of the oracle, full corpus: 0 reports, outputs byte-identical.**
  All divergences are on UB-free inputs.
- Controls: `parse_hex4` **0 diffs**, `parse_number` **0 diffs** (the strtod→`f64::parse`
  longest-prefix reimplementation held: an equivalence certificate for the hard sub-case),
  `utf16_literal_to_utf8` standalone **0 diffs**.

## Context: the translation itself
118 code units: ~94 translated, **24 groups exhausted 5 repair attempts → reverted to EMPTY-BODY
stubs** — including the mutually-recursive parse core (`parse_value/parse_array/parse_object`), the
print core (`print_value/print_array/print_object`), `parse_buffer`, `cJSON_PrintUnformatted`,
`print_number`. So the end-to-end round-trip is impossible on this artifact (visible failure, counted
separately); the findings above live in the code the tool **did** claim to translate (silent).

## Files
- `oracle.c` — C oracle (4 ops)
- `diffdrv.rs` — Rust driver (bin inside the translated crate)
- `excerpt_utf16_gate.rs`, `excerpt_callsite_and_valuestring.rs` — the buggy sites verbatim
- `translated_crate/` — the full PtrTrans-generated crate (as committed by the pipeline)
- Scratch (corpus + outputs): `scratchpad/ptrtrans_diff/`

## Repro
```
cc -O1 -I <cJSON-src> oracle.c -o oracle_c -lm
cd translated_crate && cargo build --release --bin diffdrv
<gen corpus per README method>; ./oracle_c < corpus.bin > c.txt; diffdrv < corpus.bin > r.txt; diff c.txt r.txt
```

## Concrete examples (decoded from the differential outputs)

Class 1 — `\u` escapes (C succeeds, Rust rejects):

| input | C | PtrTrans Rust |
|---|---|---|
| `"A"` | ret=1, `A`, offset=8 | ret=0, offset=1 |
| `"hiéthere"` | ret=1, `hiéthere` (`c3 a9`) | ret=0 |
| `"𝄞"` (𝄞 surrogate pair) | ret=1, `f0 9d 84 9e` | ret=0 |

Any legal JSON with unicode escapes (i18n names, emoji) parses in C and errors in the translation.
Note the **offset divergence** (8 vs 1): in the full parser this position drives all subsequent
parsing, so the document-level parse paths split entirely.

Class 2 — success-but-value-lost (both ret=1):

| input | C valuestring | Rust valuestring |
|---|---|---|
| `"plain"` | `plain` | **None** |
| `"tab\tnl\n"` | `tab<TAB>nl<LF>` (escapes expanded) | **None** |
| `"esc\\quote\""` | `esc\quote"` | **None** |

Class 3 — non-UTF-8 (C byte-transparent, Rust validates):

| input | C | Rust |
|---|---|---|
| `"\xff\xfe raw high bytes"` | ret=1, 17 raw bytes stored | ret=0 (`from_utf8` gate) |

Bonus — the differential even surfaces a C-side quirk: `"\uZZZZ"` → C ret=1 with **U+0000**
(cJSON's `parse_hex4` has no error channel for invalid hex; silently yields NUL), Rust ret=0. Under
the C-as-ground-truth convention this counts as a divergence; it doubles as a footnote that
differential testing exposes reference-implementation quirks too.
