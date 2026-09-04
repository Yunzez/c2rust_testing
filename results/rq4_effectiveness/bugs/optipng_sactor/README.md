# optipng × SACTOR (gpt-5.1): parse-fail — resolver dies on zlib's member-function-pointer allocator

**Verdict: `✗(parse)`** — upgraded from `—` (2026-07-10). SACTOR's structure-aware C resolver
aborts during dependency analysis of optipng, **before any LLM call ($0)**, on the bundled zlib.

## The wall

Entry TU `optipng.c`; whole-program dependency walk (53 TUs incl. bundled zlib/libpng/pngxtern)
reaches zlib's `deflate.c` and dies:

```
Unresolved reference: <unknown> (USR=None) at deflate.c:277
```

Line 277 is the deflate-state allocation:
```c
s = (deflate_state *) ZALLOC(strm, 1, sizeof(deflate_state));
```
where `ZALLOC` expands (zutil.h) to a **member-function-pointer call**:
```c
#define ZALLOC(strm, items, size) \
           (*((strm)->zalloc))((strm)->opaque, (items), (size))
```

This is the **identical construct** that defeats SACTOR on bzip2 (`BZALLOC`'s
`(strm->bzalloc)(...)`, `results/rq4_effectiveness/bugs/bzip2_sactor/`) and behind genann's fn-ptr scaffold break: a
struct member that is itself a function pointer, called through `(*(s->fp))(...)`. SACTOR's resolver
binds it to `USR=None` and cannot proceed. Same failure family, new library.

## Rigor notes (why this is a real SACTOR limit, not a harness artifact)

- **The C compiles clean.** `gcc -I. -c optipng.c / optim.c / deflate.c` → exit 0 (deflate.c only
  emits `-Wold-style-definition` warnings — K&R signatures, valid C). So the resolver failure is
  SACTOR's, not a broken build.
- **The dependency walk is legitimate.** optipng genuinely bundles and calls into zlib for
  compression; you cannot produce a working optipng without it. The E1 row is explicitly
  "optipng (incl. zlib)". The wall being in `deflate.c` is a whole-program verdict, consistent with
  how bzip2 (allocator fn-ptr) was judged.
- **$0 / no LLM.** Died at the resolver stage; probe log shows zero `Translating …` lines. Watchdog
  armed but SACTOR exited on its own first.

## Method

C source: the RustAssure-bundled optipng (`tools/frameworks/rustassure/.../inputs-complex/optipng/src`),
53 core TUs (optipng-proper + zlib + libpng + pngxtern + gifread + pnmio + minitiff; test/demo TUs
with their own `main()` excluded). `compile_commands.json` (this dir) + minimal `test_task.json`
(2 CLI samples — never reached). Probe: `sactor translate --type bin --entry-tu-file optipng.c`
under the same venv/config as the other SACTOR cells (gpt-5.1, sactor.toml).

## Contrast in the SACTOR column

optipng joins bzip2 as a **parse-stage** casualty (member-fn-ptr allocator), the earliest of
SACTOR's failure modes. The column's failure ladder: recursive-core circular-deps refusal
(cJSON/quadtree/lil), fn-ptr-typedef scaffold break (genann → recovered to headline #6; lodepng),
and member-fn-ptr parse-fail (bzip2, **optipng**). Only mechanical c2rust translates optipng
faithfully.
