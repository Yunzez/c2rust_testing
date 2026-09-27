# Same-corpus C reach — bzip2_c2rust

Paired reach diagnostic (docs/c_reach_plan.md): the cell's archived corpus replayed in `C2R_MODE=c-only` through
harnesses rebuilt with generator `fd1f75b4716a1d45` (`--c-coverage`, C oracle `bzip2lib.c` with
`-fprofile-instr-generate -fcoverage-mapping`, `cargo fuzz build --sanitizer none`). The corpus is Rust-guided:
this is not a C coverage baseline. The Rust side is the archived cell, not re-measured. A C-side `completed`
shows reach, not C-definedness.

## Side-specific reach (not comparable across sides)

| side | functions | regions |
|---|---|---|
| C (this replay) | 46 / 64 (0.719) | 1599 / 2212 (0.723) |
| Rust (archived campaign) | 46 / 66 (0.697) | 7090 / 8789 (0.807) |

Inputs replayed on the C side: {'completed': 1281, 'crash': 4, 'timeout': 1} over 18 / 19 archived-built boundaries.

## Matched functions — accepted pairs ∩ C scope ∩ Rust scope

Map: `results/rq1_matching/raw/group_a/bzip2__c2rust/matcher_output.json` (`deployment`, {'topo': True, 'abstain_eps': 0.01}); accepted pairs in both scopes: 60 (out of scope: 1).

| both | C only (Rust terminated) | Rust only (C terminated) | neither | ambiguous | C unmatched (reached) | Rust unmatched (reached) |
|---|---|---|---|---|---|---|
| 42 | 0 (0) | 0 (0) | 18 | 3 | 1 (1) | 3 (1) |

## Per boundary

| boundary | status | corpus | outcomes | C fn | C reg |
|---|---|---|---|---|---|
| BZ2_bzBuffToBuffCompress | ok | 590 | {'completed': 590} | 35/64 | 978/2212 |
| BZ2_bzBuffToBuffDecompress | ok | 437 | {'completed': 437} | 13/64 | 632/2212 |
| BZ2_bz__AssertH__fail | ok | 1 | {'crash': 1} | 0/64 | 0/2212 |
| BZ2_bzlibVersion | ok | 1 | {'completed': 1} | 1/64 | 1/2212 |
| BZ2_hbAssignCodes | ok | 45 | {'completed': 45} | 1/64 | 9/2212 |
| BZ2_hbCreateDecodeTables | rebuild failed | 52 | error[E0425]: cannot find value `alphaSize` in this scope
error[E0425]: cannot find value `alphaSize` in this scope
error[E0425]: cannot find value `alphaSize` in this scope | / | / |
| BZ2_hbMakeCodeLengths | ok | 7 | {'completed': 6, 'timeout': 1} | 1/64 | 20/2212 |
| BZ2_indexIntoF | ok | 21 | {'completed': 21} | 1/64 | 6/2212 |
| bz_config_ok | ok | 1 | {'completed': 1} | 1/64 | 7/2212 |
| default_bzalloc | ok | 6 | {'completed': 6} | 1/64 | 1/2212 |
| default_bzfree | ok | 1 | {'completed': 1} | 1/64 | 3/2212 |
| fallbackQSort3 | ok | 19 | {'completed': 19} | 2/64 | 49/2212 |
| fallbackSimpleSort | ok | 21 | {'completed': 21} | 1/64 | 18/2212 |
| fallbackSort | ok | 57 | {'completed': 57} | 3/64 | 129/2212 |
| mainGtU | ok | 5 | {'completed': 5} | 1/64 | 65/2212 |
| mainQSort3 | ok | 38 | {'completed': 37, 'crash': 1} | 4/64 | 120/2212 |
| mainSimpleSort | ok | 31 | {'completed': 30, 'crash': 1} | 2/64 | 87/2212 |
| mainSort | ok | 1 | {'crash': 1} | 0/64 | 0/2212 |
| mmed3 | ok | 4 | {'completed': 4} | 1/64 | 7/2212 |

## Procedure, deviations, and what is not established

<!-- prose -->
