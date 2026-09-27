# External corpus provenance (real upstream C libraries)

These pairs are vendored from real, permissively-licensed upstream C libraries to test the external
validity of the boundary-validity findings (see `results/external_validity_v1.md`). Each is a
self-contained single translation unit that transpiles cleanly under c2rust 0.22.1 + clang 21.

The musl functions are renamed `mu_<name>` (e.g. `strlen` → `mu_strlen`). This is a **pure symbol
rename** to avoid a libc symbol collision: the c2rust output is `#[no_mangle] pub extern "C" fn
<name>`, which would clash with libc's own `<name>` when the fuzz binary links libc. The algorithm
body is byte-for-byte musl. base64's symbols (`base64_encode`/`base64_decode`) do not collide and
are kept verbatim.

| program | upstream source | license | upstream symbol | mechanism focus |
|---|---|---|---|---|
| base64 | github.com/zhicheng/base64 `base64.c` | Public domain | base64_encode / base64_decode | isolation (output-size precondition) |
| mu_strlen | musl `src/string/strlen.c` | MIT | strlen | isolation (NUL-termination) |
| mu_strspn | musl `src/string/strspn.c` | MIT | strspn | isolation (two NUL strings) |
| mu_strncmp | musl `src/string/strncmp.c` | MIT | strncmp | isolation (NUL strings + bound) |
| mu_memcmp | musl `src/string/memcmp.c` | MIT | memcmp | census-excluded (void* buffer) |
| mu_memchr | musl `src/string/memchr.c` | MIT | memchr | census-excluded (void* buffer) |
| mu_atoi | musl `src/stdlib/atoi.c` | MIT | atoi | intrinsic-UB (signed multiply-accumulate) |
| mu_llabs | musl `src/stdlib/llabs.c` | MIT | llabs | intrinsic-UB (LLONG_MIN negation) |

## Licensing

- **musl** is distributed under the MIT license (a single upstream COPYRIGHT file, not per-file
  headers). The MIT rights statement is retained in `LICENSE-musl.txt` in this directory. Copyright
  © 2005–2020 Rich Felker and musl contributors.
- **base64.c** carries its own public-domain notice in the file header ("public domain base64
  implementation written by WEI Zhicheng").

## Funnel (real libs exercise the exclusion paths honestly)

- **void\* buffers** (`mu_memcmp`, `mu_memchr`): the `const void *` pointee has no element type, so
  the census gates them `UNSUPPORTED_PARAM (struct_ptr)` — not constructible by the current
  generator. A genuine coverage limit on real code, recorded rather than hidden.
- **pointer returns** (memchr/strchrnul-style) return a pointer INTO the input; C and Rust return
  numerically different addresses, so the differential harness cannot compare them.
