# bzip2 × SACTOR (gpt-5.1): parse-fail — cannot ingest stock bzlib.c

**Verdict: `✗(parse)`** — SACTOR's C reference resolver dies on stock bzip2 before any translation
can begin; each local rewrite reveals the next wall (`parser_errors.txt`):

1. `Unresolved reference: <unknown> (USR=None) at bzlib.c:168` — the `BZALLOC` macro's
   **member-function-pointer call** `(strm->bzalloc)(strm->opaque,(nnn),1)`. Rewrote to
   `strm->bzalloc(...)` (output-verified identical roundtrip) →
2. `Unresolved reference: <function> (USR=c:@F@fdopen)` — the high-level `BZ2_bzopen/bzdopen`
   FILE API; SACTOR's resolver has no binding for `fdopen` (and `fclose/ferror/fgetc` queue behind it).

Behind the parser lies a third, decisive wall: `bz_stream`'s **function-pointer members**
(`void *(*bzalloc)(void*,int,int)`) — precisely the construct that breaks SACTOR's typedef/struct
scaffold on genann (0/15, `results/rq4_effectiveness/bugs/genann_sactor/`) and lodepng (probe). Stripping the FILE API +
allocator hooks from bzlib.c to squeeze past would no longer be testing bzip2.

**bzip2 now defeats ALL FIVE non-mechanical tools, five different ways**: Laertes CRC-zeroing
(headline #4) / C2SaferRust NULL-empty + panic / CROWN corrupt output + heap corruption (headline #3)
/ PtrTrans module-assembly compile-fail / **SACTOR can't even parse it**. Only mechanical c2rust is
faithful. Progressively *earlier* failure stages, too: semantic bug → crash+semantic → memory
corruption → compile-fail → parse-fail.

## Files
- `parser_errors.txt` — both probe walls
- `driver.c` (BuffToBuffCompress→Decompress roundtrip, checksum), `test_samples.json`,
  `test_task.json`, `compile_commands.json` — harness (durable copy)
