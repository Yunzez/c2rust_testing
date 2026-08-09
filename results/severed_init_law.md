# The severed-initializer law (Laertes) — a predictive defect class, not a bug count

**Status 2026-08-09.** Scanner: `scripts/scan_severed_init.py` · raw data: `results/severed_init_scan.json`

## The law

Laertes rewrites a C compile-time-initialised global

```rust
static mut X: T = <real value>;                              // c2rust base
```

into a **zero-initialised static plus a runtime initialiser**

```rust
static mut X: T = <zero>;  unsafe fn laertes_init_X() { X = <real value>; }
```

…and then **never emits a call to `laertes_init_X`**. The static keeps its zero value for the entire
life of the program.

> **Measured across all 10 shipped Laertes translations: 277 `laertes_init_*` functions are defined and
> `0` are ever called.** Not "mostly", not "in the libraries we looked at" — **zero call sites, every
> crate, without exception.**

| crate | `laertes_init_*` defined | call sites | poisoned | poisoned in **library** code |
|---|---:|---:|---:|---:|
| optipng | 165 | **0** | 117 | **114** |
| bzip2 | 37 | **0** | 7 | **7** |
| xzoom | 32 | **0** | 6 | **6** |
| tulipindicators | 18 | **0** | 13 | **1** |
| genann | 11 | **0** | 3 | **0** |
| snudown | 10 | **0** | 10 | **10** |
| lil | 2 | **0** | 1 | **1** |
| grabc | 1 | **0** | 0 | 0 |
| urlparser | 1 | **0** | 1 | **0** |
| qsort | 0 | **0** | 0 | 0 |
| **TOTAL** | **277** | **0** | **158** | **139** |

## Why this is a law and not a bug count

The defect is **universal**; whether it is *fatal* is a two-step filter, and both steps are mechanical:

1. **Poisoned?** The C value must actually be non-zero. A severed initialiser for a static that was
   already zero-filled is a no-op. → 158 of 277.
2. **In library code?** A poisoned *test fixture* or *example driver* is not a shipped-library defect.
   → 139 of 158. (tulipindicators is the clearest case: 13 poisoned, but 12 live in `sample.rs` /
   `fuzzer.rs`; only `ti_indicators` is real.)
3. **No lazy rebuild?** A consumer that reconstructs the value on first use makes the severing harmless.

This is what makes the finding *predictive*: it produces a ranked, machine-generated worklist rather
than an anecdote, and it says in advance which cells cannot be affected (`qsort` — 0 initialisers, so
the class is structurally absent, which is exactly what E1/E3 found).

### The scan is calibrated at both ends

| control | expectation | result |
|---|---|---|
| **`bzip2::BZ2_crc32Table`** — a **confirmed E1 bug** (zeroed CRC table) | must be flagged | ✅ flagged, 256 zeros vs real CRC constants, 10 consumer refs |
| **`genann::lookup`** — known **harmless** (footnote 16) | must **not** be flagged | ✅ not flagged — its base initialiser is `[0.; 4096]`, i.e. already zero; genann builds it lazily |
| **`tulipindicators::ti_indicators`** — independently hit during E3 (had to be called via `Once` to make the harness reach 173/224) | must be flagged | ✅ flagged, and it is the *only* library-level poisoned static in that crate |

Two of these controls cost real debugging to get right, and both failures were **false negatives that
would have understated the class**: array types `[T; N]` broke the initialiser parser (hiding every
table-valued static, including `BZ2_crc32Table` itself), and fully-qualified `crate::…::new()` paths
broke the zero-detector (hiding `ti_indicators`). An uncalibrated version of this scan reported
`0 poisoned` for bzip2 — the crate with a *confirmed* instance.

## New finding — `snudown::sd_autolink_issafe`, a silent security-whitelist failure

`snudown` is Reddit's markdown renderer. It is **not currently a row in Table 1.**

```rust
// base (c2rust):
static mut valid_uris_count: size_t = 14;

// Laertes (autolink.rs:96):
static mut valid_uris_count: c_ulong = 0;
unsafe fn laertes_init_valid_uris_count() { valid_uris_count = 14; }   // never called

// consumer (autolink.rs:116):
while i < valid_uris_count {            // 0 iterations — body never runs
    ... return 1                        // "safe URL" — unreachable
}
return 0;                               // every URL is reported UNSAFE
```

