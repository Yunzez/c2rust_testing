# Deep corpus provenance (Layer 2 — call-graph frontier pilot)

The Layer-1 corpus (authored + musl/base64) is too shallow for frontier selection: Step 0
(`results/callgraph_depth_audit_v1.md`) found only 7/48 programs with any frontier choice and a max
call-graph depth of 5. Frontier selection only matters on **deep, multi-level** call graphs, so this
deep corpus is vendored specifically for Layer 2 (the STU frontier experiment), not for the Layer-1
risk model.

All three are real, permissively-licensed, self-contained single-TU C libraries that transpile clean
under c2rust 0.22.1 + clang 21, with **C↔Rust name-mapping coverage 1.00**. Their exported symbols do
**not** collide with libc, so (unlike the musl programs) they are kept under their upstream names.

| program | upstream source | license | C funcs | call-graph depth | internal nodes | character |
|---|---|---|--:|--:|--:|---|
| regex | github.com/kokke/tiny-regex-c `re.c` | Unlicense (public domain) | 18 | 8 | 6 | recursive backtracking matcher |
| bignum | github.com/kokke/tiny-bignum-c `bn.c` | Unlicense (public domain) | 27 | 5 | 5 | layered arbitrary-precision arithmetic |
| tinyexpr | github.com/codeplea/tinyexpr `tinyexpr.c` | zlib | 29 | 5 | 4 | recursive-descent expression parser |

(Depth/structure measured by `scripts/callgraph_depth_audit.py`; compare the authored max of 5 at
hash_table with only 4 internal nodes.)

## Licensing

- **tiny-regex-c**, **tiny-bignum-c** — released under the **Unlicense** (public-domain dedication);
  see `LICENSE-unlicense.txt` in this directory. Upstream: https://github.com/kokke
- **tinyexpr** — **zlib license**; the copyright/permission notice is retained in the header of
  `tinyexpr/source/tinyexpr.c` (© 2015–2020 Lewis Van Winkle). Upstream:
  https://github.com/codeplea/tinyexpr

## Why these (and not more, yet)

Per `docs/roadmap_frontier.md` Step 1, this is a **pilot** (3 deep programs spanning matcher /
arithmetic / parser). Scale only if the G3 headline table shows insufficient variance or a
reviewer-risk gap — not by default.
