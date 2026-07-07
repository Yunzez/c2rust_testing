# External validity — real upstream C libraries (dataset v4) — 2026-06-24

The authored corpus could be accused of being shaped to fit the story. This milestone tests the
boundary-validity findings on **real, permissively-licensed, widely-deployed C** that we did not
write: base64 (public domain) and seven musl libc functions (MIT). Each is a self-contained single
translation unit that transpiles cleanly under c2rust 0.22.1 + clang 21. Provenance and licensing:
`benchmark/pairs/EXTERNAL_PROVENANCE.md`.

## What it took to run real code (three tooling gaps, all fixed)

The authored corpus is self-contained single-`.c`, `size_t` lengths, non-libc names. Real libraries
broke three hidden assumptions; each fix is general and regression-green (101/101 unit, 17/17
byte-identity):

1. **Sibling headers** — real libs split into `.c` + `.h`; the harness copied only the `.c`, so
   `#include "base64.h"` failed. Now copy every `source/*.h` into the harness `c/` dir.
2. **Non-`size_t` lengths** — a buffer's length is derived from `.len()` (usize), but base64 uses
   `unsigned int` (`E0308: expected u32, found usize`). Cast `.len()` to the length param's actual
   Rust type; no cast for usize (byte-identical for the authored corpus).
3. **libc symbol collision** — c2rust emits `#[no_mangle] pub extern "C" fn <name>`; a function
   literally named `strlen`/`memcmp` clashes with libc at link time. The musl programs are renamed
   `mu_<name>` (pure symbol rename; the algorithm body is byte-for-byte musl).

(1) and (2) are committed in the generator; (3) is a corpus-prep convention.

## The mechanisms reproduce on un-authored code (sanitizer-confirmed)

9 external boundaries: 7 harvested (4 valid : 3 invalid) + 2 census-excluded. **Both negative
mechanisms appear naturally on real code and are confirmed by an independent sanitizer**, not by us:

| boundary | validity_v2 | mechanism | independent evidence |
|---|---|---|---|
| base64_decode | valid | — | 1.16M execs, no divergence |
| mu_strlen / mu_strncmp / mu_strspn | valid (×3) | — | 0.58–0.86M execs (NUL satisfied by construction) |
| base64_encode | invalid_isolation_invariant | output-size precondition | ASan heap-overflow (needs 85 B, our cap 64) |
| mu_atoi | invalid_intrinsic_ub | signed multiply-accumulate overflow | UBSan `10 * n cannot be represented in 'int'` |
| mu_llabs | invalid_intrinsic_ub | negation of LLONG_MIN | UBSan `negation of -9223372036854775808 …` |
| mu_memcmp / mu_memchr | excluded (census) | `const void*` buffer not constructible | gate `UNSUPPORTED_PARAM (struct_ptr)` |

These are not bugs in musl/base64 — they are documented preconditions (atoi requires a representable
input; `llabs(LLONG_MIN)` is UB by the C standard; `out` must be `BASE64_ENCODE_OUT_SIZE(inlen)`).
The boundary is an invalid differential harness *because* of them. An independent run of the audit
heuristics agreed on 6/7 (high confidence on both UB cases); base64_encode was correctly flagged for
spot-check (ASan crash, no UBSan signal) and resolved to isolation by the output-size analysis.

## External code revealed both feature families were narrower than their mechanism

Feature firing on the external programs exposed an honest overfit to the authored operator forms:

- **intrinsic-UB**: the UB-op signal was `div/mod`, `shift`, and compound-assign (`+= -= *=`) only.
  `mu_atoi`'s overflow is `n = n*10 + d` (plain binary `*`), and `mu_llabs` is unary `-a` — **both
  were missed**. Generalized (AST): added `rf_mul` (binary `*`) and `rf_negate` (unary `-`), folded
  into the `rf_unguarded_ubop` interaction. Both are genuinely overflow-capable on signed types.
- **isolation**: the existing signals targeted struct-field-index trust (ring-buffer cursors).
  base64's "caller must size the output buffer" is a different sub-mechanism. Added `rf_unsized_output`
  (a non-const pointer param written via subscript with no following size/capacity param).

## Re-run on v4 (134 boundaries) — the contribution holds and strengthens

Program-grouped 5-fold CV (external programs are their own groups → genuine held-out transfer):

| task | generic | boundary-specific | combined | boundary v3 → v4 |
|---|---|---|---|---|
| lumped | 0.690 | 0.702 | 0.735 | 0.660 → 0.702 |
| valid vs **isolation** | 0.767 | **0.865** | 0.795 | 0.852 → 0.865 |
| valid vs **intrinsic-UB** | 0.591 | **0.749** | **0.777** | 0.676 → 0.749 |

The per-mechanism advantage holds on the combined corpus and **strengthens** for intrinsic-UB — the
gain is directly attributable to `rf_mul`/`rf_negate` capturing the real-world overflow forms.
Size-confound control on v4: `size_only` ≈ 0.36 (≤ chance), `generic_ablated` ≥ `generic` — size is
still not the confound (`validity_baseline_size_control_v1.md`). Generic stays near-random (0.59) on
intrinsic-UB: structural features cannot see whether arithmetic is guarded.

## Honest caveats

- **`rf_unsized_output` is loose**: it also fires on 15 authored in-place/sliced buffers (valid),
  whose bounds are caller-supplied indices (`lo`/`hi`/`mid`) the size-keyword check does not match.
  The intrinsic-UB gain is cleanly from `rf_mul`/`rf_negate`; the isolation gain is partly the
  external data itself. The feature was **not** tightened to base64 alone (that would overfit one
  example); tightening to pure write-only outputs is future work.
- **base64_decode "valid"** carries the same output-size precondition as encode; it is valid only
  because the realized decoded size (≤45) fits our cap-64 — a realized-outcome label, not a
  mechanism-free one.
- **Small data**: ~18–19 invalid-bearing programs plus a handful of external ones. Directional, not
  definitive; more external corpora remain the next robustness step.
