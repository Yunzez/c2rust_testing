# Bug class: C2SaferRust injects UTF-8-validity panics (`to_str().unwrap()`)

**Finding.** C2SaferRust's LLM-based safety lift systematically replaces C byte-string handling with
`CStr::from_ptr(p).to_str().unwrap()` (and `String::from_utf8(...).unwrap()`). `to_str()` returns
`Err` on any non-UTF-8 byte, so `.unwrap()` **panics (aborts the program)** on inputs the original C
handled fine via byte-wise `strcmp`/`strlen`/indexing. This is an input-triggered
correctness/robustness regression — the translation is NOT semantics-preserving: C returns a value,
the Rust port crashes. Realistic triggers: non-ASCII filenames (Linux paths are arbitrary bytes),
network/protocol data, binary input.

Found by differential testing (OOP): faithful c2rust `base` (= C behavior, UB-gated by ASan) vs
C2SaferRust `_WIP`, same source. A divergence where base is UB-free and WIP panics = a real bug.

## Two CONFIRMED differential instances

### Instance A — urlparser `url_is_ssh` (differential fuzz)
- base (= C): `strdup` + `strcmp(str,"ssh")||strcmp(str,"git")` → handles any bytes → returns false.
- WIP: `CStr::from_ptr(str).to_str().unwrap()` → panics on non-UTF-8.
- Trigger (harness bytes `31 72 8e`): idx→url_is_ssh, URL with byte 0x8e. base → `b:0` (rc 0);
  WIP → panic at `urlparser_WIP/test.rs:486`. Harness: `results/rq1_c2saferrust_poc/` (url_*).

### Instance B — bzip2 `endsInBz2` (standalone differential repro, `bzip2_endsInBz2_diff.rs`)
- base (= C): `strlen` + byte-wise compare of the last 4 chars to ".bz2" → handles any bytes.
- WIP: `CStr::from_ptr(name).to_str().unwrap()` → panics on non-UTF-8.
- Repro output (name = `[0x8e, '.','b','z','2']`, a valid `.bz2` file with a non-ASCII byte):
  ```
  base (=C/faithful c2rust): returns 1  (clean)   <- correctly identifies a .bz2 file
  WIP  (C2SaferRust):        PANIC (to_str().unwrap on non-UTF-8)
  ```
  A legitimately-named `.bz2` file with a non-UTF-8 byte crashes `bzip2recover`. Build+run:
  `rustc --edition 2021 -O bzip2_endsInBz2_diff.rs -o bzip2_diff && ./bzip2_diff`.

## Systematic: ~27 sites across 6 programs (`site_census.txt`)

| program | to_str/from_utf8 `.unwrap()` sites |
|---|---:|
| optipng | 12 | **confirmed (bug #4)** |
| tulipindicators | 7 | no |
| genann | 4 |
| bzip2 | 2 |
| lil | 1 |
| urlparser | 1 |

Not every site is reachable with attacker-controlled non-UTF-8 (some are on constant strings or
example/CLI code — genann's 4 are in `example3/example4`, lil's is on a CLI arg). The two confirmed
library-function instances (url_is_ssh on a protocol string, endsInBz2 on a filename) take external
`char*` and are cleanly triggerable. The census bounds the class; per-site reachability is future work.

## Attribution

The faithful c2rust `base` (same source, byte-wise) is UB-free and returns a value on the trigger;
C2SaferRust's rewrite is the sole difference (it added the `to_str().unwrap()`). Same protocol as
bug #1 (qsort): base correct, `_WIP` diverges. Not a harness artifact — the base oracle is built
from the SAME source as the WIP (see results/rq1_c2saferrust_round.md for the version-match lesson).

## Relation to bug #1

Bug #1 (qsort, `results/rq4_effectiveness/bugs/qsort_c2saferrust/`) is a distinct mechanism (`int→usize` sentinel
break → infinite recursion + OOB read). Together: **3 confirmed bugs across 3 programs, 2 mechanisms**
(one memory/logic, one systematic UTF-8-panic class).
