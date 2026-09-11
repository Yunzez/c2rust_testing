# Seeds-only census — plan-guided seeds vs the default seed vs the archived campaign

Per cell: harnesses rebuilt with generator 0.9.1 (seed-ir branch; emitted code identical to 0.9), one coverage build per harness, coverage of (a) the default 64-byte seed alone and (b) default + plan-guided seeds (frozen policy, `docs/seeding_policy_plan.md` §3), against the cell's archived universe; the archived campaign (3 600 s, generator 0.8) beside them. **seed effect** = policy − default; **rerun criterion** = policy seeds-only ≥ archived campaign + 5 pp.

| cell | built (census / archived) | plan-guided / planned | default seed | policy seeds | archived campaign | seed effect | policy − campaign | qualifies |
|---|---:|---:|---:|---:|---:|---:|---:|:-:|
| bzip2 × c2rust | 18 / 19 | 16 / 19 | 2070 (0.236) | 2288 (0.260) | 7090 (0.807) | +2.5 pp | -54.6 pp | no |
| bzip2 × c2saferrust | 16 / 17 | 14 / 17 | 432 (0.053) | 538 (0.065) | 1158 (0.141) | +1.3 pp | -7.5 pp | no |
| bzip2 × crown | 18 / 19 | 16 / 19 | 804 (0.089) | 826 (0.091) | 5414 (0.596) | +0.2 pp | -50.5 pp | no |
| bzip2 × laertes | 18 / 19 | 16 / 19 | 2076 (0.206) | 2294 (0.228) | 6206 (0.617) | +2.2 pp | -38.9 pp | no |
| cjson × c2rust | 39 / 39 | 3 / 39 | 325 (0.145) | 326 (0.146) | 1816 (0.812) | +0.0 pp | -66.6 pp | no |
| cjson × ptrtrans | 9 / 9 | 4 / 15 | 68 (0.032) | 68 (0.032) | 68 (0.032) | +0.0 pp | +0.0 pp | no |
| genann × c2rust | 10 / 10 | 5 / 10 | 447 (0.780) | 447 (0.780) | 462 (0.806) | +0.0 pp | -2.6 pp | no |
| genann × c2saferrust | 10 / 10 | 5 / 10 | 444 (0.789) | 444 (0.789) | 459 (0.815) | +0.0 pp | -2.7 pp | no |
| genann × crown | 10 / 10 | 5 / 10 | 451 (0.786) | 451 (0.786) | 467 (0.814) | +0.0 pp | -2.8 pp | no |
| genann × laertes | 10 / 10 | 5 / 10 | 452 (0.770) | 452 (0.770) | 467 (0.796) | +0.0 pp | -2.6 pp | no |
| genann × sactor | 13 / 13 | 1 / 13 | 475 (0.663) | 475 (0.663) | 506 (0.707) | +0.0 pp | -4.3 pp | no |
| lil × c2rust | 51 / 50 | 5 / 51 | 1424 (0.249) | 1426 (0.249) | 4999 (0.872) | +0.0 pp | -62.4 pp | no |
| lil × c2saferrust | 47 / 47 | 5 / 47 | 292 (0.051) | 296 (0.051) | 362 (0.063) | +0.1 pp | -1.1 pp | no |
| lil × crown | 42 / 42 | 4 / 42 | 1191 (0.186) | 1193 (0.186) | 5294 (0.826) | +0.0 pp | -64.0 pp | no |
| lil × laertes | 51 / 51 | 5 / 51 | 1436 (0.234) | 1438 (0.234) | 5028 (0.818) | +0.0 pp | -58.4 pp | no |
| lodepng × c2rust | 54 / 54 | 41 / 64 | 457 (0.034) | 542 (0.041) | 1675 (0.126) | +0.6 pp | -8.5 pp | no |
| lodepng × crown | 54 / 54 | 37 / 57 | 466 (0.033) | 551 (0.038) | 1449 (0.101) | +0.6 pp | -6.3 pp | no |
| optipng × c2rust | 54 / 54 | 63 / 128 | 6775 (0.179) | 7176 (0.190) | 9970 (0.263) | +1.1 pp | -7.4 pp | no |
| optipng × c2saferrust | 96 / 96 | 63 / 121 | 6021 (0.161) | 6070 (0.163) | 9815 (0.263) | +0.1 pp | -10.0 pp | no |
| optipng × laertes | 55 / 55 | 64 / 121 | 5511 (0.112) | 5565 (0.114) | 6611 (0.135) | +0.1 pp | -2.1 pp | no |
| qsort × c2rust | 3 / 3 | 3 / 3 | 54 (0.982) | 54 (0.982) | 55 (1.000) | +0.0 pp | -1.8 pp | no |
| qsort × c2saferrust | 3 / 3 | 3 / 3 | 56 (0.982) | 56 (0.982) | 57 (1.000) | +0.0 pp | -1.8 pp | no |
| qsort × crown | 3 / 3 | 3 / 3 | 60 (0.984) | 60 (0.984) | 61 (1.000) | +0.0 pp | -1.6 pp | no |
| qsort × laertes | 3 / 3 | 3 / 3 | 61 (0.984) | 61 (0.984) | 62 (1.000) | +0.0 pp | -1.6 pp | no |
| qsort × ptrtrans | 3 / 3 | 3 / 3 | 103 (0.858) | 103 (0.858) | 105 (0.875) | +0.0 pp | -1.7 pp | no |
| qsort × sactor | 3 / 3 | 3 / 3 | 61 (0.311) | 61 (0.311) | 99 (0.505) | +0.0 pp | -19.4 pp | no |
| quadtree × c2rust | 17 / 17 | 3 / 17 | 202 (0.463) | 202 (0.463) | 220 (0.505) | +0.0 pp | -4.1 pp | no |
| quadtree × crown | 13 / 13 | 3 / 13 | 204 (0.334) | 204 (0.334) | 208 (0.341) | +0.0 pp | -0.7 pp | no |
| quadtree × ptrtrans | 11 / 11 | 3 / 11 | 134 (0.329) | 134 (0.329) | 134 (0.329) | +0.0 pp | +0.0 pp | no |
| urlparser × c2rust | 20 / 20 | 1 / 20 | 110 (0.092) | 110 (0.092) | 110 (0.092) | +0.0 pp | +0.0 pp | no |
| urlparser × c2saferrust | 18 / 18 | 1 / 18 | 107 (0.090) | 107 (0.090) | 107 (0.090) | +0.0 pp | +0.0 pp | no |
| urlparser × crown | 19 / 19 | 1 / 20 | 98 (0.086) | 98 (0.086) | 98 (0.086) | +0.0 pp | +0.0 pp | no |
| urlparser × laertes | 20 / 20 | 1 / 20 | 41 (0.028) | 41 (0.028) | 41 (0.028) | +0.0 pp | +0.0 pp | no |
| tulip × c2rust | 212 / 212 | 162 / 212 | 3180 (0.342) | 8454 (0.909) | 3197 (0.344) | +56.7 pp | +56.5 pp | **yes** |

34 cells; qualifying: tulip × c2rust.
