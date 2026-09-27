# RQ4 — reach of the generated campaign: optipng

*`rq3_coverage/` is a legacy directory name; this is current **RQ4**.* Protocol: [`../PROTOCOL.md`](../PROTOCOL.md)
(§2 amendment 2026-09-07); plan: [`../../../docs/rq4_plan_ten_libraries.md`](../../../docs/rq4_plan_ten_libraries.md);
denominator: [`../../../docs/rq4_denominator_decision_2026-09-08.md`](../../../docs/rq4_denominator_decision_2026-09-08.md).
Status 2026-09-09: **three cells complete and verified**, 3 600 s each, one campaign and one corpus per
cell, `-max_len 4096`, seed 42, generator **0.8**. The largest artifact in the study and the only multi-unit
pair: 52 translation units (`make_pair.py --tus`) over optipng, its vendored zlib, libpng, gifread, minitiff,
pnmio, pngxtern and cexcept; 552 boundaries matched. Three translations build (E1: SACTOR ✗ parse,
PtrTrans ✗ PA).

**Tests side**: no test target → TEST-UNAVAILABLE on all three; universe = the translation's own
instrumented objects (Laertes' includes its runtime, so its fraction is not comparable with the other two).

## Cell table

| tool | planned / built of 552 | exported | corpus | fn ours | reg ours | confirmed (sample) |
|---|---:|---:|---:|---|---|---|
| **c2rust** | 128 / 54 | 47 | 2 134 | 106 / 555 (0.191) | 9 970 / 37 840 (0.263) | **0** (436 ub_associated, 208 instrument_only, 42 not reproducible, 21 inconclusive, 3 ub_associated_termination) |
| **C2SaferRust** | 121 / 96 | 79 | 1 792 | 144 / 564 (0.255) | 9 815 / 37 297 (0.263) | **771 `confirmed_termination` + 397 `confirmed_divergence`** on 11 boundaries, all triaged (`c2saferrust/TRIAGE.md`) → **S19, S20, C15, S21, C16, C13, S16** new; **S1, S2** re-found; 1 non-defect |
| **Laertes** | 121 / 55 | 48 | 999 | 71 / 820 (0.087) | 6 611 / 49 009 (0.135) | **97 `confirmed_divergence`** (+ 200 sanitizer-only panics, `instrument_only`) → **S4 re-found, S17, S18** |

## What this library says

1. **Two named limits decide the funnel, and neither is coverage.** (a) 424–431 of 552 boundaries fail at the
   signature on struct-invariant parameters (`png_struct_def` 178 + 30 opaque, `z_stream_s` 39, `gzFile_s`
   29, `deflate_state` 28, `FILE` 24, `gz_state` 13). (b) Of the ~125 planned, 57 of c2rust's 74 build
   failures (66 on Laertes, 25 on C2SaferRust) are duplicate C symbols in the multi-unit build
   (`the_exception_context`, a header tentative definition under `-fno-common`; libpng's `png_get_uint_*`
   macros). `-fcommon` would raise the built count but changes the pair's frozen build condition, so it
   was **not** applied; if ever, as a separate sensitivity run over all three cells.
2. **c2rust confirms nothing on the largest artifact** — the negative control holds for a ninth library.
3. **C2SaferRust: 1 168 confirmed outcomes (of 1 199 records) on 11 boundaries, every one read to its root cause**
   (`c2saferrust/TRIAGE.md`). Six defects: **S19** (`send_bits` shifts a `u16` by 16 where C promotes
   to int: 228 panics under overflow checks, a corrupted stream in release), **S20** (inflate compares
   the state POINTER with the byte count instead of `state->offset`: 169 `destLen` divergences + 4 NULL
   dereferences), **C15** (`inflate_fast` lost `from = out - dist`, slices over NULL: 48 panics),
   **S21** (`crc32_combine_` squares the wrong matrix and swaps `even`/`odd`: 23 + 23 divergences),
   **C13** (`opng_free`), **S16** (`opng_strcasecmp`); two re-finds, **S1** (`crc32` through `crc32_z`)
   and **S2** (the `adler32_z` rewrite behind `compress2`'s 169 panics + 99 divergences). **C16**
   (`optimize_cmf`, 200) is profile-dependent: C's unsigned wrap is defined, the translation's checked
   `-=` panics under the crate's debug profile and matches C only in release — settled by a
   deterministic probe on the valid header `08 1d`. One cluster is not a defect: `bmp_memset_bytes`
   (117: a harness input-model gap, `offset` unbounded before `memset`). 31 `uncompress` rows failed
   only under ASan and are `instrument_only`.
4. **Laertes: every confirmed row is the severed-init law** (178 `laertes_init_*` defined, 0 called).
   `crc32`/`crc32_z` 23/23 = **S4 re-found** (deflate's zeroed tables behind `compress`/`compress2` are
   the same law, recorded under S4); two new sites join the family: **S17** `opng_path_make_backup`
   (`".bak"` is five zero bytes: the backup path equals the input path, 19/19) and **S18** `uncompress`
   (inflate's `order[19]` zero → `lens[1..18]` unwritten → `inflate_table` reads uninitialised heap: 4
   `destLen` divergences; the 200 index-out-of-bounds panics appear only in the ASan build, whose fill
   pattern 0xBEBE = 48830 goes out of range, and every no-sanitizer replay returned normally — so the
   entry is semantic, not crash).

## Gaps, deviations and limits

- Pre-accepted crash-alls per pair (`preflight_accept.txt`): five functions whose contract is to exit
  (`DefaultError`, `default_error_handler`, `ErrorAlloc`, `pngx_gif_error`, `pngx_tiff_error`),
  `opng_get_alpha_row`, and `png_format_number` (`*--end`: the `(begin, end)` range-pair modelling gap,
  as in lodepng); on C2SaferRust nine more Rust-side crash-alls, each adjudicated individually.
- optipng × c2rust's coverage analysis was recovered on 2026-09-09 (scratch directory deleted;
  harnesses regenerated; explicit `--path-map`; `analysis/recovery.json`); 0 functions outside the
  universe. optipng × Laertes is the rerun of 2026-09-09 (`--reuse-bins`).
- Laertes' 0.087 is over a universe that includes the runtime; compare only-ours counts, not fractions.

## Files

`tests_side_results.json`, `cells.json`, `<tool>/` (RUN.md with §7 prose, funnel.json, plans.json, analysis/
[+ recovery.json on c2rust], divergences/, confirm_sample/, confirmed_inputs/, candidates_sample/,
corpus.tar.gz, harness_exports.tar.gz, artifact_hashes.json, raw/denominator.json). Pairs:
`benchmark/pairs/rq4/optipng_{c2rust,c2saferrust,laertes}/` (+ `preflight_accept.txt`, `PROVENANCE.json`,
`optipng_multi.c`). Manifest: **C13, S16, S17, S18, S19, S20, C15, S21, C16** (new), **S1, S2, S4** re-found. Triage: `c2saferrust/TRIAGE.md`.
