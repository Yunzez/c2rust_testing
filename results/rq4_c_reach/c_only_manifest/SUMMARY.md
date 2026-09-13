# C-guided application reach — completed manifest

Only C was fuzzed and only C coverage was measured in these runs.

| C realization | boundaries | batches | max fuzzers | C functions | C regions | inputs |
|---|---:|---:|---:|---:|---:|---:|
| bzip2_c2rust | 19 | 1 | 19 | 46/64 | 1677/2212 | 1323 |
| cjson_c2rust | 39 | 2 | 28 | 49/58 | 759/926 | 12382 |
| cjson_ptrtrans | 9 | 1 | 9 | 10/113 | 37/1618 | 23 |
| genann_c2rust | 10 | 1 | 10 | 10/12 | 142/172 | 211 |
| genann_sactor | 13 | 1 | 13 | 13/15 | 151/182 | 338 |
| lil_laertes | 51 | 2 | 28 | 140/145 | 1952/2249 | 5618 |
| lodepng_c2rust | 54 | 2 | 28 | 56/235 | 570/3874 | 1626 |
| optipng_c2saferrust | 96 | 4 | 28 | 136/552 | 1960/10394 | 2441 |
| qsort_sactor | 3 | 1 | 3 | 3/3 | 10/10 | 81 |
| quadtree_c2rust | 17 | 1 | 17 | 20/24 | 106/205 | 128 |
| tulip_c2rust | 212 | 8 | 28 | 212/213 | 434/1584 | 3809 |
| urlparser_c2rust | 20 | 1 | 20 | 7/21 | 39/307 | 49 |

Valid units: 12/12.  
Maximum concurrent boundary fuzzers: 28/28.
