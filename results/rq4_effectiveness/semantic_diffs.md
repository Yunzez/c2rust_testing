# RQ4 — SEMANTIC-DIFFERENCE defects (Priority-0 class)

*Legacy label: this document was “RQ1” under the retired E1/E2/E3 numbering (see `results/INDEX.md`).*

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
| **6** | gpt-3.5-turbo (backend of RustAssure's artifact) | u8c `u8strlen` | idiomatic rewrite `s.chars().filter(\|c\| (c as u8 & 0xC0)!=0x80)` iterates decoded codepoints then truncates to low byte — byte-level continuation test becomes meaningless | any string with a codepoint whose low byte ∈ 0x80..0xBF (e.g. "©", "日本語") | C counts codepoints correctly vs gpt-3.5 **undercounts** ("©"→0 not 1) | exhaustive single codepoints U+0020..U+FFFF: 15,877 diffs; gpt-4o backend 0 diffs |
| **7** | gpt-3.5-turbo (backend of RustAssure's artifact) | u8c `u8next_` | decode loop `break`s after the lead byte in every arm — **never reads continuation bytes**; returns masked lead byte as the codepoint | any multi-byte UTF-8 char | C decodes full codepoint vs gpt-3.5 returns masked lead byte ("©"→2 not 169, "😀"→0 not 128512) | exhaustive single codepoints U+0020..U+10FFFF: **1,111,936 / 1,112,032 diverge**; gpt-4o carries full FSM |

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

## RustAssure & Flourine are VALIDATORS (related work), not buggy translators — correction

Important framing correction: **RustAssure and Flourine are differential *validation* tools, not
translators.** RustAssure ("Differential Symbolic Testing for LLM-Transpiled C-to-Rust", UC Davis) =
KLEE symbolic execution + edit-distance similarity; Flourine = UB-blind differential fuzzing with a
repair loop (the project's own `paper_snippets_rq2_rq3.tex` already lists both as RQ4 head-to-head
baselines). They *use* an LLM backend to translate, then validate — so a wrong translation in their
artifact is the **backend model's** fault (gpt-3.5-turbo), not the tool's. #6–#7 are therefore **tier-2
weak-model** diffs, surfaced from RustAssure's shipped multi-model `u8c` artifact — NOT a "second buggy
published translator."

Their real role is as **baselines our method is measured against**: concrete UB-gated differential
fuzzing + bounded exhaustion + name-independent matcher vs their symbolic (RustAssure) / UB-blind-fuzz
(Flourine) approaches. #6–#7 double as evidence for that comparison — both trigger on **multi-byte
UTF-8**, exactly the input structure symbolic execution struggles to explore; concrete fuzzing hits
1.1M diffs instantly. **Only ONE published *translator* has confirmed semantic diffs so far
(C2SaferRust); SACTOR / Laertes / CROWN / Flourine translate faithfully (certificates).**

## Multi-tool contrast (the spectrum, with evidence)

Same `crc32_z`, three published translators on the same source (`laertes_benchmarks/`):
`SOURCE=0x12345678` · **C2SaferRust=0x00000000 [BUG]** · **Laertes=0x12345678 [ok]**. Laertes
(rule-based, conservative) left the function as raw-pointer c2rust with the original `is_null`;
C2SaferRust (aggressive LLM) lifted it and broke it. Differential testing **places each tool on a
faithfulness spectrum with evidence** — aggressive-LLM (bugs) → principled-lifter (faithful) →
frontier-LLM (faithful-or-fails). Single-program fuzzing of any one output produces none of this.

## Repro
- #1–#2 + multi-tool + end-to-end: `results/rq4_effectiveness/bugs/crc32_c2saferrust/` (`cargo run --release`).
- #4–#5: `scratchpad/fix_exhaust`, `scratchpad/leb_exhaust` (swap `src/translated.rs`); translations in
  `scratchpad/weak_batch/`.
