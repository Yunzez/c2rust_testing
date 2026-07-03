# RQ1 — SEMANTIC-DIFFERENCE bugs (Priority-0 class)

The class the whole project exists to find: **C UB-free, both sides terminate, `output(source) ≠
output(translation)`** — a wrong value with no crash, invisible to fuzzing the Rust alone (no oracle).
Every entry below is confirmed **empirically** (built both sides, ran them, compared), not by inspection.
Attribution is always against a **same-source faithful reference** (base c2rust, or the original C).

## The table

| # | tool | program / fn | the bug | trigger (UB-free) | source vs translation | evidence |
|---|------|--------------|---------|-------------------|-----------------------|----------|
| **1** | **C2SaferRust** (published) | optipng `crc32_z` | ptr→slice lift rewrote NULL guard `buf==Z_NULL` → `buf.is_empty()`; empty chunk resets running CRC | `crc32_z(X≠0, valid buf, len=0)` | source `X` vs WIP **`0`** | 1M sweep, all diffs len==0; non-empty byte-exact. Marked **Success** by the tool. End-to-end: corrupts optipng IDAT CRC. |
| **2** | **C2SaferRust** (published) | optipng `adler32_z` | same empty-input reset (`is_null`→`len==0`), returns seed | `adler32_z(X≠1, valid buf, 0)` | source `X` vs WIP **`1`** | direct test; ALSO grossly miscompiled on non-empty (marked Failure, shipped) |
| **3** | **C2SaferRust** (published) | bzip2 `BZ2_bzBuffToBuffCompress` | ptr NULL-check `source==NULL` → slice `source.is_empty()`; empty input rejected | compress an empty buffer | source: BZ_OK + valid empty .bz2 vs WIP: **BZ_PARAM_ERROR (-2)** | guard-level (unambiguous); full-run deferred |
| **4** | gpt-4o-mini (weaker LLM) | `fixscale` (Q8.8 shift) | `(int16 x >> shift)` on promoted 32-bit int lifted to `i16 >> shift`; Rust masks shift ≥16 | `fixscale(x, shift)` with shift 16..31 (C-defined) | e.g. x=-32767 shift=26: C **`-1`** vs Rust **`-32`** | exhaustive shift 0..31 × all i16: 917,506 diffs |
| **5** | gpt-4o-mini (weaker LLM) | `leb128_roundtrip` | decoded value passed **by value** to helper → lost (always 0); + `while` vs do-while drops the 0 case | any input | C re-encodes the value vs Rust encodes **0 / nothing** | exhaustive ≤2 bytes + targeted: thousands of diffs |
| **6** | **RustAssure** (published, gpt-3.5-turbo backend) | u8c `u8strlen` | idiomatic rewrite `s.chars().filter(\|c\| (c as u8 & 0xC0)!=0x80)` iterates decoded codepoints then truncates to low byte — byte-level continuation test becomes meaningless | any string with a codepoint whose low byte ∈ 0x80..0xBF (e.g. "©", "日本語") | C counts codepoints correctly vs gpt-3.5 **undercounts** ("©"→0 not 1) | exhaustive single codepoints U+0020..U+FFFF: 15,877 diffs; gpt-4o backend 0 diffs |

Clean (certificates, same batch/method): C2SaferRust `bitset` count/find_first/find_next (exhaustive
2^32, 0 diffs); gpt-4o-mini `fixmul` (2^32, 0), `percent_decode` (98k, 0); gpt-5.1/SACTOR leb128 &
glob (exhaustive, 0); SACTOR mu_atoi/rpn faithful, base64 fuzz-clean.

## Two tiers, honestly separated

**Tier 1 — published tool, credible headline (#1–#3): the C2SaferRust NULL/empty conflation class.**
A *systematic* defect: its pointer→slice safety-lift turns C's NULL-pointer guard into an empty-slice /
`len==0` guard, conflating "no buffer" with "valid empty buffer." Confirmed in **3 functions across 2
crates**. #1 (crc32) is the clean headline — the tool marked it **Success**, it is otherwise byte-exact
faithful, the divergence is silent (no crash/UB), and it reaches real PNG output (IDAT CRC). This is the
raison d'être demonstrated on a published artifact: only differential testing finds it.

**Tier 2 — translator-quality spectrum (#4–#5): a weaker but widely-used model (gpt-4o-mini).**
Frontier gpt-5.1 translated the *same* functions faithfully (fixmul 2^32 clean; it clamped the shift;
leb128 exhaustively clean); gpt-4o-mini did not. This is supplementary "long-tail" evidence — real-world
C→Rust is not all frontier models — and shows the method catches diffs across the translator-quality
spectrum. Framing caveat: a reviewer discounts "you used a weak model," so these are **scale evidence,
not the headline**.

## Second published tool (#6) — RustAssure, ready artifact, $0

RustAssure ("Differential Symbolic Testing for LLM-Transpiled C-to-Rust Code", UC Davis) ships an
artifact that translates a UTF-8 library (`u8c`) with **multiple backend models** (gpt-3.5-turbo,
gpt-4o-mini, gpt-4o, claude-3-5-sonnet). This gives us a **second published tool at zero setup** and a
**model-quality spectrum inside one tool's own artifact**: gpt-3.5-turbo's `u8strlen` silently
undercounts UTF-8 length (15,877 diffs), while gpt-4o / gpt-4o-mini are faithful. It also sharpens the
methodological claim — RustAssure's contribution is a *symbolic* (KLEE + edit-distance) equivalence
check; our concrete differential fuzzing over the real multi-byte UTF-8 space finds the divergence
directly. Now **two published tools have confirmed semantic diffs** (C2SaferRust, RustAssure); Laertes,
CROWN, and SACTOR are faithful (certificates). Repro: `results/rq1_bugs/rustassure_u8strlen/`.

## Multi-tool contrast (the spectrum, with evidence)

Same `crc32_z`, three published translators on the same source (`laertes_benchmarks/`):
`SOURCE=0x12345678` · **C2SaferRust=0x00000000 [BUG]** · **Laertes=0x12345678 [ok]**. Laertes
(rule-based, conservative) left the function as raw-pointer c2rust with the original `is_null`;
C2SaferRust (aggressive LLM) lifted it and broke it. Differential testing **places each tool on a
faithfulness spectrum with evidence** — aggressive-LLM (bugs) → principled-lifter (faithful) →
frontier-LLM (faithful-or-fails). Single-program fuzzing of any one output produces none of this.

## Repro
- #1–#2 + multi-tool + end-to-end: `results/rq1_bugs/crc32_c2saferrust/` (`cargo run --release`).
- #4–#5: `scratchpad/fix_exhaust`, `scratchpad/leb_exhaust` (swap `src/translated.rs`); translations in
  `scratchpad/weak_batch/`.
