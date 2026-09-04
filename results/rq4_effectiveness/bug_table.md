# RQ4 — Confirmed real defects in published C→Rust translations

*Legacy label: this document was “RQ1” under the retired E1/E2/E3 numbering (see `results/INDEX.md`).*

All bugs below are in **published, safety-lifted translations produced by C2SaferRust**
(LLM-based c2rust → safe-Rust lifter; Laertes benchmark suite). Each is confirmed by **differential
testing** against the SAME-SOURCE faithful c2rust `base` (= original C behavior; UB-gated by ASan),
so a divergence localizes to C2SaferRust's rewrite — not to the original C and not to a harness
artifact (see the version-match lesson in `rq1_c2saferrust_round.md`).

## Confirmed bugs

| # | Tool | Program | Function | Mechanism | Class | Confirmation | Trigger / evidence |
|---|------|---------|----------|-----------|-------|--------------|--------------------|
| 1 | C2SaferRust | qsort | `quickSort`/`partition` | `int→usize` rewrite breaks a negative-sentinel loop → **infinite recursion + `*arr.offset(-1)` OOB read** | memory/logic | differential fuzz (~81 execs) | 2-byte input `[5,0]`; `results/rq4_effectiveness/bugs/qsort_c2saferrust/` |
| 2 | C2SaferRust | urlparser | `url_is_ssh` | `CStr::from_ptr(s).to_str().unwrap()` **panics on non-UTF-8**; C (`strcmp`) returns `false` | UTF-8-panic | differential fuzz | harness bytes `31 72 8e`; `results/rq4_effectiveness/bugs/utf8_panic_c2saferrust/` |
| 3 | C2SaferRust | bzip2 | `endsInBz2` | same `to_str().unwrap()` on a **char\* filename** → a valid `.bz2` file with a non-ASCII byte **crashes `bzip2recover`** (C returns 1) | UTF-8-panic | standalone differential repro | `bzip2_endsInBz2_diff.rs` |
| 4 | C2SaferRust | optipng | `opng_process_file` (`-dir`) | same `to_str().unwrap()` on the **char\* directory name** → a non-UTF-8 `--dir` **crashes optipng**; C passes raw bytes to `mkdir` | UTF-8-panic | standalone differential repro | `optipng_dirname_diff.rs` |
| 5 | C2SaferRust | lil | `do_system` (`system` builtin) | same `to_str().unwrap()` on **argv**; a lil script passing a non-UTF-8 arg to `system` **crashes the interpreter**; C concatenates raw bytes | UTF-8-panic | standalone differential repro | `lil_do_system_diff.rs` |

## The UTF-8-panic class (bugs #2, #3 are instances)

C2SaferRust's LLM systematically replaces C byte-string handling (`strcmp`/`strlen`/pointer
indexing) with `CStr::to_str().unwrap()` / `String::from_utf8(...).unwrap()`, which **panic on any
non-UTF-8 byte** — where the original C accepted arbitrary bytes. This is an input-triggered
correctness/robustness regression (the translation is not semantics-preserving).

**Site census — ~27 sites across 6 programs** (`results/rq4_effectiveness/bugs/utf8_panic_c2saferrust/site_census.txt`):

| program | `to_str/from_utf8().unwrap()` sites | confirmed instance? |
|---|---:|---|
| optipng | 12 | **YES — `-dir` path (bug #4)** |
| tulipindicators | 7 | candidate (all in `sample.rs` CLI driver; untriaged) |
| genann | 4 | **NO — all on constant strings** (`CString::new("example/xor.ann")`) → never panic |
| bzip2 | 2 | **YES — `endsInBz2` (bug #3)** |
| lil | 1 | **YES — `do_system` (bug #5)** |
| urlparser | 1 | **YES — `url_is_ssh` (bug #2)** |

The census is an **upper bound**: per-site reachability varies. genann's 4 sites are on *constant*
paths (never panic — not bugs), illustrating that not every site is a bug. **4 confirmed
library-reachable instances across 4 programs** (protocol string, filename, directory name, interpreter
`system` arg) establish the class is real and systematic; tulipindicators' 7 (CLI-driver) are untriaged.
snudown is NOT in the class (it uses safe string forms — no `to_str/from_utf8().unwrap()`).

## Summary

**5 confirmed bugs / 5 programs / 2 mechanisms**, all in C2SaferRust's published output:
one memory/logic bug (qsort `int→usize`), and a **systematic UTF-8-panic class** confirmed in
**4 programs** (urlparser, bzip2, optipng, lil).
Differential testing (faithful-c2rust `base` vs C2SaferRust `_WIP`, same source) is the method;
the UB gate correctly excludes functions that are already UB in the original C (e.g. urlparser's
`malloc(1)`+`sscanf` `url_get_*`), so no false positives are attributed to the translation.

## Not-yet-explored (available for more)

- optipng (`+2495` rewrite), snudown (`+529`) — large pointer-graph codecs; per-function value-boundary
  selection needed. The optipng `dir_name` UTF-8 instance is the cheapest next confirmation.
- SACTOR — a different tool, richer value-semantics lift; needs LLM $ authorization.
