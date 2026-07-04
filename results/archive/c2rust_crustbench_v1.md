# CRUST-bench × c2rust 0.22.1 — full transpile sweep (v1)

Per repo: library .c files (test/example excluded) transpiled file-by-file with header dirs on -I.

| repo | #c | #transpiled | #rs | status |
|---|--:|--:|--:|---|
| 2DPartInt | 2 | 2 | 2 | OK |
| 42-Kocaeli-Printf | 1 | 0 | 0 | FAIL |
| aes128-SIMD | 7 | 7 | 7 | OK |
| amp | 1 | 1 | 1 | OK |
| approxidate | 2 | 2 | 2 | OK |
| avalanche | 1 | 1 | 1 | OK |
| bhshell | 5 | 5 | 5 | OK |
| bigint | 0 | 0 | 0 | FAIL |
| bitset | 1 | 1 | 1 | OK |
| blt | 3 | 1 | 1 | PARTIAL |
| bostree | 1 | 1 | 1 | OK |
| btree-map | 1 | 1 | 1 | OK |
| c-aces | 6 | 6 | 6 | OK |
| carrays | 1 | 1 | 1 | OK |
| c-blind-rsa-signatures | 1 | 1 | 1 | OK |
| cfsm | 8 | 8 | 8 | OK |
| chtrie | 1 | 1 | 1 | OK |
| CircularBuffer | 1 | 1 | 1 | OK |
| cissy | 4 | 4 | 4 | OK |
| cJSON | 1 | 1 | 1 | OK |
| clhash | 3 | 3 | 3 | OK |
| clog | 0 | 0 | 0 | FAIL |
| coroutine | 1 | 1 | 1 | OK |
| cset | 0 | 0 | 0 | FAIL |
| c-string | 0 | 0 | 0 | FAIL |
| csyncmers | 0 | 0 | 0 | FAIL |
| dict | 1 | 1 | 1 | OK |
| emlang | 7 | 7 | 7 | OK |
| expr | 2 | 2 | 2 | OK |
| FastHamming | 1 | 1 | 1 | OK |
| fft | 1 | 0 | 0 | FAIL |
| file2str | 1 | 1 | 1 | OK |
| fleur | 4 | 4 | 4 | OK |
| fs_c | 2 | 2 | 2 | OK |
| fslib | 17 | 17 | 17 | OK |
| Genetic-neural-network-for-simple-control | 14 | 5 | 5 | PARTIAL |
| geofence | 1 | 1 | 1 | OK |
| gfc | 1 | 0 | 0 | FAIL |
| gorilla-paper-encode | 3 | 3 | 3 | OK |
| Graph-recogniser | 4 | 4 | 4 | OK |
| hamta | 1 | 1 | 1 | OK |
| Holdem-Odds | 5 | 5 | 5 | OK |
| hydra | 1 | 1 | 1 | OK |
| impcheck | 15 | 15 | 15 | OK |
| inversion_list | 1 | 1 | 1 | OK |
| jccc | 8 | 2 | 2 | PARTIAL |
| kairoCompiler | 9 | 9 | 9 | OK |
| kd3 | 1 | 1 | 1 | OK |
| lambda-calculus-eval | 8 | 8 | 8 | OK |
| leftpad | 3 | 3 | 3 | OK |
| lib2bit | 1 | 1 | 1 | OK |
| libbase122 | 1 | 1 | 1 | OK |
| libbeaufort | 6 | 5 | 5 | PARTIAL |
| libfor | 4 | 3 | 3 | PARTIAL |
| libm17 | 9 | 9 | 9 | OK |
| libpgn | 14 | 14 | 14 | OK |
| libpsbt | 6 | 6 | 6 | OK |
| libqueue | 1 | 0 | 0 | FAIL |
| libtinyfseq | 1 | 1 | 1 | OK |
| libutf | 30 | 29 | 29 | PARTIAL |
| libvcd | 1 | 1 | 1 | OK |
| libwecan | 1 | 1 | 1 | OK |
| Linear-Algebra-C | 4 | 4 | 4 | OK |
| ljmm | 1 | 1 | 1 | OK |
| LTRE | 4 | 4 | 4 | OK |
| Math-Library-in-C | 1 | 1 | 1 | OK |
| matrix_multiplication | 3 | 3 | 3 | OK |
| mdb | 4 | 4 | 4 | OK |
| Megalania | 17 | 17 | 17 | OK |
| merkle-tree-c | 0 | 0 | 0 | FAIL |
| morton | 1 | 1 | 1 | OK |
| murmurhash_c | 4 | 4 | 4 | OK |
| mvptree | 3 | 3 | 3 | OK |
| NandC | 1 | 1 | 1 | OK |
| Phills_DHT | 1 | 1 | 1 | OK |
| quadtree | 6 | 6 | 6 | OK |
| razz_simulation | 3 | 3 | 3 | OK |
| rbtree-lab | 7 | 7 | 7 | OK |
| recordManager | 7 | 6 | 6 | PARTIAL |
| rect_pack_h | 0 | 0 | 0 | FAIL |
| Remimu | 0 | 0 | 0 | FAIL |
| rhbloom | 2 | 2 | 2 | OK |
| roaring-bitmap | 3 | 3 | 3 | OK |
| rubiksolver | 7 | 7 | 7 | OK |
| satc | 1 | 1 | 1 | OK |
| Simple-Config | 2 | 2 | 2 | OK |
| simple_lang | 8 | 8 | 8 | OK |
| Simple-Sparsehash | 2 | 2 | 2 | OK |
| SimpleXML | 3 | 1 | 1 | PARTIAL |
| skp | 1 | 1 | 1 | OK |
| SlothLang | 6 | 6 | 6 | OK |
| ted | 4 | 3 | 3 | PARTIAL |
| tisp | 8 | 1 | 1 | PARTIAL |
| totp | 4 | 4 | 4 | OK |
| ulidgen | 2 | 2 | 2 | OK |
| utf8 | 1 | 1 | 1 | OK |
| VaultSync | 11 | 10 | 10 | PARTIAL |
| vec | 1 | 1 | 1 | OK |
| worsp | 3 | 2 | 2 | PARTIAL |
| XOpt | 2 | 0 | 0 | FAIL |

## Summary

- repos: 100  (full=75, partial=12, fail=13)
- C translation units: 376 attempted, 337 transpiled (89%)

## Refined breakdown (2026-06-28)

- **87/100 repos yielded ≥1 c2rust translation** (75 full + 12 partial) = **337 translated TUs, 65 MB**.
- The 13 "FAIL" split into:
  - **8 header-only / no-TU**: only `.c` files are tests; implementation is in headers → no translation
    unit for c2rust (salvageable later by synthesizing a `.c` that `#include`s the header).
    Not a c2rust defect.
  - **5 real transpile errors** (single/few TU): `42-Kocaeli-Printf`, `fft`, `gfc`, `libqueue`, `XOpt`
    — likely need proper compile flags / compile_commands; deferred.
- **c2rust usability on real CRUST-bench C ≈ 87% of repos, 90% of attempted TUs.**

This is a free, name-preserving translation corpus, ready as: (a) differential-fuzz targets to hunt
**c2rust's own** bugs at scale; (b) the c2rust anchor for 3-way attribution when testing LLM rewrites;
(c) name-preserving controls for the matcher.