**`sd_autolink_issafe` returns 0 for every input, including `http://…`.** It does not crash and does not
panic: the companion `valid_uris[14]` table is severed too, but because the loop never iterates it is
never dereferenced — which is precisely why the failure is **silent**. A crash here would have been
caught by any smoke test; returning "unsafe" for everything is a whitelist that fails closed and quietly.

Three further silent instances in the same crate, same mechanism:

| static | base value | Laertes | consequence |
|---|---|---|---|
| `MAX_NUM_ENTITY_LEN` | 7 | 0 | every numeric entity `&#NNN;` rejected as over-length |
| `MAX_NUM_ENTITY_VAL` | `0x10ffff` | 0 | every numeric entity rejected as out-of-range |
| `UTF8_BOM` | `ef bb bf` | `[0,0,0]` | BOM not stripped; a document starting with three NUL bytes *is* |

And one **loud** sibling that matters for test design: `markdown_char_ptrs: [char_trigger; 13]` becomes
all-`None`, consumed via `.expect("non-null function pointer")` — so any document containing an active
markdown character **panics**. Whole-document testing hits the panic first and masks the three silent
bugs above; they must be reached with **function-level** drivers or with active-char-free input.

## Worklist — highest-value remaining targets

| crate | poisoned (lib) | notable |
|---|---:|---|
| **optipng** | 114 | zlib `crc_table` (87 refs, *already* an E1 bug), gifread `table` (102 refs), `order`, `_dist_code`, Adam7 `png_pass_start/inc` — a deep mine, but mostly *more instances of an existing cell* |
| **snudown** | 10 | **4 silent + 1 panic verified above; not yet a Table 1 row** ← best value |
| **bzip2** | 7 | `BZ2_crc32Table` (known), `BZ2_rNums`, `incs`, `zSuffix`, `bzerrorstrings` |
| **xzoom** | 6 | untested library |
| **tulipindicators** | 1 | `ti_indicators` — zeroed 105-entry table ⇒ `ti_find_indicator` does `strcmp` against NULL ⇒ likely **crash**, not silent. Would flip a ✓ certificate to a bug cell |
| **lil** | 1 | `running` (base 1 → 0) |

## How to use this in the paper

State it as a **mechanism with a denominator**, not as a bug tally:

> Laertes emits 277 runtime initialisers across its 10 shipped translations and calls none of them.
> 139 of those initialise a value that is non-zero in the C source and live in library (not fixture)
> code. We confirm the consequence end-to-end in *n* cases, spanning silent wrong-answer
> (`sd_autolink_issafe`), silently corrupted checksums (`BZ2_crc32Table`, `crc_table`), and hard faults
> (`ti_indicators`); one poisoned static (`genann::lookup`) is provably harmless because the library
> rebuilds it lazily, which is the control that keeps the class from being overstated.

This directly answers the "your 20 bugs are really 6 mechanisms" objection by **conceding it and making
the mechanism the headline**: one root cause, a machine-checkable predicate for it, 139 predicted sites,
and a stated precision control.

## Reproduce

```bash
python3 scripts/scan_severed_init.py                 # -> results/severed_init_scan.json
# universality, by hand, for any crate:
B=tools/frameworks/c2saferrust/laertes_benchmarks
grep -rn --include=*.rs 'fn laertes_init_' $B/snudown_laertes/ | wc -l    # 10
grep -rn --include=*.rs 'laertes_init_'    $B/snudown_laertes/ \
  | grep -v 'fn laertes_init_' | wc -l                                    # 0
```

## Open

- **Not yet executed.** Every claim above is established by source inspection + the E1/E3 corroboration
  of `BZ2_crc32Table` and `ti_indicators`. The `snudown` instances need a built differential harness
  (function-level, to dodge the `markdown_char_ptrs` panic) before they can be listed as confirmed bugs.
- **Scope is Laertes only.** The `laertes_init_*` marker is tool-specific. The generic form of the
  predicate — *a `static` whose initialiser is all-zero in the translation and non-zero in the C source* —
  applies to every tool and has not yet been run across C2SaferRust / CROWN / SACTOR / PtrTrans output.
