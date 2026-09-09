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
| **C2SaferRust** | 121 / 96 | 79 | 1 792 | 144 / 564 (0.255) | 9 815 / 37 297 (0.263) | **802 `confirmed_termination` + 397 `confirmed_divergence`** on 11 boundaries → **C13, S16, S1 re-found**; the rest un-triaged |
| **Laertes** | 121 / 55 | 48 | 999 | 71 / 820 (0.087) | 6 611 / 49 009 (0.135) | **200 `confirmed_termination` + 97 `confirmed_divergence`** → **S4 re-found, S17, C14** |

## What this library says

1. **Two named limits decide the funnel, and neither is coverage.** (a) 424–431 of 552 boundaries fail at the
   signature on struct-invariant parameters (`png_struct_def` 178 + 30 opaque, `z_stream_s` 39, `gzFile_s`
   29, `deflate_state` 28, `FILE` 24, `gz_state` 13). (b) Of the ~125 planned, 57 of c2rust's 74 build
   failures (66 on Laertes, 25 on C2SaferRust) are duplicate C symbols in the multi-unit build
   (`the_exception_context`, a header tentative definition under `-fno-common`; libpng's `png_get_uint_*`
   macros). `-fcommon` would raise the built count but changes the pair's frozen build condition, so it
   was **not** applied; if ever, as a separate sensitivity run over all three cells.
2. **c2rust confirms nothing on the largest artifact** — the negative control holds for a ninth library.
3. **C2SaferRust: 1 199 confirmed rows on 11 boundaries, read at root-cause level.** Two new entries
   (**C13** `opng_free`: `free(ptr)` → `drop(Box::from_raw(ptr))`, panics on NULL where C is a no-op;
   **S16** `opng_strcasecmp`: byte-wise `tolower` compare replaced by a lossy UTF-8 decode, 55/55
   divergence) and one re-find (**S1**: `crc32` 26/26 through `crc32_z`'s `is_empty`). The remaining
   ~1 100 rows (`compress`, `compress2`, `uncompress`, `optimize_cmf`, `bmp_memset_bytes`, `adler32`,
   `crc32_combine[64]`) are confirmed against a clean C reference but **not root-caused and therefore not
   in the manifest**; they are triage candidates, plausibly a handful of zlib root causes (this
   translation is E3's CRASH-ALL trio member).
4. **Laertes: every confirmed row is the severed-init law** (178 `laertes_init_*` defined, 0 called).
   `crc32`/`crc32_z` 23/23 = **S4 re-found** (deflate's zeroed tables behind `compress`/`compress2` are
   the same law, recorded under S4); two new sites join the family: **S17** `opng_path_make_backup`
   (`".bak"` is five zero bytes: the backup path equals the input path, 19/19) and **C14** `uncompress`
   (inflate's `order[19]` zero → `lens[1..18]` unwritten → `inflate_table` indexes `count[16]` with garbage;
   200 panics, C returns; backtrace uncompress → uncompress2 → inflate → inflate_table).

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
`optipng_multi.c`). Manifest: **C13, S16, S17, C14** (new), **S1, S4** re-found.
